use std::path::Path;
use std::sync::Arc;
use crate::audio::decoder::AudioTrack;
use crate::audio::player::AudioPlayer;
use crate::audio::sync::AudioSync;
use crate::dsp::ema::EmaSmoother;
use crate::dsp::fft::FftProcessor;
use crate::dsp::frequency::FrequencyBands;

#[derive(Debug, Clone)]
pub struct AudioMetadata {
    pub title: String,
    pub duration_seconds: f32,
    pub sample_rate: u32,
    pub channels: u16,
}

pub struct AudioVisualizerEngine {
    player: AudioPlayer,
    track: Option<Arc<AudioTrack>>,
    fft: FftProcessor,
    bands: FrequencyBands,
    smoother: EmaSmoother,
    fft_size: usize,
    num_bands: usize,
    gain_multiplier: f32,
}

impl AudioVisualizerEngine {
    pub fn new(fft_size: usize, num_bands: usize) -> Result<Self, String> {
        let player = AudioPlayer::new()?;
        let fft = FftProcessor::new(fft_size);
        let bands = FrequencyBands::new(num_bands, fft_size, 44100);
        let smoother = EmaSmoother::new(num_bands, 0.85, 0.15);

        Ok(Self {
            player,
            track: None,
            fft,
            bands,
            smoother,
            fft_size,
            num_bands,
            gain_multiplier: 1.0,
        })
    }

    pub fn load_audio_file<P: AsRef<Path>>(&mut self, path: P) -> Result<AudioMetadata, String> {
        let track = AudioTrack::load_from_file(path)?;
        let metadata = AudioMetadata {
            title: track.title.clone(),
            duration_seconds: track.duration_seconds,
            sample_rate: track.sample_rate,
            channels: track.channels,
        };

        self.bands = FrequencyBands::new(self.num_bands, self.fft_size, track.sample_rate);
        self.player.load_track(track.clone())?;
        self.track = Some(Arc::new(track));
        self.smoother.reset();

        Ok(metadata)
    }

    pub fn load_audio_from_memory(&mut self, bytes: Vec<u8>, filename_hint: Option<&str>) -> Result<AudioMetadata, String> {
        let track = AudioTrack::load_from_memory(bytes, filename_hint)?;
        let metadata = AudioMetadata {
            title: track.title.clone(),
            duration_seconds: track.duration_seconds,
            sample_rate: track.sample_rate,
            channels: track.channels,
        };

        self.bands = FrequencyBands::new(self.num_bands, self.fft_size, track.sample_rate);
        self.player.load_track(track.clone())?;
        self.track = Some(Arc::new(track));
        self.smoother.reset();

        Ok(metadata)
    }

    pub fn play(&self) {
        self.player.play();
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn toggle_play_pause(&self) {
        self.player.toggle_play_pause();
    }

    pub fn seek_seconds(&self, seconds: f32) {
        self.player.seek_seconds(seconds);
    }

    pub fn set_volume(&self, vol: f32) {
        self.player.set_volume(vol);
    }

    pub fn get_volume(&self) -> f32 {
        self.player.get_volume()
    }

    pub fn is_playing(&self) -> bool {
        self.player.is_playing()
    }

    pub fn current_time(&self) -> f32 {
        self.player.current_time_seconds()
    }

    pub fn duration_seconds(&self) -> f32 {
        self.player.duration_seconds()
    }

    pub fn set_gain_multiplier(&mut self, gain: f32) {
        self.gain_multiplier = gain.max(0.1);
    }

    pub fn update_and_get_spectrum(&mut self, dt: f32) -> Vec<f32> {
        let Some(track) = &self.track else {
            return vec![0.0; self.num_bands];
        };

        let current_frame = self.player.current_frame();
        let pcm_window = AudioSync::extract_pcm_window(&track.samples, current_frame, self.fft_size);
        let raw_magnitudes = self.fft.process(&pcm_window);
        let band_magnitudes = self.bands.aggregate(&raw_magnitudes, self.gain_multiplier);
        self.smoother.update(&band_magnitudes, dt);
        self.smoother.values().to_vec()
    }
}
