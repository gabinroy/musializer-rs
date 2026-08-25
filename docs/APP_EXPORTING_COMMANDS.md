# Musializer-RS: App Export & Packaging Commands

This document contains command-line instructions for building, packaging, and exporting **Musializer-RS** across all supported target platforms.

---

## 🖥️ 1. Desktop Releases (Linux, Windows, macOS)

### Native Release Binary
```bash
# Build optimized desktop binary
cargo build --release

# Run release binary
cargo run --release
```
- **Linux**: Output at `target/release/musializer-rs`
- **Windows**: Output at `target/release/musializer-rs.exe`
- **macOS**: Output at `target/release/musializer-rs`

### macOS Application Bundle (.app)
```bash
# Install cargo-bundle
cargo install cargo-bundle

# Generate macOS .app bundle
cargo bundle --release
# Output at: target/release/bundle/osx/Musializer-RS.app
```

### Windows Installer (.msi / .exe)
```bash
# Install cargo-wix for MSI installers
cargo install cargo-wix
cargo wix --release
```

---

## 📱 2. Android APK Packaging

### Debug & Release APK
```bash
# Prerequisites
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo install cargo-apk cargo-ndk

# Build Debug APK
cargo apk build

# Build Optimized Release APK
cargo apk build --release
```
- **Output APK**: `target/release/apk/Musializer-RS.apk`

### Install on Device via ADB
```bash
adb install target/release/apk/Musializer-RS.apk
```

---

## 🍏 3. iOS Xcode Static Library Export

### Universal iOS Library (Device + Simulator)
```bash
# Prerequisites
rustup target add aarch64-apple-ios x86_64-apple-ios
cargo install cargo-lipo

# Build Universal Static Library
cargo lipo --release
```
- **Output Library**: `target/universal/release/libmusializer_rs.a`
- Link `libmusializer_rs.a` into your Xcode iOS project alongside `AVFoundation`, `CoreAudio`, `Metal`, and `QuartzCore`.

---

## 🌐 4. WebAssembly (WASM) Static Web Export

### Local Development Server
```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install trunk

# Start local server at http://127.0.0.1:8080
trunk serve
```

### Production Web Distribution (GitHub Pages / Netlify / Vercel)
```bash
# Build optimized static distribution
trunk build --release
```
- **Output Directory**: `dist/` (contains `index.html`, WASM binary, JS loader, and assets).

---

## 🎬 5. In-App Video Exporting (MP4 Video Rendering)

To render high-definition 60 FPS visualizer `.mp4` videos with muxed audio:

1. Launch Musializer-RS: `cargo run --release`
2. Load an audio track (`.mp3`, `.wav`, `.flac`, `.ogg`, `.aac`).
3. Click the **"🎬 Export Video"** button in the top right.
4. Select resolution (1080p / 720p), framerate (60 FPS / 30 FPS), and output filename.
5. Click **"🚀 Start Export"**. The offline engine renders frame-by-frame and muxes directly with FFmpeg.
