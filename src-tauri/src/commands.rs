//! Tauri commands exposed to the frontend.
//!
//! Each command is an `async fn` returning `Result<T, String>` (Tauri convention).
//! Every command here is registered in `lib.rs`'s `collect_commands!` and
//! exported to `src/lib/bindings.ts` on debug builds — adding one without
//! registering it is a silent no-op on the frontend. gw2-api types never cross
//! IPC directly; they map into the views in `dto.rs`.

use tauri::State;

use gw2_api::{
    Gw2Client,
    endpoints::{
        account::Account,
        items::{Item, ItemId},
        recipes::RecipeId,
    },
};

use crate::dto::{AccountView, ItemPriceView, ItemView};
use crate::settings::AppSettings;
use crate::state::AppState;

pub async fn test_command(state: State<'_, AppState>) -> Result<String, String> {
    let client: Gw2Client<gw2_api::Authenticated> = state
        .auth_client
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or("No API key configured")?;

    let collection_singleton: Vec<gw2_api::endpoints::account::MaterialEntry> = client
        .account()
        .materials()
        .get()
        .await
        .map_err(|e| e.to_string())?;

    let singleton: Account = client.account().get().await.map_err(|e| e.to_string())?;

    let ressource: Item = client.items().get(19976).await.map_err(|e| e.to_string())?;

    let method: Vec<RecipeId> = client
        .recipes()
        .search()
        .output(19976)
        .await
        .map_err(|e| e.to_string())?;

    let item_id: ItemId = ItemId(19976);
    let id: u32 = item_id.0;

    let item: Item = item_id.get(&client).await.map_err(|e| e.to_string())?;

    let item: Item = client.items().get(id).await.map_err(|e| e.to_string())?;
    let skin_id: gw2_api::endpoints::skins::SkinId =
        item.default_skin.ok_or("Item has no default skin")?;
    let raw_skin_id: u32 = skin_id.0;
    let skin: gw2_api::endpoints::skins::Skin =
        skin_id.get(&client).await.map_err(|e| e.to_string())?;
    let skin_id = skin.id;

    Ok("Hello from Rust!".to_string())
}

/// Get account info (requires API key).
#[tauri::command]
#[specta::specta]
pub async fn get_account_info(state: State<'_, AppState>) -> Result<AccountView, String> {
    let client = state
        .auth_client
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or("No API key configured")?;
    client
        .account()
        .get()
        .await
        .map(Into::into)
        .map_err(|e| e.to_string())
}

/// Get a single item by ID.
#[tauri::command]
#[specta::specta]
pub async fn get_item(state: State<'_, AppState>, id: u32) -> Result<ItemView, String> {
    state
        .public_client
        .items()
        .get(id)
        .await
        .map(Into::into)
        .map_err(|e| e.to_string())
}

/// Get multiple items by ID.
#[tauri::command]
#[specta::specta]
pub async fn get_items(state: State<'_, AppState>, ids: Vec<u32>) -> Result<Vec<ItemView>, String> {
    state
        .public_client
        .items()
        .get_many(ids.into_iter().map(ItemId))
        .await
        .map(|items| items.into_iter().map(Into::into).collect())
        .map_err(|e| e.to_string())
}

/// Get trading post price for an item.
#[tauri::command]
#[specta::specta]
pub async fn get_item_price(state: State<'_, AppState>, id: u32) -> Result<ItemPriceView, String> {
    state
        .public_client
        .commerce()
        .prices()
        .get(id)
        .await
        .map(Into::into)
        .map_err(|e| e.to_string())
}

/// Search recipes that produce a given item.
#[tauri::command]
#[specta::specta]
pub async fn search_recipes_by_output(
    state: State<'_, AppState>,
    item_id: u32,
) -> Result<Vec<u32>, String> {
    state
        .public_client
        .recipes()
        .search()
        .output(item_id)
        .await
        .map(|ids| ids.into_iter().map(|id| id.0).collect())
        .map_err(|e| e.to_string())
}

/// Set (or update) the API key. Persists to DB.
#[tauri::command]
#[specta::specta]
pub async fn set_api_key(state: State<'_, AppState>, key: String) -> Result<bool, String> {
    // Validate by creating an authenticated client (sharing the public client's
    // rate limiter and cache) and testing it
    let client = state
        .public_client
        .authenticate(&key)
        .map_err(|e| e.to_string())?;

    // Test the key by fetching account info
    client
        .account()
        .get()
        .await
        .map_err(|e| format!("API key validation failed: {e}"))?;

    // Store in DB
    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.conn()
            .execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('api_key', ?1)",
                [&key],
            )
            .map_err(|e| e.to_string())?;
    }

    // Update the auth client
    {
        let mut auth = state.auth_client.lock().map_err(|e| e.to_string())?;
        *auth = Some(client);
    }

    Ok(true)
}

/// Check if an API key is configured.
#[tauri::command]
#[specta::specta]
pub async fn get_api_key_status(state: State<'_, AppState>) -> Result<bool, String> {
    let client = state.auth_client.lock().map_err(|e| e.to_string())?;
    Ok(client.is_some())
}

#[tauri::command]
#[specta::specta]
pub async fn settings_get(state: State<'_, AppState>) -> Result<AppSettings, String> {
    Ok(state.settings.lock().map_err(|e| e.to_string())?.clone())
}

/// Whole-snapshot save — the frontend merges its patch and sends everything.
#[tauri::command]
#[specta::specta]
pub async fn settings_save(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), String> {
    *state.settings.lock().map_err(|e| e.to_string())? = settings;
    state.persist_settings().map_err(|e| e.to_string())
}
