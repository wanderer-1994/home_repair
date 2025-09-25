use crate::{CachedNode, GlobalId};
use account_service_db as acc_db;
use async_graphql::{Context, ID, Object};
use core_service_graphql_context::RequestContext;
use entity_type::CustomerId;
use error::{Error, Result};
use std::sync::Arc;

pub type CustomerProfile = CachedNode<CustomerId, Arc<acc_db::CustomerProfile>>;

impl CustomerProfile {
    async fn load(
        id: CustomerId,
        context: &RequestContext,
    ) -> Result<Arc<acc_db::CustomerProfile>> {
        context
            .customer_loaders
            .profile_by_id_loader
            .load_one(id)
            .await?
            .ok_or_else(|| Error::not_found("CustomerProfile not found"))
    }

    async fn get(&self, ctx: &Context<'_>) -> Result<&Arc<acc_db::CustomerProfile>> {
        let context = ctx.data::<RequestContext>()?;
        self.get_or_load(|id| Self::load(*id, context)).await
    }
}

#[Object]
impl CustomerProfile {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }

    async fn nick_name(&self, ctx: &Context<'_>) -> Result<&str> {
        Ok(&self.get(ctx).await?.nick_name)
    }
}
