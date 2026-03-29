use std::thread;
use std::time::Duration;
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

    // Close any existing popup first, with a small delay to avoid X11 race
    if let Some(existing) = app.get_webview_window(POPUP_LABEL) {
        let _ = existing.close();
        thread::sleep(Duration::from_millis(100));
    }

    let (x, y) = get_popup_position(app, popup_cfg.margin_bottom);

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
        .resizable(false)
        .build()
        .map_err(|e| format!("Failed to create popup window: {e}"))?;

    Ok(())
}

fn get_popup_position(app: &AppHandle, margin_bottom: u32) -> (f64, f64) {
    // Try to get cursor position from any available window
    let cursor_pos = app
        .get_webview_window("main")
        .and_then(|w| w.cursor_position().ok())
        .or_else(|| {
            // If main window is hidden, try to get position via xdotool
            std::process::Command::new("xdotool")
                .args(["getmouselocation", "--shell"])
                .output()
                .ok()
                .and_then(|out| {
                    let s = String::from_utf8_lossy(&out.stdout);
                    let x = parse_xdotool_var(&s, "X=")?;
                    let y = parse_xdotool_var(&s, "Y=")?;
                    Some(tauri::PhysicalPosition::new(x, y))
                })
        });

    // Try to find the monitor at cursor position
    if let Some(cursor) = cursor_pos {
        if let Some(window) = app.get_webview_window("main") {
            if let Ok(Some(monitor)) = window.monitor_from_point(cursor.x, cursor.y) {
                let scale = monitor.scale_factor();
                let mon_pos = monitor.position().to_logical::<f64>(scale);
                let mon_size = monitor.size().to_logical::<f64>(scale);
                let x = mon_pos.x + (mon_size.width - POPUP_WIDTH) / 2.0;
                let y = mon_pos.y + mon_size.height - POPUP_HEIGHT - margin_bottom as f64;
                return (x, y);
            }
        }
    }

    // Fallback: try primary monitor
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(Some(monitor)) = window.primary_monitor() {
            let scale = monitor.scale_factor();
            let mon_pos = monitor.position().to_logical::<f64>(scale);
            let mon_size = monitor.size().to_logical::<f64>(scale);
            let x = mon_pos.x + (mon_size.width - POPUP_WIDTH) / 2.0;
            let y = mon_pos.y + mon_size.height - POPUP_HEIGHT - margin_bottom as f64;
            return (x, y);
        }
    }

    // Last resort: center-ish on a 1920x1080 screen
    (860.0, 940.0)
}

fn parse_xdotool_var(output: &str, prefix: &str) -> Option<f64> {
    output
        .lines()
        .find(|l| l.starts_with(prefix))
        .and_then(|l| l.trim_start_matches(prefix).parse().ok())
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
