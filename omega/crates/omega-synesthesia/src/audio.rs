//! Audio Analysis Module
//!
//! Handles loading, decoding, and analyzing audio data using FFT.

use crate::{Result, SynesthesiaError};
use rustfft::{FftPlanner, num_complex::Complex};
use std::path::Path;

/// Source of audio data
#[derive(Debug, Clone)]
pub enum AudioSource {
    /// Load from file path
    File(String),
    /// Raw PCM samples
    Samples(Vec<f32>, u32),  // samples, sample_rate
    /// Generated test signal
    TestSignal(TestSignalType),
}

/// Types of test signals for development
#[derive(Debug, Clone)]
pub enum TestSignalType {
    /// Simple sine wave
    Sine { frequency: f32, duration: f32 },
    /// Multiple harmonics
    Harmonics { fundamental: f32, num_harmonics: usize, duration: f32 },
    /// Sweep from low to high frequency
    Sweep { start_freq: f32, end_freq: f32, duration: f32 },
    /// Simulated music with beats
    SimulatedMusic { bpm: f32, duration: f32 },
}

/// A single frame of audio data with spectral analysis
#[derive(Debug, Clone)]
pub struct AudioFrame {
    /// Timestamp in seconds
    pub timestamp: f64,
    /// Raw samples for this frame
    pub samples: Vec<f32>,
    /// RMS amplitude
    pub amplitude: f32,
    /// Peak amplitude
    pub peak: f32,
    /// Spectral data from FFT
    pub spectrum: SpectralData,
}

/// Spectral analysis data from FFT
#[derive(Debug, Clone)]
pub struct SpectralData {
    /// Frequency bins (Hz)
    pub frequencies: Vec<f32>,
    /// Magnitude at each frequency
    pub magnitudes: Vec<f32>,
    /// Phase at each frequency
    pub phases: Vec<f32>,
    /// Dominant frequency
    pub dominant_freq: f32,
    /// Spectral centroid (brightness)
    pub centroid: f32,
    /// Spectral spread
    pub spread: f32,
    /// Spectral flatness (noise vs tone)
    pub flatness: f32,
}

impl SpectralData {
    pub fn empty() -> Self {
        Self {
            frequencies: Vec::new(),
            magnitudes: Vec::new(),
            phases: Vec::new(),
            dominant_freq: 0.0,
            centroid: 0.0,
            spread: 0.0,
            flatness: 0.0,
        }
    }
}

/// Audio analyzer with FFT capabilities
pub struct AudioAnalyzer {
    /// Sample rate
    sample_rate: u32,
    /// FFT size
    fft_size: usize,
    /// Loaded samples
    samples: Vec<f32>,
    /// Precomputed frequency bins
    freq_bins: Vec<f32>,
    /// FFT planner
    fft_planner: FftPlanner<f32>,
}

impl AudioAnalyzer {
    /// Create a new audio analyzer
    pub fn new(sample_rate: u32, fft_size: usize) -> Self {
        // Precompute frequency bins
        let freq_bins: Vec<f32> = (0..fft_size / 2)
            .map(|i| i as f32 * sample_rate as f32 / fft_size as f32)
            .collect();

        Self {
            sample_rate,
            fft_size,
            samples: Vec::new(),
            freq_bins,
            fft_planner: FftPlanner::new(),
        }
    }

    /// Load audio from source
    pub fn load(&mut self, source: AudioSource) -> Result<()> {
        self.samples = match source {
            AudioSource::File(path) => self.load_file(&path)?,
            AudioSource::Samples(samples, _) => samples,
            AudioSource::TestSignal(signal_type) => self.generate_test_signal(signal_type),
        };

        Ok(())
    }

    /// Load audio from file
    fn load_file(&self, path: &str) -> Result<Vec<f32>> {
        let path = Path::new(path);

        if path.extension().map(|e| e == "wav").unwrap_or(false) {
            // Load WAV file
            let reader = hound::WavReader::open(path)
                .map_err(|e| SynesthesiaError::AudioLoadError(e.to_string()))?;

            let spec = reader.spec();
            let samples: Vec<f32> = match spec.sample_format {
                hound::SampleFormat::Int => {
                    let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
                    reader
                        .into_samples::<i32>()
                        .filter_map(|s| s.ok())
                        .map(|s| s as f32 / max_val)
                        .collect()
                }
                hound::SampleFormat::Float => {
                    reader
                        .into_samples::<f32>()
                        .filter_map(|s| s.ok())
                        .collect()
                }
            };

            Ok(samples)
        } else {
            Err(SynesthesiaError::AudioLoadError(
                format!("Unsupported audio format: {:?}", path.extension())
            ))
        }
    }

    /// Generate test signal
    fn generate_test_signal(&self, signal_type: TestSignalType) -> Vec<f32> {
        match signal_type {
            TestSignalType::Sine { frequency, duration } => {
                let num_samples = (duration * self.sample_rate as f32) as usize;
                (0..num_samples)
                    .map(|i| {
                        let t = i as f32 / self.sample_rate as f32;
                        (2.0 * std::f32::consts::PI * frequency * t).sin()
                    })
                    .collect()
            }
            TestSignalType::Harmonics { fundamental, num_harmonics, duration } => {
                let num_samples = (duration * self.sample_rate as f32) as usize;
                (0..num_samples)
                    .map(|i| {
                        let t = i as f32 / self.sample_rate as f32;
                        let mut sample = 0.0;
                        for h in 1..=num_harmonics {
                            let amplitude = 1.0 / h as f32;
                            sample += amplitude * (2.0 * std::f32::consts::PI * fundamental * h as f32 * t).sin();
                        }
                        sample / num_harmonics as f32
                    })
                    .collect()
            }
            TestSignalType::Sweep { start_freq, end_freq, duration } => {
                let num_samples = (duration * self.sample_rate as f32) as usize;
                (0..num_samples)
                    .map(|i| {
                        let t = i as f32 / self.sample_rate as f32;
                        let progress = t / duration;
                        let freq = start_freq + (end_freq - start_freq) * progress;
                        (2.0 * std::f32::consts::PI * freq * t).sin()
                    })
                    .collect()
            }
            TestSignalType::SimulatedMusic { bpm, duration } => {
                let num_samples = (duration * self.sample_rate as f32) as usize;
                let beat_period = 60.0 / bpm;

                (0..num_samples)
                    .map(|i| {
                        let t = i as f32 / self.sample_rate as f32;

                        // Bass drum on beats
                        let beat_phase = (t % beat_period) / beat_period;
                        let kick = if beat_phase < 0.1 {
                            (1.0 - beat_phase * 10.0) * (80.0 * 2.0 * std::f32::consts::PI * t).sin()
                        } else {
                            0.0
                        };

                        // Melody - simple arpeggio
                        let note_index = ((t / (beat_period / 4.0)) as usize) % 4;
                        let freqs = [261.63, 329.63, 392.0, 523.25]; // C4, E4, G4, C5
                        let melody = 0.3 * (freqs[note_index] * 2.0 * std::f32::consts::PI * t).sin();

                        // Combine
                        (kick * 0.6 + melody * 0.4).clamp(-1.0, 1.0)
                    })
                    .collect()
            }
        }
    }

    /// Get audio duration in seconds
    pub fn duration(&self) -> f64 {
        self.samples.len() as f64 / self.sample_rate as f64
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get total number of samples
    pub fn num_samples(&self) -> usize {
        self.samples.len()
    }

    /// Analyze a frame of audio at given timestamp
    pub fn analyze_frame(&mut self, timestamp: f64) -> Option<AudioFrame> {
        let sample_index = (timestamp * self.sample_rate as f64) as usize;

        if sample_index + self.fft_size > self.samples.len() {
            return None;
        }

        // Get samples for this frame
        let frame_samples: Vec<f32> = self.samples[sample_index..sample_index + self.fft_size]
            .to_vec();

        // Calculate amplitude
        let amplitude = (frame_samples.iter().map(|s| s * s).sum::<f32>() / frame_samples.len() as f32).sqrt();
        let peak = frame_samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);

        // Perform FFT
        let spectrum = self.compute_fft(&frame_samples);

        Some(AudioFrame {
            timestamp,
            samples: frame_samples,
            amplitude,
            peak,
            spectrum,
        })
    }

    /// Compute FFT and extract spectral features
    fn compute_fft(&mut self, samples: &[f32]) -> SpectralData {
        let fft = self.fft_planner.plan_fft_forward(self.fft_size);

        // Apply Hann window and convert to complex
        let mut buffer: Vec<Complex<f32>> = samples
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (self.fft_size - 1) as f32).cos());
                Complex::new(s * window, 0.0)
            })
            .collect();

        // Perform FFT
        fft.process(&mut buffer);

        // Extract magnitudes and phases (only first half - positive frequencies)
        let half_size = self.fft_size / 2;
        let magnitudes: Vec<f32> = buffer[..half_size]
            .iter()
            .map(|c| (c.re * c.re + c.im * c.im).sqrt() / half_size as f32)
            .collect();

        let phases: Vec<f32> = buffer[..half_size]
            .iter()
            .map(|c| c.im.atan2(c.re))
            .collect();

        // Find dominant frequency
        let (max_idx, _max_mag) = magnitudes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap_or((0, &0.0));

        let dominant_freq = self.freq_bins.get(max_idx).copied().unwrap_or(0.0);

        // Compute spectral centroid (brightness)
        let total_energy: f32 = magnitudes.iter().sum();
        let centroid = if total_energy > 0.0 {
            magnitudes
                .iter()
                .enumerate()
                .map(|(i, &m)| self.freq_bins.get(i).copied().unwrap_or(0.0) * m)
                .sum::<f32>()
                / total_energy
        } else {
            0.0
        };

        // Compute spectral spread
        let spread = if total_energy > 0.0 {
            (magnitudes
                .iter()
                .enumerate()
                .map(|(i, &m)| {
                    let f = self.freq_bins.get(i).copied().unwrap_or(0.0);
                    (f - centroid).powi(2) * m
                })
                .sum::<f32>()
                / total_energy)
                .sqrt()
        } else {
            0.0
        };

        // Compute spectral flatness (Wiener entropy)
        let geometric_mean = magnitudes
            .iter()
            .filter(|&&m| m > 1e-10)
            .map(|&m| m.ln())
            .sum::<f32>()
            / magnitudes.len() as f32;
        let geometric_mean = geometric_mean.exp();

        let arithmetic_mean = total_energy / magnitudes.len() as f32;

        let flatness = if arithmetic_mean > 1e-10 {
            (geometric_mean / arithmetic_mean).clamp(0.0, 1.0)
        } else {
            0.0
        };

        SpectralData {
            frequencies: self.freq_bins.clone(),
            magnitudes,
            phases,
            dominant_freq,
            centroid,
            spread,
            flatness,
        }
    }

    /// Analyze entire audio and return all frames
    pub fn analyze_all(&mut self, hop_size: usize) -> Vec<AudioFrame> {
        let mut frames = Vec::new();
        let hop_duration = hop_size as f64 / self.sample_rate as f64;

        let mut timestamp = 0.0;
        while let Some(frame) = self.analyze_frame(timestamp) {
            frames.push(frame);
            timestamp += hop_duration;
        }

        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = AudioAnalyzer::new(44100, 2048);
        assert_eq!(analyzer.sample_rate(), 44100);
    }

    #[test]
    fn test_test_signal_generation() {
        let mut analyzer = AudioAnalyzer::new(44100, 2048);
        analyzer.load(AudioSource::TestSignal(TestSignalType::Sine {
            frequency: 440.0,
            duration: 1.0,
        })).unwrap();

        assert_eq!(analyzer.num_samples(), 44100);
    }

    #[test]
    fn test_frame_analysis() {
        let mut analyzer = AudioAnalyzer::new(44100, 2048);
        analyzer.load(AudioSource::TestSignal(TestSignalType::Sine {
            frequency: 440.0,
            duration: 1.0,
        })).unwrap();

        let frame = analyzer.analyze_frame(0.0).unwrap();
        assert!(frame.amplitude > 0.0);
        assert!((frame.spectrum.dominant_freq - 440.0).abs() < 50.0);
    }
}
