//! Consciousness Core Integration
//!
//! Integrates omega-consciousness components:
//! - Integrated Information Theory (IIT)
//! - Global Workspace Theory (GWT)
//! - Free Energy Principle (FEP)

use crate::{BrainConfig, BrainError, Result};
use omega_consciousness::{
    FreeEnergyMinimizer, GlobalWorkspace, IITCalculator, PhiResult,
};

/// Consciousness core integrating IIT, GWT, and FEP
pub struct ConsciousnessCore {
    /// IIT Phi calculator
    iit: IITCalculator,
    /// Global workspace
    workspace: GlobalWorkspace,
    /// Free energy minimizer
    free_energy: FreeEnergyMinimizer,
    /// Current Phi value
    current_phi: f64,
    /// Current free energy
    current_free_energy: f64,
    /// Integration level
    integration_level: f64,
    /// Consciousness level
    consciousness_level: f64,
    /// Dimension
    dim: usize,
}

impl ConsciousnessCore {
    /// Create new consciousness core
    pub fn new(config: &BrainConfig) -> Self {
        let iit = IITCalculator::new(config.attention_dim);
        let workspace = GlobalWorkspace::new(
            config.workspace_capacity,
            config.broadcast_threshold,
        );
        let free_energy = FreeEnergyMinimizer::new(
            config.attention_dim,
            config.precision,
        );

        Self {
            iit,
            workspace,
            free_energy,
            current_phi: 0.0,
            current_free_energy: 1.0,
            integration_level: 0.5,
            consciousness_level: 0.5,
            dim: config.attention_dim,
        }
    }

    /// Integrate attended content into consciousness
    pub fn integrate(&mut self, attended: &[f64]) -> Result<Vec<f64>> {
        // Normalize input
        let normalized = self.normalize_input(attended);

        // 1. Compute IIT Phi (integration measure)
        let phi_result = self.iit.compute_phi(&normalized);
        self.current_phi = phi_result.phi;

        // 2. Broadcast through global workspace (if above threshold)
        let broadcast = if self.current_phi > 0.3 {
            self.workspace.broadcast(&normalized)
        } else {
            normalized.clone()
        };

        // 3. Minimize free energy (predictive processing)
        let predicted = self.free_energy.minimize(&broadcast);
        self.current_free_energy = self.free_energy.current_free_energy();

        // Update consciousness metrics
        self.update_consciousness_level();

        Ok(predicted)
    }

    /// Deliberate on a topic (focused conscious processing)
    pub fn deliberate(&mut self, topic: &[f64]) -> Result<Vec<f64>> {
        let normalized = self.normalize_input(topic);

        // Force into workspace
        self.workspace.force_broadcast(&normalized);

        // Deep integration
        let phi_result = self.iit.compute_deep_phi(&normalized, 3);
        self.current_phi = phi_result.phi;

        // Extended prediction
        let prediction = self.free_energy.predict_extended(&normalized, 5);

        self.update_consciousness_level();

        Ok(prediction)
    }

    /// Update consciousness level based on metrics
    fn update_consciousness_level(&mut self) {
        // Consciousness is a function of:
        // - High Phi (integration)
        // - Low free energy (good predictions)
        // - Global broadcast (workspace ignition)

        let phi_contribution = self.current_phi.min(1.0);
        let fe_contribution = 1.0 - self.current_free_energy.min(1.0);
        let broadcast_contribution = self.workspace.ignition_level();

        self.integration_level = phi_contribution;
        self.consciousness_level =
            0.4 * phi_contribution + 0.3 * fe_contribution + 0.3 * broadcast_contribution;
    }

    /// Normalize input to expected dimension
    fn normalize_input(&self, input: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.dim];
        for (i, &v) in input.iter().enumerate() {
            if i < self.dim {
                result[i] = v;
            }
        }
        result
    }

    /// Get current Phi value
    pub fn phi(&self) -> f64 {
        self.current_phi
    }

    /// Get current free energy
    pub fn free_energy(&self) -> f64 {
        self.current_free_energy
    }

    /// Get integration level
    pub fn integration_level(&self) -> f64 {
        self.integration_level
    }

    /// Get consciousness level
    pub fn consciousness_level(&self) -> f64 {
        self.consciousness_level
    }

    /// Get workspace contents
    pub fn workspace_contents(&self) -> Vec<Vec<f64>> {
        self.workspace.contents()
    }

    /// Check if workspace has ignited
    pub fn has_ignited(&self) -> bool {
        self.workspace.has_ignited()
    }

    /// Get prediction error
    pub fn prediction_error(&self) -> f64 {
        self.free_energy.prediction_error()
    }

    /// Reset the consciousness core
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
    fn test_consciousness_core_creation() {
        let config = BrainConfig::default();
        let core = ConsciousnessCore::new(&config);

        assert!(core.phi() >= 0.0);
        assert!(core.consciousness_level() >= 0.0);
    }

    #[test]
    fn test_integrate() {
        let config = BrainConfig::minimal();
        let mut core = ConsciousnessCore::new(&config);

        let input = vec![0.5; config.attention_dim];
        let result = core.integrate(&input).unwrap();

        assert_eq!(result.len(), config.attention_dim);
    }

    #[test]
    fn test_deliberate() {
        let config = BrainConfig::minimal();
        let mut core = ConsciousnessCore::new(&config);

        let topic = vec![0.8; config.attention_dim];
        let result = core.deliberate(&topic).unwrap();

        assert_eq!(result.len(), config.attention_dim);
    }

    #[test]
    fn test_consciousness_level() {
        let config = BrainConfig::minimal();
        let mut core = ConsciousnessCore::new(&config);

        // Process some input
        let input = vec![0.5; config.attention_dim];
        core.integrate(&input).unwrap();

        let level = core.consciousness_level();
        assert!(level >= 0.0 && level <= 1.0);
    }
}
