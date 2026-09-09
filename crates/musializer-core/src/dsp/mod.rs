//! # Digital Signal Processing (DSP) Pipeline
//!
//! Provides the mathematical algorithms and filters that transform raw audio PCM chunks
//! into responsive, visually pleasing frequency animations:
//! - [`window`]: Hann window function generating edge-tapered sample windows.
//! - [`fft`]: SIMD-accelerated forward Fast Fourier Transform computing complex frequency spectrums.
//! - [`frequency`]: Logarithmic frequency partitioning (20 Hz - 20 kHz) with treble compensation.
//! - [`ema`]: Asymmetric exponential moving average (EMA) smoother and peak-hold caps.

pub mod ema;
pub mod fft;
pub mod frequency;
pub mod window;

pub use ema::EmaSmoother;
pub use fft::FftProcessor;
pub use frequency::FrequencyBands;
pub use window::HannWindow;
