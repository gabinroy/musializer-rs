mod app;
mod audio;
mod dsp;
mod export;
mod ui;

use app::MusializerApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Musializer-RS")
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([720.0, 480.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "Musializer-RS",
        native_options,
        Box::new(|cc| Ok(Box::new(MusializerApp::new(cc)))),
    )
}
