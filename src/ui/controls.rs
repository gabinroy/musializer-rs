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
            .inner_margin(Margin::same(if is_compact_width {
                12.0
            } else {
                10.0
            }))
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

                // Timeline Scrubber
                let current_time = player.current_time_seconds();
                let total_duration = player.duration_seconds().max(0.01);
                let mut seek_time = current_time;

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format_time(current_time))
                            .color(Color32::from_rgb(180, 190, 210))
                            .monospace()
                            .size(12.0),
                    );

                    let available = ui.available_width();
                    let slider_width = (available - 55.0).max(60.0);
                    let slider_resp = ui.add_sized(
                        [
                            slider_width,
                            if is_compact_width { 28.0 } else { 20.0 },
                        ],
                        Slider::new(&mut seek_time, 0.0..=total_duration)
                            .show_value(false)
                            .trailing_fill(true),
                    );

                    if slider_resp.changed() {
                        player.seek_seconds(seek_time);
                    }

                    ui.label(
                        RichText::new(format_time(total_duration))
                            .color(Color32::from_rgb(140, 150, 170))
                            .monospace()
                            .size(12.0),
                    );
                });

                ui.add_space(4.0);

                // Responsive Controls Layout (Wrapped for compact windows)
                if is_compact_width {
                    // Compact Wrapped Layout
                    ui.horizontal_wrapped(|ui| {
                        let is_playing = player.is_playing();
                        let play_btn_text = if is_playing { "⏸ Pause" } else { "▶ Play" };
                        let play_btn_color = if is_playing {
                            Color32::from_rgb(255, 120, 120)
                        } else {
                            Color32::from_rgb(100, 230, 140)
                        };

                        // Large 44px+ touch targets
                        if ui
                            .add_sized(
                                [90.0, 44.0],
                                Button::new(
                                    RichText::new(play_btn_text)
                                        .color(play_btn_color)
                                        .size(16.0)
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            player.toggle_play_pause();
                        }

                        if ui.add_sized([70.0, 44.0], Button::new("⏹ Stop")).clicked() {
                            player.stop();
                        }

                        egui::ComboBox::from_id_source("viz_mode_combo_mobile")
                            .selected_text(current_mode.name())
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

                        egui::ComboBox::from_id_source("color_theme_combo_mobile")
                            .selected_text(current_theme.name())
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
                    });

                    // Extra row for sensitivity boost & circular options on mobile
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("⚡ Boost:").size(12.0));
                        ui.add_sized(
                            [90.0, 24.0],
                            Slider::new(visual_gain, 0.5..=3.5).show_value(true),
                        );

                        if *current_mode == VisualizerMode::Circular {
                            ui.separator();
                            ui.label(RichText::new("Center:").size(12.0));
                            egui::ComboBox::from_id_source("circle_center_mobile")
                                .selected_text(circle_center_display.name())
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
                                if ui.button("🖼 Image...").clicked() {
                                    *on_load_center_image = true;
                                }
                            }
                        }
                    });
                } else {
                    // Desktop Landscape Layout
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let mut vol = player.get_volume();
                            let vol_slider = ui.add_sized(
                                [75.0, 18.0],
                                Slider::new(&mut vol, 0.0..=1.5).show_value(false),
                            );
                            if vol_slider.changed() {
                                player.set_volume(vol);
                            }
                            ui.label(RichText::new(format!("🔊 {:.0}%", vol * 100.0)).size(12.0));
                            ui.separator();

                            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                let is_playing = player.is_playing();
                                let play_btn_text =
                                    if is_playing { "⏸ Pause" } else { "▶ Play" };
                                let play_btn_color = if is_playing {
                                    Color32::from_rgb(255, 120, 120)
                                } else {
                                    Color32::from_rgb(100, 230, 140)
                                };

                                if ui
                                    .add_sized(
                                        [80.0, 32.0],
                                        Button::new(
                                            RichText::new(play_btn_text)
                                                .color(play_btn_color)
                                                .strong(),
                                        ),
                                    )
                                    .clicked()
                                {
                                    player.toggle_play_pause();
                                }

                                if ui.add_sized([60.0, 32.0], Button::new("⏹ Stop")).clicked() {
                                    player.stop();
                                }

                                ui.separator();

                                ui.label("Mode:");
                                egui::ComboBox::from_id_source("viz_mode_combo")
                                    .selected_text(current_mode.name())
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

                                ui.label("Theme:");
                                egui::ComboBox::from_id_source("color_theme_combo")
                                    .selected_text(current_theme.name())
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

                                ui.separator();

                                // Visual gain / sensitivity slider
                                ui.label(RichText::new("⚡ Boost:").size(12.0));
                                ui.add_sized(
                                    [65.0, 18.0],
                                    Slider::new(visual_gain, 0.5..=3.5).show_value(true),
                                );

                                // Center Circle Customization
                                if *current_mode == VisualizerMode::Circular {
                                    ui.separator();
                                    ui.label("Center:");
                                    egui::ComboBox::from_id_source("circle_center_desktop")
                                        .selected_text(circle_center_display.name())
                                        .width(115.0)
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

                                    if *circle_center_display == CircleCenterDisplay::CustomCoverArt
                                    {
                                        if ui.button("🖼 Image...").clicked() {
                                            *on_load_center_image = true;
                                        }
                                    }
                                }
                            });
                        });
                    });
                }
            });
    }
}

fn format_time(seconds: f32) -> String {
    let s = seconds.max(0.0) as u32;
    let mins = s / 60;
    let secs = s % 60;
    format!("{:02}:{:02}", mins, secs)
}
