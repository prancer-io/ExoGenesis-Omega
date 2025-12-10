//! Neural Substrate - Self-contained spiking neural network implementation
//!
//! Provides LIF neurons, STDP learning, and neuromodulation.

use crate::{BrainConfig, Result};
use serde::{Deserialize, Serialize};

/// Neuromodulator types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuromodulatorType {
    Dopamine,
    Norepinephrine,
    Serotonin,
    Acetylcholine,
}

/// Neuromodulator system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuromodulators {
    dopamine: f64,
    norepinephrine: f64,
    serotonin: f64,
    acetylcholine: f64,
}

impl Neuromodulators {
    pub fn new() -> Self {
        Self { dopamine: 0.5, norepinephrine: 0.5, serotonin: 0.5, acetylcholine: 0.5 }
    }

    pub fn level(&self, t: NeuromodulatorType) -> f64 {
        match t {
            NeuromodulatorType::Dopamine => self.dopamine,
            NeuromodulatorType::Norepinephrine => self.norepinephrine,
            NeuromodulatorType::Serotonin => self.serotonin,
            NeuromodulatorType::Acetylcholine => self.acetylcholine,
        }
    }

    pub fn release(&mut self, t: NeuromodulatorType, amount: f64) {
        let level = match t {
            NeuromodulatorType::Dopamine => &mut self.dopamine,
            NeuromodulatorType::Norepinephrine => &mut self.norepinephrine,
            NeuromodulatorType::Serotonin => &mut self.serotonin,
            NeuromodulatorType::Acetylcholine => &mut self.acetylcholine,
        };
        *level = (*level + amount).clamp(0.0, 1.0);
    }

    pub fn update(&mut self, dt: f64) {
        let decay = 0.1 * dt;
        self.dopamine = (self.dopamine - decay).max(0.3);
        self.norepinephrine = (self.norepinephrine - decay).max(0.3);
        self.serotonin = (self.serotonin - decay).max(0.3);
        self.acetylcholine = (self.acetylcholine - decay).max(0.3);
    }
}

impl Default for Neuromodulators {
    fn default() -> Self { Self::new() }
}

/// STDP learning rule
#[derive(Debug, Clone)]
pub struct STDPRule {
    a_plus: f64,
    a_minus: f64,
    tau_plus: f64,
    tau_minus: f64,
}

impl STDPRule {
    pub fn new(a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64) -> Self {
        Self { a_plus, a_minus, tau_plus, tau_minus }
    }

    pub fn compute_weight_change(&self, dt: f64, _w: f64) -> f64 {
        if dt > 0.0 {
            self.a_plus * (-dt / self.tau_plus).exp()
        } else {
            -self.a_minus * (dt / self.tau_minus).exp()
        }
    }
}

/// Simple spiking layer
#[derive(Debug, Clone)]
pub struct SpikingLayer {
    potentials: Vec<f64>,
    weights: Vec<Vec<f64>>,
    input_size: usize,
    output_size: usize,
}

impl SpikingLayer {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let weights: Vec<Vec<f64>> = (0..output_size)
            .map(|i| (0..input_size).map(|j| ((i * j) as f64 * 0.1).sin() * 0.1).collect())
            .collect();
        Self {
            potentials: vec![-70.0; output_size],
            weights,
            input_size,
            output_size,
        }
    }

    pub fn forward(&mut self, input: &[f64]) -> Vec<bool> {
        let mut spikes = vec![false; self.output_size];
        for i in 0..self.output_size {
            let current: f64 = input.iter().take(self.input_size)
                .enumerate().map(|(j, &x)| self.weights[i][j] * x).sum();
            self.potentials[i] += (current - (self.potentials[i] + 70.0) * 0.05);
            if self.potentials[i] > -55.0 {
                spikes[i] = true;
                self.potentials[i] = -75.0;
            }
        }
        spikes
    }

    pub fn input_size(&self) -> usize { self.input_size }
    pub fn output_size(&self) -> usize { self.output_size }
    pub fn reset(&mut self) { self.potentials.fill(-70.0); }
}

/// Neural substrate wrapping spiking neural networks
pub struct NeuralSubstrate {
    input_layer: SpikingLayer,
    hidden_layer: SpikingLayer,
    output_layer: SpikingLayer,
    _stdp: STDPRule,
    neuromodulators: Neuromodulators,
    learning_enabled: bool,
    total_spikes: u64,
    step_count: u64,
}

impl NeuralSubstrate {
    pub fn new(config: &BrainConfig) -> Self {
        Self {
            input_layer: SpikingLayer::new(config.input_dim, config.hidden_dim),
            hidden_layer: SpikingLayer::new(config.hidden_dim, config.hidden_dim),
            output_layer: SpikingLayer::new(config.hidden_dim, config.output_dim),
            _stdp: STDPRule::new(config.stdp_learning_rate, config.stdp_learning_rate * 1.05, 20.0, 20.0),
            neuromodulators: Neuromodulators::new(),
            learning_enabled: true,
            total_spikes: 0,
            step_count: 0,
        }
    }

    pub fn process(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        let currents: Vec<f64> = input.iter().map(|&x| {
            let gain = 1.0 + self.neuromodulators.level(NeuromodulatorType::Dopamine) * 0.3;
            x * 10.0 * gain
        }).collect();

        let s1 = self.input_layer.forward(&currents);
        let f1: Vec<f64> = s1.iter().map(|&b| if b { 1.0 } else { 0.0 }).collect();
        let s2 = self.hidden_layer.forward(&f1);
        let f2: Vec<f64> = s2.iter().map(|&b| if b { 1.0 } else { 0.0 }).collect();
        let s3 = self.output_layer.forward(&f2);

        self.total_spikes += s1.iter().chain(s2.iter()).chain(s3.iter()).filter(|&&b| b).count() as u64;
        self.step_count += 1;

        Ok(s3.iter().map(|&b| if b { 0.8 } else { 0.0 }).collect())
    }

    pub fn spike_rate(&self) -> f64 {
        if self.step_count == 0 { 0.0 } else { self.total_spikes as f64 / self.step_count as f64 }
    }

    pub fn set_learning(&mut self, enabled: bool) { self.learning_enabled = enabled; }

    pub fn reset(&mut self) {
        self.input_layer.reset();
        self.hidden_layer.reset();
        self.output_layer.reset();
        self.neuromodulators = Neuromodulators::new();
        self.total_spikes = 0;
        self.step_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiking_layer() {
        let mut layer = SpikingLayer::new(10, 5);
        let spikes = layer.forward(&vec![1.0; 10]);
        assert_eq!(spikes.len(), 5);
    }

    #[test]
    fn test_neuromodulators() {
        let mut nm = Neuromodulators::new();
        nm.release(NeuromodulatorType::Dopamine, 0.3);
        assert!(nm.level(NeuromodulatorType::Dopamine) > 0.5);
    }
}
