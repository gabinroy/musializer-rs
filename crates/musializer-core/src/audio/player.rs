//! # CPAL Hardware Audio Streamer
//!
//! Provides [`AudioPlayer`] which creates and manages real-time CPAL output streams
//! (ALSA, WASAPI, CoreAudio, AAudio). It coordinates hardware buffer callbacks with
//! the synchronized playhead position stored in [`AudioSync`].

use crate::audio::decoder::AudioTrack;
use crate::audio::sync::AudioSync;
use cpal::Stream;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Hardware audio output player managing the active CPAL stream.
pub struct AudioPlayer {
    _stream: Option<Stream>,
    track: Option<Arc<AudioTrack>>,
    sync: AudioSync,
    volume: Arc<Mutex<f32>>,
    device_sample_rate: u32,
}

impl AudioPlayer {
    /// Creates a new `AudioPlayer` instance bound to the system default audio output device.
    ///
    /// # Errors
    /// Returns `Err(String)` if audio host enumeration fails.
    pub fn new() -> Result<Self, String> {
        let device_sample_rate = if let Some(device) = cpal::default_host().default_output_device() {
            device.default_output_config().map(|c| c.sample_rate()).unwrap_or(48000)
        } else {
            48000
        };

        Ok(Self {
            _stream: None,
            track: None,
            sync: AudioSync::new(),
            volume: Arc::new(Mutex::new(1.0)),
            device_sample_rate,
        })
    }

    /// Loads an [`AudioTrack`], binds the hardware audio stream, and resets playback position.
    ///
    /// # Arguments
    /// * `track` - The decoded audio track to stream.
    ///
    /// # Errors
    /// Returns `Err(String)` if no output device exists or stream configuration fails.
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
                    write_audio_data_f32(
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
            cpal::SampleFormat::I16 => {
                let track_for_stream_i16 = Arc::clone(&track_arc);
                let frame_atomic_i16 = self.sync.frame_handle();
                let playing_atomic_i16 = self.sync.playing_handle();
                let volume_arc_i16 = Arc::clone(&self.volume);
                device.build_output_stream(
                    config,
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        write_audio_data_i16(
                            data,
                            channels,
                            &track_for_stream_i16,
                            &frame_atomic_i16,
                            &playing_atomic_i16,
                            &volume_arc_i16,
                            device_sr,
                            track_sr,
                        );
                    },
                    err_fn,
                    None,
                )
            }
            cpal::SampleFormat::U16 => {
                let track_for_stream_u16 = Arc::clone(&track_arc);
                let frame_atomic_u16 = self.sync.frame_handle();
                let playing_atomic_u16 = self.sync.playing_handle();
                let volume_arc_u16 = Arc::clone(&self.volume);
                device.build_output_stream(
                    config,
                    move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                        write_audio_data_u16(
                            data,
                            channels,
                            &track_for_stream_u16,
                            &frame_atomic_u16,
                            &playing_atomic_u16,
                            &volume_arc_u16,
                            device_sr,
                            track_sr,
                        );
                    },
                    err_fn,
                    None,
                )
            }
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

    /// Resumes playback of the current track.
    pub fn play(&self) {
        if self.track.is_some() {
            self.sync.set_playing(true);
        }
    }

    /// Pauses playback.
    pub fn pause(&self) {
        self.sync.set_playing(false);
    }

    /// Toggles play/pause state.
    pub fn toggle_play_pause(&self) {
        let is_playing = self.sync.is_playing();
        self.sync.set_playing(!is_playing);
    }

    /// Stops playback and resets playhead to beginning.
    pub fn stop(&mut self) {
        self.sync.set_playing(false);
        self.sync.set_current_frame(0);
        self._stream = None;
    }

    /// Seeks playback to the specified time offset in seconds.
    pub fn seek_seconds(&self, seconds: f32) {
        if let Some(track) = &self.track {
            let target_frame = (seconds.max(0.0) * track.sample_rate as f32) as usize;
            let total_frames = track.samples.len() / 2;
            self.sync.set_current_frame(target_frame.min(total_frames));
        }
    }

    /// Sets the volume multiplier (clamped between 0.0 and 2.0).
    pub fn set_volume(&self, vol: f32) {
        if let Ok(mut v) = self.volume.lock() {
            *v = vol.clamp(0.0, 2.0);
        }
    }

    /// Returns the current volume level.
    pub fn get_volume(&self) -> f32 {
        self.volume.lock().map(|v| *v).unwrap_or(1.0)
    }

    /// Returns whether playback is currently running.
    pub fn is_playing(&self) -> bool {
        self.sync.is_playing()
    }

    /// Returns the current frame index in the audio track.
    pub fn current_frame(&self) -> usize {
        self.sync.get_current_frame()
    }

    /// Returns the current playback position in seconds.
    pub fn current_time_seconds(&self) -> f32 {
        if let Some(track) = &self.track {
            let frame = self.sync.get_current_frame();
            frame as f32 / track.sample_rate as f32
        } else {
            0.0
        }
    }

    /// Returns the total duration of the track in seconds.
    pub fn duration_seconds(&self) -> f32 {
        self.track
            .as_ref()
            .map(|t| t.duration_seconds)
            .unwrap_or(0.0)
    }

    /// Returns a reference to the active [`AudioTrack`], if loaded.
    pub fn track(&self) -> Option<&Arc<AudioTrack>> {
        self.track.as_ref()
    }

    /// Returns a reference to the underlying [`AudioSync`] state.
    #[allow(dead_code)]
    pub fn sync(&self) -> &AudioSync {
        &self.sync
    }
}

/// Helper callback filling f32 PCM audio output buffers for CPAL.
fn write_audio_data_f32(
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
    let initial_frame = frame_atomic.load(Ordering::Relaxed);
    let sample_ratio = track_sr / device_sr;
    let mut frame_accum = initial_frame as f64;

    for frame_chunk in output.chunks_mut(device_channels) {
        let current_frame = frame_accum as usize;
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

        frame_accum += sample_ratio;
    }

    frame_atomic.store(frame_accum as usize, Ordering::Release);
}

/// Helper callback filling i16 PCM audio output buffers for CPAL.
fn write_audio_data_i16(
    output: &mut [i16],
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
        output.fill(0);
        return;
    }

    let vol = volume_arc.lock().map(|v| *v).unwrap_or(1.0);
    let total_frames = track.samples.len() / 2;
    let initial_frame = frame_atomic.load(Ordering::Relaxed);
    let sample_ratio = track_sr / device_sr;
    let mut frame_accum = initial_frame as f64;

    for frame_chunk in output.chunks_mut(device_channels) {
        let current_frame = frame_accum as usize;
        if current_frame >= total_frames {
            playing_atomic.store(false, Ordering::Release);
            for s in frame_chunk.iter_mut() {
                *s = 0;
            }
            continue;
        }

        let track_sample_idx = current_frame * 2;
        let left = (track.samples.get(track_sample_idx).copied().unwrap_or(0.0) * vol)
            .clamp(-1.0, 1.0)
            * 32767.0;
        let right = (track.samples.get(track_sample_idx + 1).copied().unwrap_or(0.0) * vol)
            .clamp(-1.0, 1.0)
            * 32767.0;

        if device_channels == 1 {
            frame_chunk[0] = ((left + right) * 0.5) as i16;
        } else if device_channels >= 2 {
            frame_chunk[0] = left as i16;
            frame_chunk[1] = right as i16;
            for s in &mut frame_chunk[2..] {
                *s = 0;
            }
        }

        frame_accum += sample_ratio;
    }

    frame_atomic.store(frame_accum as usize, Ordering::Release);
}

/// Helper callback filling u16 PCM audio output buffers for CPAL.
fn write_audio_data_u16(
    output: &mut [u16],
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
        output.fill(32768);
        return;
    }

    let vol = volume_arc.lock().map(|v| *v).unwrap_or(1.0);
    let total_frames = track.samples.len() / 2;
    let initial_frame = frame_atomic.load(Ordering::Relaxed);
    let sample_ratio = track_sr / device_sr;
    let mut frame_accum = initial_frame as f64;

    for frame_chunk in output.chunks_mut(device_channels) {
        let current_frame = frame_accum as usize;
        if current_frame >= total_frames {
            playing_atomic.store(false, Ordering::Release);
            for s in frame_chunk.iter_mut() {
                *s = 32768;
            }
            continue;
        }

        let track_sample_idx = current_frame * 2;
        let left = ((track.samples.get(track_sample_idx).copied().unwrap_or(0.0) * vol)
            .clamp(-1.0, 1.0)
            + 1.0)
            * 32767.5;
        let right = ((track.samples.get(track_sample_idx + 1).copied().unwrap_or(0.0) * vol)
            .clamp(-1.0, 1.0)
            + 1.0)
            * 32767.5;

        if device_channels == 1 {
            frame_chunk[0] = ((left + right) * 0.5) as u16;
        } else if device_channels >= 2 {
            frame_chunk[0] = left as u16;
            frame_chunk[1] = right as u16;
            for s in &mut frame_chunk[2..] {
                *s = 32768;
            }
        }

        frame_accum += sample_ratio;
    }

    frame_atomic.store(frame_accum as usize, Ordering::Release);
}
