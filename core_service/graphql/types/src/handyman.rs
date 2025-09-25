use crate::{CachedNode, GlobalId, HandymanProfile};
use account_service_db as acc_db;
use account_service_server::LoadHandymanProfileByIdsRequest;
use async_graphql::{Context, ID, Object};
use core_service_graphql_context::RequestContext;
use entity_type::HandymanId;
use error::{Error, Result};
use std::sync::Arc;

pub type Handyman = CachedNode<HandymanId, Arc<acc_db::HandymanAccount>>;

impl Handyman {
    async fn load(
        id: HandymanId,
        context: &RequestContext,
    ) -> Result<Arc<acc_db::HandymanAccount>> {
        context
            .handyman_loaders
            .account_by_id_loader
            .load_one(id)
            .await?
            .ok_or_else(|| Error::not_found("Handyman not found"))
    }

    async fn get(&self, ctx: &Context<'_>) -> Result<&Arc<acc_db::HandymanAccount>> {
        let context = ctx.data::<RequestContext>()?;
        self.get_or_load(|id| Self::load(*id, context)).await
    }
}

#[Object]
impl Handyman {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }

    async fn phone_number(&self, ctx: &Context<'_>) -> Result<&str> {
        Ok(&self.get(ctx).await?.phone_number)
    }

    async fn profile(&self, ctx: &Context<'_>) -> Result<Option<HandymanProfile>> {
        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let profile = context
            .account_service_client
            .load_handyman_profile_by_ids(LoadHandymanProfileByIdsRequest {
                actor_auth: session_ctx.as_actor_auth(),
                account_ids: vec![self.inner_id()],
            })
            .await?
            .profiles
            .pop()
            .map(Arc::new);

        let Some(profile) = profile else {
            return Ok(None);
        };

        context
            .handyman_loaders
            .profile_by_id_loader
            .feed_one(profile.handyman_id, Arc::clone(&profile))
            .await;

        Ok(Some(HandymanProfile::new_with(
            profile.handyman_id,
            profile,
        )))
    }
}
