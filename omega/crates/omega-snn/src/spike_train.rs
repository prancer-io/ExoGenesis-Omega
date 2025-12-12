//! Spike Train Recording and Analysis
//!
//! Tools for recording, analyzing, and processing spike trains.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::neuron::NeuronId;

/// A single spike event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spike {
    /// Neuron that fired
    pub neuron_id: NeuronId,
    /// Time of spike
    pub time: Duration,
}

impl Spike {
    pub fn new(neuron_id: NeuronId, time: Duration) -> Self {
        Self { neuron_id, time }
    }
}

/// A train of spikes from a single neuron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeTrain {
    /// Neuron ID
    pub neuron_id: NeuronId,
    /// Spike times (sorted)
    pub times: Vec<Duration>,
}

impl SpikeTrain {
    pub fn new(neuron_id: NeuronId) -> Self {
        Self {
            neuron_id,
            times: Vec::new(),
        }
    }

    pub fn from_spikes(neuron_id: NeuronId, mut times: Vec<Duration>) -> Self {
        times.sort();
        Self { neuron_id, times }
    }

    /// Add a spike
    pub fn add_spike(&mut self, time: Duration) {
        // Maintain sorted order
        match self.times.binary_search(&time) {
            Ok(_) => {} // Already exists
            Err(pos) => self.times.insert(pos, time),
        }
    }

    /// Get number of spikes
    pub fn spike_count(&self) -> usize {
        self.times.len()
    }

    /// Get mean firing rate (Hz)
    pub fn firing_rate(&self, duration: Duration) -> f64 {
        if duration.is_zero() {
            return 0.0;
        }
        self.times.len() as f64 / duration.as_secs_f64()
    }

    /// Get inter-spike intervals
    pub fn isis(&self) -> Vec<Duration> {
        if self.times.len() < 2 {
            return Vec::new();
        }

        self.times
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect()
    }

    /// Get coefficient of variation of ISIs
    pub fn cv(&self) -> f64 {
        let isis = self.isis();
        if isis.is_empty() {
            return 0.0;
        }

        let isis_f64: Vec<f64> = isis.iter().map(|d| d.as_secs_f64()).collect();
        let mean = isis_f64.iter().sum::<f64>() / isis_f64.len() as f64;

        if mean == 0.0 {
            return 0.0;
        }

        let variance = isis_f64.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / isis_f64.len() as f64;
        let std = variance.sqrt();

        std / mean
    }

    /// Get spikes in a time window
    pub fn spikes_in_window(&self, start: Duration, end: Duration) -> Vec<Duration> {
        self.times
            .iter()
            .filter(|&&t| t >= start && t < end)
            .copied()
            .collect()
    }

    /// Compute instantaneous firing rate using kernel density estimation
    pub fn instantaneous_rate(&self, time: Duration, sigma: f64) -> f64 {
        let time_s = time.as_secs_f64();
        let sigma_s = sigma / 1000.0; // Convert ms to seconds

        self.times
            .iter()
            .map(|&t| {
                let dt = time_s - t.as_secs_f64();
                (-dt * dt / (2.0 * sigma_s * sigma_s)).exp()
            })
            .sum::<f64>()
            / (sigma_s * (2.0 * std::f64::consts::PI).sqrt())
    }
}

/// Analysis results for spike trains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeAnalysis {
    /// Total duration analyzed
    pub duration: Duration,
    /// Number of neurons
    pub neuron_count: usize,
    /// Total spike count
    pub total_spikes: usize,
    /// Mean firing rate (Hz)
    pub mean_rate: f64,
    /// Per-neuron firing rates
    pub neuron_rates: HashMap<NeuronId, f64>,
    /// Population synchrony measure
    pub synchrony: f64,
    /// Fano factor (variance/mean of spike counts)
    pub fano_factor: f64,
}

impl SpikeAnalysis {
    /// Analyze a collection of spike trains
    pub fn analyze(trains: &[SpikeTrain], duration: Duration) -> Self {
        let neuron_count = trains.len();
        let total_spikes: usize = trains.iter().map(|t| t.spike_count()).sum();

        let mut neuron_rates = HashMap::new();
        let mut rates = Vec::new();

        for train in trains {
            let rate = train.firing_rate(duration);
            neuron_rates.insert(train.neuron_id.clone(), rate);
            rates.push(rate);
        }

        let mean_rate = if neuron_count > 0 {
            rates.iter().sum::<f64>() / neuron_count as f64
        } else {
            0.0
        };

        // Compute synchrony (correlation of spike times)
        let synchrony = Self::compute_synchrony(trains, duration);

        // Compute Fano factor
        let spike_counts: Vec<f64> = trains.iter().map(|t| t.spike_count() as f64).collect();
        let fano_factor = Self::compute_fano(&spike_counts);

        Self {
            duration,
            neuron_count,
            total_spikes,
            mean_rate,
            neuron_rates,
            synchrony,
            fano_factor,
        }
    }

    fn compute_synchrony(trains: &[SpikeTrain], duration: Duration) -> f64 {
        if trains.len() < 2 {
            return 0.0;
        }

        // Use binned spike counts for correlation
        let bin_size = Duration::from_millis(10);
        let num_bins = (duration.as_millis() / bin_size.as_millis()) as usize;

        if num_bins == 0 {
            return 0.0;
        }

        // Create binned counts for each neuron
        let binned: Vec<Vec<u32>> = trains
            .iter()
            .map(|train| {
                let mut bins = vec![0u32; num_bins];
                for &time in &train.times {
                    let bin_idx = (time.as_millis() / bin_size.as_millis()) as usize;
                    if bin_idx < num_bins {
                        bins[bin_idx] += 1;
                    }
                }
                bins
            })
            .collect();

        // Compute mean pairwise correlation
        let mut total_corr = 0.0;
        let mut pair_count = 0;

        for i in 0..trains.len() {
            for j in (i + 1)..trains.len() {
                total_corr += Self::correlation(&binned[i], &binned[j]);
                pair_count += 1;
            }
        }

        if pair_count > 0 {
            total_corr / pair_count as f64
        } else {
            0.0
        }
    }

    fn correlation(a: &[u32], b: &[u32]) -> f64 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let n = a.len() as f64;
        let a_f: Vec<f64> = a.iter().map(|&x| x as f64).collect();
        let b_f: Vec<f64> = b.iter().map(|&x| x as f64).collect();

        let mean_a = a_f.iter().sum::<f64>() / n;
        let mean_b = b_f.iter().sum::<f64>() / n;

        let mut cov = 0.0;
        let mut var_a = 0.0;
        let mut var_b = 0.0;

        for i in 0..a.len() {
            let da = a_f[i] - mean_a;
            let db = b_f[i] - mean_b;
            cov += da * db;
            var_a += da * da;
            var_b += db * db;
        }

        if var_a == 0.0 || var_b == 0.0 {
            return 0.0;
        }

        cov / (var_a.sqrt() * var_b.sqrt())
    }

    fn compute_fano(counts: &[f64]) -> f64 {
        if counts.is_empty() {
            return 1.0;
        }

        let mean = counts.iter().sum::<f64>() / counts.len() as f64;
        if mean == 0.0 {
            return 1.0;
        }

        let variance = counts.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / counts.len() as f64;

        variance / mean
    }
}

/// Spike train buffer for real-time processing
#[derive(Debug, Clone)]
pub struct SpikeBuffer {
    /// Maximum duration to store
    max_duration: Duration,
    /// Spikes organized by neuron
    spikes: HashMap<NeuronId, Vec<Duration>>,
    /// Current time
    current_time: Duration,
}

impl SpikeBuffer {
    pub fn new(max_duration: Duration) -> Self {
        Self {
            max_duration,
            spikes: HashMap::new(),
            current_time: Duration::ZERO,
        }
    }

    /// Add a spike
    pub fn add_spike(&mut self, neuron_id: NeuronId, time: Duration) {
        self.current_time = time;

        let entry = self.spikes.entry(neuron_id).or_default();
        entry.push(time);

        // Prune old spikes
        let cutoff = if time > self.max_duration {
            time - self.max_duration
        } else {
            Duration::ZERO
        };

        entry.retain(|&t| t >= cutoff);
    }

    /// Get recent firing rate for a neuron
    pub fn recent_rate(&self, neuron_id: &NeuronId, window: Duration) -> f64 {
        let Some(spikes) = self.spikes.get(neuron_id) else {
            return 0.0;
        };

        let cutoff = if self.current_time > window {
            self.current_time - window
        } else {
            Duration::ZERO
        };

        let count = spikes.iter().filter(|&&t| t >= cutoff).count();
        count as f64 / window.as_secs_f64()
    }

    /// Get all spike trains
    pub fn to_spike_trains(&self) -> Vec<SpikeTrain> {
        self.spikes
            .iter()
            .map(|(id, times)| SpikeTrain::from_spikes(id.clone(), times.clone()))
            .collect()
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.spikes.clear();
        self.current_time = Duration::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spike_train_basic() {
        let mut train = SpikeTrain::new("n1".to_string());

        train.add_spike(Duration::from_millis(10));
        train.add_spike(Duration::from_millis(30));
        train.add_spike(Duration::from_millis(50));

        assert_eq!(train.spike_count(), 3);
    }

    #[test]
    fn test_firing_rate() {
        let train = SpikeTrain::from_spikes(
            "n1".to_string(),
            vec![
                Duration::from_millis(10),
                Duration::from_millis(30),
                Duration::from_millis(50),
                Duration::from_millis(70),
                Duration::from_millis(90),
            ],
        );

        let rate = train.firing_rate(Duration::from_millis(100));
        assert!((rate - 50.0).abs() < 1.0); // 5 spikes in 100ms = 50 Hz
    }

    #[test]
    fn test_isi() {
        let train = SpikeTrain::from_spikes(
            "n1".to_string(),
            vec![
                Duration::from_millis(10),
                Duration::from_millis(30),
                Duration::from_millis(50),
            ],
        );

        let isis = train.isis();
        assert_eq!(isis.len(), 2);
        assert_eq!(isis[0], Duration::from_millis(20));
        assert_eq!(isis[1], Duration::from_millis(20));
    }

    #[test]
    fn test_cv() {
        // Regular spiking (low CV)
        let regular = SpikeTrain::from_spikes(
            "n1".to_string(),
            vec![
                Duration::from_millis(10),
                Duration::from_millis(20),
                Duration::from_millis(30),
                Duration::from_millis(40),
            ],
        );

        let cv = regular.cv();
        assert!(cv < 0.1, "Regular spiking should have low CV");
    }

    #[test]
    fn test_spike_analysis() {
        let trains = vec![
            SpikeTrain::from_spikes(
                "n1".to_string(),
                vec![Duration::from_millis(10), Duration::from_millis(50)],
            ),
            SpikeTrain::from_spikes(
                "n2".to_string(),
                vec![Duration::from_millis(20), Duration::from_millis(60)],
            ),
        ];

        let analysis = SpikeAnalysis::analyze(&trains, Duration::from_millis(100));

        assert_eq!(analysis.neuron_count, 2);
        assert_eq!(analysis.total_spikes, 4);
        assert!(analysis.mean_rate > 0.0);
    }

    #[test]
    fn test_spike_buffer() {
        let mut buffer = SpikeBuffer::new(Duration::from_millis(100));

        buffer.add_spike("n1".to_string(), Duration::from_millis(10));
        buffer.add_spike("n1".to_string(), Duration::from_millis(50));
        buffer.add_spike("n1".to_string(), Duration::from_millis(90));

        let rate = buffer.recent_rate(&"n1".to_string(), Duration::from_millis(100));
        assert!(rate > 0.0);
    }
}
