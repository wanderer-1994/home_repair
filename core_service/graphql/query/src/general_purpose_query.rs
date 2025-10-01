use async_graphql::{Context, ID, Object};
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::{Node, parse_any_global_id};
use error::Result;

#[derive(Default)]
pub struct GeneralPurposeQuery;

#[Object]
impl GeneralPurposeQuery {
    async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Node>> {
        let node = parse_any_global_id(&id)?;
        match node {
            // Custom handle for Session node which is non-loadable
            Some(Node::Session(session)) => {
                let context = ctx.data::<RequestContext>()?;
                let session_ctx = context.try_session_context().await.ok();
                let Some(session_ctx) = session_ctx else {
                    return Ok(None);
                };
                if session.key != session_ctx.actor_type.actor_key() {
                    return Ok(None);
                }
                Ok(None)
            }
            loadable_node => Ok(loadable_node),
        }
    }

    async fn test(&self) -> Result<Vec<String>> {
        Ok(vec!["foo".into(), "bar".into()])
    }
}
