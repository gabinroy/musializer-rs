# Musializer-RS

<p align="center">
  <img src="assets/icon.png" width="128" height="128" alt="Musializer-RS Icon" />
</p>

<p align="center">
  <strong>High-Performance, Real-Time Audio Visualizer</strong><br>
  Built with <strong>Rust</strong>, <strong>egui</strong> (Desktop), and <strong>Flutter</strong> (Mobile).
</p>

<p align="center">
  <a href="https://github.com/gabinroy/musializer-rs/actions/workflows/release.yml">
    <img src="https://github.com/gabinroy/musializer-rs/actions/workflows/release.yml/badge.svg" alt="Release Build & Publish" />
  </a>
  <img src="https://img.shields.io/badge/Desktop-Rust%20%2B%20egui-blue?logo=rust" alt="Desktop Ready" />
  <img src="https://img.shields.io/badge/Mobile-Flutter%20%2B%20Rust%20Core-teal?logo=flutter" alt="Mobile Ready" />
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20Windows%20%7C%20macOS%20%7C%20Android%20%7C%20iOS-darkgreen" alt="Cross Platform" />
  <img src="https://img.shields.io/badge/License-GPL--3.0-orange" alt="License" />
</p>

---

## ✨ Features

- ⚡ **Pure Rust DSP Audio Engine**: Lossless decoding via `Symphonia`, low-latency audio via `CPAL`, Hann window FFT (`rustfft`), log frequency band aggregation, and EMA smoothing.
- 📱 **Cross-Platform Mobile App (Android & iOS)**: 120 FPS hardware-accelerated Flutter UI powered by `musializer-core` via `flutter_rust_bridge`.
- 🖥️ **Widescreen Desktop App**: Pure Rust `egui`/`eframe` with instant drag & drop, real-time waveform, spectrum bars, and radial visualizers.
- 🎬 **Offline Video Exporter**: Render complete animated **H.264 MP4 videos** in **9:16 Portrait** (Shorts/Reels/TikTok) and **16:9 Landscape** (YouTube/Desktop) with synchronized stereo AAC audio.
- 🔄 **In-App Auto-Updates**: Seamless GitHub Releases update checker with direct in-app APK download and package installer for Android.
- 🎨 **Rich Visualizer Modes & Custom Covers**: Spectrum Bars, Radial Pulse with customizable center displays (Cover Art with app icon fallback, Track Title, Elapsed/Remaining Time, Glow Core), and Waveform mode.

---

## 🏛️ Project Architecture (Multi-Crate Workspace)

```
musializer-rs/
├── Cargo.toml                    # Root workspace manifest
├── build-mobile.sh               # 1-command release APK build script
│
├── crates/
│   ├── musializer-core/          # Pure Rust DSP, FFT math, Symphonia audio decoding
│   │   ├── src/
│   │   │   ├── api.rs            # flutter_rust_bridge zero-copy interface
│   │   │   ├── audio/            # Decoder (symphonia), Player (cpal), Sync
│   │   │   ├── dsp/              # rustfft, Hann window, EMA smoother, log bands
│   │   │   └── engine.rs         # Unified AudioVisualizerEngine
│   │   ├── build-android.sh      # Android NDK build automation
│   │   └── build-ios.sh          # iOS build automation
│   │
│   └── musializer-desktop/       # High-performance egui / eframe desktop application
│       └── src/
│           ├── main.rs           # Desktop entry point
│           ├── app.rs            # Desktop state loop
│           ├── ui/               # egui controls, visualizer, themes
│           ├── export/           # Video renderer & ffmpeg pipe
│           └── updater/          # In-app GitHub release updater
│
└── mobile/                       # High-performance 120 FPS Flutter mobile app (Android & iOS)
    ├── lib/
    │   ├── main.dart             # Mobile app entry point & lifecycle manager
    │   ├── models/               # Visualizer modes, color themes, center displays
    │   ├── painters/             # 120 FPS CustomPainter visualizers (Bars, Radial, Waveform)
    │   ├── services/             # Hardware MP4 video exporter & GitHub auto-updater
    │   ├── state/                # VisualizerController & real-time ticker
    │   └── widgets/              # Playback controls, mode bar, update dialog, export modal
    ├── android/                  # Android Gradle build & permissions
    └── ios/                      # iOS Runner & Audio Background capabilities
```

---

## 🖥️ Running & Building the Desktop App

The desktop application is built with pure Rust, egui, and wgpu.

### Run Desktop App:
```bash
cargo run --bin musializer-rs
```

### Build Desktop Release Binary:
```bash
cargo build --release --package musializer-desktop
```

---

## 📱 Running & Building the Mobile App (Android & iOS)

The mobile UI is built with **Flutter** and powered by `musializer-core` via **`flutter_rust_bridge`** for 120 FPS zero-copy audio rendering.

### Prerequisites:
- **Rust** & targets:
  ```bash
  rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
  # For iOS (on macOS):
  # rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
  ```
- **Cargo Tools**: `cargo install cargo-ndk flutter_rust_bridge_codegen`
- **Flutter SDK**: `3.7.0+`

### 1-Command Android Release Build:
```bash
./build-mobile.sh
```

### Run on Android Device / Emulator:
```bash
# 1. Compile Rust core libraries for Android:
(cd crates/musializer-core && ./build-android.sh)

# 2. Run Flutter app:
cd mobile
flutter run
```

### Run on iOS (macOS / Xcode):
```bash
# 1. Compile Rust core for iOS:
(cd crates/musializer-core && ./build-ios.sh)

# 2. Run Flutter iOS app:
cd mobile
flutter run -d ios
```

---

## 🧪 Testing & Verification

Run the entire test suite across all workspace crates and the mobile app:

```bash
# Rust Workspace Unit Tests (DSP, decoding, audio sync, desktop UI)
cargo test --workspace

# Flutter Mobile Code Analysis & Unit Tests
cd mobile && flutter analyze
```

---

## 📄 License
This project is licensed under the GPL-3.0-or-later License.
