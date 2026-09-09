//! # Desktop UI Components & Styling
//!
//! Provides the UI widgets and theming for the egui desktop interface:
//! - [`controls`]: Play, pause, seek bar, volume slider, and track title display.
//! - [`visualizer`]: Spectrum bars, radial visualizer, and raw oscilloscopic waveform canvas.
//! - [`title_bar`]: Custom borderless window title bar with drag window handle and window controls.
//! - [`drag_drop`]: Visual drop target overlay for audio and album art files.
//! - [`theme`]: Dark mode color palettes, glassmorphic styling, and neon gradient definitions.
//! - [`update_modal`]: In-app dialog for downloading and applying GitHub releases.

pub mod controls;
pub mod drag_drop;
pub mod theme;
pub mod title_bar;
pub mod update_modal;
pub mod visualizer;

pub use controls::TransportControls;
pub use drag_drop::{DragDropOverlay, DroppedItem};
pub use theme::{ColorTheme, apply_theme};
pub use title_bar::{CustomTitleBar, TITLE_BAR_HEIGHT};
pub use update_modal::UpdateModal;
pub use visualizer::{CircleCenterDisplay, VisualizerMode, VisualizerWidget};
