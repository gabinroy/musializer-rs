//! # High-Level Visualizer Engine
//!
//! This module provides [`AudioVisualizerEngine`], which orchestrates the entire
//! lifecycle of audio playback and visualization: decoding files, driving audio
//! hardware sinks, running FFT calculations, binning frequency spectrums, and
//! applying smoothing algorithms.

use std::path::Path;
use std::sync::Arc;
use crate::audio::decoder::AudioTrack;
use crate::audio::player::AudioPlayer;
use crate::audio::sync::AudioSync;
use crate::dsp::ema::EmaSmoother;
use crate::dsp::fft::FftProcessor;
use crate::dsp::frequency::FrequencyBands;

/// Metadata describing a loaded audio track.
#[derive(Debug, Clone)]
pub struct AudioMetadata {
    /// Track title extracted from metadata or derived from filename.
    pub title: String,
    /// Total duration of the audio in seconds.
    pub duration_seconds: f32,
    /// Hardware sample rate in Hz (e.g., 44100 or 48000).
    pub sample_rate: u32,
    /// Number of audio channels (typically 2 for stereo).
    pub channels: u16,
}

/// The unified audio playback and real-time visualization engine.
///
/// Combines an [`AudioPlayer`] (hardware audio sink via CPAL), an [`FftProcessor`]
/// (SIMD FFT), a [`FrequencyBands`] aggregator (logarithmic spectrum scaling),
/// and an [`EmaSmoother`] (visual bar decay and peak hold).
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
    /// Creates a new visualizer engine instance.
    ///
    /// # Arguments
    /// * `fft_size` - Number of samples per FFT window (must be a power of 2, e.g., 2048).
    /// * `num_bands` - Number of visual frequency bands to output (e.g., 32, 64, or 128).
    ///
    /// # Errors
    /// Returns `Err(String)` if default audio output device initialization fails.
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

    /// Loads an audio file from a filesystem path and prepares playback and DSP pipelines.
    ///
    /// Supported formats: MP3, WAV, FLAC, OGG/Vorbis, and AAC.
    ///
    /// # Arguments
    /// * `path` - Filesystem path to the audio file.
    ///
    /// # Errors
    /// Returns `Err(String)` if the file does not exist, cannot be read, or codec decoding fails.
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

    /// Loads an in-memory encoded audio byte buffer and prepares playback and DSP pipelines.
    ///
    /// # Arguments
    /// * `bytes` - Encoded audio file bytes.
    /// * `filename_hint` - Optional filename or extension hint (e.g. `"song.mp3"`) to aid format detection.
    ///
    /// # Errors
    /// Returns `Err(String)` if the byte format is unrecognized or corrupted.
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

    /// Starts or resumes audio playback.
    pub fn play(&self) {
        self.player.play();
    }

    /// Pauses audio playback at the current playhead position.
    pub fn pause(&self) {
        self.player.pause();
    }

    /// Toggles between playing and paused states.
    pub fn toggle_play_pause(&self) {
        self.player.toggle_play_pause();
    }

    /// Seeks playback to the specified time offset in seconds.
    pub fn seek_seconds(&self, seconds: f32) {
        self.player.seek_seconds(seconds);
    }

    /// Sets the master playback volume level.
    ///
    /// # Arguments
    /// * `vol` - Volume multiplier (0.0 = silent, 1.0 = normal, > 1.0 = amplified).
    pub fn set_volume(&self, vol: f32) {
        self.player.set_volume(vol);
    }

    /// Returns the current master playback volume level.
    pub fn get_volume(&self) -> f32 {
        self.player.get_volume()
    }

    /// Returns `true` if audio is currently playing.
    pub fn is_playing(&self) -> bool {
        self.player.is_playing()
    }

    /// Returns the current playback position in seconds.
    pub fn current_time(&self) -> f32 {
        self.player.current_time_seconds()
    }

    /// Returns the total duration of the currently loaded track in seconds.
    pub fn duration_seconds(&self) -> f32 {
        self.player.duration_seconds()
    }

    /// Sets the visualizer gain sensitivity multiplier.
    ///
    /// # Arguments
    /// * `gain` - Gain factor applied to visual bar heights (minimum clamped to 0.1).
    pub fn set_gain_multiplier(&mut self, gain: f32) {
        self.gain_multiplier = gain.max(0.1);
    }

    /// Computes the current real-time smoothed frequency magnitudes for visualization.
    ///
    /// Pulls the synchronized PCM window around the playhead, applies the Hann window,
    /// computes the forward FFT, bins magnitudes into logarithmic frequency bands,
    /// and steps the asymmetric EMA smoother.
    ///
    /// # Arguments
    /// * `dt` - Elapsed delta time in seconds since the last rendered frame.
    ///
    /// # Returns
    /// A vector of normalized magnitude values in `[0.0, 1.0]` of length `num_bands`.
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

impl AudioVisualizerEngine {
    /// Returns a shared reference handle to the currently loaded [`AudioTrack`], if any.
    pub fn get_track(&self) -> Option<std::sync::Arc<AudioTrack>> {
        self.track.clone()
    }
}
