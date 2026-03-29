mod config;
mod modules;
mod popup;
mod shortcuts;
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
fn toggle_remap(app: tauri::AppHandle, remap_id: String) -> Result<remapper::RemapStatus, String> {
    let status = remapper::toggle(&remap_id)?;

    let icon = status.icon.as_deref().unwrap_or("⌨");
    let _ = popup::show(&app, &status.label, icon);
    tray::refresh(&app);

    Ok(status)
}

#[tauri::command]
fn add_macro(
    app: tauri::AppHandle,
    id: String,
    name: String,
    trigger: String,
    text: String,
    method: String,
) -> Result<(), String> {
    let mut cfg = config::load();

    // Check for duplicate ID
    if cfg.keyboard.macros.iter().any(|m| m.id == id) {
        return Err(format!("Macro with id '{id}' already exists"));
    }

    cfg.keyboard.macros.push(config::TextMacro {
        id,
        name,
        trigger,
        text,
        method,
    });
    config::save(&cfg);
    shortcuts::register_all(&app);
    Ok(())
}

#[tauri::command]
fn update_macro(
    app: tauri::AppHandle,
    id: String,
    name: String,
    trigger: String,
    text: String,
    method: String,
) -> Result<(), String> {
    let mut cfg = config::load();

    let mac = cfg
        .keyboard
        .macros
        .iter_mut()
        .find(|m| m.id == id)
        .ok_or_else(|| format!("Macro '{id}' not found"))?;

    mac.name = name;
    mac.trigger = trigger;
    mac.text = text;
    mac.method = method;

    config::save(&cfg);
    shortcuts::register_all(&app);
    Ok(())
}

#[tauri::command]
fn delete_macro(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let mut cfg = config::load();
    let before = cfg.keyboard.macros.len();
    cfg.keyboard.macros.retain(|m| m.id != id);

    if cfg.keyboard.macros.len() == before {
        return Err(format!("Macro '{id}' not found"));
    }

    config::save(&cfg);
    shortcuts::register_all(&app);
    Ok(())
}

#[tauri::command]
fn get_macros() -> Vec<config::TextMacro> {
    config::load().keyboard.macros
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(shortcuts::handle_shortcut)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            get_remap_statuses,
            toggle_remap,
            add_macro,
            update_macro,
            delete_macro,
            get_macros,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tray::setup(&handle)?;
            shortcuts::register_all(&handle);

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
