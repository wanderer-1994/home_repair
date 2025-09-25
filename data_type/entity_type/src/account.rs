use crate::define_graphql_enum;

tonic::include_proto!("account");

define_graphql_enum!(
    PgType = "text",
    AccountType #[doc = "Types of account"],
    Customer #[doc = "Person needs home repair service"],
    Handyman #[doc = "Person provides home repair service"],
);
