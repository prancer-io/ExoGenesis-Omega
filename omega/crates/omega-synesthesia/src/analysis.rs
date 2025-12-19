//! Advanced Audio Analysis Module
//!
//! Real onset detection, beat tracking, and tempo estimation.

use crate::audio::SpectralData;

/// Onset detection using spectral flux
#[derive(Debug, Clone)]
pub struct OnsetDetector {
    /// Previous spectrum for flux calculation
    prev_spectrum: Vec<f32>,
    /// Onset threshold multiplier
    pub threshold_multiplier: f32,
    /// Window size for adaptive threshold
    pub window_size: usize,
    /// Recent onset strengths for adaptive threshold
    onset_history: Vec<f32>,
    /// Minimum time between onsets (samples)
    pub min_onset_interval: usize,
    /// Last onset sample index
    last_onset: usize,
}

impl OnsetDetector {
    pub fn new() -> Self {
        Self {
            prev_spectrum: Vec::new(),
            threshold_multiplier: 1.5,
            window_size: 10,
            onset_history: Vec::new(),
            min_onset_interval: 4410, // ~100ms at 44.1kHz
            last_onset: 0,
        }
    }

    /// Calculate spectral flux between current and previous spectrum
    pub fn spectral_flux(&mut self, spectrum: &SpectralData) -> f32 {
        if self.prev_spectrum.is_empty() {
            self.prev_spectrum = spectrum.magnitudes.clone();
            return 0.0;
        }

        // Half-wave rectified spectral flux
        let flux: f32 = spectrum.magnitudes.iter()
            .zip(self.prev_spectrum.iter())
            .map(|(&curr, &prev)| {
                let diff = curr - prev;
                if diff > 0.0 { diff * diff } else { 0.0 }
            })
            .sum();

        self.prev_spectrum = spectrum.magnitudes.clone();
        flux.sqrt()
    }

    /// Detect if current frame is an onset
    pub fn detect(&mut self, spectrum: &SpectralData, sample_index: usize) -> OnsetResult {
        let flux = self.spectral_flux(spectrum);

        // Update history
        self.onset_history.push(flux);
        if self.onset_history.len() > self.window_size {
            self.onset_history.remove(0);
        }

        // Calculate adaptive threshold
        let mean = self.onset_history.iter().sum::<f32>() / self.onset_history.len() as f32;
        let std_dev = (self.onset_history.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / self.onset_history.len() as f32)
            .sqrt();

        let threshold = mean + self.threshold_multiplier * std_dev;

        // Check if onset (above threshold and minimum interval passed)
        let is_onset = flux > threshold &&
            (sample_index - self.last_onset) >= self.min_onset_interval &&
            flux > 0.001;

        if is_onset {
            self.last_onset = sample_index;
        }

        OnsetResult {
            flux,
            threshold,
            is_onset,
            strength: if is_onset { (flux - threshold) / threshold } else { 0.0 },
        }
    }

    /// Reset detector state
    pub fn reset(&mut self) {
        self.prev_spectrum.clear();
        self.onset_history.clear();
        self.last_onset = 0;
    }
}

impl Default for OnsetDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of onset detection
#[derive(Debug, Clone, Copy)]
pub struct OnsetResult {
    /// Spectral flux value
    pub flux: f32,
    /// Current adaptive threshold
    pub threshold: f32,
    /// Is this an onset?
    pub is_onset: bool,
    /// Onset strength (0-1)
    pub strength: f32,
}

/// Beat tracker using auto-correlation tempo estimation
#[derive(Debug)]
pub struct BeatTracker {
    /// Onset buffer for tempo estimation
    onset_buffer: Vec<f32>,
    /// Buffer size in frames
    buffer_size: usize,
    /// Sample rate
    sample_rate: u32,
    /// Hop size between frames
    hop_size: usize,
    /// Current tempo estimate (BPM)
    tempo: f32,
    /// Tempo confidence (0-1)
    confidence: f32,
    /// Phase accumulator
    phase: f32,
    /// Beat counter
    beat_count: usize,
    /// Frames since last beat
    frames_since_beat: usize,
}

impl BeatTracker {
    pub fn new(sample_rate: u32, hop_size: usize) -> Self {
        // ~8 seconds of onset data for tempo estimation
        let buffer_size = (sample_rate as usize * 8) / hop_size;

        Self {
            onset_buffer: Vec::with_capacity(buffer_size),
            buffer_size,
            sample_rate,
            hop_size,
            tempo: 120.0, // Default tempo
            confidence: 0.0,
            phase: 0.0,
            beat_count: 0,
            frames_since_beat: 0,
        }
    }

    /// Process onset strength and update beat tracking
    pub fn process(&mut self, onset_strength: f32) -> BeatResult {
        // Add to onset buffer
        self.onset_buffer.push(onset_strength);
        if self.onset_buffer.len() > self.buffer_size {
            self.onset_buffer.remove(0);
        }

        // Periodically re-estimate tempo
        if self.onset_buffer.len() >= self.buffer_size && self.frames_since_beat % 100 == 0 {
            self.estimate_tempo();
        }

        // Track beat phase
        let frames_per_beat = (60.0 * self.sample_rate as f32) / (self.tempo * self.hop_size as f32);
        self.phase += 1.0 / frames_per_beat;

        let is_beat = self.phase >= 1.0;
        if is_beat {
            self.phase -= 1.0;
            self.beat_count += 1;
            self.frames_since_beat = 0;
        } else {
            self.frames_since_beat += 1;
        }

        BeatResult {
            tempo: self.tempo,
            confidence: self.confidence,
            phase: self.phase,
            is_beat,
            beat_number: self.beat_count,
        }
    }

    /// Estimate tempo using auto-correlation
    fn estimate_tempo(&mut self) {
        if self.onset_buffer.len() < 100 {
            return;
        }

        // Auto-correlation for tempo estimation
        let min_lag = (60.0 * self.sample_rate as f32 / (200.0 * self.hop_size as f32)) as usize; // 200 BPM max
        let max_lag = (60.0 * self.sample_rate as f32 / (60.0 * self.hop_size as f32)) as usize;  // 60 BPM min

        let min_lag = min_lag.max(1);
        let max_lag = max_lag.min(self.onset_buffer.len() / 2);

        let mut best_lag = min_lag;
        let mut best_corr = 0.0f32;

        for lag in min_lag..max_lag {
            let mut corr = 0.0f32;
            let n = self.onset_buffer.len() - lag;

            for i in 0..n {
                corr += self.onset_buffer[i] * self.onset_buffer[i + lag];
            }
            corr /= n as f32;

            if corr > best_corr {
                best_corr = corr;
                best_lag = lag;
            }
        }

        // Convert lag to BPM
        let new_tempo = 60.0 * self.sample_rate as f32 / (best_lag as f32 * self.hop_size as f32);

        // Smooth tempo estimate
        if self.confidence > 0.0 {
            // Weighted average with previous estimate
            self.tempo = self.tempo * 0.7 + new_tempo * 0.3;
        } else {
            self.tempo = new_tempo;
        }

        // Calculate confidence based on correlation strength
        let max_possible_corr = self.onset_buffer.iter().map(|x| x * x).sum::<f32>() / self.onset_buffer.len() as f32;
        self.confidence = (best_corr / max_possible_corr.max(0.001)).clamp(0.0, 1.0);
    }

    /// Get current tempo estimate
    pub fn tempo(&self) -> f32 {
        self.tempo
    }

    /// Get tempo confidence
    pub fn confidence(&self) -> f32 {
        self.confidence
    }

    /// Reset tracker
    pub fn reset(&mut self) {
        self.onset_buffer.clear();
        self.tempo = 120.0;
        self.confidence = 0.0;
        self.phase = 0.0;
        self.beat_count = 0;
    }
}

/// Result of beat tracking
#[derive(Debug, Clone, Copy)]
pub struct BeatResult {
    /// Estimated tempo in BPM
    pub tempo: f32,
    /// Confidence in tempo estimate (0-1)
    pub confidence: f32,
    /// Current beat phase (0-1)
    pub phase: f32,
    /// Is this frame on a beat?
    pub is_beat: bool,
    /// Beat number since start
    pub beat_number: usize,
}

/// Chromagram calculator for harmony analysis
#[derive(Debug)]
pub struct ChromaAnalyzer {
    /// Reference frequency for A4
    reference_freq: f32,
    /// Number of bins per octave
    _bins_per_octave: usize,  // Reserved for chromagram refinement
    /// Frequency range
    min_freq: f32,
    max_freq: f32,
}

impl ChromaAnalyzer {
    pub fn new() -> Self {
        Self {
            reference_freq: 440.0,
            _bins_per_octave: 12,
            min_freq: 65.0,   // C2
            max_freq: 2100.0, // C7
        }
    }

    /// Compute chromagram from spectrum
    pub fn compute(&self, spectrum: &SpectralData) -> [f32; 12] {
        let mut chroma = [0.0f32; 12];
        let mut weights = [0.0f32; 12];

        for (i, &mag) in spectrum.magnitudes.iter().enumerate() {
            let freq = spectrum.frequencies.get(i).copied().unwrap_or(0.0);

            if freq >= self.min_freq && freq <= self.max_freq && mag > 0.001 {
                // Convert frequency to pitch class
                let midi_note = 12.0 * (freq / self.reference_freq).log2() + 69.0;
                let pitch_class = ((midi_note.round() as i32 % 12) + 12) % 12;

                // Weight by magnitude
                chroma[pitch_class as usize] += mag;
                weights[pitch_class as usize] += 1.0;
            }
        }

        // Normalize
        let max_val = chroma.iter().cloned().fold(0.0f32, f32::max);
        if max_val > 0.0 {
            for c in &mut chroma {
                *c /= max_val;
            }
        }

        chroma
    }

    /// Estimate key from chromagram
    pub fn estimate_key(&self, chroma: &[f32; 12]) -> KeyEstimate {
        // Major and minor key profiles (Krumhansl-Schmuckler)
        let major_profile = [6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88];
        let minor_profile = [6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17];

        let mut best_key = 0;
        let mut best_mode = KeyMode::Major;
        let mut best_corr = f32::MIN;

        // Try all 24 keys (12 major + 12 minor)
        for root in 0..12 {
            // Rotate chroma to align with key
            let rotated: Vec<f32> = (0..12).map(|i| chroma[(i + root) % 12]).collect();

            // Correlate with major profile
            let major_corr: f32 = rotated.iter()
                .zip(major_profile.iter())
                .map(|(a, b)| a * (*b as f32))
                .sum();

            if major_corr > best_corr {
                best_corr = major_corr;
                best_key = root;
                best_mode = KeyMode::Major;
            }

            // Correlate with minor profile
            let minor_corr: f32 = rotated.iter()
                .zip(minor_profile.iter())
                .map(|(a, b)| a * (*b as f32))
                .sum();

            if minor_corr > best_corr {
                best_corr = minor_corr;
                best_key = root;
                best_mode = KeyMode::Minor;
            }
        }

        // Normalize correlation to confidence
        let confidence = (best_corr / 50.0).clamp(0.0, 1.0);

        KeyEstimate {
            root: best_key as u8,
            mode: best_mode,
            confidence,
        }
    }
}

impl Default for ChromaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Key mode (major/minor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyMode {
    Major,
    Minor,
}

/// Key estimation result
#[derive(Debug, Clone, Copy)]
pub struct KeyEstimate {
    /// Root note (0-11, C=0)
    pub root: u8,
    /// Major or minor
    pub mode: KeyMode,
    /// Confidence (0-1)
    pub confidence: f32,
}

impl KeyEstimate {
    /// Get key name
    pub fn name(&self) -> String {
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let mode_name = match self.mode {
            KeyMode::Major => "Major",
            KeyMode::Minor => "Minor",
        };
        format!("{} {}", note_names[self.root as usize], mode_name)
    }
}

/// MFCC (Mel-Frequency Cepstral Coefficients) calculator
#[derive(Debug)]
pub struct MfccCalculator {
    /// Number of mel bands
    num_mel_bands: usize,
    /// Number of MFCCs to return
    num_coefficients: usize,
    /// Precomputed mel filterbank
    mel_filterbank: Vec<Vec<f32>>,
    /// Sample rate
    _sample_rate: u32,  // Reserved for mel filterbank scaling
    /// FFT size
    _fft_size: usize,  // Reserved for frequency resolution
}

impl MfccCalculator {
    pub fn new(sample_rate: u32, fft_size: usize, num_mel_bands: usize, num_coefficients: usize) -> Self {
        let mel_filterbank = Self::create_mel_filterbank(sample_rate, fft_size, num_mel_bands);

        Self {
            num_mel_bands,
            num_coefficients,
            mel_filterbank,
            _sample_rate: sample_rate,
            _fft_size: fft_size,
        }
    }

    /// Create mel filterbank
    fn create_mel_filterbank(sample_rate: u32, fft_size: usize, num_bands: usize) -> Vec<Vec<f32>> {
        let num_bins = fft_size / 2;
        let max_freq = sample_rate as f32 / 2.0;

        // Convert to mel scale
        let mel_low = Self::hz_to_mel(0.0);
        let mel_high = Self::hz_to_mel(max_freq);

        // Create mel points
        let mel_points: Vec<f32> = (0..=num_bands + 1)
            .map(|i| mel_low + (mel_high - mel_low) * i as f32 / (num_bands + 1) as f32)
            .collect();

        // Convert back to Hz
        let hz_points: Vec<f32> = mel_points.iter().map(|&m| Self::mel_to_hz(m)).collect();

        // Convert to bin indices
        let bin_points: Vec<usize> = hz_points.iter()
            .map(|&f| ((f / max_freq) * num_bins as f32) as usize)
            .collect();

        // Create filterbank
        let mut filterbank = vec![vec![0.0f32; num_bins]; num_bands];

        for i in 0..num_bands {
            let start = bin_points[i];
            let center = bin_points[i + 1];
            let end = bin_points[i + 2];

            // Rising slope
            for j in start..center {
                if center > start {
                    filterbank[i][j] = (j - start) as f32 / (center - start) as f32;
                }
            }

            // Falling slope
            for j in center..end {
                if end > center {
                    filterbank[i][j] = (end - j) as f32 / (end - center) as f32;
                }
            }
        }

        filterbank
    }

    /// Convert Hz to mel scale
    fn hz_to_mel(hz: f32) -> f32 {
        2595.0 * (1.0 + hz / 700.0).log10()
    }

    /// Convert mel to Hz
    fn mel_to_hz(mel: f32) -> f32 {
        700.0 * (10.0_f32.powf(mel / 2595.0) - 1.0)
    }

    /// Compute MFCCs from spectrum
    pub fn compute(&self, spectrum: &SpectralData) -> Vec<f32> {
        // Apply mel filterbank
        let mut mel_energies = vec![0.0f32; self.num_mel_bands];

        for (i, filter) in self.mel_filterbank.iter().enumerate() {
            let energy: f32 = spectrum.magnitudes.iter()
                .zip(filter.iter())
                .map(|(m, f)| m * m * f)
                .sum();

            mel_energies[i] = (energy + 1e-10).ln();
        }

        // Apply DCT (Type-II)
        let mut mfcc = vec![0.0f32; self.num_coefficients];
        let n = self.num_mel_bands as f32;

        for k in 0..self.num_coefficients {
            let mut sum = 0.0f32;
            for (i, &energy) in mel_energies.iter().enumerate() {
                sum += energy * (std::f32::consts::PI * k as f32 * (i as f32 + 0.5) / n).cos();
            }
            mfcc[k] = sum * (2.0 / n).sqrt();
        }

        mfcc
    }
}

/// Spectral contrast analyzer for timbre
#[derive(Debug)]
pub struct SpectralContrastAnalyzer {
    /// Number of subbands
    num_bands: usize,
    /// Neighborhood size for peak/valley detection
    neighborhood: f32,
}

impl SpectralContrastAnalyzer {
    pub fn new(num_bands: usize) -> Self {
        Self {
            num_bands,
            neighborhood: 0.2,
        }
    }

    /// Compute spectral contrast
    pub fn compute(&self, spectrum: &SpectralData) -> SpectralContrast {
        let num_bins = spectrum.magnitudes.len();
        let bins_per_band = num_bins / self.num_bands;

        let mut peaks = vec![0.0f32; self.num_bands];
        let mut valleys = vec![0.0f32; self.num_bands];

        for band in 0..self.num_bands {
            let start = band * bins_per_band;
            let end = (band + 1) * bins_per_band;

            let mut band_mags: Vec<f32> = spectrum.magnitudes[start..end].to_vec();
            band_mags.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let neighborhood_size = (band_mags.len() as f32 * self.neighborhood) as usize;
            let neighborhood_size = neighborhood_size.max(1);

            // Peak is average of top values
            peaks[band] = band_mags[band_mags.len() - neighborhood_size..]
                .iter()
                .sum::<f32>() / neighborhood_size as f32;

            // Valley is average of bottom values
            valleys[band] = band_mags[..neighborhood_size]
                .iter()
                .sum::<f32>() / neighborhood_size as f32;
        }

        // Compute contrast
        let contrast: Vec<f32> = peaks.iter()
            .zip(valleys.iter())
            .map(|(p, v)| (p - v).max(0.0))
            .collect();

        SpectralContrast {
            peaks,
            valleys,
            contrast,
        }
    }
}

/// Spectral contrast result
#[derive(Debug, Clone)]
pub struct SpectralContrast {
    /// Peak values per band
    pub peaks: Vec<f32>,
    /// Valley values per band
    pub valleys: Vec<f32>,
    /// Contrast (peak - valley) per band
    pub contrast: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onset_detector() {
        let mut detector = OnsetDetector::new();
        let spectrum = SpectralData {
            frequencies: vec![0.0; 512],
            magnitudes: vec![0.1; 512],
            phases: vec![0.0; 512],
            dominant_freq: 440.0,
            centroid: 1000.0,
            spread: 500.0,
            flatness: 0.5,
        };

        let result = detector.detect(&spectrum, 0);
        assert!(result.flux >= 0.0);
    }

    #[test]
    fn test_beat_tracker() {
        let mut tracker = BeatTracker::new(44100, 512);

        for _ in 0..100 {
            let result = tracker.process(0.5);
            assert!(result.tempo > 0.0);
        }
    }

    #[test]
    fn test_chroma_analyzer() {
        let analyzer = ChromaAnalyzer::new();
        let chroma = [1.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.8, 0.0, 0.0, 0.0, 0.0]; // C major chord

        let key = analyzer.estimate_key(&chroma);
        assert_eq!(key.root, 0); // C
    }

    #[test]
    fn test_mfcc_calculator() {
        let calc = MfccCalculator::new(44100, 2048, 26, 13);
        let spectrum = SpectralData {
            frequencies: (0..1024).map(|i| i as f32 * 21.5).collect(),
            magnitudes: vec![0.1; 1024],
            phases: vec![0.0; 1024],
            dominant_freq: 440.0,
            centroid: 1000.0,
            spread: 500.0,
            flatness: 0.5,
        };

        let mfcc = calc.compute(&spectrum);
        assert_eq!(mfcc.len(), 13);
    }
}
