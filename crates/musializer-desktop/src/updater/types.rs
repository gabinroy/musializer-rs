use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
    pub published_at: Option<String>,
    #[serde(default)]
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseInfo {
    pub version: String,
    pub title: String,
    pub changelog: String,
    pub html_url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate,
    UpdateAvailable(ReleaseInfo),
    Downloading { progress: f32, status_text: String },
    ReadyToRestart,
    Failed(String),
}

/// Commands sent from the UI thread to the background updater worker.
#[derive(Debug, Clone)]
pub enum UpdaterCommand {
    CheckNow,
    ApplyUpdate { target_version: String },
}

/// Events sent from the background worker to the UI thread.
#[derive(Debug, Clone)]
pub enum UpdaterEvent {
    StatusChanged(UpdateStatus),
    Error(String),
}
