use crate::{CachedNode, GlobalId, HandymanExpertiseGroup};
use account_service_db as acc_db;
use async_graphql::{Context, ID, Object};
use core_service_db as db;
use core_service_graphql_context::RequestContext;
use db_utils::with_readonly_db;
use entity_type::HandymanId;
use error::{Error, Result};
use scoped_futures::ScopedFutureExt;
use std::sync::Arc;

pub type HandymanProfile = CachedNode<HandymanId, Arc<acc_db::HandymanProfile>>;

impl HandymanProfile {
    async fn load(
        id: HandymanId,
        context: &RequestContext,
    ) -> Result<Arc<acc_db::HandymanProfile>> {
        context
            .handyman_loaders
            .profile_by_id_loader
            .load_one(id)
            .await?
            .ok_or_else(|| Error::not_found("Handyman not found"))
    }

    async fn get(&self, ctx: &Context<'_>) -> Result<&Arc<acc_db::HandymanProfile>> {
        let context = ctx.data::<RequestContext>()?;
        self.get_or_load(|id| Self::load(*id, context)).await
    }
}

#[Object]
impl HandymanProfile {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }

    async fn first_name(&self, ctx: &Context<'_>) -> Result<&str> {
        Ok(&self.get(ctx).await?.first_name)
    }

    async fn last_name(&self, ctx: &Context<'_>) -> Result<&str> {
        Ok(&self.get(ctx).await?.last_name)
    }

    async fn expertises(&self, ctx: &Context<'_>) -> Result<Vec<HandymanExpertiseGroup>> {
        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let actor_auth = session_ctx.as_actor_auth();
        let handyman_id = self.get(ctx).await?.handyman_id;

        let group = with_readonly_db(&context.db_connection_pool, |conn| {
            db::HandymanExpertise::get_by_handyman(&actor_auth, handyman_id, conn).scope_boxed()
        })
        .await?
        .into_group();

        Ok(group
            .into_iter()
            .map(HandymanExpertiseGroup::from)
            .collect())
    }
}
