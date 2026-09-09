# Musializer-Core API Reference & Documentation Guide

This document provides a comprehensive technical reference for `musializer-core`, the shared audio decoding and digital signal processing (DSP) engine behind Musializer-RS.

---

## 1. Generating & Viewing Rustdoc Locally

You can generate and view the full interactive HTML documentation using standard Cargo tools:

```bash
# Generate documentation without third-party crate dependencies
cargo doc --workspace --no-deps

# Generate and open in your default web browser immediately
cargo doc --package musializer-core --no-deps --open
```

The output will be generated at:
- **musializer-core**: `target/doc/musializer_core/index.html`
- **musializer-desktop**: `target/doc/musializer_desktop/index.html`

---

## 2. Core Modules Architecture

```text
[Audio Input] ────► [Symphonia Decoder] ────► [Interleaved Stereo f32 PCM]
                                                        │
                      ┌─────────────────────────────────┴─────────────────────────────────┐
                      ▼                                                                   ▼
             [CPAL Audio Sink]                                                   [DSP Pipeline]
       (Hardware Audio Output Callback)                                          - Hann Windowing (2048 samples)
                      │                                                          - Forward FFT (RustFFT)
                      ▼                                                          - Log Frequency Bands (20Hz-20kHz)
           [Atomic Head Sync] ◄──────── (Lock-free Frame Pos) ─────────────────► - Asymmetric EMA & Peak Hold
```

### Module Breakdown

| Module | Description | Primary Types |
| :--- | :--- | :--- |
| [`engine`](../crates/musializer-core/src/engine.rs) | High-level orchestrator coordinating playback, FFT analysis, and band smoothing. | `AudioVisualizerEngine`, `AudioMetadata` |
| [`audio::decoder`](../crates/musializer-core/src/audio/decoder.rs) | Symphonia multi-codec audio decoder supporting MP3, WAV, FLAC, OGG, and AAC. | `AudioTrack` |
| [`audio::player`](../crates/musializer-core/src/audio/player.rs) | Real-time CPAL audio playback stream with hardware device matching and volume control. | `AudioPlayer` |
| [`audio::sync`](../crates/musializer-core/src/audio/sync.rs) | Atomic lock-free pointer synchronization coordinating audio playback with visualization. | `AudioSync` |
| [`dsp::fft`](../crates/musializer-core/src/dsp/fft.rs) | SIMD-accelerated forward FFT computing positive frequency magnitude spectrums. | `FftProcessor` |
| [`dsp::frequency`](../crates/musializer-core/src/dsp/frequency.rs) | Maps linear FFT bins into logarithmic bands with treble compensation and dynamic range compression. | `FrequencyBands` |
| [`dsp::ema`](../crates/musializer-core/src/dsp/ema.rs) | Asymmetric Exponential Moving Average smoother with peak-hold timers. | `EmaSmoother` |
| [`dsp::window`](../crates/musializer-core/src/dsp/window.rs) | Precomputed Hann window weights mitigating spectral leakage artifacts. | `HannWindow` |
| [`api`](../crates/musializer-core/src/api.rs) | C-ABI and Flutter Rust Bridge (FRB) zero-copy FFI interface. | `init_engine`, `get_spectrum`, `MobileTrackInfo` |

---

## 3. Quick Start Code Examples

### 3.1 Basic Usage (Pure Rust)

```rust
use musializer_core::engine::AudioVisualizerEngine;

fn main() -> Result<(), String> {
    // 1. Initialize engine with 2048 FFT sample window and 64 visual frequency bands
    let mut engine = AudioVisualizerEngine::new(2048, 64)?;

    // 2. Load an audio file (MP3, WAV, FLAC, OGG, AAC)
    let meta = engine.load_audio_file("music/track.mp3")?;
    println!("Loaded '{}': {:.2}s, {}Hz", meta.title, meta.duration_seconds, meta.sample_rate);

    // 3. Start hardware audio playback
    engine.play();

    // 4. Query smoothed frequency spectrum in your 60/120 FPS render loop
    let delta_time = 1.0 / 60.0;
    let spectrum_bars = engine.update_and_get_spectrum(delta_time);

    println!("Current frequency band values: {:?}", spectrum_bars);
    Ok(())
}
```

### 3.2 Offline Video Spectrum Generation

For video export without audio clock jitter or real-time frame drops:

```rust
use musializer_core::api::{get_offline_audio_pcm, get_offline_spectrum_frames};

// Returns total_frames * num_bands flattened float magnitudes
let spectrum_frames = get_offline_spectrum_frames(60, 64, 1.0)?;

// Returns raw 16-bit stereo PCM bytes for muxing into MP4/AAC
let raw_pcm = get_offline_audio_pcm()?;
```

---

## 4. Signal Processing & Mathematics

### 4.1 Hann Window Function
To eliminate spectral boundary leakage:
$$w[n] = 0.5 \cdot \left(1 - \cos\left(\frac{2\pi n}{N - 1}\right)\right)$$

### 4.2 Logarithmic Frequency Partitioning
Maps human hearing range ($20\text{ Hz} \le f \le 20\text{ kHz}$):
$$f_{\text{low}} = f_{\min} \cdot \left(\frac{f_{\max}}{f_{\min}}\right)^{\frac{i}{N_{\text{bands}}}}$$

### 4.3 Treble Compensation Curve
Compensates for the natural $1/f$ pink noise distribution in acoustic music:
$$\text{TrebleBoost}(i) = 1.0 + 1.8 \cdot \left(\frac{i}{N_{\text{bands}}}\right)^{1.4}$$

### 4.4 Power-Law Gamma Compression
Protects high transients from clipping while lifting quiet sub-harmonics:
$$\text{Mag}_{\text{visual}} = \left(\text{Mag}_{\text{raw}} \cdot 4.0 \cdot \text{Gain} \cdot \text{TrebleBoost}\right)^{0.55}$$