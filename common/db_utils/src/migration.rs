use diesel::Connection;
use diesel_async::{async_connection_wrapper::AsyncConnectionWrapper, pg::AsyncPgConnection};
use diesel_migrations::{EmbeddedMigrations, HarnessWithOutput, MigrationHarness};
use error::{Error, Result};

/// Apply pending migrations (if exist) over a database to keep database up-to-date.
///
/// TODO(huy): when possible, take a db connection as argument rather than establish
/// new connection with `db_url`. See issue <https://github.com/weiznich/diesel_async/issues/17>
pub async fn run_migrations(db_url: String, migrations: EmbeddedMigrations) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let mut connection = AsyncConnectionWrapper::<AsyncPgConnection>::establish(&db_url)
            .map_err(|e| Error::internal(format!("Cannot establish connection {e:?}")))?;

        let mut output = Vec::<u8>::new();
        let mut harness = HarnessWithOutput::new(&mut connection, &mut output);
        harness
            .run_pending_migrations(migrations)
            .map_err(|e| Error::internal(format!("Cannot run migrations {e:?}")))?;
        Ok::<_, Error>(())
    })
    .await
    .map_err(|e| Error::internal(format!("Tokio join handle error {e:?}")))??;

    Ok(())
}
