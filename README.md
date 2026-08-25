# Musializer-RS 🎵

<p align="center">
  <img src="assets/icon.jpg" alt="Musializer-RS Icon" width="160" style="border-radius: 24px;" />
</p>

<p align="center">
  <strong>A high-performance, universal cross-platform audio visualizer written in Rust.</strong><br>
  <em>Inspired by <a href="https://github.com/tsoding/musializer">tsoding/musializer</a>.</em>
</p>

Musializer-RS brings native GPU-accelerated rendering, real-time Fourier analysis (DSP), multi-format audio playback, touch-first mobile responsiveness, and deterministic 60 FPS video export across **Linux, Windows, macOS, Android, iOS, and Web (WebAssembly)**.

---

## 🌟 Key Features

- **Cross-Platform Support**:
  - 🖥️ **Desktop**: Linux (Wayland/X11), Windows (WASAPI/DirectX), macOS (CoreAudio/Metal).
  - 📱 **Mobile**: Android (`cargo-apk` NativeActivity), iOS (Xcode Universal static library).
  - 🌐 **Web**: WebAssembly (WASM + WebGPU/WebGL) via `trunk`.
- **Multi-Format Audio Support**: Decode and play MP3, WAV, FLAC, OGG/Vorbis, and AAC files via pure-Rust `symphonia` (file-based or in-memory buffer decoding).
- **Hardware-Accelerated UI**: Built with `eframe` (egui + wgpu) for 60+ FPS rendering across all targets.
- **Touch-First Mobile UI**:
  - Responsive stacking layout for mobile portrait mode (< 600px width).
  - Large 44x44px touch hitboxes for finger navigation.
  - Adaptive prompt states ("Load Audio" tap target for mobile / drag-and-drop for desktop).
- **Fast Fourier Transform (DSP)**:
  - Hann windowing to eliminate spectral leakage.
  - SIMD-accelerated FFT via `rustfft`.
  - Logarithmic / Octave frequency band grouping (Sub-bass to Treble).
  - Asymmetric Attack/Decay Exponential Moving Average (EMA) with peak hold dots for fluid animations.
  - **Dynamic Range Compression & Sensitivity Boost**: Power-law scaling and adjustable `0.5x` to `3.5x` gain slider to ensure quiet intros and acoustic tracks generate rich, energetic spectrum bars.
- **Multiple Visualizer Modes**:
  - **Dynamic Spectrum Bars**: Colorful gradient bars with reactive heights and floating peak caps.
  - **Oscilloscope Waveform**: Smooth antialiased audio waveform.
  - **Circular / Radial Pulse**: Radial frequency burst with bass pulse and customizable center hub (Custom Cover Art / Album Image, Live Elapsed Time, Remaining Time Countdown, or Song Title).
- **Deterministic 60 FPS Video Export** (Desktop):
  - Offline frame stepping with zero dropped frames.
  - Automatic FFmpeg piping for direct `.mp4` video generation with muxed audio.
  - Native screen-recording helper prompt on mobile and web platforms.

---

## 🏛️ Documentation

- 📐 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): Detailed technical architecture, math equations, and dataflow diagrams.
- 📦 [docs/APP_EXPORTING_COMMANDS.md](docs/APP_EXPORTING_COMMANDS.md): Full command-line reference for exporting & packaging apps for all platforms.
- 📱 [docs/MOBILE_DEPLOYMENT.md](docs/MOBILE_DEPLOYMENT.md): Step-by-step guide for building Android APKs and iOS Xcode libraries.
- 🌐 [docs/WEB_DEPLOYMENT.md](docs/WEB_DEPLOYMENT.md): Guide for compiling and deploying the WebAssembly web app with Trunk.

---

## 🚀 Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/gabinroy/musializer-rs.git
cd musializer-rs
```

### 2. Run on Desktop (Linux, macOS, Windows)
```bash
cargo run --release
```

### 3. Run on Web (WebAssembly)
```bash
# Install Trunk
cargo install trunk

# Serve locally at http://127.0.0.1:8080
trunk serve
```

### 4. Build for Android (APK)
```bash
cargo apk build --release
# Output located at: target/release/apk/Musializer-RS.apk
```

### 5. Build for iOS (Universal Library)
```bash
cargo lipo --release
# Output located at: target/universal/release/libmusializer_rs.a
```

---

## 🧪 Tests & Quality
- `cargo test`: 100% tests passing.
- `cargo check`: 0 errors, 0 warnings across all targets.

---

## 📄 License
This project is licensed under the **GNU General Public License v3.0** ([GPLv3](LICENSE)).
