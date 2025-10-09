use crate::*;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(
    GeneralPurposeQuery,
    SessionQuery,
    ServiceQuery,
    HandymanDiscoveryQuery,
);
