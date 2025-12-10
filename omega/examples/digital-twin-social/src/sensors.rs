//! Sensor Integration - Keyboard and wearable emotional inference
//!
//! This module demonstrates how to integrate various sensor inputs
//! for emotional state inference while preserving privacy.

use crate::emotional::{EmotionalSignal, SignalSource};
use crate::types::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use thiserror::Error;

/// Errors in sensor processing
#[derive(Error, Debug)]
pub enum SensorError {
    #[error("Insufficient data for inference")]
    InsufficientData,
    #[error("Invalid sensor reading")]
    InvalidReading,
}

// =============================================================================
// KEYBOARD SENSOR
// =============================================================================

/// Keyboard sensor for emotional inference from typing patterns
///
/// This analyzes keystroke dynamics without capturing actual key content,
/// preserving user privacy while detecting emotional states.
pub struct KeyboardSensor {
    /// Recent keystroke samples
    samples: VecDeque<KeystrokeSample>,
    /// Maximum samples to retain
    max_samples: usize,
    /// Baseline typing metrics for this user
    baseline: Option<TypingBaseline>,
    /// Configuration
    config: KeyboardConfig,
}

/// Configuration for keyboard sensor
#[derive(Debug, Clone)]
pub struct KeyboardConfig {
    /// Minimum samples needed for inference
    pub min_samples: usize,
    /// Maximum age of samples to consider (seconds)
    pub sample_window_secs: u32,
    /// Whether to learn baseline adaptively
    pub adaptive_baseline: bool,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            min_samples: 20,
            sample_window_secs: 60,
            adaptive_baseline: true,
        }
    }
}

/// A single keystroke sample (privacy-preserving)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystrokeSample {
    /// Time key was pressed
    pub key_down: DateTime<Utc>,
    /// Time key was released
    pub key_up: DateTime<Utc>,
    /// Category of key (not the actual key)
    pub category: KeyCategory,
    /// Pressure if available
    pub pressure: Option<f32>,
}

impl KeystrokeSample {
    /// Get dwell time (how long key was held)
    pub fn dwell_time_ms(&self) -> i64 {
        (self.key_up - self.key_down).num_milliseconds()
    }
}

/// Baseline typing metrics for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingBaseline {
    pub avg_dwell_time_ms: f64,
    pub avg_flight_time_ms: f64,
    pub avg_wpm: f64,
    pub dwell_std_dev: f64,
    pub flight_std_dev: f64,
    pub sample_count: u64,
}

impl KeyboardSensor {
    /// Create a new keyboard sensor
    pub fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(1000),
            max_samples: 1000,
            baseline: None,
            config: KeyboardConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: KeyboardConfig) -> Self {
        Self {
            samples: VecDeque::with_capacity(1000),
            max_samples: 1000,
            baseline: None,
            config,
        }
    }

    /// Add a keystroke sample
    pub fn add_sample(&mut self, sample: KeystrokeSample) {
        if self.samples.len() >= self.max_samples {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);

        // Update baseline adaptively
        if self.config.adaptive_baseline {
            self.update_baseline();
        }
    }

    /// Add keystroke from raw timing data
    pub fn add_keystroke(
        &mut self,
        key_down: DateTime<Utc>,
        key_up: DateTime<Utc>,
        category: KeyCategory,
        pressure: Option<f32>,
    ) {
        self.add_sample(KeystrokeSample {
            key_down,
            key_up,
            category,
            pressure,
        });
    }

    /// Infer emotional state from recent typing patterns
    pub fn infer_emotion(&self) -> Result<EmotionalSignal, SensorError> {
        let recent = self.get_recent_samples();

        if recent.len() < self.config.min_samples {
            return Err(SensorError::InsufficientData);
        }

        // Calculate current metrics
        let metrics = self.calculate_metrics(&recent);

        // Compare to baseline
        let (valence, arousal, dominance) = self.metrics_to_emotion(&metrics);

        // Calculate confidence based on sample count and consistency
        let confidence = self.calculate_confidence(&recent);

        Ok(EmotionalSignal {
            source: SignalSource::Keyboard,
            valence,
            arousal,
            dominance,
            confidence,
            timestamp: Utc::now(),
        })
    }

    /// Get recent samples within the window
    fn get_recent_samples(&self) -> Vec<&KeystrokeSample> {
        let cutoff = Utc::now() - Duration::seconds(self.config.sample_window_secs as i64);
        self.samples.iter()
            .filter(|s| s.key_down > cutoff)
            .collect()
    }

    /// Calculate typing metrics from samples
    fn calculate_metrics(&self, samples: &[&KeystrokeSample]) -> TypingMetrics {
        if samples.is_empty() {
            return TypingMetrics::default();
        }

        // Calculate dwell times
        let dwell_times: Vec<f64> = samples.iter()
            .map(|s| s.dwell_time_ms() as f64)
            .collect();

        let avg_dwell = dwell_times.iter().sum::<f64>() / dwell_times.len() as f64;

        // Calculate flight times (time between keys)
        let mut flight_times = Vec::new();
        for i in 1..samples.len() {
            let flight = (samples[i].key_down - samples[i-1].key_up).num_milliseconds();
            if flight > 0 && flight < 2000 {
                flight_times.push(flight as f64);
            }
        }

        let avg_flight = if flight_times.is_empty() {
            200.0
        } else {
            flight_times.iter().sum::<f64>() / flight_times.len() as f64
        };

        // Calculate standard deviations
        let dwell_variance: f64 = dwell_times.iter()
            .map(|d| (d - avg_dwell).powi(2))
            .sum::<f64>() / dwell_times.len() as f64;

        let flight_variance: f64 = if flight_times.is_empty() {
            0.0
        } else {
            flight_times.iter()
                .map(|f| (f - avg_flight).powi(2))
                .sum::<f64>() / flight_times.len() as f64
        };

        // Estimate WPM (5 chars per word)
        let total_time_ms = if samples.len() > 1 {
            (samples.last().unwrap().key_up - samples.first().unwrap().key_down).num_milliseconds()
        } else {
            1000
        };
        let wpm = (samples.len() as f64 / 5.0) / (total_time_ms as f64 / 60000.0);

        // Backspace ratio
        let backspace_count = samples.iter()
            .filter(|s| matches!(s.category, KeyCategory::Backspace))
            .count();
        let backspace_ratio = backspace_count as f64 / samples.len() as f64;

        // Pause count (flights > 500ms)
        let pause_count = flight_times.iter()
            .filter(|&f| *f > 500.0)
            .count();

        TypingMetrics {
            avg_dwell_time_ms: avg_dwell,
            avg_flight_time_ms: avg_flight,
            dwell_std_dev: dwell_variance.sqrt(),
            flight_std_dev: flight_variance.sqrt(),
            wpm,
            backspace_ratio,
            pause_count,
        }
    }

    /// Convert typing metrics to emotional dimensions
    fn metrics_to_emotion(&self, metrics: &TypingMetrics) -> (f32, f32, f32) {
        let baseline = self.baseline.as_ref();

        // Valence: Faster typing with fewer corrections = more positive
        let valence = if let Some(b) = baseline {
            let wpm_diff = (metrics.wpm - b.avg_wpm) / b.avg_wpm.max(1.0);
            let backspace_factor = -metrics.backspace_ratio as f32 * 2.0;
            ((wpm_diff as f32 * 0.3 + backspace_factor) * 0.5).clamp(-1.0, 1.0)
        } else {
            // Without baseline, use absolute heuristics
            let wpm_factor = ((metrics.wpm - 40.0) / 40.0) as f32;
            let backspace_factor = -metrics.backspace_ratio as f32 * 2.0;
            (wpm_factor * 0.3 + backspace_factor * 0.3).clamp(-1.0, 1.0)
        };

        // Arousal: Higher variability and faster typing = higher arousal
        let arousal = if let Some(b) = baseline {
            let dwell_var_ratio = metrics.dwell_std_dev / b.dwell_std_dev.max(1.0);
            let speed_factor = (metrics.wpm / b.avg_wpm.max(1.0)) as f32;
            ((dwell_var_ratio as f32 * 0.3 + speed_factor * 0.3) - 0.3).clamp(0.0, 1.0)
        } else {
            let var_factor = (metrics.dwell_std_dev / 50.0) as f32;
            let speed_factor = (metrics.wpm / 60.0) as f32;
            ((var_factor + speed_factor) / 2.0).clamp(0.0, 1.0)
        };

        // Dominance: Steady typing with few pauses = higher dominance
        let dominance = {
            let pause_factor = (1.0 - metrics.pause_count as f32 / 10.0).max(0.0);
            let consistency = (1.0 - metrics.flight_std_dev as f32 / 200.0).max(0.0);
            ((pause_factor + consistency) / 2.0).clamp(0.0, 1.0)
        };

        (valence, arousal, dominance)
    }

    /// Calculate confidence in the inference
    fn calculate_confidence(&self, samples: &[&KeystrokeSample]) -> f32 {
        let sample_factor = (samples.len() as f32 / 50.0).min(1.0);
        let baseline_factor = if self.baseline.is_some() { 0.3 } else { 0.0 };

        (sample_factor * 0.7 + baseline_factor).min(0.9)
    }

    /// Update baseline from accumulated samples
    fn update_baseline(&mut self) {
        if self.samples.len() < 100 {
            return;
        }

        // Use older samples for baseline (not recent emotion-influenced ones)
        let baseline_samples: Vec<_> = self.samples.iter()
            .take(self.samples.len() / 2)
            .collect();

        let metrics = self.calculate_metrics(&baseline_samples);

        let new_baseline = TypingBaseline {
            avg_dwell_time_ms: metrics.avg_dwell_time_ms,
            avg_flight_time_ms: metrics.avg_flight_time_ms,
            avg_wpm: metrics.wpm,
            dwell_std_dev: metrics.dwell_std_dev,
            flight_std_dev: metrics.flight_std_dev,
            sample_count: baseline_samples.len() as u64,
        };

        // Exponential moving average with existing baseline
        if let Some(existing) = &self.baseline {
            let alpha = 0.1;
            self.baseline = Some(TypingBaseline {
                avg_dwell_time_ms: existing.avg_dwell_time_ms * (1.0 - alpha) + new_baseline.avg_dwell_time_ms * alpha,
                avg_flight_time_ms: existing.avg_flight_time_ms * (1.0 - alpha) + new_baseline.avg_flight_time_ms * alpha,
                avg_wpm: existing.avg_wpm * (1.0 - alpha) + new_baseline.avg_wpm * alpha,
                dwell_std_dev: existing.dwell_std_dev * (1.0 - alpha) + new_baseline.dwell_std_dev * alpha,
                flight_std_dev: existing.flight_std_dev * (1.0 - alpha) + new_baseline.flight_std_dev * alpha,
                sample_count: existing.sample_count + 1,
            });
        } else {
            self.baseline = Some(new_baseline);
        }
    }

    /// Get current sample count
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Check if sensor has enough data for inference
    pub fn has_sufficient_data(&self) -> bool {
        self.get_recent_samples().len() >= self.config.min_samples
    }
}

impl Default for KeyboardSensor {
    fn default() -> Self {
        Self::new()
    }
}

/// Typing metrics for analysis
#[derive(Debug, Clone, Default)]
struct TypingMetrics {
    avg_dwell_time_ms: f64,
    avg_flight_time_ms: f64,
    dwell_std_dev: f64,
    flight_std_dev: f64,
    wpm: f64,
    backspace_ratio: f64,
    pause_count: usize,
}

// =============================================================================
// WEARABLE SENSOR
// =============================================================================

/// Wearable sensor for emotional inference from biometrics
pub struct WearableSensor {
    /// Recent biometric samples
    samples: VecDeque<BiometricSample>,
    /// Maximum samples to retain
    max_samples: usize,
    /// Baseline biometrics
    baseline: Option<BiometricBaseline>,
    /// Configuration
    config: WearableConfig,
}

/// Configuration for wearable sensor
#[derive(Debug, Clone)]
pub struct WearableConfig {
    /// Minimum samples for inference
    pub min_samples: usize,
    /// Sample window in seconds
    pub sample_window_secs: u32,
    /// Enable HRV analysis
    pub enable_hrv: bool,
}

impl Default for WearableConfig {
    fn default() -> Self {
        Self {
            min_samples: 10,
            sample_window_secs: 300,
            enable_hrv: true,
        }
    }
}

/// A biometric sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricSample {
    pub timestamp: DateTime<Utc>,
    pub heart_rate: Option<f32>,
    pub hrv_rmssd: Option<f32>,
    pub skin_temperature: Option<f32>,
    pub eda: Option<f32>,
    pub activity_level: Option<f32>,
}

/// Baseline biometric values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricBaseline {
    pub resting_hr: f32,
    pub baseline_hrv: f32,
    pub baseline_temp: f32,
    pub baseline_eda: f32,
}

impl WearableSensor {
    /// Create a new wearable sensor
    pub fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(1000),
            max_samples: 1000,
            baseline: None,
            config: WearableConfig::default(),
        }
    }

    /// Add a biometric sample
    pub fn add_sample(&mut self, sample: BiometricSample) {
        if self.samples.len() >= self.max_samples {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }

    /// Add from BiometricData type
    pub fn add_biometric_data(&mut self, data: BiometricData) {
        self.add_sample(BiometricSample {
            timestamp: data.timestamp,
            heart_rate: data.heart_rate,
            hrv_rmssd: data.hrv_rmssd,
            skin_temperature: data.skin_temperature,
            eda: data.eda,
            activity_level: data.activity_level,
        });
    }

    /// Infer emotional state from biometrics
    pub fn infer_emotion(&self) -> Result<EmotionalSignal, SensorError> {
        let recent = self.get_recent_samples();

        if recent.len() < self.config.min_samples {
            return Err(SensorError::InsufficientData);
        }

        let (valence, arousal, dominance) = self.biometrics_to_emotion(&recent);
        let confidence = self.calculate_confidence(&recent);

        Ok(EmotionalSignal {
            source: SignalSource::Wearable,
            valence,
            arousal,
            dominance,
            confidence,
            timestamp: Utc::now(),
        })
    }

    /// Get recent samples
    fn get_recent_samples(&self) -> Vec<&BiometricSample> {
        let cutoff = Utc::now() - Duration::seconds(self.config.sample_window_secs as i64);
        self.samples.iter()
            .filter(|s| s.timestamp > cutoff)
            .collect()
    }

    /// Convert biometrics to emotional dimensions
    fn biometrics_to_emotion(&self, samples: &[&BiometricSample]) -> (f32, f32, f32) {
        // Calculate averages
        let hr_samples: Vec<f32> = samples.iter()
            .filter_map(|s| s.heart_rate)
            .collect();
        let hrv_samples: Vec<f32> = samples.iter()
            .filter_map(|s| s.hrv_rmssd)
            .collect();
        let eda_samples: Vec<f32> = samples.iter()
            .filter_map(|s| s.eda)
            .collect();

        let avg_hr = if hr_samples.is_empty() { 70.0 } else {
            hr_samples.iter().sum::<f32>() / hr_samples.len() as f32
        };
        let avg_hrv = if hrv_samples.is_empty() { 50.0 } else {
            hrv_samples.iter().sum::<f32>() / hrv_samples.len() as f32
        };
        let avg_eda = if eda_samples.is_empty() { 2.0 } else {
            eda_samples.iter().sum::<f32>() / eda_samples.len() as f32
        };

        // Valence: Higher HRV = more positive, lower stress
        let valence = if let Some(b) = &self.baseline {
            let hrv_ratio = avg_hrv / b.baseline_hrv;
            ((hrv_ratio - 1.0) * 2.0).clamp(-1.0, 1.0)
        } else {
            ((avg_hrv - 40.0) / 40.0).clamp(-1.0, 1.0)
        };

        // Arousal: Higher HR and EDA = higher arousal
        let arousal = if let Some(b) = &self.baseline {
            let hr_factor = (avg_hr - b.resting_hr) / 30.0;
            let eda_factor = (avg_eda - b.baseline_eda) / 5.0;
            ((hr_factor + eda_factor) / 2.0).clamp(0.0, 1.0)
        } else {
            let hr_factor = (avg_hr - 60.0) / 60.0;
            let eda_factor = (avg_eda - 1.0) / 10.0;
            ((hr_factor + eda_factor) / 2.0).clamp(0.0, 1.0)
        };

        // Dominance: Stable HRV = higher dominance (emotional regulation)
        let hrv_stability = if hrv_samples.len() > 1 {
            let mean = avg_hrv;
            let variance: f32 = hrv_samples.iter()
                .map(|v| (v - mean).powi(2))
                .sum::<f32>() / hrv_samples.len() as f32;
            1.0 - (variance.sqrt() / 20.0).min(1.0)
        } else {
            0.5
        };
        let dominance = hrv_stability;

        (valence, arousal, dominance)
    }

    /// Calculate confidence
    fn calculate_confidence(&self, samples: &[&BiometricSample]) -> f32 {
        let data_completeness = samples.iter()
            .filter(|s| s.heart_rate.is_some() && s.hrv_rmssd.is_some())
            .count() as f32 / samples.len() as f32;

        let sample_factor = (samples.len() as f32 / 30.0).min(1.0);
        let baseline_factor = if self.baseline.is_some() { 0.2 } else { 0.0 };

        (data_completeness * 0.4 + sample_factor * 0.4 + baseline_factor).min(0.85)
    }

    /// Set baseline from current data
    pub fn set_baseline(&mut self) {
        let samples = self.get_recent_samples();
        if samples.len() < 10 {
            return;
        }

        let hr: f32 = samples.iter()
            .filter_map(|s| s.heart_rate)
            .sum::<f32>() / samples.len() as f32;

        let hrv: f32 = samples.iter()
            .filter_map(|s| s.hrv_rmssd)
            .sum::<f32>() / samples.len() as f32;

        let temp: f32 = samples.iter()
            .filter_map(|s| s.skin_temperature)
            .sum::<f32>() / samples.len().max(1) as f32;

        let eda: f32 = samples.iter()
            .filter_map(|s| s.eda)
            .sum::<f32>() / samples.len().max(1) as f32;

        self.baseline = Some(BiometricBaseline {
            resting_hr: hr,
            baseline_hrv: hrv.max(20.0),
            baseline_temp: if temp > 0.0 { temp } else { 36.5 },
            baseline_eda: if eda > 0.0 { eda } else { 2.0 },
        });
    }

    /// Check if sensor has sufficient data
    pub fn has_sufficient_data(&self) -> bool {
        self.get_recent_samples().len() >= self.config.min_samples
    }
}

impl Default for WearableSensor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_sensor() {
        let mut sensor = KeyboardSensor::new();

        // Add keystroke samples
        for i in 0..30 {
            let now = Utc::now();
            sensor.add_keystroke(
                now,
                now + Duration::milliseconds(100),
                KeyCategory::Letter,
                None,
            );
        }

        assert!(sensor.has_sufficient_data());

        let emotion = sensor.infer_emotion().unwrap();
        assert!(emotion.confidence > 0.0);
    }

    #[test]
    fn test_wearable_sensor() {
        let mut sensor = WearableSensor::new();

        // Add biometric samples
        for i in 0..15 {
            sensor.add_sample(BiometricSample {
                timestamp: Utc::now(),
                heart_rate: Some(70.0 + i as f32),
                hrv_rmssd: Some(50.0),
                skin_temperature: Some(36.5),
                eda: Some(2.0),
                activity_level: Some(0.3),
            });
        }

        assert!(sensor.has_sufficient_data());

        let emotion = sensor.infer_emotion().unwrap();
        assert!(emotion.confidence > 0.0);
    }
}
