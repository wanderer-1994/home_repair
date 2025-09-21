use crate::FooMutation;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Mutation(FooMutation);
