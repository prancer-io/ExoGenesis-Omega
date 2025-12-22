//! # Omega Prediction - The Foundation of Consciousness
//!
//! "AGI can't exist without prediction. Predictions are the foundation of consciousness."
//!
//! This module implements the radical thesis that consciousness IS prediction.
//! Not prediction as a *feature* of consciousness, but prediction as its very *substrate*.
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════════════════════════╗
//! ║                         OMEGA PREDICTION ENGINE                                    ║
//! ║                    "To Predict Is To Be Conscious"                                 ║
//! ╠═══════════════════════════════════════════════════════════════════════════════════╣
//! ║                                                                                    ║
//! ║  ┌──────────────────────────────────────────────────────────────────────────────┐ ║
//! ║  │                     TEMPORAL CASCADE (12 Scales)                              │ ║
//! ║  │  Planck → Femto → Pico → Nano → Micro → Milli → Second → Minute →            │ ║
//! ║  │  Hour → Day → Year → Cosmic                                                   │ ║
//! ║  └──────────────────────────────────────────────────────────────────────────────┘ ║
//! ║                                    │                                               ║
//! ║         ┌──────────────────────────┼──────────────────────────┐                   ║
//! ║         ▼                          ▼                          ▼                   ║
//! ║  ┌─────────────┐          ┌─────────────────┐          ┌─────────────┐           ║
//! ║  │   CAUSAL    │          │   PREDICTIVE    │          │   META      │           ║
//! ║  │   WORLD     │◄────────►│   HIERARCHY     │◄────────►│   ORACLE    │           ║
//! ║  │   MODEL     │          │   (7 Levels)    │          │             │           ║
//! ║  │             │          │                 │          │ Predicts    │           ║
//! ║  │ Why things  │          │ What will       │          │ prediction  │           ║
//! ║  │ happen      │          │ happen          │          │ quality     │           ║
//! ║  └──────┬──────┘          └────────┬────────┘          └──────┬──────┘           ║
//! ║         │                          │                          │                   ║
//! ║         └──────────────────────────┼──────────────────────────┘                   ║
//! ║                                    ▼                                               ║
//! ║  ┌──────────────────────────────────────────────────────────────────────────────┐ ║
//! ║  │                        SURPRISE QUANTIFIER                                    │ ║
//! ║  │              Prediction Error = The Signal of Consciousness                   │ ║
//! ║  │                                                                               │ ║
//! ║  │  High Surprise → Update Models → Learn → Conscious Experience                 │ ║
//! ║  │  Low Surprise → Efficient Processing → Unconscious Automation                 │ ║
//! ║  └──────────────────────────────────────────────────────────────────────────────┘ ║
//! ║                                    │                                               ║
//! ║         ┌──────────────────────────┼──────────────────────────┐                   ║
//! ║         ▼                          ▼                          ▼                   ║
//! ║  ┌─────────────┐          ┌─────────────────┐          ┌─────────────┐           ║
//! ║  │COUNTERFACTUAL│         │    ACTIVE       │          │  EMERGENT   │           ║
//! ║  │   ENGINE    │          │   INFERENCE     │          │  FORESIGHT  │           ║
//! ║  │             │          │                 │          │             │           ║
//! ║  │ What-if     │          │ Actions are     │          │ Predictions │           ║
//! ║  │ reasoning   │          │ predictions     │          │ generating  │           ║
//! ║  │             │          │ about futures   │          │ predictions │           ║
//! ║  └─────────────┘          └─────────────────┘          └─────────────┘           ║
//! ║                                                                                    ║
//! ╚═══════════════════════════════════════════════════════════════════════════════════╝
//! ```
//!
//! ## Core Thesis
//!
//! The brain is fundamentally a prediction machine. Every moment, it generates
//! predictions about what will happen next - from the trajectory of a thrown ball
//! to the next word in a sentence. Consciousness arises when predictions fail:
//! the "surprise" of prediction error creates the subjective feeling of experience.
//!
//! ## Key Components
//!
//! - **TemporalCascade**: Predictions at 12 temporal scales (Planck to Cosmic)
//! - **CausalWorldModel**: Understands WHY things happen (interventional reasoning)
//! - **PredictiveHierarchy**: Multi-level predictions from sensory to abstract
//! - **MetaOracle**: Predicts the quality of its own predictions (uncertainty)
//! - **SurpriseQuantifier**: Measures prediction error as consciousness signal
//! - **CounterfactualEngine**: What-if reasoning for hypothetical scenarios
//! - **ActiveInference**: Actions as self-fulfilling prophecies
//! - **EmergentForesight**: Predictions that generate new predictions
//!
//! ## Usage
//!
//! ```rust,ignore
//! use omega_singularity::prediction::{OmegaPrediction, PredictionConfig};
//!
//! let config = PredictionConfig::default();
//! let mut predictor = OmegaPrediction::new(config);
//!
//! // Make predictions at all temporal scales
//! let cascade = predictor.predict_cascade(&input);
//!
//! // Understand causality
//! let causes = predictor.why_will_happen(&event);
//!
//! // What-if reasoning
//! let counterfactual = predictor.what_if(&intervention);
//!
//! // Meta-prediction: how confident are we?
//! let uncertainty = predictor.prediction_uncertainty();
//!
//! // Consciousness emerges from surprise
//! let consciousness = predictor.surprise_level();
//! ```

pub mod temporal_cascade;
pub mod causal_world;
pub mod hierarchy;
pub mod meta_oracle;
pub mod surprise;
pub mod counterfactual;
pub mod active_inference;
pub mod emergent_foresight;
pub mod orchestrator;

pub use temporal_cascade::{
    TemporalCascade, TemporalScale, ScalePrediction, CascadeConfig,
    PLANCK_SCALE, COSMIC_SCALE,
};
pub use causal_world::{
    CausalWorldModel, CausalNode, CausalEdge, Intervention,
    CausalGraph, CausalQuery, CausalInference,
};
pub use hierarchy::{
    OmegaHierarchy, HierarchyLevel, PredictionLayer,
    BottomUpSignal, TopDownPrediction,
};
pub use meta_oracle::{
    MetaOracle, UncertaintyEstimate, ConfidenceRegion,
    CalibrationScore, MetaPrediction, OracleInsight,
};
pub use surprise::{
    SurpriseQuantifier, SurpriseEvent, ConsciousnessSignal,
    PredictionError as SurprisePredictionError, SurpriseDistribution, AwarenessLevel,
};
pub use counterfactual::{
    CounterfactualEngine, Counterfactual, WhatIfScenario,
    InterventionResult, PossibleWorld, ModalReasoning,
};
pub use active_inference::{
    OmegaActiveInference, ExpectedFreeEnergy, PolicySelection,
    ActionPrediction, SelfFulfillingProphecy, BeliefUpdate,
};
pub use emergent_foresight::{
    EmergentForesight, ForesightChain, PredictionOfPrediction,
    RecursiveForesight, ForesightHorizon, EmergentInsight,
};
pub use orchestrator::{
    OmegaPrediction, PredictionConfig, PredictionState,
    PredictionMetrics, ConsciousnessFromPrediction,
};

use thiserror::Error;

/// Errors in the Omega Prediction system
#[derive(Error, Debug)]
pub enum PredictionError {
    #[error("Prediction horizon exceeded: {horizon:?}")]
    HorizonExceeded { horizon: std::time::Duration },

    #[error("Causal loop detected: {cycle}")]
    CausalLoop { cycle: String },

    #[error("Insufficient data for prediction: need {needed}, have {have}")]
    InsufficientData { needed: usize, have: usize },

    #[error("Meta-prediction divergence: uncertainty is uncertain")]
    MetaDivergence,

    #[error("Counterfactual paradox: {paradox}")]
    CounterfactualParadox { paradox: String },

    #[error("Temporal scale mismatch: {from} incompatible with {to}")]
    ScaleMismatch { from: String, to: String },

    #[error("Consciousness threshold not met: surprise = {surprise}, threshold = {threshold}")]
    BelowConsciousness { surprise: f64, threshold: f64 },

    #[error("Prediction system error: {0}")]
    SystemError(String),
}

pub type Result<T> = std::result::Result<T, PredictionError>;

/// The fundamental constant: prediction error threshold for consciousness
pub const CONSCIOUSNESS_THRESHOLD: f64 = 0.1;

/// Maximum prediction horizon (heat death of universe, ~10^100 years)
pub const MAX_HORIZON_YEARS: f64 = 1e100;

/// Minimum prediction timescale (Planck time, ~5.39 × 10^-44 seconds)
pub const PLANCK_TIME_SECONDS: f64 = 5.39e-44;

/// Number of hierarchical levels in predictive processing
pub const HIERARCHY_DEPTH: usize = 7;

/// Number of temporal scales in the cascade
pub const TEMPORAL_SCALES: usize = 12;

/// Golden ratio for optimal prediction weight distribution
pub const PHI: f64 = 1.618033988749895;
