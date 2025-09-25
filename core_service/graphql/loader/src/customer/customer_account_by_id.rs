use crate::{CacheConfig, SyncSessionContext};
use account_service_db as acc_db;
use account_service_server::{AccountService, LoadCustomerAccountByIdsRequest};
use async_graphql::dataloader::{DataLoader, HashMapCache, Loader};
use entity_type::CustomerId;
use error::{Error, Result};
use std::{collections::HashMap, ops::Deref, sync::Arc};

#[doc(hidden)]
pub struct CustomerAccountByIdLoaderInner {
    account_service_client: AccountService,
    session_ctx: SyncSessionContext,
}

impl Loader<CustomerId> for CustomerAccountByIdLoaderInner {
    type Error = Error;
    type Value = Arc<acc_db::CustomerAccount>;

    async fn load(
        &self,
        keys: &[CustomerId],
    ) -> Result<HashMap<CustomerId, Arc<acc_db::CustomerAccount>>> {
        let session_ctx = self.session_ctx.try_session_context().await?;
        let batch = self
            .account_service_client
            .load_customer_account_by_ids(LoadCustomerAccountByIdsRequest {
                actor_auth: session_ctx.as_actor_auth(),
                account_ids: keys.to_vec(),
            })
            .await?;
        Ok(batch
            .customers
            .into_iter()
            .map(|c| (c.id, Arc::new(c)))
            .collect())
    }
}

pub struct CustomerAccountByIdLoader(DataLoader<CustomerAccountByIdLoaderInner, HashMapCache>);

impl Deref for CustomerAccountByIdLoader {
    type Target = DataLoader<CustomerAccountByIdLoaderInner, HashMapCache>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CustomerAccountByIdLoader {
    pub fn new(
        account_service_client: AccountService,
        session_ctx: SyncSessionContext,
        cache_config: CacheConfig,
    ) -> Self {
        let loader = DataLoader::with_cache(
            CustomerAccountByIdLoaderInner {
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
