use serde::Serialize;
use std::fs;
use std::process::Command;

const FNMODE_SYSFS: &str = "/sys/module/hid_apple/parameters/fnmode";
const MODPROBE_CONF: &str = "/etc/modprobe.d/hid_apple.conf";

#[derive(Debug, Clone, Serialize)]
pub struct DeviceFix {
    pub id: String,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub detected: bool,
}

/// Check if a NuPhy keyboard is connected (uses hid_apple driver)
fn is_nuphy_connected() -> bool {
    fs::read_to_string("/proc/bus/input/devices")
        .map(|s| s.contains("NuPhy"))
        .unwrap_or(false)
}

/// Check if fnmode=2 (F-keys first) is active
fn is_fkeys_active() -> bool {
    fs::read_to_string(FNMODE_SYSFS)
        .map(|s| s.trim() == "2")
        .unwrap_or(false)
}

/// Get status of all device fixes
pub fn get_all() -> Vec<DeviceFix> {
    vec![DeviceFix {
        id: "nuphy-fkeys".into(),
        name: "NuPhy F1-F12".into(),
        description: "Usar F1-F12 em vez de teclas de mídia".into(),
        active: is_fkeys_active(),
        detected: is_nuphy_connected(),
    }]
}

/// Enable F-keys first mode (fnmode=2)
pub fn enable_fkeys() -> Result<(), String> {
    // Apply immediately via sysfs
    fs::write(FNMODE_SYSFS, "2")
        .or_else(|_| {
            // If direct write fails (permissions), try pkexec
            Command::new("pkexec")
                .args(["bash", "-c", &format!("echo 2 > {FNMODE_SYSFS}")])
                .status()
                .map(|_| ())
                .map_err(|e| format!("{e}"))
        })
        .map_err(|e| format!("Failed to set fnmode: {e}"))?;

    // Make permanent
    persist_fnmode(2)?;

    Ok(())
}

/// Disable F-keys first mode (back to fnmode=1, media keys default)
pub fn disable_fkeys() -> Result<(), String> {
    fs::write(FNMODE_SYSFS, "1")
        .or_else(|_| {
            Command::new("pkexec")
                .args(["bash", "-c", &format!("echo 1 > {FNMODE_SYSFS}")])
                .status()
                .map(|_| ())
                .map_err(|e| format!("{e}"))
        })
        .map_err(|e| format!("Failed to set fnmode: {e}"))?;

    // Make permanent
    persist_fnmode(1)?;

    Ok(())
}

/// Write modprobe.d config and update initramfs for persistence across reboots
fn persist_fnmode(mode: u8) -> Result<(), String> {
    let conf = format!("options hid_apple fnmode={mode}\n");
    let tmp = "/tmp/hid_apple.conf";

    fs::write(tmp, &conf).map_err(|e| format!("Failed to write temp file: {e}"))?;

    let status = Command::new("pkexec")
        .args([
            "bash",
            "-c",
            &format!("cp {tmp} {MODPROBE_CONF} && update-initramfs -u"),
        ])
        .status()
        .map_err(|e| format!("Failed to run pkexec: {e}"))?;

    if !status.success() {
        return Err("Autorização negada ou comando falhou".into());
    }

    Ok(())
}
