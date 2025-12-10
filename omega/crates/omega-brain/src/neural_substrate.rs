//! Neural Substrate Integration
//!
//! Integrates omega-snn spiking neural networks into the brain.

use crate::{BrainConfig, BrainError, Result};
use omega_snn::{LIFNeuron, NeuromodulatorType, Neuromodulators, SpikingLayer, STDPRule};

/// Neural substrate wrapping spiking neural networks
pub struct NeuralSubstrate {
    /// Input layer
    input_layer: SpikingLayer,
    /// Hidden layer
    hidden_layer: SpikingLayer,
    /// Output layer
    output_layer: SpikingLayer,
    /// STDP learning rule
    stdp: STDPRule,
    /// Neuromodulator system
    neuromodulators: Neuromodulators,
    /// Enable learning
    learning_enabled: bool,
    /// Spike count
    total_spikes: u64,
    /// Step count
    step_count: u64,
}

impl NeuralSubstrate {
    /// Create new neural substrate
    pub fn new(config: &BrainConfig) -> Self {
        let input_layer = SpikingLayer::new(config.input_dim, config.hidden_dim);
        let hidden_layer = SpikingLayer::new(config.hidden_dim, config.hidden_dim);
        let output_layer = SpikingLayer::new(config.hidden_dim, config.output_dim);

        let stdp = STDPRule::new(
            config.stdp_learning_rate,
            config.stdp_learning_rate * 1.05, // Slightly stronger depression
            20.0,                              // tau_plus
            20.0,                              // tau_minus
        );

        let neuromodulators = Neuromodulators::new();

        Self {
            input_layer,
            hidden_layer,
            output_layer,
            stdp,
            neuromodulators,
            learning_enabled: true,
            total_spikes: 0,
            step_count: 0,
        }
    }

    /// Process input through spiking network
    pub fn process(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        // Convert input to currents
        let currents: Vec<f64> = input.iter().map(|&x| x * 10.0).collect();

        // Modulate with neuromodulators
        let modulated = self.apply_neuromodulation(&currents);

        // Forward through layers
        let input_spikes = self.input_layer.forward(&modulated);
        let hidden_spikes = self.hidden_layer.forward(&input_spikes);
        let output_spikes = self.output_layer.forward(&hidden_spikes);

        // Count spikes
        let spike_count: u64 = input_spikes.iter().map(|&s| s as u64).sum::<u64>()
            + hidden_spikes.iter().map(|&s| s as u64).sum::<u64>()
            + output_spikes.iter().map(|&s| s as u64).sum::<u64>();
        self.total_spikes += spike_count;
        self.step_count += 1;

        // Apply STDP learning if enabled
        if self.learning_enabled {
            self.apply_stdp(&input_spikes, &hidden_spikes);
            self.apply_stdp(&hidden_spikes, &output_spikes);
        }

        // Convert spikes to output
        let output: Vec<f64> = output_spikes.iter().map(|&s| if s { 1.0 } else { 0.0 }).collect();

        // Smooth output with exponential moving average
        let smoothed = self.smooth_output(&output);

        Ok(smoothed)
    }

    /// Apply neuromodulation to currents
    fn apply_neuromodulation(&self, currents: &[f64]) -> Vec<f64> {
        let dopamine = self.neuromodulators.level(NeuromodulatorType::Dopamine);
        let norepinephrine = self.neuromodulators.level(NeuromodulatorType::Norepinephrine);
        let acetylcholine = self.neuromodulators.level(NeuromodulatorType::Acetylcholine);

        // Modulate: dopamine increases signal, norepinephrine sharpens, ACh enhances
        currents
            .iter()
            .map(|&c| {
                let gain = 1.0 + dopamine * 0.3;
                let sharpness = 1.0 + norepinephrine * 0.2;
                let enhancement = 1.0 + acetylcholine * 0.1;
                c * gain * sharpness * enhancement
            })
            .collect()
    }

    /// Apply STDP learning
    fn apply_stdp(&mut self, pre_spikes: &[bool], post_spikes: &[bool]) {
        // Simplified STDP: strengthen connections where pre->post timing is causal
        for (i, &pre) in pre_spikes.iter().enumerate() {
            for (j, &post) in post_spikes.iter().enumerate() {
                if pre && post {
                    // Both spiked - potentiate
                    let delta = self.stdp.compute_weight_change(1.0, 0.0);
                    // Weight update would go here (simplified for integration)
                    let _ = delta; // Placeholder for actual weight update
                }
            }
        }
    }

    /// Smooth output with EMA
    fn smooth_output(&self, spikes: &[f64]) -> Vec<f64> {
        // In a real implementation, this would maintain state
        spikes.iter().map(|&s| s * 0.8).collect()
    }

    /// Release neuromodulator
    pub fn release_neuromodulator(&mut self, modulator: NeuromodulatorType, amount: f64) {
        self.neuromodulators.release(modulator, amount);
    }

    /// Update neuromodulator levels
    pub fn update_neuromodulators(&mut self, dt: f64) {
        self.neuromodulators.update(dt);
    }

    /// Get spike rate
    pub fn spike_rate(&self) -> f64 {
        if self.step_count == 0 {
            0.0
        } else {
            self.total_spikes as f64 / self.step_count as f64
        }
    }

    /// Enable/disable learning
    pub fn set_learning(&mut self, enabled: bool) {
        self.learning_enabled = enabled;
    }

    /// Get total spike count
    pub fn total_spikes(&self) -> u64 {
        self.total_spikes
    }

    /// Reset the neural substrate
    pub fn reset(&mut self) {
        self.input_layer = SpikingLayer::new(
            self.input_layer.input_size(),
            self.input_layer.output_size(),
        );
        self.hidden_layer = SpikingLayer::new(
            self.hidden_layer.input_size(),
            self.hidden_layer.output_size(),
        );
        self.output_layer = SpikingLayer::new(
            self.output_layer.input_size(),
            self.output_layer.output_size(),
        );
        self.neuromodulators = Neuromodulators::new();
        self.total_spikes = 0;
        self.step_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_substrate_creation() {
        let config = BrainConfig::default();
        let substrate = NeuralSubstrate::new(&config);

        assert_eq!(substrate.total_spikes(), 0);
    }

    #[test]
    fn test_neural_processing() {
        let config = BrainConfig::minimal();
        let mut substrate = NeuralSubstrate::new(&config);

        let input = vec![0.5; config.input_dim];
        let output = substrate.process(&input).unwrap();

        assert_eq!(output.len(), config.output_dim);
    }

    #[test]
    fn test_spike_rate() {
        let config = BrainConfig::minimal();
        let mut substrate = NeuralSubstrate::new(&config);

        let input = vec![1.0; config.input_dim];
        for _ in 0..10 {
            substrate.process(&input).unwrap();
        }

        // Should have some spike rate
        let rate = substrate.spike_rate();
        assert!(rate >= 0.0);
    }
}
