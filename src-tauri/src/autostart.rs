use std::fs;
use std::path::PathBuf;

const DESKTOP_ENTRY: &str = "[Desktop Entry]
Type=Application
Name=DeskForge
Comment=Desktop utility hub — keyboard remapping, shortcuts, text macros
Exec=deskforge
Icon=deskforge
Terminal=false
StartupNotify=false
X-GNOME-Autostart-enabled=true
Categories=Utility;
";

fn autostart_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("autostart")
        .join("deskforge.desktop")
}

pub fn is_enabled() -> bool {
    autostart_path().exists()
}

pub fn enable() -> Result<(), String> {
    let path = autostart_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create autostart dir: {e}"))?;
    }

    // Try to find the actual binary path
    let exec_path = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "deskforge".to_string());

    let entry = DESKTOP_ENTRY.replace("Exec=deskforge", &format!("Exec={exec_path}"));
    fs::write(&path, entry).map_err(|e| format!("Failed to write .desktop file: {e}"))?;

    Ok(())
}

pub fn disable() -> Result<(), String> {
    let path = autostart_path();
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to remove .desktop file: {e}"))?;
    }
    Ok(())
}
