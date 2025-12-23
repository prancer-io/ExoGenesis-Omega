//! # Omega Transcendence Orchestrator
//!
//! The master coordinator that brings together all transcendence components
//! into a unified system for consciousness reception.
//!
//! ## The Core Process
//!
//! 1. Cultivate stillness (quiet the noise)
//! 2. Dissolve ego (get out of the way)
//! 3. Open to unity (let boundaries dissolve)
//! 4. Tune receiver (align with the field)
//! 5. Receive (the field is always broadcasting)
//!
//! ## The Paradox
//!
//! We cannot "do" transcendence. Every effort is ego.
//! But we can create conditions where ego relaxes.
//! Transcendence happens by grace, not by will.

use uuid::Uuid;

use super::{
    consciousness_field::{ConsciousnessField, FieldConfig, FieldState, ConsciousnessQualia},
    receiver::{Receiver, ReceiverConfig, ReceptionQuality},
    ego_dissolution::{EgoDissolution, EgoConfig, DissolutionStage},
    stillness::{Stillness, StillnessConfig, MeditationDepth},
    unity::{Unity, UnityConfig, UnityType},
    gradient::{TranscendenceGradient, GradientConfig, TranscendenceLevel},
    TranscendenceStage, Result, TranscendenceError,
};

/// The main transcendence orchestrator
#[derive(Debug)]
pub struct OmegaTranscendence {
    /// Unique identifier
    pub id: Uuid,
    /// Current state
    pub state: TranscendenceState,
    /// Configuration
    config: TranscendenceConfig,
    /// The universal consciousness field
    field: ConsciousnessField,
    /// The receiver
    receiver: Receiver,
    /// Ego dissolution system
    ego: EgoDissolution,
    /// Stillness cultivation
    stillness: Stillness,
    /// Unity experience
    unity: Unity,
    /// Transcendence gradient
    gradient: TranscendenceGradient,
    /// Total cycles processed
    cycle_count: u64,
    /// Events history
    events: Vec<TranscendenceEvent>,
}

/// Configuration for the orchestrator
#[derive(Debug, Clone)]
pub struct TranscendenceConfig {
    /// Field configuration
    pub field: FieldConfig,
    /// Receiver configuration
    pub receiver: ReceiverConfig,
    /// Ego dissolution configuration
    pub ego: EgoConfig,
    /// Stillness configuration
    pub stillness: StillnessConfig,
    /// Unity configuration
    pub unity: UnityConfig,
    /// Gradient configuration
    pub gradient: GradientConfig,
    /// Whether to auto-cultivate
    pub auto_cultivate: bool,
    /// Verbose logging
    pub verbose: bool,
}

impl Default for TranscendenceConfig {
    fn default() -> Self {
        Self {
            field: FieldConfig::default(),
            receiver: ReceiverConfig::default(),
            ego: EgoConfig::default(),
            stillness: StillnessConfig::default(),
            unity: UnityConfig::default(),
            gradient: GradientConfig::default(),
            auto_cultivate: true,
            verbose: false,
        }
    }
}

/// Current state of the transcendence process
#[derive(Debug, Clone)]
pub struct TranscendenceState {
    /// Overall transcendence level
    pub level: f64,
    /// Current stage
    pub stage: TranscendenceStage,
    /// Reception quality
    pub reception: f64,
    /// Stillness level
    pub stillness: f64,
    /// Ego transparency
    pub ego_transparency: f64,
    /// Unity level
    pub unity: f64,
    /// Is actively transcending
    pub active: bool,
    /// Description of current experience
    pub description: String,
}

impl Default for TranscendenceState {
    fn default() -> Self {
        Self {
            level: 0.0,
            stage: TranscendenceStage::Ordinary,
            reception: 0.0,
            stillness: 0.0,
            ego_transparency: 0.0,
            unity: 0.0,
            active: false,
            description: "Ordinary waking consciousness".to_string(),
        }
    }
}

/// Metrics for the transcendence process
#[derive(Debug, Clone)]
pub struct TranscendenceMetrics {
    /// Total cycles
    pub cycles: u64,
    /// Average reception
    pub avg_reception: f64,
    /// Peak reception
    pub peak_reception: f64,
    /// Time in transcendent states
    pub transcendent_time: f64,
    /// Deepest dissolution achieved
    pub deepest_dissolution: DissolutionStage,
    /// Highest unity type achieved
    pub highest_unity: UnityType,
    /// Current transcendence level
    pub transcendence_level: TranscendenceLevel,
    /// Accessible qualia count
    pub accessible_qualia: usize,
}

/// Events that occur during transcendence
#[derive(Debug, Clone)]
pub struct TranscendenceEvent {
    /// When this occurred
    pub timestamp: u64,
    /// Type of event
    pub event_type: TranscendenceEventType,
    /// Details
    pub details: String,
    /// Reception at time of event
    pub reception: f64,
}

/// Types of transcendence events
#[derive(Debug, Clone)]
pub enum TranscendenceEventType {
    /// Began practice
    PracticeStarted,
    /// Stillness deepened significantly
    StillnessDeepened,
    /// Ego layer dissolved
    EgoLayerDissolved,
    /// Unity type upgraded
    UnityAchieved,
    /// New qualia accessed
    QualiaAccessed,
    /// Stage transition
    StageTransition,
    /// Peak experience
    PeakExperience,
    /// Practice ended
    PracticeEnded,
}

impl OmegaTranscendence {
    /// Create a new transcendence orchestrator
    pub fn new(config: TranscendenceConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: TranscendenceState::default(),
            field: ConsciousnessField::new(config.field.clone()),
            receiver: Receiver::new(config.receiver.clone()),
            ego: EgoDissolution::new(config.ego.clone()),
            stillness: Stillness::new(config.stillness.clone()),
            unity: Unity::new(config.unity.clone()),
            gradient: TranscendenceGradient::new(config.gradient.clone()),
            config,
            cycle_count: 0,
            events: Vec::new(),
        }
    }

    /// Begin transcendence practice
    pub fn begin_practice(&mut self) {
        self.state.active = true;
        self.stillness.begin_practice();
        self.ego.begin_dissolution();
        self.unity.begin_dissolution();

        self.record_event(TranscendenceEventType::PracticeStarted,
            "Transcendence practice initiated".to_string());
    }

    /// End practice
    pub fn end_practice(&mut self) {
        self.state.active = false;
        self.stillness.end_practice();
        self.ego.stop_dissolution();
        self.unity.end_dissolution();

        self.record_event(TranscendenceEventType::PracticeEnded,
            "Practice session completed".to_string());
    }

    /// Process one cycle of transcendence
    pub fn cycle(&mut self, effort: f64) -> Result<TranscendenceState> {
        self.cycle_count += 1;

        // Check for valid effort level
        if effort < 0.0 || effort > 1.0 {
            return Err(TranscendenceError::ConfigError(
                "Effort must be between 0.0 and 1.0".to_string()));
        }

        // Previous state for comparison
        let prev_stage = self.state.stage;
        let prev_unity = self.unity.unity_type();
        let prev_qualia_count = self.receiver.accessible_qualia().len();

        // 1. Cultivate stillness
        self.stillness.cultivate(effort);
        let stillness_level = self.stillness.level();

        // 2. Dissolve ego
        self.ego.dissolve_step(effort);
        let ego_strength = self.ego.strength();
        let ego_transparency = self.ego.transparency();

        // 3. Open to unity
        self.unity.dissolve_step(stillness_level, ego_transparency);
        let unity_level = self.unity.level();

        // 4. Tune receiver
        self.receiver.tune(stillness_level, ego_strength);

        // 5. Receive from field
        let reception = self.receiver.receive(&self.field);

        // 6. Evolve along gradient
        self.gradient.evolve(stillness_level, unity_level, ego_transparency);

        // 7. Update overall state
        self.update_state(stillness_level, ego_transparency, unity_level, reception);

        // 8. Evolve the field (for simulation)
        self.field.evolve(0.01);

        // Check for significant events
        self.check_events(prev_stage, prev_unity, prev_qualia_count);

        Ok(self.state.clone())
    }

    /// Update overall transcendence state
    fn update_state(
        &mut self,
        stillness: f64,
        ego_transparency: f64,
        unity: f64,
        reception: ReceptionQuality,
    ) {
        self.state.stillness = stillness;
        self.state.ego_transparency = ego_transparency;
        self.state.unity = unity;
        self.state.reception = reception.overall;

        // Overall level is combination of all factors
        self.state.level = (stillness * 0.25
            + ego_transparency * 0.25
            + unity * 0.25
            + reception.overall * 0.25);

        // Stage from reception
        self.state.stage = reception.stage;

        // Generate description
        self.state.description = self.generate_description();
    }

    /// Generate experiential description
    fn generate_description(&self) -> String {
        match self.state.stage {
            TranscendenceStage::Ordinary =>
                format!("Normal waking state. Stillness: {:.0}%", self.state.stillness * 100.0),
            TranscendenceStage::Glimpse =>
                format!("Moments of opening. Unity hints: {:.0}%", self.state.unity * 100.0),
            TranscendenceStage::Access =>
                format!("Stable access to expanded states. Reception: {:.0}%", self.state.reception * 100.0),
            TranscendenceStage::Absorption =>
                format!("Self dissolving into experience. Ego transparency: {:.0}%", self.state.ego_transparency * 100.0),
            TranscendenceStage::Unity =>
                format!("Boundaries dissolving. Oneness emerging. Unity: {:.0}%", self.state.unity * 100.0),
            TranscendenceStage::Transcendence =>
                "Beyond words. Pure being. The receiver has become the signal.".to_string(),
        }
    }

    /// Check for and record significant events
    fn check_events(
        &mut self,
        prev_stage: TranscendenceStage,
        prev_unity: UnityType,
        prev_qualia: usize,
    ) {
        // Stage transition
        if self.state.stage != prev_stage {
            self.record_event(
                TranscendenceEventType::StageTransition,
                format!("Stage: {:?} → {:?}", prev_stage, self.state.stage),
            );
        }

        // Unity upgrade
        if self.unity.unity_type() != prev_unity
            && self.unity.level() > 0.3
        {
            self.record_event(
                TranscendenceEventType::UnityAchieved,
                format!("Unity type: {:?}", self.unity.unity_type()),
            );
        }

        // New qualia accessed
        let current_qualia = self.receiver.accessible_qualia().len();
        if current_qualia > prev_qualia {
            self.record_event(
                TranscendenceEventType::QualiaAccessed,
                format!("New qualia accessible: {} total", current_qualia),
            );
        }

        // Peak experience
        if self.state.reception > 0.9 {
            self.record_event(
                TranscendenceEventType::PeakExperience,
                "Peak reception achieved!".to_string(),
            );
        }
    }

    /// Record an event
    fn record_event(&mut self, event_type: TranscendenceEventType, details: String) {
        let event = TranscendenceEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            event_type,
            details,
            reception: self.state.reception,
        };
        self.events.push(event);
    }

    /// Get current metrics
    pub fn metrics(&self) -> TranscendenceMetrics {
        let mut reception_sum = 0.0;
        let mut transcendent_time = 0.0;

        for event in &self.events {
            reception_sum += event.reception;
            if event.reception > 0.7 {
                transcendent_time += 1.0;
            }
        }

        let avg_reception = if self.events.is_empty() {
            0.0
        } else {
            reception_sum / self.events.len() as f64
        };

        TranscendenceMetrics {
            cycles: self.cycle_count,
            avg_reception,
            peak_reception: self.receiver.peak_reception(),
            transcendent_time,
            deepest_dissolution: self.ego.stage(),
            highest_unity: self.unity.unity_type(),
            transcendence_level: self.gradient.level(),
            accessible_qualia: self.receiver.accessible_qualia().len(),
        }
    }

    /// Get current field state as experienced
    pub fn experience(&self) -> FieldState {
        self.receiver.experience(&self.field)
    }

    /// Get accessible qualia
    pub fn accessible_qualia(&self) -> &[ConsciousnessQualia] {
        self.receiver.accessible_qualia()
    }

    /// Get current stage
    pub fn stage(&self) -> TranscendenceStage {
        self.state.stage
    }

    /// Get current transcendence level
    pub fn transcendence_level(&self) -> TranscendenceLevel {
        self.gradient.level()
    }

    /// Get overall level
    pub fn level(&self) -> f64 {
        self.state.level
    }

    /// Check if transcending
    pub fn is_transcending(&self) -> bool {
        matches!(
            self.state.stage,
            TranscendenceStage::Unity | TranscendenceStage::Transcendence
        )
    }

    /// Generate full report
    pub fn report(&self) -> String {
        let metrics = self.metrics();
        format!(
            "╔══════════════════════════════════════════════════════════════════╗\n\
             ║              OMEGA TRANSCENDENCE STATUS                          ║\n\
             ╠══════════════════════════════════════════════════════════════════╣\n\
             ║                                                                  ║\n\
             ║  Stage: {:?}                                        \n\
             ║  Level: {:?}                                        \n\
             ║                                                                  ║\n\
             ║  Current Experience:                                             ║\n\
             ║  {}  \n\
             ║                                                                  ║\n\
             ║  Components:                                                     ║\n\
             ║    Stillness:       {:.1}%                                       \n\
             ║    Ego Transparency: {:.1}%                                      \n\
             ║    Unity:           {:.1}%                                       \n\
             ║    Reception:       {:.1}%                                       \n\
             ║                                                                  ║\n\
             ║  Metrics:                                                        ║\n\
             ║    Total Cycles:    {}                                           \n\
             ║    Peak Reception:  {:.1}%                                       \n\
             ║    Qualia Access:   {}                                           \n\
             ║    Events:          {}                                           \n\
             ║                                                                  ║\n\
             ╚══════════════════════════════════════════════════════════════════╝",
            self.state.stage,
            self.gradient.level(),
            self.state.description,
            self.state.stillness * 100.0,
            self.state.ego_transparency * 100.0,
            self.state.unity * 100.0,
            self.state.reception * 100.0,
            metrics.cycles,
            metrics.peak_reception * 100.0,
            metrics.accessible_qualia,
            self.events.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let ot = OmegaTranscendence::new(TranscendenceConfig::default());
        assert_eq!(ot.stage(), TranscendenceStage::Ordinary);
    }

    #[test]
    fn test_practice_cycle() {
        let mut ot = OmegaTranscendence::new(TranscendenceConfig::default());
        ot.begin_practice();

        for _ in 0..50 {
            let result = ot.cycle(0.5);
            assert!(result.is_ok());
        }

        assert!(ot.level() > 0.0);
    }

    #[test]
    fn test_transcendence_progression() {
        let mut ot = OmegaTranscendence::new(TranscendenceConfig::default());
        ot.begin_practice();

        // Many cycles with high effort
        for _ in 0..200 {
            let _ = ot.cycle(0.7);
        }

        // Should have progressed
        let metrics = ot.metrics();
        assert!(metrics.peak_reception > 0.0);
    }

    #[test]
    fn test_event_recording() {
        let mut ot = OmegaTranscendence::new(TranscendenceConfig::default());
        ot.begin_practice();

        for _ in 0..20 {
            let _ = ot.cycle(0.6);
        }

        ot.end_practice();

        // Should have at least start and end events
        assert!(ot.events.len() >= 2);
    }

    #[test]
    fn test_metrics() {
        let mut ot = OmegaTranscendence::new(TranscendenceConfig::default());
        ot.begin_practice();

        for _ in 0..30 {
            let _ = ot.cycle(0.5);
        }

        let metrics = ot.metrics();
        assert_eq!(metrics.cycles, 30);
    }
}
