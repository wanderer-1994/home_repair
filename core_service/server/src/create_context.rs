use crate::{create_schema, extract_session, make_user_context_span};
use account_service_server::AccountService;
use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use core_service_graphql_context::{
    ContextInternal, CookieConfig, EnvironmentConfig, Features, NewContextParams, RequestContext,
};
use core_service_graphql_loader::CacheConfig;
use db_utils::PgConnectionPool;
use moka::future::Cache;
use search_service_server::SearchService;
use sms_sender::SmsSender;
use std::{net::SocketAddr, sync::Arc};

#[derive(Clone)]
/// Persisted state of server, mostly contains 3rd-party service connections.
pub struct AppState {
    pub db_pool: PgConnectionPool,
    pub features: Features,
    pub cookie_config: Arc<CookieConfig>,
    pub environment_config: Arc<EnvironmentConfig>,
    pub sms_sender: Arc<dyn SmsSender>,
    pub loader_cache_config: CacheConfig,
    pub account_service_client: AccountService,
    pub search_service_client: SearchService,
    pub phone_pending_registration_cache: Arc<Cache<String, String>>,
}

/// Middleware that extracts user session, creates graphql schema and binds schema to axum request extensions.
pub async fn create_graphql_schema_extension(
    State(app_state): State<AppState>,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
    mut req: Request,
    next: Next,
) -> Response {
    let header_map = req.headers();
    let session_context = extract_session(header_map, &app_state.account_service_client).await;

    let request_context = RequestContext::new(ContextInternal::new(NewContextParams {
        session_context,
        db_connection_pool: app_state.db_pool,
        features: app_state.features,
        environment_config: app_state.environment_config,
        cookie_config: app_state.cookie_config,
        remote_addr,
        account_service_client: app_state.account_service_client,
        search_service_client: app_state.search_service_client,
        sms_sender: app_state.sms_sender,
        phone_pending_registration_cache: app_state.phone_pending_registration_cache,
        loader_cache_config: app_state.loader_cache_config,
    }));

    let user_context_span = make_user_context_span(&request_context).await;
    let schema = create_schema(crate::CreateSchemaOption::WithContext(request_context));
    req.extensions_mut().insert(user_context_span);
    req.extensions_mut().insert(schema);

    next.run(req).await
}
