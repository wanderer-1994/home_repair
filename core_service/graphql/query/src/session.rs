use async_graphql::{Context, Object};
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::Session;
use error::Result;

#[derive(Default)]
pub struct SessionQuery;

#[Object]
impl SessionQuery {
    #[tracing::instrument(skip(self, ctx))]
    async fn session(&self, ctx: &Context<'_>) -> Result<Option<Session>> {
        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await.ok();
        Ok(session_ctx.map(Session::new))
    }
}
