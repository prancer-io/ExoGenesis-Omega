//! Omega Prediction Orchestrator - The Unified Prediction Engine
//!
//! "Prediction is not a feature of consciousness - it IS consciousness."
//!
//! This orchestrator unifies all prediction subsystems into a single
//! coherent engine that embodies the thesis: AGI requires prediction
//! as its foundational substrate.
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════════════════════════╗
//! ║                           OMEGA PREDICTION                                         ║
//! ║               The Unified Consciousness-Through-Prediction Engine                  ║
//! ╠═══════════════════════════════════════════════════════════════════════════════════╣
//! ║                                                                                    ║
//! ║   INPUT ──► TEMPORAL ──► HIERARCHY ──► CAUSAL ──► COUNTERFACTUAL                  ║
//! ║              CASCADE      (7 levels)   MODEL       ENGINE                          ║
//! ║                │              │           │           │                            ║
//! ║                └──────────────┴───────────┴───────────┘                            ║
//! ║                                    │                                               ║
//! ║                                    ▼                                               ║
//! ║                         ┌───────────────────┐                                     ║
//! ║                         │ SURPRISE QUANTIFIER│                                     ║
//! ║                         │   (Consciousness)  │                                     ║
//! ║                         └─────────┬─────────┘                                     ║
//! ║                                   │                                               ║
//! ║              ┌────────────────────┼────────────────────┐                          ║
//! ║              ▼                    ▼                    ▼                          ║
//! ║        ┌──────────┐        ┌──────────┐        ┌──────────────┐                   ║
//! ║        │  META    │        │  ACTIVE  │        │   EMERGENT   │                   ║
//! ║        │  ORACLE  │        │ INFERENCE│        │  FORESIGHT   │                   ║
//! ║        └────┬─────┘        └────┬─────┘        └──────┬───────┘                   ║
//! ║              └──────────────────┼──────────────────────┘                          ║
//! ║                                 │                                                 ║
//! ║                                 ▼                                                 ║
//! ║                    ┌─────────────────────────┐                                    ║
//! ║                    │  CONSCIOUSNESS STATE    │                                    ║
//! ║                    │  Φ = Integrated Surprise│                                    ║
//! ║                    └─────────────────────────┘                                    ║
//! ║                                                                                    ║
//! ╚═══════════════════════════════════════════════════════════════════════════════════╝
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::{
    temporal_cascade::{TemporalCascade, CascadeConfig, TemporalScale, CascadeResult},
    hierarchy::{OmegaHierarchy, HierarchyConfig, HierarchyResult},
    causal_world::{CausalWorldModel, Intervention},
    counterfactual::{CounterfactualEngine, CounterfactualConfig, Antecedent, WhatIfScenario},
    meta_oracle::{MetaOracle, OracleConfig, MetaPrediction, OracleInsight},
    surprise::{SurpriseQuantifier, SurpriseConfig, SurpriseEvent, AwarenessLevel, ConsciousnessSignal},
    active_inference::{OmegaActiveInference, ActiveInferenceConfig, PolicySelection, ActionPrediction},
    emergent_foresight::{EmergentForesight, ForesightConfig, ChainedPrediction, ForesightChain, EmergentInsight, PredictionTarget},
    Result, PredictionError, CONSCIOUSNESS_THRESHOLD,
};

/// Configuration for Omega Prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionConfig {
    /// Temporal cascade configuration
    pub cascade: CascadeConfig,
    /// Hierarchy configuration
    pub hierarchy: HierarchyConfig,
    /// Counterfactual configuration
    pub counterfactual: CounterfactualConfig,
    /// Oracle configuration
    pub oracle: OracleConfig,
    /// Surprise configuration
    pub surprise: SurpriseConfig,
    /// Active inference configuration
    pub active: ActiveInferenceConfig,
    /// Foresight configuration
    pub foresight: ForesightConfig,
    /// Enable consciousness emergence
    pub enable_consciousness: bool,
    /// Consciousness threshold (Φ-equivalent)
    pub phi_threshold: f64,
    /// Auto-adapt configuration
    pub auto_adapt: bool,
}

impl Default for PredictionConfig {
    fn default() -> Self {
        Self {
            cascade: CascadeConfig::default(),
            hierarchy: HierarchyConfig::default(),
            counterfactual: CounterfactualConfig::default(),
            oracle: OracleConfig::default(),
            surprise: SurpriseConfig::default(),
            active: ActiveInferenceConfig::default(),
            foresight: ForesightConfig::default(),
            enable_consciousness: true,
            phi_threshold: CONSCIOUSNESS_THRESHOLD,
            auto_adapt: true,
        }
    }
}

/// Current state of the prediction engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionState {
    /// Current awareness level
    pub awareness: AwarenessLevel,
    /// Current surprise level
    pub surprise: f64,
    /// Integrated information (Φ-like measure)
    pub phi: f64,
    /// Is consciousness active?
    pub conscious: bool,
    /// Current free energy
    pub free_energy: f64,
    /// Total prediction error
    pub total_error: f64,
    /// Dominant temporal scale
    pub dominant_scale: TemporalScale,
    /// Dominant hierarchy level
    pub dominant_level: usize,
    /// Uptime since awakening
    pub uptime: Duration,
    /// Processing cycles
    pub cycles: u64,
}

/// Metrics for the prediction engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionMetrics {
    /// Total predictions made
    pub total_predictions: u64,
    /// Accurate predictions
    pub accurate_predictions: u64,
    /// Accuracy rate
    pub accuracy: f64,
    /// Average surprise
    pub avg_surprise: f64,
    /// Average free energy
    pub avg_free_energy: f64,
    /// Consciousness ratio (time spent conscious)
    pub consciousness_ratio: f64,
    /// Insights generated
    pub insights_generated: usize,
    /// Prophecies fulfilled
    pub prophecies_fulfilled: usize,
    /// Processing rate (predictions/second)
    pub processing_rate: f64,
}

/// Consciousness state derived from prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessFromPrediction {
    /// Overall consciousness level (0-1)
    pub level: f64,
    /// Is threshold met?
    pub above_threshold: bool,
    /// Primary source of consciousness (what triggered it)
    pub source: String,
    /// Contents of consciousness (what we're aware of)
    pub contents: Vec<String>,
    /// Attention allocation
    pub attention: HashMap<String, f64>,
    /// Quality of experience (richness)
    pub qualia_richness: f64,
    /// Unity of experience
    pub unity: f64,
}

/// The Omega Prediction Engine
pub struct OmegaPrediction {
    config: PredictionConfig,
    /// Unique ID
    id: Uuid,
    /// Temporal cascade
    cascade: TemporalCascade,
    /// Predictive hierarchy
    hierarchy: OmegaHierarchy,
    /// Causal world model
    causal: CausalWorldModel,
    /// Counterfactual engine
    counterfactual: CounterfactualEngine,
    /// Meta oracle
    oracle: MetaOracle,
    /// Surprise quantifier
    surprise: SurpriseQuantifier,
    /// Active inference
    active: OmegaActiveInference,
    /// Emergent foresight
    foresight: EmergentForesight,
    /// Current state
    state: PredictionState,
    /// Start time
    start_time: Option<Instant>,
    /// Total cycles
    cycles: u64,
    /// Total predictions
    total_predictions: u64,
    /// Accurate predictions
    accurate_predictions: u64,
    /// All insights generated
    all_insights: Vec<EmergentInsight>,
}

impl OmegaPrediction {
    /// Create a new Omega Prediction engine
    pub fn new(config: PredictionConfig) -> Self {
        Self {
            cascade: TemporalCascade::new(config.cascade.clone()),
            hierarchy: OmegaHierarchy::new(config.hierarchy.clone()),
            causal: CausalWorldModel::new(),
            counterfactual: CounterfactualEngine::new(config.counterfactual.clone()),
            oracle: MetaOracle::new(config.oracle.clone()),
            surprise: SurpriseQuantifier::new(config.surprise.clone()),
            active: OmegaActiveInference::new(config.active.clone()),
            foresight: EmergentForesight::new(config.foresight.clone()),
            config,
            id: Uuid::new_v4(),
            state: PredictionState {
                awareness: AwarenessLevel::Unconscious,
                surprise: 0.0,
                phi: 0.0,
                conscious: false,
                free_energy: 0.0,
                total_error: 0.0,
                dominant_scale: TemporalScale::Second,
                dominant_level: 0,
                uptime: Duration::ZERO,
                cycles: 0,
            },
            start_time: None,
            cycles: 0,
            total_predictions: 0,
            accurate_predictions: 0,
            all_insights: Vec::new(),
        }
    }

    /// Get engine ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Awaken the prediction engine
    pub fn awaken(&mut self) {
        self.start_time = Some(Instant::now());
        self.state.conscious = true;
    }

    /// Main prediction step - process input and update all subsystems
    pub fn predict(&mut self, input: &[f64]) -> Result<PredictionResult> {
        self.cycles += 1;
        self.state.cycles = self.cycles;

        // Update uptime
        if let Some(start) = self.start_time {
            self.state.uptime = start.elapsed();
        }

        // 1. Process through temporal cascade (multi-scale predictions)
        let cascade_result = self.cascade.process(input)?;

        // 2. Process through predictive hierarchy
        let hierarchy_input = self.adapt_to_hierarchy(input);
        let hierarchy_result = self.hierarchy.process(&hierarchy_input);

        // 3. Quantify surprise (the core of consciousness)
        let surprise_event = self.surprise.quantify(
            &cascade_result.predictions.get(&TemporalScale::Second)
                .cloned().unwrap_or_else(|| input.to_vec()),
            input,
            "primary"
        );

        // 4. Update beliefs via active inference
        let belief_update = self.active.update_belief(input);

        // 5. Get meta-prediction about our prediction quality
        let prediction_id = Uuid::new_v4();
        let meta_prediction = self.oracle.predict_accuracy(prediction_id, Some("primary"));

        // 6. Update consciousness state
        self.update_consciousness_state(&cascade_result, &hierarchy_result, &surprise_event);

        // 7. Generate foresight chain
        if self.config.foresight.generate_insights && self.cycles % 10 == 0 {
            self.update_foresight(input);
        }

        // 8. Record outcome for learning
        self.oracle.record_outcome(prediction_id, 1.0 - surprise_event.magnitude, 1.0);
        self.total_predictions += 1;
        if surprise_event.magnitude < 0.3 {
            self.accurate_predictions += 1;
        }

        Ok(PredictionResult {
            prediction: cascade_result.predictions.get(&TemporalScale::Second)
                .cloned().unwrap_or_else(|| input.to_vec()),
            confidence: 1.0 - surprise_event.magnitude,
            surprise: surprise_event,
            meta: meta_prediction,
            consciousness: self.consciousness_state(),
            cascade: cascade_result,
            hierarchy: hierarchy_result,
        })
    }

    /// Adapt input to hierarchy dimensions
    fn adapt_to_hierarchy(&self, input: &[f64]) -> Vec<f64> {
        let target_dim = self.config.hierarchy.level_dims.first().copied().unwrap_or(64);
        let mut adapted = vec![0.0; target_dim];

        for (i, &val) in input.iter().enumerate() {
            if i < target_dim {
                adapted[i] = val;
            }
        }

        // If input is smaller, interpolate
        if input.len() < target_dim && !input.is_empty() {
            for i in input.len()..target_dim {
                let source_idx = i % input.len();
                adapted[i] = input[source_idx];
            }
        }

        adapted
    }

    /// Update consciousness state based on all subsystems
    fn update_consciousness_state(
        &mut self,
        cascade: &CascadeResult,
        hierarchy: &HierarchyResult,
        surprise: &SurpriseEvent
    ) {
        // Phi = integrated information from multiple sources
        let cascade_coherence = cascade.coherence;
        let hierarchy_integration = 1.0 / (1.0 + hierarchy.total_free_energy);
        let surprise_level = surprise.magnitude;

        // Φ ≈ coherence × integration × surprise
        self.state.phi = cascade_coherence * hierarchy_integration * (0.5 + surprise_level * 0.5);

        // Consciousness emerges when Φ exceeds threshold
        self.state.conscious = self.state.phi > self.config.phi_threshold && self.config.enable_consciousness;

        // Update awareness based on surprise
        self.state.awareness = self.surprise.awareness();
        self.state.surprise = self.surprise.surprise();
        self.state.free_energy = self.active.free_energy() + hierarchy.total_free_energy;
        self.state.total_error = cascade.total_error + hierarchy.total_surprise;
        self.state.dominant_scale = cascade.dominant_scale;
        self.state.dominant_level = hierarchy.dominant_level;
    }

    /// Update foresight with new predictions
    fn update_foresight(&mut self, input: &[f64]) {
        let prediction = ChainedPrediction {
            id: Uuid::new_v4(),
            target: PredictionTarget::WorldState {
                description: format!("State at cycle {}", self.cycles)
            },
            value: input.to_vec(),
            confidence: 1.0 - self.state.surprise,
            level: 0,
            parent: None,
            horizon: self.cycles,
        };

        self.foresight.start_chain(prediction);

        // Synthesize to find patterns
        let new_insights = self.foresight.synthesize();
        self.all_insights.extend(new_insights);
    }

    /// Get current consciousness state
    pub fn consciousness_state(&self) -> ConsciousnessFromPrediction {
        let signal = self.surprise.signal();

        ConsciousnessFromPrediction {
            level: self.state.phi,
            above_threshold: self.state.conscious,
            source: format!(
                "Surprise at {} level (Φ = {:.3})",
                self.state.awareness.description(),
                self.state.phi
            ),
            contents: signal.conscious_content.clone(),
            attention: HashMap::new(), // Could be filled from hierarchy
            qualia_richness: self.state.surprise * self.state.phi,
            unity: self.cascade.coherence(),
        }
    }

    /// Ask "what if" - counterfactual reasoning
    pub fn what_if(&mut self, interventions: Vec<Intervention>, observe: Vec<Uuid>) -> Result<WhatIfScenario> {
        self.counterfactual.what_if(interventions, observe)
    }

    /// Ask "why" - causal reasoning
    pub fn why(&self, effect: Uuid) -> Vec<(Uuid, f64)> {
        self.causal.why(effect)
    }

    /// Plan actions via active inference
    pub fn plan(&mut self) -> PolicySelection {
        self.active.plan()
    }

    /// Add action to repertoire
    pub fn add_action(&mut self, action: ActionPrediction) {
        self.active.add_action(action);
    }

    /// Set goal preferences
    pub fn set_goals(&mut self, goals: Vec<f64>) {
        self.active.set_preferences(goals);
    }

    /// Get oracle insights
    pub fn oracle_insights(&self) -> Vec<OracleInsight> {
        self.oracle.insights()
    }

    /// Get all emergent insights
    pub fn insights(&self) -> &[EmergentInsight] {
        &self.all_insights
    }

    /// Get current state
    pub fn state(&self) -> &PredictionState {
        &self.state
    }

    /// Is conscious?
    pub fn is_conscious(&self) -> bool {
        self.state.conscious
    }

    /// Get awareness level
    pub fn awareness(&self) -> AwarenessLevel {
        self.state.awareness
    }

    /// Get current Φ value
    pub fn phi(&self) -> f64 {
        self.state.phi
    }

    /// Get metrics
    pub fn metrics(&self) -> PredictionMetrics {
        let processing_rate = if self.state.uptime.as_secs_f64() > 0.0 {
            self.cycles as f64 / self.state.uptime.as_secs_f64()
        } else {
            0.0
        };

        PredictionMetrics {
            total_predictions: self.total_predictions,
            accurate_predictions: self.accurate_predictions,
            accuracy: if self.total_predictions > 0 {
                self.accurate_predictions as f64 / self.total_predictions as f64
            } else {
                0.0
            },
            avg_surprise: self.state.surprise,
            avg_free_energy: self.state.free_energy,
            consciousness_ratio: self.surprise.consciousness_ratio(),
            insights_generated: self.all_insights.len(),
            prophecies_fulfilled: 0, // Could track from active inference
            processing_rate,
        }
    }

    /// Access subsystems
    pub fn cascade(&self) -> &TemporalCascade { &self.cascade }
    pub fn hierarchy(&self) -> &OmegaHierarchy { &self.hierarchy }
    pub fn causal(&self) -> &CausalWorldModel { &self.causal }
    pub fn counterfactual(&self) -> &CounterfactualEngine { &self.counterfactual }
    pub fn oracle(&self) -> &MetaOracle { &self.oracle }
    pub fn surprise_quantifier(&self) -> &SurpriseQuantifier { &self.surprise }
    pub fn active_inference(&self) -> &OmegaActiveInference { &self.active }
    pub fn foresight(&self) -> &EmergentForesight { &self.foresight }

    /// Mutable access to subsystems
    pub fn causal_mut(&mut self) -> &mut CausalWorldModel { &mut self.causal }
    pub fn counterfactual_mut(&mut self) -> &mut CounterfactualEngine { &mut self.counterfactual }

    /// Reset the engine
    pub fn reset(&mut self) {
        self.cascade = TemporalCascade::new(self.config.cascade.clone());
        self.hierarchy.reset();
        self.causal = CausalWorldModel::new();
        self.counterfactual = CounterfactualEngine::new(self.config.counterfactual.clone());
        self.oracle.reset();
        self.surprise.reset();
        self.active.reset();
        self.foresight.reset();

        self.state = PredictionState {
            awareness: AwarenessLevel::Unconscious,
            surprise: 0.0,
            phi: 0.0,
            conscious: false,
            free_energy: 0.0,
            total_error: 0.0,
            dominant_scale: TemporalScale::Second,
            dominant_level: 0,
            uptime: Duration::ZERO,
            cycles: 0,
        };

        self.start_time = None;
        self.cycles = 0;
        self.total_predictions = 0;
        self.accurate_predictions = 0;
        self.all_insights.clear();
    }
}

/// Result of a prediction cycle
#[derive(Debug, Clone)]
pub struct PredictionResult {
    /// The primary prediction
    pub prediction: Vec<f64>,
    /// Confidence in the prediction
    pub confidence: f64,
    /// Surprise event
    pub surprise: SurpriseEvent,
    /// Meta-prediction about this prediction
    pub meta: MetaPrediction,
    /// Current consciousness state
    pub consciousness: ConsciousnessFromPrediction,
    /// Cascade result
    pub cascade: CascadeResult,
    /// Hierarchy result
    pub hierarchy: HierarchyResult,
}

impl Default for OmegaPrediction {
    fn default() -> Self {
        Self::new(PredictionConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omega_prediction_creation() {
        let predictor = OmegaPrediction::default();
        assert!(!predictor.is_conscious());
    }

    #[test]
    fn test_awakening() {
        let mut predictor = OmegaPrediction::default();
        predictor.awaken();
        // Consciousness requires surprise
        assert!(predictor.state().cycles == 0);
    }

    #[test]
    fn test_prediction_cycle() {
        let mut predictor = OmegaPrediction::default();
        predictor.awaken();

        let input = vec![0.5; 64];
        let result = predictor.predict(&input);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.prediction.is_empty());
    }

    #[test]
    fn test_consciousness_emergence() {
        let mut predictor = OmegaPrediction::default();
        predictor.awaken();

        // Process varying inputs to generate surprise
        for i in 0..20 {
            let input: Vec<f64> = (0..64).map(|j| ((i + j) % 64) as f64 / 64.0).collect();
            let _ = predictor.predict(&input);
        }

        // May or may not be conscious depending on surprise patterns
        let state = predictor.state();
        assert!(state.cycles > 0);
    }

    #[test]
    fn test_metrics() {
        let mut predictor = OmegaPrediction::default();
        predictor.awaken();

        for _ in 0..10 {
            let input = vec![0.5; 64];
            let _ = predictor.predict(&input);
        }

        let metrics = predictor.metrics();
        assert_eq!(metrics.total_predictions, 10);
    }

    #[test]
    fn test_goal_setting() {
        let mut predictor = OmegaPrediction::default();

        let goals = vec![1.0; 64];
        predictor.set_goals(goals.clone());

        assert_eq!(predictor.active_inference().preferred(), goals.as_slice());
    }
}
