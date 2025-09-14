use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool};
use diesel_async::scoped_futures::ScopedBoxFuture;
use error::{Error, Result};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

pub use diesel_async::AsyncPgConnection;

/// Convenient type alias for pg connection pool
pub type PgConnectionPool = Pool<AsyncPgConnection>;

/// Returns a [PgConnectionPool] from a given postgres connection string
pub async fn new_async_connection_pool(db_url: &str) -> Result<PgConnectionPool> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::builder(config)
        .build()
        .map_err(|e| Error::internal(format!("Cannot connect postgres {e:?}")))
}

#[derive(Debug)]
/// Utility, type-safe struct generating postgres connection string from components
pub struct DbConnectionParams<'a> {
    pub user: &'a str,
    pub password: &'a str,
    pub endpoint: &'a str,
    pub port: u16,
    pub database_name: &'a str,
}

impl DbConnectionParams<'_> {
    /// Returns postgres connection string
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            // Normalize password, since it can contains spaces and special characters
            utf8_percent_encode(self.password, NON_ALPHANUMERIC),
            self.endpoint,
            self.port,
            self.database_name,
        )
    }
}

/// Check whether one db row was affected as expected
pub fn check_one_row_affected(count: usize) -> Result<()> {
    match count {
        0 => Err(Error::not_found("Entity not found")),
        1 => Ok(()),
        _ => Err(Error::internal(format!(
            "'{}' rows affected but expected 1",
            count
        ))),
    }
}

/// Given an iterator of `(K, V)` that is sorted by `K`, transforms into a `Vec<(K, Vec<V>)>` with `V`'s
/// grouped by their corresponding `K`.
pub fn group_by<K, V, I>(it: I) -> Vec<(K, Vec<V>)>
where
    K: PartialEq + Clone,
    I: IntoIterator<Item = (K, V)>,
{
    let it = it.into_iter();
    // Get upper bound for size of the iterator.
    let mut results: Vec<(K, Vec<V>)> =
        it.size_hint().1.map(Vec::with_capacity).unwrap_or_default();

    // Perform manual group_by.
    let mut prev = None;
    for (k, v) in it {
        if Some(&k) == prev.as_ref() {
            results.last_mut().unwrap().1.push(v);
        } else {
            results.push((k.clone(), vec![v]));
            prev = Some(k);
        }
    }
    results
}

/// Execute a database operation within a transaction using an existing mutable connection
pub async fn with_mutable_db<'a, F, T>(db_connection_pool: &'a PgConnectionPool, f: F) -> Result<T>
where
    F: for<'b> FnOnce(&'b mut AsyncPgConnection) -> ScopedBoxFuture<'a, 'b, Result<T>> + Send,
    T: 'static,
{
    db_connection_pool
        .get()
        .await?
        .build_transaction()
        .read_write()
        .repeatable_read()
        .run(|conn| f(conn))
        .await
}

/// Execute a database operation within a read-only transaction
pub async fn with_readonly_db<'a, F, T>(db_connection_pool: &'a PgConnectionPool, f: F) -> Result<T>
where
    F: for<'b> FnOnce(&'b mut AsyncPgConnection) -> ScopedBoxFuture<'a, 'b, Result<T>> + Send,
    T: 'static,
{
    db_connection_pool
        .get()
        .await?
        .build_transaction()
        .read_only()
        .run(|conn| f(conn))
        .await
}
