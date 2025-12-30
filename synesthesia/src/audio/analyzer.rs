//! Audio Feature Analyzer
//!
//! Extracts audio features for visualization: beats, frequencies, energy.

use std::collections::VecDeque;

/// Audio features extracted from audio stream
#[derive(Debug, Clone, Default)]
pub struct AudioFeatures {
    /// Overall loudness (RMS) 0-1
    pub rms: f32,

    /// Low frequency energy (bass) 0-1
    pub bass: f32,

    /// Mid frequency energy 0-1
    pub mid: f32,

    /// High frequency energy 0-1
    pub high: f32,

    /// Beat detected this frame
    pub is_beat: bool,

    /// Beat intensity 0-1
    pub beat_intensity: f32,

    /// Estimated BPM
    pub bpm: f32,

    /// Spectral centroid (brightness)
    pub spectral_centroid: f32,

    /// Spectral flux (change rate)
    pub spectral_flux: f32,

    /// 32-band frequency spectrum
    pub frequency_bands: [f32; 32],

    /// Current playback time in seconds
    pub time: f64,
}

/// Real-time audio analyzer
pub struct AudioAnalyzer {
    sample_rate: u32,
    buffer: VecDeque<f32>,
    prev_spectrum: [f32; 32],
    energy_history: VecDeque<f32>,
    beat_threshold: f32,
    last_beat_time: f64,
}

impl AudioAnalyzer {
    /// Create a new audio analyzer
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            buffer: VecDeque::with_capacity(4096),
            prev_spectrum: [0.0; 32],
            energy_history: VecDeque::with_capacity(64),
            beat_threshold: 1.3,
            last_beat_time: 0.0,
        }
    }

    /// Analyze a chunk of audio samples
    pub fn analyze(&mut self, samples: &[f32], current_time: f64) -> AudioFeatures {
        // Add samples to buffer
        self.buffer.extend(samples.iter());

        // Keep buffer manageable
        while self.buffer.len() > 8192 {
            self.buffer.pop_front();
        }

        if self.buffer.len() < 2048 {
            return AudioFeatures::default();
        }

        // Calculate RMS
        let rms = self.calculate_rms();

        // Calculate frequency bands using simple DFT
        let bands = self.calculate_frequency_bands();

        // Extract bass, mid, high
        let bass = bands[0..4].iter().sum::<f32>() / 4.0;
        let mid = bands[8..16].iter().sum::<f32>() / 8.0;
        let high = bands[20..32].iter().sum::<f32>() / 12.0;

        // Spectral centroid
        let spectral_centroid = self.calculate_spectral_centroid(&bands);

        // Spectral flux
        let spectral_flux = self.calculate_spectral_flux(&bands);

        // Beat detection
        let (is_beat, beat_intensity) = self.detect_beat(bass + rms, current_time);

        // Update previous spectrum
        self.prev_spectrum = bands;

        AudioFeatures {
            rms,
            bass,
            mid,
            high,
            is_beat,
            beat_intensity,
            bpm: 120.0, // TODO: Implement BPM detection
            spectral_centroid,
            spectral_flux,
            frequency_bands: bands,
            time: current_time,
        }
    }

    fn calculate_rms(&self) -> f32 {
        let sum: f32 = self.buffer.iter().take(2048).map(|s| s * s).sum();
        (sum / 2048.0).sqrt().min(1.0)
    }

    fn calculate_frequency_bands(&self) -> [f32; 32] {
        let mut bands = [0.0f32; 32];
        let samples: Vec<f32> = self.buffer.iter().take(2048).copied().collect();

        // Simple energy-based frequency band estimation
        // In production, use proper FFT
        for (i, band) in bands.iter_mut().enumerate() {
            let freq_low = (i as f32 / 32.0 * 20000.0) as usize;
            let freq_high = ((i + 1) as f32 / 32.0 * 20000.0) as usize;

            // Approximate energy in this band
            let bin_start = freq_low * 2048 / self.sample_rate as usize;
            let bin_end = (freq_high * 2048 / self.sample_rate as usize).min(1024);

            if bin_start < bin_end && bin_end <= samples.len() / 2 {
                let energy: f32 = samples[bin_start..bin_end]
                    .iter()
                    .map(|s| s.abs())
                    .sum();
                *band = (energy / (bin_end - bin_start) as f32).min(1.0);
            }
        }

        bands
    }

    fn calculate_spectral_centroid(&self, bands: &[f32; 32]) -> f32 {
        let weighted_sum: f32 = bands.iter().enumerate()
            .map(|(i, &b)| i as f32 * b)
            .sum();
        let total: f32 = bands.iter().sum();

        if total > 0.0 {
            weighted_sum / total / 32.0
        } else {
            0.5
        }
    }

    fn calculate_spectral_flux(&self, bands: &[f32; 32]) -> f32 {
        let flux: f32 = bands.iter()
            .zip(self.prev_spectrum.iter())
            .map(|(&curr, &prev)| {
                let diff = curr - prev;
                if diff > 0.0 { diff } else { 0.0 }
            })
            .sum();

        (flux / 32.0).min(1.0)
    }

    fn detect_beat(&mut self, energy: f32, current_time: f64) -> (bool, f32) {
        self.energy_history.push_back(energy);
        if self.energy_history.len() > 64 {
            self.energy_history.pop_front();
        }

        let avg_energy: f32 = self.energy_history.iter().sum::<f32>()
            / self.energy_history.len() as f32;

        let is_beat = energy > avg_energy * self.beat_threshold
            && current_time - self.last_beat_time > 0.1;

        let beat_intensity = if is_beat {
            self.last_beat_time = current_time;
            ((energy / avg_energy - 1.0) / 0.5).min(1.0).max(0.0)
        } else {
            0.0
        };

        (is_beat, beat_intensity)
    }
}
