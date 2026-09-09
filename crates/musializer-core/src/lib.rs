//! # Musializer Core
//!
//! `musializer-core` is the shared, high-performance audio decoding and digital signal
//! processing (DSP) engine behind Musializer-RS. It powers real-time audio playback,
//! hardware-accelerated Fast Fourier Transforms (FFT), logarithmic frequency band
//! aggregation, and visual bar smoothing across desktop and mobile platforms.
//!
//! ## Architecture Overview
//!
//! ```text
//! [Audio File / Buffer]
//!         │
//!         ▼
//! ┌────────────────────────────────────────────────────────┐
//! │  Symphonia Multi-Codec Decoder (MP3, WAV, FLAC, AAC)   │
//! └────────────────────────────────────────────────────────┘
//!         │
//!         ▼
//! ┌────────────────────────────────────────────────────────┐
//! │  Stereo f32 PCM Buffer & Atomic Playhead Head Position │
//! └────────────────────────────────────────────────────────┘
//!    │                                           │
//!    ▼                                           ▼
//! ┌──────────────────────┐           ┌────────────────────────────────┐
//! │ cpal Audio Sink      │           │ DSP Windowing & FFT Transform  │
//! │ (AAudio/CoreAudio/   │           │ (Hann Window + RustFFT SIMD)   │
//! │  ALSA/WASAPI Output) │           └────────────────────────────────┘
//! └──────────────────────┘                       │
//!                                                ▼
//!                                    ┌────────────────────────────────┐
//!                                    │ Logarithmic Frequency Bands    │
//!                                    │ (20 Hz - 20 kHz Human Range)   │
//!                                    └────────────────────────────────┘
//!                                                │
//!                                                ▼
//!                                    ┌────────────────────────────────┐
//!                                    │ Asymmetric EMA & Peak-Hold     │
//!                                    │ (Transient Attack, Soft Decay) │
//!                                    └────────────────────────────────┘
//! ```
//!
//! ## Core Modules
//!
//! - [`engine`]: The high-level [`AudioVisualizerEngine`] coordinating audio decoding,
//!   hardware streaming, and real-time frequency magnitude extraction.
//! - [`audio`]: Audio decoding via Symphonia, multi-platform streaming via CPAL, and
//!   lock-free atomic playhead synchronization.
//! - [`dsp`]: Hann windowing, forward FFT transform via RustFFT, logarithmic frequency
//!   binning, and exponential moving average (EMA) peak smoothing.
//! - [`api`]: Zero-copy C-ABI / FFI interface tailored for `flutter_rust_bridge` integration.
//!
//! ## Quick Start Example
//!
//! ```no_run
//! use musializer_core::engine::AudioVisualizerEngine;
//!
//! fn main() -> Result<(), String> {
//!     // Initialize engine with a 2048-sample FFT window and 64 visual frequency bands
//!     let mut engine = AudioVisualizerEngine::new(2048, 64)?;
//!
//!     // Load any supported audio track (MP3, WAV, FLAC, OGG, AAC)
//!     let meta = engine.load_audio_file("path/to/song.mp3")?;
//!     println!("Loaded: {} ({} seconds)", meta.title, meta.duration_seconds);
//!
//!     // Begin hardware playback
//!     engine.play();
//!
//!     // Retrieve visualizer bars in your render loop (e.g. at 60 or 120 FPS)
//!     let bars = engine.update_and_get_spectrum(1.0 / 60.0);
//!     println!("Current frequency magnitudes: {:?}", bars);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod audio;
pub mod dsp;
pub mod engine;
mod frb_generated;

pub use audio::decoder::AudioTrack;
pub use audio::player::AudioPlayer;
pub use audio::sync::AudioSync;
pub use dsp::ema::EmaSmoother;
pub use dsp::fft::FftProcessor;
pub use dsp::frequency::FrequencyBands;
pub use engine::{AudioMetadata, AudioVisualizerEngine};
