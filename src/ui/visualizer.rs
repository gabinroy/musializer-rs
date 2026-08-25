use crate::ui::theme::ColorTheme;
use egui::{Color32, FontFamily, FontId, Painter, Pos2, Rect, Rounding, Stroke, TextureHandle};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizerMode {
    SpectrumBars,
    Waveform,
    Circular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleCenterDisplay {
    None,
    TimeElapsed,
    TimeRemaining,
    TrackTitle,
    CustomCoverArt,
}

impl VisualizerMode {
    pub fn name(&self) -> &'static str {
        match self {
            VisualizerMode::SpectrumBars => "Spectrum Bars",
            VisualizerMode::Waveform => "Oscilloscope Wave",
            VisualizerMode::Circular => "Circular Pulse",
        }
    }
}

impl CircleCenterDisplay {
    pub fn name(&self) -> &'static str {
        match self {
            CircleCenterDisplay::None => "Minimal Glow",
            CircleCenterDisplay::TimeElapsed => "Time Elapsed",
            CircleCenterDisplay::TimeRemaining => "Time Remaining",
            CircleCenterDisplay::TrackTitle => "Track Title",
            CircleCenterDisplay::CustomCoverArt => "Cover Art / Logo",
        }
    }
}

pub struct VisualizerWidget;

impl VisualizerWidget {
    /// Main entry point to paint the audio visualizer
    pub fn show(
        painter: &Painter,
        rect: Rect,
        mode: VisualizerMode,
        theme: ColorTheme,
        bands: &[f32],
        peaks: &[f32],
        pcm_wave: &[f32],
        center_display: CircleCenterDisplay,
        center_texture: Option<&TextureHandle>,
        current_time: f64,
        total_time: f64,
        track_title: Option<&str>,
        image_loading_anim: Option<f32>,
    ) {
        // Draw background canvas
        painter.rect_filled(rect, Rounding::ZERO, Color32::from_rgb(11, 13, 19));

        match mode {
            VisualizerMode::SpectrumBars => {
                Self::paint_spectrum_bars(painter, rect, theme, bands, peaks);
            }
            VisualizerMode::Waveform => {
                Self::paint_waveform(painter, rect, theme, pcm_wave);
            }
            VisualizerMode::Circular => {
                Self::paint_circular(
                    painter,
                    rect,
                    theme,
                    bands,
                    center_display,
                    center_texture,
                    current_time,
                    total_time,
                    track_title,
                    image_loading_anim,
                );
            }
        }
    }

    /// Renders dynamic frequency bars with peak hold dots
    fn paint_spectrum_bars(
        painter: &Painter,
        rect: Rect,
        theme: ColorTheme,
        bands: &[f32],
        peaks: &[f32],
    ) {
        if bands.is_empty() {
            return;
        }

        let num_bars = bands.len();
        let margin_x = 24.0;
        let margin_bottom = 20.0;
        let available_width = rect.width() - (margin_x * 2.0);
        let bar_spacing = 3.0;
        let bar_width =
            ((available_width - (num_bars as f32 - 1.0) * bar_spacing) / num_bars as f32).max(2.0);

        let content_height = rect.height() - margin_bottom - 20.0;
        let base_y = rect.max.y - margin_bottom;

        for i in 0..num_bars {
            let val = bands[i].clamp(0.0, 1.0);
            let peak_val = if i < peaks.len() {
                peaks[i].clamp(0.0, 1.0)
            } else {
                0.0
            };

            let bar_height = (val * content_height).max(4.0);
            let bar_x = rect.min.x + margin_x + i as f32 * (bar_width + bar_spacing);

            let bar_rect = Rect::from_min_max(
                Pos2::new(bar_x, base_y - bar_height),
                Pos2::new(bar_x + bar_width, base_y),
            );

            // Calculate color based on frequency spectrum
            let freq_t = i as f32 / num_bars as f32;
            let bar_color = theme.get_gradient_color(freq_t);

            // Draw bar with rounded top corners
            painter.rect_filled(bar_rect, Rounding::same(2.0_f32), bar_color);

            // Draw floating peak indicator dot
            if peak_val > 0.02 {
                let peak_y = base_y - (peak_val * content_height).max(4.0) - 2.0;
                let peak_rect = Rect::from_min_max(
                    Pos2::new(bar_x, peak_y - 2.0),
                    Pos2::new(bar_x + bar_width, peak_y),
                );
                painter.rect_filled(
                    peak_rect,
                    Rounding::same(1.0_f32),
                    Color32::from_rgb(255, 255, 255),
                );
            }
        }
    }

    /// Renders an antialiased oscilloscope waveform line
    fn paint_waveform(painter: &Painter, rect: Rect, theme: ColorTheme, pcm_wave: &[f32]) {
        if pcm_wave.is_empty() {
            return;
        }

        let margin_x = 20.0;
        let content_width = rect.width() - 2.0 * margin_x;
        let center_y = rect.center().y;
        let amplitude_scale = (rect.height() * 0.45).max(10.0);

        let step = (pcm_wave.len() as f32 / content_width).max(1.0) as usize;
        let mut points = Vec::new();

        let mut i = 0;
        while i < pcm_wave.len() {
            let x = rect.min.x + margin_x + (i as f32 / pcm_wave.len() as f32) * content_width;
            let sample = pcm_wave[i].clamp(-1.0, 1.0);
            let y = center_y - sample * amplitude_scale;
            points.push(Pos2::new(x, y));
            i += step.max(1);
        }

        if points.len() >= 2 {
            let line_color = theme.get_gradient_color(0.5);
            painter.add(egui::Shape::line(points, Stroke::new(2.5_f32, line_color)));
        }
    }

    /// Renders a radial circular frequency visualizer with customizable center hub
    fn paint_circular(
        painter: &Painter,
        rect: Rect,
        theme: ColorTheme,
        bands: &[f32],
        center_display: CircleCenterDisplay,
        center_texture: Option<&TextureHandle>,
        current_time: f64,
        total_time: f64,
        track_title: Option<&str>,
        image_loading_anim: Option<f32>,
    ) {
        if bands.is_empty() {
            return;
        }

        let center = rect.center();
        let min_dim = rect.width().min(rect.height());
        let base_radius = min_dim * 0.23;
        let max_bar_len = min_dim * 0.22;

        // Bass pulse on inner ring
        let bass_energy = bands.iter().take(8).sum::<f32>() / 8.0;
        let inner_radius = base_radius * (1.0 + bass_energy * 0.18);

        let num_bars = bands.len();
        let angle_step = 2.0 * PI / num_bars as f32;

        // Radial frequency rays
        for i in 0..num_bars {
            let val = bands[i].clamp(0.0, 1.0);
            let bar_len = (val * max_bar_len).max(2.0);
            let angle = i as f32 * angle_step - PI / 2.0;

            let cos_a = angle.cos();
            let sin_a = angle.sin();

            let start = Pos2::new(
                center.x + cos_a * inner_radius,
                center.y + sin_a * inner_radius,
            );
            let end = Pos2::new(
                center.x + cos_a * (inner_radius + bar_len),
                center.y + sin_a * (inner_radius + bar_len),
            );

            let freq_t = i as f32 / num_bars as f32;
            let color = theme.get_gradient_color(freq_t);

            painter.line_segment([start, end], Stroke::new(3.0_f32, color));
        }

        // Inner core circle
        let core_radius = inner_radius * 0.95;
        let core_color = theme.get_gradient_color(0.2);

        // Center background
        painter.circle_filled(center, core_radius, Color32::from_rgb(18, 22, 32));

        // Draw Center Image / Cover Art if requested and available
        if center_display == CircleCenterDisplay::CustomCoverArt {
            if let Some(tex) = center_texture {
                let img_radius = core_radius * 0.98;
                let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
                let img_rect =
                    Rect::from_center_size(center, egui::vec2(img_radius * 2.0, img_radius * 2.0));

                let img_alpha = if let Some(prog) = image_loading_anim {
                    ((prog * 2.0).min(1.0) * 255.0) as u8
                } else {
                    255
                };
                let tint = Color32::from_rgba_unmultiplied(
                    img_alpha,
                    img_alpha,
                    img_alpha,
                    img_alpha,
                );
                painter.image(tex.id(), img_rect, uv, tint);
            }
        }

        // Outer glow border
        painter.circle_stroke(center, core_radius, Stroke::new(2.5_f32, core_color));

        // Loading animation spinner / glowing ring during image update
        if let Some(prog) = image_loading_anim {
            let spin_angle = prog * 10.0 * PI;
            let spinner_radius = core_radius * 0.65;
            let num_segments = 20;
            let arc_len = PI * 1.3;
            let mut arc_pts = Vec::new();
            for j in 0..=num_segments {
                let a = spin_angle + (j as f32 / num_segments as f32) * arc_len;
                arc_pts.push(Pos2::new(
                    center.x + a.cos() * spinner_radius,
                    center.y + a.sin() * spinner_radius,
                ));
            }
            let spinner_color = theme.get_gradient_color(0.5);
            let alpha = ((1.0 - prog) * 255.0) as u8;
            let spinner_col = Color32::from_rgba_unmultiplied(
                spinner_color.r(),
                spinner_color.g(),
                spinner_color.b(),
                alpha,
            );
            painter.add(egui::Shape::line(arc_pts, Stroke::new(3.5_f32, spinner_col)));

            // Outer pulse ring
            let pulse_r = core_radius * (1.0 + (1.0 - prog) * 0.15);
            painter.circle_stroke(center, pulse_r, Stroke::new(2.0_f32, spinner_col));
        }

        // Render Center Text / Time if selected
        let format_time = |secs: f64| {
            let total_s = secs.max(0.0) as u64;
            let m = total_s / 60;
            let s = total_s % 60;
            format!("{:02}:{:02}", m, s)
        };

        match center_display {
            CircleCenterDisplay::TimeElapsed => {
                let time_str = format_time(current_time);
                painter.text(
                    center - egui::vec2(0.0, 8.0),
                    egui::Align2::CENTER_CENTER,
                    time_str,
                    FontId::new(24.0, FontFamily::Proportional),
                    Color32::WHITE,
                );
                let total_str = format!("/ {}", format_time(total_time));
                painter.text(
                    center + egui::vec2(0.0, 16.0),
                    egui::Align2::CENTER_CENTER,
                    total_str,
                    FontId::new(13.0, FontFamily::Proportional),
                    Color32::from_rgb(160, 180, 200),
                );
            }
            CircleCenterDisplay::TimeRemaining => {
                let rem_time = (total_time - current_time).max(0.0);
                let time_str = format!("-{}", format_time(rem_time));
                painter.text(
                    center - egui::vec2(0.0, 6.0),
                    egui::Align2::CENTER_CENTER,
                    time_str,
                    FontId::new(24.0, FontFamily::Proportional),
                    theme.get_gradient_color(0.5),
                );
                painter.text(
                    center + egui::vec2(0.0, 16.0),
                    egui::Align2::CENTER_CENTER,
                    "REMAINING",
                    FontId::new(11.0, FontFamily::Proportional),
                    Color32::from_rgb(140, 155, 175),
                );
            }
            CircleCenterDisplay::TrackTitle => {
                let title = track_title.unwrap_or("Musializer");
                // Truncate if long
                let short_title = if title.len() > 18 {
                    format!("{}...", &title[..15])
                } else {
                    title.to_string()
                };
                painter.text(
                    center - egui::vec2(0.0, 6.0),
                    egui::Align2::CENTER_CENTER,
                    short_title,
                    FontId::new(16.0, FontFamily::Proportional),
                    Color32::WHITE,
                );
                painter.text(
                    center + egui::vec2(0.0, 16.0),
                    egui::Align2::CENTER_CENTER,
                    format_time(current_time),
                    FontId::new(13.0, FontFamily::Proportional),
                    theme.get_gradient_color(0.7),
                );
            }
            _ => {}
        }
    }
}
