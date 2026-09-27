//! Database connection management.

use std::path::Path;

use rusqlite::Connection;

use crate::error::DbError;
use crate::migrations;

/// Main database handle.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open (or create) the database at the given path and run migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;

        // Enable WAL mode for better concurrent read performance
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        let mut db = Self { conn };
        migrations::run(&mut db)?;
        Ok(db)
    }

    /// Open an in-memory database (for testing).
    pub fn in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        let mut db = Self { conn };
        migrations::run(&mut db)?;
        Ok(db)
    }

    /// Get a reference to the underlying connection.
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Get a mutable reference to the underlying connection.
    pub fn conn_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}
