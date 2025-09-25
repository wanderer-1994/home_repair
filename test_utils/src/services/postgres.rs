use std::time::Duration;

use crate::PG_CONTAINER_TAG;
use db_utils::PgConnectionPool;
use diesel_migrations::EmbeddedMigrations;
use error::{Error, Result};
use testcontainers_modules::{
    postgres,
    testcontainers::{ImageExt, core::ContainerAsync, runners::AsyncRunner},
};
use uuid::Uuid;

pub type PgContainerAsync = ContainerAsync<postgres::Postgres>;

pub struct PostgresContainer {
    pub container: PgContainerAsync,
    pub host: String,
    pub port: u16,
    pub user_name: String,
    pub password: String,
}

impl PostgresContainer {
    pub async fn db_connection(&self, db_name: &str) -> Result<(PgConnectionPool, String)> {
        let db_url = db_utils::DbConnectionParams {
            user: &self.user_name,
            password: &self.password,
            endpoint: &self.host,
            port: self.port,
            database_name: db_name,
        }
        .url();

        let db_pool = db_utils::new_async_connection_pool(&db_url).await?;

        Ok((db_pool, db_url))
    }
}

pub async fn setup_postgres() -> Result<PostgresContainer> {
    let container_name = format!("postgres-{}", Uuid::now_v7());
    let container = postgres::Postgres::default()
        .with_container_name(container_name)
        .with_tag(PG_CONTAINER_TAG)
        // Increase timeout to prevent test from failing for slow PCs
        .with_startup_timeout(Duration::from_secs(60 * 60))
        .start()
        .await
        .map_err(|e| Error::internal(format!("Cannot start postgres container {e:?}")))?;

    let host = container
        .get_host()
        .await
        .map_err(|e| Error::internal(format!("Cannot get postgres host {e:?}")))?;
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .map_err(|e| Error::internal(format!("Cannot get postgres port {e:?}")))?;

    Ok(PostgresContainer {
        container,
        host: host.to_string(),
        port,
        user_name: String::from("postgres"),
        password: String::from("postgres"),
    })
}

/// Prepare postgres container for running a test
///
/// # Returns
///
/// - Container of the docker image
/// - Connection pool to postgres database
pub async fn setup_postgres_with_migrations(
    migrations: EmbeddedMigrations,
) -> Result<(PostgresContainer, PgConnectionPool)> {
    let container = setup_postgres().await?;
    let db_params = db_utils::DbConnectionParams {
        user: &container.user_name,
        password: &container.password,
        endpoint: &container.host,
        port: container.port,
        database_name: "postgres",
    };

    let db_url = db_params.url();
    let pool = db_utils::new_async_connection_pool(&db_url).await?;
    db_utils::run_migrations(db_url.clone(), migrations).await?;

    Ok((container, pool))
}
