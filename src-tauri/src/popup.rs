use std::process::Command;

use crate::config;

pub fn show(_app: &tauri::AppHandle, text: &str, icon: &str) -> Result<(), String> {
    let cfg = config::load();
    let popup_cfg = &cfg.popup;

    if !popup_cfg.enabled {
        return Ok(());
    }

    // Use the bundled Python popup script (proven to work on X11/Wayland)
    let script_path = popup_script_path();

    Command::new("python3")
        .args([
            &script_path,
            text,
            icon,
            &popup_cfg.display_ms.to_string(),
            &popup_cfg.fade_ms.to_string(),
            &popup_cfg.margin_bottom.to_string(),
        ])
        .spawn()
        .map_err(|e| format!("Failed to spawn popup: {e}"))?;

    Ok(())
}

fn popup_script_path() -> String {
    // In dev: use legacy script from repo
    // In production: use bundled resource
    let dev_path = std::env::current_dir()
        .ok()
        .map(|p| p.join("../legacy/popup-overlay.py"))
        .filter(|p| p.exists());

    if let Some(p) = dev_path {
        return p.display().to_string();
    }

    // Try next to the binary
    if let Ok(exe) = std::env::current_exe() {
        let beside = exe.parent().unwrap().join("popup-overlay.py");
        if beside.exists() {
            return beside.display().to_string();
        }
    }

    // Fallback
    "popup-overlay.py".to_string()
}
