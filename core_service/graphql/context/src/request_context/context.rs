use crate::{CookieConfig, EnvironmentConfig, Features};
use account_service_server::AccountService;
use actor_auth::Session;
use core_service_graphql_loader::{AccountLoaders, CacheConfig, SyncSessionContext};
use db_utils::PgConnectionPool;
use error::{Error, Result};
use std::{net::SocketAddr, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

pub struct ContextInternal {
    /// User session extracted from graphql request
    /// Can be mutable when user login, logout, reset password, etc.
    pub session_context: Arc<RwLock<Option<Arc<Session>>>>,
    /// Connection pool to database
    pub db_connection_pool: PgConnectionPool,
    /// Server feature flags
    pub features: Features,
    /// Configs specific to each deployment environment
    pub environment_config: Arc<EnvironmentConfig>,
    pub cookie_config: Arc<CookieConfig>,
    pub remote_addr: SocketAddr,
    pub account_service_client: Arc<AccountService>,
    pub account_loaders: AccountLoaders,
}

pub struct NewContextParams {
    pub session_context: Option<Session>,
    pub db_connection_pool: PgConnectionPool,
    pub features: Features,
    pub environment_config: Arc<EnvironmentConfig>,
    pub cookie_config: Arc<CookieConfig>,
    pub remote_addr: SocketAddr,
    pub account_service_client: Arc<AccountService>,
    pub cache_config: CacheConfig,
}

impl ContextInternal {
    pub fn new(
        NewContextParams {
            session_context,
            db_connection_pool,
            features,
            environment_config,
            cookie_config,
            remote_addr,
            account_service_client,
            cache_config,
        }: NewContextParams,
    ) -> Self {
        let session_context = Arc::new(RwLock::new(session_context.map(Arc::new)));
        Self {
            features,
            environment_config,
            cookie_config,
            remote_addr,
            account_service_client,
            account_loaders: AccountLoaders::new(
                db_connection_pool.clone(),
                SyncSessionContext::new(session_context.clone()),
                cache_config,
            ),
            session_context,
            db_connection_pool,
        }
    }
}

#[derive(Clone)]
/// GraphQL execution per-request context.
pub struct RequestContext(Arc<ContextInternal>);

impl Deref for RequestContext {
    type Target = ContextInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RequestContext {
    pub fn new(internal: ContextInternal) -> Self {
        Self(Arc::new(internal))
    }

    pub async fn try_session_context(&self) -> Result<Arc<Session>> {
        self.session_context
            .read()
            .await
            .as_ref()
            .map(Arc::clone)
            .ok_or_else(|| Error::unauthenticated("User is not authenticated"))
    }
}
