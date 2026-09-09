//! # Musializer Desktop
//!
//! `musializer-desktop` provides the native GUI desktop client for Musializer-RS, built
//! on `egui` and `eframe`. It delivers hardware-accelerated rendering (OpenGL/WGPU),
//! interactive audio playback controls, draggable file uploads, offline FFmpeg video
//! exporting, and an integrated GitHub release auto-updater.
//!
//! ## Modules
//!
//! - [`app`]: The core [`MusializerApp`] state managing egui frames, UI interactions, and audio coordination.
//! - [`ui`]: Egui components: spectrum visualizer canvas, player controls, custom window titlebar, and update modals.
//! - [`export`]: Offline video export engine with an FFmpeg child process pipe.
//! - [`updater`]: In-app GitHub release checker and executable self-updater.

pub use musializer_core::audio;
pub use musializer_core::dsp;

pub mod app;
pub mod export;
pub mod ui;
pub mod updater;

pub use app::MusializerApp;
