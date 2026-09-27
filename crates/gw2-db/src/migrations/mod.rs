//! Schema migration system.
//!
//! Migrations are embedded SQL strings run in order. The `schema_version`
//! pragma tracks which migrations have been applied.

use crate::db::Database;
use crate::error::DbError;

/// All migrations, in order. Each entry is `(version, description, sql)`.
const MIGRATIONS: &[(u32, &str, &str)] = &[(1, "initial schema", include_str!("001_initial.sql"))];

/// Run all pending migrations.
pub fn run(db: &mut Database) -> Result<(), DbError> {
    let current_version: u32 = db
        .conn()
        .pragma_query_value(None, "user_version", |row| row.get(0))?;

    for &(version, description, sql) in MIGRATIONS {
        if version > current_version {
            tracing::info!("Running migration {version}: {description}");
            db.conn().execute_batch(sql)?;
            db.conn().pragma_update(None, "user_version", version)?;
        }
    }

    Ok(())
}
