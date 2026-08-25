use std::path::PathBuf;
use std::time::Instant;

use eframe::egui;
#[allow(unused_imports)]
use egui::{CentralPanel, Color32, ProgressBar, RichText, TopBottomPanel, Window};

use crate::audio::{AudioPlayer, AudioSync, AudioTrack};
use crate::dsp::{EmaSmoother, FftProcessor, FrequencyBands};
#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
use crate::export::{ExportConfig, VideoExporter};
use crate::ui::{
    ColorTheme, ControlPanel, DragDropOverlay, VisualizerMode, VisualizerPainter, apply_theme,
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

    last_frame_time: Instant,
    cached_wave: Vec<f32>,

    // Video Export State (Desktop)
    #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
    exporter: VideoExporter,
    show_export_modal: bool,
    #[allow(dead_code)]
    export_output_path: String,
    #[allow(dead_code)]
    export_fps: u32,
    #[allow(dead_code)]
    export_width: u32,
    #[allow(dead_code)]
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

        Self {
            player,
            fft_processor,
            bands_mapper,
            smoother,

            mode: VisualizerMode::SpectrumBars,
            theme: ColorTheme::CyberNeon,
            num_bands,
            fft_size,

            last_frame_time: Instant::now(),
            cached_wave: vec![0.0; fft_size],

            #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
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

impl eframe::App for MusializerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Continuous repaint for 60+ FPS fluid rendering
        ctx.request_repaint();

        let now = Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32().min(0.1);
        self.last_frame_time = now;

        let mut on_export_click = false;
        let mut on_open_file_click = false;

        // Bottom Controls Panel
        TopBottomPanel::bottom("bottom_controls_panel")
            .min_height(90.0)
            .show(ctx, |ui| {
                if let Ok(player) = &mut self.player {
                    ControlPanel::render(
                        ui,
                        player,
                        &mut self.mode,
                        &mut self.theme,
                        &mut on_export_click,
                        &mut on_open_file_click,
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

            // Drag and drop handler (Desktop/Web)
            if let Some(dropped_path) = DragDropOverlay::check_and_render(ui, is_empty) {
                self.load_audio_file(dropped_path);
            }

            // Audio DSP computation
            if let Ok(player) = &self.player {
                if let Some(track) = player.track() {
                    let current_frame = player.current_frame();
                    let pcm_chunk =
                        AudioSync::extract_pcm_window(&track.samples, current_frame, self.fft_size);

                    self.cached_wave = pcm_chunk.clone();

                    if player.is_playing() {
                        let mags = self.fft_processor.process(&pcm_chunk);
                        let raw_bands = self.bands_mapper.aggregate(&mags);
                        self.smoother.update(&raw_bands, dt);
                    } else {
                        let zeros = vec![0.0f32; self.num_bands];
                        self.smoother.update(&zeros, dt);
                    }
                }
            }

            let available_rect = ui.available_rect_before_wrap();
            if available_rect.width() > 10.0 && available_rect.height() > 10.0 && !is_empty {
                VisualizerPainter::paint(
                    ui.painter(),
                    available_rect,
                    self.mode,
                    self.theme,
                    self.smoother.values(),
                    self.smoother.peaks(),
                    &self.cached_wave,
                );
            }

            if let Some(err) = &self.error_msg {
                ui.add_space(8.0);
                ui.colored_label(Color32::from_rgb(255, 90, 90), format!("⚠️ {}", err));
            }
        });

        // Trigger file picker
        if on_open_file_click {
            #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
            {
                if let Some(file_path) = rfd::FileDialog::new()
                    .add_filter("Audio Files", &["mp3", "wav", "flac", "ogg", "aac", "m4a"])
                    .pick_file()
                {
                    self.load_audio_file(file_path);
                }
            }
        }

        // Open Export / Screen Record Modal
        if on_export_click {
            self.show_export_modal = true;
        }

        // Modal Dialog
        if self.show_export_modal {
            let mut open = self.show_export_modal;

            #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
            {
                Window::new("🎬 Export Video")
                    .open(&mut open)
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

                            if ui.button(RichText::new("Cancel Export").color(Color32::from_rgb(255, 100, 100))).clicked() {
                                self.exporter.cancel();
                            }
                        } else {
                            ui.add_space(4.0);
                            ui.label("Render full track to high-definition MP4 video:");
                            ui.add_space(8.0);

                            ui.horizontal(|ui| {
                                ui.label("Resolution:");
                                ui.selectable_value(&mut self.export_width, 1920, "1080p (1920x1080)");
                                ui.selectable_value(&mut self.export_width, 1280, "720p (1280x720)");
                                if self.export_width == 1920 {
                                    self.export_height = 1080;
                                } else {
                                    self.export_height = 720;
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Framerate:");
                                ui.selectable_value(&mut self.export_fps, 60, "60 FPS");
                                ui.selectable_value(&mut self.export_fps, 30, "30 FPS");
                            });

                            ui.horizontal(|ui| {
                                ui.label("Output filename:");
                                ui.text_edit_singleline(&mut self.export_output_path);
                            });

                            ui.add_space(12.0);

                            let ffmpeg_ok = VideoExporter::is_ffmpeg_available();
                            if !ffmpeg_ok {
                                ui.colored_label(
                                    Color32::from_rgb(255, 140, 60),
                                    "⚠️ FFmpeg not detected on PATH. Install FFmpeg to enable video export.",
                                );
                            }

                            ui.horizontal(|ui| {
                                let can_export = ffmpeg_ok && self.player.as_ref().map(|p| p.track().is_some()).unwrap_or(false);

                                if ui.add_enabled(can_export, egui::Button::new(RichText::new("🚀 Start Export").color(Color32::from_rgb(0, 240, 255)).strong())).clicked() {
                                    if let Ok(player) = &mut self.player {
                                        if let Some(track) = player.track() {
                                            player.pause();
                                            let track_clone = (**track).clone();
                                            let config = ExportConfig {
                                                output_path: PathBuf::from(&self.export_output_path),
                                                width: self.export_width,
                                                height: self.export_height,
                                                fps: self.export_fps,
                                                mode: self.mode,
                                                theme: self.theme,
                                                num_bands: self.num_bands,
                                            };

                                            if let Err(e) = self.exporter.start_export(track_clone, config) {
                                                self.error_msg = Some(e);
                                            }
                                        }
                                    }
                                }

                                if ui.button("Close").clicked() {
                                    self.show_export_modal = false;
                                }
                            });
                        }
                    });
            }

            #[cfg(any(target_os = "android", target_os = "ios", target_arch = "wasm32"))]
            {
                Window::new("🎥 Screen Recording Info")
                    .open(&mut open)
                    .resizable(false)
                    .collapsible(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.set_min_width(300.0);
                        ui.add_space(6.0);
                        ui.heading("High-Definition Video Recording");
                        ui.add_space(8.0);
                        ui.label("To record high-quality 60 FPS video on mobile / browser:");
                        ui.add_space(6.0);
                        ui.label("1. Use your device's built-in Screen Recorder (Control Center on iOS, Quick Settings on Android, or OS screen recorder).");
                        ui.label("2. Start audio playback in Musializer-RS.");
                        ui.label("3. Stop screen recording when done to save your MP4.");
                        ui.add_space(12.0);

                        if ui.button("Got it!").clicked() {
                            self.show_export_modal = false;
                        }
                    });
            }

            self.show_export_modal = open;
        }
    }
}
