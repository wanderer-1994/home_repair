use async_graphql::{Object, SimpleObject, Union};
use entity_type::{ServiceLayer1, ServiceLayer2};

pub struct ServiceGroup(pub ServiceLayer1);

#[Object]
impl ServiceGroup {
    async fn group_type(&self) -> ServiceLayer1 {
        self.0
    }

    async fn children(&self) -> Vec<Service> {
        self.0.layer2().iter().map(|s| Service::from(*s)).collect()
    }
}

#[derive(Union)]
pub enum Service {
    AirConditionerFixing(ServiceAirConditionerFixing),
    AirConditionerCleaning(ServiceAirConditionerCleaning),
    WashingMachineFixing(ServiceWashingMachineFixing),
    WashingMachineCleaning(ServiceWashingMachineCleaning),
    Other(ServiceOther),
}

#[derive(Default, SimpleObject)]
pub struct ServiceAirConditionerFixing {
    foo: bool,
}

#[derive(Default, SimpleObject)]
pub struct ServiceAirConditionerCleaning {
    foo: bool,
}

#[derive(Default, SimpleObject)]
pub struct ServiceWashingMachineFixing {
    foo: bool,
}

#[derive(Default, SimpleObject)]
pub struct ServiceWashingMachineCleaning {
    foo: bool,
}

#[derive(Default, SimpleObject)]
pub struct ServiceOther {
    foo: bool,
}

impl<'a> From<&'a Service> for ServiceLayer2 {
    fn from(value: &'a Service) -> Self {
        match value {
            Service::AirConditionerFixing(_) => ServiceLayer2::AirConditionerFixing,
            Service::AirConditionerCleaning(_) => ServiceLayer2::AirConditionerCleaning,
            Service::WashingMachineFixing(_) => ServiceLayer2::WashingMachineFixing,
            Service::WashingMachineCleaning(_) => ServiceLayer2::WashingMachineCleaning,
            Service::Other(_) => ServiceLayer2::Other,
        }
    }
}

impl From<ServiceLayer2> for Service {
    fn from(value: ServiceLayer2) -> Self {
        match value {
            ServiceLayer2::AirConditionerFixing => {
                Service::AirConditionerFixing(ServiceAirConditionerFixing::default())
            }
            ServiceLayer2::AirConditionerCleaning => {
                Service::AirConditionerCleaning(ServiceAirConditionerCleaning::default())
            }
            ServiceLayer2::WashingMachineFixing => {
                Service::WashingMachineFixing(ServiceWashingMachineFixing::default())
            }
            ServiceLayer2::WashingMachineCleaning => {
                Service::WashingMachineCleaning(ServiceWashingMachineCleaning::default())
            }
            ServiceLayer2::Other => Service::Other(ServiceOther::default()),
        }
    }
}
