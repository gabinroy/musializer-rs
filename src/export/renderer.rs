use crate::ui::theme::ColorTheme;
use crate::ui::visualizer::{CircleCenterDisplay, VisualizerMode};
use std::f32::consts::PI;

/// Pure-software high-performance RGBA frame rasterizer for offscreen video rendering.
pub struct OffscreenRasterizer {
    pub width: u32,
    pub height: u32,
    buffer: Vec<u8>,
}

impl OffscreenRasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = vec![0u8; (width * height * 4) as usize];
        Self {
            width,
            height,
            buffer,
        }
    }

    /// Renders a single deterministic visualization frame into the RGBA byte buffer.
    pub fn render_frame(
        &mut self,
        mode: VisualizerMode,
        theme: ColorTheme,
        bands: &[f32],
        peaks: &[f32],
        pcm_wave: &[f32],
        center_display: CircleCenterDisplay,
        center_image: Option<&image::RgbaImage>,
    ) -> &[u8] {
        // Clear background with deep dark charcoal (#0B0D13)
        self.clear(11, 13, 19, 255);

        match mode {
            VisualizerMode::SpectrumBars => {
                self.draw_spectrum_bars(theme, bands, peaks);
            }
            VisualizerMode::Waveform => {
                self.draw_waveform(theme, pcm_wave);
            }
            VisualizerMode::Circular => {
                self.draw_circular(theme, bands, center_display, center_image);
            }
        }

        &self.buffer
    }

    fn clear(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for chunk in self.buffer.chunks_exact_mut(4) {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
        }
    }

    #[inline(always)]
    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height {
            let idx = ((y * self.width + x) * 4) as usize;
            self.buffer[idx] = r;
            self.buffer[idx + 1] = g;
            self.buffer[idx + 2] = b;
            self.buffer[idx + 3] = a;
        }
    }

    #[inline(always)]
    fn blend_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height && a > 0 {
            let idx = ((y * self.width + x) * 4) as usize;
            if a == 255 {
                self.buffer[idx] = r;
                self.buffer[idx + 1] = g;
                self.buffer[idx + 2] = b;
                self.buffer[idx + 3] = 255;
            } else {
                let alpha = a as f32 / 255.0;
                let inv_alpha = 1.0 - alpha;
                let bg_r = self.buffer[idx] as f32;
                let bg_g = self.buffer[idx + 1] as f32;
                let bg_b = self.buffer[idx + 2] as f32;
                self.buffer[idx] = (r as f32 * alpha + bg_r * inv_alpha) as u8;
                self.buffer[idx + 1] = (g as f32 * alpha + bg_g * inv_alpha) as u8;
                self.buffer[idx + 2] = (b as f32 * alpha + bg_b * inv_alpha) as u8;
                self.buffer[idx + 3] = 255;
            }
        }
    }

    fn fill_rect(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, r: u8, g: u8, b: u8, a: u8) {
        let x_start = x0.min(self.width);
        let x_end = x1.min(self.width);
        let y_start = y0.min(self.height);
        let y_end = y1.min(self.height);

        for y in y_start..y_end {
            for x in x_start..x_end {
                self.set_pixel(x, y, r, g, b, a);
            }
        }
    }

    fn draw_circle_filled(&mut self, cx: f32, cy: f32, radius: f32, r: u8, g: u8, b: u8) {
        let min_x = (cx - radius).floor().max(0.0) as u32;
        let max_x = (cx + radius).ceil().min((self.width - 1) as f32) as u32;
        let min_y = (cy - radius).floor().max(0.0) as u32;
        let max_y = (cy + radius).ceil().min((self.height - 1) as f32) as u32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 + 0.5 - cx;
                let dy = y as f32 + 0.5 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= radius - 1.0 {
                    self.set_pixel(x, y, r, g, b, 255);
                } else if dist < radius {
                    let a = ((radius - dist) * 255.0) as u8;
                    self.blend_pixel(x, y, r, g, b, a);
                }
            }
        }
    }

    fn draw_circle_stroke(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        thickness: f32,
        r: u8,
        g: u8,
        b: u8,
    ) {
        let half_t = thickness * 0.5;
        let outer_r = radius + half_t;
        let min_x = (cx - outer_r).floor().max(0.0) as u32;
        let max_x = (cx + outer_r).ceil().min((self.width - 1) as f32) as u32;
        let min_y = (cy - outer_r).floor().max(0.0) as u32;
        let max_y = (cy + outer_r).ceil().min((self.height - 1) as f32) as u32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 + 0.5 - cx;
                let dy = y as f32 + 0.5 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                let dist_from_ring = (dist - radius).abs();
                if dist_from_ring <= half_t {
                    let a = ((1.0 - (dist_from_ring / (half_t + 0.5)).powi(2)) * 255.0)
                        .clamp(0.0, 255.0) as u8;
                    self.blend_pixel(x, y, r, g, b, a);
                }
            }
        }
    }

    fn draw_circle_image(&mut self, cx: f32, cy: f32, radius: f32, img: &image::RgbaImage) {
        let min_x = (cx - radius).floor().max(0.0) as u32;
        let max_x = (cx + radius).ceil().min((self.width - 1) as f32) as u32;
        let min_y = (cy - radius).floor().max(0.0) as u32;
        let max_y = (cy + radius).ceil().min((self.height - 1) as f32) as u32;

        let img_w = img.width() as f32;
        let img_h = img.height() as f32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 + 0.5 - cx;
                let dy = y as f32 + 0.5 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < radius {
                    let u = ((dx / radius) * 0.5 + 0.5).clamp(0.0, 1.0);
                    let v = ((dy / radius) * 0.5 + 0.5).clamp(0.0, 1.0);
                    let src_x = (u * (img_w - 1.0)).round() as u32;
                    let src_y = (v * (img_h - 1.0)).round() as u32;

                    let pixel = img.get_pixel(
                        src_x.min(img.width().saturating_sub(1)),
                        src_y.min(img.height().saturating_sub(1)),
                    );
                    let mut alpha = pixel[3];
                    if dist >= radius - 1.5 {
                        let edge_factor = (radius - dist) / 1.5;
                        alpha = ((alpha as f32) * edge_factor.clamp(0.0, 1.0)) as u8;
                    }
                    self.blend_pixel(x, y, pixel[0], pixel[1], pixel[2], alpha);
                }
            }
        }
    }

    fn draw_spectrum_bars(&mut self, theme: ColorTheme, bands: &[f32], peaks: &[f32]) {
        if bands.is_empty() {
            return;
        }

        let margin_x = (self.width as f32 * 0.05) as u32;
        let margin_y = (self.height as f32 * 0.08) as u32;
        let content_w = self.width - 2 * margin_x;
        let content_h = self.height - 2 * margin_y;

        let num_bars = bands.len();
        let spacing = 3u32;
        let bar_w =
            ((content_w.saturating_sub((num_bars as u32 - 1) * spacing)) / num_bars as u32).max(2);
        let base_y = self.height - margin_y;

        for i in 0..num_bars {
            let val = bands[i].clamp(0.0, 1.0);
            let peak_val = peaks.get(i).copied().unwrap_or(val).clamp(0.0, 1.0);

            let bar_h = (val * content_h as f32) as u32;
            let bar_x = margin_x + i as u32 * (bar_w + spacing);
            let bar_top = base_y.saturating_sub(bar_h);

            let freq_t = i as f32 / num_bars as f32;
            let col = theme.get_gradient_color(freq_t);

            self.fill_rect(
                bar_x,
                bar_top,
                bar_x + bar_w,
                base_y,
                col.r(),
                col.g(),
                col.b(),
                255,
            );

            // Floating peak cap
            if peak_val > 0.02 {
                let peak_y = base_y.saturating_sub((peak_val * content_h as f32) as u32 + 4);
                self.fill_rect(
                    bar_x,
                    peak_y.saturating_sub(3),
                    bar_x + bar_w,
                    peak_y,
                    255,
                    255,
                    255,
                    255,
                );
            }
        }
    }

    fn draw_waveform(&mut self, theme: ColorTheme, pcm_wave: &[f32]) {
        if pcm_wave.is_empty() {
            return;
        }

        let center_y = self.height as f32 * 0.5;
        let amp_scale = self.height as f32 * 0.35;
        let col = theme.get_gradient_color(0.5);

        for x in 0..self.width {
            let sample_idx = ((x as f32 / self.width as f32) * pcm_wave.len() as f32) as usize;
            if sample_idx < pcm_wave.len() {
                let s = pcm_wave[sample_idx].clamp(-1.0, 1.0);
                let y = (center_y - s * amp_scale).clamp(0.0, (self.height - 1) as f32) as u32;
                self.fill_rect(
                    x,
                    y.saturating_sub(1),
                    x + 1,
                    (y + 2).min(self.height),
                    col.r(),
                    col.g(),
                    col.b(),
                    255,
                );
            }
        }
    }

    fn draw_circular(
        &mut self,
        theme: ColorTheme,
        bands: &[f32],
        center_display: CircleCenterDisplay,
        center_image: Option<&image::RgbaImage>,
    ) {
        if bands.is_empty() {
            return;
        }

        let cx = self.width as f32 * 0.5;
        let cy = self.height as f32 * 0.5;
        let min_dim = (self.width.min(self.height)) as f32;
        let base_radius = min_dim * 0.23;
        let max_bar_len = min_dim * 0.22;

        let bass_energy = bands.iter().take(8).sum::<f32>() / 8.0;
        let inner_radius = base_radius * (1.0 + bass_energy * 0.18);
        let core_radius = inner_radius * 0.95;

        let num_bars = bands.len();
        let angle_step = 2.0 * PI / num_bars as f32;

        // Draw radial frequency rays
        for i in 0..num_bars {
            let val = bands[i].clamp(0.0, 1.0);
            let bar_len = (val * max_bar_len).max(2.0);
            let angle = i as f32 * angle_step - PI * 0.5;

            let col = theme.get_gradient_color(i as f32 / num_bars as f32);

            let steps = 60;
            for s in 0..steps {
                let r = inner_radius + (s as f32 / steps as f32) * bar_len;
                let px = (cx + angle.cos() * r).round() as u32;
                let py = (cy + angle.sin() * r).round() as u32;
                self.fill_rect(
                    px.saturating_sub(1),
                    py.saturating_sub(1),
                    px + 2,
                    py + 2,
                    col.r(),
                    col.g(),
                    col.b(),
                    255,
                );
            }
        }

        // Draw inner core circle background (#121620)
        self.draw_circle_filled(cx, cy, core_radius, 18, 22, 32);

        // Draw center cover image if enabled and available
        if center_display == CircleCenterDisplay::CustomCoverArt {
            if let Some(img) = center_image {
                self.draw_circle_image(cx, cy, core_radius * 0.98, img);
            }
        }

        // Draw glowing outer border ring
        let core_col = theme.get_gradient_color(0.2);
        self.draw_circle_stroke(
            cx,
            cy,
            core_radius,
            3.0,
            core_col.r(),
            core_col.g(),
            core_col.b(),
        );
    }
}
