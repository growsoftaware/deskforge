use crate::{config, modules::keyboard::macros, modules::keyboard::remapper, popup, tray};
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent, ShortcutState};

/// Parses a human-readable binding string like "Ctrl+Shift+E" into a Shortcut.
pub fn parse_binding(binding: &str) -> Option<Shortcut> {
    let parts: Vec<&str> = binding.split('+').map(|s| s.trim()).collect();
    let mut mods = Modifiers::empty();
    let mut key_str = "";

    for part in &parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => mods |= Modifiers::CONTROL,
            "shift" => mods |= Modifiers::SHIFT,
            "alt" => mods |= Modifiers::ALT,
            "super" | "meta" | "win" => mods |= Modifiers::SUPER,
            _ => key_str = part,
        }
    }

    let code = parse_key_code(key_str)?;
    let mod_opt = if mods.is_empty() { None } else { Some(mods) };
    Some(Shortcut::new(mod_opt, code))
}

fn parse_key_code(key: &str) -> Option<Code> {
    Some(match key.to_lowercase().as_str() {
        "a" => Code::KeyA,
        "b" => Code::KeyB,
        "c" => Code::KeyC,
        "d" => Code::KeyD,
        "e" => Code::KeyE,
        "f" => Code::KeyF,
        "g" => Code::KeyG,
        "h" => Code::KeyH,
        "i" => Code::KeyI,
        "j" => Code::KeyJ,
        "k" => Code::KeyK,
        "l" => Code::KeyL,
        "m" => Code::KeyM,
        "n" => Code::KeyN,
        "o" => Code::KeyO,
        "p" => Code::KeyP,
        "q" => Code::KeyQ,
        "r" => Code::KeyR,
        "s" => Code::KeyS,
        "t" => Code::KeyT,
        "u" => Code::KeyU,
        "v" => Code::KeyV,
        "w" => Code::KeyW,
        "x" => Code::KeyX,
        "y" => Code::KeyY,
        "z" => Code::KeyZ,
        "0" => Code::Digit0,
        "1" => Code::Digit1,
        "2" => Code::Digit2,
        "3" => Code::Digit3,
        "4" => Code::Digit4,
        "5" => Code::Digit5,
        "6" => Code::Digit6,
        "7" => Code::Digit7,
        "8" => Code::Digit8,
        "9" => Code::Digit9,
        "escape" | "esc" => Code::Escape,
        "space" => Code::Space,
        "enter" | "return" => Code::Enter,
        "tab" => Code::Tab,
        "backspace" => Code::Backspace,
        "delete" | "del" => Code::Delete,
        "insert" | "ins" => Code::Insert,
        "home" => Code::Home,
        "end" => Code::End,
        "pageup" => Code::PageUp,
        "pagedown" => Code::PageDown,
        "f1" => Code::F1,
        "f2" => Code::F2,
        "f3" => Code::F3,
        "f4" => Code::F4,
        "f5" => Code::F5,
        "f6" => Code::F6,
        "f7" => Code::F7,
        "f8" => Code::F8,
        "f9" => Code::F9,
        "f10" => Code::F10,
        "f11" => Code::F11,
        "f12" => Code::F12,
        ";" | "semicolon" => Code::Semicolon,
        "," | "comma" => Code::Comma,
        "." | "period" => Code::Period,
        "/" | "slash" => Code::Slash,
        "`" | "backquote" => Code::Backquote,
        "[" | "bracketleft" => Code::BracketLeft,
        "]" | "bracketright" => Code::BracketRight,
        "-" | "minus" => Code::Minus,
        "=" | "equal" => Code::Equal,
        _ => return None,
    })
}

/// Registers all shortcuts from config (remaps + macros).
/// Unregisters everything first to handle changes cleanly.
pub fn register_all(app: &AppHandle) {
    let gs = app.global_shortcut();

    // Unregister everything
    let _ = gs.unregister_all();

    let cfg = config::load();

    // Register remap shortcuts (e.g., Super+Escape for CapsLock toggle)
    for shortcut_cfg in &cfg.keyboard.shortcuts {
        if let Some(shortcut) = parse_binding(&shortcut_cfg.binding) {
            if let Err(e) = gs.register(shortcut) {
                eprintln!(
                    "Failed to register shortcut '{}' ({}): {e}",
                    shortcut_cfg.name, shortcut_cfg.binding
                );
            }
        } else {
            eprintln!(
                "Failed to parse shortcut binding: {}",
                shortcut_cfg.binding
            );
        }
    }

    // Register macro triggers
    for mac in &cfg.keyboard.macros {
        if let Some(shortcut) = parse_binding(&mac.trigger) {
            if let Err(e) = gs.register(shortcut) {
                eprintln!(
                    "Failed to register macro '{}' ({}): {e}",
                    mac.name, mac.trigger
                );
            }
        } else {
            eprintln!("Failed to parse macro trigger: {}", mac.trigger);
        }
    }
}

/// Global shortcut handler — dispatches to the right action.
pub fn handle_shortcut(app: &AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state != ShortcutState::Pressed {
        return;
    }

    let cfg = config::load();

    // Check remap shortcuts
    for shortcut_cfg in &cfg.keyboard.shortcuts {
        if let Some(parsed) = parse_binding(&shortcut_cfg.binding) {
            if shortcut == &parsed {
                match &shortcut_cfg.action {
                    config::ShortcutAction::ToggleRemap { remap_id } => {
                        if let Ok(status) = remapper::toggle(remap_id) {
                            let icon = status.icon.as_deref().unwrap_or("⌨");
                            let _ = popup::show(app, &status.label, icon);
                            tray::refresh(app);
                            let _ = app.emit("remap-changed", &status);
                        }
                    }
                }
                return;
            }
        }
    }

    // Check macro triggers
    for mac in &cfg.keyboard.macros {
        if let Some(parsed) = parse_binding(&mac.trigger) {
            if shortcut == &parsed {
                if let Err(e) = macros::execute(&mac.text, &mac.method) {
                    eprintln!("Macro '{}' failed: {e}", mac.name);
                }
                return;
            }
        }
    }
}
