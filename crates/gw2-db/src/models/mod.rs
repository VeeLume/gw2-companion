//! Database-level model types and query helpers.
//!
//! These are the "repository" layer — translating between DB rows
//! and Rust structs. Keep API types in `gw2-api`; these are for
//! local persistence concerns (goals, checklists, cache metadata).

pub mod cache;
pub mod goals;
pub mod checklist;
