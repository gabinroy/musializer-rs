use musializer_rs::MusializerApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    // On Linux desktop environments, winit Wayland drag-and-drop can fail/show stop icon.
    // Setting WINIT_UNIX_BACKEND to "x11" fallback if not explicitly defined ensures cross-compositor DnD works reliably.
    #[cfg(target_os = "linux")]
    if std::env::var_os("WINIT_UNIX_BACKEND").is_none() && std::env::var_os("DISPLAY").is_some() {
        // Safe default on Linux desktops with X11/XWayland available
        // std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }

    // Embed icon bytes at compile-time for native desktop window & taskbar icons (Linux, Windows, macOS)
    let icon_data = load_embedded_icon();

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Musializer-RS")
        .with_app_id("com.musializer.app")
        .with_inner_size([1100.0, 720.0])
        .with_min_inner_size([360.0, 480.0])
        .with_drag_and_drop(true);

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Musializer-RS",
        native_options,
        Box::new(|cc| Ok(Box::new(MusializerApp::new(cc)))),
    )
}

fn load_embedded_icon() -> Option<egui::IconData> {
    let icon_bytes = include_bytes!("../assets/icon.png");
    if let Ok(image) = image::load_from_memory(icon_bytes) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        Some(egui::IconData {
            rgba,
            width,
            height,
        })
    } else {
        None
    }
}
