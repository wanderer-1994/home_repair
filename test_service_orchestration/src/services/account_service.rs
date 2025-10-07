use crate::service_database::ACCOUNT_SERVICE_DATABASE;
use account_service_db as db;
use account_service_server::AccountService;
use account_service_server::AccountServiceContext;
use db_utils::PgConnectionPool;
use error::Result;
use jwt_signer::JwtSigner;
use random_util::Random;
use std::sync::Arc;
use test_utils::PostgresContainer;

#[derive(Debug, Default)]
pub struct AccountServiceParams {}

pub(crate) struct AccountServiceParamsInner<'a> {
    pub postgres_container: &'a PostgresContainer,
}

pub struct AccountServiceEnvironment {
    pub db_pool: PgConnectionPool,
    pub service_client: AccountService,
}

impl AccountServiceParamsInner<'_> {
    pub async fn init(self) -> Result<AccountServiceEnvironment> {
        let AccountServiceParamsInner { postgres_container } = self;

        // No need to run core service migration here because auth service already handle it.
        // Running migration twice causing error.
        let (db_pool, db_url) = postgres_container
            .db_connection(ACCOUNT_SERVICE_DATABASE)
            .await?;
        db::run_migrations(db_url).await?;

        let account_service_client = AccountService::new(AccountServiceContext {
            db_connection_pool: db_pool.clone(),
            jwt_signer: Arc::new(JwtSigner::new("my-super-secret")),
            random: Random::default(),
        });

        Ok(AccountServiceEnvironment {
            db_pool,
            service_client: account_service_client,
        })
    }
}
