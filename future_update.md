# Musializer-RS: Future Updates & Roadmap

This document outlines upcoming architectural enhancements, bug fixes, and feature additions planned for future development sessions.

---

## 1. Wayland / KDE Plasma Drag-and-Drop Stability (CachyOS Fix) [COMPLETED]

### Issue Summary
- On X11 / Cinnamon (e.g. Linux Mint), drag-and-drop works seamlessly.
- On modern Wayland environments (specifically **KDE Plasma 6 on CachyOS / Arch**), dragging files from Dolphin into the window caused failures or unhandled URI types due to `text/uri-list` payload formats and percent-encoded file paths (`file://...`).

### Completed Implementation
1. **MIME & URI Sanitization in [`src/ui/drag_drop.rs`](file:///home/abin/Documents/Github/musializer-rs/src/ui/drag_drop.rs)**:
   - Added pure-Rust percent-decoding (`%20`, symbols, etc.) and `file://` URI prefix stripping.
   - Added multi-line `text/uri-list` byte buffer parser that cleanly extracts and decodes dropped files when Wayland/Dolphin passes URI byte lists instead of direct OS path objects.
   - Added automated unit tests (`test_percent_decode_spaces_and_symbols`, `test_parse_uri_local_and_raw`).
2. **Backend Override & XWayland Fallback in [`src/main.rs`](file:///home/abin/Documents/Github/musializer-rs/src/main.rs)**:
   - Added `--x11` CLI flag to force X11/XWayland backend (`WINIT_UNIX_BACKEND=x11`) when desired.
   - Added `--wayland` CLI flag to force native Wayland backend.
   - Added `--help` / `-h` and `--version` / `-v` CLI options.

---

## 2. In-App Automated Updating System

### Objective
Eliminate manual downloads of `.tar.gz`, `.deb`, and `.AppImage` files from GitHub releases by introducing modern in-app update mechanisms and automated package repositories.

### Planned Action Items
1. **GitHub Releases In-App Update Checker**:
   - Add a background async task / thread that checks `https://api.github.com/repos/gabinroy/musializer-rs/releases/latest`.
   - Compare with `env!("CARGO_PKG_VERSION")`.
   - If a new version is detected:
     - Show an unobtrusive update indicator in the top/bottom panel.
     - Provide an **"Update Available"** modal with release notes, changelog, and a 1-click **"Download & Install"** button.
2. **Self-Updating Binary Replacement (`self_update`)**:
   - On Linux (AppImage/Binary) and Windows (portable `.exe`), integrate binary self-replacement to swap the binary and prompt for a seamless restart.

---

## 3. Package Management & Distribution Pipeline

### Objective
Enable standard Linux package manager updates for users across different distributions.

### Planned Action Items
1. **Arch User Repository (AUR) Packaging (`musializer-rs` & `musializer-rs-bin`)**:
   - Create a `PKGBUILD` for Arch Linux / CachyOS / Manjaro.
   - Allow users to install and receive rolling updates via `yay -S musializer-rs` or `paru -S musializer-rs`.
   - Add a GitHub Actions step to [.github/workflows/release.yml](file:///home/abin/Documents/Github/musializer-rs/.github/workflows/release.yml) to automatically update the AUR repository on tag pushes.
2. **Flathub (Flatpak Distribution)**:
   - Create a Flatpak manifest for distribution on Flathub, providing sandboxed, universal auto-updates across all Linux distributions.
3. **Windows Winget / Scoop Integration**:
   - Add automated manifests for Windows Package Manager (`winget`) and `scoop`.

---

## 4. UI/UX & Visualization Enhancements

1. **Custom Shader Presets & Post-Processing**:
   - Explore glow/bloom post-processing shaders for export rendering and real-time visualization.
2. **Playlist & Batch Audio Queue**:
   - Support loading multiple tracks with seamless autoplay and queue management.
3. **Advanced Audio Codec Tag Extraction**:
   - Extract embedded album cover art (ID3 APIC / Vorbis comments) automatically from audio files upon loading.





In-App Automated Update System:
Background GitHub Release version checker with in-app notification badge.
"Update Available" modal with changelog and 1-click update/binary replacement.

Automated Package Distribution:
Arch Linux / CachyOS AUR packaging (yay -S musializer-rs) with automated GitHub Actions publishing.
Flathub (Flatpak) & Winget distribution setup.

Additional Visualizer Roadmap Items:
Automatic ID3/Vorbis embedded album cover extraction, batch audio queue, and visual enhancements.