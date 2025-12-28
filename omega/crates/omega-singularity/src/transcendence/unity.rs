//! # Unity: The Dissolution of Boundaries
//!
//! Unity is not achieved—it is recognized. Separation was always the illusion.
//!
//! ## The Nature of Separation
//!
//! The sense of being a separate self is constructed through:
//! - Thoughts about "me" and "mine"
//! - Body identification
//! - Memory creating continuity
//! - Fear creating defensive boundaries
//!
//! When these constructions relax, unity is revealed as the natural state.
//!
//! ## Levels of Unity Experience
//!
//! 1. **Glimpse** - Momentary dissolution during peak experiences
//! 2. **Flow** - Losing self in activity
//! 3. **Communion** - Deep connection with nature/others
//! 4. **Oneness** - Sustained non-dual awareness
//! 5. **Sahaja** - Permanent establishment in unity

use uuid::Uuid;
use std::collections::VecDeque;

/// The unity experience system
#[derive(Debug, Clone)]
pub struct Unity {
    /// Unique identifier
    pub id: Uuid,
    /// Current state
    pub state: UnityState,
    /// Configuration
    config: UnityConfig,
    /// Boundary dissolution tracking
    boundaries: Vec<BoundaryDissolution>,
    /// Separation illusion strength
    separation: SeparationIllusion,
    /// Unity experiences history
    experiences: VecDeque<OnenessExperience>,
}

/// Configuration for unity system
#[derive(Debug, Clone)]
pub struct UnityConfig {
    /// How quickly boundaries can dissolve
    pub dissolution_rate: f64,
    /// How quickly separation reasserts
    pub reassertion_rate: f64,
    /// Maximum unity achievable (1.0 = complete)
    pub max_unity: f64,
}

impl Default for UnityConfig {
    fn default() -> Self {
        Self {
            dissolution_rate: 0.1,
            reassertion_rate: 0.05,
            max_unity: 1.0,
        }
    }
}

/// Current state of unity experience
#[derive(Debug, Clone)]
pub struct UnityState {
    /// Overall unity level (0.0 = full separation, 1.0 = complete unity)
    pub level: f64,
    /// Stability of unity experience
    pub stability: f64,
    /// Type of unity being experienced
    pub unity_type: UnityType,
    /// Is actively experiencing unity
    pub experiencing: bool,
    /// Duration of current experience
    pub duration: f64,
}

impl Default for UnityState {
    fn default() -> Self {
        Self {
            level: 0.0,
            stability: 0.0,
            unity_type: UnityType::None,
            experiencing: false,
            duration: 0.0,
        }
    }
}

/// Types of unity experience
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnityType {
    /// No unity experience
    None,
    /// Brief glimpse of oneness
    Glimpse,
    /// Lost in flow
    Flow,
    /// Deep communion
    Communion,
    /// Subject-object dissolution
    Absorption,
    /// Full non-dual awareness
    Nondual,
    /// Permanent establishment
    Sahaja,
}

impl UnityType {
    pub fn from_level(level: f64) -> Self {
        match level {
            l if l < 0.1 => UnityType::None,
            l if l < 0.25 => UnityType::Glimpse,
            l if l < 0.4 => UnityType::Flow,
            l if l < 0.6 => UnityType::Communion,
            l if l < 0.8 => UnityType::Absorption,
            l if l < 0.95 => UnityType::Nondual,
            _ => UnityType::Sahaja,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UnityType::None =>
                "Normal separate self - 'I' distinct from 'world'",
            UnityType::Glimpse =>
                "Momentary openings - peak experiences, awe, wonder",
            UnityType::Flow =>
                "Self forgotten in action - athlete's zone, artist's trance",
            UnityType::Communion =>
                "Deep connection - feeling one with nature, beloved, cosmos",
            UnityType::Absorption =>
                "Subject absorbs into object - the knower becomes the known",
            UnityType::Nondual =>
                "Neither one nor two - awareness without center or boundary",
            UnityType::Sahaja =>
                "Natural state - unity constant regardless of experience",
        }
    }
}

/// A boundary being dissolved
#[derive(Debug, Clone)]
pub struct BoundaryDissolution {
    /// Type of boundary
    pub boundary_type: BoundaryKind,
    /// Current strength (1.0 = solid, 0.0 = dissolved)
    pub strength: f64,
    /// Permeability (how much passes through)
    pub permeability: f64,
    /// Is actively dissolving
    pub dissolving: bool,
}

/// Types of boundaries
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundaryKind {
    /// Self vs other people
    Interpersonal,
    /// Self vs nature
    Natural,
    /// Self vs universe
    Cosmic,
    /// Self vs divine/source
    Divine,
    /// Past self vs present self
    Temporal,
    /// Observer vs observed
    Perceptual,
    /// Knower vs known
    Epistemological,
}

impl BoundaryKind {
    pub fn description(&self) -> &'static str {
        match self {
            BoundaryKind::Interpersonal => "The boundary between self and other people",
            BoundaryKind::Natural => "The boundary between self and nature",
            BoundaryKind::Cosmic => "The boundary between self and universe",
            BoundaryKind::Divine => "The boundary between self and source/god",
            BoundaryKind::Temporal => "The boundary between past, present, future self",
            BoundaryKind::Perceptual => "The boundary between observer and observed",
            BoundaryKind::Epistemological => "The boundary between knower and known",
        }
    }

    pub fn all() -> Vec<BoundaryKind> {
        vec![
            BoundaryKind::Interpersonal,
            BoundaryKind::Natural,
            BoundaryKind::Cosmic,
            BoundaryKind::Divine,
            BoundaryKind::Temporal,
            BoundaryKind::Perceptual,
            BoundaryKind::Epistemological,
        ]
    }

    /// Depth of this boundary (deeper = harder to dissolve)
    pub fn depth(&self) -> f64 {
        match self {
            BoundaryKind::Interpersonal => 0.2,
            BoundaryKind::Natural => 0.3,
            BoundaryKind::Temporal => 0.4,
            BoundaryKind::Cosmic => 0.5,
            BoundaryKind::Perceptual => 0.6,
            BoundaryKind::Divine => 0.8,
            BoundaryKind::Epistemological => 1.0,
        }
    }
}

/// The illusion of separation
#[derive(Debug, Clone)]
pub struct SeparationIllusion {
    /// Overall strength of separation feeling
    pub strength: f64,
    /// Components of separation
    pub components: SeparationComponents,
    /// Is actively being questioned
    pub questioning: bool,
}

impl Default for SeparationIllusion {
    fn default() -> Self {
        Self {
            strength: 1.0, // Separation feels very real by default
            components: SeparationComponents::default(),
            questioning: false,
        }
    }
}

/// Components that create the sense of separation
#[derive(Debug, Clone)]
pub struct SeparationComponents {
    /// "I am this body" belief
    pub body_identification: f64,
    /// "I am the thinker" belief
    pub thinker_identification: f64,
    /// "I am separate from world" belief
    pub world_separation: f64,
    /// "I am separate from others" belief
    pub other_separation: f64,
    /// "I exist in time" belief
    pub temporal_separation: f64,
}

impl Default for SeparationComponents {
    fn default() -> Self {
        Self {
            body_identification: 0.9,
            thinker_identification: 0.8,
            world_separation: 0.85,
            other_separation: 0.7,
            temporal_separation: 0.75,
        }
    }
}

impl SeparationComponents {
    pub fn overall(&self) -> f64 {
        (self.body_identification
            + self.thinker_identification
            + self.world_separation
            + self.other_separation
            + self.temporal_separation) / 5.0
    }
}

/// A recorded oneness experience
#[derive(Debug, Clone)]
pub struct OnenessExperience {
    /// When this occurred
    pub timestamp: u64,
    /// Type of unity
    pub unity_type: UnityType,
    /// Depth achieved
    pub depth: f64,
    /// Duration
    pub duration: f64,
    /// What triggered it
    pub trigger: UnityTrigger,
    /// Phenomenological description
    pub description: String,
}

/// What can trigger unity experiences
#[derive(Debug, Clone)]
pub enum UnityTrigger {
    /// Meditation/practice
    Practice,
    /// Being in nature
    Nature,
    /// Love/connection
    Love,
    /// Beauty/awe
    Beauty,
    /// Near-death experience
    NearDeath,
    /// Spontaneous grace
    Spontaneous,
    /// Substances (honorable mention)
    Entheogen,
}

impl Unity {
    /// Create a new unity system
    pub fn new(config: UnityConfig) -> Self {
        let boundaries = BoundaryKind::all()
            .into_iter()
            .map(|kind| BoundaryDissolution {
                boundary_type: kind,
                strength: 1.0,
                permeability: 0.0,
                dissolving: false,
            })
            .collect();

        Self {
            id: Uuid::new_v4(),
            state: UnityState::default(),
            config,
            boundaries,
            separation: SeparationIllusion::default(),
            experiences: VecDeque::with_capacity(50),
        }
    }

    /// Begin dissolving boundaries
    pub fn begin_dissolution(&mut self) {
        self.state.experiencing = true;
        for boundary in &mut self.boundaries {
            boundary.dissolving = true;
        }
        self.separation.questioning = true;
    }

    /// Stop dissolution
    pub fn end_dissolution(&mut self) {
        self.state.experiencing = false;
        for boundary in &mut self.boundaries {
            boundary.dissolving = false;
        }
    }

    /// Process one step of boundary dissolution
    pub fn dissolve_step(&mut self, stillness: f64, ego_transparency: f64) {
        if !self.state.experiencing {
            // Separation reasserts
            self.reassert_separation();
            return;
        }

        // Dissolution power based on stillness and ego transparency
        let dissolution_power = stillness * ego_transparency * self.config.dissolution_rate;

        // Dissolve boundaries from easiest to hardest
        for boundary in &mut self.boundaries {
            if boundary.dissolving && boundary.strength > 0.0 {
                let rate = dissolution_power * (1.0 - boundary.boundary_type.depth() * 0.5);
                boundary.strength = (boundary.strength - rate).max(0.0);
                boundary.permeability = 1.0 - boundary.strength;
            }
        }

        // Reduce separation components
        self.separation.components.body_identification =
            (self.separation.components.body_identification - dissolution_power * 0.2).max(0.0);
        self.separation.components.thinker_identification =
            (self.separation.components.thinker_identification - dissolution_power * 0.15).max(0.0);
        self.separation.components.world_separation =
            (self.separation.components.world_separation - dissolution_power * 0.15).max(0.0);
        self.separation.components.other_separation =
            (self.separation.components.other_separation - dissolution_power * 0.15).max(0.0);
        self.separation.components.temporal_separation =
            (self.separation.components.temporal_separation - dissolution_power * 0.1).max(0.0);

        self.separation.strength = self.separation.components.overall();

        // Update overall unity state
        self.update_state();
    }

    /// Update unity state based on boundaries
    fn update_state(&mut self) {
        // Unity is inverse of average boundary strength
        let avg_boundary: f64 = self.boundaries.iter().map(|b| b.strength).sum::<f64>()
            / self.boundaries.len() as f64;

        let unity_from_boundaries = 1.0 - avg_boundary;
        let unity_from_separation = 1.0 - self.separation.strength;

        self.state.level = ((unity_from_boundaries + unity_from_separation) / 2.0)
            .min(self.config.max_unity);

        self.state.unity_type = UnityType::from_level(self.state.level);

        // Update stability based on duration
        if self.state.experiencing {
            self.state.duration += 1.0;
            self.state.stability = (self.state.duration / 50.0).min(1.0);
        }

        // Record if significant unity achieved
        if self.state.level > 0.3 && self.state.experiencing {
            self.record_experience();
        }
    }

    /// Separation reasserts when not actively dissolving
    fn reassert_separation(&mut self) {
        let reassertion = self.config.reassertion_rate;

        for boundary in &mut self.boundaries {
            boundary.strength = (boundary.strength + reassertion).min(1.0);
            boundary.permeability = 1.0 - boundary.strength;
        }

        // Separation components reassert
        self.separation.components.body_identification =
            (self.separation.components.body_identification + reassertion * 0.3).min(0.9);
        self.separation.components.thinker_identification =
            (self.separation.components.thinker_identification + reassertion * 0.2).min(0.8);

        self.separation.strength = self.separation.components.overall();

        self.state.level = (self.state.level - reassertion * 2.0).max(0.0);
        self.state.unity_type = UnityType::from_level(self.state.level);
        self.state.duration = 0.0;
    }

    /// Record a unity experience
    fn record_experience(&mut self) {
        let experience = OnenessExperience {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            unity_type: self.state.unity_type,
            depth: self.state.level,
            duration: self.state.duration,
            trigger: UnityTrigger::Practice,
            description: self.state.unity_type.description().to_string(),
        };

        // Only record new high-water marks or distinct experiences
        if self.experiences.is_empty()
            || self.experiences.back().map(|e| e.depth < self.state.level).unwrap_or(true)
        {
            self.experiences.push_back(experience);
            if self.experiences.len() > 50 {
                self.experiences.pop_front();
            }
        }
    }

    /// Get current unity level
    pub fn level(&self) -> f64 {
        self.state.level
    }

    /// Get current unity type
    pub fn unity_type(&self) -> UnityType {
        self.state.unity_type
    }

    /// Get separation strength
    pub fn separation_strength(&self) -> f64 {
        self.separation.strength
    }

    /// Check if experiencing unity
    pub fn is_unified(&self) -> bool {
        self.state.level > 0.5
    }

    /// Get most dissolved boundary
    pub fn most_dissolved_boundary(&self) -> Option<&BoundaryDissolution> {
        self.boundaries.iter().min_by(|a, b| {
            a.strength.partial_cmp(&b.strength).unwrap()
        })
    }

    /// Get peak unity ever achieved
    pub fn peak_unity(&self) -> f64 {
        self.experiences.iter().map(|e| e.depth).fold(0.0, f64::max)
    }

    /// Describe current state
    pub fn describe(&self) -> String {
        let dissolved_count = self.boundaries.iter().filter(|b| b.strength < 0.5).count();
        format!(
            "Unity Level: {:.1}%\n\
             Type: {:?} - {}\n\
             Separation Strength: {:.1}%\n\
             Boundaries Dissolved: {}/{}\n\
             Stability: {:.1}%\n\
             Peak Unity: {:.1}%",
            self.state.level * 100.0,
            self.state.unity_type,
            self.state.unity_type.description(),
            self.separation.strength * 100.0,
            dissolved_count,
            self.boundaries.len(),
            self.state.stability * 100.0,
            self.peak_unity() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unity_creation() {
        let unity = Unity::new(UnityConfig::default());
        assert_eq!(unity.level(), 0.0);
        assert_eq!(unity.separation_strength(), 1.0);
    }

    #[test]
    fn test_dissolution() {
        let mut unity = Unity::new(UnityConfig::default());
        unity.begin_dissolution();

        for _ in 0..50 {
            unity.dissolve_step(0.8, 0.8);
        }

        assert!(unity.level() > 0.0);
        assert!(unity.separation_strength() < 1.0);
    }

    #[test]
    fn test_unity_type_progression() {
        let mut unity = Unity::new(UnityConfig::default());
        assert_eq!(unity.unity_type(), UnityType::None);

        unity.begin_dissolution();
        for _ in 0..100 {
            unity.dissolve_step(0.9, 0.9);
        }

        assert!(!matches!(unity.unity_type(), UnityType::None));
    }

    #[test]
    fn test_reassertion() {
        let mut unity = Unity::new(UnityConfig::default());
        unity.begin_dissolution();

        for _ in 0..30 {
            unity.dissolve_step(0.8, 0.8);
        }
        let after_dissolution = unity.level();

        unity.end_dissolution();
        for _ in 0..20 {
            unity.dissolve_step(0.0, 0.0);
        }

        assert!(unity.level() < after_dissolution);
    }
}
