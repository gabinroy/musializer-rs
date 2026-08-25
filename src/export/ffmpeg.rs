use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crate::audio::AudioTrack;
use crate::dsp::{EmaSmoother, FftProcessor, FrequencyBands};
use crate::export::renderer::OffscreenRasterizer;
use crate::export::stepper::OfflineStepper;
use crate::ui::theme::ColorTheme;
use crate::ui::visualizer::VisualizerMode;

#[derive(Clone, Debug)]
pub struct ExportConfig {
    pub output_path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub mode: VisualizerMode,
    pub theme: ColorTheme,
    pub num_bands: usize,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            output_path: PathBuf::from("visualizer_output.mp4"),
            width: 1920,
            height: 1080,
            fps: 60,
            mode: VisualizerMode::SpectrumBars,
            theme: ColorTheme::CyberNeon,
            num_bands: 80,
        }
    }
}

pub struct VideoExporter {
    is_exporting: Arc<AtomicBool>,
    progress: Arc<std::sync::Mutex<f32>>,
    status_msg: Arc<std::sync::Mutex<String>>,
}

impl VideoExporter {
    pub fn new() -> Self {
        Self {
            is_exporting: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(std::sync::Mutex::new(0.0)),
            status_msg: Arc::new(std::sync::Mutex::new(String::new())),
        }
    }

    pub fn is_exporting(&self) -> bool {
        self.is_exporting.load(Ordering::Relaxed)
    }

    pub fn get_progress(&self) -> f32 {
        self.progress.lock().map(|p| *p).unwrap_or(0.0)
    }

    pub fn get_status(&self) -> String {
        self.status_msg
            .lock()
            .map(|s| s.clone())
            .unwrap_or_default()
    }

    pub fn cancel(&self) {
        self.is_exporting.store(false, Ordering::Release);
    }

    /// Checks if ffmpeg executable is available on PATH
    pub fn is_ffmpeg_available() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Spawns offline deterministic rendering and pipes raw frames to ffmpeg
    pub fn start_export(&self, track: AudioTrack, config: ExportConfig) -> Result<(), String> {
        if self.is_exporting() {
            return Err("An export is already in progress.".to_string());
        }

        if !Self::is_ffmpeg_available() {
            return Err(
                "FFmpeg was not found on your system. Please install FFmpeg to export videos."
                    .to_string(),
            );
        }

        self.is_exporting.store(true, Ordering::Release);
        *self.progress.lock().unwrap() = 0.0;
        *self.status_msg.lock().unwrap() = "Initializing video export...".to_string();

        let is_exporting_flag = Arc::clone(&self.is_exporting);
        let progress_arc = Arc::clone(&self.progress);
        let status_arc = Arc::clone(&self.status_msg);

        thread::spawn(move || {
            let res = run_export_thread(
                track,
                config,
                &is_exporting_flag,
                &progress_arc,
                &status_arc,
            );

            is_exporting_flag.store(false, Ordering::Release);

            if let Err(e) = res {
                log::error!("Video export failed: {}", e);
                *status_arc.lock().unwrap() = format!("Export failed: {}", e);
            } else {
                *progress_arc.lock().unwrap() = 1.0;
                *status_arc.lock().unwrap() = "Export completed successfully! 🎉".to_string();
            }
        });

        Ok(())
    }
}

impl Default for VideoExporter {
    fn default() -> Self {
        Self::new()
    }
}

fn run_export_thread(
    track: AudioTrack,
    config: ExportConfig,
    is_exporting: &AtomicBool,
    progress: &std::sync::Mutex<f32>,
    status: &std::sync::Mutex<String>,
) -> Result<(), String> {
    let mut stepper = OfflineStepper::new(track.clone(), config.fps);
    let mut rasterizer = OffscreenRasterizer::new(config.width, config.height);

    let fft_size = 2048;
    let mut fft = FftProcessor::new(fft_size);
    let bands_mapper = FrequencyBands::new(config.num_bands, fft_size, track.sample_rate);
    let mut smoother = EmaSmoother::new(config.num_bands, 0.85, 0.15);
    let dt = 1.0 / config.fps as f32;

    *status.lock().unwrap() = format!(
        "Spawning FFmpeg encoder ({}x{} @ {}fps)...",
        config.width, config.height, config.fps
    );

    // Prepare FFmpeg command with raw RGBA stdin pipe and original audio input
    let mut child = Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "rawvideo",
            "-vcodec",
            "rawvideo",
            "-s",
            &format!("{}x{}", config.width, config.height),
            "-pix_fmt",
            "rgba",
            "-r",
            &config.fps.to_string(),
            "-i",
            "-",
            "-i",
            track
                .file_path
                .as_ref()
                .and_then(|p| p.to_str())
                .ok_or("Track must have a file path for ffmpeg export")?,
            "-c:v",
            "libx264",
            "-preset",
            "fast",
            "-crf",
            "18",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "aac",
            "-b:a",
            "192k",
            "-shortest",
            config.output_path.to_str().ok_or("Invalid output path")?,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn FFmpeg process: {}", e))?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Failed to capture FFmpeg stdin pipe".to_string())?;

    let total_frames = stepper.total_frames();

    while let Some((frame_idx, pcm_window, prog)) = stepper.next_step(fft_size) {
        if !is_exporting.load(Ordering::Relaxed) {
            let _ = child.kill();
            return Err("Export cancelled by user".to_string());
        }

        // Run DSP on deterministic frame window
        let magnitudes = fft.process(&pcm_window);
        let raw_bands = bands_mapper.aggregate(&magnitudes);
        smoother.update(&raw_bands, dt);

        // Rasterize frame
        let frame_bytes = rasterizer.render_frame(
            config.mode,
            config.theme,
            smoother.values(),
            smoother.peaks(),
            &pcm_window,
        );

        // Write raw RGBA frame to FFmpeg stdin
        if let Err(e) = stdin.write_all(frame_bytes) {
            return Err(format!("Failed writing frame to FFmpeg pipe: {}", e));
        }

        if frame_idx % 30 == 0 {
            *progress.lock().unwrap() = prog;
            *status.lock().unwrap() = format!(
                "Rendering frame {} of {} ({:.1}%)...",
                frame_idx,
                total_frames,
                prog * 100.0
            );
        }
    }

    // Flush and close stdin pipe so FFmpeg can finalize container
    drop(stdin);

    *status.lock().unwrap() = "Finalizing MP4 container...".to_string();
    let status_code = child
        .wait()
        .map_err(|e| format!("Error waiting on FFmpeg: {}", e))?;

    if !status_code.success() {
        return Err(format!(
            "FFmpeg exited with error code: {:?}",
            status_code.code()
        ));
    }

    Ok(())
}
