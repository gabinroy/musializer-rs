use crate::ui::theme::ColorTheme;
use egui::{Color32, Painter, Pos2, Rect, Rounding, Stroke};
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisualizerMode {
    SpectrumBars,
    Waveform,
    Circular,
}

impl VisualizerMode {
    pub fn name(&self) -> &'static str {
        match self {
            VisualizerMode::SpectrumBars => "📊 Spectrum Bars",
            VisualizerMode::Waveform => "〰️ Waveform",
            VisualizerMode::Circular => "⭕ Circular",
        }
    }
}

pub struct VisualizerPainter;

impl VisualizerPainter {
    /// Main paint entrypoint rendering the chosen visualization mode inside the given bounding box
    pub fn paint(
        painter: &Painter,
        rect: Rect,
        mode: VisualizerMode,
        theme: ColorTheme,
        bands: &[f32],
        peaks: &[f32],
        pcm_wave: &[f32],
    ) {
        // Draw deep subtle background glow
        painter.rect_filled(
            rect,
            Rounding::same(12.0_f32),
            Color32::from_rgb(12, 15, 22),
        );
        painter.rect_stroke(
            rect,
            Rounding::same(12.0_f32),
            Stroke::new(1.0_f32, Color32::from_rgb(28, 34, 48)),
        );

        match mode {
            VisualizerMode::SpectrumBars => {
                Self::paint_spectrum_bars(painter, rect, theme, bands, peaks);
            }
            VisualizerMode::Waveform => {
                Self::paint_waveform(painter, rect, theme, pcm_wave);
            }
            VisualizerMode::Circular => {
                Self::paint_circular(painter, rect, theme, bands);
            }
        }
    }

    /// Renders dynamic vertical gradient frequency bars with glowing peak caps
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
        let margin_x = 16.0;
        let margin_y = 16.0;
        let content_width = rect.width() - 2.0 * margin_x;
        let content_height = rect.height() - 2.0 * margin_y;

        let spacing = 3.0f32;
        let bar_width =
            ((content_width - (num_bars as f32 - 1.0) * spacing) / num_bars as f32).max(2.0);

        let base_y = rect.max.y - margin_y;

        for i in 0..num_bars {
            let val = bands[i].clamp(0.0, 1.0);
            let peak_val = peaks.get(i).copied().unwrap_or(val).clamp(0.0, 1.0);

            let bar_height = (val * content_height).max(3.0);
            let bar_x = rect.min.x + margin_x + i as f32 * (bar_width + spacing);

            let bar_rect = Rect::from_min_max(
                Pos2::new(bar_x, base_y - bar_height),
                Pos2::new(bar_x + bar_width, base_y),
            );

            // Color gradient mapped across frequency band index
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
        let amplitude_scale = (rect.height() * 0.4).max(10.0);

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

    /// Renders a radial circular frequency visualizer with bass pulse
    fn paint_circular(painter: &Painter, rect: Rect, theme: ColorTheme, bands: &[f32]) {
        if bands.is_empty() {
            return;
        }

        let center = rect.center();
        let min_dim = rect.width().min(rect.height());
        let base_radius = min_dim * 0.22;
        let max_bar_len = min_dim * 0.22;

        // Bass pulse on inner ring
        let bass_energy = bands.iter().take(8).sum::<f32>() / 8.0;
        let inner_radius = base_radius * (1.0 + bass_energy * 0.15);

        let num_bars = bands.len();
        let angle_step = 2.0 * PI / num_bars as f32;

        for i in 0..num_bars {
            let val = bands[i].clamp(0.0, 1.0);
            let bar_len = val * max_bar_len;
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

        // Inner glowing core
        let core_color = theme.get_gradient_color(0.2);
        painter.circle_filled(center, inner_radius * 0.95, Color32::from_rgb(18, 22, 32));
        painter.circle_stroke(
            center,
            inner_radius * 0.95,
            Stroke::new(2.0_f32, core_color),
        );
    }
}
