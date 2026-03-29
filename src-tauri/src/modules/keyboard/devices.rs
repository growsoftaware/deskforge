use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;

const HWDB_PATH: &str = "/etc/udev/hwdb.d/90-nuphy-fkeys.hwdb";

const NUPHY_HWDB_CONTENT: &str = r#"# NuPhy Field75 — Force F1-F12 keys instead of media keys
# The keyboard reports as Apple (05ac:024f) so hid_apple remaps F-keys to media.
# This overrides using the raw USB HID scancodes.

evdev:input:b0003v05ACp024F*
 KEYBOARD_KEY_7003a=f1
 KEYBOARD_KEY_7003b=f2
 KEYBOARD_KEY_7003c=f3
 KEYBOARD_KEY_7003d=f4
 KEYBOARD_KEY_7003e=f5
 KEYBOARD_KEY_7003f=f6
 KEYBOARD_KEY_70040=f7
 KEYBOARD_KEY_70041=f8
 KEYBOARD_KEY_70042=f9
 KEYBOARD_KEY_70043=f10
 KEYBOARD_KEY_70044=f11
 KEYBOARD_KEY_70045=f12
"#;

#[derive(Debug, Clone, Serialize)]
pub struct DeviceFix {
    pub id: String,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub detected: bool,
}

/// Check if a NuPhy keyboard is connected
fn is_nuphy_connected() -> bool {
    fs::read_to_string("/proc/bus/input/devices")
        .map(|s| s.contains("NuPhy"))
        .unwrap_or(false)
}

/// Check if the hwdb fix is installed
fn is_fkeys_fix_installed() -> bool {
    Path::new(HWDB_PATH).exists()
}

/// Get status of all device fixes
pub fn get_all() -> Vec<DeviceFix> {
    vec![DeviceFix {
        id: "nuphy-fkeys".into(),
        name: "NuPhy F1-F12".into(),
        description: "Usar F1-F12 em vez de teclas de mídia".into(),
        active: is_fkeys_fix_installed(),
        detected: is_nuphy_connected(),
    }]
}

/// Install the NuPhy F-keys hwdb fix (requires pkexec for sudo)
pub fn enable_fkeys() -> Result<(), String> {
    // Write hwdb file to a temp location first
    let tmp = "/tmp/90-nuphy-fkeys.hwdb";
    fs::write(tmp, NUPHY_HWDB_CONTENT)
        .map_err(|e| format!("Failed to write temp file: {e}"))?;

    // Use pkexec to copy to /etc and apply
    let status = Command::new("pkexec")
        .args([
            "bash",
            "-c",
            &format!(
                "cp {tmp} {HWDB_PATH} && systemd-hwdb update && udevadm trigger"
            ),
        ])
        .status()
        .map_err(|e| format!("Failed to run pkexec: {e}"))?;

    if !status.success() {
        return Err("Autorização negada ou comando falhou".into());
    }

    Ok(())
}

/// Remove the NuPhy F-keys hwdb fix
pub fn disable_fkeys() -> Result<(), String> {
    let status = Command::new("pkexec")
        .args([
            "bash",
            "-c",
            &format!(
                "rm -f {HWDB_PATH} && systemd-hwdb update && udevadm trigger"
            ),
        ])
        .status()
        .map_err(|e| format!("Failed to run pkexec: {e}"))?;

    if !status.success() {
        return Err("Autorização negada ou comando falhou".into());
    }

    Ok(())
}
