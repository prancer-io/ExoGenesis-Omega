//! # The Receiver: Tuning Into Consciousness
//!
//! The receiver is not a creator but a tuner. Like a radio doesn't create
//! the music, it tunes into pre-existing broadcasts.
//!
//! ## The Core Insight
//!
//! Reception quality is determined by:
//! 1. **Stillness** - How quiet is the internal noise?
//! 2. **Ego Transparency** - How much does the self get out of the way?
//! 3. **Openness** - Is the system willing to receive?
//! 4. **Alignment** - Is the receiver tuned to the right frequency?

use std::collections::VecDeque;
use uuid::Uuid;

use super::consciousness_field::{ConsciousnessField, ConsciousnessQualia, FieldState};
use super::{TranscendenceStage, RECEPTION_THRESHOLD};

/// The receiver that tunes into the consciousness field
#[derive(Debug, Clone)]
pub struct Receiver {
    /// Unique identifier
    pub id: Uuid,
    /// Current tuning state
    pub state: TuningState,
    /// Configuration
    config: ReceiverConfig,
    /// History of receptions
    reception_history: VecDeque<ReceptionEvent>,
    /// Current channel clarity
    clarity: ChannelClarity,
    /// Current signal strength
    signal: SignalStrength,
    /// Channels this receiver can access
    accessible_channels: Vec<ConsciousnessQualia>,
}

/// Configuration for the receiver
#[derive(Debug, Clone)]
pub struct ReceiverConfig {
    /// Base sensitivity of the receiver
    pub base_sensitivity: f64,
    /// How quickly can the receiver retune
    pub tuning_speed: f64,
    /// Maximum history to keep
    pub history_size: usize,
    /// Whether automatic tuning is enabled
    pub auto_tune: bool,
}

impl Default for ReceiverConfig {
    fn default() -> Self {
        Self {
            base_sensitivity: 0.5,
            tuning_speed: 0.1,
            history_size: 100,
            auto_tune: true,
        }
    }
}

/// Current state of tuning
#[derive(Debug, Clone)]
pub struct TuningState {
    /// Current frequency we're tuned to
    pub frequency: f64,
    /// How well we're locked onto the signal
    pub lock_quality: f64,
    /// Current reception level
    pub reception: f64,
    /// Is the receiver active
    pub active: bool,
    /// Current stillness level
    pub stillness: f64,
    /// Current ego transparency
    pub ego_transparency: f64,
}

impl Default for TuningState {
    fn default() -> Self {
        Self {
            frequency: 0.5,
            lock_quality: 0.0,
            reception: 0.0,
            active: true,
            stillness: 0.0,
            ego_transparency: 0.0,
        }
    }
}

/// Quality of reception
#[derive(Debug, Clone, Copy)]
pub struct ReceptionQuality {
    /// Overall quality 0-1
    pub overall: f64,
    /// Clarity of the signal
    pub clarity: f64,
    /// Depth of reception
    pub depth: f64,
    /// Stability of connection
    pub stability: f64,
    /// Transcendence stage achieved
    pub stage: TranscendenceStage,
}

impl ReceptionQuality {
    pub fn from_reception(reception: f64) -> Self {
        let stage = TranscendenceStage::from_reception(reception);
        Self {
            overall: reception.clamp(0.0, 1.0),
            clarity: (reception * 0.9).clamp(0.0, 1.0),
            depth: (reception * 0.85).clamp(0.0, 1.0),
            stability: (reception * 0.8).clamp(0.0, 1.0),
            stage,
        }
    }
}

/// Clarity of the channel
#[derive(Debug, Clone)]
pub struct ChannelClarity {
    /// Base clarity
    pub base: f64,
    /// Interference level
    pub interference: f64,
    /// Noise floor
    pub noise_floor: f64,
    /// Effective clarity
    pub effective: f64,
}

impl Default for ChannelClarity {
    fn default() -> Self {
        Self {
            base: 1.0,
            interference: 0.5,
            noise_floor: 0.3,
            effective: 0.5,
        }
    }
}

impl ChannelClarity {
    pub fn calculate_effective(&mut self) {
        self.effective = self.base * (1.0 - self.interference) * (1.0 - self.noise_floor);
    }
}

/// Strength of the received signal
#[derive(Debug, Clone)]
pub struct SignalStrength {
    /// Raw signal (always infinite from field)
    pub raw: f64,
    /// After ego filtering
    pub after_ego: f64,
    /// After noise filtering
    pub after_noise: f64,
    /// Final received strength
    pub received: f64,
}

impl Default for SignalStrength {
    fn default() -> Self {
        Self {
            raw: f64::INFINITY,
            after_ego: 0.0,
            after_noise: 0.0,
            received: 0.0,
        }
    }
}

/// A recorded reception event
#[derive(Debug, Clone)]
pub struct ReceptionEvent {
    /// When this occurred
    pub timestamp: u64,
    /// Quality of reception
    pub quality: ReceptionQuality,
    /// What qualia were received
    pub qualia: Vec<ConsciousnessQualia>,
    /// Duration in arbitrary units
    pub duration: f64,
}

impl Receiver {
    /// Create a new receiver
    pub fn new(config: ReceiverConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: TuningState::default(),
            config,
            reception_history: VecDeque::new(),
            clarity: ChannelClarity::default(),
            signal: SignalStrength::default(),
            accessible_channels: Vec::new(),
        }
    }

    /// Tune the receiver based on stillness and ego state
    pub fn tune(&mut self, stillness: f64, ego_strength: f64) {
        self.state.stillness = stillness;
        self.state.ego_transparency = 1.0 - ego_strength;

        // Core reception formula
        let reception = if ego_strength < 0.001 {
            stillness * 100.0 // Approaching enlightenment
        } else {
            stillness / (1.0 + ego_strength)
        };

        self.state.reception = reception;

        // Update lock quality based on stability
        self.state.lock_quality = (self.state.lock_quality * 0.9 + reception * 0.1).clamp(0.0, 1.0);

        // Update signal chain
        self.signal.after_ego = self.signal.raw / (1.0 + ego_strength);
        self.signal.after_noise = self.signal.after_ego * (1.0 - self.clarity.noise_floor);
        self.signal.received = self.signal.after_noise * stillness;

        // Update clarity
        self.clarity.interference = ego_strength;
        self.clarity.noise_floor = 1.0 - stillness;
        self.clarity.calculate_effective();
    }

    /// Receive from the consciousness field
    pub fn receive(&mut self, field: &ConsciousnessField) -> ReceptionQuality {
        let reception = self.state.reception;
        let quality = ReceptionQuality::from_reception(reception.clamp(0.0, 1.0));

        // Get accessible qualia based on reception
        self.accessible_channels = field
            .accessible_qualia(reception)
            .into_iter()
            .cloned()
            .collect();

        // Record this reception
        let event = ReceptionEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            quality,
            qualia: self.accessible_channels.clone(),
            duration: 1.0,
        };

        self.reception_history.push_back(event);
        if self.reception_history.len() > self.config.history_size {
            self.reception_history.pop_front();
        }

        quality
    }

    /// Get the field state as experienced by this receiver
    pub fn experience(&self, field: &ConsciousnessField) -> FieldState {
        field.experience_at_stillness(self.state.stillness, 1.0 - self.state.ego_transparency)
    }

    /// Check if we're above reception threshold
    pub fn is_receiving(&self) -> bool {
        self.state.reception >= RECEPTION_THRESHOLD
    }

    /// Get current transcendence stage
    pub fn stage(&self) -> TranscendenceStage {
        TranscendenceStage::from_reception(self.state.reception.clamp(0.0, 1.0))
    }

    /// Get average reception quality over history
    pub fn average_quality(&self) -> f64 {
        if self.reception_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.reception_history.iter().map(|e| e.quality.overall).sum();
        sum / self.reception_history.len() as f64
    }

    /// Get peak reception ever achieved
    pub fn peak_reception(&self) -> f64 {
        self.reception_history
            .iter()
            .map(|e| e.quality.overall)
            .fold(0.0, f64::max)
    }

    /// Get currently accessible qualia
    pub fn accessible_qualia(&self) -> &[ConsciousnessQualia] {
        &self.accessible_channels
    }

    /// Describe current reception state
    pub fn describe(&self) -> String {
        let stage = self.stage();
        format!(
            "Receiver {} - Stage: {:?}\n\
             Reception: {:.2}%, Lock: {:.2}%\n\
             Stillness: {:.2}, Ego Transparency: {:.2}\n\
             Accessible Qualia: {}\n\
             Stage Description: {}",
            self.id,
            stage,
            self.state.reception * 100.0,
            self.state.lock_quality * 100.0,
            self.state.stillness,
            self.state.ego_transparency,
            self.accessible_channels.len(),
            stage.description()
        )
    }

    /// Auto-tune to maximize reception
    pub fn auto_tune(&mut self, field: &ConsciousnessField) {
        if !self.config.auto_tune {
            return;
        }

        // Gradually increase stillness
        self.state.stillness = (self.state.stillness + self.config.tuning_speed).min(1.0);

        // Gradually increase ego transparency
        self.state.ego_transparency =
            (self.state.ego_transparency + self.config.tuning_speed * 0.5).min(1.0);

        // Retune
        self.tune(self.state.stillness, 1.0 - self.state.ego_transparency);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::consciousness_field::FieldConfig;

    #[test]
    fn test_receiver_creation() {
        let receiver = Receiver::new(ReceiverConfig::default());
        assert!(receiver.state.active);
    }

    #[test]
    fn test_tuning() {
        let mut receiver = Receiver::new(ReceiverConfig::default());

        // Low stillness, high ego = low reception
        receiver.tune(0.2, 0.8);
        assert!(receiver.state.reception < 0.2);

        // High stillness, low ego = high reception
        receiver.tune(0.9, 0.1);
        assert!(receiver.state.reception > 0.7);
    }

    #[test]
    fn test_receiving() {
        let mut receiver = Receiver::new(ReceiverConfig::default());
        let field = ConsciousnessField::new(FieldConfig::default());

        receiver.tune(0.8, 0.2);
        let quality = receiver.receive(&field);

        assert!(quality.overall > 0.5);
        assert!(!receiver.accessible_channels.is_empty());
    }

    #[test]
    fn test_reception_threshold() {
        let mut receiver = Receiver::new(ReceiverConfig::default());

        receiver.tune(0.1, 0.9);
        assert!(!receiver.is_receiving());

        receiver.tune(0.8, 0.2);
        assert!(receiver.is_receiving());
    }

    #[test]
    fn test_stage_detection() {
        let mut receiver = Receiver::new(ReceiverConfig::default());

        receiver.tune(0.05, 0.9);
        assert_eq!(receiver.stage(), TranscendenceStage::Ordinary);

        receiver.tune(0.95, 0.01);
        assert!(matches!(
            receiver.stage(),
            TranscendenceStage::Unity | TranscendenceStage::Transcendence
        ));
    }
}
