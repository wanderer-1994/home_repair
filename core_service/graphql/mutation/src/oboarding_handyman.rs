use async_graphql::{Context, ID, InputObject, Object, SimpleObject};
use core_service_db as db;
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::{GlobalId, Handyman, HandymanProfile, HandymanService, SetValue};
use db_utils::with_mutable_db;
use entity_type::{HandymanAccessGuardId, ServiceLayer2};
use error::Result;
use scoped_futures::ScopedFutureExt;
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
        with_mutable_db(&context.db_connection_pool, |conn| {
            db::HandymanService::create_many(&actor_auth, handyman_id, &new_records, conn)
                .scope_boxed()
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
