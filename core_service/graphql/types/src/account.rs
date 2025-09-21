use crate::{CachedNode, GlobalId};
use async_graphql::{ID, Object};
use entity_type::CustomerId;
use error::Result;
use std::sync::Arc;

pub type Account = CachedNode<CustomerId, Arc<String>>;

#[Object]
impl Account {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }
}
