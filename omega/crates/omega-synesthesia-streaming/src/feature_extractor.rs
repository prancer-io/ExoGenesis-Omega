//! Real-time feature extraction from audio stream

use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;
use crate::{Result, StreamError};

/// Real-time audio features extracted from streaming data
#[derive(Debug, Clone)]
pub struct StreamingFeatures {
    /// Spectral centroid (brightness) in Hz
    pub spectral_centroid: f32,

    /// RMS energy (loudness)
    pub rms_energy: f32,

    /// Zero crossing rate (noisiness)
    pub zero_crossing_rate: f32,

    /// Dominant frequency in Hz
    pub dominant_frequency: f32,

    /// Spectral flux (rate of change)
    pub spectral_flux: f32,

    /// Beat confidence (0.0 - 1.0)
    pub beat_confidence: f32,

    /// Current tempo estimate in BPM
    pub tempo_bpm: Option<f32>,

    /// Frequency spectrum (magnitude)
    pub spectrum: Vec<f32>,

    /// Timestamp in samples
    pub timestamp: u64,
}

impl Default for StreamingFeatures {
    fn default() -> Self {
        Self {
            spectral_centroid: 0.0,
            rms_energy: 0.0,
            zero_crossing_rate: 0.0,
            dominant_frequency: 0.0,
            spectral_flux: 0.0,
            beat_confidence: 0.0,
            tempo_bpm: None,
            spectrum: Vec::new(),
            timestamp: 0,
        }
    }
}

/// Real-time feature extractor
pub struct FeatureExtractor {
    sample_rate: u32,
    fft_size: usize,
    fft_planner: FftPlanner<f32>,
    window: Vec<f32>,
    previous_spectrum: Vec<f32>,
    sample_counter: u64,

    // Beat tracking state
    onset_history: Vec<(u64, f32)>,  // (timestamp, strength)
    tempo_estimates: Vec<f32>,
}

impl FeatureExtractor {
    /// Create a new feature extractor
    pub fn new(sample_rate: u32, fft_size: usize) -> Self {
        let mut fft_planner = FftPlanner::new();

        // Create Hann window
        let window: Vec<f32> = (0..fft_size)
            .map(|i| {
                let phase = 2.0 * PI * i as f32 / (fft_size - 1) as f32;
                0.5 * (1.0 - phase.cos())
            })
            .collect();

        Self {
            sample_rate,
            fft_size,
            fft_planner,
            window,
            previous_spectrum: vec![0.0; fft_size / 2],
            sample_counter: 0,
            onset_history: Vec::new(),
            tempo_estimates: Vec::new(),
        }
    }

    /// Extract features from audio chunk
    pub fn extract(&mut self, samples: &[f32]) -> Result<StreamingFeatures> {
        if samples.len() != self.fft_size {
            return Err(StreamError::FftError(
                format!("Expected {} samples, got {}", self.fft_size, samples.len())
            ));
        }

        // Apply window and convert to complex
        let mut complex_input: Vec<Complex<f32>> = samples
            .iter()
            .zip(self.window.iter())
            .map(|(s, w)| Complex::new(s * w, 0.0))
            .collect();

        // Perform FFT
        let fft = self.fft_planner.plan_fft_forward(self.fft_size);
        fft.process(&mut complex_input);

        // Calculate magnitude spectrum
        let spectrum: Vec<f32> = complex_input[..self.fft_size / 2]
            .iter()
            .map(|c| c.norm())
            .collect();

        // Calculate features
        let spectral_centroid = Self::calculate_spectral_centroid(&spectrum, self.sample_rate);
        let rms_energy = Self::calculate_rms(samples);
        let zero_crossing_rate = Self::calculate_zcr(samples);
        let dominant_frequency = Self::find_dominant_frequency(&spectrum, self.sample_rate);
        let spectral_flux = Self::calculate_spectral_flux(&spectrum, &self.previous_spectrum);

        // Detect onset (beat/attack)
        let onset_strength = spectral_flux * rms_energy;
        let is_onset = self.detect_onset(onset_strength);

        // Update beat tracking
        let (beat_confidence, tempo_bpm) = if is_onset {
            self.onset_history.push((self.sample_counter, onset_strength));
            self.update_tempo_estimate()
        } else {
            (0.0, self.tempo_estimates.last().copied())
        };

        // Update state
        self.previous_spectrum = spectrum.clone();
        self.sample_counter += samples.len() as u64;

        Ok(StreamingFeatures {
            spectral_centroid,
            rms_energy,
            zero_crossing_rate,
            dominant_frequency,
            spectral_flux,
            beat_confidence,
            tempo_bpm,
            spectrum,
            timestamp: self.sample_counter,
        })
    }

    /// Calculate spectral centroid (brightness)
    fn calculate_spectral_centroid(spectrum: &[f32], sample_rate: u32) -> f32 {
        let sum_weighted: f32 = spectrum
            .iter()
            .enumerate()
            .map(|(i, mag)| i as f32 * mag)
            .sum();

        let sum_magnitudes: f32 = spectrum.iter().sum();

        if sum_magnitudes > 0.0 {
            let bin_width = sample_rate as f32 / (2.0 * spectrum.len() as f32);
            (sum_weighted / sum_magnitudes) * bin_width
        } else {
            0.0
        }
    }

    /// Calculate RMS energy
    fn calculate_rms(samples: &[f32]) -> f32 {
        let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
        (sum_squares / samples.len() as f32).sqrt()
    }

    /// Calculate zero crossing rate
    fn calculate_zcr(samples: &[f32]) -> f32 {
        let crossings = samples
            .windows(2)
            .filter(|w| (w[0] >= 0.0 && w[1] < 0.0) || (w[0] < 0.0 && w[1] >= 0.0))
            .count();

        crossings as f32 / samples.len() as f32
    }

    /// Find dominant frequency
    fn find_dominant_frequency(spectrum: &[f32], sample_rate: u32) -> f32 {
        let max_idx = spectrum
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let bin_width = sample_rate as f32 / (2.0 * spectrum.len() as f32);
        max_idx as f32 * bin_width
    }

    /// Calculate spectral flux (rate of change)
    fn calculate_spectral_flux(current: &[f32], previous: &[f32]) -> f32 {
        current
            .iter()
            .zip(previous.iter())
            .map(|(c, p)| (c - p).max(0.0).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Detect onset using adaptive threshold
    fn detect_onset(&self, onset_strength: f32) -> bool {
        if self.onset_history.len() < 10 {
            return onset_strength > 0.1;  // Initial threshold
        }

        // Calculate adaptive threshold from recent history
        let recent_onsets: Vec<f32> = self.onset_history
            .iter()
            .rev()
            .take(50)
            .map(|(_, strength)| *strength)
            .collect();

        let mean: f32 = recent_onsets.iter().sum::<f32>() / recent_onsets.len() as f32;
        let threshold = mean * 1.5;

        onset_strength > threshold
    }

    /// Update tempo estimate using onset intervals
    fn update_tempo_estimate(&mut self) -> (f32, Option<f32>) {
        if self.onset_history.len() < 2 {
            return (0.0, None);
        }

        // Look at recent onset intervals
        let recent_intervals: Vec<f32> = self.onset_history
            .windows(2)
            .rev()
            .take(8)
            .map(|w| {
                let (t1, _) = w[0];
                let (t2, _) = w[1];
                (t1 - t2) as f32 / self.sample_rate as f32
            })
            .collect();

        if recent_intervals.is_empty() {
            return (0.0, None);
        }

        // Calculate median interval
        let mut sorted = recent_intervals.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_interval = sorted[sorted.len() / 2];

        // Convert to BPM
        if median_interval > 0.0 {
            let bpm = 60.0 / median_interval;

            // Valid tempo range: 60-180 BPM
            if (60.0..=180.0).contains(&bpm) {
                self.tempo_estimates.push(bpm);
                if self.tempo_estimates.len() > 10 {
                    self.tempo_estimates.remove(0);
                }

                // Calculate confidence based on consistency
                let tempo_std: f32 = self.calculate_std(&self.tempo_estimates);
                let confidence = (1.0 - (tempo_std / 20.0).min(1.0)).max(0.0);

                return (confidence, Some(bpm));
            }
        }

        (0.0, None)
    }

    /// Calculate standard deviation
    fn calculate_std(&self, values: &[f32]) -> f32 {
        if values.is_empty() {
            return 0.0;
        }

        let mean: f32 = values.iter().sum::<f32>() / values.len() as f32;
        let variance: f32 = values
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f32>() / values.len() as f32;

        variance.sqrt()
    }

    /// Reset extractor state
    pub fn reset(&mut self) {
        self.previous_spectrum = vec![0.0; self.fft_size / 2];
        self.sample_counter = 0;
        self.onset_history.clear();
        self.tempo_estimates.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_extraction() {
        let mut extractor = FeatureExtractor::new(44100, 512);

        // Generate test signal (440 Hz sine wave)
        let samples: Vec<f32> = (0..512)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 44100.0).sin())
            .collect();

        let features = extractor.extract(&samples).unwrap();

        // Dominant frequency should be around 440 Hz
        assert!((features.dominant_frequency - 440.0).abs() < 100.0);

        // RMS should be non-zero
        assert!(features.rms_energy > 0.0);

        // Spectrum should be populated
        assert!(!features.spectrum.is_empty());
    }

    #[test]
    fn test_rms_calculation() {
        let samples = vec![0.5, -0.5, 0.5, -0.5];
        let rms = FeatureExtractor::calculate_rms(&samples);
        assert!((rms - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_zcr_calculation() {
        let samples = vec![1.0, -1.0, 1.0, -1.0, 1.0];
        let zcr = FeatureExtractor::calculate_zcr(&samples);
        assert!(zcr > 0.5);  // High zero crossing rate
    }

    #[test]
    fn test_spectral_flux() {
        let spectrum1 = vec![1.0, 2.0, 3.0];
        let spectrum2 = vec![2.0, 3.0, 4.0];
        let flux = FeatureExtractor::calculate_spectral_flux(&spectrum2, &spectrum1);
        assert!(flux > 0.0);  // Spectrum changed
    }
}
