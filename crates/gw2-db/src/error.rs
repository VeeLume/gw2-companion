//! Database error types.

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
