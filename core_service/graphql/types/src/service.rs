use async_graphql::Object;
use entity_type::{ServiceLayer1, ServiceLayer2};

pub struct ServiceGroup(pub ServiceLayer1);

#[Object]
impl ServiceGroup {
    async fn group_type(&self) -> ServiceLayer1 {
        self.0
    }

    async fn children(&self) -> Vec<Service> {
        self.0.layer2().iter().map(|s| Service(*s)).collect()
    }
}

pub struct Service(pub ServiceLayer2);

#[Object]
impl Service {
    async fn service_type(&self) -> ServiceLayer2 {
        self.0
    }

    async fn service_group(&self) -> ServiceGroup {
        ServiceGroup(self.0.layer1())
    }
}
