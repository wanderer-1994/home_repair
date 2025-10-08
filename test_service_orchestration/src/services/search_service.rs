use crate::service_database::SEARCH_SERVICE_DATABASE;
use db_utils::PgConnectionPool;
use error::Result;
use search_service_db as db;
use search_service_server::{SearchService, SearchServiceContext};
use test_utils::PostgresContainer;

#[derive(Debug, Default)]
pub struct SearchServiceParams {}

pub(crate) struct SearchServiceParamsInner<'a> {
    pub postgres_container: &'a PostgresContainer,
}

pub struct SearchServiceEnvironment {
    pub db_pool: PgConnectionPool,
    pub service_client: SearchService,
}

impl SearchServiceParamsInner<'_> {
    pub async fn init(self) -> Result<SearchServiceEnvironment> {
        let SearchServiceParamsInner { postgres_container } = self;

        // No need to run core service migration here because auth service already handle it.
        // Running migration twice causing error.
        let (db_pool, db_url) = postgres_container
            .db_connection(SEARCH_SERVICE_DATABASE)
            .await?;
        db::run_migrations(db_url).await?;

        let search_service_client = SearchService::new(SearchServiceContext {
            db_connection_pool: db_pool.clone(),
        });

        Ok(SearchServiceEnvironment {
            db_pool,
            service_client: search_service_client,
        })
    }
}
