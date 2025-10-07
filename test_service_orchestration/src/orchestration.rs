use crate::service_database::prepare_database;
use error::Result;
use test_utils::PostgresContainer;

#[derive(Debug, Default)]
pub struct ServiceParams {
    #[cfg(feature = "account_service")]
    pub account_service: crate::account_service::AccountServiceParams,
    #[cfg(feature = "core_service")]
    pub core_service: crate::core_service::CoreServiceParams,
}

pub struct ServiceEnvironment {
    pub _pg_container: PostgresContainer,
    #[cfg(feature = "account_service")]
    pub account_service: crate::account_service::AccountServiceEnvironment,
    #[cfg(feature = "core_service")]
    pub core_service: crate::core_service::CoreServiceEnvironment,
}

impl ServiceParams {
    pub async fn init(self) -> Result<ServiceEnvironment> {
        let pg_container = prepare_database().await?;

        #[cfg(feature = "account_service")]
        let account_service = crate::account_service::AccountServiceParamsInner {
            postgres_container: &pg_container,
        }
        .init()
        .await?;

        #[cfg(feature = "core_service")]
        let core_service = crate::core_service::CoreServiceParamsInner {
            postgres_container: &pg_container,
            account_service_client: account_service.service_client.clone(),
            features: self.core_service.features,
        }
        .init()
        .await?;

        Ok(ServiceEnvironment {
            _pg_container: pg_container,
            #[cfg(feature = "account_service")]
            account_service,
            #[cfg(feature = "core_service")]
            core_service,
        })
    }
}
