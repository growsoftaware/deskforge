mod config;
mod modules;
mod tray;

use modules::keyboard::remapper;
use tauri::Manager;

#[tauri::command]
fn get_config() -> config::Config {
    config::load()
}

#[tauri::command]
fn save_config(cfg: config::Config) {
    config::save(&cfg);
}

#[tauri::command]
fn get_remap_statuses() -> Vec<remapper::RemapStatus> {
    remapper::get_all_statuses()
}

#[tauri::command]
fn toggle_remap(remap_id: String) -> Result<remapper::RemapStatus, String> {
    remapper::toggle(&remap_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            get_remap_statuses,
            toggle_remap,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tray::setup(&handle)?;

            // Hide window on close instead of quitting (minimize to tray)
            let window = app.get_webview_window("main").unwrap();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Some(w) = handle.get_webview_window("main") {
                        let _ = w.hide();
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
