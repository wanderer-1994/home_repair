use crate::{GeneralPurposeQuery, SessionQuery};
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(GeneralPurposeQuery, SessionQuery);
