use async_graphql::{Context, ID, Object};
use core_service_graphql_context::RequestContext;
use error::Result;
use std::sync::Arc;

use crate::{CachedNode, GlobalId};

pub mod db {
    #[derive(Debug)]
    pub struct FooEntity {
        pub id: i64,
    }
}

pub type FooNode = CachedNode<i64, Arc<db::FooEntity>>;

impl FooNode {
    async fn load(id: i64, _context: &RequestContext) -> Result<Arc<db::FooEntity>> {
        // Mock load from database
        Ok(Arc::new(db::FooEntity { id }))
    }

    async fn get(&self, ctx: &Context<'_>) -> Result<&Arc<db::FooEntity>> {
        let context = ctx.data::<RequestContext>()?;
        self.get_or_load(|id| Self::load(*id, context)).await
    }
}

#[Object]
impl FooNode {
    pub async fn id(&self, ctx: &Context<'_>) -> Result<ID> {
        let _node = self.get(ctx).await?;
        self.as_global_id()
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BarNode {
    id: i64,
}

impl BarNode {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

#[Object]
impl BarNode {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }
}
