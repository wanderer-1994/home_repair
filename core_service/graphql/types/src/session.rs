use crate::{CachedNode, GlobalId};
use async_graphql::{ID, Object};
use error::Result;
use std::sync::Arc;

pub type Session = CachedNode<i64, Arc<String>>;

#[Object]
impl Session {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }
}
