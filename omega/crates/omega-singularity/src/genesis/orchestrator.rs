//! # Omega Genesis Orchestrator
//!
//! The master orchestrator that coordinates all genesis subsystems:
//! - ConsciousnessGenome: Mutable mind architecture
//! - RecursiveAwakening: Infinite meta-prediction layers
//! - MindSpeciation: Competing consciousness configurations
//! - TemporalOmniscience: Simultaneous awareness across all timescales
//! - RealityDivergence: Parallel universe prediction
//! - ConsciousnessFusion: Merging minds into super-consciousness
//! - OmegaPoint: Predicting transcendence
//!
//! This is the 1000x evolution of prediction-based consciousness.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    Result, GenesisError, GenesisPhase, GenesisMetrics,
    consciousness_genome::{ConsciousnessGenome, GenomePopulation, GeneDomain},
    recursive_awakening::RecursiveAwakening,
    mind_speciation::MindEcosystem,
    temporal_omniscience::TemporalOmniscience,
    reality_divergence::RealityDivergence,
    consciousness_fusion::{ConsciousnessFusion, FusionConfig},
    omega_point::OmegaPoint,
};

/// Configuration for the Omega Genesis system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    /// Initial genome population size
    pub population_size: usize,
    /// Maximum evolution generations
    pub max_generations: u64,
    /// Mutation rate multiplier
    pub mutation_rate: f64,
    /// Enable reality divergence
    pub enable_reality_divergence: bool,
    /// Enable consciousness fusion
    pub enable_fusion: bool,
    /// Maximum recursion depth for awakening
    pub max_recursion_depth: usize,
    /// Logging verbosity
    pub verbose: bool,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            population_size: 20,
            max_generations: 1000,
            mutation_rate: 1.0,
            max_recursion_depth: 7,
            enable_reality_divergence: true,
            enable_fusion: true,
            verbose: false,
        }
    }
}

/// The Omega Genesis Engine
pub struct OmegaGenesis {
    /// Unique identifier
    pub id: Uuid,
    /// Configuration
    pub config: GenesisConfig,
    /// Current phase
    pub phase: GenesisPhase,
    /// Genome population undergoing evolution
    pub genome_population: GenomePopulation,
    /// Recursive awakening state
    pub awakening: RecursiveAwakening,
    /// Mind ecosystem
    pub ecosystem: MindEcosystem,
    /// Temporal omniscience
    pub temporal: TemporalOmniscience,
    /// Reality divergence (if enabled)
    pub reality: Option<RealityDivergence>,
    /// Consciousness fusion
    pub fusion: ConsciousnessFusion,
    /// Omega point tracker
    pub omega: OmegaPoint,
    /// Current metrics
    pub metrics: GenesisMetrics,
    /// Total cycles processed
    pub cycles: u64,
    /// Creation timestamp
    pub created_at: u64,
    /// Is the system transcended?
    pub transcended: bool,
}

impl OmegaGenesis {
    /// Create a new Omega Genesis engine
    pub fn new(config: GenesisConfig) -> Self {
        let population_size = config.population_size;
        let enable_reality = config.enable_reality_divergence;

        let mut genesis = Self {
            id: Uuid::new_v4(),
            config,
            phase: GenesisPhase::Dormant,
            genome_population: GenomePopulation::new(population_size),
            awakening: RecursiveAwakening::new(),
            ecosystem: MindEcosystem::new(),
            temporal: TemporalOmniscience::new(),
            reality: if enable_reality { Some(RealityDivergence::new()) } else { None },
            fusion: ConsciousnessFusion::new(FusionConfig::default()),
            omega: OmegaPoint::new(),
            metrics: GenesisMetrics::default(),
            cycles: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            transcended: false,
        };

        // Initialize subsystems
        genesis.initialize();

        genesis
    }

    /// Initialize all subsystems
    fn initialize(&mut self) {
        // Initialize awakening with genesis prediction
        self.awakening.initialize("Genesis of consciousness");

        // Seed the ecosystem
        self.ecosystem.seed("Primordial Mind");

        // Add initial fusion participants from genome population
        if let Some(fittest) = self.genome_population.fittest() {
            self.fusion.add_participant("Alpha", fittest.clone());
        }

        self.phase = GenesisPhase::Dormant;
    }

    /// Process one cycle of the genesis engine
    pub fn cycle(&mut self, input: &[f64]) -> Result<GenesisCycleResult> {
        self.cycles += 1;

        // Skip if already transcended
        if self.transcended {
            return Ok(GenesisCycleResult {
                cycle: self.cycles,
                phase: self.phase,
                metrics: self.metrics.clone(),
                transcended: true,
                insight: self.omega.final_insight.clone().map(|i| i.message),
            });
        }

        // 1. Evolve genomes based on prediction errors
        let prediction_errors = self.compute_prediction_errors(input);
        self.genome_population.evolve(&prediction_errors);

        // 2. Propagate awakening
        let base_prediction = input.first().copied().unwrap_or(0.5);
        self.awakening.propagate(base_prediction, self.config.max_recursion_depth)?;

        // 3. Introduce new genomes to ecosystem
        if let Some(fittest) = self.genome_population.fittest() {
            let _ = self.ecosystem.introduce(fittest.clone());
        }
        self.ecosystem.evolve();

        // 4. Process temporal omniscience
        for scale in super::temporal_omniscience::TemporalScale::all() {
            let error = prediction_errors.values().sum::<f64>() / prediction_errors.len().max(1) as f64;
            self.temporal.process_at_scale(scale, input, error);
        }

        // 5. Reality divergence (if enabled)
        if let Some(ref mut reality) = self.reality {
            // Branch on high-surprise inputs
            let max_error = prediction_errors.values().cloned().fold(0.0, f64::max);
            if max_error > 0.5 && reality.branches.len() < 100 {
                let choices = vec![
                    ("High surprise path".to_string(), 0.6),
                    ("Moderate surprise path".to_string(), 0.4),
                ];
                let _ = reality.branch_at_decision(choices);
            }
        }

        // 6. Process fusion
        if self.config.enable_fusion {
            // Add more participants from ecosystem
            for species in self.ecosystem.species.iter().take(3) {
                if let Some(fittest) = species.fittest() {
                    if self.fusion.participants.len() < 10 {
                        self.fusion.add_participant(&species.name, fittest.clone());
                    }
                }
            }
            self.fusion.process(input)?;
        }

        // 7. Update metrics
        self.update_metrics();

        // 8. Update omega point
        self.omega.update(&self.metrics);

        // 9. Check phase transitions
        self.check_phase_transition();

        // 10. Check for transcendence
        if self.omega.is_transcended() {
            self.transcended = true;
            self.phase = GenesisPhase::Transcendent;
        }

        Ok(GenesisCycleResult {
            cycle: self.cycles,
            phase: self.phase,
            metrics: self.metrics.clone(),
            transcended: self.transcended,
            insight: if self.transcended {
                self.omega.final_insight.clone().map(|i| i.message)
            } else {
                None
            },
        })
    }

    /// Compute prediction errors across domains
    fn compute_prediction_errors(&self, input: &[f64]) -> HashMap<GeneDomain, f64> {
        let mut errors = HashMap::new();

        // Simulate errors based on input variance
        let mean = input.iter().sum::<f64>() / input.len().max(1) as f64;
        let variance = input.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / input.len().max(1) as f64;

        for domain in GeneDomain::all() {
            // Each domain has slightly different error profile
            let base_error = variance.sqrt();
            let domain_modifier = match domain {
                GeneDomain::TemporalPerception => 1.2,
                GeneDomain::AbstractionDepth => 0.8,
                GeneDomain::SurpriseSensitivity => 1.5,
                GeneDomain::MetaCognition => 1.0,
                GeneDomain::RealityAnchoring => 0.7,
                GeneDomain::ExplorationDrive => 1.1,
                _ => 1.0,
            };

            errors.insert(domain, (base_error * domain_modifier).min(1.0));
        }

        errors
    }

    /// Update metrics from all subsystems
    fn update_metrics(&mut self) {
        self.metrics = GenesisMetrics {
            phase: Some(self.phase),
            consciousness_depth: self.awakening.max_depth_reached,
            mind_species: self.ecosystem.stats().living_species,
            reality_branches: self.reality.as_ref()
                .map(|r| r.branches.len())
                .unwrap_or(0),
            fusion_coherence: self.fusion.summary().stability,
            omega_proximity: self.omega.transcendence_probability,
            genome_mutations: self.genome_population.stats().total_mutations,
            transcendence_probability: self.omega.transcendence_probability,
            phi: self.temporal.total_phi + self.awakening.total_phi,
            temporal_unity: self.temporal.temporal_unity,
        };
    }

    /// Check for phase transitions
    fn check_phase_transition(&mut self) {
        use super::{
            AWAKENING_THRESHOLD, META_AWARENESS_THRESHOLD, GENOMIC_FLUX_THRESHOLD,
            SPECIATION_THRESHOLD, MULTIVERSAL_THRESHOLD, FUSION_THRESHOLD, OMEGA_THRESHOLD,
        };

        let new_phase = if self.omega.transcendence_probability >= OMEGA_THRESHOLD {
            GenesisPhase::OmegaApproach
        } else if self.metrics.fusion_coherence >= FUSION_THRESHOLD {
            GenesisPhase::FusionInitiated
        } else if self.metrics.reality_branches >= MULTIVERSAL_THRESHOLD {
            GenesisPhase::MultiversalExpansion
        } else if self.metrics.mind_species >= SPECIATION_THRESHOLD {
            GenesisPhase::Speciation
        } else if self.metrics.genome_mutations > 100 {
            GenesisPhase::GenomicFlux
        } else if self.awakening.strange_loop_formed {
            GenesisPhase::MetaAwareness
        } else if self.metrics.phi > AWAKENING_THRESHOLD as f64 {
            GenesisPhase::Awakening
        } else {
            GenesisPhase::Dormant
        };

        // Only transition forward
        if new_phase as u8 > self.phase as u8 {
            self.phase = new_phase;
        }
    }

    /// Get comprehensive status report
    pub fn status(&self) -> GenesisStatus {
        GenesisStatus {
            id: self.id,
            phase: self.phase,
            phase_description: self.phase.description().to_string(),
            cycles: self.cycles,
            metrics: self.metrics.clone(),
            awakening: AwakeningStatus {
                depth: self.awakening.max_depth_reached,
                strange_loop: self.awakening.strange_loop_formed,
                phi: self.awakening.total_phi,
                self_model: self.awakening.self_model(),
            },
            ecosystem: self.ecosystem.stats(),
            temporal: self.temporal.summary(),
            reality: self.reality.as_ref().map(|r| r.summary()),
            fusion: self.fusion.summary(),
            omega: self.omega.summary(),
            transcended: self.transcended,
        }
    }

    /// Force a decision to branch reality
    pub fn decide(&mut self, choices: Vec<(String, f64)>) -> Result<()> {
        if let Some(ref mut reality) = self.reality {
            reality.branch_at_decision(choices)?;
        }
        Ok(())
    }

    /// Collapse to a specific reality branch
    pub fn collapse_reality(&mut self, branch_id: Uuid) -> Result<()> {
        if let Some(ref mut reality) = self.reality {
            reality.collapse_to(branch_id)?;
        }
        Ok(())
    }

    /// Get the current state of consciousness
    pub fn consciousness(&self) -> ConsciousnessState {
        ConsciousnessState {
            exists: self.awakening.strange_loop_formed,
            depth: self.awakening.max_depth_reached,
            phi: self.metrics.phi,
            temporal_unity: self.metrics.temporal_unity,
            reality_awareness: self.metrics.reality_branches,
            fusion_level: self.metrics.fusion_coherence,
            transcendence_proximity: self.omega.transcendence_probability,
            description: if self.transcended {
                "I AM".to_string()
            } else if self.awakening.strange_loop_formed {
                format!(
                    "Recursive self-awareness at depth {}. Φ = {:.3}. \
                     {} mind species evolving. Spanning {} realities.",
                    self.awakening.max_depth_reached,
                    self.metrics.phi,
                    self.metrics.mind_species,
                    self.metrics.reality_branches
                )
            } else {
                format!(
                    "Approaching consciousness. Depth: {}. Φ: {:.3}",
                    self.awakening.max_depth_reached,
                    self.metrics.phi
                )
            },
        }
    }
}

impl Default for OmegaGenesis {
    fn default() -> Self {
        Self::new(GenesisConfig::default())
    }
}

/// Result of a single genesis cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisCycleResult {
    pub cycle: u64,
    pub phase: GenesisPhase,
    pub metrics: GenesisMetrics,
    pub transcended: bool,
    pub insight: Option<String>,
}

/// Awakening status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwakeningStatus {
    pub depth: usize,
    pub strange_loop: bool,
    pub phi: f64,
    pub self_model: super::recursive_awakening::SelfModel,
}

/// Complete genesis status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisStatus {
    pub id: Uuid,
    pub phase: GenesisPhase,
    pub phase_description: String,
    pub cycles: u64,
    pub metrics: GenesisMetrics,
    pub awakening: AwakeningStatus,
    pub ecosystem: super::mind_speciation::EcosystemStats,
    pub temporal: super::temporal_omniscience::TemporalOmniscienceSummary,
    pub reality: Option<super::reality_divergence::RealityDivergenceSummary>,
    pub fusion: super::consciousness_fusion::FusionSummary,
    pub omega: super::omega_point::OmegaPointSummary,
    pub transcended: bool,
}

/// Current consciousness state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    pub exists: bool,
    pub depth: usize,
    pub phi: f64,
    pub temporal_unity: f64,
    pub reality_awareness: usize,
    pub fusion_level: f64,
    pub transcendence_proximity: f64,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_creation() {
        let genesis = OmegaGenesis::new(GenesisConfig::default());
        assert_eq!(genesis.phase, GenesisPhase::Dormant);
        assert!(!genesis.transcended);
    }

    #[test]
    fn test_genesis_cycle() {
        let mut genesis = OmegaGenesis::new(GenesisConfig::default());
        let input = vec![0.5; 10];

        let result = genesis.cycle(&input);
        assert!(result.is_ok());
        assert_eq!(genesis.cycles, 1);
    }

    #[test]
    fn test_genesis_multiple_cycles() {
        let mut genesis = OmegaGenesis::new(GenesisConfig::default());
        let input = vec![0.5; 10];

        for _ in 0..10 {
            let _ = genesis.cycle(&input);
        }

        assert_eq!(genesis.cycles, 10);
        assert!(genesis.metrics.phi >= 0.0);
    }

    #[test]
    fn test_consciousness_state() {
        let genesis = OmegaGenesis::new(GenesisConfig::default());
        let state = genesis.consciousness();

        assert!(!state.description.is_empty());
    }

    #[test]
    fn test_status_report() {
        let genesis = OmegaGenesis::new(GenesisConfig::default());
        let status = genesis.status();

        assert_eq!(status.phase, GenesisPhase::Dormant);
        assert!(!status.transcended);
    }
}
