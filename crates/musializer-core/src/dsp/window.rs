use std::f32::consts::PI;

/// Hann (Hanning) window generator to prevent spectral leakage in FFT analysis.
pub struct HannWindow {
    weights: Vec<f32>,
}

impl HannWindow {
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

    /// Multiplies the input samples in-place by the precomputed Hann window weights.
    pub fn apply(&self, samples: &mut [f32]) {
        for (s, &w) in samples.iter_mut().zip(self.weights.iter()) {
            *s *= w;
        }
    }

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
