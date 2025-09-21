use async_graphql::dataloader::{DataLoader, HashMapCache, Loader};
use core_service_graphql_loader::CacheConfig;
use error::{Error, Result};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::Mutex;

pub struct OneLoader {
    load_count: Arc<Mutex<usize>>,
}

impl OneLoader {
    fn new() -> (Self, Arc<Mutex<usize>>) {
        let data = Arc::new(Mutex::new(0));
        (
            Self {
                load_count: Arc::clone(&data),
            },
            data,
        )
    }
}

impl Loader<i64> for OneLoader {
    type Error = Error;
    type Value = String;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, String>> {
        let mut data = self.load_count.lock().await;
        *data += 1;
        let result = keys
            .iter()
            .filter_map(|key| {
                let key = *key;
                if key == 1 {
                    return Some((key, "one".to_string()));
                }
                if key == 3 {
                    return Some((key, "three".to_string()));
                }
                None
            })
            .collect::<HashMap<_, _>>();
        Ok(result)
    }
}

pub struct AccountIdLoader(DataLoader<OneLoader, HashMapCache>);

impl Deref for AccountIdLoader {
    type Target = DataLoader<OneLoader, HashMapCache>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AccountIdLoader {
    pub fn new(loader: OneLoader, cache_config: CacheConfig) -> Self {
        let loader = DataLoader::with_cache(loader, tokio::spawn, HashMapCache::new());

        if matches!(cache_config, CacheConfig::NoCache) {
            loader.enable_all_cache(false);
        }

        Self(loader)
    }
}

#[tokio::test]
async fn load_in_batch() -> Result<()> {
    let (inner, counter) = OneLoader::new();
    let loader = AccountIdLoader::new(inner, CacheConfig::Cache);
    let data = loader.load_one(1).await?;
    assert_eq!(data.as_deref(), Some("one"));
    assert_eq!(*counter.lock().await, 1);

    let data = loader.load_one(2).await?;
    assert_eq!(data, None);
    assert_eq!(*counter.lock().await, 2);

    let data = loader.load_many([1, 2]).await?;
    assert_eq!(data.values().len(), 1);
    assert_eq!(data.values().next().unwrap(), "one");
    assert_eq!(*counter.lock().await, 3);

    let data = loader.load_many([1, 2, 3]).await?;
    // !IMPORTANT: load_many does not necessarily preserves ordering of results
    assert!(data.values().any(|v| v.eq("one")));
    assert!(data.values().any(|v| v.eq("three")));
    assert_eq!(*counter.lock().await, 4);

    Ok(())
}

#[tokio::test]
async fn load_from_cache() -> Result<()> {
    let (inner, counter) = OneLoader::new();
    let loader = AccountIdLoader::new(inner, CacheConfig::Cache);
    loader.load_one(1).await?;
    assert_eq!(*counter.lock().await, 1);

    loader.load_one(1).await?;
    assert_eq!(*counter.lock().await, 1);

    loader.load_one(3).await?;
    assert_eq!(*counter.lock().await, 2);

    loader.load_many([1, 3]).await?;
    assert_eq!(*counter.lock().await, 2);

    Ok(())
}

#[tokio::test]
async fn load_no_cache() -> Result<()> {
    let (inner, counter) = OneLoader::new();
    let loader = AccountIdLoader::new(inner, CacheConfig::NoCache);
    loader.load_one(1).await?;
    assert_eq!(*counter.lock().await, 1);

    loader.load_one(1).await?;
    assert_eq!(*counter.lock().await, 2);

    loader.load_one(3).await?;
    assert_eq!(*counter.lock().await, 3);

    loader.load_many([1, 3]).await?;
    assert_eq!(*counter.lock().await, 4);

    Ok(())
}
