use crate::{CachedNode, GlobalId};
use account_service_db as acc_db;
use async_graphql::{Context, ID, Object};
use core_service_graphql_context::RequestContext;
use entity_type::HandymanId;
use error::{Error, Result};
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
}
