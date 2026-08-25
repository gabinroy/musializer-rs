# Musializer-RS: Desktop App Export & Packaging Commands

This document contains command-line instructions for building, packaging, and exporting **Musializer-RS** across desktop platforms (Linux, Windows, and macOS).

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

### Linux Packaging (AppImage, Debian `.deb`, & Tarball)

#### 1. Debian Package (`.deb`)
```bash
# Install cargo-deb
cargo install cargo-deb

# Generate .deb package
cargo deb
# Output at: target/debian/musializer-rs_*.deb

# Install on Debian/Ubuntu/Mint
sudo dpkg -i target/debian/musializer-rs_*.deb
```

#### 2. Universal Linux AppImage / Standalone Bundle
```bash
# Install cargo-bundle
cargo install cargo-bundle

# Generate universal AppImage / Linux bundle
cargo bundle --release
# Output at: target/release/bundle/deb/ or target/release/bundle/appimage/
```

#### 3. Linux Desktop Entry & Icon Installation
```bash
# Copy binary to system path
sudo cp target/release/musializer-rs /usr/local/bin/

# Install app icon
sudo mkdir -p /usr/local/share/icons/hicolor/256x256/apps/
sudo cp assets/icon.png /usr/local/share/icons/hicolor/256x256/apps/musializer-rs.png

# Create desktop launcher file
cat << 'EOF' > ~/.local/share/applications/musializer-rs.desktop
[Desktop Entry]
Name=Musializer-RS
Comment=High-Performance Cross-Platform Audio Visualizer
Exec=musializer-rs
Icon=musializer-rs
Terminal=false
Type=Application
Categories=AudioVideo;Audio;Player;
EOF
```

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

## 🤖 2. Automated Multi-Platform Releases (GitHub Actions)

The repository includes a unified CI/CD workflow located at [`.github/workflows/release.yml`](../.github/workflows/release.yml) that builds and publishes releases for all platforms without local OS dependencies.

### Triggering a Release via Git Tag:
```bash
git tag v0.1.0
git push origin v0.1.0
```

### Triggering a Release Manually (with Custom Release Notes):
1. Navigate to **Actions** $\rightarrow$ **Release Build & Publish** in your GitHub repository.
2. Click **Run workflow**.
3. Fill in the release parameters:
   - **Release Tag** (e.g., `v0.1.0`)
   - **Release Title** (e.g., `Musializer v0.1.0 — Initial Release`)
   - **Custom Release Notes** (Markdown notes describing what changed)
4. Click **Run workflow**.

Generated artifacts uploaded automatically to the GitHub Release:
- **Linux**: `.AppImage`, `.deb`, `.tar.gz`
- **Windows**: `.msi` installer, `.zip` (containing `musializer-rs.exe`)
- **macOS (Intel & Apple Silicon)**: `.app.zip`, `.tar.gz`


---

## 🎬 2. In-App Video Exporting (MP4 Video Rendering)

To render high-definition 60 FPS visualizer `.mp4` videos with muxed audio:

1. Launch Musializer-RS: `cargo run --release`
2. Load an audio track (`.mp3`, `.wav`, `.flac`, `.ogg`, `.aac`).
3. Click the **"🎬 Export Video"** button in the top right.
4. Select resolution (1080p / 720p), framerate (60 FPS / 30 FPS), and output filename.
5. Click **"🚀 Start Export"**. The offline engine renders frame-by-frame and muxes directly with FFmpeg.

