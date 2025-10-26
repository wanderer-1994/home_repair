use crate::*;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Mutation(
    SignUpAndAuthMutation,
    OnboardingHandymanMutation,
    CustomerCreateTaskMutation,
);
