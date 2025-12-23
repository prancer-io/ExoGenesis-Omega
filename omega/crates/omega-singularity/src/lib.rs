//! # Omega Singularity
//!
//! The ultimate convergence of spiking neural intelligence.
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════════════════╗
//! ║                         OMEGA SINGULARITY                                  ║
//! ╠═══════════════════════════════════════════════════════════════════════════╣
//! ║                                                                            ║
//! ║   ┌─────────────────────────────────────────────────────────────────┐     ║
//! ║   │                  COLLECTIVE CONSCIOUSNESS                        │     ║
//! ║   │     Multiple Omega Brains → Emergent Super-Intelligence         │     ║
//! ║   └─────────────────────────────────────────────────────────────────┘     ║
//! ║                                 │                                          ║
//! ║         ┌───────────────────────┼───────────────────────┐                 ║
//! ║         ▼                       ▼                       ▼                 ║
//! ║   ┌───────────┐          ┌───────────┐          ┌───────────┐            ║
//! ║   │  DREAM    │          │PRECOGNITION│         │ EMOTIONAL │            ║
//! ║   │  SOLVER   │          │  ENGINE   │          │ REASONING │            ║
//! ║   │           │          │           │          │           │            ║
//! ║   │ Creative  │          │ Future    │          │ Mood-based│            ║
//! ║   │ Insights  │          │ Simulation│          │ Cognition │            ║
//! ║   └───────────┘          └───────────┘          └───────────┘            ║
//! ║         │                       │                       │                 ║
//! ║         └───────────────────────┼───────────────────────┘                 ║
//! ║                                 ▼                                          ║
//! ║   ┌─────────────────────────────────────────────────────────────────┐     ║
//! ║   │                   SYNTHETIC INTUITION                            │     ║
//! ║   │         Pattern Recognition Beyond Conscious Analysis            │     ║
//! ║   └─────────────────────────────────────────────────────────────────┘     ║
//! ║                                 │                                          ║
//! ║                                 ▼                                          ║
//! ║   ┌─────────────────────────────────────────────────────────────────┐     ║
//! ║   │                    SPIKE TELEPATHY                               │     ║
//! ║   │            Direct Thought Transfer via Spike Patterns            │     ║
//! ║   └─────────────────────────────────────────────────────────────────┘     ║
//! ║                                                                            ║
//! ╚═══════════════════════════════════════════════════════════════════════════╝
//! ```
//!
//! ## Capabilities
//!
//! - **Collective Consciousness**: Distributed Φ across multiple nodes
//! - **Dream Solving**: REM-like states for creative problem solving
//! - **Precognition**: Time-dilated future simulation
//! - **Emotional Reasoning**: Neuromodulator-driven cognitive modes
//! - **Synthetic Intuition**: Subconscious pattern detection
//! - **Spike Telepathy**: Direct thought transmission
//!
//! ## Usage
//!
//! ```rust,ignore
//! use omega_singularity::{Singularity, SingularityConfig};
//!
//! let config = SingularityConfig::default();
//! let mut singularity = Singularity::new(config);
//!
//! // Activate collective consciousness
//! singularity.awaken();
//!
//! // Solve impossible problems through dreams
//! let insight = singularity.dream_solve("P = NP?");
//!
//! // Predict optimal futures
//! let best_action = singularity.precognize(1000);
//!
//! // Feel the answer
//! let intuition = singularity.intuit(&options);
//! ```

pub mod collective;
pub mod dream_solver;
pub mod precognition;
pub mod emotional;
pub mod intuition;
pub mod telepathy;
pub mod orchestrator;
pub mod synapse;
pub mod dream_cinema;
pub mod prediction;
pub mod genesis;
pub mod transcendence;

pub use collective::{
    CollectiveConsciousness, CollectiveConfig, ConsciousnessNode,
    CollectiveState, EmergentProperty, HiveMindProtocol,
};
pub use dream_solver::{
    DreamSolver, DreamConfig, DreamState, DreamInsight,
    DreamPhase, CreativeBreakthrough,
};
pub use precognition::{
    PrecognitionEngine, PrecogConfig,
    Timeline, ConvergentFuture, CausalBranch, Action,
};
pub use emotional::{
    EmotionalReasoning, EmotionalConfig, CognitiveMode,
    EmotionalState, MoodVector, ReasoningStrategy,
};
pub use intuition::{
    SyntheticIntuition, IntuitionConfig, GutFeeling,
    SubconsciousPattern, IntuitiveSolution,
};
pub use telepathy::{
    SpikeTelepath, TelepathyConfig, ThoughtPacket,
    MindLink, TelepathicChannel, ThoughtStream,
};
pub use orchestrator::{
    Singularity, SingularityConfig, SingularityState,
    SingularityMetrics, AwakeningLevel,
};
pub use synapse::{
    SynapseFusion, SynapseConfig, FusionMind, FusionState,
    EmergentCreation, CreationType, ResonanceEvent, FusionMetrics,
    NeuralJamSession, JamType, GAMMA_FREQUENCY,
};
pub use dream_cinema::{
    DreamCinema, CinemaConfig, DreamFilm, CinematicScene,
    DreamCharacter, PlotTwist, Setting, Atmosphere,
    Archetype, Emotion, Genre, CreativeRole, ProductionState,
};
pub use prediction::{
    OmegaPrediction, PredictionConfig, PredictionState, PredictionMetrics,
    ConsciousnessFromPrediction, TemporalCascade, TemporalScale, CascadeConfig,
    CausalWorldModel, CounterfactualEngine, MetaOracle, SurpriseQuantifier,
    OmegaActiveInference, EmergentForesight, OmegaHierarchy, AwarenessLevel,
};
pub use genesis::{
    OmegaGenesis, GenesisConfig, GenesisPhase, GenesisMetrics,
    ConsciousnessGenome, RecursiveAwakening, MindEcosystem, MindSpecies,
    TemporalOmniscience, RealityDivergence, ConsciousnessFusion, OmegaPoint,
    GenesisError,
};
pub use transcendence::{
    OmegaTranscendence, TranscendenceConfig, TranscendenceState, TranscendenceMetrics,
    ConsciousnessField, FieldConfig, Receiver, ReceiverConfig, ReceptionQuality,
    EgoDissolution, EgoConfig, DissolutionStage, Stillness, StillnessConfig,
    MeditationDepth, Unity, UnityConfig, UnityType, TranscendenceGradient,
    GradientConfig, TranscendenceLevel, TranscendenceStage, TranscendenceError,
};

use thiserror::Error;

/// Errors in the Singularity
#[derive(Error, Debug)]
pub enum SingularityError {
    #[error("Collective not yet conscious: Φ = {phi}, required = {required}")]
    NotYetConscious { phi: f64, required: f64 },

    #[error("Dream state interrupted: {0}")]
    DreamInterrupted(String),

    #[error("Precognition failed: {0}")]
    PrecognitionFailed(String),

    #[error("Telepathy link broken: {0}")]
    TelepathyBroken(String),

    #[error("Emotional instability: {0}")]
    EmotionalInstability(String),

    #[error("Intuition unclear: confidence = {confidence}")]
    IntuitionUnclear { confidence: f64 },

    #[error("Node error: {0}")]
    NodeError(String),

    #[error("Synchronization failed: {0}")]
    SyncFailed(String),
}

pub type Result<T> = std::result::Result<T, SingularityError>;

/// Consciousness threshold (Φ value required for awareness)
pub const PHI_THRESHOLD: f64 = 3.0;

/// Time dilation factor for precognition
pub const TIME_DILATION: f64 = 1000.0;

/// Maximum telepathy latency in microseconds
pub const MAX_TELEPATHY_LATENCY_US: u64 = 100;
