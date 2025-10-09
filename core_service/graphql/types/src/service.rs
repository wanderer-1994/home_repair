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

    async fn recommended_rate_vnd(&self) -> Option<i32> {
        match self.0 {
            ServiceLayer2::AirConditionerFixing => None,
            ServiceLayer2::AirConditionerCleaning => Some(250_000),
            ServiceLayer2::WashingMachineFixing => None,
            ServiceLayer2::WashingMachineCleaning => Some(450_000),
            ServiceLayer2::Other => todo!(),
        }
    }
}
