//! # Flutter Rust Bridge (FRB) C-ABI Interface
//!
//! This module exposes thread-safe, global FFI functions called directly by the Flutter
//! mobile application via `flutter_rust_bridge`. It maintains a global [`AudioVisualizerEngine`]
//! instance protected by a mutex and provides functions for track loading, playback controls,
//! real-time FFT queries, and offline video rendering.

use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::audio::sync::AudioSync;
use crate::dsp::ema::EmaSmoother;
use crate::dsp::fft::FftProcessor;
use crate::dsp::frequency::FrequencyBands;
use crate::engine::AudioVisualizerEngine;

lazy_static! {
    /// Thread-safe global singleton holding the active audio visualizer engine.
    static ref ENGINE: Mutex<Option<AudioVisualizerEngine>> = Mutex::new(None);
}

/// Track metadata transferred to the Flutter mobile client over FFI.
#[derive(Debug, Clone)]
pub struct MobileTrackInfo {
    /// Track title or inferred filename.
    pub title: String,
    /// Total track length in seconds.
    pub duration_seconds: f32,
    /// Native sample rate in Hz (e.g. 44100 or 48000).
    pub sample_rate: u32,
    /// Audio channels count (2 for stereo).
    pub channels: u16,
}

/// Initializes the global audio engine singleton.
///
/// Must be invoked by the Flutter application prior to any playback or analysis calls.
///
/// # Arguments
/// * `fft_size` - FFT window size in samples (typically 2048).
/// * `num_bands` - Number of visual frequency bands (typically 64).
///
/// # Errors
/// Returns an error string if audio device acquisition fails.
pub fn init_engine(fft_size: usize, num_bands: usize) -> Result<(), String> {
    let engine = AudioVisualizerEngine::new(fft_size, num_bands)?;
    let mut lock = ENGINE.lock().map_err(|e| format!("Mutex lock error: {:?}", e))?;
    *lock = Some(engine);
    Ok(())
}

/// Decodes and loads an audio file from the mobile device filesystem.
///
/// # Arguments
/// * `path` - Full path to the audio file on the device.
///
/// # Errors
/// Returns an error string if the engine is uninitialized or decoding fails.
pub fn load_audio_file(path: String) -> Result<MobileTrackInfo, String> {
    let mut lock = ENGINE.lock().map_err(|e| format!("Mutex lock error: {:?}", e))?;
    let engine = lock.as_mut().ok_or_else(|| "Engine not initialized".to_string())?;
    
    let meta = engine.load_audio_file(path)?;
    Ok(MobileTrackInfo {
        title: meta.title,
        duration_seconds: meta.duration_seconds,
        sample_rate: meta.sample_rate,
        channels: meta.channels,
    })
}

/// Decodes and loads an audio track from an in-memory byte buffer (e.g. SAF / Document Picker).
///
/// # Arguments
/// * `bytes` - Raw byte contents of the audio file.
/// * `filename_hint` - Optional filename or extension to assist format identification.
///
/// # Errors
/// Returns an error string if decoding fails.
pub fn load_audio_bytes(bytes: Vec<u8>, filename_hint: Option<String>) -> Result<MobileTrackInfo, String> {
    let mut lock = ENGINE.lock().map_err(|e| format!("Mutex lock error: {:?}", e))?;
    let engine = lock.as_mut().ok_or_else(|| "Engine not initialized".to_string())?;
    
    let meta = engine.load_audio_from_memory(bytes, filename_hint.as_deref())?;
    Ok(MobileTrackInfo {
        title: meta.title,
        duration_seconds: meta.duration_seconds,
        sample_rate: meta.sample_rate,
        channels: meta.channels,
    })
}

/// Begins or resumes hardware audio playback.
pub fn play() {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.play();
        }
    }
}

/// Pauses audio playback.
pub fn pause() {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.pause();
        }
    }
}

/// Toggles between playback and paused states.
pub fn toggle_play_pause() {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.toggle_play_pause();
        }
    }
}

/// Seeks playback to the target timestamp in seconds.
pub fn seek_seconds(seconds: f32) {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.seek_seconds(seconds);
        }
    }
}

/// Sets the master volume level (0.0 to 1.0+).
pub fn set_volume(volume: f32) {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.set_volume(volume);
        }
    }
}

/// Returns the current playback volume level.
pub fn get_volume() -> f32 {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.get_volume();
        }
    }
    1.0
}

/// Returns `true` if playback is currently active.
pub fn is_playing() -> bool {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.is_playing();
        }
    }
    false
}

/// Returns current playback position in seconds.
pub fn current_time() -> f32 {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.current_time();
        }
    }
    0.0
}

/// Returns total duration of the current audio track in seconds.
pub fn duration_seconds() -> f32 {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.duration_seconds();
        }
    }
    0.0
}

/// Adjusts visual sensitivity gain for frequency bars.
pub fn set_gain_multiplier(gain: f32) {
    if let Ok(mut lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_mut() {
            engine.set_gain_multiplier(gain);
        }
    }
}

/// Retrieves the real-time smoothed frequency spectrum bars.
///
/// # Arguments
/// * `dt` - Elapsed delta time in seconds since the last frame render.
pub fn get_spectrum(dt: f32) -> Vec<f32> {
    if let Ok(mut lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_mut() {
            return engine.update_and_get_spectrum(dt);
        }
    }
    Vec::new()
}

/// Computes the exact deterministic FFT frequency spectrum for all video frames offline.
///
/// Designed for offline video rendering to guarantee frame-perfect video sync without
/// relying on real-time playback clock drift.
///
/// # Arguments
/// * `fps` - Target video frame rate (e.g. 30 or 60).
/// * `num_bands` - Number of frequency bands per frame.
/// * `gain_multiplier` - Visual sensitivity scaling factor.
///
/// # Returns
/// Flattened `Vec<f32>` containing `total_frames * num_bands` magnitude values.
pub fn get_offline_spectrum_frames(fps: u32, num_bands: usize, gain_multiplier: f32) -> Result<Vec<f32>, String> {
    let lock = ENGINE.lock().map_err(|e| format!("Engine lock error: {:?}", e))?;
    let engine = lock.as_ref().ok_or_else(|| "Engine not initialized".to_string())?;
    let track = engine.get_track().ok_or_else(|| "No track loaded".to_string())?;

    let fft_size = 2048;
    let mut fft = FftProcessor::new(fft_size);
    let bands_mapper = FrequencyBands::new(num_bands, fft_size, track.sample_rate);
    let mut smoother = EmaSmoother::new(num_bands, 0.85, 0.15);
    let dt = 1.0 / fps.max(1) as f32;

    let samples_per_frame = (track.sample_rate as f32 / fps as f32).round() as usize;
    let total_audio_frames = track.samples.len() / 2;
    let total_video_frames = ((total_audio_frames as f32 / samples_per_frame as f32).ceil() as usize).max(1);

    let mut result = Vec::with_capacity(total_video_frames * num_bands);

    for frame_idx in 0..total_video_frames {
        let audio_frame_pos = frame_idx * samples_per_frame;
        let pcm_window = AudioSync::extract_pcm_window(&track.samples, audio_frame_pos, fft_size);
        let magnitudes = fft.process(&pcm_window);
        let raw_bands = bands_mapper.aggregate(&magnitudes, gain_multiplier.max(0.1));
        smoother.update(&raw_bands, dt);
        result.extend_from_slice(smoother.values());
    }

    Ok(result)
}

/// Extracts 16-bit signed little-endian PCM audio bytes for offline video export (stereo, 44100Hz).
///
/// Used by the mobile video exporter to mux audio into the final MP4 container.
pub fn get_offline_audio_pcm() -> Result<Vec<u8>, String> {
    let lock = ENGINE.lock().map_err(|e| format!("Engine lock error: {:?}", e))?;
    let engine = lock.as_ref().ok_or_else(|| "Engine not initialized".to_string())?;
    let track = engine.get_track().ok_or_else(|| "No track loaded".to_string())?;
    
    // Convert stereo f32 samples to 16-bit PCM bytes
    let mut pcm_bytes = Vec::with_capacity(track.samples.len() * 2);
    for &sample in &track.samples {
        let clamped = sample.clamp(-1.0, 1.0);
        let s16 = (clamped * 32767.0) as i16;
        pcm_bytes.extend_from_slice(&s16.to_le_bytes());
    }
    Ok(pcm_bytes)
}

// Android JNI initializer for ndk-context / CPAL / AAudio
#[cfg(target_os = "android")]
static CONTEXT_INIT_ONCE: std::sync::Once = std::sync::Once::new();

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_musializer_mobile_MainActivity_initAndroidContext(
    env: *mut jni::sys::JNIEnv,
    _class: jni::sys::jclass,
    context: jni::sys::jobject,
) {
    CONTEXT_INIT_ONCE.call_once(|| {
        let mut vm: *mut jni::sys::JavaVM = std::ptr::null_mut();
        if unsafe { ((**env).v1_1.GetJavaVM)(env, &mut vm) } == 0 && !vm.is_null() {
            let global_context = unsafe { ((**env).v1_1.NewGlobalRef)(env, context) };
            if !global_context.is_null() {
                unsafe {
                    ndk_context::initialize_android_context(
                        vm as *mut std::ffi::c_void,
                        global_context as *mut std::ffi::c_void,
                    );
                }
            }
        }
    });
}
