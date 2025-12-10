//! Circadian Rhythm
//!
//! 24-hour biological clock:
//! - Sleep/wake propensity
//! - Melatonin release
//! - Core body temperature
//! - Alertness cycles

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Time of day representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours (0-23)
    pub hours: u8,
    /// Minutes (0-59)
    pub minutes: u8,
}

impl TimeOfDay {
    pub fn new(hours: u8, minutes: u8) -> Self {
        Self {
            hours: hours % 24,
            minutes: minutes % 60,
        }
    }

    /// Create from decimal hours
    pub fn from_decimal(decimal_hours: f64) -> Self {
        let hours = (decimal_hours.floor() as u8) % 24;
        let minutes = ((decimal_hours.fract() * 60.0).round() as u8) % 60;
        Self { hours, minutes }
    }

    /// Convert to decimal hours
    pub fn to_decimal(&self) -> f64 {
        self.hours as f64 + self.minutes as f64 / 60.0
    }

    /// Get circadian phase (0 to 2π)
    pub fn to_phase(&self) -> f64 {
        (self.to_decimal() / 24.0) * 2.0 * PI
    }
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self::new(12, 0)
    }
}

/// Circadian rhythm controller
pub struct CircadianRhythm {
    /// Current time
    current_time: TimeOfDay,
    /// Period (hours, ~24.2 for humans)
    period: f64,
    /// Phase offset (allows for chronotype differences)
    phase_offset: f64,
    /// Amplitude of rhythm
    amplitude: f64,
    /// Light exposure (lux, affects rhythm)
    light_exposure: f64,
    /// Melatonin level
    melatonin: f64,
    /// Core body temperature deviation
    temperature_deviation: f64,
}

impl CircadianRhythm {
    pub fn new() -> Self {
        Self {
            current_time: TimeOfDay::new(8, 0), // Start at 8 AM
            period: 24.2,
            phase_offset: 0.0,
            amplitude: 1.0,
            light_exposure: 500.0, // Moderate indoor light
            melatonin: 0.0,
            temperature_deviation: 0.0,
        }
    }

    /// Set current time
    pub fn set_time(&mut self, time: TimeOfDay) {
        self.current_time = time;
        self.update_markers();
    }

    /// Advance time by minutes
    pub fn advance(&mut self, minutes: f64) {
        let current_decimal = self.current_time.to_decimal();
        let new_decimal = (current_decimal + minutes / 60.0) % 24.0;
        self.current_time = TimeOfDay::from_decimal(new_decimal);
        self.update_markers();
    }

    /// Update physiological markers
    fn update_markers(&mut self) {
        let phase = self.current_time.to_phase() + self.phase_offset;

        // Melatonin peaks around 3 AM (phase ≈ π/4)
        let melatonin_phase = phase - PI / 8.0;
        self.melatonin = self.amplitude * ((-melatonin_phase.cos() + 1.0) / 2.0);

        // Suppress melatonin with light
        if self.light_exposure > 1000.0 {
            self.melatonin *= 0.2;
        } else if self.light_exposure > 100.0 {
            self.melatonin *= 0.5;
        }

        // Core body temperature nadir around 4-5 AM
        let temp_phase = phase - PI / 6.0;
        self.temperature_deviation = self.amplitude * 0.5 * temp_phase.cos();
    }

    /// Set light exposure (lux)
    pub fn set_light(&mut self, lux: f64) {
        self.light_exposure = lux;
        self.update_markers();
    }

    /// Get current sleep drive (0-1)
    pub fn current_sleep_drive(&self) -> f64 {
        // High melatonin = high sleep drive
        // Low temperature = high sleep drive
        let melatonin_drive = self.melatonin;
        let temp_drive = (-self.temperature_deviation + 0.5).max(0.0).min(1.0);

        (melatonin_drive * 0.6 + temp_drive * 0.4).min(1.0)
    }

    /// Get current alertness (0-1)
    pub fn current_alertness(&self) -> f64 {
        1.0 - self.current_sleep_drive()
    }

    /// Is it appropriate to sleep?
    pub fn should_sleep(&self) -> bool {
        self.current_sleep_drive() > 0.6
    }

    /// Get optimal wake time
    pub fn optimal_wake_time(&self) -> TimeOfDay {
        // Wake when temperature starts rising (around 6-7 AM for typical person)
        TimeOfDay::new(7, 0)
    }

    /// Get optimal sleep time
    pub fn optimal_sleep_time(&self) -> TimeOfDay {
        // Sleep when melatonin is rising (around 10-11 PM)
        TimeOfDay::new(22, 30)
    }

    /// Get melatonin level
    pub fn melatonin(&self) -> f64 {
        self.melatonin
    }

    /// Get temperature deviation
    pub fn temperature_deviation(&self) -> f64 {
        self.temperature_deviation
    }

    /// Get current time
    pub fn current_time(&self) -> TimeOfDay {
        self.current_time
    }

    /// Set chronotype (phase offset in hours, positive = night owl)
    pub fn set_chronotype(&mut self, offset_hours: f64) {
        self.phase_offset = (offset_hours / 24.0) * 2.0 * PI;
        self.update_markers();
    }
}

impl Default for CircadianRhythm {
    fn default() -> Self {
        Self::new()
    }
}

/// Circadian state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianState {
    pub time: TimeOfDay,
    pub melatonin: f64,
    pub temperature_deviation: f64,
    pub sleep_drive: f64,
    pub alertness: f64,
}

impl CircadianRhythm {
    /// Get current state snapshot
    pub fn state(&self) -> CircadianState {
        CircadianState {
            time: self.current_time,
            melatonin: self.melatonin,
            temperature_deviation: self.temperature_deviation,
            sleep_drive: self.current_sleep_drive(),
            alertness: self.current_alertness(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_of_day() {
        let time = TimeOfDay::new(14, 30);
        assert_eq!(time.hours, 14);
        assert_eq!(time.minutes, 30);
        assert!((time.to_decimal() - 14.5).abs() < 0.01);
    }

    #[test]
    fn test_time_from_decimal() {
        let time = TimeOfDay::from_decimal(14.5);
        assert_eq!(time.hours, 14);
        assert_eq!(time.minutes, 30);
    }

    #[test]
    fn test_circadian_rhythm() {
        let mut rhythm = CircadianRhythm::new();

        // Check 8 AM (should be alert)
        rhythm.set_time(TimeOfDay::new(8, 0));
        assert!(rhythm.current_alertness() > 0.5);

        // Check 3 AM (should be sleepy)
        rhythm.set_time(TimeOfDay::new(3, 0));
        assert!(rhythm.current_sleep_drive() > 0.5);
    }

    #[test]
    fn test_light_suppresses_melatonin() {
        let mut rhythm = CircadianRhythm::new();
        rhythm.set_time(TimeOfDay::new(23, 0));

        let dark_melatonin = rhythm.melatonin();

        rhythm.set_light(2000.0); // Bright light
        let light_melatonin = rhythm.melatonin();

        assert!(light_melatonin < dark_melatonin);
    }

    #[test]
    fn test_advance_time() {
        let mut rhythm = CircadianRhythm::new();
        rhythm.set_time(TimeOfDay::new(8, 0));

        rhythm.advance(90.0); // 1.5 hours
        assert_eq!(rhythm.current_time().hours, 9);
        assert_eq!(rhythm.current_time().minutes, 30);
    }

    #[test]
    fn test_chronotype() {
        let mut morning_person = CircadianRhythm::new();
        morning_person.set_chronotype(-2.0); // 2 hours earlier

        let mut night_owl = CircadianRhythm::new();
        night_owl.set_chronotype(2.0); // 2 hours later

        // At 10 PM, night owl should be more alert
        morning_person.set_time(TimeOfDay::new(22, 0));
        night_owl.set_time(TimeOfDay::new(22, 0));

        assert!(night_owl.current_alertness() > morning_person.current_alertness());
    }
}
