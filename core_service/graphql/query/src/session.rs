use async_graphql::{Context, Object};
use core_service_graphql_types::Session;
use error::Result;

#[derive(Default)]
pub struct SessionQuery;

#[Object]
impl SessionQuery {
    async fn session(&self, _ctx: &Context<'_>) -> Result<Option<Session>> {
        todo!()
    }
}
