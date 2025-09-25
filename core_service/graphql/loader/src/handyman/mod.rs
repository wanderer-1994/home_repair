use crate::{CacheConfig, SyncSessionContext};
use account_service_server::AccountService;

mod handyman_account_by_id;
pub use handyman_account_by_id::*;

mod handyman_profile_by_account_id;
pub use handyman_profile_by_account_id::*;

pub struct HandymanLoaders {
    pub account_by_id_loader: HandymanAccountByIdLoader,
    pub profile_by_id_loader: HandymanProfileByIdLoader,
}

impl HandymanLoaders {
    pub fn new(
        account_service_client: AccountService,
        session_ctx: SyncSessionContext,
        cache_config: CacheConfig,
    ) -> Self {
        Self {
            account_by_id_loader: HandymanAccountByIdLoader::new(
                account_service_client.clone(),
                session_ctx.clone(),
                cache_config,
            ),
            profile_by_id_loader: HandymanProfileByIdLoader::new(
                account_service_client,
                session_ctx,
                cache_config,
            ),
        }
    }
}
