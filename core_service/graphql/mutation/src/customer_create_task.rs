use async_graphql::{Context, InputObject, Object, SimpleObject};
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::ScheduleInput;
use entity_type::ServiceLayer2;
use error::Result;

#[derive(Default)]
pub struct CustomerCreateTaskMutation;

#[Object]
impl CustomerCreateTaskMutation {
    #[tracing::instrument(skip(self, ctx))]
    async fn customer_create_task(
        &self,
        ctx: &Context<'_>,
        input: CustomerCreateTaskInput,
    ) -> Result<CustomerCreateTaskPayload> {
        let context = ctx.data::<RequestContext>()?;
        let session_ctx = context.try_session_context().await?;
        let _actor_auth = session_ctx.as_actor_auth();

        Ok(CustomerCreateTaskPayload { foo: true })
    }
}

#[derive(Debug, InputObject)]
struct CustomerCreateTaskInput {
    service: ServiceLayer2,
    /// Plain text title
    title: String,
    /// Markdown note
    note: String,
    /// Schedule for the task
    schedule: ScheduleInput,
}

#[derive(SimpleObject)]
struct CustomerCreateTaskPayload {
    foo: bool,
}
