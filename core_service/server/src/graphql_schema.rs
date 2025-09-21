use async_graphql::{EmptySubscription, Schema};
use core_service_graphql_context::RequestContext;
use core_service_graphql_mutation::Mutation;
use core_service_graphql_query::Query;
use core_service_graphql_types::Node;

/// Service GraphQL schema
pub type ServiceSchema = Schema<Query, Mutation, EmptySubscription>;

pub enum CreateSchemaOption {
    /// Use this for sdl export purpose (i.e. gen `schema.graphql` file).
    NoContext,
    /// Use this for creating executable schema.
    WithContext(RequestContext),
}

/// Create an instance of [ServiceSchema]
pub fn create_schema(option: CreateSchemaOption) -> ServiceSchema {
    let builder = ServiceSchema::build(Query::default(), Mutation::default(), EmptySubscription)
        .register_output_type::<Node>();
    match option {
        CreateSchemaOption::NoContext => builder.finish(),
        CreateSchemaOption::WithContext(context) => builder.data(context).finish(),
    }
}
