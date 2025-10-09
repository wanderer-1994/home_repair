use crate::{GlobalId, Service};
use async_graphql::{ID, Object, SimpleObject};
use core_service_db as db;
use entity_type::{HandymanServiceId, ServiceLayer1};
use error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct HandymanService {
    pub id: HandymanServiceId,
    #[serde(skip, default = "Option::default")]
    inner: Option<Arc<db::HandymanService>>,
}

impl HandymanService {
    pub fn new(inner: Arc<db::HandymanService>) -> Self {
        Self {
            id: inner.id,
            inner: Some(inner),
        }
    }

    fn get(&self) -> Result<&db::HandymanService> {
        self.inner
            .as_deref()
            .ok_or_else(|| Error::internal("HandymanService is initiated with non value"))
    }
}

#[Object]
impl HandymanService {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }

    async fn service(&self) -> Result<Service> {
        Ok(Service(self.get()?.service))
    }

    async fn note(&self) -> Result<Option<&str>> {
        Ok(self.get()?.note.as_deref())
    }

    async fn rate_vnd(&self) -> Result<Option<i32>> {
        Ok(self.get()?.rate_vnd)
    }
}

#[derive(SimpleObject)]
pub struct HandymanServiceGroup {
    group: ServiceLayer1,
    services: Vec<HandymanService>,
}

impl From<db::HandymanServiceGroup> for HandymanServiceGroup {
    fn from(db::HandymanServiceGroup { group, services }: db::HandymanServiceGroup) -> Self {
        HandymanServiceGroup {
            group,
            services: services
                .into_iter()
                .map(|e| HandymanService::new(Arc::new(e)))
                .collect(),
        }
    }
}
