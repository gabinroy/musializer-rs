use egui::{Color32, Margin, Rounding, Shadow, Stroke, Style, Visuals};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorTheme {
    CyberNeon,   // Neon cyan to hot magenta
    SunsetGlow,  // Radiant amber to crimson
    EmeraldDeep, // Mint green to oceanic blue
    Monochrome,  // Sleek minimalist silver
}

impl ColorTheme {
    pub fn name(&self) -> &'static str {
        match self {
            ColorTheme::CyberNeon => "Cyber Neon",
            ColorTheme::SunsetGlow => "Sunset Glow",
            ColorTheme::EmeraldDeep => "Emerald Deep",
            ColorTheme::Monochrome => "Monochrome",
        }
    }

    /// Returns a gradient color for a normalized fraction [0.0, 1.0] across frequency or height
    pub fn get_gradient_color(&self, t: f32) -> Color32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            ColorTheme::CyberNeon => {
                if t < 0.5 {
                    let k = t * 2.0;
                    lerp_color(
                        Color32::from_rgb(0, 240, 255),
                        Color32::from_rgb(140, 60, 255),
                        k,
                    )
                } else {
                    let k = (t - 0.5) * 2.0;
                    lerp_color(
                        Color32::from_rgb(140, 60, 255),
                        Color32::from_rgb(255, 40, 160),
                        k,
                    )
                }
            }
            ColorTheme::SunsetGlow => {
                if t < 0.5 {
                    let k = t * 2.0;
                    lerp_color(
                        Color32::from_rgb(255, 210, 50),
                        Color32::from_rgb(255, 90, 40),
                        k,
                    )
                } else {
                    let k = (t - 0.5) * 2.0;
                    lerp_color(
                        Color32::from_rgb(255, 90, 40),
                        Color32::from_rgb(220, 25, 70),
                        k,
                    )
                }
            }
            ColorTheme::EmeraldDeep => {
                if t < 0.5 {
                    let k = t * 2.0;
                    lerp_color(
                        Color32::from_rgb(30, 255, 180),
                        Color32::from_rgb(0, 190, 140),
                        k,
                    )
                } else {
                    let k = (t - 0.5) * 2.0;
                    lerp_color(
                        Color32::from_rgb(0, 190, 140),
                        Color32::from_rgb(20, 110, 240),
                        k,
                    )
                }
            }
            ColorTheme::Monochrome => {
                let val = (255.0 - t * 180.0) as u8;
                Color32::from_rgb(val, val, (val as f32 * 1.05).min(255.0) as u8)
            }
        }
    }
}

fn lerp_color(c1: Color32, c2: Color32, t: f32) -> Color32 {
    let r = (c1.r() as f32 + t * (c2.r() as f32 - c1.r() as f32)) as u8;
    let g = (c1.g() as f32 + t * (c2.g() as f32 - c1.g() as f32)) as u8;
    let b = (c1.b() as f32 + t * (c2.b() as f32 - c1.b() as f32)) as u8;
    Color32::from_rgb(r, g, b)
}

/// Applies custom modern sleek dark theme styling to egui context
pub fn apply_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    // Dark OLED charcoal background
    visuals.panel_fill = Color32::from_rgb(11, 13, 19);
    visuals.window_fill = Color32::from_rgb(16, 19, 28);
    visuals.faint_bg_color = Color32::from_rgb(20, 24, 36);
    visuals.extreme_bg_color = Color32::from_rgb(7, 9, 13);

    // Widget styling (rounded corners, soft borders)
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(18, 22, 33);
    visuals.widgets.noninteractive.rounding = Rounding::same(8.0_f32);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(30, 36, 52));

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(24, 30, 44);
    visuals.widgets.inactive.rounding = Rounding::same(8.0_f32);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(40, 48, 70));

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(36, 44, 65);
    visuals.widgets.hovered.rounding = Rounding::same(8.0_f32);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(120, 80, 240));

    visuals.widgets.active.bg_fill = Color32::from_rgb(60, 50, 120);
    visuals.widgets.active.rounding = Rounding::same(8.0_f32);
    visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, Color32::from_rgb(180, 100, 255));

    // Selection & Accent
    visuals.selection.bg_fill = Color32::from_rgb(130, 60, 240);
    visuals.selection.stroke = Stroke::new(1.0_f32, Color32::from_rgb(220, 180, 255));
    visuals.window_shadow = Shadow::NONE;

    ctx.set_visuals(visuals);

    let mut style: Style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.window_margin = Margin::same(16.0);
    style.spacing.button_padding = egui::vec2(14.0, 8.0);
    ctx.set_style(style);
}
