use eframe::egui;
use egui::{
    Align, Color32, Layout, Rect, RichText, Rounding, Sense, Stroke, TextureHandle, Ui, Vec2,
    ViewportCommand,
};

/// Height of the custom title bar in pixels.
pub const TITLE_BAR_HEIGHT: f32 = 36.0;

/// Width of each window control button.
const CONTROL_WIDTH: f32 = 35.0;
/// Total width of the right-hand window control button block including spacing.
const CONTROL_BLOCK_WIDTH: f32 = CONTROL_WIDTH * 3.0 + 2.0 * 2.0;

pub struct CustomTitleBar;

impl CustomTitleBar {
    /// Renders the custom title bar and returns the drag interaction response
    /// for the flexible spacer area so the window can be moved by dragging.
    pub fn show(
        ui: &mut Ui,
        logo_texture: Option<&TextureHandle>,
        title: &str,
    ) -> egui::Response {
        let height = TITLE_BAR_HEIGHT;
        let full_width = ui.available_width();

        // Draw the title bar background.
        ui.painter().rect_filled(
            egui::Rect::from_min_size(ui.cursor().min, egui::vec2(full_width, height)),
            0.0,
            Color32::from_rgb(14, 17, 24),
        );

        let mut drag_rect: Option<egui::Rect> = None;

        ui.horizontal(|ui| {
            ui.set_height(height);
            ui.spacing_mut().item_spacing.x = 8.0;

            // Left group: logo + title as a single indivisible unit.
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                if let Some(tex) = logo_texture {
                    let logo_size = 22.0;
                    ui.add(
                        egui::Image::new((tex.id(), egui::vec2(logo_size, logo_size)))
                            .rounding(Rounding::same(3.0)),
                    );
                }
                ui.label(
                    RichText::new(title)
                        .size(14.0)
                        .strong()
                        .color(Color32::from_rgb(220, 228, 240)),
                );
            });

            // Flexible spacer: gap between the left group and the right controls.
            let spacer_min = ui.cursor().min;
            let spacer_width = ui.available_width().max(0.0);
            ui.add_space(spacer_width);

            // Right group: window control buttons, flush right.
            let control_block_width = CONTROL_BLOCK_WIDTH;
            let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 2.0;

                if control_button(ui, [CONTROL_WIDTH, height], "Close", |painter, rect, stroke| {
                    close_icon(painter, rect, stroke);
                })
                .clicked()
                {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                }

                if control_button(
                    ui,
                    [CONTROL_WIDTH, height],
                    "Maximize",
                    |painter, rect, stroke| {
                        if maximized {
                            restore_icon(painter, rect, stroke);
                        } else {
                            maximize_icon(painter, rect, stroke);
                        }
                    },
                )
                .clicked()
                {
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
                }

                if control_button(
                    ui,
                    [CONTROL_WIDTH, height],
                    "Minimize",
                    minimize_icon,
                )
                .clicked()
                {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
                }
            });

            // Draggable spacer area excludes the control button block.
            let drag_width = (spacer_width - control_block_width).max(0.0);
            if drag_width > 0.0 {
                drag_rect = Some(egui::Rect::from_min_size(
                    spacer_min,
                    egui::vec2(drag_width, height),
                ));
            }
        });

        if let Some(rect) = drag_rect {
            let response =
                ui.interact(rect, egui::Id::new("title_bar_drag_area"), Sense::drag());
            if response.is_pointer_button_down_on() {
                ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
            }
            response
        } else {
            ui.interact(
                egui::Rect::from_min_size(ui.cursor().min, egui::vec2(0.0, height)),
                egui::Id::new("title_bar_drag_area"),
                Sense::drag(),
            )
        }
    }
}

/// A custom window-control button with a manually drawn icon.
///
/// Unlike a text-based `Button`, this does not rely on any font glyph being
/// available (which would otherwise render as an empty "tofu" box). The
/// foreground is drawn by `draw_icon` using the given painter.
fn control_button(
    ui: &mut Ui,
    size: [f32; 2],
    hover_text: &str,
    draw_icon: impl FnOnce(&egui::Painter, Rect, Stroke),
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(size[0], size[1]), Sense::click());
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();

    let background = if pressed {
        Color32::from_rgb(40, 45, 58)
    } else if hovered {
        Color32::from_rgb(32, 36, 47)
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 0.0, background);

    let width = if pressed { 1.25_f32 } else { 1.0_f32 };
    let stroke = Stroke::new(width, Color32::from_rgb(210, 218, 230));
    let painter = ui.painter_at(rect);
    draw_icon(&painter, rect, stroke);

    response.on_hover_text(hover_text)
}

/// Minimize: a single short horizontal line.
fn minimize_icon(painter: &egui::Painter, rect: Rect, stroke: Stroke) {
    let y = rect.center().y;
    let x0 = rect.center().x - 7.0;
    let x1 = rect.center().x + 7.0;
    painter.line_segment([egui::pos2(x0, y), egui::pos2(x1, y)], stroke);
}

/// Maximize: a hollow rectangle.
fn maximize_icon(painter: &egui::Painter, rect: Rect, stroke: Stroke) {
    let half = 7.0;
    let center = rect.center();
    let r = Rect::from_center_size(center, Vec2::new(half * 2.0, half * 2.0));
    painter.rect_stroke(r, 0.0, stroke);
}

/// Restore (shown while maximized): two overlapping rectangles.
fn restore_icon(painter: &egui::Painter, rect: Rect, stroke: Stroke) {
    let center = rect.center();
    let w = 12.0;
    let h = 12.0;
    let front_rect = Rect::from_center_size(center, Vec2::new(w, h));
    painter.rect_stroke(front_rect, 0.0, stroke);

    let back_rect = Rect::from_min_max(
        egui::pos2(front_rect.min.x - 3.0, front_rect.min.y - 3.0),
        egui::pos2(front_rect.max.x - 3.0, front_rect.max.y - 3.0),
    );
    painter.rect_stroke(back_rect, 0.0, stroke);
}

/// Close: two crossing diagonal lines.
fn close_icon(painter: &egui::Painter, rect: Rect, stroke: Stroke) {
    let center = rect.center();
    let d = 7.0;
    painter.line_segment(
        [egui::pos2(center.x - d, center.y - d), egui::pos2(center.x + d, center.y + d)],
        stroke,
    );
    painter.line_segment(
        [egui::pos2(center.x + d, center.y - d), egui::pos2(center.x - d, center.y + d)],
        stroke,
    );
}
