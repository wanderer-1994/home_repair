use async_graphql::{ID, Object};
use core_service_graphql_types::{Node, parse_any_global_id};
use error::{Error, Result};

#[derive(Default)]
pub struct GeneralPurposeQuery;

#[Object]
impl GeneralPurposeQuery {
    #[tracing::instrument(skip(self))]
    async fn node(&self, id: ID) -> Result<Option<Node>> {
        let node = parse_any_global_id(&id)?;
        match node {
            // Custom non-loadable nodes
            Some(Node::Session(_) | Node::HandymanService(_)) => Err(Error::invalid_argument(
                "Entity unsuported with \"node\" query",
            )),
            loadable_node => Ok(loadable_node),
        }
    }
}
