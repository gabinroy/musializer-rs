use std::path::{Path, PathBuf};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

#[derive(Clone)]
pub struct AudioTrack {
    /// Interleaved stereo f32 samples in range [-1.0, 1.0]
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    #[allow(dead_code)]
    pub total_samples: usize,
    pub duration_seconds: f32,
    pub file_path: PathBuf,
    pub title: String,
}

impl AudioTrack {
    /// Decodes an audio file at the given path into memory as normalized stereo f32 PCM samples.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_buf = path.as_ref().to_path_buf();
        let file = std::fs::File::open(&path_buf)
            .map_err(|e| format!("Failed to open audio file {:?}: {}", path_buf, e))?;

        let title = path_buf
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown Track")
            .to_string();

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(extension) = path_buf.extension().and_then(|ext| ext.to_str()) {
            hint.with_extension(extension);
        }

        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .map_err(|e| format!("Unsupported audio format: {}", e))?;

        let mut format = probed.format;

        // Find the default audio track
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| "No supported audio tracks found in file".to_string())?;

        let track_id = track.id;
        let sample_rate = track
            .codec_params
            .sample_rate
            .ok_or_else(|| "Unknown sample rate".to_string())?;

        let dec_opts: DecoderOptions = Default::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .map_err(|e| format!("Failed to create audio decoder: {}", e))?;

        let mut stereo_samples: Vec<f32> = Vec::new();
        let mut sample_buf: Option<SampleBuffer<f32>> = None;

        while let Ok(packet) = format.next_packet() {
            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(audio_buf_ref) => {
                    let spec = *audio_buf_ref.spec();
                    let channels = spec.channels.count();

                    // Initialize sample buffer with spec if needed
                    let buf = sample_buf.get_or_insert_with(|| {
                        SampleBuffer::<f32>::new(audio_buf_ref.capacity() as u64, spec)
                    });

                    // Ensure capacity
                    if buf.capacity() < audio_buf_ref.frames() {
                        *buf = SampleBuffer::<f32>::new(audio_buf_ref.capacity() as u64, spec);
                    }

                    buf.copy_interleaved_ref(audio_buf_ref);

                    let raw_samples = buf.samples();

                    if channels == 1 {
                        // Duplicate mono to stereo [L, R, L, R, ...]
                        for &s in raw_samples {
                            stereo_samples.push(s);
                            stereo_samples.push(s);
                        }
                    } else if channels >= 2 {
                        // Convert multi-channel or stereo directly to stereo
                        for chunk in raw_samples.chunks(channels) {
                            let left = chunk[0];
                            let right = chunk[1];
                            stereo_samples.push(left);
                            stereo_samples.push(right);
                        }
                    }
                }
                Err(SymphoniaError::DecodeError(err)) => {
                    log::warn!("Decode warning: {}", err);
                    continue;
                }
                Err(SymphoniaError::IoError(err)) => {
                    log::warn!("I/O warning during decoding: {}", err);
                    break;
                }
                Err(err) => {
                    return Err(format!("Fatal decode error: {}", err));
                }
            }
        }

        let total_samples = stereo_samples.len();
        let total_frames = total_samples / 2;
        let duration_seconds = total_frames as f32 / sample_rate as f32;

        Ok(AudioTrack {
            samples: stereo_samples,
            sample_rate,
            channels: 2,
            total_samples,
            duration_seconds,
            file_path: path_buf,
            title,
        })
    }
}
