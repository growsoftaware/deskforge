use std::process::Command;
use std::thread;
use std::time::Duration;

/// Executes a text macro by pasting text via clipboard.
/// 1. Saves current clipboard content
/// 2. Sets clipboard to macro text
/// 3. Simulates Ctrl+V
/// 4. Restores original clipboard (after small delay)
pub fn execute(text: &str, method: &str) -> Result<(), String> {
    match method {
        "type" => execute_type(text),
        _ => execute_clipboard(text), // "clipboard" is default
    }
}

/// Paste via clipboard: xclip + xdotool Ctrl+V
fn execute_clipboard(text: &str) -> Result<(), String> {
    // Save current clipboard
    let old_clipboard = Command::new("xclip")
        .args(["-selection", "clipboard", "-o"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).to_string())
            } else {
                None
            }
        });

    // Set clipboard to macro text
    let mut child = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run xclip: {e}"))?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| format!("Failed to write to xclip: {e}"))?;
    }
    child
        .wait()
        .map_err(|e| format!("xclip failed: {e}"))?;

    // Small delay for clipboard to settle
    thread::sleep(Duration::from_millis(50));

    // Simulate Ctrl+V
    Command::new("xdotool")
        .args(["key", "--clearmodifiers", "ctrl+v"])
        .output()
        .map_err(|e| format!("Failed to run xdotool: {e}"))?;

    // Restore old clipboard after a delay (in background)
    if let Some(old) = old_clipboard {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            let mut child = Command::new("xclip")
                .args(["-selection", "clipboard"])
                .stdin(std::process::Stdio::piped())
                .spawn()
                .ok();
            if let Some(ref mut c) = child {
                if let Some(stdin) = c.stdin.as_mut() {
                    use std::io::Write;
                    let _ = stdin.write_all(old.as_bytes());
                }
                let _ = c.wait();
            }
        });
    }

    Ok(())
}

/// Type text directly via xdotool (slower but preserves clipboard)
fn execute_type(text: &str) -> Result<(), String> {
    Command::new("xdotool")
        .args(["type", "--clearmodifiers", "--delay", "12", text])
        .output()
        .map_err(|e| format!("Failed to run xdotool type: {e}"))?;

    Ok(())
}
