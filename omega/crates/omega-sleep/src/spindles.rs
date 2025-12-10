//! Sleep Spindles and K-Complexes
//!
//! NREM Stage 2 phenomena:
//! - Sleep spindles: 12-16 Hz bursts lasting 0.5-2 seconds
//! - K-complexes: Large amplitude sharp waves
//! - Both involved in memory consolidation
//! - Thalamocortical oscillations

use rand::Rng;
use serde::{Deserialize, Serialize};

/// A sleep spindle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepSpindle {
    /// Frequency (Hz, typically 12-16)
    pub frequency: f64,
    /// Duration (seconds)
    pub duration: f64,
    /// Amplitude
    pub amplitude: f64,
    /// Number of cycles
    pub cycles: usize,
    /// Fast (>13 Hz) or slow (<13 Hz)
    pub is_fast: bool,
    /// Timestamp
    pub timestamp: u64,
}

impl SleepSpindle {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();

        let frequency = rng.gen_range(11.0..16.0);
        let duration = rng.gen_range(0.5..2.0);
        let cycles = (frequency * duration) as usize;

        Self {
            frequency,
            duration,
            amplitude: rng.gen_range(0.3..0.8),
            cycles,
            is_fast: frequency > 13.0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Calculate memory consolidation contribution
    pub fn consolidation_strength(&self) -> f64 {
        // Longer, higher amplitude spindles = better consolidation
        self.amplitude * self.duration.sqrt()
    }
}

/// A K-complex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KComplex {
    /// Peak amplitude
    pub amplitude: f64,
    /// Duration (ms)
    pub duration: f64,
    /// Evoked (by stimulus) or spontaneous
    pub evoked: bool,
    /// Associated spindle (often follows K-complex)
    pub has_spindle: bool,
    /// Timestamp
    pub timestamp: u64,
}

impl KComplex {
    pub fn generate(evoked: bool) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            amplitude: rng.gen_range(0.5..1.0),
            duration: rng.gen_range(500.0..1000.0), // 500ms - 1s
            evoked,
            has_spindle: rng.gen::<bool>(), // Often paired with spindles
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

/// Sleep spindle generator
pub struct SpindleGenerator {
    /// Time in N2 (minutes)
    time_in_n2: f64,
    /// Spindle rate (per minute)
    spindle_rate: f64,
    /// K-complex rate (per minute)
    k_complex_rate: f64,
    /// Time until next spindle
    next_spindle_time: f64,
    /// Time until next K-complex
    next_k_complex_time: f64,
    /// Spindle count
    spindle_count: usize,
    /// K-complex count
    k_complex_count: usize,
    /// Sigma power (12-16 Hz)
    sigma_power: f64,
}

impl SpindleGenerator {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            time_in_n2: 0.0,
            spindle_rate: 3.0, // ~3 spindles per minute typical
            k_complex_rate: 1.0, // ~1 K-complex per minute
            next_spindle_time: rng.gen_range(0.0..1.0),
            next_k_complex_time: rng.gen_range(0.0..2.0),
            spindle_count: 0,
            k_complex_count: 0,
            sigma_power: 0.0,
        }
    }

    /// Step and possibly generate spindle
    pub fn step(&mut self, dt_minutes: f64) -> Option<SleepSpindle> {
        self.time_in_n2 += dt_minutes;
        self.next_spindle_time -= dt_minutes;
        self.next_k_complex_time -= dt_minutes;

        // Update sigma power
        self.sigma_power = (self.time_in_n2 / 10.0).min(1.0) * 0.5 + 0.5;

        let mut rng = rand::thread_rng();

        // Generate spindle
        if self.next_spindle_time <= 0.0 {
            self.spindle_count += 1;
            self.next_spindle_time = 1.0 / self.spindle_rate + rng.gen_range(-0.2..0.2);

            return Some(SleepSpindle::generate());
        }

        None
    }

    /// Step and possibly generate K-complex
    pub fn step_k_complex(&mut self, dt_minutes: f64) -> Option<KComplex> {
        self.next_k_complex_time -= dt_minutes;

        let mut rng = rand::thread_rng();

        if self.next_k_complex_time <= 0.0 {
            self.k_complex_count += 1;
            self.next_k_complex_time = 1.0 / self.k_complex_rate + rng.gen_range(-0.5..0.5);

            return Some(KComplex::generate(false));
        }

        None
    }

    /// Generate evoked K-complex (response to stimulus)
    pub fn evoke_k_complex(&mut self) -> KComplex {
        self.k_complex_count += 1;
        KComplex::generate(true)
    }

    /// Get spindle count
    pub fn spindle_count(&self) -> usize {
        self.spindle_count
    }

    /// Get K-complex count
    pub fn k_complex_count(&self) -> usize {
        self.k_complex_count
    }

    /// Get sigma power
    pub fn sigma_power(&self) -> f64 {
        self.sigma_power
    }

    /// Reset for new N2 period
    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.time_in_n2 = 0.0;
        self.next_spindle_time = rng.gen_range(0.0..1.0);
        self.next_k_complex_time = rng.gen_range(0.0..2.0);
        self.spindle_count = 0;
        self.k_complex_count = 0;
        self.sigma_power = 0.0;
    }
}

impl Default for SpindleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Thalamocortical oscillation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThalamocorticalState {
    /// Spindle oscillation phase
    pub spindle_phase: f64,
    /// Spindle active
    pub spindle_active: bool,
    /// Thalamic reticular activity
    pub reticular_activity: f64,
    /// Thalamocortical relay activity
    pub relay_activity: f64,
}

impl ThalamocorticalState {
    pub fn new() -> Self {
        Self {
            spindle_phase: 0.0,
            spindle_active: false,
            reticular_activity: 0.0,
            relay_activity: 0.0,
        }
    }

    /// Update state
    pub fn update(&mut self, dt_seconds: f64, sigma_power: f64) {
        // Spindle oscillation at ~14 Hz when active
        if self.spindle_active {
            let omega = 2.0 * std::f64::consts::PI * 14.0;
            self.spindle_phase += omega * dt_seconds;

            if self.spindle_phase > 2.0 * std::f64::consts::PI * 20.0 {
                // End spindle after ~20 cycles
                self.spindle_active = false;
                self.spindle_phase = 0.0;
            }
        }

        // Reticular activity drives spindles
        self.reticular_activity = sigma_power * 0.8;

        // Relay activity is inverse of reticular
        self.relay_activity = 1.0 - self.reticular_activity * 0.5;
    }

    /// Start a spindle
    pub fn start_spindle(&mut self) {
        self.spindle_active = true;
        self.spindle_phase = 0.0;
    }
}

impl Default for ThalamocorticalState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_spindle() {
        let spindle = SleepSpindle::generate();

        assert!(spindle.frequency >= 11.0 && spindle.frequency <= 16.0);
        assert!(spindle.duration >= 0.5 && spindle.duration <= 2.0);
        assert!(spindle.cycles > 0);
    }

    #[test]
    fn test_k_complex() {
        let kc = KComplex::generate(false);
        assert!(!kc.evoked);

        let kc_evoked = KComplex::generate(true);
        assert!(kc_evoked.evoked);
    }

    #[test]
    fn test_spindle_generator() {
        let mut gen = SpindleGenerator::new();

        let mut spindles = 0;
        for _ in 0..30 {
            if gen.step(1.0).is_some() {
                spindles += 1;
            }
        }

        // Should generate some spindles over 30 minutes
        assert!(spindles > 0, "Expected at least some spindles, got {}", spindles);
    }

    #[test]
    fn test_sigma_power() {
        let mut gen = SpindleGenerator::new();

        for _ in 0..20 {
            gen.step(1.0);
        }

        assert!(gen.sigma_power() > 0.5);
    }

    #[test]
    fn test_thalamocortical_state() {
        let mut state = ThalamocorticalState::new();

        state.start_spindle();
        assert!(state.spindle_active);

        // Update through spindle (need enough time for spindle to complete ~20 cycles)
        // At 14 Hz, 20 cycles = ~1.43 seconds. With dt=0.01, need ~143 iterations
        for _ in 0..200 {
            state.update(0.01, 0.8);
        }

        // Spindle should have ended
        assert!(!state.spindle_active, "Spindle should end after sufficient time");
    }
}
