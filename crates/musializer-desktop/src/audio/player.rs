use crate::audio::decoder::AudioTrack;
use crate::audio::sync::AudioSync;
use cpal::Stream;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    _stream: Option<Stream>,
    track: Option<Arc<AudioTrack>>,
    sync: AudioSync,
    volume: Arc<Mutex<f32>>,
    device_sample_rate: u32,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| "No default audio output device found".to_string())?;

        let default_config = device
            .default_output_config()
            .map_err(|e| format!("Failed to query default output config: {}", e))?;

        let device_sample_rate = default_config.sample_rate();

        Ok(Self {
            _stream: None,
            track: None,
            sync: AudioSync::new(),
            volume: Arc::new(Mutex::new(1.0)),
            device_sample_rate,
        })
    }

    pub fn load_track(&mut self, track: AudioTrack) -> Result<(), String> {
        self.stop();

        let track_arc = Arc::new(track);
        self.track = Some(Arc::clone(&track_arc));
        self.sync.set_current_frame(0);

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| "No default audio output device found".to_string())?;

        let supported_config = device
            .default_output_config()
            .map_err(|e| format!("Failed to get output config: {}", e))?;

        let sample_format = supported_config.sample_format();
        let config: cpal::StreamConfig = supported_config.into();
        self.device_sample_rate = config.sample_rate;

        let frame_atomic = self.sync.frame_handle();
        let playing_atomic = self.sync.playing_handle();
        let volume_arc = Arc::clone(&self.volume);
        let track_for_stream = Arc::clone(&track_arc);
        let device_sr = self.device_sample_rate as f64;
        let track_sr = track_arc.sample_rate as f64;
        let channels = config.channels as usize;

        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_output_stream(
                config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    write_audio_data(
                        data,
                        channels,
                        &track_for_stream,
                        &frame_atomic,
                        &playing_atomic,
                        &volume_arc,
                        device_sr,
                        track_sr,
                    );
                },
                err_fn,
                None,
            ),
            _ => {
                return Err(format!(
                    "Unsupported output sample format: {:?}",
                    sample_format
                ));
            }
        }
        .map_err(|e| format!("Failed to build CPAL output stream: {}", e))?;

        stream
            .play()
            .map_err(|e| format!("Failed to start audio stream: {}", e))?;

        self._stream = Some(stream);
        Ok(())
    }

    pub fn play(&self) {
        if self.track.is_some() {
            self.sync.set_playing(true);
        }
    }

    pub fn pause(&self) {
        self.sync.set_playing(false);
    }

    pub fn toggle_play_pause(&self) {
        let is_playing = self.sync.is_playing();
        self.sync.set_playing(!is_playing);
    }

    pub fn stop(&mut self) {
        self.sync.set_playing(false);
        self.sync.set_current_frame(0);
        self._stream = None;
    }

    pub fn seek_seconds(&self, seconds: f32) {
        if let Some(track) = &self.track {
            let target_frame = (seconds.max(0.0) * track.sample_rate as f32) as usize;
            let total_frames = track.samples.len() / 2;
            self.sync.set_current_frame(target_frame.min(total_frames));
        }
    }

    pub fn set_volume(&self, vol: f32) {
        if let Ok(mut v) = self.volume.lock() {
            *v = vol.clamp(0.0, 2.0);
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume.lock().map(|v| *v).unwrap_or(1.0)
    }

    pub fn is_playing(&self) -> bool {
        self.sync.is_playing()
    }

    pub fn current_frame(&self) -> usize {
        self.sync.get_current_frame()
    }

    pub fn current_time_seconds(&self) -> f32 {
        if let Some(track) = &self.track {
            let frame = self.sync.get_current_frame();
            frame as f32 / track.sample_rate as f32
        } else {
            0.0
        }
    }

    pub fn duration_seconds(&self) -> f32 {
        self.track
            .as_ref()
            .map(|t| t.duration_seconds)
            .unwrap_or(0.0)
    }

    pub fn track(&self) -> Option<&Arc<AudioTrack>> {
        self.track.as_ref()
    }

    #[allow(dead_code)]
    pub fn sync(&self) -> &AudioSync {
        &self.sync
    }
}

fn write_audio_data(
    output: &mut [f32],
    device_channels: usize,
    track: &AudioTrack,
    frame_atomic: &AtomicUsize,
    playing_atomic: &AtomicBool,
    volume_arc: &Arc<Mutex<f32>>,
    device_sr: f64,
    track_sr: f64,
) {
    let is_playing = playing_atomic.load(Ordering::Relaxed);
    if !is_playing {
        output.fill(0.0);
        return;
    }

    let vol = volume_arc.lock().map(|v| *v).unwrap_or(1.0);
    let total_frames = track.samples.len() / 2;
    let mut current_frame = frame_atomic.load(Ordering::Relaxed);
    let sample_ratio = track_sr / device_sr;

    for frame_chunk in output.chunks_mut(device_channels) {
        if current_frame >= total_frames {
            playing_atomic.store(false, Ordering::Release);
            for s in frame_chunk.iter_mut() {
                *s = 0.0;
            }
            continue;
        }

        let track_sample_idx = current_frame * 2;
        let left = track.samples.get(track_sample_idx).copied().unwrap_or(0.0) * vol;
        let right = track
            .samples
            .get(track_sample_idx + 1)
            .copied()
            .unwrap_or(0.0)
            * vol;

        if device_channels == 1 {
            frame_chunk[0] = (left + right) * 0.5;
        } else if device_channels >= 2 {
            frame_chunk[0] = left;
            frame_chunk[1] = right;
            for s in &mut frame_chunk[2..] {
                *s = 0.0;
            }
        }

        if sample_ratio == 1.0 {
            current_frame += 1;
        } else {
            current_frame = (current_frame as f64 + sample_ratio) as usize;
        }
    }

    frame_atomic.store(current_frame, Ordering::Release);
}
