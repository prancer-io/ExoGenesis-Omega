//! Slow Wave Sleep (SWS)
//!
//! Deep sleep characterized by:
//! - Delta waves (0.5-4 Hz)
//! - High-amplitude slow oscillations
//! - Memory consolidation
//! - Growth hormone release

use rand::Rng;
use serde::{Deserialize, Serialize};

/// A single slow wave oscillation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowWave {
    /// Wave frequency (Hz)
    pub frequency: f64,
    /// Wave amplitude
    pub amplitude: f64,
    /// Duration (ms)
    pub duration: f64,
    /// Up-state activity level
    pub up_state_activity: f64,
    /// Down-state depth
    pub down_state_depth: f64,
    /// Timestamp
    pub timestamp: u64,
}

impl SlowWave {
    pub fn new(frequency: f64, amplitude: f64) -> Self {
        Self {
            frequency,
            amplitude,
            duration: 1000.0 / frequency,
            up_state_activity: amplitude * 0.8,
            down_state_depth: amplitude * 0.6,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Is this in the delta band?
    pub fn is_delta(&self) -> bool {
        self.frequency >= 0.5 && self.frequency <= 4.0
    }

    /// Calculate consolidation strength
    pub fn consolidation_strength(&self) -> f64 {
        // Higher amplitude and lower frequency = stronger consolidation
        self.amplitude * (1.0 / self.frequency.max(0.5))
    }
}

/// Slow wave sleep processor
pub struct SlowWaveSleep {
    /// Current slow wave power
    power: f64,
    /// Time in SWS (minutes)
    time_in_sws: f64,
    /// Wave generation rate (per minute)
    wave_rate: f64,
    /// Time until next wave
    next_wave_time: f64,
    /// Generated waves this session
    wave_count: usize,
    /// Delta power (0.5-4 Hz)
    delta_power: f64,
}

impl SlowWaveSleep {
    pub fn new() -> Self {
        Self {
            power: 0.0,
            time_in_sws: 0.0,
            wave_rate: 2.0, // 2 significant waves per minute
            next_wave_time: 0.0,
            wave_count: 0,
            delta_power: 0.0,
        }
    }

    /// Step forward and possibly generate a slow wave
    pub fn step(&mut self, dt_minutes: f64) -> Option<SlowWave> {
        self.time_in_sws += dt_minutes;
        self.next_wave_time -= dt_minutes;

        // Update delta power (builds up during SWS)
        self.delta_power = (self.time_in_sws / 30.0).min(1.0);
        self.power = self.delta_power;

        if self.next_wave_time <= 0.0 {
            // Generate a slow wave
            let mut rng = rand::thread_rng();

            let frequency = rng.gen_range(0.5..2.0); // Delta band
            let amplitude = 0.5 + self.delta_power * 0.5; // Increases with time in SWS

            self.wave_count += 1;
            self.next_wave_time = 1.0 / self.wave_rate + rng.gen_range(-0.2..0.2);

            Some(SlowWave::new(frequency, amplitude))
        } else {
            None
        }
    }

    /// Get current power level
    pub fn power(&self) -> f64 {
        self.power
    }

    /// Get delta band power
    pub fn delta_power(&self) -> f64 {
        self.delta_power
    }

    /// Get wave count
    pub fn wave_count(&self) -> usize {
        self.wave_count
    }

    /// Reset for new sleep session
    pub fn reset(&mut self) {
        self.power = 0.0;
        self.time_in_sws = 0.0;
        self.next_wave_time = 0.0;
        self.wave_count = 0;
        self.delta_power = 0.0;
    }
}

impl Default for SlowWaveSleep {
    fn default() -> Self {
        Self::new()
    }
}

/// Deep sleep state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSleepState {
    /// Delta power (0-1)
    pub delta_power: f64,
    /// Slow oscillation phase (0-2π)
    pub phase: f64,
    /// Up-state active
    pub in_up_state: bool,
    /// Cortical silence depth
    pub silence_depth: f64,
}

impl DeepSleepState {
    pub fn new() -> Self {
        Self {
            delta_power: 0.0,
            phase: 0.0,
            in_up_state: true,
            silence_depth: 0.0,
        }
    }

    /// Update state based on time
    pub fn update(&mut self, dt_seconds: f64, target_power: f64) {
        // Slow oscillation at ~0.8 Hz
        let omega = 2.0 * std::f64::consts::PI * 0.8;
        self.phase += omega * dt_seconds;
        if self.phase > 2.0 * std::f64::consts::PI {
            self.phase -= 2.0 * std::f64::consts::PI;
        }

        // Up state is when phase is in first half
        self.in_up_state = self.phase.cos() > 0.0;

        // Delta power tracks target
        self.delta_power = 0.9 * self.delta_power + 0.1 * target_power;

        // Silence depth is inverse of up-state
        self.silence_depth = if self.in_up_state { 0.0 } else { self.delta_power };
    }

    /// Check if good time for memory replay
    pub fn is_replay_window(&self) -> bool {
        // Up-state is optimal for replay
        self.in_up_state && self.delta_power > 0.5
    }
}

impl Default for DeepSleepState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slow_wave() {
        let wave = SlowWave::new(1.0, 0.8);
        assert!(wave.is_delta());
        assert!(wave.consolidation_strength() > 0.0);
    }

    #[test]
    fn test_sws_processor() {
        let mut sws = SlowWaveSleep::new();

        let mut waves = 0;
        for _ in 0..60 {
            if sws.step(1.0).is_some() {
                waves += 1;
            }
        }

        // Should generate ~120 waves in 60 minutes at 2/min
        assert!(waves > 50);
    }

    #[test]
    fn test_deep_sleep_state() {
        let mut state = DeepSleepState::new();

        for _ in 0..100 {
            state.update(0.1, 0.8);
        }

        assert!(state.delta_power > 0.5);
    }

    #[test]
    fn test_up_down_states() {
        let mut state = DeepSleepState::new();

        let mut up_count = 0;
        let mut down_count = 0;

        for _ in 0..100 {
            state.update(0.1, 0.8);
            if state.in_up_state {
                up_count += 1;
            } else {
                down_count += 1;
            }
        }

        // Should have roughly equal up and down states
        assert!(up_count > 20);
        assert!(down_count > 20);
    }
}
