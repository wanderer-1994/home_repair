use diesel_async::RunQueryDsl;
use error::Result;
use test_utils::PostgresContainer;

pub const SHARED_SERVICE_DATABASE: &str = "home_repair_share_database";

/// All services database, more will be added in future
const SERVICE_DATABASES: [&str; 1] = [SHARED_SERVICE_DATABASE];

pub async fn prepare_database() -> Result<PostgresContainer> {
    let postgres = test_utils::setup_postgres().await?;

    let db_params = db_utils::DbConnectionParams {
        user: &postgres.user_name,
        password: &postgres.password,
        endpoint: &postgres.host,
        port: postgres.port,
        database_name: "postgres",
    };
    let db_pool = db_utils::new_async_connection_pool(&db_params.url()).await?;
    let mut connection = db_pool.get().await?;

    for db_name in SERVICE_DATABASES {
        diesel::sql_query(format!("CREATE DATABASE {db_name}"))
            .execute(&mut connection)
            .await?;
    }

    Ok(postgres)
}
