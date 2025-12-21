//! # Omega Longevity - AI-Powered Biological Aging Research
//!
//! This crate leverages the Omega ExoGenesis Brain cognitive architecture
//! to accelerate biological longevity research through:
//!
//! - **Mechanistic genome-based simulation** - Model aging from DNA → cells → organs → death
//! - **Causal discovery** - Simulate millions of lives to discover what causes aging
//! - **VUS interpretation** - Classify Variants of Unknown Significance via simulation
//! - **Dream-based hypothesis generation** for novel intervention discovery
//! - **Time-dilated lifespan simulation** for intervention evaluation
//! - **Intuitive pattern detection** in multi-omics aging data
//! - **Collective intelligence** for research literature synthesis
//!
//! ## The Key Insight: Bottom-Up Simulation
//!
//! Traditional approaches apply known hallmarks top-down. This system simulates
//! molecular mechanisms bottom-up and lets aging patterns EMERGE:
//!
//! ```text
//! GENOME (DNA + variants + epigenetics)
//!     ↓
//! CELLS (damage, repair, senescence, death)
//!     ↓
//! TISSUES (stem cell exhaustion, inflammation)
//!     ↓
//! ORGANS (function decline, failure cascades)
//!     ↓
//! ORGANISM (diseases, frailty, death)
//!     ↓
//! POPULATION (millions of lives → causal patterns)
//! ```
//!
//! ## Architecture Overview
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────────┐
//!  │                     OMEGA LONGEVITY SYSTEM                              │
//!  ├─────────────────────────────────────────────────────────────────────────┤
//!  │                                                                         │
//!  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                │
//!  │  │  HALLMARKS  │───►│  BIOMARKER  │───►│  LIFESPAN   │                │
//!  │  │  GRAPH      │    │  DREAMER    │    │  SIMULATOR  │                │
//!  │  │             │    │             │    │             │                │
//!  │  │ 12 Hallmarks│    │ REM-state   │    │ 10,000      │                │
//!  │  │ Causal Net  │    │ Exploration │    │ Futures     │                │
//!  │  │ Interventions    │ Novel Targets    │ Time-dilated│                │
//!  │  └─────────────┘    └─────────────┘    └─────────────┘                │
//!  │         │                  │                  │                        │
//!  │         ▼                  ▼                  ▼                        │
//!  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                │
//!  │  │ SENESCENCE  │◄──►│  RESEARCH   │◄──►│  LONGEVITY  │                │
//!  │  │ DETECTOR    │    │ INTEGRATOR  │    │ ADVISOR     │                │
//!  │  │             │    │             │    │             │                │
//!  │  │ Multi-omics │    │ Literature  │    │ Personalized│                │
//!  │  │ Intuition   │    │ Synthesis   │    │ Protocols   │                │
//!  │  │ Patterns    │    │ Rankings    │    │ Optimization│                │
//!  │  └─────────────┘    └─────────────┘    └─────────────┘                │
//!  │                                                                         │
//!  └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Key Capabilities
//!
//! ### 1. Hallmarks Knowledge Graph
//! Models the 12+ hallmarks of biological aging as an interconnected causal
//! network, enabling systematic identification of intervention points.
//!
//! ### 2. Biomarker Dreamer
//! Uses REM-like dream states to explore "forbidden" connections between
//! aging biomarkers, discovering novel drug targets that conscious analysis
//! would miss.
//!
//! ### 3. Lifespan Simulator
//! Employs time-dilated future simulation (1000x real-time) to predict
//! long-term effects of interventions across thousands of virtual lifespans.
//!
//! ### 4. Senescence Detector
//! Applies synthetic intuition to multi-omics data, detecting subtle
//! senescence patterns and predicting biological age.
//!
//! ### 5. Research Integrator
//! Synthesizes research literature using collective intelligence,
//! generating evidence-based intervention rankings.
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use omega_longevity::{LongevityAdvisor, LongevityConfig};
//!
//! // Create the longevity advisor
//! let mut advisor = LongevityAdvisor::new(LongevityConfig::default());
//!
//! // Run a dream session to discover novel targets
//! let discoveries = advisor.dream_about("cellular senescence")?;
//!
//! // Simulate intervention protocols
//! let protocol = advisor.create_protocol(vec!["Rapamycin", "NMN"]);
//! let results = advisor.simulate_lifespan(protocol)?;
//!
//! // Analyze multi-omics data for senescence patterns
//! let patterns = advisor.detect_senescence(omics_sample)?;
//!
//! // Get evidence-based intervention rankings
//! let rankings = advisor.rank_interventions()?;
//! ```
//!
//! ## How Omega Brain Solves Longevity Research Challenges
//!
//! | Research Challenge | Omega Brain Solution |
//! |-------------------|---------------------|
//! | Hypothesis generation | Dream Solver explores forbidden connections |
//! | Long-term prediction | Precognition Engine simulates 10,000 futures |
//! | Pattern recognition | Synthetic Intuition detects subtle biomarkers |
//! | Literature synthesis | Collective Consciousness integrates research |
//! | Multi-hallmark targeting | Knowledge Graph finds leverage points |
//!
//! ## Scientific Basis
//!
//! This system is built on established geroscience frameworks:
//!
//! - **López-Otín et al.** Hallmarks of Aging (Cell, 2013, updated 2023)
//! - **Horvath** Epigenetic clocks (Genome Biology, 2013)
//! - **Unity Biotechnology** Senolytic development
//! - **Sinclair Lab** NAD+ and epigenetic reprogramming
//! - **Kaeberlein Lab** mTOR and rapamycin research

// Core hallmarks knowledge
pub mod hallmarks;

// AI-powered research tools
pub mod biomarker_dreamer;
pub mod lifespan_simulator;
pub mod senescence_detector;
pub mod research_integrator;

// Mechanistic simulation (bottom-up)
pub mod genome;
pub mod cell;
pub mod organism;
pub mod causal_discovery;
pub mod vus_interpreter;

// The Immortality Engine - not just predicting death, PREVENTING it
pub mod immortality_engine;

// Re-exports
pub use hallmarks::{Hallmark, HallmarkCategory, HallmarksGraph, Intervention, InterventionType};
pub use biomarker_dreamer::{BiomarkerDreamer, DreamerConfig, NovelTarget, DreamSession};

// Mechanistic simulation re-exports
pub use genome::{Genome, Gene, GeneState, GeneVariant, MitochondrialDNA, Epigenome, TelomereState,
                 GeneticRiskScore, GeneticRiskFactor, GeneticProtectiveFactor};
pub use cell::{Cell, CellType, CellularDamage, MolecularMachinery, CellFate, CellPopulation};
pub use organism::{Organism, Lifestyle, OrganState, SystemicState, Disease, DiseaseType, DeathCause, DeathRecord,
                   LifespanPrediction, DiseaseRiskPrediction};
pub use causal_discovery::{PopulationSimulator, PopulationConfig, PopulationResults, CausalPattern, LifeSummary};
pub use lifespan_simulator::{LifespanSimulator, SimulatorConfig, InterventionProtocol, SimulationResults};
pub use senescence_detector::{SenescenceDetector, DetectorConfig, SenescencePattern, BiologicalAgePrediction};
pub use research_integrator::{ResearchIntegrator, IntegratorConfig, EvidenceSummary, InterventionRanking};
pub use vus_interpreter::{VUSInterpreter, VUSConfig, VUSInterpretation, VariantQuery, ACMGClassification, query_vus};

// Immortality Engine re-exports
pub use immortality_engine::{
    ImmortalityEngine, ImmortalityConfig, ImmortalityProtocol,
    Intervention as ImmortalityIntervention, InterventionCategory, EvidenceLevel,
    AgingMechanism, Protocol, DosingSchedule, ProtocolPhase, PhaseIntervention,
    Priority, ProtocolPredictions, LimitingFactor, MaxLifespanEstimate,
    MonitoringSchedule, BiomarkerSchedule, Contingency,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use thiserror::Error;

/// Errors that can occur in longevity operations
#[derive(Error, Debug)]
pub enum LongevityError {
    #[error("Insufficient data: {0}")]
    InsufficientData(String),

    #[error("Simulation failed: {0}")]
    SimulationFailed(String),

    #[error("Prediction failed: {0}")]
    PredictionFailed(String),

    #[error("Dream session failed: {0}")]
    DreamFailed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Research integration error: {0}")]
    IntegrationError(String),
}

pub type Result<T> = std::result::Result<T, LongevityError>;

/// Master configuration for the longevity system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongevityConfig {
    pub dreamer: DreamerConfig,
    pub simulator: SimulatorConfig,
    pub detector: DetectorConfig,
    pub integrator: IntegratorConfig,
    pub vus: VUSConfig,
    /// Enable full system integration
    pub integrated_mode: bool,
    /// Enable learning from outcomes
    pub learning_enabled: bool,
}

impl Default for LongevityConfig {
    fn default() -> Self {
        Self {
            dreamer: DreamerConfig::default(),
            simulator: SimulatorConfig::default(),
            detector: DetectorConfig::default(),
            integrator: IntegratorConfig::default(),
            vus: VUSConfig::default(),
            integrated_mode: true,
            learning_enabled: true,
        }
    }
}

/// The unified Longevity Advisor - orchestrates all subsystems
pub struct LongevityAdvisor {
    config: LongevityConfig,
    /// Hallmarks knowledge graph
    hallmarks: HallmarksGraph,
    /// Biomarker dreamer for hypothesis generation
    dreamer: BiomarkerDreamer,
    /// Lifespan simulator for intervention evaluation
    simulator: LifespanSimulator,
    /// Senescence detector for pattern recognition
    detector: SenescenceDetector,
    /// Research integrator for literature synthesis
    integrator: ResearchIntegrator,
    /// VUS interpreter for variant classification
    vus_interpreter: VUSInterpreter,
    /// Session history
    sessions: Vec<AdvisorSession>,
}

/// A session with the longevity advisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisorSession {
    pub id: Uuid,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub queries: Vec<AdvisorQuery>,
    pub discoveries: usize,
    pub simulations_run: usize,
    pub patterns_detected: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisorQuery {
    pub query_type: QueryType,
    pub input: String,
    pub result_summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryType {
    Dream,
    Simulate,
    Detect,
    Synthesize,
    Rank,
    Advise,
    InterpretVUS,
}

impl LongevityAdvisor {
    /// Create a new longevity advisor with the given configuration
    pub fn new(config: LongevityConfig) -> Self {
        Self {
            hallmarks: HallmarksGraph::new(),
            dreamer: BiomarkerDreamer::new(config.dreamer.clone()),
            simulator: LifespanSimulator::new(config.simulator.clone()),
            detector: SenescenceDetector::new(config.detector.clone()),
            integrator: ResearchIntegrator::new(config.integrator.clone()),
            vus_interpreter: VUSInterpreter::new(config.vus.clone()),
            config,
            sessions: Vec::new(),
        }
    }

    /// Start a new advisory session
    pub fn new_session(&mut self) -> Uuid {
        let session = AdvisorSession {
            id: Uuid::new_v4(),
            started_at: chrono::Utc::now(),
            queries: Vec::new(),
            discoveries: 0,
            simulations_run: 0,
            patterns_detected: 0,
        };
        let id = session.id;
        self.sessions.push(session);
        id
    }

    /// Run a dream session to discover novel targets
    pub fn dream_about(&mut self, problem: &str) -> Result<DreamSession> {
        let session = self.dreamer.dream_session(problem)?;

        // Record query
        if let Some(advisor_session) = self.sessions.last_mut() {
            advisor_session.queries.push(AdvisorQuery {
                query_type: QueryType::Dream,
                input: problem.to_string(),
                result_summary: format!("{} novel targets discovered", session.novel_targets.len()),
            });
            advisor_session.discoveries += session.novel_targets.len();
        }

        Ok(session)
    }

    /// Create an intervention protocol from intervention names
    pub fn create_protocol(&self, interventions: Vec<&str>) -> InterventionProtocol {
        use lifespan_simulator::{ProtocolIntervention, InterventionProtocol};

        let protocol_interventions: Vec<ProtocolIntervention> = interventions.iter()
            .map(|name| {
                let mut hallmark_effects = HashMap::new();
                // Get effects from hallmarks graph
                for int in self.hallmarks.top_interventions(50) {
                    if int.name.to_lowercase().contains(&name.to_lowercase()) {
                        for hallmark in &int.targets {
                            hallmark_effects.insert(*hallmark, -0.2); // Beneficial effect
                        }
                    }
                }

                ProtocolIntervention {
                    name: name.to_string(),
                    intervention_type: InterventionType::SmallMolecule,
                    hallmark_effects,
                    efficacy: 0.7,
                    side_effect_prob: 0.02,
                    interactions: Vec::new(),
                }
            })
            .collect();

        InterventionProtocol {
            id: Uuid::new_v4(),
            name: format!("Custom Protocol: {}", interventions.join(" + ")),
            interventions: protocol_interventions,
            start_age: 50.0,
            end_age: None,
        }
    }

    /// Simulate an intervention protocol
    pub fn simulate_protocol(&mut self, protocol: InterventionProtocol) -> Result<SimulationResults> {
        let results = self.simulator.simulate_protocol(protocol)?;

        // Record query
        if let Some(session) = self.sessions.last_mut() {
            session.queries.push(AdvisorQuery {
                query_type: QueryType::Simulate,
                input: results.protocol.name.clone(),
                result_summary: format!(
                    "+{:.1}y mean lifespan, {:.1}% centenarian probability",
                    results.summary.mean_lifespan_extension,
                    results.summary.prob_centenarian * 100.0
                ),
            });
            session.simulations_run += 1;
        }

        Ok(results)
    }

    /// Detect senescence patterns in multi-omics data
    pub fn detect_senescence(
        &mut self,
        sample: &senescence_detector::OmicsSample,
    ) -> Result<Vec<SenescencePattern>> {
        let patterns = self.detector.analyze_sample(sample)?;

        // Record query
        if let Some(session) = self.sessions.last_mut() {
            session.queries.push(AdvisorQuery {
                query_type: QueryType::Detect,
                input: sample.sample_id.clone(),
                result_summary: format!("{} patterns detected", patterns.len()),
            });
            session.patterns_detected += patterns.len();
        }

        Ok(patterns)
    }

    /// Predict biological age from multi-omics data
    pub fn predict_biological_age(
        &mut self,
        sample: &senescence_detector::OmicsSample,
    ) -> Result<BiologicalAgePrediction> {
        self.detector.predict_biological_age(sample)
    }

    /// Synthesize research evidence for an intervention
    pub fn synthesize_evidence(&mut self, intervention: &str) -> Result<EvidenceSummary> {
        self.integrator.synthesize_intervention(intervention)
    }

    /// Interpret a Variant of Unknown Significance (VUS)
    ///
    /// This is the key capability for clinical genomics interpretation.
    /// Given a VUS, we simulate its functional effect across thousands
    /// of virtual lifespans to determine if it's likely pathogenic or benign.
    pub fn interpret_vus(&mut self, query: VariantQuery) -> VUSInterpretation {
        let mut rng = rand::thread_rng();
        let result = self.vus_interpreter.interpret(query.clone(), &mut rng);

        // Record query
        if let Some(session) = self.sessions.last_mut() {
            session.queries.push(AdvisorQuery {
                query_type: QueryType::InterpretVUS,
                input: format!("{:?}:{}", query.gene, query.hgvs_cdna),
                result_summary: format!(
                    "{:?} (confidence: {:.0}%)",
                    result.predicted_classification,
                    result.confidence * 100.0
                ),
            });
        }

        result
    }

    /// Batch interpret multiple VUS variants
    pub fn interpret_vus_batch(&mut self, queries: Vec<VariantQuery>) -> Vec<VUSInterpretation> {
        let mut rng = rand::thread_rng();
        let results = self.vus_interpreter.batch_interpret(queries.clone(), &mut rng);

        // Record query
        if let Some(session) = self.sessions.last_mut() {
            session.queries.push(AdvisorQuery {
                query_type: QueryType::InterpretVUS,
                input: format!("{} variants", queries.len()),
                result_summary: format!(
                    "{} interpreted ({} pathogenic/likely pathogenic)",
                    results.len(),
                    results.iter()
                        .filter(|r| matches!(
                            r.predicted_classification,
                            vus_interpreter::ACMGClassification::Pathogenic |
                            vus_interpreter::ACMGClassification::LikelyPathogenic
                        ))
                        .count()
                ),
            });
        }

        results
    }

    /// Get ranked interventions based on evidence
    pub fn rank_interventions(&mut self) -> Vec<InterventionRanking> {
        // Add sample papers if none exist
        if self.integrator.paper_count() == 0 {
            self.integrator.add_sample_papers();
        }

        let rankings = self.integrator.rank_interventions();

        // Record query
        if let Some(session) = self.sessions.last_mut() {
            session.queries.push(AdvisorQuery {
                query_type: QueryType::Rank,
                input: "All interventions".to_string(),
                result_summary: format!("{} interventions ranked", rankings.len()),
            });
        }

        rankings
    }

    /// Get personalized longevity advice based on individual data
    pub fn get_personalized_advice(
        &mut self,
        biological_age: f64,
        chronological_age: f64,
        primary_concerns: Vec<Hallmark>,
    ) -> PersonalizedAdvice {
        let age_acceleration = biological_age - chronological_age;

        // Find leverage points based on concerns
        let leverage_points = self.hallmarks.find_leverage_points();
        let relevant_leverage: Vec<_> = leverage_points.into_iter()
            .filter(|(h, _)| primary_concerns.contains(h))
            .take(3)
            .collect();

        // Get relevant interventions
        let mut recommended_interventions = Vec::new();
        for (hallmark, _) in &relevant_leverage {
            let interventions = self.hallmarks.interventions_for(*hallmark);
            for int in interventions.into_iter().take(2) {
                recommended_interventions.push(int.name.clone());
            }
        }

        // Generate advice based on age acceleration
        let urgency = if age_acceleration > 10.0 {
            Urgency::High
        } else if age_acceleration > 5.0 {
            Urgency::Medium
        } else if age_acceleration > 0.0 {
            Urgency::Low
        } else {
            Urgency::Maintenance
        };

        let action_items = match urgency {
            Urgency::High => vec![
                "Consult with longevity medicine physician".to_string(),
                "Consider senolytic therapy evaluation".to_string(),
                "Comprehensive biomarker panel recommended".to_string(),
            ],
            Urgency::Medium => vec![
                "Optimize lifestyle factors (sleep, exercise, nutrition)".to_string(),
                "Consider targeted supplementation".to_string(),
                "Regular biomarker monitoring".to_string(),
            ],
            Urgency::Low => vec![
                "Maintain current healthy practices".to_string(),
                "Consider preventive interventions".to_string(),
                "Annual biological age assessment".to_string(),
            ],
            Urgency::Maintenance => vec![
                "Excellent biological age - maintain regimen".to_string(),
                "Continue current lifestyle".to_string(),
                "Monitor for any changes".to_string(),
            ],
        };

        PersonalizedAdvice {
            id: Uuid::new_v4(),
            biological_age,
            chronological_age,
            age_acceleration,
            urgency,
            priority_hallmarks: relevant_leverage.iter().map(|(h, _)| *h).collect(),
            recommended_interventions,
            action_items,
            estimated_potential_years: if age_acceleration > 0.0 {
                Some(age_acceleration * 0.8) // Conservative estimate of recoverable years
            } else {
                None
            },
        }
    }

    /// Run comprehensive analysis combining all subsystems
    pub fn comprehensive_analysis(
        &mut self,
        problem_focus: &str,
        omics_sample: Option<&senescence_detector::OmicsSample>,
    ) -> Result<ComprehensiveReport> {
        self.new_session();

        // 1. Dream about the problem
        let dream_session = self.dream_about(problem_focus)?;

        // 2. Detect patterns if sample provided
        let patterns = if let Some(sample) = omics_sample {
            Some(self.detect_senescence(sample)?)
        } else {
            None
        };

        // 3. Get intervention rankings
        let rankings = self.rank_interventions();

        // 4. Simulate top interventions
        let top_interventions: Vec<_> = rankings.iter()
            .take(3)
            .map(|r| r.intervention_name.as_str())
            .collect();

        let simulations = if !top_interventions.is_empty() {
            let protocol = self.create_protocol(top_interventions);
            Some(self.simulate_protocol(protocol)?)
        } else {
            None
        };

        // 5. Find leverage points
        let leverage_points = self.hallmarks.find_leverage_points();

        Ok(ComprehensiveReport {
            id: Uuid::new_v4(),
            generated_at: chrono::Utc::now(),
            problem_focus: problem_focus.to_string(),
            novel_targets: dream_session.novel_targets,
            detected_patterns: patterns.unwrap_or_default(),
            intervention_rankings: rankings,
            simulation_results: simulations,
            leverage_points,
            key_insights: vec![
                "Analysis complete - see detailed sections".to_string(),
            ],
        })
    }

    /// Get current session
    pub fn current_session(&self) -> Option<&AdvisorSession> {
        self.sessions.last()
    }

    /// Get all sessions
    pub fn sessions(&self) -> &[AdvisorSession] {
        &self.sessions
    }

    /// Access the hallmarks graph directly
    pub fn hallmarks(&self) -> &HallmarksGraph {
        &self.hallmarks
    }

    /// Access the hallmarks graph mutably
    pub fn hallmarks_mut(&mut self) -> &mut HallmarksGraph {
        &mut self.hallmarks
    }
}

/// Personalized longevity advice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedAdvice {
    pub id: Uuid,
    pub biological_age: f64,
    pub chronological_age: f64,
    pub age_acceleration: f64,
    pub urgency: Urgency,
    pub priority_hallmarks: Vec<Hallmark>,
    pub recommended_interventions: Vec<String>,
    pub action_items: Vec<String>,
    pub estimated_potential_years: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Urgency {
    High,
    Medium,
    Low,
    Maintenance,
}

/// Comprehensive analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveReport {
    pub id: Uuid,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub problem_focus: String,
    pub novel_targets: Vec<NovelTarget>,
    pub detected_patterns: Vec<SenescencePattern>,
    pub intervention_rankings: Vec<InterventionRanking>,
    pub simulation_results: Option<SimulationResults>,
    pub leverage_points: Vec<(Hallmark, f64)>,
    pub key_insights: Vec<String>,
}

impl Default for LongevityAdvisor {
    fn default() -> Self {
        Self::new(LongevityConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advisor_creation() {
        let advisor = LongevityAdvisor::default();
        assert!(advisor.sessions.is_empty());
    }

    #[test]
    fn test_new_session() {
        let mut advisor = LongevityAdvisor::default();
        let id = advisor.new_session();
        assert!(!advisor.sessions.is_empty());
        assert_eq!(advisor.current_session().unwrap().id, id);
    }

    #[test]
    fn test_create_protocol() {
        let advisor = LongevityAdvisor::default();
        let protocol = advisor.create_protocol(vec!["Rapamycin", "NMN"]);
        assert_eq!(protocol.interventions.len(), 2);
    }

    #[test]
    fn test_rank_interventions() {
        let mut advisor = LongevityAdvisor::default();
        let rankings = advisor.rank_interventions();
        assert!(!rankings.is_empty());
    }

    #[test]
    fn test_personalized_advice() {
        let mut advisor = LongevityAdvisor::default();
        let advice = advisor.get_personalized_advice(
            70.0,
            60.0,
            vec![Hallmark::CellularSenescence],
        );
        assert!(advice.age_acceleration > 0.0);
        assert_eq!(advice.urgency, Urgency::High);
    }

    #[test]
    fn test_hallmarks_access() {
        let advisor = LongevityAdvisor::default();
        let leverage = advisor.hallmarks().find_leverage_points();
        assert!(!leverage.is_empty());
    }

    #[test]
    fn test_vus_interpretation() {
        let mut advisor = LongevityAdvisor::default();
        advisor.new_session();

        // Create a VUS query for a BRCA1 variant
        let query = query_vus(Gene::BRCA1, "c.1234A>G");
        let result = advisor.interpret_vus(query);

        // Should produce a classification with recommendations
        assert!(!result.recommendations.is_empty());
        assert!(result.confidence >= 0.0);

        // Session should record the query
        let session = advisor.current_session().unwrap();
        assert!(!session.queries.is_empty());
        assert!(matches!(session.queries.last().unwrap().query_type, QueryType::InterpretVUS));
    }
}
