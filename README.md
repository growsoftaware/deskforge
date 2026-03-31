# DeskForge

**Your desktop, your rules.** A single app to bend your Linux desktop to your workflow — key remapping, hardware fixes, app launchers, macros, and more.

Built for power users who want a unified control panel instead of scattered shell scripts and gsettings commands.

## Why DeskForge?

Every Linux power user ends up with a `~/scripts/` folder full of hacks: a shell script to remap CapsLock, another to fix NuPhy F-keys, aliases to start VMs. These work, but they're fragile, hard to toggle, and invisible.

DeskForge wraps all of that into a native app with:
- **One-click toggles** instead of terminal commands
- **System tray** presence so it's always accessible
- **Global shortcuts** that work from any app
- **Persistent config** that survives reboots
- **Visual feedback** via popup overlays

## Features

### Keyboard Manager
- **CapsLock → Escape** toggle via GNOME XKB (no root needed)
- **NuPhy F-keys** fix — switch between F1-F12 and media keys (kernel sysfs)
- **Global shortcuts** — `Super+Escape` toggles CapsLock from anywhere
- **Text macros** — key combos that paste custom text (clipboard or simulated typing)

### App Launcher
- **Quick launch** for frequently used tools
- **VM management** — start libvirt VMs and connect via RDP in one click
- **Terminal or background** execution modes

### System Integration
- **System tray** with status display and quick toggles
- **Popup overlay** — minimal notification with fade animation
- **Autostart** via XDG desktop entry
- **Catppuccin Mocha** theme with monospace typography

## Tech Stack

| Layer | Tech |
|-------|------|
| Backend | Rust + Tauri 2.0 |
| Frontend | Svelte 5 + TypeScript |
| Build | Vite 6, SvelteKit (static adapter) |
| Config | TOML (`~/.config/deskforge/config.toml`) |
| Theme | Catppuccin Mocha + JetBrains Mono |
| Platform | Linux (Pop!_OS / GNOME) |

## Quick Start

### Prerequisites

```bash
# System dependencies (Ubuntu/Pop!_OS)
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev \
  build-essential xdotool xclip

# Rust 1.88+ and Node.js 18+
```

### Development

```bash
git clone https://github.com/growsoftaware/deskforge.git
cd deskforge
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
# Output: src-tauri/target/release/bundle/deb/deskforge_*.deb
```

## Architecture

```
src-tauri/src/
  lib.rs              — Tauri setup, 13 invoke commands
  config.rs           — TOML config with serde
  tray.rs             — System tray with dynamic menu
  popup.rs            — GTK3 popup overlay (Python)
  autostart.rs        — XDG autostart management
  shortcuts.rs        — Global shortcut registration
  modules/keyboard/
    remapper.rs       — XKB remap state (gsettings)
    devices.rs        — Hardware fixes (NuPhy fnmode via sysfs)
    macros.rs         — Text macro execution (xclip + xdotool)
    platform/linux.rs — gsettings/XKB wrapper

src/routes/
  +layout.svelte      — Shell: sidebar, nav, autostart toggle
  keyboard/+page.svelte — Remaps, device fixes, macro editor
  apps/+page.svelte   — App launcher grid
```

## Config

Auto-created at `~/.config/deskforge/config.toml` on first run:

```toml
[app]
start_on_login = false
start_minimized = true

[keyboard]
enabled = true

[[keyboard.remaps]]
id = "capslock-escape"
source = "CapsLock"
target = "Escape"
active = true

[[keyboard.macros]]
id = "email-sig"
name = "Email Signature"
trigger = "Ctrl+Shift+E"
text = "Best regards,\nAndre"
method = "clipboard"
```

## Roadmap

- [ ] **Wayland support** — migrate from xdotool/xclip to wtype/wl-clipboard
- [ ] **Window management** — tiling shortcuts, workspace rules
- [ ] **Display profiles** — monitor layout presets (xrandr/wlr-randr)
- [ ] **Audio routing** — per-app volume and output device control (PipeWire)
- [ ] **Dotfile sync** — backup/restore config across machines
- [ ] **Plugin system** — user-defined modules with hot-reload
- [ ] **Cross-platform** — Windows and macOS support via Tauri's native APIs

## License

MIT
