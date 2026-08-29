use eframe::egui;
use egui::{Align, Align2, Color32, Layout, ProgressBar, RichText, Rounding, Vec2, Window};

use crate::updater::types::{ReleaseInfo, UpdateStatus, UpdaterCommand};

pub struct UpdateModal;

impl UpdateModal {
    pub fn show(
        ctx: &egui::Context,
        open: &mut bool,
        status: &UpdateStatus,
        cmd_tx: &crossbeam_channel::Sender<UpdaterCommand>,
    ) {
        if !*open {
            return;
        }

        let mut is_open = *open;
        let mut should_close = false;

        Window::new("🚀 Software Update")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .default_width(480.0)
            .open(&mut is_open)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 12.0;

                match status {
                    UpdateStatus::UpdateAvailable(info) => {
                        Self::render_update_available(ui, info, &mut should_close, cmd_tx);
                    }
                    UpdateStatus::Downloading {
                        progress,
                        status_text,
                    } => {
                        Self::render_downloading(ui, *progress, status_text);
                    }
                    UpdateStatus::ReadyToRestart => {
                        Self::render_ready_to_restart(ui);
                    }
                    UpdateStatus::Checking => {
                        Self::render_checking(ui);
                    }
                    UpdateStatus::UpToDate => {
                        Self::render_up_to_date(ui, &mut should_close);
                    }
                    UpdateStatus::Failed(err_msg) => {
                        Self::render_failed(ui, err_msg, &mut should_close, cmd_tx);
                    }
                    UpdateStatus::Idle => {
                        ui.label("No update check in progress.");
                    }
                }
            });

        *open = is_open && !should_close;
    }

    fn render_update_available(
        ui: &mut egui::Ui,
        info: &ReleaseInfo,
        should_close: &mut bool,
        cmd_tx: &crossbeam_channel::Sender<UpdaterCommand>,
    ) {
        // Version Comparison Pill Box
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("CURRENT VERSION")
                        .size(10.0)
                        .color(Color32::from_rgb(148, 163, 184)),
                );
                ui.label(
                    RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .strong()
                        .size(15.0)
                        .color(Color32::from_rgb(226, 232, 240)),
                );
            });

            ui.add_space(20.0);
            ui.label(
                RichText::new("➔")
                    .size(18.0)
                    .color(Color32::from_rgb(56, 189, 248)),
            );
            ui.add_space(20.0);

            ui.vertical(|ui| {
                ui.label(
                    RichText::new("NEW VERSION")
                        .size(10.0)
                        .color(Color32::from_rgb(56, 189, 248)),
                );
                ui.label(
                    RichText::new(&info.version)
                        .strong()
                        .size(15.0)
                        .color(Color32::from_rgb(56, 189, 248)),
                );
            });
        });

        ui.separator();

        // Release Title & Changelog Box
        ui.label(
            RichText::new(format!("Release: {}", info.title))
                .strong()
                .size(13.0)
                .color(Color32::from_rgb(241, 245, 249)),
        );

        egui::Frame::canvas(ui.style())
            .fill(Color32::from_rgb(15, 23, 42))
            .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(30, 41, 59)))
            .rounding(Rounding::same(6.0))
            .inner_margin(10.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(180.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.add(
                            egui::Label::new(
                                RichText::new(&info.changelog)
                                    .size(12.0)
                                    .color(Color32::from_rgb(203, 213, 225)),
                            )
                            .wrap(),
                        );
                    });
            });

        ui.separator();

        // Action Buttons
        ui.horizontal(|ui| {
            if ui.button("Remind Me Later").clicked() {
                *should_close = true;
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let update_btn = egui::Button::new(
                    RichText::new("⚡ Update Now")
                        .size(13.0)
                        .strong()
                        .color(Color32::from_rgb(15, 23, 42)),
                )
                .fill(Color32::from_rgb(56, 189, 248))
                .rounding(Rounding::same(6.0))
                .min_size(Vec2::new(120.0, 30.0));

                if ui.add(update_btn).clicked() {
                    let _ = cmd_tx.send(UpdaterCommand::ApplyUpdate {
                        target_version: info.version.clone(),
                    });
                }
            });
        });
    }

    fn render_downloading(ui: &mut egui::Ui, progress: f32, status_text: &str) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.add(egui::Spinner::new().size(26.0));
            ui.add_space(8.0);
            ui.label(
                RichText::new(status_text)
                    .size(13.0)
                    .color(Color32::from_rgb(226, 232, 240)),
            );
            ui.add_space(10.0);
            ui.add(
                ProgressBar::new(progress)
                    .desired_width(ui.available_width() - 32.0)
                    .animate(true),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new("Please do not close the application during update installation.")
                    .size(11.0)
                    .color(Color32::from_rgb(148, 163, 184)),
            );
        });
    }

    fn render_ready_to_restart(ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(6.0);
            ui.label(
                RichText::new("✅ Update Installed Successfully!")
                    .strong()
                    .size(15.0)
                    .color(Color32::from_rgb(74, 222, 128)),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new("Restart Musializer-RS to launch the updated version.")
                    .size(12.5)
                    .color(Color32::from_rgb(203, 213, 225)),
            );
            ui.add_space(14.0);

            let restart_btn = egui::Button::new(
                RichText::new("🔄 Restart Application")
                    .strong()
                    .size(13.0)
                    .color(Color32::from_rgb(15, 23, 42)),
            )
            .fill(Color32::from_rgb(74, 222, 128))
            .rounding(Rounding::same(6.0))
            .min_size(Vec2::new(170.0, 32.0));

            if ui.add(restart_btn).clicked() {
                if let Ok(current_exe) = std::env::current_exe() {
                    let _ = std::process::Command::new(current_exe).spawn();
                    std::process::exit(0);
                }
            }
        });
    }

    fn render_checking(ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.add(egui::Spinner::new().size(24.0));
            ui.add_space(8.0);
            ui.label("Checking for latest releases on GitHub...");
            ui.add_space(10.0);
        });
    }

    fn render_up_to_date(ui: &mut egui::Ui, should_close: &mut bool) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.label(
                RichText::new("✨ You're all up to date!")
                    .strong()
                    .size(14.0)
                    .color(Color32::from_rgb(74, 222, 128)),
            );
            ui.label(format!("Musializer-RS v{} is the latest version.", env!("CARGO_PKG_VERSION")));
            ui.add_space(12.0);

            if ui.button("Close").clicked() {
                *should_close = true;
            }
        });
    }

    fn render_failed(
        ui: &mut egui::Ui,
        error: &str,
        should_close: &mut bool,
        cmd_tx: &crossbeam_channel::Sender<UpdaterCommand>,
    ) {
        ui.vertical(|ui| {
            ui.label(
                RichText::new("❌ Update Check Failed")
                    .strong()
                    .size(14.0)
                    .color(Color32::from_rgb(248, 113, 113)),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new(error)
                    .size(12.0)
                    .color(Color32::from_rgb(252, 165, 165)),
            );

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if ui.button("Dismiss").clicked() {
                    *should_close = true;
                }
                if ui.button("🔄 Retry Check").clicked() {
                    let _ = cmd_tx.send(UpdaterCommand::CheckNow);
                }
            });
        });
    }
}
