use async_graphql::{Context, InputObject, Object, SimpleObject};
use core_service_graphql_context::RequestContext;
use error::Result;

#[derive(Default)]
pub struct FooMutation;

#[Object]
impl FooMutation {
    #[tracing::instrument(skip_all, fields(foo = input.foo.as_str()))]
    async fn foo_mutation(
        &self,
        ctx: &Context<'_>,
        input: FooMutationInput,
    ) -> Result<FooMutationPayload> {
        let _context = ctx.data::<RequestContext>()?;
        Ok(FooMutationPayload {
            bar: format!("Hello {}", input.foo),
        })
    }
}

#[derive(InputObject)]
struct FooMutationInput {
    foo: String,
}

#[derive(SimpleObject)]
struct FooMutationPayload {
    bar: String,
}
