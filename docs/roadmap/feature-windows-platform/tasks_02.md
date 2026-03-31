# Tasks — Epic 02: Windows Keyboard Engine

## Task 2.1 — Implement low-level keyboard hook for CapsLock remap

**Priority:** P0 (core feature)
**Estimate:** Large
**File:** `src-tauri/src/modules/keyboard/platform/windows.rs`

### Description
Implementar CapsLock → Escape usando `SetWindowsHookEx` com `WH_KEYBOARD_LL`. Este hook intercepts keypresses no nível do sistema e permite remapear sem reboot.

### Technical Design

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::Foundation::*;

static CAPSLOCK_REMAP_ACTIVE: AtomicBool = AtomicBool::new(false);
static mut HOOK_HANDLE: HHOOK = std::ptr::null_mut();

/// The low-level keyboard hook callback
unsafe extern "system" fn keyboard_hook_proc(
    code: i32, w_param: WPARAM, l_param: LPARAM
) -> LRESULT {
    if code == HC_ACTION && CAPSLOCK_REMAP_ACTIVE.load(Ordering::Relaxed) {
        let kbd = &*(l_param as *const KBDLLHOOKSTRUCT);
        if kbd.vkCode == VK_CAPITAL as u32 {
            // Intercept CapsLock → send Escape instead
            let mut input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_ESCAPE,
                        wScan: 0,
                        dwFlags: if w_param == WM_KEYUP as usize { KEYEVENTF_KEYUP } else { 0 },
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
            return 1; // swallow original CapsLock
        }
    }
    CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param)
}
```

### Key Decisions
- Use `windows-sys` crate (lighter than `windows` crate, raw FFI)
- Hook runs on a dedicated message pump thread (must call `GetMessage` loop)
- `AtomicBool` for thread-safe toggle without mutex overhead
- The hook thread is started once on app init, toggle just flips the atomic

### Integration Points
- `is_option_active("caps:escape")` → reads `CAPSLOCK_REMAP_ACTIVE`
- `add_option("caps:escape")` → sets atomic to true
- `remove_option("caps:escape")` → sets atomic to false
- `toggle_option("caps:escape")` → flips atomic

### Testing Checklist
- [ ] CapsLock key produces Escape in Notepad
- [ ] CapsLock LED does NOT toggle (key is swallowed before reaching the LED driver)
- [ ] Toggle off restores normal CapsLock behavior
- [ ] Multiple rapid toggles don't crash
- [ ] Hook survives sleep/wake cycle

### Done when
- CapsLock → Escape works in real Windows apps
- Toggle is instant (< 1ms)
- No memory leaks in hook thread

---

## Task 2.2 — Add Cargo dependency: `windows-sys`

**Priority:** P0 (blocker for 2.1)
**Estimate:** Small
**File:** `src-tauri/Cargo.toml`

### Description
Adicionar `windows-sys` com as features necessárias para keyboard hooks.

```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows-sys = { version = "0.52", features = [
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_Foundation",
] }
```

### Done when
- Cargo.toml updated
- `cargo check --target x86_64-pc-windows-msvc` passes

---

## Task 2.3 — Hook lifecycle management

**Priority:** P0
**Estimate:** Medium
**File:** `src-tauri/src/modules/keyboard/platform/windows.rs`

### Description
Gerenciar o ciclo de vida do keyboard hook: start on app init, stop on app exit.

### Implementation

```rust
use std::thread;

/// Start the keyboard hook on a dedicated thread with message pump
pub fn start_hook() -> Result<(), String> {
    thread::spawn(|| unsafe {
        HOOK_HANDLE = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            std::ptr::null_mut(), // current module
            0, // all threads
        );
        if HOOK_HANDLE.is_null() {
            eprintln!("Failed to install keyboard hook");
            return;
        }
        // Message pump — required for low-level hooks
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        UnhookWindowsHookEx(HOOK_HANDLE);
    });
    Ok(())
}

/// Stop the hook (called on app exit)
pub fn stop_hook() {
    unsafe {
        if !HOOK_HANDLE.is_null() {
            PostThreadMessageW(/* hook thread id */, WM_QUIT, 0, 0);
        }
    }
}
```

### Integration in `lib.rs`
- Call `platform::start_hook()` in Tauri `.setup()` callback (after `remapper::apply_saved()`)
- Call `platform::stop_hook()` before `app.exit(0)` in tray quit handler

### Done when
- Hook starts with app, stops with app
- No orphaned hook threads
- App exit is clean (no process hanging)

---

## Task 2.4 — Implement text macros with `arboard` + `enigo`

**Priority:** P0 (core feature)
**Estimate:** Medium
**File:** `src-tauri/src/modules/keyboard/macros.rs`

### Description
Implementar text macros no Windows usando `arboard` (clipboard) e `enigo` (input simulation).

### Implementation — Clipboard method

```rust
#[cfg(target_os = "windows")]
fn execute_windows(mac: &TextMacro) -> Result<(), String> {
    match mac.method.as_str() {
        "clipboard" => paste_via_clipboard_win(&mac.text),
        "type" => type_text_win(&mac.text),
        _ => Err(format!("Unknown method: {}", mac.method)),
    }
}

#[cfg(target_os = "windows")]
fn paste_via_clipboard_win(text: &str) -> Result<(), String> {
    use arboard::Clipboard;
    use enigo::{Enigo, Key, KeyboardControllable};

    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {e}"))?;

    // Save current clipboard
    let saved = clipboard.get_text().ok();

    // Set new text
    clipboard.set_text(text).map_err(|e| format!("Failed to set clipboard: {e}"))?;

    // Simulate Ctrl+V
    let mut enigo = Enigo::new();
    std::thread::sleep(std::time::Duration::from_millis(50));
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);

    // Restore clipboard after delay
    if let Some(saved_text) = saved {
        let saved_clone = saved_text.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Ok(mut cb) = Clipboard::new() {
                let _ = cb.set_text(&saved_clone);
            }
        });
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn type_text_win(text: &str) -> Result<(), String> {
    use enigo::{Enigo, KeyboardControllable};
    let mut enigo = Enigo::new();
    // Small delay between chars for reliability
    for ch in text.chars() {
        enigo.key_sequence(&ch.to_string());
        std::thread::sleep(std::time::Duration::from_millis(12));
    }
    Ok(())
}
```

### Testing Checklist
- [ ] Clipboard method works in Notepad
- [ ] Clipboard method works in Chrome URL bar
- [ ] Type method works in Notepad
- [ ] Original clipboard content is restored after paste
- [ ] Unicode characters work (accents, emojis)
- [ ] Multi-line text works

### Done when
- Both macro methods work on Windows
- Clipboard is restored after paste
- No crashes on empty clipboard

---

## Task 2.5 — Wire up remapper to read/persist state from config

**Priority:** P1
**Estimate:** Small
**File:** `src-tauri/src/modules/keyboard/remapper.rs`

### Description
O `remapper.rs` chama `platform::is_option_active()` para ler o estado do sistema, mas no Windows o estado vive no `AtomicBool` do hook, não no gsettings. Garantir que:

1. `get_all_statuses()` lê corretamente o estado no Windows
2. `apply_saved()` no startup restaura o estado salvo no config
3. `toggle()` atualiza o AtomicBool E o config

### Implementation
No Windows, `is_option_active` deve ler o `AtomicBool`. `add_option` e `remove_option` devem setar o `AtomicBool`. O config TOML já é salvo pelo `remapper.rs` — isso não muda.

### Done when
- App startup restaura o remap salvo
- Toggle persiste entre sessões do app
- UI mostra estado correto

---

## Task 2.6 — Optional: Registry Scancode Map for persist-across-reboot

**Priority:** P2 (nice-to-have)
**Estimate:** Medium
**File:** `src-tauri/src/modules/keyboard/platform/windows.rs`

### Description
Adicionar opção de persistir o remap via Registry Scancode Map, para sobreviver reboot sem o app rodando. É o equivalente Windows do nosso `persist_fnmode`.

### Technical Notes
- Registry path: `HKLM\SYSTEM\CurrentControlSet\Control\Keyboard Layout`
- Value name: `Scancode Map`
- Binary format: header (8 bytes) + entries (4 bytes each: target scancode + source scancode) + null terminator (4 bytes)
- CapsLock (0x3A) → Escape (0x01): `01 00 3A 00`
- **Requires admin elevation** — usar `runas` ou UAC prompt
- **Requires logoff/reboot** to take effect

### Implementation with `winreg`
```rust
use winreg::RegKey;
use winreg::enums::*;

fn persist_remap_to_registry() -> Result<(), String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\Keyboard Layout",
        KEY_SET_VALUE,
    ).map_err(|e| format!("Registry access denied (run as admin): {e}"))?;

    // Scancode Map binary: header + 1 entry + null terminator
    let scancode_map: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, // version
        0x00, 0x00, 0x00, 0x00, // flags
        0x02, 0x00, 0x00, 0x00, // number of entries (1 + null)
        0x01, 0x00, 0x3A, 0x00, // CapsLock → Escape
        0x00, 0x00, 0x00, 0x00, // null terminator
    ];

    key.set_raw_value("Scancode Map", &winreg::RegValue {
        vtype: REG_BINARY,
        bytes: scancode_map,
    }).map_err(|e| format!("Failed to write Scancode Map: {e}"))?;

    Ok(())
}
```

### Done when
- UI has optional "Persist across reboot" checkbox
- Registry write works when app is run as admin
- Clear error message when not admin

---

## Task Order & Dependencies

```
2.2 Cargo deps ──── 2.1 Keyboard hook ──── 2.3 Hook lifecycle ──── 2.5 Remapper wire-up
                                                                          │
                     2.4 Text macros (can run in parallel with 2.1) ──────┤
                                                                          │
                     2.6 Registry persist (optional, can run last) ───────┘
```

## Estimated Effort
- **Total:** ~8-12 hours of focused work
- **Risk:** Medium — keyboard hooks and input simulation are tricky on Windows
- **Key testing:** Must test on real Windows (not Wine/VM build — the hooks need real Win32)
