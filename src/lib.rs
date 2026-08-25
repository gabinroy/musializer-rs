pub mod app;
pub mod audio;
pub mod dsp;
pub mod export;
pub mod ui;

pub use app::MusializerApp;

// WebAssembly (WASM) browser entrypoint
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "musializer_canvas",
                web_options,
                Box::new(|cc| Ok(Box::new(MusializerApp::new(cc)))),
            )
            .await
            .expect("Failed to start eframe on canvas");
    });
}
