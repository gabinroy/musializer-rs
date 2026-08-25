# Musializer-RS 🎵

A high-performance, cross-platform audio visualizer written in Rust, inspired by [`tsoding/musializer`](https://github.com/tsoding/musializer).

Musializer-RS brings native GPU-accelerated rendering, real-time Fourier analysis (DSP), multi-format audio playback, and deterministic 60 FPS video export into a clean, modern desktop experience.

---

## 🌟 Key Features

- **Multi-Format Audio Support**: Decode and play MP3, WAV, FLAC, OGG/Vorbis, and AAC files via `symphonia`.
- **Hardware-Accelerated UI**: Built with `eframe` (egui + wgpu) for 60+ FPS rendering across Windows, macOS, Linux, and WebAssembly.
- **Fast Fourier Transform (DSP)**:
  - Hann windowing to prevent spectral leakage.
  - SIMD-accelerated FFT via `rustfft`.
  - Logarithmic / Octave frequency band grouping (Sub-bass to Treble).
  - Asymmetric Attack/Decay Exponential Moving Average (EMA) with peak hold dots for fluid animations.
- **Multiple Visualizer Modes**:
  - **Dynamic Spectrum Bars**: Colorful gradient bars with reactive heights and floating peak caps.
  - **Oscilloscope Waveform**: Smooth antialiased audio waveform.
  - **Circular / Radial Mode**: Radial frequency burst with bass pulse.
- **Deterministic 60 FPS Video Export**:
  - Offline frame stepping with zero dropped frames.
  - Automatic FFmpeg piping for direct `.mp4` video generation with muxed audio.
- **Modern User Experience**:
  - Drag-and-drop audio loading.
  - Scrubbable timeline with time elapsed / remaining.
  - Deep dark theme with soft glassmorphism accents.

---

## 🏛️ Architecture Overview

The system is decoupled into four core modules:

```
musializer-rs/
├── src/
│   ├── main.rs               # Window initialization and app entrypoint
│   ├── app.rs                # Main application state and egui update loop
│   ├── audio/                # Module 1: The Engine (Symphonia, CPAL, Sync)
│   ├── dsp/                  # Module 2: The Math (Hann, RustFFT, Log Bands, EMA)
│   ├── ui/                   # Module 3: The Visuals (Theme, Painters, Controls)
│   └── export/               # Module 4: The Exporter (Offline Stepper, FFmpeg Pipe)
└── docs/
    └── ARCHITECTURE.md       # Detailed technical architecture design document
```

For complete technical specifications, formulas, and data flow diagrams, see [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

---

## 🚀 Getting Started

### Prerequisites
- **Rust Toolchain**: Rust 1.80+ ([Install Rust](https://www.rust-lang.org/tools/install))
- **FFmpeg** *(Optional, required for video export)*: Ensure `ffmpeg` is installed and available on your system `PATH`.

### Build & Run
```bash
# Clone the repository
git clone https://github.com/your-username/musializer-rs.git
cd musializer-rs

# Run locally
cargo run --release
```

---

## 📄 License
MIT / Apache-2.0
