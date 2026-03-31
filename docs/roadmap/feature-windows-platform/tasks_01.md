# Tasks — Epic 01: Platform Abstraction Layer

## Task 1.1 — Setup Windows build target

**Priority:** P0 (blocker)
**Estimate:** Small

### Description
Instalar o target `x86_64-pc-windows-msvc` via rustup e verificar que o toolchain compila Tauri apps para Windows.

### Steps
1. `rustup target add x86_64-pc-windows-msvc`
2. Verificar `cargo check --target x86_64-pc-windows-msvc` (vai falhar — é esperado, serve como baseline)
3. Documentar os erros de compilação exatos para referência

### Done when
- Target instalado
- Lista de erros de compilação documentada

---

## Task 1.2 — Create `platform/windows.rs` with stubs

**Priority:** P0 (blocker)
**Estimate:** Small
**File:** `src-tauri/src/modules/keyboard/platform/windows.rs`

### Description
Criar o módulo Windows com as mesmas 5 funções exportadas que `linux.rs`, retornando stubs.

### Implementation

```rust
// src-tauri/src/modules/keyboard/platform/windows.rs

/// Map a remap pair to a Windows-compatible identifier
/// On Windows, CapsLock→Escape is done via Scancode Map in registry
pub fn remap_to_xkb_option(source: &str, target: &str) -> Option<String> {
    match (source, target) {
        ("CapsLock", "Escape") => Some("caps:escape".into()),
        _ => None,
    }
}

pub fn is_option_active(_option: &str) -> Result<bool, String> {
    Err("Windows keyboard remapping not yet implemented".into())
}

pub fn add_option(_option: &str) -> Result<bool, String> {
    Err("Windows keyboard remapping not yet implemented".into())
}

pub fn remove_option(_option: &str) -> Result<bool, String> {
    Err("Windows keyboard remapping not yet implemented".into())
}

pub fn toggle_option(_option: &str) -> Result<bool, String> {
    Err("Windows keyboard remapping not yet implemented".into())
}
```

### Also update `platform/mod.rs`:
```rust
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "windows")]
pub use windows::*;
```

### Done when
- `platform/windows.rs` exists with all 5 function stubs
- `platform/mod.rs` has `#[cfg(target_os = "windows")]` exports

---

## Task 1.3 — Gate `devices.rs` for Linux-only

**Priority:** P0 (blocker)
**Estimate:** Small
**File:** `src-tauri/src/modules/keyboard/devices.rs`

### Description
O módulo `devices.rs` acessa `/proc/bus/input/devices` e `/sys/module/hid_apple/` — paths que não existem no Windows. O NuPhy fix é Linux-only por design (o driver hid_apple é do kernel Linux).

### Implementation

Opção mais limpa: mover a lógica para dentro de `#[cfg]` e fornecer stubs:

```rust
// No topo do arquivo
#[cfg(target_os = "linux")]
mod linux_impl {
    // ... todo o código atual de devices.rs
}

#[cfg(target_os = "linux")]
pub use linux_impl::*;

// Stub para plataformas sem device fixes
#[cfg(not(target_os = "linux"))]
pub fn get_all() -> Vec<DeviceFix> {
    vec![]
}

#[cfg(not(target_os = "linux"))]
pub fn enable_fkeys() -> Result<(), String> {
    Err("Device fixes are only available on Linux".into())
}

#[cfg(not(target_os = "linux"))]
pub fn disable_fkeys() -> Result<(), String> {
    Err("Device fixes are only available on Linux".into())
}
```

A struct `DeviceFix` precisa ficar fora do `#[cfg]` pois é usada no retorno dos comandos Tauri.

### Done when
- `devices.rs` compila em ambos os targets
- No Windows, `get_all()` retorna `vec![]`
- No Linux, comportamento idêntico ao atual

---

## Task 1.4 — Gate `macros.rs` with platform stubs

**Priority:** P0 (blocker)
**Estimate:** Small
**File:** `src-tauri/src/modules/keyboard/macros.rs`

### Description
`macros.rs` usa `xclip` e `xdotool` diretamente. No Epic 02 vamos trocar por `arboard` + `enigo`, mas neste epic basta criar stubs.

### Implementation
Envolver as funções `save_clipboard`, `set_clipboard`, `paste_via_clipboard`, `type_text` com `#[cfg(target_os = "linux")]` e criar stubs `#[cfg(target_os = "windows")]` que retornam `Err("Not yet implemented")`.

A função pública `execute(macro_entry)` deve ter o dispatch:

```rust
pub fn execute(mac: &TextMacro) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return execute_linux(mac);

    #[cfg(target_os = "windows")]
    return Err("Text macros not yet implemented on Windows".into());
}
```

### Done when
- Compila em ambos os targets
- No Linux, funciona como antes
- No Windows, retorna erro amigável

---

## Task 1.5 — Gate `popup.rs` for cross-platform

**Priority:** P1
**Estimate:** Small
**File:** `src-tauri/src/popup.rs`

### Description
`popup.rs` spawna `python3 popup-overlay.py` (GTK3). No Windows isso não funciona. Stub por enquanto, implementação real no Epic 03.

### Implementation
```rust
pub fn show(_app: &tauri::AppHandle, text: &str, icon: &str) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return show_linux(_app, text, icon);

    #[cfg(target_os = "windows")]
    {
        // TODO: Epic 03 — implement Tauri webview popup
        let _ = (text, icon);
        Ok(())  // silently no-op for now
    }
}
```

### Done when
- Compila em ambos os targets
- No Windows, `show()` é no-op silencioso (não crashea)

---

## Task 1.6 — Gate `autostart.rs` for cross-platform

**Priority:** P1
**Estimate:** Small
**File:** `src-tauri/src/autostart.rs`

### Description
`autostart.rs` escreve `.desktop` file para XDG autostart. No Windows, autostart é via Registry.

### Implementation
```rust
pub fn is_enabled() -> bool {
    #[cfg(target_os = "linux")]
    return is_enabled_linux();

    #[cfg(target_os = "windows")]
    return false; // stub
}

pub fn enable() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return enable_linux();

    #[cfg(target_os = "windows")]
    return Err("Autostart not yet implemented on Windows".into());
}

pub fn disable() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return disable_linux();

    #[cfg(target_os = "windows")]
    return Err("Autostart not yet implemented on Windows".into());
}
```

### Done when
- Compila em ambos os targets
- No Windows, is_enabled retorna false, enable/disable retorna erro amigável

---

## Task 1.7 — Platform-aware `launch_app` in `lib.rs`

**Priority:** P1
**Estimate:** Small
**File:** `src-tauri/src/lib.rs`

### Description
`launch_app` usa `alacritty` e `bash -c` hardcoded. No Windows: `cmd.exe /C` para shell, `wt.exe` ou `cmd.exe` para terminal.

### Implementation
```rust
#[tauri::command]
fn launch_app(command: String, terminal: bool) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if terminal {
            std::process::Command::new("alacritty")
                .args(["-e", "bash", "-c", &command])
                .spawn()
                .map_err(|e| format!("Failed to launch in terminal: {e}"))?;
        } else {
            std::process::Command::new("bash")
                .args(["-c", &command])
                .spawn()
                .map_err(|e| format!("Failed to launch: {e}"))?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        if terminal {
            std::process::Command::new("cmd.exe")
                .args(["/C", "start", "cmd.exe", "/K", &command])
                .spawn()
                .map_err(|e| format!("Failed to launch in terminal: {e}"))?;
        } else {
            std::process::Command::new("cmd.exe")
                .args(["/C", &command])
                .spawn()
                .map_err(|e| format!("Failed to launch: {e}"))?;
        }
    }

    Ok(())
}
```

### Done when
- No Linux, `alacritty` + `bash` como antes
- No Windows, `cmd.exe` para ambos os modos
- Compila em ambos os targets

---

## Task 1.8 — Add Windows-specific Cargo dependencies

**Priority:** P0 (blocker)
**Estimate:** Small
**File:** `src-tauri/Cargo.toml`

### Description
Adicionar crates que serão usadas nos Epics 02 e 03, condicionadas ao target Windows.

### Implementation
Adicionar ao `Cargo.toml`:

```toml
[target.'cfg(target_os = "windows")'.dependencies]
winreg = "0.52"
arboard = "3"
enigo = { version = "0.2", features = ["x11rb"] }  # x11rb feature é ignorado no Windows

[target.'cfg(target_os = "linux")'.dependencies]
# (futuro: mover deps Linux-only aqui se necessário)
```

Nota: `arboard` e `enigo` são cross-platform mas só serão usados no Windows por enquanto (Linux continua com xclip/xdotool que são mais battle-tested).

### Done when
- `Cargo.toml` tem seção Windows-specific
- `cargo check` passa em ambos os targets

---

## Task 1.9 — Verify full compilation on Windows target

**Priority:** P0 (blocker, final gate)
**Estimate:** Small

### Description
Rodar `cargo check --target x86_64-pc-windows-msvc` e confirmar zero erros. Se possível, rodar `npm run tauri build --target x86_64-pc-windows-msvc` para gerar o binário.

### Steps
1. `cargo check --target x86_64-pc-windows-msvc` — deve passar
2. Se em máquina Windows: `npm run tauri dev` — o app deve abrir com sidebar e nav
3. Keyboard page deve mostrar "Nenhum remapeamento configurado" (stubs)
4. Apps page deve mostrar os 3 apps (mas "Abrir" pode falhar — OK neste epic)
5. Device fixes section não deve aparecer (vec![] no Windows)

### Done when
- Zero erros de compilação
- App abre no Windows com UI funcional (mesmo que features não funcionem)
- Nenhuma regressão no Linux

---

## Task Order & Dependencies

```
1.1 Setup target          ──┐
                             ├── 1.8 Cargo deps ──┐
1.2 platform/windows.rs ──┤                       │
                             ├── 1.9 Verify ──── DONE
1.3 Gate devices.rs       ──┤                       │
1.4 Gate macros.rs        ──┤                       │
1.5 Gate popup.rs         ──┤                       │
1.6 Gate autostart.rs     ──┤                       │
1.7 Gate launch_app       ──┘                       │
                                                    │
(Tasks 1.1-1.7 can run in parallel) ────────────────┘
```

## Estimated Effort
- **Total:** ~3-4 hours of focused work
- **Risk:** Low — all changes are additive, no existing code is modified
- **Reviewer focus:** Ensure all `#[cfg]` gates are correct and no Linux code leaks
