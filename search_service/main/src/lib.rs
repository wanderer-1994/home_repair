use error::Result;
use search_service_db as db;
use search_service_server::{SearchService, SearchServiceContext};

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

pub async fn start_server(cmd_args: CmdArgs) -> Result<SearchService> {
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

    let service = SearchService::new(SearchServiceContext { db_connection_pool });

    Ok(service)
}
