use musializer_desktop::MusializerApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    // Check for CLI help or flags
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Musializer-RS - High-Performance Real-Time Audio Visualizer\n");
        println!("Usage: musializer-rs [OPTIONS]\n");
        println!("Options:");
        println!("  --x11       Force X11 / XWayland backend (recommended if native Wayland DnD issues occur)");
        println!("  --wayland   Force native Wayland backend");
        println!("  -h, --help  Print help information");
        println!("  -v, --version Print version");
        return Ok(());
    }

    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("Musializer-RS v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Backend selection for Linux environments
    #[cfg(target_os = "linux")]
    {
        if args.iter().any(|a| a == "--x11") {
            // SAFETY: Executed at the very start of main before multi-threaded activity
            unsafe {
                std::env::set_var("WINIT_UNIX_BACKEND", "x11");
            }
            log::info!("Forced X11/XWayland backend via --x11 CLI flag");
        } else if args.iter().any(|a| a == "--wayland") {
            // SAFETY: Executed at the very start of main before multi-threaded activity
            unsafe {
                std::env::set_var("WINIT_UNIX_BACKEND", "wayland");
            }
            log::info!("Forced native Wayland backend via --wayland CLI flag");
        }
    }

    // Embed icon bytes at compile-time for native desktop window & taskbar icons (Linux, Windows, macOS)
    let icon_data = load_embedded_icon();

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Musializer-RS")
        .with_app_id("com.musializer.app")
        .with_inner_size([1100.0, 720.0])
        .with_min_inner_size([360.0, 480.0])
        .with_drag_and_drop(true)
        .with_decorations(false);

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
