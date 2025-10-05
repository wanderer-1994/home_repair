use async_graphql::Object;
use core_service_graphql_types::ServiceGroup;
use entity_type::ServiceLayer1;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct ServiceQuery;

#[Object]
impl ServiceQuery {
    async fn service_groups(&self) -> Vec<ServiceGroup> {
        ServiceLayer1::iter().map(ServiceGroup).collect()
    }
}
