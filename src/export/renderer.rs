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
        current_time: f64,
        total_time: f64,
        track_title: Option<&str>,
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
                self.draw_circular(
                    theme,
                    bands,
                    center_display,
                    center_image,
                    current_time,
                    total_time,
                    track_title,
                );
            }
        }

        &self.buffer
    }

    pub fn to_rgba_image(&self) -> image::RgbaImage {
        image::RgbaImage::from_raw(self.width, self.height, self.buffer.clone()).unwrap_or_default()
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

    /// Renders a glyph from the 5x7 embedded bitmap font scaled up by `scale` with anti-aliasing
    fn draw_glyph_5x7(
        &mut self,
        top_left_x: f32,
        top_left_y: f32,
        ch: char,
        scale: f32,
        r: u8,
        g: u8,
        b: u8,
        alpha: u8,
    ) {
        let bitmap = get_glyph_bitmap(ch);
        let s = scale.max(1.0);

        for row in 0..7 {
            let row_bits = bitmap[row];
            for col in 0..5 {
                if (row_bits & (1 << (4 - col))) != 0 {
                    let x0 = top_left_x + col as f32 * s;
                    let y0 = top_left_y + row as f32 * s;
                    let x1 = (x0 + s).ceil() as u32;
                    let y1 = (y0 + s).ceil() as u32;
                    let x_start = x0.floor().max(0.0) as u32;
                    let y_start = y0.floor().max(0.0) as u32;

                    for py in y_start..y1.min(self.height) {
                        for px in x_start..x1.min(self.width) {
                            self.blend_pixel(px, py, r, g, b, alpha);
                        }
                    }
                }
            }
        }
    }

    /// Renders text centered horizontally around `cx` and vertically around `cy`.
    fn draw_text_centered(
        &mut self,
        cx: f32,
        cy: f32,
        text: &str,
        scale: f32,
        r: u8,
        g: u8,
        b: u8,
        alpha: u8,
    ) {
        let char_w = 5.0 * scale;
        let char_spacing = 1.0 * scale;
        let char_h = 7.0 * scale;
        let total_w =
            text.len() as f32 * char_w + (text.len().saturating_sub(1)) as f32 * char_spacing;

        let mut curr_x = cx - total_w * 0.5;
        let top_y = cy - char_h * 0.5;

        // Render soft drop shadow first
        if scale >= 2.0 {
            let shadow_offset = (scale * 0.4).max(1.0);
            let mut shadow_x = curr_x + shadow_offset;
            let shadow_y = top_y + shadow_offset;
            for ch in text.chars() {
                self.draw_glyph_5x7(
                    shadow_x,
                    shadow_y,
                    ch,
                    scale,
                    0,
                    0,
                    0,
                    (alpha as f32 * 0.6) as u8,
                );
                shadow_x += char_w + char_spacing;
            }
        }

        for ch in text.chars() {
            self.draw_glyph_5x7(curr_x, top_y, ch, scale, r, g, b, alpha);
            curr_x += char_w + char_spacing;
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
        current_time: f64,
        total_time: f64,
        track_title: Option<&str>,
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

        // Helper to format MM:SS
        let format_time = |secs: f64| {
            let total_s = secs.max(0.0) as u64;
            let m = total_s / 60;
            let s = total_s % 60;
            format!("{:02}:{:02}", m, s)
        };

        // Render Center Text / Time if selected
        let font_scale_lg = (min_dim / 1080.0 * 5.0).max(2.0);
        let font_scale_sm = (min_dim / 1080.0 * 2.8).max(1.2);

        match center_display {
            CircleCenterDisplay::TimeElapsed => {
                let time_str = format_time(current_time);
                let total_str = format!("/ {}", format_time(total_time));
                self.draw_text_centered(
                    cx,
                    cy - 12.0 * (min_dim / 1080.0).max(1.0),
                    &time_str,
                    font_scale_lg,
                    255,
                    255,
                    255,
                    255,
                );
                self.draw_text_centered(
                    cx,
                    cy + 22.0 * (min_dim / 1080.0).max(1.0),
                    &total_str,
                    font_scale_sm,
                    160,
                    180,
                    200,
                    255,
                );
            }
            CircleCenterDisplay::TimeRemaining => {
                let rem_time = (total_time - current_time).max(0.0);
                let time_str = format!("-{}", format_time(rem_time));
                let c = theme.get_gradient_color(0.5);
                self.draw_text_centered(
                    cx,
                    cy - 12.0 * (min_dim / 1080.0).max(1.0),
                    &time_str,
                    font_scale_lg,
                    c.r(),
                    c.g(),
                    c.b(),
                    255,
                );
                self.draw_text_centered(
                    cx,
                    cy + 22.0 * (min_dim / 1080.0).max(1.0),
                    "REMAINING",
                    font_scale_sm,
                    140,
                    155,
                    175,
                    255,
                );
            }
            CircleCenterDisplay::TrackTitle => {
                let title = track_title.unwrap_or("Musializer");
                let short_title = if title.len() > 16 {
                    format!("{}...", &title[..13])
                } else {
                    title.to_string()
                };
                let c = theme.get_gradient_color(0.7);
                let title_scale = (min_dim / 1080.0 * 3.4).max(1.5);
                self.draw_text_centered(
                    cx,
                    cy - 10.0 * (min_dim / 1080.0).max(1.0),
                    &short_title,
                    title_scale,
                    255,
                    255,
                    255,
                    255,
                );
                self.draw_text_centered(
                    cx,
                    cy + 22.0 * (min_dim / 1080.0).max(1.0),
                    &format_time(current_time),
                    font_scale_sm,
                    c.r(),
                    c.g(),
                    c.b(),
                    255,
                );
            }
            _ => {}
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

/// 5x7 dot-matrix bitmap definitions for ASCII characters (0-9, A-Z, symbols)
fn get_glyph_bitmap(ch: char) -> [u8; 7] {
    let c = ch.to_ascii_uppercase();
    match c {
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
        ],
        ':' => [
            0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b01100, 0b00000,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '/' => [
            0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000, 0b00000,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100,
        ],
        ' ' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'J' => [
            0b00001, 0b00001, 0b00001, 0b00001, 0b10001, 0b10001, 0b01110,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10001, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10011, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01110, 0b10001, 0b10000, 0b01110, 0b00001, 0b10001, 0b01110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '_' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
        ],
        _ => [
            0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111,
        ], // square box fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_circular_with_time_elapsed() {
        let mut rasterizer = OffscreenRasterizer::new(640, 360);
        let bands = vec![0.5f32; 80];
        let peaks = vec![0.5f32; 80];
        let pcm = vec![0.0f32; 2048];

        let frame = rasterizer.render_frame(
            VisualizerMode::Circular,
            ColorTheme::CyberNeon,
            &bands,
            &peaks,
            &pcm,
            CircleCenterDisplay::TimeElapsed,
            None,
            45.0,
            180.0,
            Some("Test Song"),
        );

        assert_eq!(frame.len(), 640 * 360 * 4);
        // Verify frame is not completely blank (has drawn pixels)
        let non_zero_count = frame.iter().filter(|&&b| b > 20).count();
        assert!(non_zero_count > 100);
    }

    #[test]
    fn test_render_circular_with_time_remaining_and_title() {
        let mut rasterizer = OffscreenRasterizer::new(640, 360);
        let bands = vec![0.5f32; 80];
        let peaks = vec![0.5f32; 80];
        let pcm = vec![0.0f32; 2048];

        let frame_rem = rasterizer.render_frame(
            VisualizerMode::Circular,
            ColorTheme::CyberNeon,
            &bands,
            &peaks,
            &pcm,
            CircleCenterDisplay::TimeRemaining,
            None,
            45.0,
            180.0,
            Some("Test Song"),
        );
        assert_eq!(frame_rem.len(), 640 * 360 * 4);

        let frame_title = rasterizer.render_frame(
            VisualizerMode::Circular,
            ColorTheme::SunsetGlow,
            &bands,
            &peaks,
            &pcm,
            CircleCenterDisplay::TrackTitle,
            None,
            45.0,
            180.0,
            Some("Long Track Title That Truncates"),
        );
        assert_eq!(frame_title.len(), 640 * 360 * 4);
    }
}

