//! Emergence Detection
//!
//! Detects emergent properties in complex systems:
//! - Downward causation: Higher levels influencing lower levels
//! - Novel causal powers: Properties not reducible to parts
//! - Self-organization: Spontaneous order formation
//!
//! Based on complexity science and emergence theory

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Types of emergent properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmergenceType {
    /// Weak emergence: predictable from parts in principle
    Weak,
    /// Strong emergence: not reducible to parts
    Strong,
    /// Downward causation: top-down effects
    DownwardCausation,
    /// Self-organization: spontaneous order
    SelfOrganization,
}

/// A detected emergent property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentProperty {
    /// Type of emergence
    pub emergence_type: EmergenceType,
    /// Strength of emergence (0.0 to 1.0)
    pub strength: f64,
    /// Description
    pub description: String,
    /// Evidence for this property
    pub evidence: Vec<String>,
}

/// Causal power of the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalPower {
    /// Name of the power
    pub name: String,
    /// Magnitude
    pub magnitude: f64,
    /// Whether it's novel (not in parts)
    pub is_novel: bool,
    /// Direction (upward or downward)
    pub direction: CausalDirection,
}

/// Direction of causal influence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalDirection {
    Upward,   // Parts → Whole
    Downward, // Whole → Parts
    Lateral,  // Same level
}

/// Self-organization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfOrganization {
    /// Order parameter (measure of organization)
    pub order_parameter: f64,
    /// Entropy of the system
    pub entropy: f64,
    /// Complexity (edge of chaos)
    pub complexity: f64,
    /// Criticality (near phase transition)
    pub criticality: f64,
}

impl SelfOrganization {
    pub fn new() -> Self {
        Self {
            order_parameter: 0.0,
            entropy: 1.0,
            complexity: 0.0,
            criticality: 0.0,
        }
    }

    /// Check if system is self-organizing
    pub fn is_self_organizing(&self) -> bool {
        // Self-organization: moderate order, moderate entropy, high complexity
        self.order_parameter > 0.3
            && self.order_parameter < 0.9
            && self.complexity > 0.5
    }

    /// Check if near critical point
    pub fn is_critical(&self) -> bool {
        self.criticality > 0.7
    }
}

impl Default for SelfOrganization {
    fn default() -> Self {
        Self::new()
    }
}

/// Emergence detector
pub struct EmergenceDetector {
    /// History of system states
    state_history: VecDeque<Vec<f64>>,
    /// History of Phi values
    phi_history: VecDeque<f64>,
    /// History of free energy values
    fe_history: VecDeque<f64>,
    /// Maximum history length
    max_history: usize,
    /// Current self-organization metrics
    self_org: SelfOrganization,
    /// Detected properties
    properties: Vec<EmergentProperty>,
}

impl EmergenceDetector {
    pub fn new() -> Self {
        Self {
            state_history: VecDeque::with_capacity(100),
            phi_history: VecDeque::with_capacity(100),
            fe_history: VecDeque::with_capacity(100),
            max_history: 100,
            self_org: SelfOrganization::new(),
            properties: Vec::new(),
        }
    }

    /// Detect emergence in current state
    pub fn detect(&mut self, state: &[f64], phi: f64, free_energy: f64) -> f64 {
        // Store history
        self.state_history.push_back(state.to_vec());
        self.phi_history.push_back(phi);
        self.fe_history.push_back(free_energy);

        if self.state_history.len() > self.max_history {
            self.state_history.pop_front();
            self.phi_history.pop_front();
            self.fe_history.pop_front();
        }

        // Update self-organization metrics
        self.update_self_organization(state);

        // Detect emergent properties
        self.detect_properties(state, phi, free_energy);

        // Compute overall emergence level
        self.compute_emergence_level(phi, free_energy)
    }

    /// Update self-organization metrics
    fn update_self_organization(&mut self, state: &[f64]) {
        // Order parameter: variance of state (low variance = high order)
        let mean: f64 = state.iter().sum::<f64>() / state.len().max(1) as f64;
        let variance: f64 = state.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / state.len().max(1) as f64;

        self.self_org.order_parameter = 1.0 - variance.min(1.0);

        // Entropy: distribution of state values
        let mut bins = [0.0; 10];
        for &x in state {
            let bin = ((x.abs() * 9.0).min(9.0) as usize).min(9);
            bins[bin] += 1.0;
        }
        let total: f64 = bins.iter().sum();
        if total > 0.0 {
            self.self_org.entropy = -bins
                .iter()
                .filter(|&&b| b > 0.0)
                .map(|&b| {
                    let p = b / total;
                    p * p.ln()
                })
                .sum::<f64>()
                / (10.0f64).ln(); // Normalize to [0, 1]
        }

        // Complexity: product of order and entropy
        self.self_org.complexity =
            4.0 * self.self_org.order_parameter * (1.0 - self.self_org.order_parameter)
                * self.self_org.entropy
                * (1.0 - self.self_org.entropy);

        // Criticality: based on history fluctuations
        if self.phi_history.len() > 10 {
            let recent: Vec<f64> = self.phi_history.iter().rev().take(10).copied().collect();
            let phi_mean: f64 = recent.iter().sum::<f64>() / recent.len() as f64;
            let phi_var: f64 = recent.iter().map(|x| (x - phi_mean).powi(2)).sum::<f64>()
                / recent.len() as f64;

            // High variance in Phi suggests criticality
            self.self_org.criticality = (phi_var * 10.0).min(1.0);
        }
    }

    /// Detect specific emergent properties
    fn detect_properties(&mut self, state: &[f64], phi: f64, _free_energy: f64) {
        self.properties.clear();

        // Check for strong emergence (high Phi)
        if phi > 0.5 {
            self.properties.push(EmergentProperty {
                emergence_type: EmergenceType::Strong,
                strength: phi,
                description: "High integrated information indicates strong emergence".to_string(),
                evidence: vec![format!("Phi = {:.3}", phi)],
            });
        }

        // Check for self-organization
        if self.self_org.is_self_organizing() {
            self.properties.push(EmergentProperty {
                emergence_type: EmergenceType::SelfOrganization,
                strength: self.self_org.complexity,
                description: "System exhibits self-organizing behavior".to_string(),
                evidence: vec![
                    format!("Order = {:.3}", self.self_org.order_parameter),
                    format!("Entropy = {:.3}", self.self_org.entropy),
                    format!("Complexity = {:.3}", self.self_org.complexity),
                ],
            });
        }

        // Check for downward causation (Phi affecting state structure)
        if let Some(dc_strength) = self.detect_downward_causation(state, phi) {
            if dc_strength > 0.3 {
                self.properties.push(EmergentProperty {
                    emergence_type: EmergenceType::DownwardCausation,
                    strength: dc_strength,
                    description: "Whole-to-part causal influence detected".to_string(),
                    evidence: vec![format!("DC strength = {:.3}", dc_strength)],
                });
            }
        }

        // Check for criticality
        if self.self_org.is_critical() {
            self.properties.push(EmergentProperty {
                emergence_type: EmergenceType::Weak,
                strength: self.self_org.criticality,
                description: "System near critical point".to_string(),
                evidence: vec![format!("Criticality = {:.3}", self.self_org.criticality)],
            });
        }
    }

    /// Detect downward causation
    fn detect_downward_causation(&self, state: &[f64], phi: f64) -> Option<f64> {
        if self.state_history.len() < 2 {
            return None;
        }

        let prev_state = self.state_history.back().unwrap();
        let prev_phi = *self.phi_history.back().unwrap_or(&0.0);

        // Downward causation: high-level (Phi) changes predict low-level (state) changes
        let phi_change = phi - prev_phi;
        let state_change: f64 = state
            .iter()
            .zip(prev_state.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<f64>()
            / state.len().max(1) as f64;

        // Correlation between Phi change and state change
        if phi_change.abs() > 0.01 {
            let dc_strength = (state_change / phi_change.abs()).min(1.0);
            Some(dc_strength)
        } else {
            None
        }
    }

    /// Compute overall emergence level
    fn compute_emergence_level(&self, phi: f64, free_energy: f64) -> f64 {
        // Emergence combines:
        // - Integrated information (Phi)
        // - Self-organization complexity
        // - Criticality
        // - Inverse free energy (low FE = good predictions = coherent system)

        let phi_contrib = phi * 0.4;
        let complexity_contrib = self.self_org.complexity * 0.3;
        let criticality_contrib = self.self_org.criticality * 0.2;
        let fe_contrib = (1.0 / (1.0 + free_energy)) * 0.1;

        phi_contrib + complexity_contrib + criticality_contrib + fe_contrib
    }

    /// Get current self-organization metrics
    pub fn self_organization(&self) -> &SelfOrganization {
        &self.self_org
    }

    /// Get detected properties
    pub fn properties(&self) -> &[EmergentProperty] {
        &self.properties
    }

    /// Get causal powers
    pub fn causal_powers(&self) -> Vec<CausalPower> {
        let mut powers = Vec::new();

        // Upward causation (always present)
        powers.push(CausalPower {
            name: "Bottom-up processing".to_string(),
            magnitude: 1.0 - self.self_org.order_parameter,
            is_novel: false,
            direction: CausalDirection::Upward,
        });

        // Downward causation (if detected)
        for prop in &self.properties {
            if prop.emergence_type == EmergenceType::DownwardCausation {
                powers.push(CausalPower {
                    name: "Top-down influence".to_string(),
                    magnitude: prop.strength,
                    is_novel: true,
                    direction: CausalDirection::Downward,
                });
            }
        }

        // Self-organization power
        if self.self_org.is_self_organizing() {
            powers.push(CausalPower {
                name: "Self-organization".to_string(),
                magnitude: self.self_org.complexity,
                is_novel: true,
                direction: CausalDirection::Lateral,
            });
        }

        powers
    }

    /// Reset detector
    pub fn reset(&mut self) {
        self.state_history.clear();
        self.phi_history.clear();
        self.fe_history.clear();
        self.self_org = SelfOrganization::new();
        self.properties.clear();
    }
}

impl Default for EmergenceDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergence_detector_creation() {
        let detector = EmergenceDetector::new();
        assert!(detector.properties().is_empty());
    }

    #[test]
    fn test_detect_emergence() {
        let mut detector = EmergenceDetector::new();

        let state = vec![0.5; 16];
        let phi = 0.6;
        let fe = 0.3;

        let emergence = detector.detect(&state, phi, fe);
        assert!(emergence >= 0.0);
    }

    #[test]
    fn test_self_organization() {
        let mut so = SelfOrganization::new();
        so.order_parameter = 0.5;
        so.entropy = 0.5;
        so.complexity = 0.6;  // Must be > 0.5 (strictly greater)

        assert!(so.is_self_organizing());
    }

    #[test]
    fn test_criticality() {
        let mut so = SelfOrganization::new();
        so.criticality = 0.8;

        assert!(so.is_critical());
    }

    #[test]
    fn test_causal_powers() {
        let mut detector = EmergenceDetector::new();

        // Build up history
        for i in 0..20 {
            let state: Vec<f64> = (0..16).map(|j| ((i + j) % 16) as f64 / 16.0).collect();
            detector.detect(&state, 0.5 + (i as f64) * 0.01, 0.3);
        }

        let powers = detector.causal_powers();
        assert!(!powers.is_empty());
    }

    #[test]
    fn test_emergent_properties() {
        let mut detector = EmergenceDetector::new();

        // High Phi should trigger strong emergence detection
        let state = vec![0.5; 16];
        detector.detect(&state, 0.7, 0.2);

        let props = detector.properties();
        assert!(props.iter().any(|p| p.emergence_type == EmergenceType::Strong));
    }
}
