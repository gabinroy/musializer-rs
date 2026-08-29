/// Asymmetric Exponential Moving Average (EMA) and Peak-Hold smoother.
/// Attack coefficient governs fast responsiveness to transients.
/// Decay coefficient governs graceful falling speed of visual bars.
pub struct EmaSmoother {
    values: Vec<f32>,
    peaks: Vec<f32>,
    peak_hold_timers: Vec<f32>,
    attack: f32,
    decay: f32,
    peak_decay: f32,
}

impl EmaSmoother {
    pub fn new(num_bands: usize, attack: f32, decay: f32) -> Self {
        Self {
            values: vec![0.0f32; num_bands],
            peaks: vec![0.0f32; num_bands],
            peak_hold_timers: vec![0.0f32; num_bands],
            attack: attack.clamp(0.01, 1.0),
            decay: decay.clamp(0.001, 0.99),
            peak_decay: 0.05,
        }
    }

    /// Updates smoothed values and peak caps with new incoming raw target values.
    /// `dt` is delta time in seconds since last frame.
    pub fn update(&mut self, targets: &[f32], dt: f32) {
        if targets.len() != self.values.len() {
            self.values.resize(targets.len(), 0.0);
            self.peaks.resize(targets.len(), 0.0);
            self.peak_hold_timers.resize(targets.len(), 0.0);
        }

        for (i, &target) in targets.iter().enumerate() {
            let current = self.values[i];

            if target > current {
                // Fast Attack: rise rapidly
                let alpha = (self.attack * (dt * 60.0)).clamp(0.0, 1.0);
                self.values[i] = current + alpha * (target - current);
            } else {
                // Smooth Decay: fall gracefully
                let alpha = (self.decay * (dt * 60.0)).clamp(0.0, 1.0);
                self.values[i] = current + alpha * (target - current);
            }

            // Peak Hold Logic
            if self.values[i] > self.peaks[i] {
                self.peaks[i] = self.values[i];
                self.peak_hold_timers[i] = 0.2; // Hold peak for 200ms
            } else {
                if self.peak_hold_timers[i] > 0.0 {
                    self.peak_hold_timers[i] -= dt;
                } else {
                    self.peaks[i] =
                        (self.peaks[i] - self.peak_decay * (dt * 60.0)).max(self.values[i]);
                }
            }
        }
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }

    pub fn peaks(&self) -> &[f32] {
        &self.peaks
    }

    pub fn reset(&mut self) {
        self.values.fill(0.0);
        self.peaks.fill(0.0);
        self.peak_hold_timers.fill(0.0);
    }
}
