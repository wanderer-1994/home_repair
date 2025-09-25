use crate::service_database::prepare_database;
use error::Result;
use test_utils::PostgresContainer;

#[derive(Debug, Default)]
pub struct ServiceParams {
    pub core_service: crate::core_service::CoreServiceParams,
}

pub struct ServiceEnvironment {
    pub _pg_container: PostgresContainer,
    pub core_service: crate::core_service::CoreServiceEnvironment,
}

impl ServiceParams {
    pub async fn init(self) -> Result<ServiceEnvironment> {
        let _pg_container = prepare_database().await?;

        let core_service = crate::core_service::CoreServiceParamsInner {
            postgres_container: &_pg_container,
            features: self.core_service.features,
        }
        .init()
        .await?;

        Ok(ServiceEnvironment {
            _pg_container,
            core_service,
        })
    }
}
