use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use semver::Version;
use tokio::time::sleep;

use crate::updater::types::{GitHubRelease, ReleaseInfo, UpdateStatus, UpdaterCommand, UpdaterEvent};

pub const GITHUB_REPO_OWNER: &str = "gabinroy";
pub const GITHUB_REPO_NAME: &str = "musializer-rs";
pub const USER_AGENT: &str = concat!("musializer-rs-updater/", env!("CARGO_PKG_VERSION"));

pub struct UpdaterBackend {
    cmd_rx: Receiver<UpdaterCommand>,
    event_tx: Sender<UpdaterEvent>,
    poll_interval: Duration,
}

impl UpdaterBackend {
    /// Spawns the background updater service on a dedicated multi-threaded tokio runtime.
    pub fn spawn(
        cmd_rx: Receiver<UpdaterCommand>,
        event_tx: Sender<UpdaterEvent>,
        poll_interval: Duration,
    ) {
        let builder = std::thread::Builder::new().name("updater-service".into());
        let spawn_res = builder.spawn(move || {
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(err) => {
                    log::error!("Failed to create tokio runtime for updater: {err}");
                    let _ = event_tx.send(UpdaterEvent::Error(format!(
                        "Failed to initialize async updater runtime: {err}"
                    )));
                    return;
                }
            };

            rt.block_on(async move {
                let mut backend = UpdaterBackend {
                    cmd_rx,
                    event_tx,
                    poll_interval,
                };
                backend.run().await;
            });
        });

        if let Err(err) = spawn_res {
            log::error!("Failed to spawn updater backend thread: {err}");
        }
    }

    async fn run(&mut self) {
        log::info!("Updater background service started. Initial check scheduled in 5 seconds...");
        sleep(Duration::from_secs(5)).await;
        self.check_for_updates().await;

        let mut check_interval = tokio::time::interval(self.poll_interval);

        loop {
            tokio::select! {
                _ = check_interval.tick() => {
                    log::debug!("Scheduled periodic update check triggered.");
                    self.check_for_updates().await;
                }
                cmd = async { self.cmd_rx.try_recv().ok() } => {
                    if let Some(cmd) = cmd {
                        match cmd {
                            UpdaterCommand::CheckNow => {
                                log::info!("Manual update check requested from UI.");
                                self.check_for_updates().await;
                            }
                            UpdaterCommand::ApplyUpdate { target_version } => {
                                log::info!("Applying update to target version: {target_version}");
                                self.apply_update(&target_version).await;
                            }
                        }
                    } else {
                        sleep(Duration::from_millis(200)).await;
                    }
                }
            }
        }
    }

    async fn check_for_updates(&self) {
        let _ = self
            .event_tx
            .send(UpdaterEvent::StatusChanged(UpdateStatus::Checking));

        match self.fetch_latest_release().await {
            Ok(Some(release)) => {
                log::info!("New version available: {}", release.version);
                let _ = self
                    .event_tx
                    .send(UpdaterEvent::StatusChanged(UpdateStatus::UpdateAvailable(
                        release,
                    )));
            }
            Ok(None) => {
                log::debug!("Application is currently up-to-date.");
                let _ = self
                    .event_tx
                    .send(UpdaterEvent::StatusChanged(UpdateStatus::UpToDate));
            }
            Err(err) => {
                let err_msg = err.to_string();
                log::warn!("Update check failed: {err_msg}");
                let _ = self
                    .event_tx
                    .send(UpdaterEvent::StatusChanged(UpdateStatus::Failed(err_msg)));
            }
        }
    }

    async fn fetch_latest_release(
        &self,
    ) -> Result<Option<ReleaseInfo>, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(15))
            .build()?;

        let url = format!(
            "https://api.github.com/repos/{GITHUB_REPO_OWNER}/{GITHUB_REPO_NAME}/releases/latest"
        );

        let resp = client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(format!("GitHub API error: HTTP {}", resp.status()).into());
        }

        let release: GitHubRelease = resp.json().await?;
        let latest_tag = release.tag_name.trim_start_matches('v');
        let current_tag = env!("CARGO_PKG_VERSION");

        let current_semver = Version::parse(current_tag)
            .map_err(|e| format!("Failed to parse current version '{current_tag}': {e}"))?;
        let latest_semver = Version::parse(latest_tag)
            .map_err(|e| format!("Failed to parse release version '{latest_tag}': {e}"))?;

        if latest_semver > current_semver {
            Ok(Some(ReleaseInfo {
                version: release.tag_name.clone(),
                title: release.name.unwrap_or_else(|| release.tag_name.clone()),
                changelog: release
                    .body
                    .unwrap_or_else(|| "No changelog notes provided for this release.".to_string()),
                html_url: release.html_url,
            }))
        } else {
            Ok(None)
        }
    }

    async fn apply_update(&self, _target_version: &str) {
        let _ = self
            .event_tx
            .send(UpdaterEvent::StatusChanged(UpdateStatus::Downloading {
                progress: 0.25,
                status_text: "Downloading update package from GitHub...".into(),
            }));

        let tx = self.event_tx.clone();
        let update_result = tokio::task::spawn_blocking(
            move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                let status = self_update::backends::github::Update::configure()
                    .repo_owner(GITHUB_REPO_OWNER)
                    .repo_name(GITHUB_REPO_NAME)
                    .bin_name("musializer-rs")
                    .show_download_progress(false)
                    .current_version(env!("CARGO_PKG_VERSION"))
                    .build()?
                    .update()?;

                log::info!(
                    "Self update succeeded! Updated to version: {}",
                    status.version()
                );
                Ok(())
            },
        )
        .await;

        match update_result {
            Ok(Ok(_)) => {
                let _ = tx.send(UpdaterEvent::StatusChanged(UpdateStatus::ReadyToRestart));
            }
            Ok(Err(err)) => {
                let err_msg = format!("Update failed: {err}");
                log::error!("{err_msg}");
                let _ = tx.send(UpdaterEvent::StatusChanged(UpdateStatus::Failed(err_msg)));
            }
            Err(join_err) => {
                let err_msg = format!("Background worker error: {join_err}");
                log::error!("{err_msg}");
                let _ = tx.send(UpdaterEvent::StatusChanged(UpdateStatus::Failed(err_msg)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_comparison() {
        let v1 = Version::parse("0.1.0").unwrap();
        let v2 = Version::parse("0.2.0").unwrap();
        let v3 = Version::parse("0.1.1").unwrap();
        assert!(v2 > v1);
        assert!(v3 > v1);
        assert!(v2 > v3);
    }

    #[test]
    fn test_github_release_deserialization() {
        let raw_json = concat!(
            "{\n",
            "  \"tag_name\": \"v0.2.0\",\n",
            "  \"name\": \"Musializer-RS v0.2.0 Release\",\n",
            "  \"body\": \"## What Changed\\n* Added in-app updater\\n* Performance boosts\",\n",
            "  \"html_url\": \"https://github.com/gabinroy/musializer-rs/releases/tag/v0.2.0\",\n",
            "  \"assets\": []\n",
            "}"
        );

        let release: GitHubRelease = serde_json::from_str(raw_json).expect("Deserialization failed");
        assert_eq!(release.tag_name, "v0.2.0");
        assert_eq!(release.name.as_deref(), Some("Musializer-RS v0.2.0 Release"));
    }
}
