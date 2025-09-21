use async_graphql::{ID, Object};
use core_service_graphql_types::{Node, parse_any_global_id};
use error::Result;

#[derive(Default)]
pub struct FooQuery;

#[Object]
impl FooQuery {
    async fn node(&self, id: ID) -> Result<Option<Node>> {
        parse_any_global_id(&id)
    }
}
