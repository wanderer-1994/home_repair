use crate::{FooQuery, SessionQuery};
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(FooQuery, SessionQuery);
