# Tasks — Epic 03: System Integration (Windows)

## Task 3.1 — Implement Windows autostart via Registry

**Priority:** P0
**Estimate:** Small
**File:** `src-tauri/src/autostart.rs`

### Description
No Windows, autostart é configurado via Registry key `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`. Não precisa de admin (HKCU = current user).

### Implementation

```rust
#[cfg(target_os = "windows")]
fn is_enabled_windows() -> bool {
    use winreg::RegKey;
    use winreg::enums::*;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .and_then(|key| key.get_value::<String, _>("DeskForge"))
        .is_ok()
}

#[cfg(target_os = "windows")]
fn enable_windows() -> Result<(), String> {
    use winreg::RegKey;
    use winreg::enums::*;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE,
    ).map_err(|e| format!("Failed to open registry: {e}"))?;

    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {e}"))?;

    key.set_value("DeskForge", &exe_path.to_string_lossy().to_string())
        .map_err(|e| format!("Failed to set autostart: {e}"))?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn disable_windows() -> Result<(), String> {
    use winreg::RegKey;
    use winreg::enums::*;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE,
    ).map_err(|e| format!("Failed to open registry: {e}"))?;

    key.delete_value("DeskForge")
        .map_err(|e| format!("Failed to remove autostart: {e}"))?;

    Ok(())
}
```

### Testing Checklist
- [ ] Enable autostart → key appears in `regedit` at `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
- [ ] Value points to correct `.exe` path
- [ ] After reboot/logoff+logon, DeskForge starts automatically
- [ ] Disable autostart → key removed from registry
- [ ] Toggle in UI reflects correct state

### Done when
- Autostart toggle works via UI checkbox
- Survives reboot test

---

## Task 3.2 — Replace Python/GTK popup with Tauri webview window

**Priority:** P1
**Estimate:** Medium
**Files:** `src-tauri/src/popup.rs`, `src-tauri/tauri.conf.json`, `src/routes/popup/+page.svelte`

### Description
Migrar o popup de notificação de Python/GTK para uma janela Tauri nativa. Isso funciona em ambas as plataformas e elimina a dependência de `python3` e GTK.

### Technical Design

1. **Registrar segunda janela no `tauri.conf.json`:**
```json
{
  "label": "popup",
  "url": "/popup",
  "width": 300,
  "height": 60,
  "decorations": false,
  "transparent": true,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "resizable": false,
  "visible": false,
  "center": true
}
```

2. **Popup Rust side — `popup.rs`:**
```rust
pub fn show(app: &tauri::AppHandle, text: &str, icon: &str) -> Result<(), String> {
    let popup_window = app.get_webview_window("popup")
        .ok_or("Popup window not found")?;

    // Emit event to frontend with text/icon
    popup_window.emit("show-popup", serde_json::json!({
        "text": text,
        "icon": icon,
    })).map_err(|e| format!("{e}"))?;

    // Show window
    popup_window.show().map_err(|e| format!("{e}"))?;

    // Auto-hide after display_ms
    let config = crate::config::load();
    let display_ms = config.popup.display_ms;
    let handle = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(display_ms));
        if let Some(w) = handle.get_webview_window("popup") {
            let _ = w.hide();
        }
    });

    Ok(())
}
```

3. **Popup Svelte page — já existe em `src/routes/popup/+page.svelte`:**
   Atualizar para escutar o evento `show-popup` e renderizar com animação CSS de fade.

### Benefits
- Cross-platform (Linux + Windows + macOS)
- Usa o mesmo design system (Catppuccin)
- Elimina dependência de Python/GTK
- Mais fácil de customizar (é só CSS)

### Consideração
- No Linux, manter o Python/GTK como fallback por enquanto (é mais battle-tested)
- No Windows, usar a Tauri window como única opção
- Futuramente (v0.3.0), migrar Linux também para Tauri window e remover Python/GTK

### Done when
- Popup aparece centered, without decorations, always-on-top
- Mostra texto + ícone com fade out
- Funciona no Windows
- Não quebra no Linux (fallback to Python/GTK)

---

## Task 3.3 — Platform-aware app launcher

**Priority:** P1
**Estimate:** Small
**Files:** `src-tauri/src/lib.rs`, `src/routes/apps/+page.svelte`

### Description
O app launcher precisa de duas mudanças:

1. **Backend:** `launch_app` já foi gated no Epic 01 (Task 1.7). Verificar que funciona com `cmd.exe` / Windows Terminal.

2. **Frontend:** Os apps hardcoded no `+page.svelte` são Linux-specific (virsh, xfreerdp). Adicionar campo `platform` aos apps e filtrar no render.

### Implementation — Frontend

```typescript
interface AppEntry {
  id: string;
  name: string;
  description: string;
  icon: string;
  command: string;
  terminal: boolean;
  platform?: "linux" | "windows" | "all";  // NEW
}

const apps: AppEntry[] = [
  {
    id: "grok-media",
    name: "Grok Media",
    description: "Gerar imagens e vídeos com a API do Grok (xAI)",
    icon: "🎨",
    command: "python3 ~/grok-media/app.py",
    terminal: true,
    platform: "linux",
  },
  // ... etc
];

// Filter by platform
const currentPlatform = navigator.userAgent.includes("Windows") ? "windows" : "linux";
const visibleApps = apps.filter(a => !a.platform || a.platform === "all" || a.platform === currentPlatform);
```

Alternativamente, detectar plataforma via Tauri API: `import { platform } from '@tauri-apps/plugin-os'`.

### Done when
- Linux-only apps não aparecem no Windows
- Windows-only apps não aparecem no Linux
- `launch_app` executa comandos corretamente em ambas plataformas

---

## Task 3.4 — Verify tray icon and minimize-to-tray on Windows

**Priority:** P1
**Estimate:** Small
**File:** `src-tauri/src/tray.rs`

### Description
O tray.rs usa API Tauri pura — deveria funcionar no Windows out of the box. Mas precisa verificar:

1. Ícone aparece na system tray do Windows
2. Menu de contexto abre no right-click
3. CapsLock toggle no menu funciona (depende do Epic 02)
4. "Show" abre a janela
5. "Quit" fecha o app
6. Fechar a janela minimiza para tray (não fecha o app)
7. Ícone tem boa resolução no Windows (pode precisar de `.ico` específico)

### Testing Checklist
- [ ] Tray icon visible in Windows notification area
- [ ] Right-click shows menu
- [ ] "⎋ CapsLock → Escape" toggle works
- [ ] "Show DeskForge" brings window to front
- [ ] "Quit" exits the app
- [ ] Closing window hides to tray
- [ ] Icon is sharp at 100%, 125%, 150% DPI scaling

### Done when
- All checklist items pass
- Document any Windows-specific tray quirks

---

## Task Order & Dependencies

```
3.1 Autostart (Registry)     ──── independent, can start immediately
3.2 Popup (Tauri window)     ──── independent, can start immediately
3.3 App launcher (platform)  ──── depends on Task 1.7 (launch_app gate)
3.4 Tray verification        ──── depends on Epic 02 (remap status)

(3.1 and 3.2 can run in parallel)
```

## Estimated Effort
- **Total:** ~4-6 hours
- **Risk:** Low — mostly integration work, no new complex algorithms
- **Key testing:** DPI scaling on Windows (tray icon, popup positioning)
