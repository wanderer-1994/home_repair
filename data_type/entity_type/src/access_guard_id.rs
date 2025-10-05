use crate::{CustomerId, HandymanId};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
/// A resource ID coupled with its owner's Handyman ID
/// This struct relies on the fact that resource ownership is often known at the call site.
pub struct HandymanAccessGuardId<T: Debug + Clone + Copy> {
    pub handyman_id: HandymanId,
    pub entity_id: T,
}

#[derive(Debug, Clone, Copy)]
/// A resource ID coupled with its owner's Customer ID
/// This struct relies on the fact that resource ownership is often known at the call site.
pub struct CustomerAccessGuardId<T: Debug + Clone + Copy> {
    pub customer_id: CustomerId,
    pub entity_id: T,
}
