//! App-global preferences: one JSON file, one snapshot struct, whole-snapshot
//! updates. Per-key setters can come back if the surface grows past what one
//! settings page edits.
//!
//! `#[serde(default)]` keeps old settings files loading after new fields are
//! added — never rename or repurpose a field, add a new one.
//!
//! Display preferences (theme, density) deliberately live in the frontend's
//! localStorage instead: they must apply before any IPC round-trip completes.
//! The API key lives in the database's `settings` table, validated on save by
//! `set_api_key`, not in this snapshot.

use serde::{Deserialize, Serialize};

// `Default` is derived while every field starts at its type default (a manual
// impl would trip `clippy::derivable_impls`). Swap to a manual impl the moment
// a field needs a different starting value.
#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(default)]
pub struct AppSettings {
    /// Whether first-launch onboarding has been completed or skipped.
    pub onboarding_completed: bool,
}

