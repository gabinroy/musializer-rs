//! # Hann Window Generator
//!
//! Generates and applies a Hann (Hanning) window function to incoming audio PCM frames.
//! Windowing tapers both ends of a finite sample window down to zero, mitigating
//! spectral leakage artifacts when performing the discrete Fast Fourier Transform.

use std::f32::consts::PI;

/// Precomputed Hann window weight coefficients.
pub struct HannWindow {
    weights: Vec<f32>,
}

impl HannWindow {
    /// Precomputes Hann window weights for an FFT buffer of size `size`.
    ///
    /// # Arguments
    /// * `size` - Number of samples in the window (typically 2048).
    pub fn new(size: usize) -> Self {
        let mut weights = Vec::with_capacity(size);
        if size <= 1 {
            weights.push(1.0);
        } else {
            let n_minus_1 = (size - 1) as f32;
            for n in 0..size {
                let w = 0.5 * (1.0 - (2.0 * PI * n as f32 / n_minus_1).cos());
                weights.push(w);
            }
        }
        Self { weights }
    }

    /// Multiplies the input sample buffer in-place by the precomputed Hann window weights.
    ///
    /// # Arguments
    /// * `samples` - Mutable slice of audio PCM samples.
    pub fn apply(&self, samples: &mut [f32]) {
        for (s, &w) in samples.iter_mut().zip(self.weights.iter()) {
            *s *= w;
        }
    }

    /// Returns the window size.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.weights.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hann_window_bounds() {
        let size = 2048;
        let window = HannWindow::new(size);
        assert_eq!(window.size(), size);
        // Hann window starts and ends near 0.0, peaks at 1.0 around the center
        assert!((window.weights[0] - 0.0).abs() < 1e-5);
        assert!((window.weights[size - 1] - 0.0).abs() < 1e-5);
        let mid = size / 2;
        assert!((window.weights[mid] - 1.0).abs() < 1e-3);
    }
}
