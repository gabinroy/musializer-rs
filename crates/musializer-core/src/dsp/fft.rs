use crate::dsp::window::HannWindow;
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftPlanner};
use std::sync::Arc;

/// High-performance FFT processor converting temporal PCM audio slices into frequency magnitude spectrums.
pub struct FftProcessor {
    fft: Arc<dyn Fft<f32>>,
    window: HannWindow,
    buffer: Vec<Complex<f32>>,
    scratch: Vec<Complex<f32>>,
    size: usize,
}

impl FftProcessor {
    pub fn new(size: usize) -> Self {
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(size);
        let scratch = vec![Complex::new(0.0, 0.0); fft.get_inplace_scratch_len()];
        let buffer = vec![Complex::new(0.0, 0.0); size];
        let window = HannWindow::new(size);

        Self {
            fft,
            window,
            buffer,
            scratch,
            size,
        }
    }

    /// Computes the frequency magnitude spectrum from an input chunk of PCM samples.
    /// Returns the first `size / 2` bins (positive frequency range 0 Hz to Nyquist).
    pub fn process(&mut self, pcm_samples: &[f32]) -> Vec<f32> {
        let mut windowed = vec![0.0f32; self.size];
        let copy_len = pcm_samples.len().min(self.size);
        windowed[..copy_len].copy_from_slice(&pcm_samples[..copy_len]);

        // Apply Hann window
        self.window.apply(&mut windowed);

        // Fill complex buffer
        for (c, &sample) in self.buffer.iter_mut().zip(windowed.iter()) {
            c.re = sample;
            c.im = 0.0;
        }

        // Perform forward FFT in place
        self.fft
            .process_with_scratch(&mut self.buffer, &mut self.scratch);

        // Extract magnitudes for 0..size/2
        let num_bins = self.size / 2;
        let mut magnitudes = Vec::with_capacity(num_bins);
        let norm_factor = 2.0 / (self.size as f32);

        for c in &self.buffer[..num_bins] {
            let mag = (c.re * c.re + c.im * c.im).sqrt() * norm_factor;
            magnitudes.push(mag);
        }

        magnitudes
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_fft_sine_peak() {
        let size = 2048;
        let mut processor = FftProcessor::new(size);
        let sample_rate = 44100.0;
        let target_freq = 440.0; // A4 tone

        // Generate pure sine wave at 440 Hz
        let mut samples = vec![0.0f32; size];
        for (i, s) in samples.iter_mut().enumerate() {
            *s = (2.0 * PI * target_freq * i as f32 / sample_rate).sin();
        }

        let mags = processor.process(&samples);
        let expected_bin = (target_freq * size as f32 / sample_rate).round() as usize;

        // Find the bin with the highest magnitude
        let max_bin = mags
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        assert!((max_bin as isize - expected_bin as isize).abs() <= 1);
    }
}
