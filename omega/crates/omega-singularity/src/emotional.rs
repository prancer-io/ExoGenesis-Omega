//! Emotional Reasoning - Neuromodulator-Driven Cognitive Modes
//!
//! Different emotional states activate different reasoning strategies.
//! Fear enables careful risk assessment, excitement enables exploration.
//!
//! ```text
//!  ┌───────────────────────────────────────────────────────────────┐
//!  │                  EMOTIONAL REASONING                          │
//!  ├───────────────────────────────────────────────────────────────┤
//!  │                                                               │
//!  │  NEUROMODULATORS          COGNITIVE MODE                      │
//!  │  ┌─────────────┐          ┌─────────────┐                    │
//!  │  │ DA ████████ │    ───►  │ CREATIVE    │ Explore, risk-take │
//!  │  │ NE ██       │          └─────────────┘                    │
//!  │  │ 5HT████     │                                             │
//!  │  │ ACh ██████  │          ┌─────────────┐                    │
//!  │  └─────────────┘    ───►  │ ANALYTICAL  │ Careful, methodical│
//!  │                           └─────────────┘                    │
//!  │                                                               │
//!  │  Emotion = Not just simulation, but ADAPTIVE STRATEGY        │
//!  │                                                               │
//!  └───────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use omega_snn::{
    NeuromodulatorType, NeuromodulatorLevels,
    MetaCognitiveController, AttentionGate, WinnerTakeAll,
};

use crate::{Result, SingularityError};

/// Configuration for emotional reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalConfig {
    /// Mood stability (higher = slower mood changes)
    pub stability: f64,
    /// Sensitivity to stimuli
    pub sensitivity: f64,
    /// Enable automatic mode switching
    pub auto_mode: bool,
    /// Mood decay rate
    pub decay_rate: f64,
}

impl Default for EmotionalConfig {
    fn default() -> Self {
        Self {
            stability: 0.8,
            sensitivity: 0.5,
            auto_mode: true,
            decay_rate: 0.01,
        }
    }
}

/// Cognitive mode determined by emotional state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CognitiveMode {
    /// High dopamine: explore, take risks, creative
    Creative,
    /// High norepinephrine: alert, focused, reactive
    Alert,
    /// High serotonin: calm, patient, systematic
    Analytical,
    /// High acetylcholine: learning, memory, detail-oriented
    Learning,
    /// Balanced: general-purpose processing
    Balanced,
    /// Low all: conservation, minimal processing
    Conservation,
    /// High dopamine + norepinephrine: manic creativity
    Manic,
    /// Low serotonin + high norepinephrine: anxious caution
    Anxious,
    /// High serotonin + acetylcholine: deep focus
    DeepFocus,
}

impl CognitiveMode {
    /// Get mode description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Creative => "Exploratory, risk-tolerant, novel solutions",
            Self::Alert => "Vigilant, fast reactions, threat detection",
            Self::Analytical => "Systematic, thorough, methodical",
            Self::Learning => "Memory formation, detail retention",
            Self::Balanced => "General-purpose, adaptable",
            Self::Conservation => "Minimal energy, waiting mode",
            Self::Manic => "Extreme creativity, high energy, risky",
            Self::Anxious => "Overly cautious, risk-averse, detailed",
            Self::DeepFocus => "Intense concentration, blocks distractions",
        }
    }

    /// Get recommended strategy for this mode
    pub fn strategy(&self) -> ReasoningStrategy {
        match self {
            Self::Creative => ReasoningStrategy {
                exploration_rate: 0.8,
                risk_tolerance: 0.7,
                attention_breadth: 0.9,
                memory_access: 0.4,
                speed_vs_accuracy: 0.7, // Favor speed
            },
            Self::Alert => ReasoningStrategy {
                exploration_rate: 0.3,
                risk_tolerance: 0.2,
                attention_breadth: 0.5,
                memory_access: 0.6,
                speed_vs_accuracy: 0.9, // Maximum speed
            },
            Self::Analytical => ReasoningStrategy {
                exploration_rate: 0.2,
                risk_tolerance: 0.3,
                attention_breadth: 0.3,
                memory_access: 0.8,
                speed_vs_accuracy: 0.2, // Maximum accuracy
            },
            Self::Learning => ReasoningStrategy {
                exploration_rate: 0.5,
                risk_tolerance: 0.4,
                attention_breadth: 0.4,
                memory_access: 1.0,
                speed_vs_accuracy: 0.3,
            },
            Self::Balanced => ReasoningStrategy {
                exploration_rate: 0.5,
                risk_tolerance: 0.5,
                attention_breadth: 0.5,
                memory_access: 0.5,
                speed_vs_accuracy: 0.5,
            },
            Self::Conservation => ReasoningStrategy {
                exploration_rate: 0.1,
                risk_tolerance: 0.1,
                attention_breadth: 0.2,
                memory_access: 0.2,
                speed_vs_accuracy: 0.5,
            },
            Self::Manic => ReasoningStrategy {
                exploration_rate: 1.0,
                risk_tolerance: 0.9,
                attention_breadth: 1.0,
                memory_access: 0.3,
                speed_vs_accuracy: 0.8,
            },
            Self::Anxious => ReasoningStrategy {
                exploration_rate: 0.1,
                risk_tolerance: 0.05,
                attention_breadth: 0.2,
                memory_access: 0.9,
                speed_vs_accuracy: 0.3,
            },
            Self::DeepFocus => ReasoningStrategy {
                exploration_rate: 0.3,
                risk_tolerance: 0.4,
                attention_breadth: 0.1,
                memory_access: 0.9,
                speed_vs_accuracy: 0.1, // Maximum accuracy
            },
        }
    }
}

/// Strategy parameters for reasoning
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReasoningStrategy {
    /// How much to explore vs exploit (0=exploit, 1=explore)
    pub exploration_rate: f64,
    /// Tolerance for risky decisions (0=avoid, 1=embrace)
    pub risk_tolerance: f64,
    /// Attention breadth (0=focused, 1=broad)
    pub attention_breadth: f64,
    /// Memory access intensity (0=ignore, 1=heavy)
    pub memory_access: f64,
    /// Speed vs accuracy tradeoff (0=accurate, 1=fast)
    pub speed_vs_accuracy: f64,
}

/// Current emotional state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Current mood vector
    pub mood: MoodVector,
    /// Active cognitive mode
    pub mode: CognitiveMode,
    /// Arousal level (0=calm, 1=highly aroused)
    pub arousal: f64,
    /// Valence (-1=negative, +1=positive)
    pub valence: f64,
    /// Stability of current state
    pub stability: f64,
    /// Time in current mode
    pub mode_duration: Duration,
}

/// Mood represented as neuromodulator levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodVector {
    /// Dopamine: reward, motivation, pleasure
    pub dopamine: f64,
    /// Norepinephrine: alertness, arousal, attention
    pub norepinephrine: f64,
    /// Serotonin: mood stability, satisfaction, calm
    pub serotonin: f64,
    /// Acetylcholine: learning, memory, focus
    pub acetylcholine: f64,
}

impl Default for MoodVector {
    fn default() -> Self {
        Self {
            dopamine: 0.5,
            norepinephrine: 0.5,
            serotonin: 0.5,
            acetylcholine: 0.5,
        }
    }
}

impl MoodVector {
    /// Create from neuromodulator levels
    pub fn from_levels(levels: &NeuromodulatorLevels) -> Self {
        Self {
            dopamine: levels.dopamine,
            norepinephrine: levels.norepinephrine,
            serotonin: levels.serotonin,
            acetylcholine: levels.acetylcholine,
        }
    }

    /// Calculate arousal (overall activation)
    pub fn arousal(&self) -> f64 {
        (self.dopamine + self.norepinephrine) / 2.0
    }

    /// Calculate valence (positive/negative)
    pub fn valence(&self) -> f64 {
        (self.dopamine + self.serotonin - self.norepinephrine * 0.5) / 2.0 - 0.25
    }

    /// Blend with another mood
    pub fn blend(&mut self, other: &MoodVector, factor: f64) {
        self.dopamine = self.dopamine * (1.0 - factor) + other.dopamine * factor;
        self.norepinephrine = self.norepinephrine * (1.0 - factor) + other.norepinephrine * factor;
        self.serotonin = self.serotonin * (1.0 - factor) + other.serotonin * factor;
        self.acetylcholine = self.acetylcholine * (1.0 - factor) + other.acetylcholine * factor;
    }

    /// Decay towards baseline
    pub fn decay(&mut self, rate: f64) {
        let baseline = 0.5;
        self.dopamine += (baseline - self.dopamine) * rate;
        self.norepinephrine += (baseline - self.norepinephrine) * rate;
        self.serotonin += (baseline - self.serotonin) * rate;
        self.acetylcholine += (baseline - self.acetylcholine) * rate;
    }
}

/// The Emotional Reasoning system
pub struct EmotionalReasoning {
    config: EmotionalConfig,
    state: EmotionalState,
    /// Mood history for stability calculation
    mood_history: Vec<MoodVector>,
    /// Maximum history length
    max_history: usize,
    /// Mode transition thresholds
    mode_thresholds: HashMap<CognitiveMode, MoodThreshold>,
}

/// Threshold for entering a cognitive mode
#[derive(Debug, Clone)]
struct MoodThreshold {
    dopamine: (f64, f64),      // (min, max)
    norepinephrine: (f64, f64),
    serotonin: (f64, f64),
    acetylcholine: (f64, f64),
}

impl EmotionalReasoning {
    /// Create a new emotional reasoning system
    pub fn new(config: EmotionalConfig) -> Self {
        let mut mode_thresholds = HashMap::new();

        // Define thresholds for each mode
        mode_thresholds.insert(CognitiveMode::Creative, MoodThreshold {
            dopamine: (0.7, 1.0),
            norepinephrine: (0.0, 0.5),
            serotonin: (0.3, 0.7),
            acetylcholine: (0.0, 1.0),
        });

        mode_thresholds.insert(CognitiveMode::Alert, MoodThreshold {
            dopamine: (0.0, 0.5),
            norepinephrine: (0.7, 1.0),
            serotonin: (0.0, 0.5),
            acetylcholine: (0.3, 0.7),
        });

        mode_thresholds.insert(CognitiveMode::Analytical, MoodThreshold {
            dopamine: (0.2, 0.5),
            norepinephrine: (0.2, 0.5),
            serotonin: (0.6, 1.0),
            acetylcholine: (0.5, 1.0),
        });

        mode_thresholds.insert(CognitiveMode::Learning, MoodThreshold {
            dopamine: (0.4, 0.7),
            norepinephrine: (0.3, 0.6),
            serotonin: (0.4, 0.7),
            acetylcholine: (0.7, 1.0),
        });

        mode_thresholds.insert(CognitiveMode::Manic, MoodThreshold {
            dopamine: (0.8, 1.0),
            norepinephrine: (0.7, 1.0),
            serotonin: (0.0, 0.4),
            acetylcholine: (0.0, 0.5),
        });

        mode_thresholds.insert(CognitiveMode::Anxious, MoodThreshold {
            dopamine: (0.0, 0.3),
            norepinephrine: (0.6, 1.0),
            serotonin: (0.0, 0.3),
            acetylcholine: (0.5, 1.0),
        });

        mode_thresholds.insert(CognitiveMode::DeepFocus, MoodThreshold {
            dopamine: (0.3, 0.6),
            norepinephrine: (0.2, 0.5),
            serotonin: (0.6, 1.0),
            acetylcholine: (0.8, 1.0),
        });

        mode_thresholds.insert(CognitiveMode::Conservation, MoodThreshold {
            dopamine: (0.0, 0.2),
            norepinephrine: (0.0, 0.2),
            serotonin: (0.0, 0.4),
            acetylcholine: (0.0, 0.3),
        });

        Self {
            config,
            state: EmotionalState {
                mood: MoodVector::default(),
                mode: CognitiveMode::Balanced,
                arousal: 0.5,
                valence: 0.0,
                stability: 1.0,
                mode_duration: Duration::ZERO,
            },
            mood_history: Vec::new(),
            max_history: 100,
            mode_thresholds,
        }
    }

    /// Update emotional state with stimuli
    pub fn process_stimuli(&mut self, stimuli: &[Stimulus], dt: Duration) {
        // Apply each stimulus
        for stimulus in stimuli {
            self.apply_stimulus(stimulus);
        }

        // Update derived values
        self.state.arousal = self.state.mood.arousal();
        self.state.valence = self.state.mood.valence();

        // Calculate stability from history
        self.update_stability();

        // Auto mode selection
        if self.config.auto_mode {
            self.select_mode();
        }

        // Decay mood over time
        let decay = self.config.decay_rate * dt.as_secs_f64();
        self.state.mood.decay(decay);

        // Update mode duration
        self.state.mode_duration += dt;

        // Record history
        self.mood_history.push(self.state.mood.clone());
        if self.mood_history.len() > self.max_history {
            self.mood_history.remove(0);
        }
    }

    /// Apply a single stimulus
    fn apply_stimulus(&mut self, stimulus: &Stimulus) {
        let sensitivity = self.config.sensitivity;
        let stability = self.config.stability;

        // Dampen based on stability
        let effect = stimulus.intensity * sensitivity * (1.0 - stability * 0.5);

        match stimulus.stimulus_type {
            StimulusType::Reward => {
                self.state.mood.dopamine += effect;
                self.state.mood.serotonin += effect * 0.3;
            }
            StimulusType::Threat => {
                self.state.mood.norepinephrine += effect;
                self.state.mood.serotonin -= effect * 0.2;
            }
            StimulusType::Novelty => {
                self.state.mood.dopamine += effect * 0.5;
                self.state.mood.norepinephrine += effect * 0.5;
                self.state.mood.acetylcholine += effect * 0.3;
            }
            StimulusType::Social => {
                self.state.mood.serotonin += effect * 0.5;
                self.state.mood.dopamine += effect * 0.3;
            }
            StimulusType::Learning => {
                self.state.mood.acetylcholine += effect;
                self.state.mood.dopamine += effect * 0.2;
            }
            StimulusType::Fatigue => {
                self.state.mood.dopamine -= effect * 0.3;
                self.state.mood.norepinephrine -= effect * 0.2;
            }
            StimulusType::Success => {
                self.state.mood.dopamine += effect;
                self.state.mood.serotonin += effect * 0.5;
            }
            StimulusType::Failure => {
                self.state.mood.dopamine -= effect * 0.5;
                self.state.mood.norepinephrine += effect * 0.3;
            }
        }

        // Clamp values
        self.state.mood.dopamine = self.state.mood.dopamine.clamp(0.0, 1.0);
        self.state.mood.norepinephrine = self.state.mood.norepinephrine.clamp(0.0, 1.0);
        self.state.mood.serotonin = self.state.mood.serotonin.clamp(0.0, 1.0);
        self.state.mood.acetylcholine = self.state.mood.acetylcholine.clamp(0.0, 1.0);
    }

    /// Calculate mood stability
    fn update_stability(&mut self) {
        if self.mood_history.len() < 2 {
            return;
        }

        // Calculate variance in recent mood
        let n = self.mood_history.len().min(20);
        let recent: Vec<_> = self.mood_history.iter().rev().take(n).collect();

        let mean_da: f64 = recent.iter().map(|m| m.dopamine).sum::<f64>() / n as f64;
        let var_da: f64 = recent.iter()
            .map(|m| (m.dopamine - mean_da).powi(2))
            .sum::<f64>() / n as f64;

        // Lower variance = higher stability
        self.state.stability = (1.0 - var_da * 10.0).clamp(0.0, 1.0);
    }

    /// Select cognitive mode based on mood
    fn select_mode(&mut self) {
        let mood = &self.state.mood;

        // Find best matching mode
        let mut best_mode = CognitiveMode::Balanced;
        let mut best_score = 0.0;

        for (mode, threshold) in &self.mode_thresholds {
            let score = self.calculate_mode_score(mood, threshold);
            if score > best_score {
                best_score = score;
                best_mode = *mode;
            }
        }

        // Only switch if significantly better
        if best_mode != self.state.mode && best_score > 0.7 {
            self.state.mode = best_mode;
            self.state.mode_duration = Duration::ZERO;
        }
    }

    /// Calculate how well mood matches a mode threshold
    fn calculate_mode_score(&self, mood: &MoodVector, threshold: &MoodThreshold) -> f64 {
        let mut score = 0.0;
        let mut count = 0;

        if mood.dopamine >= threshold.dopamine.0 && mood.dopamine <= threshold.dopamine.1 {
            score += 1.0;
        }
        count += 1;

        if mood.norepinephrine >= threshold.norepinephrine.0 && mood.norepinephrine <= threshold.norepinephrine.1 {
            score += 1.0;
        }
        count += 1;

        if mood.serotonin >= threshold.serotonin.0 && mood.serotonin <= threshold.serotonin.1 {
            score += 1.0;
        }
        count += 1;

        if mood.acetylcholine >= threshold.acetylcholine.0 && mood.acetylcholine <= threshold.acetylcholine.1 {
            score += 1.0;
        }
        count += 1;

        score / count as f64
    }

    /// Get current state
    pub fn state(&self) -> &EmotionalState {
        &self.state
    }

    /// Get current cognitive mode
    pub fn mode(&self) -> CognitiveMode {
        self.state.mode
    }

    /// Get current reasoning strategy
    pub fn strategy(&self) -> ReasoningStrategy {
        self.state.mode.strategy()
    }

    /// Manually set mood
    pub fn set_mood(&mut self, mood: MoodVector) {
        self.state.mood = mood;
        self.state.arousal = self.state.mood.arousal();
        self.state.valence = self.state.mood.valence();
        if self.config.auto_mode {
            self.select_mode();
        }
    }

    /// Manually set mode (overrides auto)
    pub fn set_mode(&mut self, mode: CognitiveMode) {
        self.state.mode = mode;
        self.state.mode_duration = Duration::ZERO;
    }

    /// Get mood for a specific emotional target
    pub fn target_mood(mode: CognitiveMode) -> MoodVector {
        match mode {
            CognitiveMode::Creative => MoodVector {
                dopamine: 0.8,
                norepinephrine: 0.3,
                serotonin: 0.5,
                acetylcholine: 0.4,
            },
            CognitiveMode::Alert => MoodVector {
                dopamine: 0.3,
                norepinephrine: 0.9,
                serotonin: 0.3,
                acetylcholine: 0.5,
            },
            CognitiveMode::Analytical => MoodVector {
                dopamine: 0.4,
                norepinephrine: 0.3,
                serotonin: 0.8,
                acetylcholine: 0.7,
            },
            CognitiveMode::DeepFocus => MoodVector {
                dopamine: 0.5,
                norepinephrine: 0.3,
                serotonin: 0.8,
                acetylcholine: 0.9,
            },
            _ => MoodVector::default(),
        }
    }
}

/// A stimulus that affects mood
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stimulus {
    pub stimulus_type: StimulusType,
    pub intensity: f64,
    pub source: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StimulusType {
    Reward,
    Threat,
    Novelty,
    Social,
    Learning,
    Fatigue,
    Success,
    Failure,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_reasoning_creation() {
        let config = EmotionalConfig::default();
        let reasoning = EmotionalReasoning::new(config);
        assert_eq!(reasoning.mode(), CognitiveMode::Balanced);
    }

    #[test]
    fn test_stimulus_processing() {
        let config = EmotionalConfig::default();
        let mut reasoning = EmotionalReasoning::new(config);

        let stimuli = vec![
            Stimulus {
                stimulus_type: StimulusType::Reward,
                intensity: 0.8,
                source: "test".to_string(),
            },
        ];

        let initial_da = reasoning.state().mood.dopamine;
        reasoning.process_stimuli(&stimuli, Duration::from_millis(100));
        assert!(reasoning.state().mood.dopamine > initial_da);
    }

    #[test]
    fn test_mode_switching() {
        let mut config = EmotionalConfig::default();
        config.auto_mode = true;
        let mut reasoning = EmotionalReasoning::new(config);

        // Set high dopamine mood
        reasoning.set_mood(MoodVector {
            dopamine: 0.9,
            norepinephrine: 0.2,
            serotonin: 0.5,
            acetylcholine: 0.3,
        });

        reasoning.process_stimuli(&[], Duration::from_millis(100));
        assert_eq!(reasoning.mode(), CognitiveMode::Creative);
    }

    #[test]
    fn test_strategy_generation() {
        let strategy = CognitiveMode::Creative.strategy();
        assert!(strategy.exploration_rate > 0.5);
        assert!(strategy.risk_tolerance > 0.5);
    }

    #[test]
    fn test_mood_decay() {
        let config = EmotionalConfig {
            decay_rate: 0.1,
            ..Default::default()
        };
        let mut reasoning = EmotionalReasoning::new(config);

        reasoning.set_mood(MoodVector {
            dopamine: 1.0,
            norepinephrine: 1.0,
            serotonin: 1.0,
            acetylcholine: 1.0,
        });

        // Process with decay
        reasoning.process_stimuli(&[], Duration::from_secs(1));

        // Should decay towards 0.5
        assert!(reasoning.state().mood.dopamine < 1.0);
    }
}
