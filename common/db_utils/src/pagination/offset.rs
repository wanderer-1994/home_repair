//! Implement offset pagination for diesel_async.
//! See user guide <https://diesel.rs/guides/extending-diesel.html#using-custom-sql-and-how-to-extend-the-query-dsl>

use diesel::{
    QueryResult,
    pg::Pg,
    query_builder::{AstPass, Query, QueryFragment, QueryId},
    sql_types,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, methods::LoadQuery};
use paging::PagingOffsetConfig;

pub trait PaginateOffset: Sized {
    fn paginate_offset(self, paging_config: PagingOffsetConfig) -> PaginatedOffset<Self>;
}

impl<T> PaginateOffset for T {
    fn paginate_offset(self, paging_config: PagingOffsetConfig) -> PaginatedOffset<Self> {
        PaginatedOffset {
            query: self,
            paging_config,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct PaginatedOffset<T> {
    query: T,
    paging_config: PagingOffsetConfig,
}

impl<T: Query> Query for PaginatedOffset<T> {
    type SqlType = (T::SqlType, sql_types::BigInt);
}

impl<T: Send> PaginatedOffset<T> {
    /// Load result of the query and count total matching records.
    /// Returns NOT_FOUND error if page_size overflow total matching records.
    ///
    /// NB: We need to manually express the lifetimes of this async fn to work around rustc's issue
    /// related to higher-rank lifetime evaluation for auto traits.
    /// See <https://github.com/rust-lang/rust/issues/102211>
    #[allow(clippy::manual_async_fn)]
    pub fn load_and_count_total<'query, 'conn, U>(
        self,
        conn: &'conn mut AsyncPgConnection,
    ) -> impl Future<Output = QueryResult<(Vec<U>, i64)>> + Send + 'conn
    where
        Self: LoadQuery<'query, AsyncPgConnection, (U, i64)> + 'query,
        U: Send,
        T: 'conn,
    {
        // Async block in non-async function so that we can be more expressive with the future
        // return type. This allows us to properly propagate the lifetimes.
        async {
            let results = self.load::<(U, i64)>(conn).await?;
            if results.is_empty() {
                return Err(diesel::result::Error::NotFound);
            }
            #[allow(clippy::get_first)]
            let total_count = results.get(0).map(|x| x.1).unwrap_or(0);
            let records = results.into_iter().map(|x| x.0).collect();
            Ok((records, total_count))
        }
    }
}

impl<T> QueryFragment<Pg> for PaginatedOffset<T>
where
    T: QueryFragment<Pg>,
{
    /// Run query and returns total count using COUNT(*) OVER() window function.
    /// The resulting query looks like
    /// ```sql
    /// SELECT *, COUNT(*) OVER () FROM (subselect t) as t LIMIT $1 OFFSET $2
    /// ```
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") as t LIMIT ");
        out.push_bind_param::<sql_types::BigInt, _>(&self.paging_config.page_size)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<sql_types::BigInt, _>(&self.paging_config.offset)?;
        Ok(())
    }
}
