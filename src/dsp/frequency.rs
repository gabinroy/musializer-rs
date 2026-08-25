/// Maps linear FFT magnitude bins into logarithmically spaced frequency bands (human hearing range 20Hz - 20kHz).
pub struct FrequencyBands {
    num_bands: usize,
    band_bin_ranges: Vec<(usize, usize)>,
}

impl FrequencyBands {
    pub fn new(num_bands: usize, fft_size: usize, sample_rate: u32) -> Self {
        let min_freq = 20.0f32;
        let max_freq = (sample_rate as f32 / 2.0).min(20000.0);
        let freq_resolution = sample_rate as f32 / fft_size as f32;
        let nyquist_bins = fft_size / 2;

        let mut band_bin_ranges = Vec::with_capacity(num_bands);

        for i in 0..num_bands {
            // Logarithmic frequency partition
            let f_low = min_freq * (max_freq / min_freq).powf(i as f32 / num_bands as f32);
            let f_high = min_freq * (max_freq / min_freq).powf((i + 1) as f32 / num_bands as f32);

            let bin_low =
                ((f_low / freq_resolution).floor() as usize).min(nyquist_bins.saturating_sub(1));
            let mut bin_high = ((f_high / freq_resolution).ceil() as usize).min(nyquist_bins);
            if bin_high <= bin_low {
                bin_high = (bin_low + 1).min(nyquist_bins);
            }

            band_bin_ranges.push((bin_low, bin_high));
        }

        Self {
            num_bands,
            band_bin_ranges,
        }
    }

    /// Aggregates linear FFT magnitude bins into visual frequency bands with dynamic range compression and gain boost.
    pub fn aggregate(&self, magnitudes: &[f32], gain_multiplier: f32) -> Vec<f32> {
        let mut bands = vec![0.0f32; self.num_bands];

        for (i, &(bin_low, bin_high)) in self.band_bin_ranges.iter().enumerate() {
            if bin_low >= magnitudes.len() {
                continue;
            }
            let high = bin_high.min(magnitudes.len());
            if high <= bin_low {
                continue;
            }

            let slice = &magnitudes[bin_low..high];
            // Compute average power and max peak in the band
            let max_val = slice.iter().copied().fold(0.0f32, f32::max);
            let sum: f32 = slice.iter().sum();
            let avg = sum / slice.len() as f32;

            // Blend max and average to balance punchiness and energy representation
            let raw_val = 0.6 * max_val + 0.4 * avg;

            // Treble compensation curve for natural 1/f falloff
            let treble_boost = 1.0 + 1.8 * (i as f32 / self.num_bands as f32).powf(1.4);

            // Dynamic Range Compression: Power-law curve (gamma ~ 0.45)
            // Lifts quiet sections while preventing loud spikes from clipping
            let scaled_val = (raw_val * 4.0 * gain_multiplier * treble_boost).max(0.0);
            let compressed = scaled_val.powf(0.55);

            bands[i] = compressed.clamp(0.0, 1.0);
        }

        bands
    }

    #[allow(dead_code)]
    pub fn num_bands(&self) -> usize {
        self.num_bands
    }
}
