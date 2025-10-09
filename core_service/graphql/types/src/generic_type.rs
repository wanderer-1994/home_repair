use async_graphql::{InputObject, InputType, OutputType, SimpleObject};
use paging::PagingOffsetInfo;

/// A wrapper input object around an `Option<T>`.
/// This is needed so that we can represent rust `Option<Option<T>>` in GraphQL,
///
/// Allowing clients to explicitly set a value, clear a value (by setting to null),
/// or leave the value unchanged (by omitting the field).
#[derive(Debug, InputObject)]
#[graphql(
    concrete(name = "SetValueString", params(String),),
    concrete(name = "SetValueInt32", params(i32))
)]
pub struct SetValue<T: InputType> {
    pub value: Option<T>,
}

#[derive(Debug, SimpleObject)]
pub struct PagingOffsetPayload<T: OutputType> {
    pub paging_info: PagingOffsetInfo,
    pub items: Vec<T>,
}

impl<T: OutputType> From<paging::PagingOffsetPayload<T>> for PagingOffsetPayload<T> {
    fn from(value: paging::PagingOffsetPayload<T>) -> Self {
        Self {
            paging_info: value.paging_info,
            items: value.items,
        }
    }
}
