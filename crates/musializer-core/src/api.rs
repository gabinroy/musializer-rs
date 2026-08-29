use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::engine::AudioVisualizerEngine;

lazy_static! {
    static ref ENGINE: Mutex<Option<AudioVisualizerEngine>> = Mutex::new(None);
}

#[derive(Debug, Clone)]
pub struct MobileTrackInfo {
    pub title: String,
    pub duration_seconds: f32,
    pub sample_rate: u32,
    pub channels: u16,
}

pub fn init_engine(fft_size: usize, num_bands: usize) -> Result<(), String> {
    let engine = AudioVisualizerEngine::new(fft_size, num_bands)?;
    let mut lock = ENGINE.lock().map_err(|e| format!("Mutex lock error: {:?}", e))?;
    *lock = Some(engine);
    Ok(())
}

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

pub fn play() {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.play();
        }
    }
}

pub fn pause() {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.pause();
        }
    }
}

pub fn toggle_play_pause() {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.toggle_play_pause();
        }
    }
}

pub fn seek_seconds(seconds: f32) {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.seek_seconds(seconds);
        }
    }
}

pub fn set_volume(volume: f32) {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            engine.set_volume(volume);
        }
    }
}

pub fn get_volume() -> f32 {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.get_volume();
        }
    }
    1.0
}

pub fn is_playing() -> bool {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.is_playing();
        }
    }
    false
}

pub fn current_time() -> f32 {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.current_time();
        }
    }
    0.0
}

pub fn duration_seconds() -> f32 {
    if let Ok(lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_ref() {
            return engine.duration_seconds();
        }
    }
    0.0
}

pub fn set_gain_multiplier(gain: f32) {
    if let Ok(mut lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_mut() {
            engine.set_gain_multiplier(gain);
        }
    }
}

pub fn get_spectrum(dt: f32) -> Vec<f32> {
    if let Ok(mut lock) = ENGINE.lock() {
        if let Some(engine) = lock.as_mut() {
            return engine.update_and_get_spectrum(dt);
        }
    }
    Vec::new()
}

// Android JNI initializer for ndk-context / CPAL / AAudio
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_musializer_mobile_MainActivity_initAndroidContext(
    env: *mut jni::sys::JNIEnv,
    _class: jni::sys::jclass,
    context: jni::sys::jobject,
) {
    let mut vm: *mut jni::sys::JavaVM = std::ptr::null_mut();
    if unsafe { ((**env).v1_1.GetJavaVM)(env, &mut vm) } == 0 && !vm.is_null() {
        let global_context = unsafe { ((**env).v1_1.NewGlobalRef)(env, context) };
        unsafe {
            ndk_context::initialize_android_context(
                vm as *mut std::ffi::c_void,
                global_context as *mut std::ffi::c_void,
            );
        }
    }
}
