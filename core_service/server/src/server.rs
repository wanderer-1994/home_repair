use crate::{
    AppState, ServiceSchema, config_types::HttpConfig, create_graphql_schema_extension,
    health_check,
};
use account_service_server::AccountService;
use async_graphql::http::{
    ALL_WEBSOCKET_PROTOCOLS, GraphQLPlaygroundConfig, graphiql_source, playground_source,
};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    Extension, Router,
    extract::WebSocketUpgrade,
    http::{HeaderName, HeaderValue, Method, header},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use core_service_graphql_context::{EnvironmentConfig, Features};
use core_service_graphql_loader::CacheConfig;
use db_utils::PgConnectionPool;
use error::{Error, Result};
use moka::future::Cache;
use search_service_server::SearchService;
use service_http::ACCESS_TOKEN_COOKIE_KEY;
use sms_sender::SmsSender;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Instrument, Span};

pub struct Server {
    pub db_connection_pool: PgConnectionPool,
    pub environment_config: Arc<EnvironmentConfig>,
    pub features: Features,
    pub http_config: HttpConfig,
    pub account_service_client: AccountService,
    pub search_service_client: SearchService,
    pub sms_sender: Arc<dyn SmsSender>,
    pub phone_pending_registration_cache: Arc<Cache<String, String>>,
}

impl Server {
    pub async fn serve(&self, server_socket: TcpListener) -> Result<()> {
        // Create middleware stacks
        let middleware = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(self.create_cors().expect("Invalid cors configuration"));

        let graphql_path = "/graphql";
        let subscriptions_path = "/subscriptions";

        let app = Router::new()
            .route("/health", get(health_check))
            .route(
                "/graphiql",
                get(async || Html(graphiql_source(graphql_path, Some(subscriptions_path)))),
            )
            .route(
                "/playground",
                get(async || {
                    Html(playground_source(
                        GraphQLPlaygroundConfig::new(graphql_path)
                            .subscription_endpoint(subscriptions_path),
                    ))
                }),
            )
            .route(
                graphql_path,
                post(graphql_handler)
                    // axum middleware are executed from bottom to top.
                    // See <https://docs.rs/axum/0.7.5/axum/middleware/index.html#ordering>
                    .layer(middleware::from_fn_with_state(
                        self.create_app_state(CacheConfig::Cache),
                        create_graphql_schema_extension,
                    )),
            )
            .route(
                subscriptions_path,
                get(graphql_subscriptions).layer(middleware::from_fn_with_state(
                    self.create_app_state(CacheConfig::NoCache),
                    create_graphql_schema_extension,
                )),
            )
            .layer(middleware)
            .into_make_service_with_connect_info::<SocketAddr>();

        tracing::info!("Server listening on {}", server_socket.local_addr()?.port());
        axum::serve(server_socket, app)
            .await
            .map_err(|e| Error::internal(format!("Failed to run axum server {e:?}")))?;

        Ok(())
    }

    fn create_app_state(&self, loader_cache_config: CacheConfig) -> AppState {
        AppState {
            db_pool: self.db_connection_pool.clone(),
            features: self.features,
            cookie_config: self.http_config.cookie_config.clone(),
            environment_config: Arc::clone(&self.environment_config),
            sms_sender: self.sms_sender.clone(),
            account_service_client: self.account_service_client.clone(),
            phone_pending_registration_cache: self.phone_pending_registration_cache.clone(),
            loader_cache_config,
        }
    }

    fn create_cors(&self) -> Result<CorsLayer> {
        let header_values = self
            .http_config
            .cors_origins
            .iter()
            .map(|origin| {
                // Validate origin format (must be absolute URL)
                if !origin.starts_with("http://") && !origin.starts_with("https://") {
                    return Err(Error::internal(format!(
                        "Invalid CORS origin format: {origin}",
                    )));
                }

                HeaderValue::from_str(origin).map_err(|e| {
                    tracing::error!("Failed to convert CORS origin to HeaderValue: {e}");
                    Error::internal("Failed to convert CORS origin to HeaderValue")
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([
                header::CONTENT_TYPE,
                HeaderName::from_static(ACCESS_TOKEN_COOKIE_KEY),
                HeaderName::from_static(header::REFERER.as_str()),
            ])
            .allow_credentials(true)
            .allow_origin(AllowOrigin::list(header_values)))
    }
}

/// axum handler that execute graphql request
async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
    Extension(user_context_span): Extension<Span>,
    req: GraphQLRequest,
) -> Response {
    let graphql_response = GraphQLResponse::from(
        schema
            .execute(req.into_inner())
            .instrument(user_context_span.clone())
            .await,
    );
    user_context_span.in_scope(|| maybe_log_batch_response_err(&graphql_response));

    graphql_response.into_response()
}

/// axum handler that execute graphql subscription request
pub async fn graphql_subscriptions(
    Extension(schema): Extension<ServiceSchema>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .max_frame_size(1024)
        .max_message_size(1024)
        .max_write_buffer_size(2048)
        .on_upgrade(move |stream| GraphQLWebSocket::new(stream, schema.clone(), protocol).serve())
}

/// If any of the batch response contains an error, logs an error event.
fn maybe_log_batch_response_err(response: &GraphQLResponse) {
    match &response.0 {
        async_graphql::BatchResponse::Single(response) => maybe_log_response_err(response),
        async_graphql::BatchResponse::Batch(responses) => {
            for response in responses.iter() {
                maybe_log_response_err(response);
            }
        }
    };
}

/// Logs an error event if the response is not ok.
fn maybe_log_response_err(response: &async_graphql::Response) {
    if !response.is_ok() {
        tracing::error!(error = ?response, "GraphQL protocol error")
    }
}
