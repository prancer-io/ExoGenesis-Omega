//! Signal Analysis - Layer 1
//!
//! Real-time audio signal processing: FFT, beats, energy, spectral features.
//! This runs every audio frame with <10ms latency.

use std::collections::VecDeque;

/// Real-time signal features
#[derive(Debug, Clone)]
pub struct SignalFeatures {
    /// FFT spectrum (magnitude, 512 bins)
    pub spectrum: [f32; 512],

    /// Root mean square (overall loudness)
    pub rms: f32,

    /// Bass energy (20-250 Hz)
    pub bass: f32,

    /// Mid energy (250-4000 Hz)
    pub mid: f32,

    /// High energy (4000-20000 Hz)
    pub high: f32,

    /// Beat detected this frame
    pub is_beat: bool,

    /// Beat strength (0.0 - 1.0)
    pub beat_strength: f32,

    /// Onset detected (new sound started)
    pub is_onset: bool,

    /// Onset strength
    pub onset_strength: f32,

    /// Spectral centroid (brightness, Hz)
    pub spectral_centroid: f32,

    /// Spectral flux (rate of change)
    pub spectral_flux: f32,

    /// Zero crossing rate (noisiness)
    pub zero_crossing_rate: f32,

    /// Estimated instantaneous tempo (BPM)
    pub instant_tempo: f32,
}

impl Default for SignalFeatures {
    fn default() -> Self {
        Self {
            spectrum: [0.0; 512],
            rms: 0.0,
            bass: 0.0,
            mid: 0.0,
            high: 0.0,
            is_beat: false,
            beat_strength: 0.0,
            is_onset: false,
            onset_strength: 0.0,
            spectral_centroid: 1000.0,
            spectral_flux: 0.0,
            zero_crossing_rate: 0.0,
            instant_tempo: 120.0,
        }
    }
}

/// Real-time signal analyzer
pub struct SignalAnalyzer {
    /// Sample rate
    sample_rate: u32,

    /// FFT size
    fft_size: usize,

    /// Previous spectrum for flux calculation
    prev_spectrum: [f32; 512],

    /// Energy history for beat detection
    energy_history: VecDeque<f32>,

    /// Beat interval history for tempo estimation
    beat_times: VecDeque<f64>,

    /// Onset detection threshold
    onset_threshold: f32,

    /// Beat detection threshold multiplier
    beat_threshold_mult: f32,

    /// Current time
    time: f64,

    /// Samples since last beat
    samples_since_beat: usize,
}

impl SignalAnalyzer {
    /// Create new signal analyzer
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            fft_size: 1024,
            prev_spectrum: [0.0; 512],
            energy_history: VecDeque::with_capacity(43), // ~1 second at 43 fps
            beat_times: VecDeque::with_capacity(16),
            onset_threshold: 0.15,
            beat_threshold_mult: 1.4,
            time: 0.0,
            samples_since_beat: 0,
        }
    }

    /// Analyze audio samples
    pub fn analyze(&mut self, samples: &[f32]) -> SignalFeatures {
        let mut features = SignalFeatures::default();

        if samples.is_empty() {
            return features;
        }

        // Update time
        self.time += samples.len() as f64 / self.sample_rate as f64;
        self.samples_since_beat += samples.len();

        // RMS (loudness)
        features.rms = self.calculate_rms(samples);

        // Zero crossing rate
        features.zero_crossing_rate = self.calculate_zcr(samples);

        // FFT spectrum
        self.calculate_spectrum(samples, &mut features.spectrum);

        // Frequency bands
        self.calculate_bands(&features.spectrum, &mut features);

        // Spectral features
        features.spectral_centroid = self.calculate_spectral_centroid(&features.spectrum);
        features.spectral_flux = self.calculate_spectral_flux(&features.spectrum);

        // Beat detection
        self.detect_beat(&mut features);

        // Onset detection
        self.detect_onset(&mut features);

        // Update previous spectrum
        self.prev_spectrum = features.spectrum;

        // Tempo estimation
        features.instant_tempo = self.estimate_tempo();

        features
    }

    /// Calculate RMS energy
    fn calculate_rms(&self, samples: &[f32]) -> f32 {
        let sum: f32 = samples.iter().map(|s| s * s).sum();
        (sum / samples.len() as f32).sqrt()
    }

    /// Calculate zero crossing rate
    fn calculate_zcr(&self, samples: &[f32]) -> f32 {
        if samples.len() < 2 {
            return 0.0;
        }

        let crossings: usize = samples
            .windows(2)
            .filter(|w| (w[0] >= 0.0) != (w[1] >= 0.0))
            .count();

        crossings as f32 / (samples.len() - 1) as f32
    }

    /// Calculate FFT spectrum (simplified - real impl would use rustfft)
    fn calculate_spectrum(&self, samples: &[f32], spectrum: &mut [f32; 512]) {
        // Simplified: Use energy in frequency-sized windows
        // Real implementation would use FFT

        let samples_per_bin = samples.len() / 512;
        if samples_per_bin == 0 {
            return;
        }

        for (i, bin) in spectrum.iter_mut().enumerate() {
            let start = i * samples_per_bin;
            let end = (start + samples_per_bin).min(samples.len());

            if start < samples.len() {
                let energy: f32 = samples[start..end]
                    .iter()
                    .map(|s| s.abs())
                    .sum::<f32>()
                    / samples_per_bin as f32;

                *bin = energy;
            }
        }
    }

    /// Calculate frequency bands
    fn calculate_bands(&self, spectrum: &[f32; 512], features: &mut SignalFeatures) {
        // Assuming 44100 Hz sample rate, 1024 FFT
        // Each bin = 44100 / 1024 ≈ 43 Hz
        // Bass: 20-250 Hz ≈ bins 0-6
        // Mid: 250-4000 Hz ≈ bins 6-93
        // High: 4000-20000 Hz ≈ bins 93-465

        let bass_bins = &spectrum[0..7];
        let mid_bins = &spectrum[6..94];
        let high_bins = &spectrum[93..466.min(512)];

        features.bass = bass_bins.iter().sum::<f32>() / bass_bins.len() as f32;
        features.mid = mid_bins.iter().sum::<f32>() / mid_bins.len() as f32;
        features.high = high_bins.iter().sum::<f32>() / high_bins.len() as f32;

        // Normalize
        let max = features.bass.max(features.mid).max(features.high).max(0.001);
        features.bass /= max;
        features.mid /= max;
        features.high /= max;
    }

    /// Calculate spectral centroid (brightness)
    fn calculate_spectral_centroid(&self, spectrum: &[f32; 512]) -> f32 {
        let hz_per_bin = self.sample_rate as f32 / (self.fft_size as f32);

        let mut weighted_sum = 0.0;
        let mut magnitude_sum = 0.0;

        for (i, &mag) in spectrum.iter().enumerate() {
            let freq = i as f32 * hz_per_bin;
            weighted_sum += freq * mag;
            magnitude_sum += mag;
        }

        if magnitude_sum > 0.0 {
            weighted_sum / magnitude_sum
        } else {
            1000.0 // Default
        }
    }

    /// Calculate spectral flux (change from previous frame)
    fn calculate_spectral_flux(&self, spectrum: &[f32; 512]) -> f32 {
        let flux: f32 = spectrum
            .iter()
            .zip(self.prev_spectrum.iter())
            .map(|(curr, prev)| {
                let diff = curr - prev;
                if diff > 0.0 { diff * diff } else { 0.0 }
            })
            .sum();

        flux.sqrt()
    }

    /// Detect beats using energy threshold
    fn detect_beat(&mut self, features: &mut SignalFeatures) {
        // Add current energy to history
        self.energy_history.push_back(features.bass + features.rms);
        if self.energy_history.len() > 43 {
            self.energy_history.pop_front();
        }

        // Calculate threshold
        let avg_energy: f32 = self.energy_history.iter().sum::<f32>()
            / self.energy_history.len() as f32;
        let threshold = avg_energy * self.beat_threshold_mult;

        let current_energy = features.bass + features.rms;

        // Minimum time between beats (for 200 BPM max = 300ms)
        let min_samples = (self.sample_rate as f32 * 0.3) as usize;

        if current_energy > threshold && self.samples_since_beat > min_samples {
            features.is_beat = true;
            features.beat_strength = (current_energy / threshold - 1.0).min(1.0);

            // Record beat time
            self.beat_times.push_back(self.time);
            if self.beat_times.len() > 16 {
                self.beat_times.pop_front();
            }

            self.samples_since_beat = 0;
        }
    }

    /// Detect onsets (new sounds)
    fn detect_onset(&mut self, features: &mut SignalFeatures) {
        if features.spectral_flux > self.onset_threshold {
            features.is_onset = true;
            features.onset_strength = (features.spectral_flux / self.onset_threshold).min(1.0);
        }
    }

    /// Estimate tempo from beat intervals
    fn estimate_tempo(&self) -> f32 {
        if self.beat_times.len() < 4 {
            return 120.0; // Default
        }

        // Calculate intervals
        let intervals: Vec<f64> = self.beat_times
            .iter()
            .zip(self.beat_times.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();

        // Average interval
        let avg_interval: f64 = intervals.iter().sum::<f64>() / intervals.len() as f64;

        if avg_interval > 0.0 {
            (60.0 / avg_interval) as f32
        } else {
            120.0
        }
    }
}
