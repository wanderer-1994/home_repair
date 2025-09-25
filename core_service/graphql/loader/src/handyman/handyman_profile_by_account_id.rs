use crate::{CacheConfig, SyncSessionContext};
use account_service_db as acc_db;
use account_service_server::{AccountService, LoadHandymanProfileByIdsRequest};
use async_graphql::dataloader::{DataLoader, HashMapCache, Loader};
use entity_type::HandymanId;
use error::{Error, Result};
use std::{collections::HashMap, ops::Deref, sync::Arc};

#[doc(hidden)]
pub struct HandymanProfileByIdLoaderInner {
    account_service_client: AccountService,
    session_ctx: SyncSessionContext,
}

impl Loader<HandymanId> for HandymanProfileByIdLoaderInner {
    type Error = Error;
    type Value = Arc<acc_db::HandymanProfile>;

    async fn load(
        &self,
        keys: &[HandymanId],
    ) -> Result<HashMap<HandymanId, Arc<acc_db::HandymanProfile>>> {
        let session_ctx = self.session_ctx.try_session_context().await?;
        let batch = self
            .account_service_client
            .load_handyman_profile_by_ids(LoadHandymanProfileByIdsRequest {
                actor_auth: session_ctx.as_actor_auth(),
                account_ids: keys.to_vec(),
            })
            .await?;
        Ok(batch
            .profiles
            .into_iter()
            .map(|c| (c.handyman_id, Arc::new(c)))
            .collect())
    }
}

pub struct HandymanProfileByIdLoader(DataLoader<HandymanProfileByIdLoaderInner, HashMapCache>);

impl Deref for HandymanProfileByIdLoader {
    type Target = DataLoader<HandymanProfileByIdLoaderInner, HashMapCache>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HandymanProfileByIdLoader {
    pub fn new(
        account_service_client: AccountService,
        session_ctx: SyncSessionContext,
        cache_config: CacheConfig,
    ) -> Self {
        let loader = DataLoader::with_cache(
            HandymanProfileByIdLoaderInner {
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
