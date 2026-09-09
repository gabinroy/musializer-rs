//! # Audio Decoding, Playback, and Synchronization
//!
//! This module provides all audio I/O mechanisms:
//! - [`decoder`]: Multi-codec audio decoding into 32-bit floating point PCM via Symphonia.
//! - [`player`]: Hardware audio streaming via CPAL with thread-safe volume and transport controls.
//! - [`sync`]: Lock-free atomic playhead synchronization coordinating playback position with the visualizer.

pub mod decoder;
pub mod player;
pub mod sync;

pub use decoder::AudioTrack;
pub use player::AudioPlayer;
pub use sync::AudioSync;
