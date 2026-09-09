//! # GitHub Release Auto-Updater
//!
//! Checks GitHub Releases for new binary releases, streams progress during download,
//! verifies platform assets, and performs atomic binary replacement on Linux, macOS, and Windows.
//!
//! - [`service`]: Background worker thread polling GitHub API and downloading assets.
//! - [`types`]: Cross-thread communication events, release metadata, and status enums.

pub mod service;
pub mod types;

pub use service::UpdaterBackend;
pub use types::{ReleaseInfo, UpdateStatus, UpdaterCommand, UpdaterEvent};
