use std::path::PathBuf;
use std::sync::Arc;
use web_time::Instant;

use eframe::egui;
#[allow(unused_imports)]
use egui::{
    CentralPanel, Color32, ColorImage, ProgressBar, RichText, TextureHandle, TextureOptions,
    TopBottomPanel, Window,
};

use crate::audio::{AudioPlayer, AudioSync, AudioTrack};
use crate::dsp::{EmaSmoother, FftProcessor, FrequencyBands};
use crate::export::{ExportConfig, VideoExporter};
use crate::ui::{
    CircleCenterDisplay, ColorTheme, DragDropOverlay, DroppedItem, TransportControls,
    VisualizerMode, VisualizerWidget, apply_theme,
};

pub struct MusializerApp {
    player: Result<AudioPlayer, String>,
    fft_processor: FftProcessor,
    bands_mapper: FrequencyBands,
    smoother: EmaSmoother,

    mode: VisualizerMode,
    theme: ColorTheme,
    num_bands: usize,
    fft_size: usize,

    // DSP dynamics & visual gain
    visual_gain: f32,

    // Circular visualizer center options
    circle_center_display: CircleCenterDisplay,
    circle_center_texture: Option<TextureHandle>,
    raw_cover_image: Option<Arc<image::RgbaImage>>,
    image_loading_timer: Option<Instant>,

    last_frame_time: Instant,
    cached_wave: Vec<f32>,

    // Video Export State (Desktop)
    exporter: VideoExporter,
    show_export_modal: bool,
    export_output_path: String,
    export_fps: u32,
    export_width: u32,
    export_height: u32,

    error_msg: Option<String>,
}

impl MusializerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_theme(&cc.egui_ctx);

        let fft_size = 2048;
        let num_bands = 80;
        let sample_rate = 44100;

        let player = AudioPlayer::new();
        let fft_processor = FftProcessor::new(fft_size);
        let bands_mapper = FrequencyBands::new(num_bands, fft_size, sample_rate);
        let smoother = EmaSmoother::new(num_bands, 0.85, 0.15);

        // Load default embedded logo texture for center cover art
        let (circle_center_texture, raw_cover_image) = load_default_logo(&cc.egui_ctx);

        Self {
            player,
            fft_processor,
            bands_mapper,
            smoother,

            mode: VisualizerMode::SpectrumBars,
            theme: ColorTheme::CyberNeon,
            num_bands,
            fft_size,

            visual_gain: 1.2,
            circle_center_display: CircleCenterDisplay::TimeElapsed,
            circle_center_texture,
            raw_cover_image,
            image_loading_timer: None,

            last_frame_time: Instant::now(),
            cached_wave: vec![0.0; fft_size],

            exporter: VideoExporter::new(),
            show_export_modal: false,
            export_output_path: "musializer_render.mp4".to_string(),
            export_fps: 60,
            export_width: 1920,
            export_height: 1080,

            error_msg: None,
        }
    }

    pub fn load_audio_file(&mut self, path: PathBuf) {
        log::info!("Loading audio file: {:?}", path);
        match AudioTrack::load_from_file(&path) {
            Ok(track) => {
                self.load_track_internal(track);
            }
            Err(e) => {
                log::error!("Failed to load audio file: {}", e);
                self.error_msg = Some(format!("Failed to load audio: {}", e));
            }
        }
    }

    #[allow(dead_code)]
    pub fn load_audio_from_bytes(&mut self, bytes: Vec<u8>, filename: Option<&str>) {
        match AudioTrack::load_from_memory(bytes, filename) {
            Ok(track) => {
                self.load_track_internal(track);
            }
            Err(e) => {
                log::error!("Failed to decode audio memory buffer: {}", e);
                self.error_msg = Some(format!("Failed to decode audio: {}", e));
            }
        }
    }

    pub fn load_custom_cover_image(&mut self, ctx: &egui::Context, path: PathBuf) {
        match std::fs::read(&path) {
            Ok(bytes) => {
                self.load_custom_cover_image_from_bytes(ctx, &bytes);
            }
            Err(e) => {
                log::error!("Failed to read cover image file {:?}: {}", path, e);
                self.error_msg = Some(format!("Failed to read image file: {}", e));
            }
        }
    }

    pub fn load_custom_cover_image_from_bytes(&mut self, ctx: &egui::Context, bytes: &[u8]) {
        match image::load_from_memory(bytes) {
            Ok(img) => {
                let (color_img, rgba_img) = make_circular_color_and_rgba_image(img);
                self.raw_cover_image = Some(Arc::new(rgba_img));
                if let Some(tex) = &mut self.circle_center_texture {
                    tex.set(color_img, TextureOptions::LINEAR);
                } else {
                    self.circle_center_texture =
                        Some(ctx.load_texture("custom_cover", color_img, TextureOptions::LINEAR));
                }
                self.circle_center_display = CircleCenterDisplay::CustomCoverArt;
                self.image_loading_timer = Some(Instant::now());
            }
            Err(e) => {
                log::error!("Failed to decode cover image memory buffer: {}", e);
                self.error_msg = Some(format!("Failed to decode image: {}", e));
            }
        }
    }

    fn load_track_internal(&mut self, track: AudioTrack) {
        let sample_rate = track.sample_rate;
        self.bands_mapper = FrequencyBands::new(self.num_bands, self.fft_size, sample_rate);
        self.smoother.reset();

        if let Ok(player) = &mut self.player {
            if let Err(e) = player.load_track(track) {
                self.error_msg = Some(format!("Error initializing playback: {}", e));
            } else {
                player.play();
                self.error_msg = None;
            }
        }
    }
}

fn make_circular_color_and_rgba_image(
    dynamic_img: image::DynamicImage,
) -> (ColorImage, image::RgbaImage) {
    let (w, h) = (dynamic_img.width(), dynamic_img.height());
    let size = w.min(h);
    let x_offset = (w - size) / 2;
    let y_offset = (h - size) / 2;
    let cropped = dynamic_img
        .crop_imm(x_offset, y_offset, size, size)
        .into_rgba8();

    let mut raw_pixels = cropped.into_raw();
    let radius = size as f32 / 2.0;
    let center = radius;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 + 0.5 - center;
            let dy = y as f32 + 0.5 - center;
            let dist = (dx * dx + dy * dy).sqrt();

            let idx = ((y * size + x) * 4) as usize;
            if dist >= radius {
                raw_pixels[idx + 3] = 0; // Fully transparent outside the circular disc
            } else if dist >= radius - 1.5 {
                // Anti-aliased smooth edge
                let alpha_factor = (radius - dist) / 1.5;
                raw_pixels[idx + 3] =
                    ((raw_pixels[idx + 3] as f32) * alpha_factor.clamp(0.0, 1.0)) as u8;
            }
        }
    }

    let rgba_img = image::RgbaImage::from_raw(size, size, raw_pixels.clone()).unwrap_or_default();
    let color_img = ColorImage::from_rgba_unmultiplied([size as usize, size as usize], &raw_pixels);

    (color_img, rgba_img)
}

fn load_default_logo(
    ctx: &egui::Context,
) -> (Option<TextureHandle>, Option<Arc<image::RgbaImage>>) {
    let icon_bytes = include_bytes!("../assets/icon.png");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let (color_img, rgba_img) = make_circular_color_and_rgba_image(img);
        let tex = ctx.load_texture("default_logo_texture", color_img, TextureOptions::LINEAR);
        return (Some(tex), Some(Arc::new(rgba_img)));
    }
    (None, None)
}

impl eframe::App for MusializerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Continuous repaint for 60+ FPS fluid rendering
        ctx.request_repaint();

        let now = Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32().min(0.1);
        self.last_frame_time = now;

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                log::info!("Dropped files event received: {:?}", i.raw.dropped_files);
            }
            if !i.raw.hovered_files.is_empty() {
                log::debug!("Hovered files event received: {:?}", i.raw.hovered_files);
            }
        });

        let mut on_export_click = false;
        let mut on_load_center_image = false;
        let mut on_open_file_click = false;

        let is_exporting = self.exporter.is_exporting();
        let export_prog = self.exporter.get_progress();

        // Bottom Controls Panel
        TopBottomPanel::bottom("bottom_controls_panel")
            .min_height(90.0)
            .show(ctx, |ui| {
                if let Ok(player) = &mut self.player {
                    TransportControls::show(
                        ui,
                        player,
                        &mut self.mode,
                        &mut self.theme,
                        &mut self.visual_gain,
                        &mut self.circle_center_display,
                        &mut on_load_center_image,
                        &mut on_open_file_click,
                        &mut on_export_click,
                        is_exporting,
                        export_prog,
                    );
                } else if let Err(err) = &self.player {
                    ui.colored_label(
                        Color32::from_rgb(255, 100, 100),
                        format!("Audio sink error: {}", err),
                    );
                }
            });

        // Main Visualizer Canvas Area
        CentralPanel::default().show(ctx, |ui| {
            let is_empty = match &self.player {
                Ok(p) => p.track().is_none(),
                Err(_) => true,
            };

            // Drag and drop handler (Desktop & Web)
            if let Some(dropped_item) =
                DragDropOverlay::check_and_render(ui, is_empty, &mut on_open_file_click)
            {
                match dropped_item {
                    DroppedItem::AudioPath(path) => {
                        self.load_audio_file(path);
                    }
                    DroppedItem::AudioBytes { name, bytes } => {
                        self.load_audio_from_bytes(bytes, name.as_deref());
                    }
                    DroppedItem::ImagePath(path) => {
                        self.load_custom_cover_image(ctx, path);
                    }
                    DroppedItem::ImageBytes(bytes) => {
                        self.load_custom_cover_image_from_bytes(ctx, &bytes);
                    }
                }
            }

            let mut current_time = 0.0;
            let mut total_time = 0.0;
            let mut track_title: Option<String> = None;

            // Audio DSP computation
            if let Ok(player) = &self.player {
                if let Some(track) = player.track() {
                    current_time = player.current_time_seconds() as f64;
                    total_time = player.duration_seconds() as f64;
                    track_title = Some(track.title.clone());

                    let current_frame = player.current_frame();
                    let pcm_chunk =
                        AudioSync::extract_pcm_window(&track.samples, current_frame, self.fft_size);

                    self.cached_wave = pcm_chunk.clone();

                    if player.is_playing() {
                        let mags = self.fft_processor.process(&pcm_chunk);
                        let raw_bands = self.bands_mapper.aggregate(&mags, self.visual_gain);
                        self.smoother.update(&raw_bands, dt);
                    } else {
                        let zeros = vec![0.0f32; self.num_bands];
                        self.smoother.update(&zeros, dt);
                    }
                }
            }

            let image_anim_prog = if let Some(start) = self.image_loading_timer {
                let elapsed = start.elapsed().as_secs_f32();
                if elapsed < 0.6 {
                    Some((elapsed / 0.6).clamp(0.0, 1.0))
                } else {
                    self.image_loading_timer = None;
                    None
                }
            } else {
                None
            };

            let available_rect = ui.available_rect_before_wrap();
            if available_rect.width() > 10.0 && available_rect.height() > 10.0 && !is_empty {
                VisualizerWidget::show(
                    ui.painter(),
                    available_rect,
                    self.mode,
                    self.theme,
                    self.smoother.values(),
                    self.smoother.peaks(),
                    &self.cached_wave,
                    self.circle_center_display,
                    self.circle_center_texture.as_ref(),
                    current_time,
                    total_time,
                    track_title.as_deref(),
                    image_anim_prog,
                );
            }

            if let Some(err) = &self.error_msg {
                ui.add_space(8.0);
                ui.colored_label(Color32::from_rgb(255, 90, 90), format!("⚠️ {}", err));
            }
        });

        // Trigger audio file picker
        if on_open_file_click {
            if let Some(audio_path) = rfd::FileDialog::new()
                .add_filter(
                    "Audio Files",
                    &["mp3", "wav", "flac", "ogg", "aac", "m4a", "aiff"],
                )
                .pick_file()
            {
                self.load_audio_file(audio_path);
            }
        }

        // Trigger custom center cover image picker
        if on_load_center_image {
            if let Some(img_path) = rfd::FileDialog::new()
                .add_filter(
                    "Image Files (PNG, JPG, WebP, BMP, GIF, ICO, TIFF)",
                    &[
                        "png", "jpg", "jpeg", "webp", "bmp", "gif", "ico", "tiff", "tif",
                    ],
                )
                .add_filter("All Files", &["*"])
                .pick_file()
            {
                self.load_custom_cover_image(ctx, img_path);
            }
        }

        // Open Export / Screen Record Modal
        if on_export_click {
            self.show_export_modal = true;
        }

        // Modal Dialog
        if self.show_export_modal {
            let mut is_open = true;
            let mut close_modal = false;

            Window::new("🎬 Export Video")
                .open(&mut is_open)
                .resizable(false)
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.set_min_width(380.0);

                    let is_exporting = self.exporter.is_exporting();

                    if is_exporting {
                        let prog = self.exporter.get_progress();
                        let status = self.exporter.get_status();

                        ui.add_space(8.0);
                        ui.label(RichText::new(&status).strong());
                        ui.add_space(6.0);
                        ui.add(ProgressBar::new(prog).show_percentage());
                        ui.add_space(12.0);

                        if ui
                            .button(
                                RichText::new("Cancel Export")
                                    .color(Color32::from_rgb(255, 100, 100)),
                            )
                            .clicked()
                        {
                            self.exporter.cancel();
                        }
                    } else {
                        ui.add_space(4.0);
                        ui.label("Render full track to high-definition MP4 video:");
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            ui.label("Resolution:");
                            egui::ComboBox::from_id_source("export_res_combo")
                                .selected_text(format!(
                                    "{}x{}",
                                    self.export_width, self.export_height
                                ))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.export_width,
                                        1920,
                                        "1920x1080 (1080p FHD)",
                                    );
                                    if self.export_width == 1920 {
                                        self.export_height = 1080;
                                    }
                                    ui.selectable_value(
                                        &mut self.export_width,
                                        1280,
                                        "1280x720 (720p HD)",
                                    );
                                    if self.export_width == 1280 {
                                        self.export_height = 720;
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Framerate:");
                            egui::ComboBox::from_id_source("export_fps_combo")
                                .selected_text(format!("{} FPS", self.export_fps))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.export_fps,
                                        60,
                                        "60 FPS (Ultra Smooth)",
                                    );
                                    ui.selectable_value(
                                        &mut self.export_fps,
                                        30,
                                        "30 FPS (Standard)",
                                    );
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Output:");
                            ui.text_edit_singleline(&mut self.export_output_path);
                            if ui.button("📂 Browse...").clicked() {
                                if let Some(save_path) = rfd::FileDialog::new()
                                    .set_file_name("musializer_render.mp4")
                                    .add_filter("MP4 Video", &["mp4"])
                                    .save_file()
                                {
                                    self.export_output_path =
                                        save_path.to_string_lossy().to_string();
                                }
                            }
                        });

                        ui.add_space(12.0);

                        ui.horizontal(|ui| {
                            if ui
                                .button(
                                    RichText::new("🚀 Start Export")
                                        .color(Color32::from_rgb(0, 240, 255))
                                        .strong(),
                                )
                                .clicked()
                            {
                                if let Ok(player) = &self.player {
                                    if let Some(track) = player.track() {
                                        let config = ExportConfig {
                                            width: self.export_width,
                                            height: self.export_height,
                                            fps: self.export_fps,
                                            output_path: PathBuf::from(&self.export_output_path),
                                            mode: self.mode,
                                            theme: self.theme,
                                            num_bands: self.num_bands,
                                            center_display: self.circle_center_display,
                                            center_image: self.raw_cover_image.clone(),
                                        };

                                        let _ =
                                            self.exporter.start_export((**track).clone(), config);
                                    }
                                }
                            }

                            if ui.button("Close").clicked() {
                                close_modal = true;
                            }
                        });
                    }
                });

            self.show_export_modal = is_open && !close_modal;
        }
    }
}
