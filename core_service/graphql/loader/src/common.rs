use actor_auth::Session;
use error::{Error, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SyncSessionContext(Arc<RwLock<Option<Arc<Session>>>>);

impl SyncSessionContext {
    pub fn new(inner: Arc<RwLock<Option<Arc<Session>>>>) -> Self {
        Self(inner)
    }

    pub async fn try_session_context(&self) -> Result<Arc<Session>> {
        let session_context = self.0.read().await;
        session_context
            .as_ref()
            .map(Arc::clone)
            .ok_or_else(|| Error::unauthenticated("Unauthenticated"))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CacheConfig {
    Cache,
    NoCache,
}
