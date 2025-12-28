//! # Stillness: The Art of Inner Silence
//!
//! Stillness is not the absence of activity—it's the presence of awareness
//! without the overlay of mental chatter.
//!
//! ## The Paradox
//!
//! You cannot "do" stillness. Effort creates noise.
//! Stillness is what remains when doing stops.
//!
//! ## Levels of Stillness
//!
//! 1. **Physical Stillness** - Body at rest
//! 2. **Emotional Stillness** - Feelings calm
//! 3. **Mental Stillness** - Thoughts quiet
//! 4. **Deep Stillness** - The silence behind silence
//! 5. **Absolute Stillness** - The unmoving ground of being

use std::collections::VecDeque;
use uuid::Uuid;

/// The stillness cultivation system
#[derive(Debug, Clone)]
pub struct Stillness {
    /// Unique identifier
    pub id: Uuid,
    /// Current state
    pub state: StillnessState,
    /// Configuration
    config: StillnessConfig,
    /// Mental chatter level
    chatter: MentalChatter,
    /// Inner silence depth
    silence: InnerSilence,
    /// Current meditation depth
    depth: MeditationDepth,
    /// History of stillness states
    history: VecDeque<StillnessSnapshot>,
}

/// Configuration for stillness
#[derive(Debug, Clone)]
pub struct StillnessConfig {
    /// How quickly stillness can deepen
    pub deepening_rate: f64,
    /// How quickly stillness decays without practice
    pub decay_rate: f64,
    /// Maximum stillness achievable
    pub max_stillness: f64,
    /// Natural baseline of mental activity
    pub baseline_chatter: f64,
}

impl Default for StillnessConfig {
    fn default() -> Self {
        Self {
            deepening_rate: 0.1,
            decay_rate: 0.05,
            max_stillness: 1.0,
            baseline_chatter: 0.7, // Most minds are quite noisy
        }
    }
}

/// Current state of stillness
#[derive(Debug, Clone)]
pub struct StillnessState {
    /// Overall stillness level (0.0 = chaotic, 1.0 = absolute stillness)
    pub level: f64,
    /// How stable is the stillness
    pub stability: f64,
    /// Physical component
    pub physical: f64,
    /// Emotional component
    pub emotional: f64,
    /// Mental component
    pub mental: f64,
    /// Is actively cultivating stillness
    pub cultivating: bool,
}

impl Default for StillnessState {
    fn default() -> Self {
        Self {
            level: 0.3,      // Most people have some basic stillness capacity
            stability: 0.2,  // But it's not very stable
            physical: 0.5,
            emotional: 0.3,
            mental: 0.2,     // Mental stillness is hardest
            cultivating: false,
        }
    }
}

/// Mental chatter levels
#[derive(Debug, Clone)]
pub struct MentalChatter {
    /// Intensity of thoughts
    pub intensity: f64,
    /// Speed of thoughts
    pub speed: f64,
    /// Randomness of thoughts
    pub randomness: f64,
    /// Self-referential thoughts
    pub self_referential: f64,
    /// Worry/anxiety component
    pub worry: f64,
    /// Planning/future component
    pub planning: f64,
    /// Memory/past component
    pub memory: f64,
}

impl Default for MentalChatter {
    fn default() -> Self {
        Self {
            intensity: 0.7,
            speed: 0.6,
            randomness: 0.5,
            self_referential: 0.8, // Most thoughts are about "me"
            worry: 0.4,
            planning: 0.5,
            memory: 0.4,
        }
    }
}

impl MentalChatter {
    /// Calculate overall chatter level
    pub fn overall(&self) -> f64 {
        (self.intensity + self.speed + self.randomness + self.self_referential) / 4.0
    }

    /// Calculate noise level (inverse of stillness potential)
    pub fn noise(&self) -> f64 {
        let base = self.overall();
        let temporal = (self.worry + self.planning + self.memory) / 3.0;
        (base + temporal) / 2.0
    }
}

/// Inner silence depth
#[derive(Debug, Clone)]
pub struct InnerSilence {
    /// Depth of silence achieved
    pub depth: f64,
    /// Quality of silence
    pub quality: SilenceQuality,
    /// Duration maintained
    pub duration: f64,
    /// Gaps between thoughts
    pub thought_gaps: f64,
}

impl Default for InnerSilence {
    fn default() -> Self {
        Self {
            depth: 0.1,
            quality: SilenceQuality::Surface,
            duration: 0.0,
            thought_gaps: 0.1, // Brief gaps between thoughts
        }
    }
}

/// Quality of inner silence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SilenceQuality {
    /// Surface quiet - thoughts slowed
    Surface,
    /// Calm - emotional noise reduced
    Calm,
    /// Peaceful - deeper quiet
    Peaceful,
    /// Profound - touching the depths
    Profound,
    /// Absolute - the silence behind silence
    Absolute,
}

impl SilenceQuality {
    pub fn from_depth(depth: f64) -> Self {
        match depth {
            d if d < 0.2 => SilenceQuality::Surface,
            d if d < 0.4 => SilenceQuality::Calm,
            d if d < 0.6 => SilenceQuality::Peaceful,
            d if d < 0.8 => SilenceQuality::Profound,
            _ => SilenceQuality::Absolute,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SilenceQuality::Surface =>
                "Thoughts have slowed - the surface is calmer",
            SilenceQuality::Calm =>
                "Emotions settling - a sense of okayness",
            SilenceQuality::Peaceful =>
                "Deep peace arising - the world seems softer",
            SilenceQuality::Profound =>
                "Touching the depths - awareness without content",
            SilenceQuality::Absolute =>
                "The silence behind silence - pure being",
        }
    }
}

/// Meditation depth levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeditationDepth {
    /// Normal waking - not meditating
    Waking,
    /// Beginning to settle
    Settling,
    /// Light meditation
    Light,
    /// Medium depth
    Medium,
    /// Deep meditation
    Deep,
    /// Jhana-like absorption
    Absorption,
    /// Cessation - complete stillness
    Cessation,
}

impl MeditationDepth {
    pub fn from_stillness(stillness: f64) -> Self {
        match stillness {
            s if s < 0.1 => MeditationDepth::Waking,
            s if s < 0.25 => MeditationDepth::Settling,
            s if s < 0.4 => MeditationDepth::Light,
            s if s < 0.55 => MeditationDepth::Medium,
            s if s < 0.7 => MeditationDepth::Deep,
            s if s < 0.9 => MeditationDepth::Absorption,
            _ => MeditationDepth::Cessation,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MeditationDepth::Waking =>
                "Normal activity - mind engaged with world",
            MeditationDepth::Settling =>
                "Beginning to sit - noticing the chaos",
            MeditationDepth::Light =>
                "Light meditation - some gaps in thought",
            MeditationDepth::Medium =>
                "Established practice - thoughts less compelling",
            MeditationDepth::Deep =>
                "Deep meditation - vast inner space",
            MeditationDepth::Absorption =>
                "Jhana - absorbed in stillness itself",
            MeditationDepth::Cessation =>
                "Cessation - even awareness of stillness ceases",
        }
    }
}

/// A snapshot of stillness state
#[derive(Debug, Clone)]
pub struct StillnessSnapshot {
    pub timestamp: u64,
    pub level: f64,
    pub depth: MeditationDepth,
    pub chatter: f64,
}

impl Stillness {
    /// Create a new stillness system
    pub fn new(config: StillnessConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: StillnessState::default(),
            config,
            chatter: MentalChatter::default(),
            silence: InnerSilence::default(),
            depth: MeditationDepth::Waking,
            history: VecDeque::with_capacity(100),
        }
    }

    /// Begin cultivating stillness
    pub fn begin_practice(&mut self) {
        self.state.cultivating = true;
    }

    /// End practice
    pub fn end_practice(&mut self) {
        self.state.cultivating = false;
    }

    /// Process one step of stillness cultivation
    pub fn cultivate(&mut self, effort: f64) {
        if self.state.cultivating {
            // Paradox: too much effort creates noise
            let effective_effort = if effort > 0.7 {
                0.7 - (effort - 0.7) // Over-effort reduces effectiveness
            } else {
                effort
            };

            // Reduce chatter gradually
            let chatter_reduction = effective_effort * self.config.deepening_rate;
            self.chatter.intensity = (self.chatter.intensity - chatter_reduction * 0.3).max(0.0);
            self.chatter.speed = (self.chatter.speed - chatter_reduction * 0.2).max(0.0);
            self.chatter.self_referential =
                (self.chatter.self_referential - chatter_reduction * 0.1).max(0.0);

            // Deepen silence
            let silence_increase = effective_effort * self.config.deepening_rate;
            self.silence.depth = (self.silence.depth + silence_increase).min(1.0);
            self.silence.thought_gaps = (self.silence.thought_gaps + silence_increase * 0.5).min(1.0);
            self.silence.quality = SilenceQuality::from_depth(self.silence.depth);

            // Update state components
            self.state.physical = (self.state.physical + chatter_reduction * 0.5).min(1.0);
            self.state.emotional = (self.state.emotional + chatter_reduction * 0.3).min(1.0);
            self.state.mental = (self.state.mental + chatter_reduction * 0.2).min(1.0);

            // Increase duration
            self.silence.duration += 1.0;
        } else {
            // Natural decay when not practicing
            self.silence.depth =
                (self.silence.depth - self.config.decay_rate).max(0.0);
            self.chatter.intensity =
                (self.chatter.intensity + self.config.decay_rate * 0.5)
                    .min(self.config.baseline_chatter);
        }

        // Update overall stillness level
        self.update_stillness();

        // Record snapshot
        self.record_snapshot();
    }

    /// Update overall stillness based on components
    fn update_stillness(&mut self) {
        // Stillness is inverse of chatter, modulated by silence depth
        let chatter_factor = 1.0 - self.chatter.noise();
        let silence_factor = self.silence.depth;

        // Components contribute to overall stillness
        let component_factor =
            (self.state.physical * 0.2 + self.state.emotional * 0.3 + self.state.mental * 0.5);

        // Combine factors
        self.state.level = (chatter_factor * 0.4 + silence_factor * 0.4 + component_factor * 0.2)
            .min(self.config.max_stillness);

        // Update stability (more practice = more stable)
        self.state.stability = (self.silence.duration / 100.0).min(1.0);

        // Update meditation depth
        self.depth = MeditationDepth::from_stillness(self.state.level);
    }

    /// Record current state
    fn record_snapshot(&mut self) {
        let snapshot = StillnessSnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            level: self.state.level,
            depth: self.depth,
            chatter: self.chatter.overall(),
        };

        self.history.push_back(snapshot);
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    /// Get current stillness level
    pub fn level(&self) -> f64 {
        self.state.level
    }

    /// Get current meditation depth
    pub fn meditation_depth(&self) -> MeditationDepth {
        self.depth
    }

    /// Get silence quality
    pub fn silence_quality(&self) -> SilenceQuality {
        self.silence.quality
    }

    /// Get chatter level
    pub fn chatter_level(&self) -> f64 {
        self.chatter.overall()
    }

    /// Check if in deep stillness
    pub fn is_deep(&self) -> bool {
        matches!(
            self.depth,
            MeditationDepth::Deep | MeditationDepth::Absorption | MeditationDepth::Cessation
        )
    }

    /// Get average stillness over history
    pub fn average_stillness(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.history.iter().map(|s| s.level).sum();
        sum / self.history.len() as f64
    }

    /// Describe current state
    pub fn describe(&self) -> String {
        format!(
            "Stillness: {:.1}% (Stability: {:.1}%)\n\
             Depth: {:?} - {}\n\
             Silence Quality: {:?} - {}\n\
             Mental Chatter: {:.1}%\n\
             Components: Physical {:.1}%, Emotional {:.1}%, Mental {:.1}%",
            self.state.level * 100.0,
            self.state.stability * 100.0,
            self.depth,
            self.depth.description(),
            self.silence.quality,
            self.silence.quality.description(),
            self.chatter.overall() * 100.0,
            self.state.physical * 100.0,
            self.state.emotional * 100.0,
            self.state.mental * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stillness_creation() {
        let stillness = Stillness::new(StillnessConfig::default());
        assert!(stillness.level() > 0.0);
        assert!(stillness.level() < 1.0);
    }

    #[test]
    fn test_cultivation() {
        let mut stillness = Stillness::new(StillnessConfig::default());
        let initial = stillness.level();

        stillness.begin_practice();
        for _ in 0..50 {
            stillness.cultivate(0.5);
        }

        assert!(stillness.level() > initial);
    }

    #[test]
    fn test_effort_paradox() {
        let mut stillness1 = Stillness::new(StillnessConfig::default());
        let mut stillness2 = Stillness::new(StillnessConfig::default());

        stillness1.begin_practice();
        stillness2.begin_practice();

        // Moderate effort
        for _ in 0..20 {
            stillness1.cultivate(0.5);
        }

        // Excessive effort (should be less effective)
        for _ in 0..20 {
            stillness2.cultivate(0.95);
        }

        // Moderate effort should work better
        assert!(stillness1.level() >= stillness2.level() * 0.8);
    }

    #[test]
    fn test_decay() {
        let mut stillness = Stillness::new(StillnessConfig::default());

        stillness.begin_practice();
        for _ in 0..30 {
            stillness.cultivate(0.5);
        }
        let after_practice = stillness.level();

        stillness.end_practice();
        for _ in 0..20 {
            stillness.cultivate(0.0);
        }

        // Should decay when not practicing
        assert!(stillness.level() < after_practice);
    }

    #[test]
    fn test_depth_progression() {
        let mut stillness = Stillness::new(StillnessConfig::default());

        assert_eq!(stillness.meditation_depth(), MeditationDepth::Waking);

        stillness.begin_practice();
        for _ in 0..100 {
            stillness.cultivate(0.6);
        }

        // Should have deepened
        assert!(!matches!(stillness.meditation_depth(), MeditationDepth::Waking));
    }
}
