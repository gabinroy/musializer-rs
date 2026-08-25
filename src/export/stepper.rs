use crate::audio::AudioTrack;

/// Deterministic offline stepper that yields exact sample windows for each 60 FPS video frame.
pub struct OfflineStepper {
    track: AudioTrack,
    #[allow(dead_code)]
    fps: u32,
    samples_per_frame: usize,
    current_frame_idx: usize,
    total_video_frames: usize,
}

impl OfflineStepper {
    pub fn new(track: AudioTrack, fps: u32) -> Self {
        let samples_per_frame = (track.sample_rate as f32 / fps as f32).round() as usize;
        let total_audio_frames = track.samples.len() / 2;
        let total_video_frames = ((total_audio_frames as f32 / samples_per_frame as f32).ceil() as usize).max(1);

        Self {
            track,
            fps,
            samples_per_frame,
            current_frame_idx: 0,
            total_video_frames,
        }
    }

    /// Advances one video frame (1/fps seconds) and returns the corresponding PCM window.
    pub fn next_step(&mut self, window_size: usize) -> Option<(usize, Vec<f32>, f32)> {
        if self.current_frame_idx >= self.total_video_frames {
            return None;
        }

        let audio_frame_pos = self.current_frame_idx * self.samples_per_frame;
        let pcm_window = crate::audio::sync::AudioSync::extract_pcm_window(
            &self.track.samples,
            audio_frame_pos,
            window_size,
        );

        let progress = self.current_frame_idx as f32 / self.total_video_frames as f32;
        let frame_num = self.current_frame_idx;

        self.current_frame_idx += 1;
        Some((frame_num, pcm_window, progress))
    }

    pub fn total_frames(&self) -> usize {
        self.total_video_frames
    }

    #[allow(dead_code)]
    pub fn fps(&self) -> u32 {
        self.fps
    }

    #[allow(dead_code)]
    pub fn track(&self) -> &AudioTrack {
        &self.track
    }
}
