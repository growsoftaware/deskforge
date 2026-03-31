# Tasks — Epic 04: Polish, CI/CD & Installer

## Task 4.1 — GitHub Actions CI/CD for dual-platform build

**Priority:** P0
**Estimate:** Medium
**File:** `.github/workflows/build.yml`

### Description
Criar workflow que builda o app em Linux e Windows automaticamente.

### Implementation

```yaml
name: Build

on:
  push:
    branches: [main]
    tags: ['v*']
  pull_request:
    branches: [main]

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-22.04
            target: x86_64-unknown-linux-gnu
            name: linux
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            name: windows

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            src-tauri/target
          key: ${{ runner.os }}-cargo-${{ hashFiles('src-tauri/Cargo.lock') }}

      - name: Install Linux deps
        if: matrix.name == 'linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev \
            libayatana-appindicator3-dev librsvg2-dev xdotool xclip

      - name: Install frontend deps
        run: npm ci

      - name: Build Tauri app
        run: npm run tauri build

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: deskforge-${{ matrix.name }}
          path: |
            src-tauri/target/release/bundle/deb/*.deb
            src-tauri/target/release/bundle/appimage/*.AppImage
            src-tauri/target/release/bundle/msi/*.msi
            src-tauri/target/release/bundle/nsis/*.exe
          if-no-files-found: ignore

  release:
    needs: build
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/download-artifact@v4

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            deskforge-linux/**
            deskforge-windows/**
          generate_release_notes: true
```

### Testing
- Push to a branch and verify both builds pass
- Create a test tag and verify release is created with artifacts

### Done when
- Both Linux and Windows builds succeed in CI
- Artifacts downloadable from Actions tab
- Release auto-created on tag push

---

## Task 4.2 — Windows icon and installer configuration

**Priority:** P1
**Estimate:** Small
**Files:** `src-tauri/tauri.conf.json`, `src-tauri/icons/`

### Description
Configurar o ícone Windows (.ico) e opções do installer MSI/NSIS.

### Steps
1. Gerar ícone `.ico` a partir do ícone existente (Tauri CLI tem `tauri icon` command)
2. Atualizar `tauri.conf.json`:

```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

3. Configurar NSIS installer (melhor UX que MSI no Windows):

```json
{
  "bundle": {
    "targets": ["deb", "appimage", "nsis", "msi"],
    "nsis": {
      "oneClick": false,
      "allowElevation": true,
      "installerIcon": "icons/icon.ico",
      "displayLanguageSelector": false,
      "languages": ["English", "PortugueseBR"]
    }
  }
}
```

### Done when
- `tauri icon` generates all required formats
- MSI and NSIS installers build correctly
- Icon shows in Windows taskbar, tray, and Alt+Tab

---

## Task 4.3 — Uninstall cleanup

**Priority:** P2
**Estimate:** Small

### Description
Quando o app é desinstalado no Windows, garantir que:
1. Registry autostart key é removida
2. Config directory (`%APPDATA%\deskforge\`) é opcionalmente removida

### Implementation
Tauri NSIS installer supports `installScript` and `uninstallScript` hooks.

Criar `src-tauri/nsis/uninstall.nsh`:
```nsis
; Remove autostart registry key
DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "DeskForge"

; Ask user if they want to remove config
MessageBox MB_YESNO "Remover configurações do DeskForge?" IDNO skip_config
  RMDir /r "$APPDATA\deskforge"
skip_config:
```

### Done when
- Autostart key removed on uninstall
- User prompted about config removal
- Clean uninstall verified in "Add or Remove Programs"

---

## Task 4.4 — Update README with Windows instructions

**Priority:** P1
**Estimate:** Small
**File:** `README.md`

### Description
Adicionar seção de instalação Windows ao README.

### Content to add
- Download link para release (.msi ou .exe)
- Prerequisites (nenhum no Windows! Tauri bundles WebView2)
- Known limitations (macros don't work in elevated apps, etc)
- Screenshots de ambas plataformas

### Done when
- README has clear Windows installation section
- Known limitations documented
- Download links point to GitHub Releases

---

## Task 4.5 — Create CHANGELOG.md for v0.2.0

**Priority:** P1
**Estimate:** Small
**File:** `CHANGELOG.md`

### Content

```markdown
# Changelog

## [0.2.0] — Bifrost — YYYY-MM-DD

### Added
- **Windows support** — DeskForge now runs natively on Windows 10/11
- CapsLock → Escape toggle via low-level keyboard hook (instant, no reboot)
- Text macros on Windows via arboard + enigo
- Autostart via Windows Registry
- Windows installer (.msi and .exe)
- Platform-aware app launcher (filters apps by OS)
- CI/CD with GitHub Actions (dual-platform builds)

### Changed
- Popup notification: migrated from Python/GTK to Tauri webview window (cross-platform)
- UI redesign: Catppuccin Mocha theme with monospace typography

### Fixed
- pkexec calls no longer freeze the app (spawned in background thread)

### Known Limitations (Windows)
- Text macros don't work in apps running as Administrator (UAC elevation)
- NuPhy F-key fix is Linux-only (NuPhy has its own Windows software)
```

### Done when
- CHANGELOG.md exists with accurate v0.2.0 notes
- Version in `Cargo.toml` and `tauri.conf.json` updated to `0.2.0`

---

## Task 4.6 — Tag and release v0.2.0

**Priority:** P0 (final gate)
**Estimate:** Small

### Steps
1. Merge `feature/windows-platform` → `main`
2. Update version in:
   - `src-tauri/Cargo.toml` → `version = "0.2.0"`
   - `src-tauri/tauri.conf.json` → `"version": "0.2.0"`
   - `package.json` → `"version": "0.2.0"`
3. Commit: `chore: bump version to 0.2.0`
4. Tag: `git tag -a v0.2.0 -m "Bifrost: Windows platform support"`
5. Push: `git push origin main --tags`
6. Verify GitHub Actions creates release with artifacts
7. Download and test both `.deb` and `.msi` on clean machines

### Done when
- Tag `v0.2.0` exists
- GitHub Release has Linux + Windows artifacts
- Both installers verified on clean OS installs

---

## Task Order & Dependencies

```
4.1 CI/CD workflow        ──── can start immediately (test with stubs)
4.2 Icons & installer     ──── can start immediately
4.3 Uninstall cleanup     ──── after 4.2
4.4 README update         ──── after all features work
4.5 CHANGELOG             ──── after all features work
4.6 Tag & release         ──── LAST (after everything)
```

## Estimated Effort
- **Total:** ~3-4 hours
- **Risk:** Low — mostly configuration and documentation
- **Key testing:** Install/uninstall cycle on clean Windows VM
