use crate::modules::keyboard::remapper;
use crate::popup;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconId},
    AppHandle, Manager,
};

fn capslock_label() -> String {
    let statuses = remapper::get_all_statuses();
    match statuses.iter().find(|s| s.id == "capslock-escape") {
        Some(s) if s.active => format!("⎋ CapsLock → {}", s.label),
        _ => "⇪ CapsLock → CapsLock".to_string(),
    }
}

/// Rebuilds the tray menu to reflect current state
fn rebuild_menu(app: &AppHandle) {
    let tray_id = TrayIconId::new("main");
    if let Some(tray) = app.tray_by_id(&tray_id) {
        if let Ok(menu) = build_menu(app) {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

fn build_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let caps_status =
        MenuItem::with_id(app, "caps-toggle", &capslock_label(), true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let show = MenuItem::with_id(app, "show", "Abrir DeskForge", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;
    Ok(Menu::with_items(app, &[&caps_status, &separator, &show, &quit])?)
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let menu = build_menu(app)?;

    TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("DeskForge")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "caps-toggle" => {
                if let Ok(status) = remapper::toggle("capslock-escape") {
                    let icon = status.icon.as_deref().unwrap_or("⌨");
                    let _ = popup::show(app, &status.label, icon);
                    rebuild_menu(app);
                }
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click { .. } = event {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

/// Called after any remap toggle to keep tray in sync
pub fn refresh(app: &AppHandle) {
    rebuild_menu(app);
}
