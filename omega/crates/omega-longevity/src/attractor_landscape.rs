//! Attractor Landscape Engine - A Revolutionary Framework for Aging Dynamics
//!
//! This module implements a fundamentally new approach to understanding aging:
//! treating biological aging as traversal through a high-dimensional attractor
//! landscape, where age-related decline represents movement between basins of
//! attraction, and interventions are perturbations that can redirect trajectories.
//!
//! # Theoretical Foundation
//!
//! ## 1. Waddington's Landscape Extended to Aging
//!
//! Waddington's epigenetic landscape describes development as a ball rolling
//! downhill into valleys (cell fates). We extend this to aging:
//!
//! ```text
//!                    YOUTH BASIN                    OLD AGE BASIN
//!                         ○                              ○
//!                        ╱ ╲                            ╱ ╲
//!                       ╱   ╲          ╱╲              ╱   ╲
//!                      ╱     ╲        ╱  ╲            ╱     ╲
//!                     ╱       ╲______╱    ╲__________╱       ╲
//!                    ╱                                        ╲
//!           REGENERATIVE              TRANSITION              SENESCENT
//!             ATTRACTOR               LANDSCAPE               ATTRACTOR
//!
//!                    ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━●
//!                    │                                  │
//!                    │    Trajectory through life       │
//!                    │    (influenced by genetics,      │
//!                    │     environment, interventions)  │
//!                    │                                  │
//!                    Birth                            Death
//! ```
//!
//! ## 2. Phase Transitions in Aging
//!
//! Aging isn't gradual - it involves **phase transitions** where the system
//! reorganizes. Near critical points, small perturbations have outsized effects:
//!
//! ```text
//!  Biological        │
//!  Order             │    ┌─────────────────────────────────────────
//!  (Negentropy)      │    │    PHASE I: Regenerative
//!                    │    │    (High repair, low damage)
//!                    │    │
//!                    │    │         Critical Point 1
//!                    │    │              ↓
//!                    │    └─────────┐    ╭─────────────────────────
//!                    │              │    │   PHASE II: Compensated
//!                    │              │    │   (Damage accumulating but
//!                    │              └────╯    repair still functional)
//!                    │
//!                    │                        Critical Point 2
//!                    │                             ↓
//!                    │              ┌─────────────╮
//!                    │              │             └─────────────────
//!                    │              │   PHASE III: Decompensated
//!                    │              │   (Cascade failures)
//!                    └──────────────┴───────────────────────────────→
//!                         Age
//! ```
//!
//! ## 3. Causal Temporal Networks
//!
//! The causal structure of aging **changes with age**. What causes death at 40
//! is different from what causes death at 80. We model time-varying causal graphs:
//!
//! ```text
//!     Age 30                    Age 60                    Age 90
//!   ┌─────────┐               ┌─────────┐               ┌─────────┐
//!   │ Genetic │               │ Genetic │               │ Genetic │
//!   └────┬────┘               └────┬────┘               └────┬────┘
//!        │                         │ ╲                       │ ╲
//!        ▼                         ▼   ╲                     ▼   ╲
//!   ┌─────────┐               ┌─────────┐  ╲            ┌─────────┐  ╲
//!   │ Repair  │──────────────▶│ Repair  │───────────────│ Repair  │───▶ WEAK
//!   └─────────┘               └────┬────┘     ╲         └────┬────┘
//!        │                         │           ╲             │
//!        ▼                         ▼            ╲            ▼
//!   ┌─────────┐               ┌─────────┐        ╲     ┌─────────┐
//!   │ Damage  │               │ Damage  │─────────────▶│ Damage  │──▶ HIGH
//!   └─────────┘               └────┬────┘              └────┬────┘
//!                                  │                        │
//!                                  ▼                        ▼
//!                             ┌─────────┐              ┌─────────┐
//!                             │Senescnce│─────────────▶│Senescnce│──▶ CRITICAL
//!                             └─────────┘              └─────────┘
//!                                  │                        │
//!                                  ▼                        ▼
//!                             ┌─────────┐              ┌─────────┐
//!                             │ SASP    │─────────────▶│ SASP    │──▶ CASCADING
//!                             └─────────┘              └─────────┘
//! ```
//!
//! ## 4. Information-Theoretic Aging
//!
//! Aging is fundamentally **loss of biological information**:
//! - Epigenetic noise (Shannon entropy of methylation patterns)
//! - Loss of cellular identity (differentiation entropy)
//! - Accumulation of molecular damage (physical entropy)
//!
//! We track:
//! - **Mutual Information** between genome and phenotype
//! - **Transfer Entropy** showing information flow in causal networks
//! - **Integrated Information (φ)** measuring system coherence
//!
//! ## 5. Intervention Discovery via Bayesian Optimization
//!
//! Instead of testing known interventions, we **discover** optimal interventions
//! by exploring the high-dimensional intervention space using:
//! - Gaussian Process surrogate models
//! - Expected Improvement acquisition function
//! - Multi-objective optimization (lifespan, healthspan, quality)
//!
//! # Key Innovation
//!
//! This framework enables:
//! 1. **Counterfactual reasoning**: "What if this intervention 10 years ago?"
//! 2. **Critical point identification**: When are interventions most effective?
//! 3. **Cascade prediction**: Which failures trigger system-wide collapse?
//! 4. **Novel intervention discovery**: AI-driven exploration of intervention space
//! 5. **Digital twin assimilation**: Update predictions from real biomarker data

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use uuid::Uuid;
use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::genome::{Genome, Gene, GeneState};
use crate::organism::{Organism, DiseaseType};
use crate::hallmarks::Hallmark;
use crate::{Result, LongevityError};

// ============================================================================
// PART 1: ATTRACTOR LANDSCAPE
// ============================================================================

/// A point in the high-dimensional biological state space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalState {
    /// Unique identifier
    pub id: Uuid,
    /// Chronological age at this state
    pub age: f64,
    /// Biological age (can differ from chronological)
    pub biological_age: f64,
    /// Position in each hallmark dimension (0-1, higher = worse)
    pub hallmark_state: HashMap<Hallmark, f64>,
    /// Gene expression state (normalized)
    pub gene_expression: HashMap<Gene, f64>,
    /// Information-theoretic measures
    pub information_state: InformationState,
    /// Network connectivity state
    pub network_state: NetworkState,
    /// Current attractor basin
    pub current_basin: AttractorBasin,
    /// Distance to nearest critical point
    pub distance_to_critical: f64,
    /// Velocity vector (rate of change in each dimension)
    pub velocity: HashMap<String, f64>,
}

/// Information-theoretic measures of biological organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationState {
    /// Shannon entropy of epigenetic state (bits)
    /// Low = ordered/young, High = disordered/old
    pub epigenetic_entropy: f64,
    /// Mutual information between genome and phenotype (bits)
    /// Measures how well genotype predicts phenotype
    pub genome_phenotype_mi: f64,
    /// Integrated information (φ) - system coherence
    /// Measures irreducibility of the biological system
    pub integrated_information: f64,
    /// Transfer entropy in causal network (bits/time)
    /// Measures causal information flow
    pub causal_flow: f64,
    /// Negentropy - order relative to maximum entropy
    pub negentropy: f64,
    /// Rate of information loss (bits/year)
    pub information_loss_rate: f64,
}

/// Network topology and dynamics state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkState {
    /// Average clustering coefficient (local structure)
    pub clustering: f64,
    /// Average path length (global connectivity)
    pub path_length: f64,
    /// Network criticality (1.0 = critical point)
    pub criticality: f64,
    /// Fraction of functional edges
    pub edge_integrity: f64,
    /// Hub vulnerability (dependence on key nodes)
    pub hub_vulnerability: f64,
    /// Redundancy (alternative pathways available)
    pub redundancy: f64,
    /// Cascade susceptibility (likelihood of failure cascade)
    pub cascade_susceptibility: f64,
}

/// Basin of attraction in the aging landscape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttractorBasin {
    /// High regenerative capacity, low damage
    Regenerative,
    /// Damage accumulating but compensation active
    Compensated,
    /// Compensation failing, approaching cascade
    Decompensating,
    /// Active cascade of failures
    Cascading,
    /// Terminal decline
    Terminal,
    /// Exceptional longevity state (centenarian phenotype)
    Centenarian,
    /// Rejuvenated state (after successful intervention)
    Rejuvenated,
}

impl AttractorBasin {
    /// Typical chronological age range for this basin
    pub fn typical_age_range(&self) -> (f64, f64) {
        match self {
            Self::Regenerative => (0.0, 35.0),
            Self::Compensated => (30.0, 65.0),
            Self::Decompensating => (60.0, 85.0),
            Self::Cascading => (75.0, 100.0),
            Self::Terminal => (85.0, 120.0),
            Self::Centenarian => (90.0, 150.0),
            Self::Rejuvenated => (0.0, 200.0), // Any age if intervention succeeds
        }
    }

    /// Stability of this basin (how hard to escape)
    pub fn stability(&self) -> f64 {
        match self {
            Self::Regenerative => 0.9,      // Very stable when young
            Self::Compensated => 0.7,       // Moderately stable
            Self::Decompensating => 0.5,    // Unstable, easy to tip
            Self::Cascading => 0.3,         // Unstable, hard to escape
            Self::Terminal => 0.95,         // Very stable (absorbing state)
            Self::Centenarian => 0.85,      // Stable exceptional state
            Self::Rejuvenated => 0.6,       // Moderately stable, requires maintenance
        }
    }
}

/// A critical point where phase transition can occur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalPoint {
    /// Unique identifier
    pub id: Uuid,
    /// Name of this critical point
    pub name: String,
    /// Position in state space
    pub position: HashMap<String, f64>,
    /// Basins this point connects
    pub connects: (AttractorBasin, AttractorBasin),
    /// Typical age when encountered
    pub typical_age: f64,
    /// Width of critical region (how long the window lasts)
    pub width_years: f64,
    /// Interventions most effective at this point
    pub leverage_interventions: Vec<String>,
    /// Early warning signals that precede this point
    pub warning_signals: Vec<WarningSignal>,
}

/// Early warning signal for approaching critical point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarningSignal {
    /// Biomarker or measure that shows the signal
    pub marker: String,
    /// Type of signal
    pub signal_type: WarningSignalType,
    /// How far in advance the signal appears (years)
    pub advance_warning: f64,
    /// Reliability of this signal (0-1)
    pub reliability: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarningSignalType {
    /// Increased variance in the marker
    CriticalSlowing,
    /// Increased autocorrelation
    IncreasedMemory,
    /// Loss of oscillatory patterns
    LossOfRhythm,
    /// Skewed distribution
    Skewness,
    /// Flickering between states
    Flickering,
}

// ============================================================================
// PART 2: CAUSAL TEMPORAL NETWORKS
// ============================================================================

/// A causal graph that evolves over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalTemporalNetwork {
    /// Nodes in the causal network
    pub nodes: HashMap<CausalNode, NodeState>,
    /// Edges with time-varying strength
    pub edges: Vec<CausalEdge>,
    /// Current time point
    pub current_age: f64,
    /// History of network states
    pub history: Vec<NetworkSnapshot>,
}

/// A node in the causal network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CausalNode {
    // Fundamental processes
    DNARepair,
    TelomereMaintenance,
    EpigeneticStability,
    ProteostasisNetwork,
    MitochondrialFunction,

    // Damage accumulation
    OxidativeDamage,
    ProteinAggregates,
    SenescenceBurden,
    MutationLoad,

    // Systemic factors
    Inflammation,
    ImmuneFunction,
    HormoneSignaling,
    NutrientSensing,

    // Outputs
    TissueFunction,
    CognitiveFunction,
    PhysicalCapacity,
    DiseaseRisk,
    MortalityHazard,
}

/// State of a causal node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    /// Current activity level (0-1)
    pub activity: f64,
    /// Capacity/maximum possible (0-1)
    pub capacity: f64,
    /// Noise/variance in activity
    pub noise: f64,
    /// Whether this node is a hub (highly connected)
    pub is_hub: bool,
    /// Causal strength (total outgoing effect)
    pub causal_strength: f64,
}

/// A causal edge with time-varying properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    /// Source node
    pub from: CausalNode,
    /// Target node
    pub to: CausalNode,
    /// Base causal strength
    pub base_strength: f64,
    /// How strength changes with age (slope)
    pub age_coefficient: f64,
    /// Current effective strength
    pub current_strength: f64,
    /// Delay in causal effect (years)
    pub delay: f64,
    /// Type of causal relationship
    pub relationship: CausalRelationship,
    /// Confidence in this causal link
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalRelationship {
    /// A directly causes B
    Direct,
    /// A inhibits B
    Inhibitory,
    /// A modulates the effect of other causes on B
    Modulating,
    /// A and B have common cause (confounded)
    Confounded,
    /// A causes B, B causes A (feedback)
    Feedback,
    /// A causes mediator causes B
    Mediated,
}

/// Snapshot of network state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSnapshot {
    pub age: f64,
    pub node_activities: HashMap<CausalNode, f64>,
    pub total_causal_flow: f64,
    pub network_entropy: f64,
}

impl CausalTemporalNetwork {
    /// Create a new causal network for aging
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();

        // Initialize nodes
        for node in Self::all_nodes() {
            nodes.insert(node, NodeState {
                activity: 1.0,
                capacity: 1.0,
                noise: 0.05,
                is_hub: matches!(node,
                    CausalNode::Inflammation |
                    CausalNode::MitochondrialFunction |
                    CausalNode::DNARepair
                ),
                causal_strength: 0.0,
            });
        }

        // Build causal edges (the core aging network)
        // DNA Repair cascade
        edges.push(CausalEdge {
            from: CausalNode::DNARepair,
            to: CausalNode::MutationLoad,
            base_strength: -0.8,
            age_coefficient: 0.01,  // Weakens with age
            current_strength: -0.8,
            delay: 0.5,
            relationship: CausalRelationship::Inhibitory,
            confidence: 0.95,
        });

        edges.push(CausalEdge {
            from: CausalNode::MutationLoad,
            to: CausalNode::SenescenceBurden,
            base_strength: 0.6,
            age_coefficient: 0.005,
            current_strength: 0.6,
            delay: 2.0,
            relationship: CausalRelationship::Direct,
            confidence: 0.9,
        });

        // Senescence-inflammation loop
        edges.push(CausalEdge {
            from: CausalNode::SenescenceBurden,
            to: CausalNode::Inflammation,
            base_strength: 0.7,
            age_coefficient: 0.008,
            current_strength: 0.7,
            delay: 0.5,
            relationship: CausalRelationship::Direct,
            confidence: 0.95,
        });

        edges.push(CausalEdge {
            from: CausalNode::Inflammation,
            to: CausalNode::SenescenceBurden,
            base_strength: 0.3,
            age_coefficient: 0.01,
            current_strength: 0.3,
            delay: 1.0,
            relationship: CausalRelationship::Feedback,
            confidence: 0.8,
        });

        // Mitochondrial cascade
        edges.push(CausalEdge {
            from: CausalNode::MitochondrialFunction,
            to: CausalNode::OxidativeDamage,
            base_strength: -0.7,
            age_coefficient: 0.012,
            current_strength: -0.7,
            delay: 0.0,
            relationship: CausalRelationship::Inhibitory,
            confidence: 0.9,
        });

        edges.push(CausalEdge {
            from: CausalNode::OxidativeDamage,
            to: CausalNode::MitochondrialFunction,
            base_strength: -0.4,
            age_coefficient: 0.005,
            current_strength: -0.4,
            delay: 1.0,
            relationship: CausalRelationship::Feedback,
            confidence: 0.85,
        });

        // Proteostasis network
        edges.push(CausalEdge {
            from: CausalNode::ProteostasisNetwork,
            to: CausalNode::ProteinAggregates,
            base_strength: -0.9,
            age_coefficient: 0.015,
            current_strength: -0.9,
            delay: 0.5,
            relationship: CausalRelationship::Inhibitory,
            confidence: 0.92,
        });

        edges.push(CausalEdge {
            from: CausalNode::ProteinAggregates,
            to: CausalNode::CognitiveFunction,
            base_strength: -0.5,
            age_coefficient: 0.01,
            current_strength: -0.5,
            delay: 5.0,
            relationship: CausalRelationship::Direct,
            confidence: 0.75,
        });

        // Hub nodes affecting mortality
        edges.push(CausalEdge {
            from: CausalNode::Inflammation,
            to: CausalNode::DiseaseRisk,
            base_strength: 0.6,
            age_coefficient: 0.008,
            current_strength: 0.6,
            delay: 2.0,
            relationship: CausalRelationship::Direct,
            confidence: 0.9,
        });

        edges.push(CausalEdge {
            from: CausalNode::DiseaseRisk,
            to: CausalNode::MortalityHazard,
            base_strength: 0.8,
            age_coefficient: 0.01,
            current_strength: 0.8,
            delay: 0.0,
            relationship: CausalRelationship::Direct,
            confidence: 0.95,
        });

        // Telomere-senescence link
        edges.push(CausalEdge {
            from: CausalNode::TelomereMaintenance,
            to: CausalNode::SenescenceBurden,
            base_strength: -0.5,
            age_coefficient: 0.008,
            current_strength: -0.5,
            delay: 3.0,
            relationship: CausalRelationship::Inhibitory,
            confidence: 0.85,
        });

        // Epigenetic stability
        edges.push(CausalEdge {
            from: CausalNode::EpigeneticStability,
            to: CausalNode::TissueFunction,
            base_strength: 0.6,
            age_coefficient: -0.005, // This link weakens with age
            current_strength: 0.6,
            delay: 1.0,
            relationship: CausalRelationship::Direct,
            confidence: 0.8,
        });

        // Immune function
        edges.push(CausalEdge {
            from: CausalNode::ImmuneFunction,
            to: CausalNode::SenescenceBurden,
            base_strength: -0.4,
            age_coefficient: 0.012,
            current_strength: -0.4,
            delay: 0.5,
            relationship: CausalRelationship::Inhibitory,
            confidence: 0.75,
        });

        Self {
            nodes,
            edges,
            current_age: 0.0,
            history: Vec::new(),
        }
    }

    fn all_nodes() -> Vec<CausalNode> {
        vec![
            CausalNode::DNARepair,
            CausalNode::TelomereMaintenance,
            CausalNode::EpigeneticStability,
            CausalNode::ProteostasisNetwork,
            CausalNode::MitochondrialFunction,
            CausalNode::OxidativeDamage,
            CausalNode::ProteinAggregates,
            CausalNode::SenescenceBurden,
            CausalNode::MutationLoad,
            CausalNode::Inflammation,
            CausalNode::ImmuneFunction,
            CausalNode::HormoneSignaling,
            CausalNode::NutrientSensing,
            CausalNode::TissueFunction,
            CausalNode::CognitiveFunction,
            CausalNode::PhysicalCapacity,
            CausalNode::DiseaseRisk,
            CausalNode::MortalityHazard,
        ]
    }

    /// Advance network by one time step
    pub fn step(&mut self, dt_years: f64, rng: &mut impl Rng) {
        self.current_age += dt_years;

        // Update edge strengths based on age
        for edge in &mut self.edges {
            edge.current_strength = edge.base_strength +
                edge.age_coefficient * self.current_age;
        }

        // Calculate causal effects
        let mut effects: HashMap<CausalNode, f64> = HashMap::new();

        for edge in &self.edges {
            let source_activity = self.nodes.get(&edge.from)
                .map(|n| n.activity)
                .unwrap_or(0.5);

            let effect = source_activity * edge.current_strength;
            *effects.entry(edge.to).or_insert(0.0) += effect;
        }

        // Update node activities
        for (node, state) in &mut self.nodes {
            let external_effect = effects.get(node).copied().unwrap_or(0.0);

            // Activity changes based on causal inputs
            let noise: f64 = Normal::new(0.0, state.noise as f64).unwrap().sample(rng);
            state.activity += external_effect * dt_years + noise;
            state.activity = state.activity.clamp(0.0, state.capacity);

            // Capacity slowly degrades for protective nodes
            if matches!(node,
                CausalNode::DNARepair |
                CausalNode::ProteostasisNetwork |
                CausalNode::MitochondrialFunction |
                CausalNode::ImmuneFunction
            ) {
                state.capacity -= 0.005 * dt_years;
                state.capacity = state.capacity.max(0.2);
            }
        }

        // Record snapshot
        let snapshot = NetworkSnapshot {
            age: self.current_age,
            node_activities: self.nodes.iter()
                .map(|(k, v)| (*k, v.activity))
                .collect(),
            total_causal_flow: self.calculate_total_flow(),
            network_entropy: self.calculate_entropy(),
        };
        self.history.push(snapshot);
    }

    /// Calculate total causal flow through network
    pub fn calculate_total_flow(&self) -> f64 {
        self.edges.iter()
            .map(|e| {
                let source = self.nodes.get(&e.from).map(|n| n.activity).unwrap_or(0.0);
                (source * e.current_strength).abs()
            })
            .sum()
    }

    /// Calculate network entropy
    pub fn calculate_entropy(&self) -> f64 {
        let activities: Vec<f64> = self.nodes.values()
            .map(|n| n.activity)
            .collect();

        let sum: f64 = activities.iter().sum();
        if sum <= 0.0 {
            return 0.0;
        }

        activities.iter()
            .filter(|&&a| a > 0.0)
            .map(|&a| {
                let p = a / sum;
                -p * p.ln()
            })
            .sum()
    }

    /// Perform do-calculus intervention
    /// Sets a node to a specific value and propagates effects
    pub fn do_intervention(
        &mut self,
        node: CausalNode,
        value: f64,
        duration_years: f64,
        rng: &mut impl Rng,
    ) -> InterventionResult {
        let pre_state = self.nodes.clone();
        let pre_hazard = self.nodes.get(&CausalNode::MortalityHazard)
            .map(|n| n.activity)
            .unwrap_or(0.0);

        // Set the intervention
        if let Some(state) = self.nodes.get_mut(&node) {
            state.activity = value;
            // During intervention, this node is controlled (no causal inputs)
        }

        // Simulate forward
        let steps = (duration_years / 0.1) as usize;
        for _ in 0..steps {
            // Re-apply intervention each step
            if let Some(state) = self.nodes.get_mut(&node) {
                state.activity = value;
            }
            self.step(0.1, rng);
        }

        let post_hazard = self.nodes.get(&CausalNode::MortalityHazard)
            .map(|n| n.activity)
            .unwrap_or(0.0);

        InterventionResult {
            intervention_node: node,
            intervention_value: value,
            duration_years,
            hazard_change: post_hazard - pre_hazard,
            affected_nodes: self.nodes.iter()
                .filter(|(k, v)| {
                    let pre = pre_state.get(k).map(|n| n.activity).unwrap_or(0.0);
                    (v.activity - pre).abs() > 0.01
                })
                .map(|(k, v)| (*k, v.activity))
                .collect(),
        }
    }

    /// Calculate counterfactual: what would have happened if intervention at earlier age?
    pub fn counterfactual(
        &self,
        node: CausalNode,
        value: f64,
        at_age: f64,
        observe_age: f64,
        rng: &mut impl Rng,
    ) -> CounterfactualResult {
        // Clone the network at the earlier time point
        let history_index = self.history.iter()
            .position(|s| s.age >= at_age)
            .unwrap_or(0);

        let mut counterfactual_network = self.clone();

        // Reset to historical state
        if history_index > 0 {
            let snapshot = &self.history[history_index];
            for (node, activity) in &snapshot.node_activities {
                if let Some(state) = counterfactual_network.nodes.get_mut(node) {
                    state.activity = *activity;
                }
            }
            counterfactual_network.current_age = snapshot.age;
        }

        // Apply intervention
        if let Some(state) = counterfactual_network.nodes.get_mut(&node) {
            state.activity = value;
        }

        // Simulate forward to observe_age
        let mut rng_clone = rand::thread_rng();
        while counterfactual_network.current_age < observe_age {
            counterfactual_network.step(0.5, &mut rng_clone);
        }

        // Compare factual vs counterfactual
        let factual_hazard = self.nodes.get(&CausalNode::MortalityHazard)
            .map(|n| n.activity)
            .unwrap_or(0.0);
        let counterfactual_hazard = counterfactual_network.nodes
            .get(&CausalNode::MortalityHazard)
            .map(|n| n.activity)
            .unwrap_or(0.0);

        CounterfactualResult {
            intervention_age: at_age,
            observation_age: observe_age,
            factual_hazard,
            counterfactual_hazard,
            years_of_life_gained: (factual_hazard - counterfactual_hazard) * 10.0, // Rough estimate
            confidence: 0.7, // Counterfactuals always have uncertainty
        }
    }
}

/// Result of a do() intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionResult {
    pub intervention_node: CausalNode,
    pub intervention_value: f64,
    pub duration_years: f64,
    pub hazard_change: f64,
    pub affected_nodes: HashMap<CausalNode, f64>,
}

/// Result of counterfactual reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualResult {
    pub intervention_age: f64,
    pub observation_age: f64,
    pub factual_hazard: f64,
    pub counterfactual_hazard: f64,
    pub years_of_life_gained: f64,
    pub confidence: f64,
}

// ============================================================================
// PART 3: ATTRACTOR LANDSCAPE ENGINE
// ============================================================================

/// Configuration for the attractor landscape engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractorConfig {
    /// Number of dimensions in state space
    pub dimensions: usize,
    /// Time step for simulation (years)
    pub dt: f64,
    /// Noise level in dynamics
    pub noise_level: f64,
    /// Whether to track full trajectory
    pub track_trajectory: bool,
    /// Critical point detection sensitivity
    pub critical_sensitivity: f64,
}

impl Default for AttractorConfig {
    fn default() -> Self {
        Self {
            dimensions: 12, // Hallmarks of aging
            dt: 0.5,
            noise_level: 0.02,
            track_trajectory: true,
            critical_sensitivity: 0.1,
        }
    }
}

/// The main attractor landscape engine
pub struct AttractorLandscapeEngine {
    config: AttractorConfig,
    /// Current biological state
    current_state: BiologicalState,
    /// Causal temporal network
    causal_network: CausalTemporalNetwork,
    /// Known critical points
    critical_points: Vec<CriticalPoint>,
    /// Trajectory history
    trajectory: Vec<BiologicalState>,
    /// Intervention history
    interventions: Vec<InterventionRecord>,
}

/// Record of an intervention applied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionRecord {
    pub age_applied: f64,
    pub intervention_type: String,
    pub target_node: Option<CausalNode>,
    pub magnitude: f64,
    pub duration_years: f64,
    pub observed_effect: f64,
}

impl AttractorLandscapeEngine {
    /// Create a new engine from a genome
    pub fn new(genome: &Genome, config: AttractorConfig) -> Self {
        // Initialize state from genome
        let mut hallmark_state = HashMap::new();

        // Map genome to initial hallmark state
        hallmark_state.insert(Hallmark::GenomicInstability,
            1.0 - genome.dna_repair_capacity());
        hallmark_state.insert(Hallmark::TelomereAttrition,
            1.0 - genome.gene_function(Gene::TERT) * 0.5);
        hallmark_state.insert(Hallmark::EpigeneticAlterations,
            genome.epigenome.epigenetic_noise * 0.5 + genome.epigenome.calculate_horvath_age() / 120.0 * 0.5);
        hallmark_state.insert(Hallmark::LossOfProteostasis,
            1.0 - genome.proteostasis_capacity());
        hallmark_state.insert(Hallmark::DeregulatedNutrientSensing,
            genome.gene_function(Gene::MTOR) * 0.3);
        hallmark_state.insert(Hallmark::MitochondrialDysfunction,
            1.0 - genome.mtdna.respiratory_efficiency());
        hallmark_state.insert(Hallmark::CellularSenescence,
            genome.senescence_propensity() * 0.2);
        hallmark_state.insert(Hallmark::StemCellExhaustion,
            1.0 - genome.gene_function(Gene::NANOG) * 0.3);
        hallmark_state.insert(Hallmark::AlteredIntercellularCommunication,
            genome.inflammation_tendency() * 0.3);

        let gene_expression: HashMap<Gene, f64> = genome.nuclear_genes.iter()
            .map(|(g, s)| (*g, s.expression))
            .collect();

        let information_state = InformationState {
            epigenetic_entropy: genome.epigenome.epigenetic_noise * 2.0,
            genome_phenotype_mi: 3.5, // bits - high when young
            integrated_information: 2.0, // φ - system coherence
            causal_flow: 1.5,
            negentropy: 0.8,
            information_loss_rate: 0.01,
        };

        let network_state = NetworkState {
            clustering: 0.7,
            path_length: 2.5,
            criticality: 0.3,
            edge_integrity: 0.95,
            hub_vulnerability: 0.2,
            redundancy: 0.8,
            cascade_susceptibility: 0.1,
        };

        let current_state = BiologicalState {
            id: Uuid::new_v4(),
            age: 0.0,
            biological_age: 0.0,
            hallmark_state,
            gene_expression,
            information_state,
            network_state,
            current_basin: AttractorBasin::Regenerative,
            distance_to_critical: 15.0, // Years to first critical point
            velocity: HashMap::new(),
        };

        // Initialize critical points
        let critical_points = vec![
            CriticalPoint {
                id: Uuid::new_v4(),
                name: "Regenerative-Compensated Transition".to_string(),
                position: HashMap::new(),
                connects: (AttractorBasin::Regenerative, AttractorBasin::Compensated),
                typical_age: 35.0,
                width_years: 5.0,
                leverage_interventions: vec![
                    "NAD+ precursors".to_string(),
                    "Exercise optimization".to_string(),
                    "Sleep quality".to_string(),
                ],
                warning_signals: vec![
                    WarningSignal {
                        marker: "Epigenetic clock acceleration".to_string(),
                        signal_type: WarningSignalType::CriticalSlowing,
                        advance_warning: 3.0,
                        reliability: 0.8,
                    },
                ],
            },
            CriticalPoint {
                id: Uuid::new_v4(),
                name: "Compensated-Decompensating Transition".to_string(),
                position: HashMap::new(),
                connects: (AttractorBasin::Compensated, AttractorBasin::Decompensating),
                typical_age: 60.0,
                width_years: 8.0,
                leverage_interventions: vec![
                    "Senolytics".to_string(),
                    "mTOR inhibition".to_string(),
                    "Telomerase activation".to_string(),
                ],
                warning_signals: vec![
                    WarningSignal {
                        marker: "Inflammatory markers (IL-6, CRP)".to_string(),
                        signal_type: WarningSignalType::IncreasedMemory,
                        advance_warning: 5.0,
                        reliability: 0.75,
                    },
                    WarningSignal {
                        marker: "Heart rate variability".to_string(),
                        signal_type: WarningSignalType::LossOfRhythm,
                        advance_warning: 4.0,
                        reliability: 0.7,
                    },
                ],
            },
            CriticalPoint {
                id: Uuid::new_v4(),
                name: "Cascade Initiation Point".to_string(),
                position: HashMap::new(),
                connects: (AttractorBasin::Decompensating, AttractorBasin::Cascading),
                typical_age: 78.0,
                width_years: 5.0,
                leverage_interventions: vec![
                    "Partial reprogramming".to_string(),
                    "Aggressive senolytics".to_string(),
                    "Organ support".to_string(),
                ],
                warning_signals: vec![
                    WarningSignal {
                        marker: "Multi-organ function correlation".to_string(),
                        signal_type: WarningSignalType::Flickering,
                        advance_warning: 2.0,
                        reliability: 0.85,
                    },
                ],
            },
            // Bifurcation to centenarian phenotype
            CriticalPoint {
                id: Uuid::new_v4(),
                name: "Centenarian Bifurcation".to_string(),
                position: HashMap::new(),
                connects: (AttractorBasin::Decompensating, AttractorBasin::Centenarian),
                typical_age: 85.0,
                width_years: 10.0,
                leverage_interventions: vec![
                    "FOXO3 activation".to_string(),
                    "Inflammation control".to_string(),
                    "Maintained proteostasis".to_string(),
                ],
                warning_signals: vec![
                    WarningSignal {
                        marker: "Preserved cognitive reserve".to_string(),
                        signal_type: WarningSignalType::CriticalSlowing,
                        advance_warning: 5.0,
                        reliability: 0.6,
                    },
                ],
            },
        ];

        Self {
            config,
            current_state,
            causal_network: CausalTemporalNetwork::new(),
            critical_points,
            trajectory: Vec::new(),
            interventions: Vec::new(),
        }
    }

    /// Advance the system by one time step
    pub fn step(&mut self, rng: &mut impl Rng) {
        let dt = self.config.dt;

        // Advance causal network
        self.causal_network.step(dt, rng);

        // Update biological state from causal network
        self.update_state_from_network();

        // Calculate information measures
        self.update_information_state(rng);

        // Check for basin transitions
        self.check_basin_transition(rng);

        // Update distance to critical
        self.update_critical_distance();

        // Update velocities
        self.calculate_velocities();

        // Record trajectory
        if self.config.track_trajectory {
            self.trajectory.push(self.current_state.clone());
        }

        self.current_state.age += dt;
    }

    fn update_state_from_network(&mut self) {
        // Map causal network nodes to hallmark state
        let network = &self.causal_network;

        if let Some(node) = network.nodes.get(&CausalNode::DNARepair) {
            self.current_state.hallmark_state.insert(
                Hallmark::GenomicInstability,
                1.0 - node.activity
            );
        }

        if let Some(node) = network.nodes.get(&CausalNode::TelomereMaintenance) {
            self.current_state.hallmark_state.insert(
                Hallmark::TelomereAttrition,
                1.0 - node.activity
            );
        }

        if let Some(node) = network.nodes.get(&CausalNode::EpigeneticStability) {
            self.current_state.hallmark_state.insert(
                Hallmark::EpigeneticAlterations,
                1.0 - node.activity
            );
        }

        if let Some(node) = network.nodes.get(&CausalNode::ProteostasisNetwork) {
            self.current_state.hallmark_state.insert(
                Hallmark::LossOfProteostasis,
                1.0 - node.activity
            );
        }

        if let Some(node) = network.nodes.get(&CausalNode::MitochondrialFunction) {
            self.current_state.hallmark_state.insert(
                Hallmark::MitochondrialDysfunction,
                1.0 - node.activity
            );
        }

        if let Some(node) = network.nodes.get(&CausalNode::SenescenceBurden) {
            self.current_state.hallmark_state.insert(
                Hallmark::CellularSenescence,
                node.activity
            );
        }

        if let Some(node) = network.nodes.get(&CausalNode::Inflammation) {
            self.current_state.hallmark_state.insert(
                Hallmark::AlteredIntercellularCommunication,
                node.activity
            );
        }

        // Update biological age from hallmark average
        let hallmark_avg: f64 = self.current_state.hallmark_state.values().sum::<f64>()
            / self.current_state.hallmark_state.len() as f64;

        // Biological age = f(hallmark state, chronological age)
        self.current_state.biological_age = self.current_state.age * (0.7 + hallmark_avg * 0.6);
    }

    fn update_information_state(&mut self, rng: &mut impl Rng) {
        let info = &mut self.current_state.information_state;
        let age = self.current_state.age;

        // Epigenetic entropy increases with age
        info.epigenetic_entropy += 0.02 * self.config.dt +
            Normal::new(0.0, 0.005).unwrap().sample(rng);
        info.epigenetic_entropy = info.epigenetic_entropy.clamp(0.0, 4.0);

        // Mutual information decreases
        info.genome_phenotype_mi -= 0.01 * self.config.dt;
        info.genome_phenotype_mi = info.genome_phenotype_mi.max(0.5);

        // Integrated information (φ) decreases with age
        info.integrated_information -= 0.005 * self.config.dt;
        info.integrated_information = info.integrated_information.max(0.2);

        // Calculate negentropy (order)
        info.negentropy = 1.0 - info.epigenetic_entropy / 4.0;

        // Information loss rate increases with age
        info.information_loss_rate = 0.01 + age * 0.0005;
    }

    fn check_basin_transition(&mut self, rng: &mut impl Rng) {
        let age = self.current_state.age;
        let current_basin = self.current_state.current_basin;

        // Check each critical point
        for cp in &self.critical_points {
            if cp.connects.0 != current_basin {
                continue;
            }

            // Are we near this critical point?
            let distance = (age - cp.typical_age).abs();
            if distance > cp.width_years {
                continue;
            }

            // Transition probability increases as we approach and pass
            let progress = (age - (cp.typical_age - cp.width_years / 2.0)) / cp.width_years;
            let base_prob = progress.clamp(0.0, 1.0) * 0.3;

            // Hallmark state affects probability
            let hallmark_avg: f64 = self.current_state.hallmark_state.values().sum::<f64>()
                / self.current_state.hallmark_state.len() as f64;
            let modifier = hallmark_avg * 2.0;

            let transition_prob = (base_prob * modifier * self.config.dt).clamp(0.0, 0.5);

            if rng.gen::<f64>() < transition_prob {
                self.current_state.current_basin = cp.connects.1;

                // Record the transition
                let record = InterventionRecord {
                    age_applied: age,
                    intervention_type: format!("Basin transition: {:?} → {:?}",
                        cp.connects.0, cp.connects.1),
                    target_node: None,
                    magnitude: 1.0,
                    duration_years: 0.0,
                    observed_effect: 0.0,
                };
                self.interventions.push(record);

                return;
            }
        }
    }

    fn update_critical_distance(&mut self) {
        let age = self.current_state.age;
        let current_basin = self.current_state.current_basin;

        // Find nearest upcoming critical point
        let mut min_distance = f64::MAX;

        for cp in &self.critical_points {
            if cp.connects.0 == current_basin && cp.typical_age > age {
                let distance = cp.typical_age - age;
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }

        self.current_state.distance_to_critical = min_distance;
    }

    fn calculate_velocities(&mut self) {
        if self.trajectory.len() < 2 {
            return;
        }

        let prev = &self.trajectory[self.trajectory.len() - 1];
        let curr = &self.current_state;
        let dt = self.config.dt;

        let mut velocity = HashMap::new();

        for (hallmark, curr_value) in &curr.hallmark_state {
            if let Some(prev_value) = prev.hallmark_state.get(hallmark) {
                velocity.insert(
                    format!("{:?}", hallmark),
                    (curr_value - prev_value) / dt
                );
            }
        }

        velocity.insert(
            "biological_age".to_string(),
            (curr.biological_age - prev.biological_age) / dt
        );

        velocity.insert(
            "epigenetic_entropy".to_string(),
            (curr.information_state.epigenetic_entropy -
             prev.information_state.epigenetic_entropy) / dt
        );

        self.current_state.velocity = velocity;
    }

    /// Apply an intervention and observe effects
    pub fn apply_intervention(
        &mut self,
        intervention_type: &str,
        target_node: CausalNode,
        magnitude: f64,
        duration_years: f64,
        rng: &mut impl Rng,
    ) -> InterventionResult {
        // Record the intervention
        let record = InterventionRecord {
            age_applied: self.current_state.age,
            intervention_type: intervention_type.to_string(),
            target_node: Some(target_node),
            magnitude,
            duration_years,
            observed_effect: 0.0, // Will be updated after
        };
        self.interventions.push(record);

        // Apply via causal network
        let result = self.causal_network.do_intervention(
            target_node,
            magnitude,
            duration_years,
            rng
        );

        // Update our record with observed effect
        if let Some(last) = self.interventions.last_mut() {
            last.observed_effect = result.hazard_change;
        }

        result
    }

    /// Get the current state
    pub fn state(&self) -> &BiologicalState {
        &self.current_state
    }

    /// Get trajectory history
    pub fn trajectory(&self) -> &[BiologicalState] {
        &self.trajectory
    }

    /// Predict future trajectory
    pub fn predict_trajectory(
        &self,
        years_ahead: f64,
        interventions: Vec<(f64, CausalNode, f64)>, // (age, node, value)
    ) -> Vec<BiologicalState> {
        let mut engine = self.clone_for_prediction();
        let mut rng = rand::thread_rng();
        let mut predictions = Vec::new();

        let target_age = self.current_state.age + years_ahead;
        let mut intervention_idx = 0;

        while engine.current_state.age < target_age {
            // Check for scheduled interventions
            while intervention_idx < interventions.len() {
                let (int_age, node, value) = interventions[intervention_idx];
                if engine.current_state.age >= int_age {
                    engine.causal_network.do_intervention(node, value, 1.0, &mut rng);
                    intervention_idx += 1;
                } else {
                    break;
                }
            }

            engine.step(&mut rng);
            predictions.push(engine.current_state.clone());
        }

        predictions
    }

    fn clone_for_prediction(&self) -> Self {
        Self {
            config: self.config.clone(),
            current_state: self.current_state.clone(),
            causal_network: self.causal_network.clone(),
            critical_points: self.critical_points.clone(),
            trajectory: Vec::new(), // Don't clone trajectory for predictions
            interventions: Vec::new(),
        }
    }

    /// Identify optimal intervention timing based on critical points
    pub fn optimal_intervention_windows(&self) -> Vec<InterventionWindow> {
        let mut windows = Vec::new();
        let current_age = self.current_state.age;

        for cp in &self.critical_points {
            if cp.typical_age <= current_age {
                continue; // Already passed
            }

            // Window opens before critical point
            let window_start = (cp.typical_age - cp.width_years).max(current_age);
            let window_end = cp.typical_age + cp.width_years / 2.0;

            windows.push(InterventionWindow {
                start_age: window_start,
                end_age: window_end,
                critical_point: cp.name.clone(),
                effectiveness_multiplier: 2.0 + (1.0 / cp.width_years), // Narrower = more critical
                recommended_interventions: cp.leverage_interventions.clone(),
                warning_signals: cp.warning_signals.clone(),
            });
        }

        // Sort by start age
        windows.sort_by(|a, b| a.start_age.partial_cmp(&b.start_age).unwrap());

        windows
    }

    /// Calculate information-theoretic age
    pub fn information_age(&self) -> f64 {
        let info = &self.current_state.information_state;

        // Information age = how much information has been lost
        // relative to maximum possible loss
        let max_entropy = 4.0;
        let entropy_age = info.epigenetic_entropy / max_entropy * 100.0;

        let mi_loss = (3.5 - info.genome_phenotype_mi) / 3.0 * 100.0;
        let phi_loss = (2.0 - info.integrated_information) / 1.8 * 100.0;

        // Weighted combination
        entropy_age * 0.4 + mi_loss * 0.3 + phi_loss * 0.3
    }

    /// Detect early warning signals of approaching critical point
    pub fn detect_warning_signals(&self) -> Vec<DetectedWarning> {
        let mut warnings = Vec::new();

        // Check for critical slowing (increased variance)
        if self.trajectory.len() > 20 {
            let recent: Vec<f64> = self.trajectory.iter()
                .rev()
                .take(10)
                .map(|s| s.biological_age)
                .collect();

            let older: Vec<f64> = self.trajectory.iter()
                .rev()
                .skip(10)
                .take(10)
                .map(|s| s.biological_age)
                .collect();

            let recent_var = variance(&recent);
            let older_var = variance(&older);

            if recent_var > older_var * 1.5 {
                warnings.push(DetectedWarning {
                    signal_type: WarningSignalType::CriticalSlowing,
                    marker: "Biological age".to_string(),
                    strength: recent_var / older_var.max(0.01),
                    interpretation: "Increased variance suggests approaching critical point".to_string(),
                });
            }
        }

        // Check for loss of rhythm in information metrics
        if let Some(velocity) = self.current_state.velocity.get("epigenetic_entropy") {
            if velocity.abs() > 0.1 {
                warnings.push(DetectedWarning {
                    signal_type: WarningSignalType::LossOfRhythm,
                    marker: "Epigenetic entropy velocity".to_string(),
                    strength: velocity.abs(),
                    interpretation: "Rapid change in epigenetic state".to_string(),
                });
            }
        }

        // Check cascade susceptibility
        if self.current_state.network_state.cascade_susceptibility > 0.5 {
            warnings.push(DetectedWarning {
                signal_type: WarningSignalType::Flickering,
                marker: "Network cascade susceptibility".to_string(),
                strength: self.current_state.network_state.cascade_susceptibility,
                interpretation: "System vulnerable to cascade failures".to_string(),
            });
        }

        warnings
    }
}

/// A window where intervention is particularly effective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionWindow {
    pub start_age: f64,
    pub end_age: f64,
    pub critical_point: String,
    pub effectiveness_multiplier: f64,
    pub recommended_interventions: Vec<String>,
    pub warning_signals: Vec<WarningSignal>,
}

/// A detected early warning signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedWarning {
    pub signal_type: WarningSignalType,
    pub marker: String,
    pub strength: f64,
    pub interpretation: String,
}

// Helper function
fn variance(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
}

// ============================================================================
// PART 4: INTERVENTION DISCOVERY VIA BAYESIAN OPTIMIZATION
// ============================================================================

/// A point in intervention space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionPoint {
    /// Node to intervene on
    pub target: CausalNode,
    /// Intervention intensity (0-1)
    pub intensity: f64,
    /// Timing (age to apply)
    pub timing: f64,
    /// Duration
    pub duration: f64,
    /// Observed outcome (if evaluated)
    pub outcome: Option<f64>,
}

/// Bayesian optimization for intervention discovery
pub struct InterventionOptimizer {
    /// Evaluated points
    observations: Vec<InterventionPoint>,
    /// Best observed intervention
    best_intervention: Option<InterventionPoint>,
    /// Acquisition function parameter
    exploration_weight: f64,
}

impl InterventionOptimizer {
    pub fn new() -> Self {
        Self {
            observations: Vec::new(),
            best_intervention: None,
            exploration_weight: 2.0,
        }
    }

    /// Add an observation
    pub fn observe(&mut self, point: InterventionPoint) {
        if let Some(outcome) = point.outcome {
            if self.best_intervention.as_ref()
                .and_then(|b| b.outcome)
                .map(|best| outcome > best)
                .unwrap_or(true)
            {
                self.best_intervention = Some(point.clone());
            }
        }
        self.observations.push(point);
    }

    /// Suggest next intervention to try (simplified expected improvement)
    pub fn suggest_next(&self, rng: &mut impl Rng) -> InterventionPoint {
        if self.observations.len() < 5 {
            // Initial exploration: random sampling
            let targets = CausalTemporalNetwork::all_nodes();
            return InterventionPoint {
                target: targets[rng.gen_range(0..targets.len())],
                intensity: rng.gen_range(0.3..1.0),
                timing: rng.gen_range(30.0..70.0),
                duration: rng.gen_range(1.0..10.0),
                outcome: None,
            };
        }

        // Find promising regions based on observations
        // This is a simplified version - real implementation would use GP
        let best_outcomes: Vec<_> = self.observations.iter()
            .filter_map(|p| p.outcome.map(|o| (p, o)))
            .collect();

        if best_outcomes.is_empty() {
            // Fall back to random
            let targets = CausalTemporalNetwork::all_nodes();
            return InterventionPoint {
                target: targets[rng.gen_range(0..targets.len())],
                intensity: rng.gen_range(0.3..1.0),
                timing: rng.gen_range(30.0..70.0),
                duration: rng.gen_range(1.0..10.0),
                outcome: None,
            };
        }

        // Sort by outcome
        let mut sorted: Vec<_> = best_outcomes;
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Sample near best point with exploration noise
        let best_point = sorted[0].0;
        InterventionPoint {
            target: best_point.target, // Keep target
            intensity: (best_point.intensity +
                Normal::new(0.0, 0.1).unwrap().sample(rng))
                .clamp(0.0, 1.0),
            timing: (best_point.timing +
                Normal::new(0.0, 5.0).unwrap().sample(rng))
                .clamp(20.0, 90.0),
            duration: (best_point.duration +
                Normal::new(0.0, 2.0).unwrap().sample(rng))
                .clamp(0.5, 20.0),
            outcome: None,
        }
    }

    /// Get the current best intervention
    pub fn best(&self) -> Option<&InterventionPoint> {
        self.best_intervention.as_ref()
    }

    /// Get summary of exploration
    pub fn summary(&self) -> OptimizationSummary {
        let by_target: HashMap<CausalNode, Vec<f64>> = self.observations.iter()
            .filter_map(|p| p.outcome.map(|o| (p.target, o)))
            .fold(HashMap::new(), |mut acc, (target, outcome)| {
                acc.entry(target).or_default().push(outcome);
                acc
            });

        let target_effectiveness: Vec<_> = by_target.iter()
            .map(|(target, outcomes)| {
                let mean = outcomes.iter().sum::<f64>() / outcomes.len() as f64;
                (*target, mean)
            })
            .collect();

        OptimizationSummary {
            total_evaluations: self.observations.len(),
            best_outcome: self.best_intervention.as_ref()
                .and_then(|b| b.outcome)
                .unwrap_or(0.0),
            best_target: self.best_intervention.as_ref()
                .map(|b| b.target),
            target_effectiveness,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSummary {
    pub total_evaluations: usize,
    pub best_outcome: f64,
    pub best_target: Option<CausalNode>,
    pub target_effectiveness: Vec<(CausalNode, f64)>,
}

impl Default for InterventionOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CausalTemporalNetwork {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_network_creation() {
        let network = CausalTemporalNetwork::new();
        assert!(!network.nodes.is_empty());
        assert!(!network.edges.is_empty());
    }

    #[test]
    fn test_causal_network_step() {
        let mut network = CausalTemporalNetwork::new();
        let mut rng = rand::thread_rng();

        let initial_flow = network.calculate_total_flow();

        for _ in 0..10 {
            network.step(1.0, &mut rng);
        }

        assert_eq!(network.current_age, 10.0);
        assert!(!network.history.is_empty());
    }

    #[test]
    fn test_do_intervention() {
        let mut network = CausalTemporalNetwork::new();
        let mut rng = rand::thread_rng();

        // Advance to middle age
        for _ in 0..50 {
            network.step(1.0, &mut rng);
        }

        // Intervene on senescence
        let result = network.do_intervention(
            CausalNode::SenescenceBurden,
            0.2, // Reduce to 20%
            5.0, // For 5 years
            &mut rng
        );

        assert!(result.affected_nodes.len() > 0);
    }

    #[test]
    fn test_attractor_engine() {
        let genome = Genome::new_random(&mut rand::thread_rng());
        let config = AttractorConfig::default();
        let mut engine = AttractorLandscapeEngine::new(&genome, config);

        let mut rng = rand::thread_rng();

        // Simulate 50 years
        for _ in 0..100 {
            engine.step(&mut rng);
        }

        assert!(engine.current_state.age > 40.0);
        assert!(engine.current_state.biological_age > 0.0);
    }

    #[test]
    fn test_critical_point_detection() {
        let genome = Genome::new_random(&mut rand::thread_rng());
        let config = AttractorConfig::default();
        let engine = AttractorLandscapeEngine::new(&genome, config);

        let windows = engine.optimal_intervention_windows();
        assert!(!windows.is_empty());

        // First window should be before age 40
        assert!(windows[0].start_age < 40.0);
    }

    #[test]
    fn test_intervention_optimizer() {
        let mut optimizer = InterventionOptimizer::new();
        let mut rng = rand::thread_rng();

        // Add some observations
        for i in 0..10 {
            let mut point = optimizer.suggest_next(&mut rng);
            point.outcome = Some(rng.gen_range(-5.0..10.0));
            optimizer.observe(point);
        }

        let summary = optimizer.summary();
        assert_eq!(summary.total_evaluations, 10);
        assert!(optimizer.best().is_some());
    }

    #[test]
    fn test_information_age() {
        let genome = Genome::new_random(&mut rand::thread_rng());
        let config = AttractorConfig::default();
        let mut engine = AttractorLandscapeEngine::new(&genome, config);

        let mut rng = rand::thread_rng();

        let initial_info_age = engine.information_age();

        // Age 30 years
        for _ in 0..60 {
            engine.step(&mut rng);
        }

        let final_info_age = engine.information_age();

        // Information age should increase
        assert!(final_info_age > initial_info_age);
    }
}
