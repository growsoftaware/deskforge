use std::process::Command;

/// Reads the current xkb-options from gsettings.
/// Returns the raw string like "['caps:escape']" or "@as []".
fn get_xkb_options_raw() -> Result<String, String> {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.input-sources", "xkb-options"])
        .output()
        .map_err(|e| format!("Failed to run gsettings: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "gsettings failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Sets xkb-options via gsettings.
fn set_xkb_options_raw(value: &str) -> Result<(), String> {
    let output = Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.input-sources",
            "xkb-options",
            value,
        ])
        .output()
        .map_err(|e| format!("Failed to run gsettings: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "gsettings set failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Parses the gsettings array string into a Vec of options.
/// e.g. "['caps:escape', 'compose:ralt']" -> vec!["caps:escape", "compose:ralt"]
fn parse_options(raw: &str) -> Vec<String> {
    let trimmed = raw.trim();
    if trimmed == "@as []" || trimmed == "[]" {
        return vec![];
    }
    // Remove brackets, split by comma, strip quotes and whitespace
    trimmed
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().trim_matches('\'').trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Formats a Vec of options back into gsettings array string.
fn format_options(opts: &[String]) -> String {
    if opts.is_empty() {
        "[]".to_string()
    } else {
        let inner: Vec<String> = opts.iter().map(|o| format!("'{o}'")).collect();
        format!("[{}]", inner.join(", "))
    }
}

/// Checks if a specific XKB option (e.g. "caps:escape") is active.
pub fn is_option_active(option: &str) -> Result<bool, String> {
    let raw = get_xkb_options_raw()?;
    let opts = parse_options(&raw);
    Ok(opts.iter().any(|o| o == option))
}

/// Adds an XKB option if not present. Returns true if it was added.
pub fn add_option(option: &str) -> Result<bool, String> {
    let raw = get_xkb_options_raw()?;
    let mut opts = parse_options(&raw);

    if opts.iter().any(|o| o == option) {
        return Ok(false);
    }

    opts.push(option.to_string());
    set_xkb_options_raw(&format_options(&opts))?;
    Ok(true)
}

/// Removes an XKB option if present. Returns true if it was removed.
pub fn remove_option(option: &str) -> Result<bool, String> {
    let raw = get_xkb_options_raw()?;
    let opts = parse_options(&raw);
    let filtered: Vec<String> = opts.into_iter().filter(|o| o != option).collect();

    if filtered.len() == parse_options(&raw).len() {
        return Ok(false);
    }

    set_xkb_options_raw(&format_options(&filtered))?;
    Ok(true)
}

/// Toggles an XKB option. Returns true if the option is now active.
pub fn toggle_option(option: &str) -> Result<bool, String> {
    if is_option_active(option)? {
        remove_option(option)?;
        Ok(false)
    } else {
        add_option(option)?;
        Ok(true)
    }
}

/// Maps a remap source+target to the corresponding XKB option string.
pub fn remap_to_xkb_option(source: &str, target: &str) -> Option<String> {
    match (source.to_lowercase().as_str(), target.to_lowercase().as_str()) {
        ("capslock", "escape") => Some("caps:escape".to_string()),
        ("capslock", "backspace") => Some("caps:backspace".to_string()),
        ("capslock", "ctrl_l") | ("capslock", "control") => Some("caps:ctrl_modifier".to_string()),
        ("capslock", "super") => Some("caps:super".to_string()),
        _ => None,
    }
}
