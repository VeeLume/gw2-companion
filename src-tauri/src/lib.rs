mod commands;
mod state;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gw2_companion=debug,gw2_api=debug".into()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_state = AppState::new(app.handle())?;
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_account_info,
            commands::get_item,
            commands::get_items,
            commands::get_item_price,
            commands::search_recipes_by_output,
            commands::set_api_key,
            commands::get_api_key_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
