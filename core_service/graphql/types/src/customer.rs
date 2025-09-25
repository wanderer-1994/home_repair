use crate::{CachedNode, CustomerProfile, GlobalId};
use account_service_db as acc_db;
use account_service_server::LoadCustomerProfileByIdsRequest;
use async_graphql::{Context, ID, Object};
use core_service_graphql_context::RequestContext;
use entity_type::CustomerId;
use error::{Error, Result};
use std::sync::Arc;

pub type Customer = CachedNode<CustomerId, Arc<acc_db::CustomerAccount>>;

impl Customer {
    async fn load(
        id: CustomerId,
        context: &RequestContext,
    ) -> Result<Arc<acc_db::CustomerAccount>> {
        context
            .customer_loaders
            .account_by_id_loader
            .load_one(id)
            .await?
            .ok_or_else(|| Error::not_found("Customer not found"))
    }

    async fn get(&self, ctx: &Context<'_>) -> Result<&Arc<acc_db::CustomerAccount>> {
        let context = ctx.data::<RequestContext>()?;
        self.get_or_load(|id| Self::load(*id, context)).await
    }
}

#[Object]
impl Customer {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }

    async fn phone_number(&self, ctx: &Context<'_>) -> Result<&str> {
        Ok(&self.get(ctx).await?.phone_number)
    }

    async fn profile(&self, ctx: &Context<'_>) -> Result<Option<CustomerProfile>> {
        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let profile = context
            .account_service_client
            .load_customer_profile_by_ids(LoadCustomerProfileByIdsRequest {
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
            .customer_loaders
            .profile_by_id_loader
            .feed_one(profile.customer_id, Arc::clone(&profile))
            .await;

        Ok(Some(CustomerProfile::new_with(
            profile.customer_id,
            profile,
        )))
    }
}
