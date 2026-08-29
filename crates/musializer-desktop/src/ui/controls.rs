use crate::audio::AudioPlayer;
use crate::ui::theme::ColorTheme;
use crate::ui::visualizer::{CircleCenterDisplay, VisualizerMode};
use egui::{
    Align, Button, Color32, FontId, Frame, Layout, Margin, Pos2, RichText, Rounding,
    Sense, Slider, Stroke, Ui, Vec2, Widget,
};
use std::ops::RangeInclusive;

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

                            // The slider's painted width is driven by `spacing.slider_width`
                            // (default 100px). `add_sized` only grew the hover region, leaving a
                            // short bar with a large gap. Setting `slider_width` to the available
                            // space makes it stretch to fill the middle (egui's flex-grow: 1).
                            let available_slider_width = ui.available_width().max(40.0);
                            ui.spacing_mut().slider_width = available_slider_width;
                            let slider_resp = ui.add(
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
                // `horizontal_wrapped` (Align::Center cross axis) pushes overflowing controls
                // to a new line instead of clipping them off the window edge when width is tight.
                //
                // Each logical block is wrapped in its own `ui::horizontal` sub-row so it
                // becomes an indivisible unit that wraps together, and every element within
                // a group is vertically centered against its peers.
                ui.horizontal_wrapped(|ui| {
                    let is_playing = player.is_playing();
                    let combo_width = if is_compact_width { 78.0 } else { 95.0 };
                    let theme_combo_width = if is_compact_width { 72.0 } else { 90.0 };
                    let play_btn_text = if is_playing { "⏸ Pause" } else { "▶ Play" };
                    let play_btn_color = if is_playing {
                        Color32::from_rgb(255, 120, 120)
                    } else {
                        Color32::from_rgb(100, 230, 140)
                    };

                    // Playback controls group
                    ui.horizontal(|ui| {
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

                        if ui
                            .add_sized(
                                [50.0, 28.0],
                                Button::new(RichText::new("⏹ Stop").size(12.0)),
                            )
                            .clicked()
                        {
                            player.stop();
                        }
                    });

                    ui.separator();

                    // Mode Dropdown group (Label + ComboBox)
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Mode:").size(12.0).color(Color32::from_rgb(160, 175, 200)));
                        egui::ComboBox::from_id_source("viz_mode_combo")
                            .selected_text(RichText::new(current_mode.name()).size(12.0))
                            .width(combo_width)
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
                    });

                    // Theme Dropdown group (Label + ComboBox)
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Theme:").size(12.0).color(Color32::from_rgb(160, 175, 200)));
                        egui::ComboBox::from_id_source("color_theme_combo")
                            .selected_text(RichText::new(current_theme.name()).size(12.0))
                            .width(theme_combo_width)
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

                    // Center Dropdown group (only in Circular mode: Label + ComboBox + optional Image button)
                    if *current_mode == VisualizerMode::Circular {
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Center:").size(12.0).color(Color32::from_rgb(160, 175, 200)));
                            egui::ComboBox::from_id_source("circle_center_combo")
                                .selected_text(RichText::new(circle_center_display.name()).size(12.0))
                                .width(combo_width)
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
                        });
                    }

                    ui.separator();

                    // Gain group (Label + Knob + Value inside the knob)
                    ui.horizontal_centered(|ui| {
                        ui.label(
                            RichText::new("⚡ Gain")
                                .size(11.0)
                                .color(Color32::from_rgb(160, 175, 200)),
                        );
                        ui.add(GainKnob::new(visual_gain, 0.5..=3.5).size(48.0));
                    });
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

/// A circular dial ("knob") that edits an `f32` within a range.
///
/// egui has no built-in knob, so we draw a compact one:
/// a dark circle with a 270° arc showing the current value, a pointer dot at the
/// arc's leading edge, and the numeric value rendered in the center. Drag
/// vertically (or horizontally) to change the value; scroll while hovering to
/// nudge it. The knob edits the same [`f32`] (and range) that the old Gain
/// slider used, so state updates are identical.
struct GainKnob<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    size: f32,
}

impl<'a> GainKnob<'a> {
    fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            size: 48.0,
        }
    }

    fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Widget for GainKnob<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        use std::f32::consts::PI;

        let (min, max) = (*self.range.start(), *self.range.end());
        let span = (max - min).max(1e-6);
        let size = self.size;

        let (rect, mut response) = ui.allocate_exact_size(Vec2::splat(size), Sense::drag());

        let mut changed = false;

        if response.dragged() {
            let delta = response.drag_delta();
            let dominant = if delta.x.abs() > delta.y.abs() {
                delta.x
            } else {
                -delta.y
            };
            let sensitivity = 0.5;
            let value_delta = dominant * (span / size) * sensitivity;
            if value_delta != 0.0 {
                *self.value = (*self.value + value_delta).clamp(min, max);
                changed = true;
            }
        }

        let scroll = if response.hovered() {
            ui.input(|i| i.raw_scroll_delta.y)
        } else {
            0.0
        };
        if scroll != 0.0 {
            let next = *self.value + scroll.signum() * span / 40.0;
            if next != *self.value {
                *self.value = next.clamp(min, max);
                changed = true;
            }
        }

        if changed {
            response.mark_changed();
        }

        let frac = ((*self.value - min) / span).clamp(0.0, 1.0);
        let center = rect.center();
        let radius = size * 0.5 - 2.0;
        let painter = ui.painter();

        // Knob body
        painter.circle_filled(center, radius, Color32::from_rgb(22, 28, 44));
        painter.circle_stroke(center, radius, Stroke::new(1.0_f32, Color32::from_rgb(45, 65, 100)));

        // Background arc (270°) and value arc, both drawn on a ring inside the body.
        let arc_radius = radius - 5.0;
        let start_angle = 135.0_f32.to_radians();
        let sweep = PI * 1.5;
        let current_angle = start_angle + sweep * frac;

        let arc_stroke = Stroke::new(2.5_f32, Color32::from_rgb(50, 60, 85));
        painter.add(egui::Shape::line(
            arc_points(center, start_angle, start_angle + sweep, arc_radius, 24),
            arc_stroke,
        ));

        painter.add(egui::Shape::line(
            arc_points(center, start_angle, current_angle, arc_radius, 24),
            Stroke::new(2.5_f32, Color32::from_rgb(0, 240, 255)),
        ));

        // Pointer dot at the arc's leading edge
        painter.circle_filled(
            center + Vec2::angled(current_angle) * arc_radius,
            2.5,
            Color32::from_rgb(0, 240, 255),
        );

        // Value text, anchored dead-center on the knob rect. `Align2::CENTER_CENTER`
        // pins the *center* of the laid-out galley to `center` (top-left anchoring
        // at `center` is what pushes text down-and-right). Lay out the galley first
        // so we can also shrink the font until the label fits inside the arc ring.
        let label = format!("{:.1}x", *self.value);
        let text_color = Color32::from_rgb(225, 235, 255);
        let mut font_size = size * 0.32;
        let max_label_width = (arc_radius * 2.0 - 6.0).max(8.0);
        let mut galley = painter.layout_no_wrap(label.clone(), FontId::monospace(font_size), text_color);
        if galley.size().x > max_label_width {
            font_size *= max_label_width / galley.size().x;
            galley = painter.layout_no_wrap(label, FontId::monospace(font_size), text_color);
        }
        painter.galley(center - 0.5 * galley.size(), galley, text_color);

        response.on_hover_text(format!("Gain: {:.2}x (drag or scroll)", *self.value))
    }
}

fn arc_points(center: Pos2, start_angle: f32, end_angle: f32, radius: f32, segments: usize) -> Vec<Pos2> {
    let n = (segments as f32 * ((end_angle - start_angle) / (std::f32::consts::PI * 1.5)).abs())
        .ceil()
        .max(2.0) as usize;
    let n = n.min(256);
    (0..=n)
        .map(|i| {
            let t = i as f32 / n as f32;
            center + Vec2::angled(start_angle + (end_angle - start_angle) * t) * radius
        })
        .collect()
}
