//! # gw2-db
//!
//! SQLite persistence layer for the GW2 Companion app.
//! Handles cached API data, user goals/checklists, and schema migrations.

pub mod db;
pub mod error;
pub mod migrations;
pub mod models;

pub use db::Database;
pub use error::DbError;
