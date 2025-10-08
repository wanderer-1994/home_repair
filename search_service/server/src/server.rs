use db_utils::PgConnectionPool;

#[derive(Clone)]
pub struct SearchServiceContext {
    pub db_connection_pool: PgConnectionPool,
}

#[derive(Clone)]
pub struct SearchService {
    pub(crate) context: SearchServiceContext,
}

impl SearchService {
    pub fn new(context: SearchServiceContext) -> Self {
        Self { context }
    }
}
