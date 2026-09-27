mod commands;
mod dto;
mod settings;
mod state;
mod store;

// Only used by the debug-only bindings export; gate the import so release
// builds don't warn about it.
#[cfg(all(debug_assertions, desktop))]
use specta_typescript::Typescript;
use state::AppState;
use tauri::Manager;
use tauri_specta::{Builder, collect_commands};

/// Where the generated TypeScript IPC bindings land. Anchored to the crate
/// directory, not the working directory — `tauri dev` and `cargo run` from the
/// workspace root have different cwds, and a relative path silently writes the
/// file into the wrong tree.
pub fn bindings_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/lib/bindings.ts")
}

/// The single definition of the IPC surface. A command missing from this list
/// exists in Rust but not on the frontend.
///
/// Shared with `src/bin/export_bindings.rs` so the bindings can be regenerated
/// without launching a window.
pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::get_account_info,
        commands::get_item,
        commands::get_items,
        commands::get_item_price,
        commands::search_recipes_by_output,
        commands::set_api_key,
        commands::get_api_key_status,
        commands::settings_get,
        commands::settings_save,
    ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gw2_companion=debug,gw2_api=debug".into()),
        )
        .init();

    let builder = specta_builder();

    // Writes into the source tree, which doesn't exist on a device — running
    // this on mobile panics and kills the app.
    #[cfg(all(debug_assertions, desktop))]
    builder
        .export(Typescript::default(), bindings_path())
        .expect("failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
                app.handle().plugin(tauri_plugin_process::init())?;
            }

            let app_state = AppState::new(app.handle())?;
            app.manage(app_state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
