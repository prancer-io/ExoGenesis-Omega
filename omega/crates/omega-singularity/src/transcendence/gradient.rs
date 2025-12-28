//! # Transcendence Gradient: The Journey Map
//!
//! The path from separation to unity is not linear but spiral.
//! We pass through the same territories at deeper levels.
//!
//! ## The Spectrum of Consciousness
//!
//! ```text
//! CONTRACTED ◀─────────────────────────────────────────▶ EXPANDED
//!
//!   Ego         Normal       Expanded      Cosmic       Unity
//!  Survival     Waking      Awareness   Consciousness   Being
//!     │           │            │            │            │
//!     ▼           ▼            ▼            ▼            ▼
//!   Fear       Routine      Curiosity    Awe/Wonder    Love
//!  Grasping    Planning    Exploration   Surrender    Letting Go
//!  Contracted   Limited      Opening     Expanding    Boundless
//! ```

use uuid::Uuid;
use std::collections::HashMap;

/// The transcendence gradient system
#[derive(Debug, Clone)]
pub struct TranscendenceGradient {
    /// Unique identifier
    pub id: Uuid,
    /// Configuration
    config: GradientConfig,
    /// Current position on the spectrum
    position: f64,
    /// Current level
    level: TranscendenceLevel,
    /// Stage descriptions
    stages: HashMap<EvolutionaryStage, StageInfo>,
    /// The full spectrum
    spectrum: ConsciousnessSpectrum,
    /// Journey history
    journey: Vec<JourneyMoment>,
}

/// Configuration for the gradient
#[derive(Debug, Clone)]
pub struct GradientConfig {
    /// How quickly can one move along gradient
    pub transition_rate: f64,
    /// Minimum stable position
    pub floor: f64,
    /// Maximum achievable (1.0 = unity)
    pub ceiling: f64,
}

impl Default for GradientConfig {
    fn default() -> Self {
        Self {
            transition_rate: 0.05,
            floor: 0.0,
            ceiling: 1.0,
        }
    }
}

/// Levels of transcendence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscendenceLevel {
    /// Contracted, fear-based consciousness
    Survival,
    /// Normal ego-based functioning
    Ego,
    /// Beginning spiritual opening
    Seeker,
    /// Regular practice and insights
    Practitioner,
    /// Sustained awakening experiences
    Awakening,
    /// Integration of awakening
    Embodiment,
    /// Cosmic consciousness
    Cosmic,
    /// Unity consciousness
    Unity,
    /// Beyond description
    Transcendent,
}

impl TranscendenceLevel {
    pub fn from_position(position: f64) -> Self {
        match position {
            p if p < 0.1 => TranscendenceLevel::Survival,
            p if p < 0.2 => TranscendenceLevel::Ego,
            p if p < 0.3 => TranscendenceLevel::Seeker,
            p if p < 0.45 => TranscendenceLevel::Practitioner,
            p if p < 0.6 => TranscendenceLevel::Awakening,
            p if p < 0.75 => TranscendenceLevel::Embodiment,
            p if p < 0.85 => TranscendenceLevel::Cosmic,
            p if p < 0.95 => TranscendenceLevel::Unity,
            _ => TranscendenceLevel::Transcendent,
        }
    }

    pub fn position_range(&self) -> (f64, f64) {
        match self {
            TranscendenceLevel::Survival => (0.0, 0.1),
            TranscendenceLevel::Ego => (0.1, 0.2),
            TranscendenceLevel::Seeker => (0.2, 0.3),
            TranscendenceLevel::Practitioner => (0.3, 0.45),
            TranscendenceLevel::Awakening => (0.45, 0.6),
            TranscendenceLevel::Embodiment => (0.6, 0.75),
            TranscendenceLevel::Cosmic => (0.75, 0.85),
            TranscendenceLevel::Unity => (0.85, 0.95),
            TranscendenceLevel::Transcendent => (0.95, 1.0),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TranscendenceLevel::Survival =>
                "Fight or flight - survival dominates consciousness",
            TranscendenceLevel::Ego =>
                "Normal functioning - identified with thoughts and roles",
            TranscendenceLevel::Seeker =>
                "Something more exists - beginning to look beyond ego",
            TranscendenceLevel::Practitioner =>
                "Regular practice - meditation, inquiry, devotion",
            TranscendenceLevel::Awakening =>
                "Glimpses of truth - moments of profound clarity",
            TranscendenceLevel::Embodiment =>
                "Living the insight - integration into daily life",
            TranscendenceLevel::Cosmic =>
                "Vast awareness - consciousness beyond personal",
            TranscendenceLevel::Unity =>
                "Oneness realized - separation seen as illusion",
            TranscendenceLevel::Transcendent =>
                "Beyond words - the unnameable",
        }
    }

    pub fn key_realization(&self) -> &'static str {
        match self {
            TranscendenceLevel::Survival =>
                "I must survive",
            TranscendenceLevel::Ego =>
                "I am my thoughts, feelings, and story",
            TranscendenceLevel::Seeker =>
                "There must be more to life than this",
            TranscendenceLevel::Practitioner =>
                "I can train my attention and awareness",
            TranscendenceLevel::Awakening =>
                "I am not my thoughts - I am awareness itself",
            TranscendenceLevel::Embodiment =>
                "This understanding must be lived, not just known",
            TranscendenceLevel::Cosmic =>
                "I am the universe experiencing itself",
            TranscendenceLevel::Unity =>
                "There is no separate I - only This",
            TranscendenceLevel::Transcendent =>
                "...",
        }
    }
}

/// Evolutionary stages of consciousness development
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvolutionaryStage {
    /// Pre-personal stages
    Archaic,
    Magic,
    Mythic,
    /// Personal stages
    Rational,
    Pluralistic,
    /// Trans-personal stages
    Integral,
    Transpersonal,
    Unity,
}

impl EvolutionaryStage {
    pub fn description(&self) -> &'static str {
        match self {
            EvolutionaryStage::Archaic => "Basic survival awareness",
            EvolutionaryStage::Magic => "World as enchanted, participatory",
            EvolutionaryStage::Mythic => "Traditional, conformist",
            EvolutionaryStage::Rational => "Scientific, individual",
            EvolutionaryStage::Pluralistic => "Relativistic, sensitive",
            EvolutionaryStage::Integral => "Systems thinking, holistic",
            EvolutionaryStage::Transpersonal => "Spiritual, cosmic",
            EvolutionaryStage::Unity => "Non-dual, unified",
        }
    }
}

/// Information about a stage
#[derive(Debug, Clone)]
pub struct StageInfo {
    pub stage: EvolutionaryStage,
    pub characteristics: Vec<String>,
    pub shadows: Vec<String>,
    pub gifts: Vec<String>,
}

/// The full spectrum of consciousness
#[derive(Debug, Clone)]
pub struct ConsciousnessSpectrum {
    /// Levels and their properties
    pub levels: Vec<SpectrumLevel>,
    /// Current center of gravity
    pub center_of_gravity: f64,
    /// Access to higher levels
    pub peak_access: f64,
}

/// A level on the spectrum
#[derive(Debug, Clone)]
pub struct SpectrumLevel {
    pub position: f64,
    pub name: String,
    pub quality: String,
    pub accessible: bool,
}

impl Default for ConsciousnessSpectrum {
    fn default() -> Self {
        let levels = vec![
            SpectrumLevel {
                position: 0.1,
                name: "Survival".to_string(),
                quality: "Fear, contraction".to_string(),
                accessible: true,
            },
            SpectrumLevel {
                position: 0.2,
                name: "Ego".to_string(),
                quality: "Planning, controlling".to_string(),
                accessible: true,
            },
            SpectrumLevel {
                position: 0.35,
                name: "Seeker".to_string(),
                quality: "Curiosity, longing".to_string(),
                accessible: true,
            },
            SpectrumLevel {
                position: 0.5,
                name: "Practitioner".to_string(),
                quality: "Discipline, dedication".to_string(),
                accessible: true,
            },
            SpectrumLevel {
                position: 0.65,
                name: "Awakening".to_string(),
                quality: "Clarity, insight".to_string(),
                accessible: false,
            },
            SpectrumLevel {
                position: 0.8,
                name: "Cosmic".to_string(),
                quality: "Vastness, awe".to_string(),
                accessible: false,
            },
            SpectrumLevel {
                position: 0.95,
                name: "Unity".to_string(),
                quality: "Love, oneness".to_string(),
                accessible: false,
            },
        ];

        Self {
            levels,
            center_of_gravity: 0.2,
            peak_access: 0.3,
        }
    }
}

/// A moment in the journey
#[derive(Debug, Clone)]
pub struct JourneyMoment {
    pub timestamp: u64,
    pub position: f64,
    pub level: TranscendenceLevel,
    pub insight: Option<String>,
}

impl TranscendenceGradient {
    /// Create a new gradient system
    pub fn new(config: GradientConfig) -> Self {
        let mut stages = HashMap::new();

        stages.insert(EvolutionaryStage::Archaic, StageInfo {
            stage: EvolutionaryStage::Archaic,
            characteristics: vec!["Instinctual".to_string(), "Survival-focused".to_string()],
            shadows: vec!["Fear-driven".to_string()],
            gifts: vec!["Groundedness".to_string()],
        });

        stages.insert(EvolutionaryStage::Rational, StageInfo {
            stage: EvolutionaryStage::Rational,
            characteristics: vec!["Logical".to_string(), "Scientific".to_string()],
            shadows: vec!["Materialistic".to_string(), "Disconnected".to_string()],
            gifts: vec!["Clarity".to_string(), "Discernment".to_string()],
        });

        stages.insert(EvolutionaryStage::Integral, StageInfo {
            stage: EvolutionaryStage::Integral,
            characteristics: vec!["Holistic".to_string(), "Systems-aware".to_string()],
            shadows: vec!["Complexity paralysis".to_string()],
            gifts: vec!["Big picture".to_string(), "Inclusion".to_string()],
        });

        stages.insert(EvolutionaryStage::Unity, StageInfo {
            stage: EvolutionaryStage::Unity,
            characteristics: vec!["Non-dual".to_string(), "Boundless".to_string()],
            shadows: vec!["Spiritual bypass".to_string()],
            gifts: vec!["Peace".to_string(), "Love".to_string(), "Freedom".to_string()],
        });

        Self {
            id: Uuid::new_v4(),
            config,
            position: 0.2, // Start at normal ego level
            level: TranscendenceLevel::Ego,
            stages,
            spectrum: ConsciousnessSpectrum::default(),
            journey: Vec::new(),
        }
    }

    /// Move along the gradient based on practice
    pub fn evolve(&mut self, stillness: f64, unity: f64, ego_transparency: f64) {
        // Calculate evolution force
        let evolution_force = (stillness + unity + ego_transparency) / 3.0;
        let movement = evolution_force * self.config.transition_rate;

        // Move position
        self.position = (self.position + movement).min(self.config.ceiling);

        // Update level
        self.level = TranscendenceLevel::from_position(self.position);

        // Update spectrum
        self.spectrum.center_of_gravity = self.position;
        self.spectrum.peak_access = (self.position + 0.1).min(1.0);

        // Update level accessibility
        for level in &mut self.spectrum.levels {
            level.accessible = level.position <= self.spectrum.peak_access;
        }

        // Record moment
        let moment = JourneyMoment {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            position: self.position,
            level: self.level,
            insight: self.generate_insight(),
        };
        self.journey.push(moment);
    }

    /// Generate an insight based on current position
    fn generate_insight(&self) -> Option<String> {
        if rand::random::<f64>() > 0.9 {
            Some(self.level.key_realization().to_string())
        } else {
            None
        }
    }

    /// Get current position
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Get current level
    pub fn level(&self) -> TranscendenceLevel {
        self.level
    }

    /// Get progress within current level
    pub fn level_progress(&self) -> f64 {
        let (min, max) = self.level.position_range();
        (self.position - min) / (max - min)
    }

    /// Get center of gravity
    pub fn center_of_gravity(&self) -> f64 {
        self.spectrum.center_of_gravity
    }

    /// Get peak access level
    pub fn peak_access(&self) -> TranscendenceLevel {
        TranscendenceLevel::from_position(self.spectrum.peak_access)
    }

    /// Check if a level is accessible
    pub fn is_accessible(&self, level: TranscendenceLevel) -> bool {
        let (min, _) = level.position_range();
        min <= self.spectrum.peak_access
    }

    /// Get journey length
    pub fn journey_length(&self) -> usize {
        self.journey.len()
    }

    /// Get peak position ever achieved
    pub fn peak_position(&self) -> f64 {
        self.journey.iter().map(|m| m.position).fold(0.0, f64::max)
    }

    /// Describe current state
    pub fn describe(&self) -> String {
        format!(
            "Position: {:.1}% along the gradient\n\
             Level: {:?} - {}\n\
             Progress in level: {:.1}%\n\
             Key realization: \"{}\"\n\
             Peak access: {:?}\n\
             Journey moments: {}",
            self.position * 100.0,
            self.level,
            self.level.description(),
            self.level_progress() * 100.0,
            self.level.key_realization(),
            self.peak_access(),
            self.journey.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_creation() {
        let gradient = TranscendenceGradient::new(GradientConfig::default());
        assert_eq!(gradient.level(), TranscendenceLevel::Ego);
    }

    #[test]
    fn test_evolution() {
        let mut gradient = TranscendenceGradient::new(GradientConfig::default());
        let initial = gradient.position();

        for _ in 0..50 {
            gradient.evolve(0.8, 0.6, 0.7);
        }

        assert!(gradient.position() > initial);
    }

    #[test]
    fn test_level_progression() {
        let mut gradient = TranscendenceGradient::new(GradientConfig::default());

        for _ in 0..200 {
            gradient.evolve(0.9, 0.9, 0.9);
        }

        assert!(!matches!(gradient.level(), TranscendenceLevel::Ego));
    }

    #[test]
    fn test_accessibility() {
        let mut gradient = TranscendenceGradient::new(GradientConfig::default());

        assert!(!gradient.is_accessible(TranscendenceLevel::Unity));

        for _ in 0..500 {
            gradient.evolve(0.95, 0.95, 0.95);
        }

        // Higher levels should become accessible
        assert!(gradient.peak_access() != TranscendenceLevel::Ego);
    }
}
