use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::config;

const POPUP_WIDTH: f64 = 200.0;
const POPUP_HEIGHT: f64 = 60.0;
const POPUP_LABEL: &str = "popup";

pub fn show(app: &AppHandle, text: &str, icon: &str) -> Result<(), String> {
    let cfg = config::load();
    let popup_cfg = &cfg.popup;

    if !popup_cfg.enabled {
        return Ok(());
    }

    // Close any existing popup
    if let Some(existing) = app.get_webview_window(POPUP_LABEL) {
        let _ = existing.close();
    }

    // Get cursor position to determine which monitor to use
    let (x, y) = get_popup_position(app, popup_cfg.margin_bottom)?;

    let url = format!(
        "/popup?text={}&icon={}&display_ms={}&fade_ms={}",
        urlencoding(text),
        urlencoding(icon),
        popup_cfg.display_ms,
        popup_cfg.fade_ms,
    );

    WebviewWindowBuilder::new(app, POPUP_LABEL, WebviewUrl::App(url.into()))
        .title("DeskForge Popup")
        .inner_size(POPUP_WIDTH, POPUP_HEIGHT)
        .position(x, y)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .transparent(true)
        .focused(false)
        .resizable(false)
        .build()
        .map_err(|e| format!("Failed to create popup window: {e}"))?;

    Ok(())
}

fn get_popup_position(app: &AppHandle, margin_bottom: u32) -> Result<(f64, f64), String> {
    // Get cursor position from the main window
    let main_window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    let cursor = main_window
        .cursor_position()
        .map_err(|e| format!("Failed to get cursor position: {e}"))?;

    // Find which monitor the cursor is on
    let monitor = main_window
        .monitor_from_point(cursor.x, cursor.y)
        .map_err(|e| format!("Failed to get monitor: {e}"))?
        .ok_or("No monitor found at cursor position")?;

    let monitor_pos = monitor.position().to_logical::<f64>(monitor.scale_factor());
    let monitor_size = monitor.size().to_logical::<f64>(monitor.scale_factor());

    // Center horizontally on monitor, position above bottom margin
    let x = monitor_pos.x + (monitor_size.width - POPUP_WIDTH) / 2.0;
    let y = monitor_pos.y + monitor_size.height - POPUP_HEIGHT - margin_bottom as f64;

    Ok((x, y))
}

/// Simple URL encoding for query params
fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '&' => "%26".to_string(),
            '=' => "%3D".to_string(),
            '+' => "%2B".to_string(),
            '#' => "%23".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
