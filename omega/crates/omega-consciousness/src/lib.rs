//! # Omega Consciousness - Substrate for Conscious Experience
//!
//! Implements theoretical frameworks for consciousness and cognition:
//!
//! ## Integrated Information Theory (IIT)
//! - Phi (Φ) computation: measure of integrated information
//! - Information geometry: cause-effect structures
//! - Exclusion and composition axioms
//!
//! ## Free Energy Principle (FEP)
//! - Predictive processing hierarchy
//! - Prediction error minimization
//! - Active inference for action selection
//!
//! ## Global Workspace Theory (GWT)
//! - Conscious access via broadcast
//! - Competition for workspace access
//! - Integration of specialized processors
//!
//! ## Emergence Detection
//! - Downward causation patterns
//! - Novel causal powers
//! - Self-organization metrics
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    CONSCIOUSNESS SYSTEM                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  ┌────────────────────────────────────────────────────────┐ │
//! │  │                 GLOBAL WORKSPACE                        │ │
//! │  │     Broadcast │ Competition │ Integration              │ │
//! │  └────────────────────────────────────────────────────────┘ │
//! │                          ↕                                   │
//! │  ┌──────────────────┐  ┌──────────────────────────────────┐ │
//! │  │ IIT (Φ)          │  │ FREE ENERGY PRINCIPLE            │ │
//! │  │                  │  │                                  │ │
//! │  │ • Integration    │  │ • Prediction hierarchy           │ │
//! │  │ • Exclusion      │  │ • Error minimization             │ │
//! │  │ • Composition    │  │ • Active inference               │ │
//! │  └──────────────────┘  └──────────────────────────────────┘ │
//! │                          ↕                                   │
//! │  ┌────────────────────────────────────────────────────────┐ │
//! │  │              EMERGENCE DETECTION                        │ │
//! │  │   Downward causation │ Novel powers │ Self-organization│ │
//! │  └────────────────────────────────────────────────────────┘ │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod iit;
pub mod free_energy;
pub mod global_workspace;
pub mod emergence;

pub use iit::{IntegratedInformation, PhiComputer, CauseEffectStructure, Partition};
pub use free_energy::{FreeEnergyMinimizer, PredictiveHierarchy, PredictionError, ActiveInference};
pub use global_workspace::{GlobalWorkspace, WorkspaceContent, BroadcastEvent, Coalition};
pub use emergence::{EmergenceDetector, EmergentProperty, CausalPower, SelfOrganization};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors in consciousness computation
#[derive(Error, Debug)]
pub enum ConsciousnessError {
    #[error("Invalid system state: {0}")]
    InvalidState(String),

    #[error("Computation failed: {0}")]
    ComputationError(String),

    #[error("Insufficient data for Phi calculation")]
    InsufficientData,

    #[error("System not integrated")]
    NotIntegrated,

    #[error("Prediction error: {0}")]
    PredictionError(String),
}

pub type Result<T> = std::result::Result<T, ConsciousnessError>;

/// Configuration for consciousness system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessConfig {
    /// Dimension of state vectors
    pub state_dim: usize,
    /// Number of hierarchical levels for FEP
    pub hierarchy_levels: usize,
    /// Global workspace capacity
    pub workspace_capacity: usize,
    /// Minimum Phi for consciousness threshold
    pub phi_threshold: f64,
    /// Free energy precision weight
    pub precision_weight: f64,
}

impl Default for ConsciousnessConfig {
    fn default() -> Self {
        Self {
            state_dim: 64,
            hierarchy_levels: 5,
            workspace_capacity: 7,
            phi_threshold: 0.1,
            precision_weight: 1.0,
        }
    }
}

/// Consciousness state measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    /// Integrated information (Phi)
    pub phi: f64,
    /// Free energy
    pub free_energy: f64,
    /// Prediction error
    pub prediction_error: f64,
    /// Emergence level
    pub emergence: f64,
    /// Whether system is conscious (Phi > threshold)
    pub is_conscious: bool,
    /// Current workspace contents
    pub workspace_contents: Vec<String>,
    /// Active coalitions
    pub active_coalitions: usize,
}

/// Main consciousness engine
pub struct ConsciousnessEngine {
    config: ConsciousnessConfig,
    iit: PhiComputer,
    fep: FreeEnergyMinimizer,
    workspace: GlobalWorkspace,
    emergence: EmergenceDetector,
}

impl ConsciousnessEngine {
    /// Create a new consciousness engine
    pub fn new(config: ConsciousnessConfig) -> Self {
        Self {
            iit: PhiComputer::new(config.state_dim),
            fep: FreeEnergyMinimizer::new(config.hierarchy_levels, config.state_dim),
            workspace: GlobalWorkspace::new(config.workspace_capacity),
            emergence: EmergenceDetector::new(),
            config,
        }
    }

    /// Process input through consciousness system
    pub fn process(&mut self, input: &[f64], context: &[f64]) -> Result<ConsciousnessState> {
        // 1. Compute integrated information (Phi)
        let phi = self.iit.compute_phi(input)?;

        // 2. Update predictive hierarchy and compute free energy
        let (free_energy, prediction_error) = self.fep.process(input, context)?;

        // 3. Compete for global workspace access
        if phi > self.config.phi_threshold {
            let content = WorkspaceContent::new(input.to_vec(), phi, "processed_input".to_string());
            self.workspace.compete(content);
        }

        // 4. Broadcast workspace contents
        self.workspace.broadcast();

        // 5. Detect emergence
        let emergence = self.emergence.detect(input, phi, free_energy);

        // 6. Determine consciousness status
        let is_conscious = phi > self.config.phi_threshold;

        Ok(ConsciousnessState {
            phi,
            free_energy,
            prediction_error,
            emergence,
            is_conscious,
            workspace_contents: self.workspace.content_ids(),
            active_coalitions: self.workspace.active_coalitions(),
        })
    }

    /// Get current Phi value
    pub fn phi(&self) -> f64 {
        self.iit.current_phi()
    }

    /// Get current free energy
    pub fn free_energy(&self) -> f64 {
        self.fep.current_free_energy()
    }

    /// Check if system is conscious
    pub fn is_conscious(&self) -> bool {
        self.iit.current_phi() > self.config.phi_threshold
    }

    /// Access global workspace
    pub fn workspace(&self) -> &GlobalWorkspace {
        &self.workspace
    }

    /// Access predictive hierarchy
    pub fn hierarchy(&self) -> &PredictiveHierarchy {
        self.fep.hierarchy()
    }

    /// Get configuration
    pub fn config(&self) -> &ConsciousnessConfig {
        &self.config
    }

    /// Reset consciousness state
    pub fn reset(&mut self) {
        self.iit.reset();
        self.fep.reset();
        self.workspace.clear();
        self.emergence.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_engine_creation() {
        let config = ConsciousnessConfig::default();
        let engine = ConsciousnessEngine::new(config);

        assert!(!engine.is_conscious());
    }

    #[test]
    fn test_consciousness_processing() {
        let config = ConsciousnessConfig::default();
        let mut engine = ConsciousnessEngine::new(config);

        let input = vec![0.5; 64];
        let context = vec![0.3; 64];

        let result = engine.process(&input, &context);
        assert!(result.is_ok());

        let state = result.unwrap();
        assert!(state.phi >= 0.0);
        assert!(state.free_energy >= 0.0);
    }
}
