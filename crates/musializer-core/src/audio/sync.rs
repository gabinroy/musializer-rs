use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Thread-safe synchronization primitive for coordinating audio playback position with the visualizer.
pub struct AudioSync {
    /// Playback head measured in stereo sample pairs (frames). 0 <= frame < total_frames.
    current_frame: Arc<AtomicUsize>,
    /// Whether playback is currently active.
    is_playing: Arc<AtomicBool>,
}

impl AudioSync {
    pub fn new() -> Self {
        Self {
            current_frame: Arc::new(AtomicUsize::new(0)),
            is_playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn frame_handle(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.current_frame)
    }

    pub fn playing_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.is_playing)
    }

    pub fn get_current_frame(&self) -> usize {
        self.current_frame.load(Ordering::Relaxed)
    }

    pub fn set_current_frame(&self, frame: usize) {
        self.current_frame.store(frame, Ordering::Release);
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    pub fn set_playing(&self, playing: bool) {
        self.is_playing.store(playing, Ordering::Release);
    }

    /// Extracts a temporal window of mono PCM samples centered or trailing the current playback position.
    /// Returns a slice of normalized `f32` samples suitable for FFT windowing.
    pub fn extract_pcm_window(
        samples: &[f32],
        current_frame: usize,
        window_size: usize,
    ) -> Vec<f32> {
        let mut window = vec![0.0f32; window_size];
        if samples.is_empty() {
            return window;
        }

        let total_frames = samples.len() / 2;
        if current_frame >= total_frames {
            return window;
        }

        // We want the most recent `window_size` samples ending around `current_frame`
        let start_frame = if current_frame >= window_size {
            current_frame - window_size
        } else {
            0
        };

        let num_frames = (current_frame - start_frame).min(window_size);
        let dest_offset = window_size - num_frames;

        for i in 0..num_frames {
            let sample_idx = (start_frame + i) * 2;
            if sample_idx + 1 < samples.len() {
                // Average left and right channels to obtain mono signal for FFT
                let mono = (samples[sample_idx] + samples[sample_idx + 1]) * 0.5;
                window[dest_offset + i] = mono;
            }
        }

        window
    }
}

impl Default for AudioSync {
    fn default() -> Self {
        Self::new()
    }
}
