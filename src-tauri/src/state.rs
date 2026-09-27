//! Application state managed by Tauri.

use std::path::PathBuf;
use std::sync::Mutex;

use gw2_api::{Authenticated, Gw2Client, Unauthenticated};
use gw2_db::Database;

use crate::settings::AppSettings;
use crate::store;

/// Shared application state.
pub struct AppState {
    /// Tauri's per-app data dir, created at startup.
    pub data_dir: PathBuf,
    /// App preferences, persisted as `settings.json` in `data_dir`. The API key
    /// is not in here: it lives in the database's `settings` table.
    pub settings: Mutex<AppSettings>,
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

            stored_key.and_then(|key| public_client.authenticate(key).ok())
        };

        let settings = store::read_json(&app_dir.join(SETTINGS_FILE));

        Ok(Self {
            data_dir: app_dir,
            settings: Mutex::new(settings),
            db: Mutex::new(db),
            public_client,
            auth_client: Mutex::new(auth_client),
        })
    }

    /// Write the current settings snapshot to disk.
    pub fn persist_settings(&self) -> std::io::Result<()> {
        let snapshot = self
            .settings
            .lock()
            .map_err(|e| std::io::Error::other(e.to_string()))?
            .clone();
        store::write_json(&self.data_dir.join(SETTINGS_FILE), &snapshot)
    }
}

const SETTINGS_FILE: &str = "settings.json";

use tauri::Manager;
