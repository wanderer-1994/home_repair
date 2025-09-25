use crate::SignUpAndAuthMutation;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Mutation(SignUpAndAuthMutation);
