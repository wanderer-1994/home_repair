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
use testcontainers::{core::{IntoContainerPort, WaitFor}, runners::SyncRunner, GenericImage, ImageExt};

pub type PgContainerAsync = ContainerAsync<postgres::Postgres>

pub struct PostGisContainer {
    pub container: PgContainerAsync,
    pub host: String,
    pub port: u16,
    pub user_name: String,
    pub password: String,
}

pub async fn setup_postgis() -> Result<PostGisContainer> {
    let container_name = format!("self_built_postgis-{}", Uuid::now_v7());
    let container = GenericImage::new("self_built_postgis", "1")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .start()
        .expect("Failed to start Redis");

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
