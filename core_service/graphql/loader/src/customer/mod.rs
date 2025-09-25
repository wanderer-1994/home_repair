use crate::{CacheConfig, SyncSessionContext};
use account_service_server::AccountService;

mod customer_account_by_id;
pub use customer_account_by_id::*;

mod customer_profile_by_account_id;
pub use customer_profile_by_account_id::*;

pub struct CustomerLoaders {
    pub account_by_id_loader: CustomerAccountByIdLoader,
    pub profile_by_id_loader: CustomerProfileByIdLoader,
}

impl CustomerLoaders {
    pub fn new(
        account_service_client: AccountService,
        session_ctx: SyncSessionContext,
        cache_config: CacheConfig,
    ) -> Self {
        Self {
            account_by_id_loader: CustomerAccountByIdLoader::new(
                account_service_client.clone(),
                session_ctx.clone(),
                cache_config,
            ),
            profile_by_id_loader: CustomerProfileByIdLoader::new(
                account_service_client,
                session_ctx,
                cache_config,
            ),
        }
    }
}
