# Musializer-RS 🎵

<p align="center">
  <img src="assets/icon.jpg" alt="Musializer-RS Icon" width="160" style="border-radius: 24px;" />
</p>

<p align="center">
  <strong>A high-performance, GPU-accelerated audio visualizer written in Rust.</strong><br>
  <em>Inspired by <a href="https://github.com/tsoding/musializer">tsoding/musializer</a>.</em>
</p>

Musializer-RS brings native GPU-accelerated rendering, real-time Fourier analysis (DSP), multi-format audio playback, and deterministic 60 FPS video export for **Desktop (Linux, Windows, and macOS)**.

---

## 🌟 Key Features

- 🖥️ **Stable Desktop Support**: Native 60+ FPS playback and rendering on Linux (Wayland/X11), Windows (WASAPI/DirectX), and macOS (CoreAudio/Metal).
- 🎵 **Multi-Format Audio Support**: Decode and play MP3, WAV, FLAC, OGG/Vorbis, and AAC files via pure-Rust `symphonia`.
- ⚡ **Hardware-Accelerated UI**: Built with `eframe` (egui + wgpu) for ultra-low latency, fluid animations.
- 🧮 **Fast Fourier Transform (DSP)**:
  - Hann windowing to eliminate spectral leakage.
  - SIMD-accelerated FFT via `rustfft`.
  - Logarithmic / Octave frequency band grouping (Sub-bass to Treble).
  - Asymmetric Attack/Decay Exponential Moving Average (EMA) with peak hold dots for fluid animations.
  - **Dynamic Range Compression & Sensitivity Boost**: Power-law scaling and adjustable `0.5x` to `3.5x` gain slider to ensure quiet intros and acoustic tracks generate rich, energetic spectrum bars.
- 🎨 **Multiple Visualizer Modes**:
  - **Dynamic Spectrum Bars**: Colorful gradient bars with reactive heights and floating peak caps.
  - **Oscilloscope Waveform**: Smooth antialiased audio waveform.
  - **Circular / Radial Pulse**: Radial frequency burst with bass pulse and customizable center hub (Custom Cover Art / Album Image, Live Elapsed Time, Remaining Time Countdown, or Song Title).
- 🎬 **Deterministic 60 FPS Video Export**:
  - Offline frame stepping with zero dropped frames.
  - Automatic FFmpeg piping for direct `.mp4` video generation with muxed audio.

---

## 🚧 Web & Mobile Support (Under Development)

> [!NOTE]
> **Web (WebAssembly) and Mobile (Android / iOS)** versions are currently **under active development**. 
> 
> Due to browser audio autoplay limitations, threading sandbox constraints, and mobile-specific background audio lifecycles, full support for Web and Mobile is undergoing a complete architectural redesign. The currently maintained and fully stable platform is the **Desktop application** (Linux, macOS, Windows).

---

## 🐧 Linux Drag & Drop Note

> [!NOTE]
> File drag-and-drop works seamlessly on Windows, macOS, and standard Linux desktop environments (e.g., Linux Mint, Cinnamon, X11). 
> 
> On certain **customized Linux Wayland compositors** (such as KDE Plasma 6 / Dolphin on Arch / CachyOS), cross-window drag-and-drop data offers may occasionally fail due to upstream Wayland protocol limitations. 
> 
> If you encounter this:
> - Simply click the center card or the **"📂 Open Audio File"** button to load tracks via the native system file picker.
> - Or launch the app with the XWayland fallback flag: `cargo run --release -- --x11`.

---

## 🏛️ Documentation

- 📐 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): Detailed technical architecture, math equations, and dataflow diagrams.
- 📦 [docs/APP_EXPORTING_COMMANDS.md](docs/APP_EXPORTING_COMMANDS.md): Full command-line reference for exporting & packaging desktop apps.

---

## 🚀 Quick Start (Desktop)

### 1. Clone the Repository
```bash
git clone https://github.com/gabinroy/musializer-rs.git
cd musializer-rs
```

### 2. Run on Desktop (Linux, macOS, Windows)
```bash
cargo run --release
```

### 3. Build Release Binary
```bash
cargo build --release
```

---

## 🧪 Tests & Quality
- `cargo test`: 100% tests passing.
- `cargo check`: 0 errors, 0 warnings.

---

## 📄 License
This project is licensed under the **GNU General Public License v3.0** ([GPLv3](LICENSE)).
