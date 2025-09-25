use account_service_server::{AccountService, AccountServiceContext};
use clap::Parser;
use core_service_graphql_context::{
    CookieConfig as CookieConfigInner, EnvironmentConfig, Features, OTP_CODE_TTL_SECONDS,
};
use core_service_server::{
    Server,
    config_types::{HttpConfig, SameSiteConfig},
};
use jwt_signer::JwtSigner;
use moka::future::CacheBuilder;
use random_util::Random;
use serde::Deserialize;
use sms_sender::TerminalSmsSender;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, runtime::Builder};

#[derive(Parser, Debug)]
struct CmdArgs {
    #[arg(long)]
    /// Port on which to host server. While host is default to `0.0.0.0`.
    port: u16,

    /// Endpoint (DNS name or IP address) of the postgres db connection
    #[clap(long)]
    db_endpoint: String,

    /// Port for the postgres db.
    #[clap(long)]
    db_port: u16,

    /// Name of the postgres db.
    #[clap(long)]
    db_name: String,

    /// Username for postgres db connection.
    #[clap(long)]
    db_user: String,

    /// Password for postgres db connection.
    #[clap(long)]
    db_password: String,

    /// Dhall configuration file for [ServerConfig].
    #[clap(long)]
    config_file: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Dhall serde-able of [`CookieConfigInner`].
/// Note that the [`CookieConfigInner`] contains `cookie::SameSite` which is not serde-able.
struct CookieConfig {
    /// Is the server going through a secure transport? (e.g. HTTPS)
    use_https: bool,
    /// Apply same site `None` in cookies, if `false` default to `Strict`
    same_site: SameSiteConfig,
    /// Target domain of cookies
    cookie_domain: String,
}

impl From<CookieConfig> for CookieConfigInner {
    fn from(value: CookieConfig) -> Self {
        CookieConfigInner {
            use_https: value.use_https,
            same_site: value.same_site.into(),
            cookie_domain: value.cookie_domain,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerConfig {
    /// Config for setting session cookies
    pub cookie_config: CookieConfig,

    /// List of CORS origins that are whitelisted for this service.
    pub cors_origins: Vec<String>,

    /// Configurations specific to deployment environment
    pub environment_config: EnvironmentConfig,

    /// Configuration for core service feature flags
    pub features: Features,

    /// Secret for signing / verifying session JWT token
    pub jwt_secret: String,
}

async fn start_server() {
    let cmd_args = CmdArgs::parse();
    let config = serde_dhall::from_file(&cmd_args.config_file)
        .parse::<ServerConfig>()
        .expect("Failed to parse config");

    let db_params = db_utils::DbConnectionParams {
        user: &cmd_args.db_user,
        password: &cmd_args.db_password,
        endpoint: &cmd_args.db_endpoint,
        port: cmd_args.db_port,
        database_name: &cmd_args.db_name,
    };
    let db_url = db_params.url();
    share_service_schema::run_migrations(db_url.clone())
        .await
        .expect("Cannot run migrations");
    let db_connection_pool = db_utils::new_async_connection_pool(&db_url)
        .await
        .expect("Failed to establish postgres connection");
    let server_socket = create_tcp_listener(cmd_args.port).await;

    tracing::info!("Server listening on {}", cmd_args.port);
    let environment_config = Arc::new(config.environment_config);

    Server {
        db_connection_pool: db_connection_pool.clone(),
        environment_config,
        features: config.features,
        http_config: HttpConfig {
            cookie_config: Arc::new(CookieConfigInner::from(config.cookie_config)),
            cors_origins: config.cors_origins,
        },
        account_service_client: AccountService::new(AccountServiceContext {
            db_connection_pool,
            jwt_signer: Arc::new(JwtSigner::new(&config.jwt_secret)),
            random: Random::default(),
        }),
        // TODO (MVP): implement zalo SMS sender
        sms_sender: Arc::new(TerminalSmsSender),
        // TODO: replace with Redis cache. For MVP, temporary in-memory cache
        // with max 10_000 entries per 15 mins of TTL should be sufficient.
        phone_pending_registration_cache: Arc::new(
            CacheBuilder::new(10_000)
                .time_to_live(std::time::Duration::from_secs(OTP_CODE_TTL_SECONDS))
                .build(),
        ),
    }
    .serve(server_socket)
    .await
    .expect("Failed to run server");
}

fn main() {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Cannot create tokio runtime")
        .block_on(start_server());
}

async fn create_tcp_listener(port: u16) -> TcpListener {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to listen on {addr}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_configs() {
        for file in [
            "config/Local.dhall",
            // "config/Development.dhall",
            // "config/Staging.dhall",
            // "config/Production.dhall",
        ] {
            serde_dhall::from_file(file)
                .parse::<ServerConfig>()
                .expect("Failed to parse config");
        }
    }
}
