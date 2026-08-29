use crate::audio::AudioPlayer;
use crate::ui::theme::ColorTheme;
use crate::ui::visualizer::{CircleCenterDisplay, VisualizerMode};
use egui::{Align, Button, Color32, Frame, Layout, Margin, RichText, Rounding, Slider, Stroke, Ui};

pub struct TransportControls;

impl TransportControls {
    pub fn show(
        ui: &mut Ui,
        player: &mut AudioPlayer,
        current_mode: &mut VisualizerMode,
        current_theme: &mut ColorTheme,
        visual_gain: &mut f32,
        circle_center_display: &mut CircleCenterDisplay,
        on_load_center_image: &mut bool,
        on_open_file_click: &mut bool,
        on_export_click: &mut bool,
        is_exporting: bool,
        export_progress: f32,
    ) {
        let is_compact_width = ui.available_width() < 600.0;

        Frame::none()
            .fill(Color32::from_rgb(18, 22, 32))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(35, 42, 60)))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(if is_compact_width { 12.0 } else { 10.0 }))
            .show(ui, |ui| {
                // Top metadata and action row
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let export_btn_text = if is_exporting {
                            format!("⏳ Exporting ({:.0}%)", export_progress * 100.0)
                        } else {
                            "🎬 Export Video".to_string()
                        };

                        if ui
                            .add_enabled(
                                player.track().is_some() && !is_exporting,
                                Button::new(
                                    RichText::new(export_btn_text)
                                        .color(Color32::from_rgb(0, 240, 255))
                                        .size(if is_compact_width { 13.0 } else { 12.0 }),
                                ),
                            )
                            .clicked()
                        {
                            *on_export_click = true;
                        }

                        // Open Audio button with prominent styling
                        let open_btn = Button::new(
                            RichText::new("📂 Open Audio...")
                                .color(Color32::from_rgb(0, 240, 255))
                                .size(if is_compact_width { 13.0 } else { 12.0 })
                                .strong(),
                        )
                        .fill(Color32::from_rgb(22, 28, 44))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 65, 100)))
                        .rounding(Rounding::same(6.0_f32));

                        if ui.add(open_btn).clicked() {
                            *on_open_file_click = true;
                        }

                        // Left track info fills available space
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            if let Some(track) = player.track() {
                                ui.label(
                                    RichText::new(format!("🎵 {}", track.title))
                                        .strong()
                                        .size(13.0)
                                        .color(Color32::from_rgb(0, 240, 255)),
                                );

                                if !is_compact_width {
                                    ui.label(
                                        RichText::new(format!("({} Hz Stereo)", track.sample_rate))
                                            .size(11.0)
                                            .color(Color32::from_rgb(130, 140, 160)),
                                    );
                                }
                            } else {
                                ui.label(
                                    RichText::new("No Track Loaded — Open an audio file to start")
                                        .italics()
                                        .size(13.0)
                                        .color(Color32::from_rgb(140, 150, 170)),
                                );
                            }
                        });
                    });
                });

                ui.add_space(4.0);

                // Top Scrubber & Volume Row: Elapsed Time | Timeline Slider | Total Time | Separator | Volume Slider
                let current_time = player.current_time_seconds();
                let total_duration = player.duration_seconds().max(0.01);
                let mut seek_time = current_time;

                ui.horizontal(|ui| {
                    // Right controls (Volume + Total Duration)
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // Volume Slider & Label
                        let mut vol = player.get_volume();
                        let vol_slider = ui.add_sized(
                            [70.0, 16.0],
                            Slider::new(&mut vol, 0.0..=1.5).show_value(false),
                        );
                        if vol_slider.changed() {
                            player.set_volume(vol);
                        }
                        ui.label(
                            RichText::new(format!("🔊 {:.0}%", vol * 100.0))
                                .size(11.0)
                                .color(Color32::from_rgb(170, 185, 210)),
                        );

                        ui.separator();

                        // Total Duration Label
                        ui.label(
                            RichText::new(format_time(total_duration))
                                .color(Color32::from_rgb(140, 150, 170))
                                .monospace()
                                .size(12.0),
                        );

                        // In the remaining space on left, show elapsed time & stretch slider
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.label(
                                RichText::new(format_time(current_time))
                                    .color(Color32::from_rgb(180, 190, 210))
                                    .monospace()
                                    .size(12.0),
                            );

                            let available_slider_width = ui.available_width().max(40.0);
                            let slider_resp = ui.add_sized(
                                [available_slider_width, 18.0],
                                Slider::new(&mut seek_time, 0.0..=total_duration)
                                    .show_value(false)
                                    .trailing_fill(true),
                            );

                            if slider_resp.changed() {
                                player.seek_seconds(seek_time);
                            }
                        });
                    });
                });

                ui.add_space(6.0);

                // Bottom Controls Row: Full Width for Playback, Mode, Theme, Center, Gain
                ui.horizontal(|ui| {
                    let is_playing = player.is_playing();
                    let play_btn_text = if is_playing { "⏸ Pause" } else { "▶ Play" };
                    let play_btn_color = if is_playing {
                        Color32::from_rgb(255, 120, 120)
                    } else {
                        Color32::from_rgb(100, 230, 140)
                    };

                    // Play button
                    if ui
                        .add_sized(
                            [68.0, 28.0],
                            Button::new(
                                RichText::new(play_btn_text)
                                    .color(play_btn_color)
                                    .size(12.0)
                                    .strong(),
                            ),
                        )
                        .clicked()
                    {
                        player.toggle_play_pause();
                    }

                    // Stop button
                    if ui
                        .add_sized(
                            [50.0, 28.0],
                            Button::new(RichText::new("⏹ Stop").size(12.0)),
                        )
                        .clicked()
                    {
                        player.stop();
                    }

                    ui.separator();

                    // Mode Dropdown
                    ui.label(RichText::new("Mode:").size(12.0).color(Color32::from_rgb(160, 175, 200)));
                    egui::ComboBox::from_id_source("viz_mode_combo")
                        .selected_text(RichText::new(current_mode.name()).size(12.0))
                        .width(95.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                current_mode,
                                VisualizerMode::SpectrumBars,
                                VisualizerMode::SpectrumBars.name(),
                            );
                            ui.selectable_value(
                                current_mode,
                                VisualizerMode::Waveform,
                                VisualizerMode::Waveform.name(),
                            );
                            ui.selectable_value(
                                current_mode,
                                VisualizerMode::Circular,
                                VisualizerMode::Circular.name(),
                            );
                        });

                    // Theme Dropdown
                    ui.label(RichText::new("Theme:").size(12.0).color(Color32::from_rgb(160, 175, 200)));
                    egui::ComboBox::from_id_source("color_theme_combo")
                        .selected_text(RichText::new(current_theme.name()).size(12.0))
                        .width(90.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                current_theme,
                                ColorTheme::CyberNeon,
                                ColorTheme::CyberNeon.name(),
                            );
                            ui.selectable_value(
                                current_theme,
                                ColorTheme::SunsetGlow,
                                ColorTheme::SunsetGlow.name(),
                            );
                            ui.selectable_value(
                                current_theme,
                                ColorTheme::EmeraldDeep,
                                ColorTheme::EmeraldDeep.name(),
                            );
                            ui.selectable_value(
                                current_theme,
                                ColorTheme::Monochrome,
                                ColorTheme::Monochrome.name(),
                            );
                        });

                    // Center Dropdown (when in Circular mode)
                    if *current_mode == VisualizerMode::Circular {
                        ui.separator();
                        ui.label(RichText::new("Center:").size(12.0).color(Color32::from_rgb(160, 175, 200)));
                        egui::ComboBox::from_id_source("circle_center_combo")
                            .selected_text(RichText::new(circle_center_display.name()).size(12.0))
                            .width(95.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    circle_center_display,
                                    CircleCenterDisplay::None,
                                    CircleCenterDisplay::None.name(),
                                );
                                ui.selectable_value(
                                    circle_center_display,
                                    CircleCenterDisplay::TimeElapsed,
                                    CircleCenterDisplay::TimeElapsed.name(),
                                );
                                ui.selectable_value(
                                    circle_center_display,
                                    CircleCenterDisplay::TimeRemaining,
                                    CircleCenterDisplay::TimeRemaining.name(),
                                );
                                ui.selectable_value(
                                    circle_center_display,
                                    CircleCenterDisplay::TrackTitle,
                                    CircleCenterDisplay::TrackTitle.name(),
                                );
                                ui.selectable_value(
                                    circle_center_display,
                                    CircleCenterDisplay::CustomCoverArt,
                                    CircleCenterDisplay::CustomCoverArt.name(),
                                );
                            });

                        if *circle_center_display == CircleCenterDisplay::CustomCoverArt {
                            if ui.button(RichText::new("🖼 Image...").size(11.0)).clicked() {
                                *on_load_center_image = true;
                            }
                        }
                    }

                    ui.separator();

                    // Sensitivity Boost Slider
                    ui.label(RichText::new("⚡ Gain:").size(12.0).color(Color32::from_rgb(160, 175, 200)));
                    ui.add_sized(
                        [60.0, 18.0],
                        Slider::new(visual_gain, 0.5..=3.5).show_value(false),
                    );
                    ui.label(RichText::new(format!("{:.1}x", *visual_gain)).size(11.0).monospace());
                });
            });
    }
}

fn format_time(seconds: f32) -> String {
    let s = seconds.max(0.0) as u32;
    let mins = s / 60;
    let secs = s % 60;
    format!("{:02}:{:02}", mins, secs)
}
