use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

/// Captures the next key press using xev, returns the key name.
/// Blocks for up to `timeout` seconds waiting for a key.
pub fn capture_next_key(timeout_secs: u64) -> Result<String, String> {
    // Use xev with -root to capture keys globally (not just in xev's window)
    let mut child = Command::new("timeout")
        .args([
            &timeout_secs.to_string(),
            "xev",
            "-root",
            "-event",
            "keyboard",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start xev: {e}"))?;

    let stdout = child.stdout.take().ok_or("No stdout")?;
    let reader = BufReader::new(stdout);

    let mut keysym_name = None;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Read error: {e}"))?;

        // Look for KeyPress events (ignore KeyRelease)
        // xev output format: "    state 0x0, keycode 65 (keysym 0xff52, F7), same_screen YES,"
        if line.contains("keysym") && line.contains("KeyPress") == false {
            // The keysym line comes after KeyPress
            if let Some(name) = parse_keysym_name(&line) {
                // Skip modifier keys
                if is_modifier(&name) {
                    continue;
                }
                keysym_name = Some(name);
                break;
            }
        }
    }

    let _ = child.kill();
    let _ = child.wait();

    keysym_name.ok_or_else(|| "Nenhuma tecla detectada".into())
}

/// Parse keysym name from xev output line like:
/// "    state 0x0, keycode 65 (keysym 0xff52, Up), same_screen YES,"
fn parse_keysym_name(line: &str) -> Option<String> {
    // Find pattern: keysym 0xHEX, NAME)
    let keysym_idx = line.find("keysym ")?;
    let after_keysym = &line[keysym_idx..];
    let comma_idx = after_keysym.find(", ")?;
    let after_comma = &after_keysym[comma_idx + 2..];
    let end_idx = after_comma.find(')')?;
    let name = after_comma[..end_idx].trim().to_string();

    if name.is_empty() {
        return None;
    }

    Some(name)
}

fn is_modifier(name: &str) -> bool {
    matches!(
        name,
        "Shift_L"
            | "Shift_R"
            | "Control_L"
            | "Control_R"
            | "Alt_L"
            | "Alt_R"
            | "Super_L"
            | "Super_R"
            | "Meta_L"
            | "Meta_R"
            | "Caps_Lock"
            | "Num_Lock"
    )
}

/// Convert X11 keysym name to our binding format.
/// e.g. "F7" -> "F7", "Up" -> "Up", "a" -> "A"
pub fn keysym_to_binding(keysym: &str) -> String {
    match keysym {
        // Function keys pass through
        s if s.starts_with('F') && s[1..].parse::<u32>().is_ok() => s.to_string(),
        // Letters
        s if s.len() == 1 && s.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) => {
            s.to_uppercase()
        }
        // Special keys
        "Escape" => "Escape".into(),
        "Return" => "Enter".into(),
        "space" => "Space".into(),
        "BackSpace" => "Backspace".into(),
        "Tab" => "Tab".into(),
        "Delete" => "Delete".into(),
        "Insert" => "Insert".into(),
        "Home" => "Home".into(),
        "End" => "End".into(),
        "Prior" | "Page_Up" => "PageUp".into(),
        "Next" | "Page_Down" => "PageDown".into(),
        // Pass through as-is for anything else
        other => other.to_string(),
    }
}
