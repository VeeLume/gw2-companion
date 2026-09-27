//! Application state managed by Tauri.

use std::sync::Mutex;

use gw2_api::{Authenticated, Gw2Client, Unauthenticated};
use gw2_db::Database;

/// Shared application state.
pub struct AppState {
    pub db: Mutex<Database>,
    pub public_client: Gw2Client<Unauthenticated>,
    pub auth_client: Mutex<Option<Gw2Client<Authenticated>>>,
}

impl AppState {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        // Store the DB next to the app's data directory
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("failed to get app data dir");
        std::fs::create_dir_all(&app_dir)?;
        let db_path = app_dir.join("gw2-companion.db");

        let db = Database::open(&db_path)?;
        let public_client = Gw2Client::new();

        // Try to restore API key from DB
        let auth_client = {
            let stored_key: Option<String> = db
                .conn()
                .query_row(
                    "SELECT value FROM settings WHERE key = 'api_key'",
                    [],
                    |row| row.get(0),
                )
                .ok();

            stored_key.and_then(|key| Gw2Client::builder().api_key(key).build().ok())
        };

        Ok(Self {
            db: Mutex::new(db),
            public_client,
            auth_client: Mutex::new(auth_client),
        })
    }
}

use tauri::Manager;
