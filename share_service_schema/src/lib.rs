use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use error::Result;

mod schema;
pub use schema::*;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// Apply pending migrations (if exist) over core_service databaseS.
pub async fn run_migrations(db_url: String) -> Result<()> {
    db_utils::run_migrations(db_url, MIGRATIONS).await
}
