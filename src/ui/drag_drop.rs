use egui::{Align2, Color32, FontId, Rounding, Stroke, Ui};

pub struct DragDropOverlay;

impl DragDropOverlay {
    /// Checks for dropped files or renders an appropriate desktop/mobile empty prompt
    pub fn check_and_render(ui: &mut Ui, is_empty: bool) -> Option<std::path::PathBuf> {
        let mut dropped_path = None;

        // Desktop OS drag-and-drop
        let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        for file in dropped_files {
            if let Some(path) = file.path {
                dropped_path = Some(path);
                break;
            }
        }

        let is_mobile = cfg!(any(target_os = "android", target_os = "ios"));

        // Draw empty state prompt if no track is loaded
        if is_empty {
            let available_rect = ui.available_rect_before_wrap();
            let painter = ui.painter();

            painter.rect_stroke(
                available_rect.shrink(20.0),
                Rounding::same(16.0_f32),
                Stroke::new(2.0_f32, Color32::from_rgb(45, 55, 75)),
            );

            let center = available_rect.center();
            let prompt_text = if is_mobile {
                "🎵 Tap 'Load Audio' below\nto select an audio file\n(MP3, WAV, FLAC, OGG, AAC)"
            } else {
                "🎵 Drag & Drop Audio File Here\n(MP3, WAV, FLAC, OGG, AAC)\n\nor click 'Load Audio'"
            };

            painter.text(
                center,
                Align2::CENTER_CENTER,
                prompt_text,
                FontId::proportional(if is_mobile { 18.0 } else { 20.0 }),
                Color32::from_rgb(130, 145, 175),
            );
        }

        // Highlight viewport if user is currently hovering a file over the window (Desktop/Web)
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

        dropped_path
    }
}
