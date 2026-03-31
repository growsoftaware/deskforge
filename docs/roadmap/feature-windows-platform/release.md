# Release Plan — DeskForge v0.2.0: Windows Platform Support

## Vision

DeskForge nasceu como um "painel de controle pessoal" para Linux. A v0.2.0 transforma ele num app verdadeiramente cross-platform, rodando nativamente no Windows com as mesmas features — sem emulação, sem WSL, sem gambiarras.

O objetivo não é portar código Linux para Windows. É **criar uma camada de abstração de plataforma** que permita que qualquer feature futura funcione em ambos os sistemas com zero fricção.

## Release Target

| Campo | Valor |
|-------|-------|
| Versão | `0.2.0` |
| Codename | `Bifrost` (a ponte entre dois mundos) |
| Branch | `feature/windows-platform` |
| Milestone | Windows Feature Parity |
| Merge target | `main` |

## Scope

### In Scope
- Compilação e execução nativa no Windows 10/11
- CapsLock → Escape toggle (via Windows Registry Scancode Map)
- Text macros com clipboard e simulated input (via `arboard` + `enigo`)
- System tray (já cross-platform via Tauri 2)
- Global shortcuts (já cross-platform via `tauri-plugin-global-shortcut`)
- Popup de notificação (migrar de Python/GTK para Tauri webview window)
- Autostart via Windows Registry (`HKCU\...\Run`)
- App launcher com `cmd.exe` / `powershell` / Windows Terminal
- Installer `.msi` via Tauri bundler
- CI/CD para build dual-platform (GitHub Actions)

### Out of Scope (v0.2.0)
- macOS support (planejado para v0.3.0)
- NuPhy F-keys no Windows (o driver HID Apple é Linux-only; NuPhy tem software próprio no Windows)
- GUI de configuração para escolher remaps customizados (além de CapsLock)
- Plugin system
- Auto-updater

## Architecture Decision: Platform Abstraction

A decisão mais importante desta release é **como** abstrair plataforma. Existem 3 abordagens:

### Opção A: `#[cfg]` direto (escolhida)
```rust
// platform/mod.rs
#[cfg(target_os = "linux")]  pub mod linux;
#[cfg(target_os = "windows")] pub mod windows;
#[cfg(target_os = "linux")]  pub use linux::*;
#[cfg(target_os = "windows")] pub use windows::*;
```
**Por quê:** Simples, zero overhead, o compilador elimina o código da outra plataforma. Cada módulo de plataforma exporta as mesmas funções com as mesmas assinaturas. Sem traits, sem dyn dispatch, sem indireção. DeskForge não é um framework — é um app que roda em 2 plataformas. KISS.

### Opção B: Trait `Platform` (rejeitada)
Mais flexível, mas over-engineering para 2 plataformas. Adiciona indireção desnecessária e boilerplate.

### Opção C: Crates cross-platform como `enigo`/`arboard` everywhere (parcial)
Usaremos `arboard` e `enigo` para macros (onde faz sentido), mas o remapping de teclas não tem crate cross-platform — cada OS tem seu mecanismo (XKB vs Scancode Map).

## Delivery Strategy

### 4 Epics, entregues sequencialmente:

```
Epic 1: Platform Abstraction Layer     ─── O app COMPILA no Windows
         │
Epic 2: Windows Keyboard Engine        ─── Remaps e macros FUNCIONAM
         │
Epic 3: System Integration             ─── Autostart, popup, launcher
         │
Epic 4: Polish, CI/CD & Installer      ─── DISTRIBUÍVEL para usuários
```

Cada epic é um PR mergeable. Cada um deixa o app num estado funcional — nunca quebra `main`.

## Risk Assessment

| Risco | Impacto | Mitigação |
|-------|---------|-----------|
| Scancode Map exige reboot | Alto — UX ruim vs Linux toggle instantâneo | Implementar low-level keyboard hook (`SetWindowsHookEx`) como alternativa runtime sem reboot |
| `enigo` não funciona com UAC/admin apps | Médio — macros não colam em apps elevados | Documentar limitação; futuramente explorar UI Access |
| Tauri webview popup pode ter z-order issues | Baixo | Fallback: Win32 `CreateWindowEx` com `WS_EX_TOPMOST` |
| GitHub Actions Windows build lento | Baixo | Cache de Cargo e node_modules |

## Definition of Done

- [ ] `npm run tauri build` produz `.msi` funcional no Windows
- [ ] CapsLock ↔ Escape toggle funciona sem reboot (hook mode)
- [ ] Text macros colam texto via clipboard em apps normais
- [ ] Global shortcut `Super+Escape` funciona
- [ ] System tray aparece com status correto
- [ ] Popup de notificação aparece e some com fade
- [ ] Autostart toggle funciona (Registry)
- [ ] App launcher executa comandos via `cmd.exe`/`powershell`
- [ ] CI/CD produz artifacts para Linux (.deb) e Windows (.msi)
- [ ] Zero warnings no `cargo clippy` em ambas plataformas

## Rollback Plan

A branch `feature/windows-platform` é isolada. Se a release falhar:
1. Não mergear na `main`
2. O app Linux continua funcionando como antes
3. Nenhum código Linux existente é deletado — apenas gated com `#[cfg]`
