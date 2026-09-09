//! # Offline Video Export Subsystem
//!
//! Renders audio visualizations frame-by-frame offline and pipes raw RGB frames into an
//! FFmpeg child process, producing a deterministic, high-definition MP4 video file.
//!
//! - [`ffmpeg`]: FFmpeg child process management, pipe streaming, and encoding configuration.
//! - [`renderer`]: Headless canvas frame rendering of spectrum bars, circular visualizers, and waveforms.
//! - [`stepper`]: Offline deterministic audio-visual sync stepper.

pub mod ffmpeg;
pub mod renderer;
pub mod stepper;

#[allow(unused_imports)]
pub use ffmpeg::{ExportConfig, VideoExporter};
