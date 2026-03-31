# DeskForge — Claude Code Context

## What is this?

A Tauri 2 + Svelte 5 desktop app for Linux (Pop!_OS/GNOME) that replaces scattered shell scripts with a unified control panel. Think "System Preferences for power users."

## Build & Run

```bash
npm install
npm run tauri dev      # Dev with hot-reload (Vite on :1420)
npm run tauri build    # Production .deb
```

## Key Architecture Decisions

- **Remapping**: Uses GNOME gsettings XKB options (`caps:escape`), not kernel udev/hwdb. Non-invasive, no root for toggle.
- **NuPhy F-keys**: Direct sysfs write to `/sys/module/hid_apple/parameters/fnmode`. Falls back to `pkexec` for permissions. Persistence via `/etc/modprobe.d/hid_apple.conf`.
- **pkexec calls run in spawned threads** to avoid blocking the Tauri main thread (learned the hard way — freezes the whole app).
- **Popup overlay**: Uses legacy Python GTK3 script, not a Tauri window. More reliable on X11/Wayland.
- **Macros**: xclip for clipboard, xdotool for key simulation. Two methods: clipboard paste or simulated typing.
- **Config**: TOML at `~/.config/deskforge/config.toml`, auto-created on first run.

## Frontend

- Svelte 5 with `$state` and `$derived` runes (not stores)
- `@tauri-apps/api/core` `invoke()` for all backend calls — **only works inside Tauri webview**, not in browser
- Theme: Catppuccin Mocha with CSS variables in `app.css` (prefix `--`)
- Font: JetBrains Mono / Fira Code (monospace)

## Rust Backend

- All commands in `lib.rs` via `#[tauri::command]`
- Keyboard logic in `src-tauri/src/modules/keyboard/`
- Platform-specific code in `platform/linux.rs`

## Important Gotchas

- `invoke()` throws "Cannot read properties of undefined" when accessed from a regular browser (localhost:1420). Must use the Tauri window.
- `pkexec` (password dialog) blocks the calling thread. Always spawn in a separate thread.
- `persist_fnmode()` runs `update-initramfs -u` which takes several seconds — must be async/background.
- NuPhy keyboard detection reads `/proc/bus/input/devices` for "NuPhy" string.

## External CLI Dependencies

gsettings, xdotool, xclip, pkexec, python3, alacritty (terminal)
