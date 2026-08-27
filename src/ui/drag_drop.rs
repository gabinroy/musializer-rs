use egui::{Align, Align2, Button, Color32, FontId, Layout, RichText, Rounding, Stroke, Ui};

#[derive(Debug)]
pub enum DroppedItem {
    AudioPath(std::path::PathBuf),
    AudioBytes {
        name: Option<String>,
        bytes: Vec<u8>,
    },
    ImagePath(std::path::PathBuf),
    ImageBytes(Vec<u8>),
}

pub struct DragDropOverlay;

impl DragDropOverlay {
    /// Checks for dropped files or renders an appropriate desktop empty prompt
    pub fn check_and_render(
        ui: &mut Ui,
        is_empty: bool,
        on_open_file_click: &mut bool,
    ) -> Option<DroppedItem> {
        let mut dropped_item = None;

        // Desktop drag-and-drop
        let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        for file in dropped_files {
            if let Some(path) = file.path {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if matches!(
                    ext.as_str(),
                    "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "ico" | "tiff" | "tif"
                ) {
                    dropped_item = Some(DroppedItem::ImagePath(path));
                } else {
                    dropped_item = Some(DroppedItem::AudioPath(path));
                }
                break;
            } else if let Some(bytes) = file.bytes {
                let name = file.name.clone();
                let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
                if matches!(
                    ext.as_str(),
                    "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "ico" | "tiff" | "tif"
                ) {
                    dropped_item = Some(DroppedItem::ImageBytes(bytes.to_vec()));
                } else {
                    dropped_item = Some(DroppedItem::AudioBytes {
                        name: Some(name),
                        bytes: bytes.to_vec(),
                    });
                }
                break;
            }
        }

        let is_linux = cfg!(target_os = "linux");

        // Draw empty state prompt if no track is loaded
        if is_empty {
            let available_rect = ui.available_rect_before_wrap();
            let drop_area_rect = available_rect.shrink(24.0);

            let response = ui.allocate_rect(drop_area_rect, egui::Sense::click());
            if response.clicked() {
                *on_open_file_click = true;
            }

            let painter = ui.painter();
            let is_hovered = response.hovered();

            // Background card
            let bg_color = if is_hovered {
                Color32::from_rgb(18, 22, 34)
            } else {
                Color32::from_rgb(13, 16, 24)
            };
            let stroke_color = if is_hovered {
                Color32::from_rgb(0, 240, 255)
            } else {
                Color32::from_rgb(38, 48, 68)
            };

            painter.rect_filled(drop_area_rect, Rounding::same(16.0_f32), bg_color);
            painter.rect_stroke(
                drop_area_rect,
                Rounding::same(16.0_f32),
                Stroke::new(if is_hovered { 2.0_f32 } else { 1.5_f32 }, stroke_color),
            );

            // Centered content
            let mut child_ui = ui.child_ui(
                drop_area_rect,
                Layout::top_down(Align::Center).with_cross_align(Align::Center),
                None,
            );

            child_ui.add_space((drop_area_rect.height() * 0.22).max(10.0));

            child_ui.label(RichText::new("🎵").size(44.0));
            child_ui.add_space(8.0);

            let main_title = if is_linux {
                "Click to Open Audio File"
            } else {
                "Drag & Drop Audio File Here"
            };

            child_ui.label(
                RichText::new(main_title)
                    .size(22.0)
                    .strong()
                    .color(Color32::from_rgb(225, 235, 255)),
            );

            child_ui.add_space(4.0);
            child_ui.label(
                RichText::new("Supports MP3, WAV, FLAC, OGG, AAC, M4A")
                    .size(13.0)
                    .color(Color32::from_rgb(130, 145, 175)),
            );

            child_ui.add_space(18.0);

            let open_btn = Button::new(
                RichText::new("📂 Open Audio File")
                    .size(14.0)
                    .strong()
                    .color(Color32::from_rgb(0, 240, 255)),
            )
            .fill(Color32::from_rgb(24, 30, 48))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(50, 70, 110)))
            .rounding(Rounding::same(8.0_f32));

            if child_ui.add_sized([160.0, 36.0], open_btn).clicked() {
                *on_open_file_click = true;
            }
        }

        // Highlight viewport if user is currently hovering a file over the window (Desktop)
        let is_hovering_file = ui.ctx().input(|i| !i.raw.hovered_files.is_empty());
        if is_hovering_file {
            let screen_rect = ui.ctx().screen_rect();
            let painter = ui.ctx().layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("drag_drop_overlay"),
            ));

            painter.rect_filled(
                screen_rect,
                Rounding::ZERO,
                Color32::from_rgba_premultiplied(20, 25, 45, 200),
            );

            painter.rect_stroke(
                screen_rect.shrink(30.0),
                Rounding::same(20.0_f32),
                Stroke::new(3.0_f32, Color32::from_rgb(0, 240, 255)),
            );

            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                "✨ Drop Audio Track to Load",
                FontId::proportional(28.0),
                Color32::from_rgb(255, 255, 255),
            );
        }

        dropped_item
    }
}
