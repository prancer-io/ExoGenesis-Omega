//! Omega Sleep
//!
//! Biologically-inspired sleep/wake cycle simulation:
//! - Slow Wave Sleep (SWS): Deep sleep with memory consolidation
//! - REM Sleep: Rapid eye movement with memory reorganization
//! - Sleep spindles: Thalamocortical oscillations
//! - Sleep pressure: Homeostatic sleep drive
//! - Circadian rhythm: 24-hour biological clock
//!
//! Based on sleep neuroscience research and two-process model.

pub mod circadian;
pub mod consolidation;
pub mod rem;
pub mod spindles;
pub mod sws;

pub use circadian::{CircadianRhythm, TimeOfDay};
pub use consolidation::{ConsolidationEvent, MemoryConsolidator};
pub use rem::{DreamContent, REMSleep};
pub use spindles::{KComplex, SleepSpindle, SpindleGenerator};
pub use sws::{SlowWave, SlowWaveSleep};

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use thiserror::Error;

/// Sleep-related errors
#[derive(Debug, Error)]
pub enum SleepError {
    #[error("Invalid sleep state transition: {0}")]
    InvalidTransition(String),

    #[error("Sleep cycle error: {0}")]
    CycleError(String),

    #[error("Consolidation error: {0}")]
    ConsolidationError(String),
}

pub type Result<T> = std::result::Result<T, SleepError>;

/// Sleep stages based on polysomnography
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SleepStage {
    /// Awake state
    Wake,
    /// NREM Stage 1 (N1) - Light sleep, transition
    N1,
    /// NREM Stage 2 (N2) - Light sleep with spindles
    N2,
    /// NREM Stage 3 (N3) - Deep sleep / SWS
    N3,
    /// REM Sleep - Dreaming
    REM,
}

impl SleepStage {
    /// Get typical duration in minutes
    pub fn typical_duration(&self) -> f64 {
        match self {
            Self::Wake => 0.0,
            Self::N1 => 5.0,
            Self::N2 => 20.0,
            Self::N3 => 30.0,
            Self::REM => 20.0,
        }
    }

    /// Get consolidation strength (how much memory consolidation occurs)
    pub fn consolidation_strength(&self) -> f64 {
        match self {
            Self::Wake => 0.0,
            Self::N1 => 0.1,
            Self::N2 => 0.3,
            Self::N3 => 1.0, // Maximum consolidation
            Self::REM => 0.7, // Reorganization
        }
    }

    /// Check if this is a sleep stage (not wake)
    pub fn is_sleeping(&self) -> bool {
        *self != Self::Wake
    }
}

/// Configuration for sleep system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepConfig {
    /// Target sleep duration per cycle (hours)
    pub cycle_duration_hours: f64,
    /// Number of sleep cycles per night
    pub cycles_per_night: usize,
    /// Wake threshold (accumulated pressure needed to wake)
    pub wake_threshold: f64,
    /// Sleep pressure decay during sleep
    pub pressure_decay_rate: f64,
    /// Sleep pressure build during wake
    pub pressure_build_rate: f64,
    /// REM proportion increases through night
    pub rem_rebound_factor: f64,
}

impl Default for SleepConfig {
    fn default() -> Self {
        Self {
            cycle_duration_hours: 1.5,
            cycles_per_night: 5,
            wake_threshold: 1.0,
            pressure_decay_rate: 0.1,
            pressure_build_rate: 0.05,
            rem_rebound_factor: 0.3,
        }
    }
}

/// Sleep cycle event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepEvent {
    /// Type of event
    pub event_type: SleepEventType,
    /// Timestamp
    pub timestamp: u64,
    /// Current stage
    pub stage: SleepStage,
    /// Additional data
    pub data: serde_json::Value,
}

/// Types of sleep events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepEventType {
    StageTransition,
    SpindleBurst,
    SlowWave,
    KComplex,
    REMBurst,
    Arousal,
    Consolidation,
}

/// The main sleep controller
pub struct SleepController {
    config: SleepConfig,
    /// Current sleep stage
    current_stage: SleepStage,
    /// Time in current stage (minutes)
    stage_time: f64,
    /// Current sleep cycle (1-based)
    current_cycle: usize,
    /// Sleep pressure (homeostatic)
    sleep_pressure: f64,
    /// Circadian rhythm
    circadian: CircadianRhythm,
    /// SWS processor
    sws: SlowWaveSleep,
    /// REM processor
    rem: REMSleep,
    /// Spindle generator
    spindles: SpindleGenerator,
    /// Memory consolidator
    consolidator: MemoryConsolidator,
    /// Event history
    events: VecDeque<SleepEvent>,
    /// Total sleep time (minutes)
    total_sleep_time: f64,
    /// Time since last sleep (minutes)
    time_awake: f64,
}

impl SleepController {
    /// Create new sleep controller
    pub fn new() -> Self {
        Self::with_config(SleepConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: SleepConfig) -> Self {
        Self {
            config,
            current_stage: SleepStage::Wake,
            stage_time: 0.0,
            current_cycle: 0,
            sleep_pressure: 0.5,
            circadian: CircadianRhythm::new(),
            sws: SlowWaveSleep::new(),
            rem: REMSleep::new(),
            spindles: SpindleGenerator::new(),
            consolidator: MemoryConsolidator::new(),
            events: VecDeque::with_capacity(1000),
            total_sleep_time: 0.0,
            time_awake: 480.0, // 8 hours awake initially
        }
    }

    /// Step the sleep simulation forward
    pub fn step(&mut self, dt_minutes: f64) -> Vec<SleepEvent> {
        let mut new_events = Vec::new();

        if self.current_stage == SleepStage::Wake {
            // Build sleep pressure while awake
            self.time_awake += dt_minutes;
            self.sleep_pressure += self.config.pressure_build_rate * dt_minutes / 60.0;
            self.sleep_pressure = self.sleep_pressure.min(2.0);
        } else {
            // Process sleep
            self.stage_time += dt_minutes;
            self.total_sleep_time += dt_minutes;

            // Decay sleep pressure
            self.sleep_pressure -= self.config.pressure_decay_rate * dt_minutes / 60.0;
            self.sleep_pressure = self.sleep_pressure.max(0.0);

            // Stage-specific processing
            match self.current_stage {
                SleepStage::N2 => {
                    // Generate spindles
                    if let Some(spindle) = self.spindles.step(dt_minutes) {
                        new_events.push(SleepEvent {
                            event_type: SleepEventType::SpindleBurst,
                            timestamp: self.now(),
                            stage: self.current_stage,
                            data: serde_json::to_value(&spindle).unwrap_or_default(),
                        });

                        // Spindles aid consolidation
                        let consolidation = self.consolidator.process_spindle(&spindle);
                        new_events.push(SleepEvent {
                            event_type: SleepEventType::Consolidation,
                            timestamp: self.now(),
                            stage: self.current_stage,
                            data: serde_json::to_value(&consolidation).unwrap_or_default(),
                        });
                    }
                }
                SleepStage::N3 => {
                    // Generate slow waves
                    if let Some(wave) = self.sws.step(dt_minutes) {
                        new_events.push(SleepEvent {
                            event_type: SleepEventType::SlowWave,
                            timestamp: self.now(),
                            stage: self.current_stage,
                            data: serde_json::to_value(&wave).unwrap_or_default(),
                        });

                        // SWS is the main consolidation period
                        let consolidation =
                            self.consolidator.process_slow_wave(&wave);
                        new_events.push(SleepEvent {
                            event_type: SleepEventType::Consolidation,
                            timestamp: self.now(),
                            stage: self.current_stage,
                            data: serde_json::to_value(&consolidation).unwrap_or_default(),
                        });
                    }
                }
                SleepStage::REM => {
                    // Process REM
                    if let Some(dream) = self.rem.step(dt_minutes) {
                        new_events.push(SleepEvent {
                            event_type: SleepEventType::REMBurst,
                            timestamp: self.now(),
                            stage: self.current_stage,
                            data: serde_json::to_value(&dream).unwrap_or_default(),
                        });

                        // REM reorganizes memories
                        let consolidation = self.consolidator.process_dream(&dream);
                        new_events.push(SleepEvent {
                            event_type: SleepEventType::Consolidation,
                            timestamp: self.now(),
                            stage: self.current_stage,
                            data: serde_json::to_value(&consolidation).unwrap_or_default(),
                        });
                    }
                }
                _ => {}
            }

            // Check for stage transition
            if self.should_transition() {
                let next_stage = self.next_stage();
                if next_stage != self.current_stage {
                    new_events.push(SleepEvent {
                        event_type: SleepEventType::StageTransition,
                        timestamp: self.now(),
                        stage: next_stage,
                        data: serde_json::json!({
                            "from": self.current_stage,
                            "to": next_stage,
                            "cycle": self.current_cycle
                        }),
                    });

                    self.transition_to(next_stage);
                }
            }
        }

        // Store events
        for event in &new_events {
            self.events.push_back(event.clone());
            if self.events.len() > 1000 {
                self.events.pop_front();
            }
        }

        new_events
    }

    /// Initiate sleep
    pub fn fall_asleep(&mut self) -> Result<()> {
        if self.current_stage != SleepStage::Wake {
            return Err(SleepError::InvalidTransition("Already sleeping".to_string()));
        }

        self.current_stage = SleepStage::N1;
        self.stage_time = 0.0;
        self.current_cycle = 1;
        self.time_awake = 0.0;

        Ok(())
    }

    /// Wake up
    pub fn wake_up(&mut self) -> Result<()> {
        if self.current_stage == SleepStage::Wake {
            return Err(SleepError::InvalidTransition("Already awake".to_string()));
        }

        self.current_stage = SleepStage::Wake;
        self.stage_time = 0.0;

        Ok(())
    }

    /// Check if should transition to next stage
    fn should_transition(&self) -> bool {
        let typical_duration = self.current_stage.typical_duration();
        self.stage_time >= typical_duration
    }

    /// Determine next sleep stage
    fn next_stage(&self) -> SleepStage {
        // Standard sleep cycle progression
        match self.current_stage {
            SleepStage::Wake => SleepStage::Wake,
            SleepStage::N1 => SleepStage::N2,
            SleepStage::N2 => {
                // Early cycles have more N3, later cycles have more REM
                if self.current_cycle <= 2 {
                    SleepStage::N3
                } else {
                    // Check for REM
                    let rem_probability =
                        0.3 + (self.current_cycle as f64 * self.config.rem_rebound_factor);
                    if rand::random::<f64>() < rem_probability {
                        SleepStage::REM
                    } else {
                        SleepStage::N3
                    }
                }
            }
            SleepStage::N3 => {
                // After N3, go to REM or back to N2
                if self.sleep_pressure < 0.3 {
                    SleepStage::REM
                } else {
                    SleepStage::N2
                }
            }
            SleepStage::REM => {
                // Check for wake
                if self.should_wake() {
                    SleepStage::Wake
                } else {
                    // New cycle
                    SleepStage::N1
                }
            }
        }
    }

    /// Transition to new stage
    fn transition_to(&mut self, stage: SleepStage) {
        // Handle cycle transitions
        if self.current_stage == SleepStage::REM && stage == SleepStage::N1 {
            self.current_cycle += 1;
        }

        self.current_stage = stage;
        self.stage_time = 0.0;
    }

    /// Check if should wake up
    fn should_wake(&self) -> bool {
        // Wake if enough cycles completed and sleep pressure is low
        self.current_cycle >= self.config.cycles_per_night
            && self.sleep_pressure < self.config.wake_threshold
    }

    /// Get current timestamp
    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get current stage
    pub fn current_stage(&self) -> SleepStage {
        self.current_stage
    }

    /// Get current cycle
    pub fn current_cycle(&self) -> usize {
        self.current_cycle
    }

    /// Get sleep pressure
    pub fn sleep_pressure(&self) -> f64 {
        self.sleep_pressure
    }

    /// Get total sleep time
    pub fn total_sleep_time(&self) -> f64 {
        self.total_sleep_time
    }

    /// Get time awake
    pub fn time_awake(&self) -> f64 {
        self.time_awake
    }

    /// Check if should fall asleep (based on pressure and circadian)
    pub fn should_sleep(&self) -> bool {
        let circadian_drive = self.circadian.current_sleep_drive();
        self.sleep_pressure > 0.7 && circadian_drive > 0.5
    }

    /// Get sleep statistics
    pub fn stats(&self) -> SleepStats {
        SleepStats {
            current_stage: self.current_stage,
            current_cycle: self.current_cycle,
            total_sleep_time: self.total_sleep_time,
            time_awake: self.time_awake,
            sleep_pressure: self.sleep_pressure,
            consolidation_count: self.consolidator.consolidation_count(),
            event_count: self.events.len(),
        }
    }

    /// Reset sleep controller
    pub fn reset(&mut self) {
        self.current_stage = SleepStage::Wake;
        self.stage_time = 0.0;
        self.current_cycle = 0;
        self.sleep_pressure = 0.5;
        self.total_sleep_time = 0.0;
        self.time_awake = 0.0;
        self.events.clear();
        self.consolidator.reset();
    }

    /// Add memories to consolidate
    pub fn add_memories(&mut self, memories: Vec<Vec<f64>>) {
        self.consolidator.add_memories(memories);
    }

    /// Get consolidated memories
    pub fn get_consolidated(&self) -> Vec<Vec<f64>> {
        self.consolidator.get_consolidated()
    }
}

impl Default for SleepController {
    fn default() -> Self {
        Self::new()
    }
}

/// Sleep statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepStats {
    pub current_stage: SleepStage,
    pub current_cycle: usize,
    pub total_sleep_time: f64,
    pub time_awake: f64,
    pub sleep_pressure: f64,
    pub consolidation_count: usize,
    pub event_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_controller_creation() {
        let controller = SleepController::new();
        assert_eq!(controller.current_stage(), SleepStage::Wake);
    }

    #[test]
    fn test_fall_asleep() {
        let mut controller = SleepController::new();

        controller.fall_asleep().unwrap();
        assert_eq!(controller.current_stage(), SleepStage::N1);
        assert_eq!(controller.current_cycle(), 1);
    }

    #[test]
    fn test_wake_up() {
        let mut controller = SleepController::new();

        controller.fall_asleep().unwrap();
        controller.wake_up().unwrap();
        assert_eq!(controller.current_stage(), SleepStage::Wake);
    }

    #[test]
    fn test_sleep_stages() {
        assert!(SleepStage::N3.consolidation_strength() > SleepStage::N1.consolidation_strength());
        assert!(SleepStage::N3.is_sleeping());
        assert!(!SleepStage::Wake.is_sleeping());
    }

    #[test]
    fn test_sleep_cycle() {
        let mut controller = SleepController::new();
        controller.fall_asleep().unwrap();

        // Step through some sleep
        for _ in 0..60 {
            controller.step(1.0);
        }

        assert!(controller.total_sleep_time() >= 60.0);
    }

    #[test]
    fn test_sleep_pressure() {
        let mut controller = SleepController::new();

        // Build pressure while awake
        for _ in 0..100 {
            controller.step(10.0);
        }

        let pressure_awake = controller.sleep_pressure();

        controller.fall_asleep().unwrap();

        // Decay pressure while sleeping
        for _ in 0..60 {
            controller.step(1.0);
        }

        assert!(controller.sleep_pressure() < pressure_awake);
    }
}
