use crate::service_database::CORE_SERVICE_DATABASE;
use account_service_server::AccountService;
use cookie::SameSite;
use core_service_db as db;
use core_service_graphql_context::CookieConfig;
use core_service_graphql_context::EnvironmentConfig;
use core_service_graphql_context::Features;
use core_service_graphql_context::OTP_CODE_TTL_SECONDS;
use core_service_server::Server;
use core_service_server::config_types::HttpConfig;
use db_utils::PgConnectionPool;
use error::{Error, Result};
use moka::future::CacheBuilder;
use search_service_server::SearchService;
use sms_sender::{TestSmsReceiver, TestSmsSender};
use std::sync::Arc;
use test_utils::PostgresContainer;

pub const TEST_ORIGIN: &str = "http://localhost:3000";

#[derive(Debug, Default)]
pub struct CoreServiceParams {
    pub features: Features,
}

pub(crate) struct CoreServiceParamsInner<'a> {
    pub postgres_container: &'a PostgresContainer,
    pub account_service_client: AccountService,
    pub search_service_client: SearchService,
    pub features: Features,
}

pub struct CoreServiceEnvironment {
    pub db_pool: PgConnectionPool,
    pub service_host: String,
    pub service_port: u16,
    pub sms_receiver: TestSmsReceiver,
}

impl CoreServiceParamsInner<'_> {
    pub async fn init(self) -> Result<CoreServiceEnvironment> {
        let CoreServiceParamsInner {
            postgres_container,
            account_service_client,
            search_service_client,
            features,
        } = self;

        // No need to run core service migration here because auth service already handle it.
        // Running migration twice causing error.
        let (db_pool, db_url) = postgres_container
            .db_connection(CORE_SERVICE_DATABASE)
            .await?;
        db::run_migrations(db_url).await?;

        let server_socket = test_utils::register_random_os_socket().await?;
        let local_addr = server_socket
            .local_addr()
            .map_err(|e| Error::internal(format!("Cannot get socket local address {e:?}")))?;
        let service_port = local_addr.port();

        let db_pool_cloned = db_pool.clone();
        let environment_config = Arc::new(EnvironmentConfig {
            frontend_host: TEST_ORIGIN.into(),
        });
        let (sms_sender, sms_receiver) = TestSmsSender::new();

        // Server will be dropped when tokio runtime is dropped
        tokio::spawn(async move {
            Server {
                db_connection_pool: db_pool_cloned.clone(),
                environment_config,
                features,
                http_config: HttpConfig {
                    cookie_config: Arc::new(CookieConfig {
                        use_https: false,
                        same_site: SameSite::None,
                        cookie_domain: "localhost".into(),
                    }),
                    cors_origins: vec![TEST_ORIGIN.to_string()],
                },
                account_service_client,
                search_service_client,
                sms_sender: Arc::new(sms_sender),
                phone_pending_registration_cache: Arc::new(
                    CacheBuilder::new(10_000)
                        .time_to_live(std::time::Duration::from_secs(OTP_CODE_TTL_SECONDS))
                        .build(),
                ),
            }
            .serve(server_socket)
            .await
            .expect("Failed to start server")
        });

        Ok(CoreServiceEnvironment {
            db_pool,
            service_host: local_addr.to_string(),
            service_port,
            sms_receiver,
        })
    }
}
