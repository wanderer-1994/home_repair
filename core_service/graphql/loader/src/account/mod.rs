use crate::{CacheConfig, SyncSessionContext};
use db_utils::PgConnectionPool;

pub struct AccountLoaders {}

impl AccountLoaders {
    pub fn new(
        _db_pool: PgConnectionPool,
        _session_context: SyncSessionContext,
        _cache_config: CacheConfig,
    ) -> Self {
        Self {}
    }
}
