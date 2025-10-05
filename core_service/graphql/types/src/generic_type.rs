use async_graphql::{InputObject, InputType};

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
