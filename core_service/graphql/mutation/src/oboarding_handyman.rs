use async_graphql::{Context, ID, InputObject, Object, SimpleObject};
use core_service_db as db;
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::{GlobalId, Handyman, HandymanProfile, HandymanService, SetValue};
use db_utils::with_mutable_db;
use entity_type::{HandymanAccessGuardId, ServiceLayer2};
use error::{Error, Result};
use scoped_futures::ScopedFutureExt;
use search_service_server::{HandymanIndexRequest, HandymanIndexType};
use std::sync::Arc;

#[derive(Default)]
pub struct OnboardingHandymanMutation;

#[Object]
impl OnboardingHandymanMutation {
    #[tracing::instrument(skip(self, ctx))]
    async fn handyman_profile_add_services(
        &self,
        ctx: &Context<'_>,
        input: HandymanProfileAddServicesInput,
    ) -> Result<HandymanProfileAddServicesPayload> {
        let HandymanProfileAddServicesInput {
            handyman_id,
            services,
        } = input;
        let handyman_id = Handyman::from_global_id(&handyman_id)?.inner_id();

        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let actor_auth = session_ctx.as_actor_auth();
        actor_auth.require_handyman_access(handyman_id)?;

        let new_records = services
            .iter()
            .map(db::NewHandymanService::from)
            .collect::<Vec<_>>();
        let added_services = with_mutable_db(&context.db_connection_pool, |conn| {
            db::HandymanService::create_many(&actor_auth, handyman_id, &new_records, conn)
                .scope_boxed()
        })
        .await?;

        context
            .search_service_client
            .handyman_index(HandymanIndexRequest {
                handyman_id,
                index_type: HandymanIndexType::AddSkills(
                    added_services.iter().map(|s| s.service).collect(),
                ),
            })
            .await?;

        Ok(HandymanProfileAddServicesPayload {
            profile: HandymanProfile::new(handyman_id),
        })
    }

    #[tracing::instrument(skip(self, ctx))]
    async fn handyman_profile_update_service(
        &self,
        ctx: &Context<'_>,
        input: HandymanProfileUpdateServiceInput,
    ) -> Result<HandymanProfileUpdateServicePayload> {
        let HandymanProfileUpdateServiceInput {
            handyman_id,
            service_id,
            changeset,
        } = input;
        let handyman_id = Handyman::from_global_id(&handyman_id)?.inner_id();
        let service_id = HandymanService::from_global_id(&service_id)?.id;

        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let actor_auth = session_ctx.as_actor_auth();
        actor_auth.require_handyman_access(handyman_id)?;

        let updated = with_mutable_db(&context.db_connection_pool, |conn| {
            db::HandymanService::update(
                &actor_auth,
                HandymanAccessGuardId {
                    handyman_id,
                    entity_id: service_id,
                },
                db::HandymanServiceChangeset::from(&changeset),
                conn,
            )
            .scope_boxed()
        })
        .await?;

        Ok(HandymanProfileUpdateServicePayload {
            service: HandymanService::new(Arc::new(updated)),
        })
    }

    #[tracing::instrument(skip(self, ctx))]
    async fn handyman_profile_remove_service(
        &self,
        ctx: &Context<'_>,
        input: HandymanProfileRemoveServiceInput,
    ) -> Result<HandymanProfileRemoveServicePayload> {
        let HandymanProfileRemoveServiceInput {
            handyman_id,
            service_id,
        } = input;
        let handyman_id = Handyman::from_global_id(&handyman_id)?.inner_id();
        let ids_to_delete = vec![HandymanService::from_global_id(&service_id)?.id];

        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let actor_auth = session_ctx.as_actor_auth();
        actor_auth.require_handyman_access(handyman_id)?;

        let (deleted, should_remove_service_index) =
            with_mutable_db(&context.db_connection_pool, |conn| {
                async {
                    let deleted = db::HandymanService::delete_many(
                        &actor_auth,
                        handyman_id,
                        &ids_to_delete,
                        conn,
                    )
                    .await?
                    .pop()
                    .ok_or_else(|| Error::internal("Expect 1 deleted service"))?;
                    let should_remove_service_index =
                        !db::HandymanService::handyman_service_exists(
                            handyman_id,
                            deleted.service,
                            conn,
                        )
                        .await?;
                    Ok((deleted, should_remove_service_index))
                }
                .scope_boxed()
            })
            .await?;

        if should_remove_service_index {
            context
                .search_service_client
                .handyman_index(HandymanIndexRequest {
                    handyman_id,
                    index_type: HandymanIndexType::RemoveSkill(deleted.service),
                })
                .await?;
        }

        Ok(HandymanProfileRemoveServicePayload {
            profile: HandymanProfile::new(handyman_id),
            removed_service_id: service_id,
        })
    }
}

#[derive(Debug, InputObject)]
struct HandymanProfileAddServicesInput {
    handyman_id: ID,
    services: Vec<NewHandymanService>,
}

#[derive(Debug, InputObject)]
struct NewHandymanService {
    service: ServiceLayer2,
    note: Option<String>,
    rate_vnd: Option<u32>,
}

impl<'a> From<&'a NewHandymanService> for db::NewHandymanService<'a> {
    fn from(
        NewHandymanService {
            service,
            note,
            rate_vnd,
        }: &'a NewHandymanService,
    ) -> Self {
        db::NewHandymanService {
            service: *service,
            note: note.as_deref(),
            rate_vnd: rate_vnd.map(|r| r.try_into().unwrap_or(i32::MAX)),
        }
    }
}

#[derive(SimpleObject)]
struct HandymanProfileAddServicesPayload {
    profile: HandymanProfile,
}

#[derive(Debug, InputObject)]
struct HandymanProfileUpdateServiceInput {
    handyman_id: ID,
    service_id: ID,
    changeset: HandymanProfileUpdateServiceChangeset,
}

#[derive(Debug, InputObject)]
struct HandymanProfileUpdateServiceChangeset {
    note: Option<SetValue<String>>,
    rate_vnd: Option<SetValue<i32>>,
}

impl<'a> From<&'a HandymanProfileUpdateServiceChangeset> for db::HandymanServiceChangeset<'a> {
    fn from(value: &'a HandymanProfileUpdateServiceChangeset) -> Self {
        db::HandymanServiceChangeset {
            note: value.note.as_ref().map(|s| s.value.as_deref()),
            rate_vnd: value.rate_vnd.as_ref().map(|s| s.value),
        }
    }
}

#[derive(SimpleObject)]
struct HandymanProfileUpdateServicePayload {
    service: HandymanService,
}

#[derive(Debug, InputObject)]
struct HandymanProfileRemoveServiceInput {
    handyman_id: ID,
    service_id: ID,
}

#[derive(SimpleObject)]
struct HandymanProfileRemoveServicePayload {
    profile: HandymanProfile,
    removed_service_id: ID,
}
