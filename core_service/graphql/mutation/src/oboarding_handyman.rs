use async_graphql::{Context, ID, InputObject, Object, SimpleObject};
use core_service_db as db;
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::{
    GlobalId, Handyman, HandymanExpertise, HandymanProfile, SetValue,
};
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
    async fn handyman_profile_add_expertises(
        &self,
        ctx: &Context<'_>,
        input: HandymanProfileAddExpertisesInput,
    ) -> Result<HandymanProfileAddExpertisesPayload> {
        let HandymanProfileAddExpertisesInput {
            handyman_id,
            expertises,
        } = input;
        let handyman_id = Handyman::from_global_id(&handyman_id)?.inner_id();

        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let actor_auth = session_ctx.as_actor_auth();
        actor_auth.require_handyman_access(handyman_id)?;

        let new_records = expertises
            .iter()
            .map(db::NewHandymanExpertise::from)
            .collect::<Vec<_>>();
        with_mutable_db(&context.db_connection_pool, |conn| {
            db::HandymanExpertise::create_many(&actor_auth, handyman_id, &new_records, conn)
                .scope_boxed()
        })
        .await?;

        Ok(HandymanProfileAddExpertisesPayload {
            profile: HandymanProfile::new(handyman_id),
        })
    }

    async fn handyman_profile_update_expertise(
        &self,
        ctx: &Context<'_>,
        input: HandymanProfileUpdateExpertiseInput,
    ) -> Result<HandymanProfileUpdateExpertisePayload> {
        let HandymanProfileUpdateExpertiseInput {
            handyman_id,
            expertise_id,
            changeset,
        } = input;
        let handyman_id = Handyman::from_global_id(&handyman_id)?.inner_id();
        let expertise_id = HandymanExpertise::from_global_id(&expertise_id)?.id;

        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let actor_auth = session_ctx.as_actor_auth();
        actor_auth.require_handyman_access(handyman_id)?;

        let updated = with_mutable_db(&context.db_connection_pool, |conn| {
            db::HandymanExpertise::update(
                &actor_auth,
                HandymanAccessGuardId {
                    handyman_id,
                    entity_id: expertise_id,
                },
                db::HandymanExpertiseChangeset::from(&changeset),
                conn,
            )
            .scope_boxed()
        })
        .await?;

        Ok(HandymanProfileUpdateExpertisePayload {
            expertise: HandymanExpertise::new(Arc::new(updated)),
        })
    }
}

#[derive(Debug, InputObject)]
struct HandymanProfileAddExpertisesInput {
    handyman_id: ID,
    expertises: Vec<NewHandymanExpertise>,
}

#[derive(Debug, InputObject)]
struct NewHandymanExpertise {
    service: ServiceLayer2,
    note: Option<String>,
    rate_vnd: Option<u32>,
}

impl<'a> From<&'a NewHandymanExpertise> for db::NewHandymanExpertise<'a> {
    fn from(
        NewHandymanExpertise {
            service,
            note,
            rate_vnd,
        }: &'a NewHandymanExpertise,
    ) -> Self {
        db::NewHandymanExpertise {
            service: *service,
            note: note.as_deref(),
            rate_vnd: rate_vnd.map(|r| r.try_into().unwrap_or(i32::MAX)),
        }
    }
}

#[derive(SimpleObject)]
struct HandymanProfileAddExpertisesPayload {
    profile: HandymanProfile,
}

#[derive(Debug, InputObject)]
struct HandymanProfileUpdateExpertiseInput {
    handyman_id: ID,
    expertise_id: ID,
    changeset: HandymanProfileUpdateExpertiseChangeset,
}

#[derive(Debug, InputObject)]
struct HandymanProfileUpdateExpertiseChangeset {
    note: Option<SetValue<String>>,
    rate_vnd: Option<SetValue<i32>>,
}

impl<'a> From<&'a HandymanProfileUpdateExpertiseChangeset> for db::HandymanExpertiseChangeset<'a> {
    fn from(value: &'a HandymanProfileUpdateExpertiseChangeset) -> Self {
        db::HandymanExpertiseChangeset {
            note: value.note.as_ref().map(|s| s.value.as_deref()),
            rate_vnd: value.rate_vnd.as_ref().map(|s| s.value),
        }
    }
}

#[derive(SimpleObject)]
struct HandymanProfileUpdateExpertisePayload {
    expertise: HandymanExpertise,
}
