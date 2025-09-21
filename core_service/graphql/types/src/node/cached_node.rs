use serde::{Deserialize, Serialize};
use std::future::Future;
use tokio::sync::OnceCell;

/// Utility for implementing lazy loading of GraphQL object data. Typically it is used to back Node
/// graphql types which must be loadable by some ID.
///
/// Derives Serialize and Deserialize for use as a GlobalId Node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedNode<KeyT, DataT> {
    pub k: KeyT,
    #[serde(skip, default = "OnceCell::default")]
    data: OnceCell<DataT>,
}

impl<KeyT, DataT> CachedNode<KeyT, DataT> {
    pub fn new(k: KeyT) -> Self {
        Self {
            k,
            data: OnceCell::new(),
        }
    }

    pub fn new_with(k: KeyT, data: DataT) -> Self {
        Self {
            k,
            data: OnceCell::new_with(Some(data)),
        }
    }

    pub async fn get_or_load<LoadFn, Fut, ErrorT>(&self, load_fn: LoadFn) -> Result<&DataT, ErrorT>
    where
        LoadFn: Fn(&KeyT) -> Fut,
        Fut: Future<Output = Result<DataT, ErrorT>>,
    {
        self.data.get_or_try_init(|| load_fn(&self.k)).await
    }
}

impl<KeyT: Copy, DataT> CachedNode<KeyT, DataT> {
    pub fn inner_id(&self) -> KeyT {
        self.k
    }
}
