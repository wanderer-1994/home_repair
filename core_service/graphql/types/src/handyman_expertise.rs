use crate::{GlobalId, Service};
use async_graphql::{ID, Object, SimpleObject};
use core_service_db as db;
use entity_type::{HandymanExpertiseId, ServiceLayer1};
use error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct HandymanExpertise {
    pub id: HandymanExpertiseId,
    #[serde(skip, default = "Option::default")]
    inner: Option<Arc<db::HandymanExpertise>>,
}

impl HandymanExpertise {
    pub fn new(inner: Arc<db::HandymanExpertise>) -> Self {
        Self {
            id: inner.id,
            inner: Some(inner),
        }
    }

    fn get(&self) -> Result<&db::HandymanExpertise> {
        self.inner
            .as_deref()
            .ok_or_else(|| Error::internal("HandymanExpertise is initiated with non value"))
    }
}

#[Object]
impl HandymanExpertise {
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
pub struct HandymanExpertiseGroup {
    group: ServiceLayer1,
    expertises: Vec<HandymanExpertise>,
}

impl From<db::HandymanExpertiseGroup> for HandymanExpertiseGroup {
    fn from(db::HandymanExpertiseGroup { group, expertises }: db::HandymanExpertiseGroup) -> Self {
        HandymanExpertiseGroup {
            group,
            expertises: expertises
                .into_iter()
                .map(|e| HandymanExpertise::new(Arc::new(e)))
                .collect(),
        }
    }
}
