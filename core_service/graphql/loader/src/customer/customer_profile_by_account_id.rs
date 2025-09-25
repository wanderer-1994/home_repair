use crate::{CacheConfig, SyncSessionContext};
use account_service_db as acc_db;
use account_service_server::{AccountService, LoadCustomerProfileByIdsRequest};
use async_graphql::dataloader::{DataLoader, HashMapCache, Loader};
use entity_type::CustomerId;
use error::{Error, Result};
use std::{collections::HashMap, ops::Deref, sync::Arc};

#[doc(hidden)]
pub struct CustomerProfileByIdLoaderInner {
    account_service_client: AccountService,
    session_ctx: SyncSessionContext,
}

impl Loader<CustomerId> for CustomerProfileByIdLoaderInner {
    type Error = Error;
    type Value = Arc<acc_db::CustomerProfile>;

    async fn load(
        &self,
        keys: &[CustomerId],
    ) -> Result<HashMap<CustomerId, Arc<acc_db::CustomerProfile>>> {
        let session_ctx = self.session_ctx.try_session_context().await?;
        let batch = self
            .account_service_client
            .load_customer_profile_by_ids(LoadCustomerProfileByIdsRequest {
                actor_auth: session_ctx.as_actor_auth(),
                account_ids: keys.to_vec(),
            })
            .await?;
        Ok(batch
            .profiles
            .into_iter()
            .map(|c| (c.customer_id, Arc::new(c)))
            .collect())
    }
}

pub struct CustomerProfileByIdLoader(DataLoader<CustomerProfileByIdLoaderInner, HashMapCache>);

impl Deref for CustomerProfileByIdLoader {
    type Target = DataLoader<CustomerProfileByIdLoaderInner, HashMapCache>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CustomerProfileByIdLoader {
    pub fn new(
        account_service_client: AccountService,
        session_ctx: SyncSessionContext,
        cache_config: CacheConfig,
    ) -> Self {
        let loader = DataLoader::with_cache(
            CustomerProfileByIdLoaderInner {
                account_service_client,
                session_ctx,
            },
            tokio::spawn,
            HashMapCache::new(),
        );

        if matches!(cache_config, CacheConfig::NoCache) {
            loader.enable_all_cache(false);
        }

        Self(loader)
    }
}
