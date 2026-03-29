# DeskForge

Cross-platform desktop utility hub. Manage keyboard remappings, custom shortcuts, text macros, and more — from a single app.

**Status:** In development (Linux first)

## Tech Stack

- **Backend:** Rust + Tauri 2.0
- **Frontend:** Svelte 5 + TypeScript
- **Config:** TOML (`~/.config/deskforge/config.toml`)

## Modules

### Keyboard Manager
- Key remapping (e.g., CapsLock → Escape toggle)
- Custom keyboard shortcuts
- Text macros (trigger combo → paste custom text)

## Development

```bash
# Install dependencies
npm install

# Run in dev mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Legacy

The `legacy/` directory contains the original shell/Python scripts that inspired this project — a CapsLock↔Escape toggle with popup overlay for Pop!_OS/GNOME.

## License

MIT
