# Musializer Mobile (Android & iOS)

High-Performance, Real-Time Audio Visualizer Mobile Client powered by `musializer-core` (Rust) and Flutter.

---

## ✨ Features

- ⚡ **120 FPS Real-Time Audio Visualizer**: Zero-copy bridge powered by `flutter_rust_bridge` and `musializer-core`.
- 📊 **Multiple Visualization Modes**:
  - **Spectrum Bars**: 64 neon equalizer bars with dynamic peak hold decay.
  - **Radial Pulse**: 360° pulsating circular visualizer with customizable center modes (Custom Cover Art with app logo fallback, Track Title, Elapsed/Remaining Time, Glow Core).
  - **Waveform**: Smooth Bezier ribbon oscilloscope.
- 🎨 **Curated Color Themes**: Cyberpunk Neon, Synthwave Purple, Emerald Matrix, Deep Ocean, Sunset Fire, Obsidian Gold.
- 🎬 **Offline Hardware MP4 Video Exporter**:
  - Encodes real H.264 video + synchronized stereo AAC audio MP4 files directly to the device Movies gallery.
  - Supports **9:16 Portrait** (Shorts/Reels/TikTok) and **16:9 Landscape** (YouTube/Desktop).
- 🔄 **In-App Auto-Updates**: Seamless GitHub Releases version check with direct in-app APK download and package installer for Android.
- ⏸️ **Smart Lifecycle Management**: Automatically pauses audio playback when minimized or backgrounded.

---

## 🛠️ Build & Run

### Prerequisites
- **Flutter SDK** (`3.7.0+`)
- **Android SDK & NDK** (`ndk 30.x+`)
- **Rust & Cargo** with `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android` targets

### Run Locally on Android Device / Emulator
```bash
# 1. Compile core shared library for Android
(cd ../crates/musializer-core && ./build-android.sh)

# 2. Run Flutter mobile app
flutter run
```

### Run on iOS (macOS / Xcode)
```bash
# 1. Compile core library for iOS
(cd ../crates/musializer-core && ./build-ios.sh)

# 2. Run Flutter iOS app
flutter run -d ios
```

---

## 📄 License
This project is licensed under the GPL-3.0-or-later License.
