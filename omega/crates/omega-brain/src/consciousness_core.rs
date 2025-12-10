//! Consciousness Core - Self-contained IIT, GWT, and Free Energy implementation
//!
//! Provides consciousness metrics based on Integrated Information Theory,
//! Global Workspace Theory, and Free Energy Principle.

use crate::{BrainConfig, Result};
use serde::{Deserialize, Serialize};

/// Phi computation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiResult {
    pub phi: f64,
}

/// IIT Calculator - computes integration measure
#[derive(Debug, Clone)]
pub struct IITCalculator {
    dim: usize,
    history: Vec<Vec<f64>>,
}

impl IITCalculator {
    pub fn new(dim: usize) -> Self {
        Self { dim, history: Vec::new() }
    }

    pub fn compute_phi(&mut self, state: &[f64]) -> PhiResult {
        self.history.push(state.to_vec());
        if self.history.len() > 100 { self.history.remove(0); }

        // Simplified phi: variance * entropy proxy
        let mean = state.iter().sum::<f64>() / state.len().max(1) as f64;
        let variance = state.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / state.len().max(1) as f64;
        let entropy = state.iter().map(|&x| {
            let p = x.abs().min(0.999).max(0.001);
            -p * p.ln()
        }).sum::<f64>() / state.len().max(1) as f64;

        PhiResult { phi: (variance * entropy).min(1.0) }
    }

    pub fn compute_deep_phi(&mut self, state: &[f64], depth: usize) -> PhiResult {
        let mut phi = 0.0;
        for d in 0..depth {
            let scaled: Vec<f64> = state.iter().map(|&x| x * (1.0 / (d + 1) as f64)).collect();
            phi += self.compute_phi(&scaled).phi;
        }
        PhiResult { phi: phi / depth.max(1) as f64 }
    }
}

/// Global Workspace - broadcasting mechanism
#[derive(Debug, Clone)]
pub struct GlobalWorkspace {
    workspace: Vec<Vec<f64>>,
    capacity: usize,
    threshold: f64,
    ignited: bool,
    ignition_level: f64,
}

impl GlobalWorkspace {
    pub fn new(capacity: usize, threshold: f64) -> Self {
        Self { workspace: Vec::new(), capacity, threshold, ignited: false, ignition_level: 0.0 }
    }

    pub fn broadcast(&mut self, content: &[f64]) -> Vec<f64> {
        let strength: f64 = content.iter().map(|x| x.abs()).sum::<f64>() / content.len().max(1) as f64;
        if strength > self.threshold {
            self.ignited = true;
            self.ignition_level = strength;
            if self.workspace.len() >= self.capacity { self.workspace.remove(0); }
            self.workspace.push(content.to_vec());
        } else {
            self.ignited = false;
            self.ignition_level = strength * 0.5;
        }
        content.to_vec()
    }

    pub fn force_broadcast(&mut self, content: &[f64]) {
        self.ignited = true;
        self.ignition_level = 1.0;
        if self.workspace.len() >= self.capacity { self.workspace.remove(0); }
        self.workspace.push(content.to_vec());
    }

    pub fn contents(&self) -> Vec<Vec<f64>> { self.workspace.clone() }
    pub fn ignition_level(&self) -> f64 { self.ignition_level }
    pub fn has_ignited(&self) -> bool { self.ignited }
}

/// Free Energy Minimizer - predictive processing
#[derive(Debug, Clone)]
pub struct FreeEnergyMinimizer {
    dim: usize,
    precision: f64,
    prediction: Vec<f64>,
    free_energy: f64,
    prediction_error: f64,
}

impl FreeEnergyMinimizer {
    pub fn new(dim: usize, precision: f64) -> Self {
        Self { dim, precision, prediction: vec![0.0; dim], free_energy: 1.0, prediction_error: 0.5 }
    }

    pub fn minimize(&mut self, observation: &[f64]) -> Vec<f64> {
        // Compute prediction error
        self.prediction_error = observation.iter().zip(self.prediction.iter())
            .map(|(&o, &p)| (o - p).powi(2)).sum::<f64>() / self.dim.max(1) as f64;

        // Update prediction (simple Kalman-like update)
        let gain = 0.3 * self.precision;
        for (i, &obs) in observation.iter().enumerate() {
            if i < self.dim {
                self.prediction[i] += gain * (obs - self.prediction[i]);
            }
        }

        // Free energy = prediction error + complexity
        self.free_energy = self.prediction_error + 0.1 * self.prediction.iter().map(|x| x.abs()).sum::<f64>() / self.dim.max(1) as f64;

        self.prediction.clone()
    }

    pub fn predict_extended(&mut self, state: &[f64], steps: usize) -> Vec<f64> {
        let mut result = state.to_vec();
        for _ in 0..steps {
            result = self.minimize(&result);
        }
        result
    }

    pub fn current_free_energy(&self) -> f64 { self.free_energy }
    pub fn prediction_error(&self) -> f64 { self.prediction_error }
}

/// Consciousness core integrating IIT, GWT, and FEP
pub struct ConsciousnessCore {
    iit: IITCalculator,
    workspace: GlobalWorkspace,
    free_energy: FreeEnergyMinimizer,
    current_phi: f64,
    current_free_energy: f64,
    integration_level: f64,
    consciousness_level: f64,
    dim: usize,
}

impl ConsciousnessCore {
    pub fn new(config: &BrainConfig) -> Self {
        Self {
            iit: IITCalculator::new(config.attention_dim),
            workspace: GlobalWorkspace::new(config.workspace_capacity, config.broadcast_threshold),
            free_energy: FreeEnergyMinimizer::new(config.attention_dim, config.precision),
            current_phi: 0.0,
            current_free_energy: 1.0,
            integration_level: 0.5,
            consciousness_level: 0.5,
            dim: config.attention_dim,
        }
    }

    pub fn integrate(&mut self, attended: &[f64]) -> Result<Vec<f64>> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| attended.get(i).copied().unwrap_or(0.0)).collect();

        let phi_result = self.iit.compute_phi(&normalized);
        self.current_phi = phi_result.phi;

        let broadcast = if self.current_phi > 0.3 {
            self.workspace.broadcast(&normalized)
        } else { normalized.clone() };

        let predicted = self.free_energy.minimize(&broadcast);
        self.current_free_energy = self.free_energy.current_free_energy();

        self.integration_level = self.current_phi.min(1.0);
        self.consciousness_level = 0.4 * self.current_phi.min(1.0)
            + 0.3 * (1.0 - self.current_free_energy.min(1.0))
            + 0.3 * self.workspace.ignition_level();

        Ok(predicted)
    }

    pub fn deliberate(&mut self, topic: &[f64]) -> Result<Vec<f64>> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| topic.get(i).copied().unwrap_or(0.0)).collect();
        self.workspace.force_broadcast(&normalized);
        let phi_result = self.iit.compute_deep_phi(&normalized, 3);
        self.current_phi = phi_result.phi;
        Ok(self.free_energy.predict_extended(&normalized, 5))
    }

    pub fn phi(&self) -> f64 { self.current_phi }
    pub fn free_energy(&self) -> f64 { self.current_free_energy }
    pub fn integration_level(&self) -> f64 { self.integration_level }
    pub fn consciousness_level(&self) -> f64 { self.consciousness_level }
    pub fn workspace_contents(&self) -> Vec<Vec<f64>> { self.workspace.contents() }
    pub fn has_ignited(&self) -> bool { self.workspace.has_ignited() }
    pub fn prediction_error(&self) -> f64 { self.free_energy.prediction_error() }

    pub fn reset(&mut self) {
        self.iit = IITCalculator::new(self.dim);
        self.workspace = GlobalWorkspace::new(7, 0.5);
        self.free_energy = FreeEnergyMinimizer::new(self.dim, 1.0);
        self.current_phi = 0.0;
        self.current_free_energy = 1.0;
        self.integration_level = 0.5;
        self.consciousness_level = 0.5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iit_calculator() {
        let mut iit = IITCalculator::new(16);
        let result = iit.compute_phi(&vec![0.5; 16]);
        assert!(result.phi >= 0.0);
    }

    #[test]
    fn test_global_workspace() {
        let mut gw = GlobalWorkspace::new(5, 0.3);
        gw.broadcast(&vec![0.5; 8]);
        assert!(gw.ignition_level() > 0.0);
    }

    #[test]
    fn test_free_energy() {
        let mut fe = FreeEnergyMinimizer::new(8, 1.0);
        fe.minimize(&vec![0.5; 8]);
        assert!(fe.current_free_energy() >= 0.0);
    }
}
