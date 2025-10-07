use account_service_db as db;
use account_service_server::{AccountService, AccountServiceContext};
use error::Result;
use jwt_signer::JwtSigner;
use random_util::Random;
use std::sync::Arc;

#[derive(Debug)]
pub struct CmdArgs {
    /// Endpoint (DNS name or IP address) of the postgres db connection
    pub db_endpoint: String,

    /// Port for the postgres db.
    pub db_port: u16,

    /// Name of the postgres db.
    pub db_name: String,

    /// Username for postgres db connection.
    pub db_user: String,

    /// Password for postgres db connection.
    pub db_password: String,
}

#[derive(Debug)]
pub struct ServerConfig {
    /// Secret for signing / verifying session JWT token
    pub jwt_secret: String,
}

pub async fn start_server(
    cmd_args: CmdArgs,
    server_config: ServerConfig,
) -> Result<AccountService> {
    let db_params = db_utils::DbConnectionParams {
        user: &cmd_args.db_user,
        password: &cmd_args.db_password,
        endpoint: &cmd_args.db_endpoint,
        port: cmd_args.db_port,
        database_name: &cmd_args.db_name,
    };
    let db_url = db_params.url();
    db::run_migrations(db_url.clone()).await?;
    let db_connection_pool = db_utils::new_async_connection_pool(&db_url).await?;

    let service = AccountService::new(AccountServiceContext {
        db_connection_pool,
        jwt_signer: Arc::new(JwtSigner::new(&server_config.jwt_secret)),
        random: Random::default(),
    });

    Ok(service)
}
