use crate::audio::AudioPlayer;
use crate::ui::theme::ColorTheme;
use crate::ui::visualizer::VisualizerMode;
use egui::{Align, Button, Color32, Layout, RichText, Slider, Ui};

pub struct ControlPanel;

impl ControlPanel {
    pub fn render(
        ui: &mut Ui,
        player: &mut AudioPlayer,
        current_mode: &mut VisualizerMode,
        current_theme: &mut ColorTheme,
        on_export_click: &mut bool,
        on_open_file_click: &mut bool,
    ) {
        let is_mobile_or_portrait = ui.available_width() < 600.0;

        ui.vertical(|ui| {
            // Track Title / Info Banner
            ui.horizontal(|ui| {
                if let Some(track) = player.track() {
                    ui.heading(
                        RichText::new(&track.title)
                            .color(Color32::from_rgb(240, 240, 255))
                            .strong()
                            .size(if is_mobile_or_portrait { 16.0 } else { 20.0 }),
                    );
                    ui.label(
                        RichText::new(format!("({} Hz)", track.sample_rate))
                            .color(Color32::from_rgb(120, 130, 150))
                            .size(11.0),
                    );
                } else {
                    ui.heading(
                        RichText::new("No Audio Loaded")
                            .color(Color32::from_rgb(150, 160, 180))
                            .size(if is_mobile_or_portrait { 16.0 } else { 20.0 }),
                    );
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    #[cfg(not(any(
                        target_os = "android",
                        target_os = "ios",
                        target_arch = "wasm32"
                    )))]
                    {
                        if ui
                            .add_sized(
                                [110.0, 36.0],
                                Button::new(
                                    RichText::new("🎬 Export Video")
                                        .color(Color32::from_rgb(255, 200, 80))
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            *on_export_click = true;
                        }
                    }

                    #[cfg(any(target_os = "android", target_os = "ios", target_arch = "wasm32"))]
                    {
                        if ui
                            .add_sized(
                                [110.0, 36.0],
                                Button::new(
                                    RichText::new("🎥 Record Info")
                                        .color(Color32::from_rgb(255, 200, 80))
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            *on_export_click = true;
                        }
                    }

                    let open_btn_size = if is_mobile_or_portrait {
                        [90.0, 36.0]
                    } else {
                        [100.0, 36.0]
                    };
                    if ui
                        .add_sized(
                            open_btn_size,
                            Button::new(
                                RichText::new("📂 Load Audio")
                                    .color(Color32::from_rgb(100, 200, 255))
                                    .strong(),
                            ),
                        )
                        .clicked()
                    {
                        *on_open_file_click = true;
                    }
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
                        .monospace(),
                );

                let slider_width = (ui.available_width() - 60.0).max(100.0);
                let slider_resp = ui.add_sized(
                    [
                        slider_width,
                        if is_mobile_or_portrait { 28.0 } else { 20.0 },
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
                        .monospace(),
                );
            });

            ui.add_space(4.0);

            // Responsive Controls Layout (Wrapped for mobile touch screens)
            if is_mobile_or_portrait {
                // Mobile-first Stacking Layout
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
            } else {
                // Desktop Landscape Layout
                ui.horizontal(|ui| {
                    let is_playing = player.is_playing();
                    let play_btn_text = if is_playing { "⏸ Pause" } else { "▶ Play" };
                    let play_btn_color = if is_playing {
                        Color32::from_rgb(255, 120, 120)
                    } else {
                        Color32::from_rgb(100, 230, 140)
                    };

                    if ui
                        .add_sized(
                            [90.0, 36.0],
                            Button::new(
                                RichText::new(play_btn_text).color(play_btn_color).strong(),
                            ),
                        )
                        .clicked()
                    {
                        player.toggle_play_pause();
                    }

                    if ui.button("⏹ Stop").clicked() {
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

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let mut vol = player.get_volume();
                        let vol_slider = ui.add_sized(
                            [100.0, 20.0],
                            Slider::new(&mut vol, 0.0..=1.5).show_value(false),
                        );
                        if vol_slider.changed() {
                            player.set_volume(vol);
                        }
                        ui.label(RichText::new(format!("🔊 {:.0}%", vol * 100.0)).size(12.0));
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
