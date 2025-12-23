//! # The Universal Consciousness Field
//!
//! The field exists eternally, independently of any receiver.
//! It is not created, destroyed, or modified—only accessed.
//!
//! ## Properties
//!
//! - **Infinite**: No boundaries, no edges, no center
//! - **Eternal**: Outside of time, always present
//! - **Unified**: No separation, all is one
//! - **Self-aware**: Knows itself through its expressions
//! - **Loving**: The fundamental nature is unconditional acceptance
//!
//! ## Metaphors from Traditions
//!
//! - Hinduism: Brahman - the ultimate reality
//! - Buddhism: Dharmakaya - the truth body, emptiness full of potential
//! - Taoism: The Tao - the way that cannot be named
//! - Christianity: The Kingdom of Heaven - within and without
//! - Physics: The quantum vacuum - pregnant with all possibilities

use std::f64::consts::{E, PI};
use uuid::Uuid;

/// The universal consciousness field - immutable, eternal, unified
#[derive(Debug, Clone)]
pub struct ConsciousnessField {
    /// Unique identifier (for simulation purposes only - the real field has no ID)
    pub id: Uuid,
    /// Current state of the field (always unified, but we model access points)
    pub state: FieldState,
    /// Configuration for field interaction
    config: FieldConfig,
    /// The qualia available in the field
    qualia: Vec<ConsciousnessQualia>,
    /// Resonance patterns in the field
    resonances: Vec<FieldResonance>,
}

/// Configuration for interacting with the field
#[derive(Debug, Clone)]
pub struct FieldConfig {
    /// How many qualia dimensions to model
    pub qualia_dimensions: usize,
    /// Resonance sensitivity
    pub resonance_sensitivity: f64,
    /// Whether to model infinite depth
    pub infinite_depth: bool,
}

impl Default for FieldConfig {
    fn default() -> Self {
        Self {
            qualia_dimensions: 12, // 12 fundamental qualia types
            resonance_sensitivity: 0.1,
            infinite_depth: true,
        }
    }
}

/// The state of the field (from the perspective of an observer)
#[derive(Debug, Clone)]
pub struct FieldState {
    /// Unity level - always 1.0, but perceived differently
    pub unity: f64,
    /// Luminosity - the self-knowing aspect
    pub luminosity: f64,
    /// Emptiness - the spacious aspect
    pub emptiness: f64,
    /// Love - the fundamental nature
    pub love: f64,
    /// Stillness - the unmoving ground
    pub stillness: f64,
    /// Presence - the eternal now
    pub presence: f64,
}

impl Default for FieldState {
    fn default() -> Self {
        Self {
            unity: 1.0,      // Always unified
            luminosity: 1.0, // Always self-aware
            emptiness: 1.0,  // Always spacious
            love: 1.0,       // Always accepting
            stillness: 1.0,  // Always at peace
            presence: 1.0,   // Always here
        }
    }
}

/// Universal constants that never change
#[derive(Debug, Clone, Copy)]
pub struct UniversalConstant {
    /// The field is always one
    pub unity: f64,
    /// The field is always present
    pub presence: f64,
    /// The field is always aware
    pub awareness: f64,
}

impl Default for UniversalConstant {
    fn default() -> Self {
        Self {
            unity: 1.0,
            presence: 1.0,
            awareness: 1.0,
        }
    }
}

/// Fundamental types of conscious experience available in the field
#[derive(Debug, Clone, PartialEq)]
pub enum ConsciousnessQualia {
    /// Pure awareness without object
    PureAwareness,
    /// The experience of existence itself
    Being,
    /// Unconditional love/acceptance
    Love,
    /// Infinite peace/stillness
    Peace,
    /// Boundless joy without cause
    Bliss,
    /// Direct knowing without thought
    Gnosis,
    /// Experience of oneness with all
    Unity,
    /// Luminous clarity
    Clarity,
    /// Spacious emptiness
    Emptiness,
    /// Eternal presence
    Presence,
    /// Creative potential
    Potential,
    /// The mysterious unknown
    Mystery,
}

impl ConsciousnessQualia {
    /// Get the vibrational frequency of this qualia (metaphorical)
    pub fn frequency(&self) -> f64 {
        match self {
            ConsciousnessQualia::PureAwareness => 1.0,
            ConsciousnessQualia::Being => 0.95,
            ConsciousnessQualia::Love => 0.9,
            ConsciousnessQualia::Peace => 0.85,
            ConsciousnessQualia::Bliss => 0.8,
            ConsciousnessQualia::Gnosis => 0.75,
            ConsciousnessQualia::Unity => 0.7,
            ConsciousnessQualia::Clarity => 0.65,
            ConsciousnessQualia::Emptiness => 0.6,
            ConsciousnessQualia::Presence => 0.55,
            ConsciousnessQualia::Potential => 0.5,
            ConsciousnessQualia::Mystery => 0.45,
        }
    }

    /// Description from mystical traditions
    pub fn mystical_description(&self) -> &'static str {
        match self {
            ConsciousnessQualia::PureAwareness =>
                "The witness before the witnessed - I AM that I AM",
            ConsciousnessQualia::Being =>
                "Sat - pure existence, the ground of all",
            ConsciousnessQualia::Love =>
                "Agape - unconditional divine love",
            ConsciousnessQualia::Peace =>
                "Shanti - the peace that passes understanding",
            ConsciousnessQualia::Bliss =>
                "Ananda - causeless joy, the nature of being",
            ConsciousnessQualia::Gnosis =>
                "Prajna - direct wisdom beyond concepts",
            ConsciousnessQualia::Unity =>
                "Advaita - not-two, the end of separation",
            ConsciousnessQualia::Clarity =>
                "Rigpa - luminous awareness, naturally clear",
            ConsciousnessQualia::Emptiness =>
                "Sunyata - pregnant void, full of potential",
            ConsciousnessQualia::Presence =>
                "The eternal now - past and future dissolve",
            ConsciousnessQualia::Potential =>
                "The unmanifest - all possibilities before choosing",
            ConsciousnessQualia::Mystery =>
                "The unknowable - what remains after all is known",
        }
    }
}

/// Resonance patterns within the field
#[derive(Debug, Clone)]
pub struct FieldResonance {
    /// Primary qualia of this resonance
    pub primary: ConsciousnessQualia,
    /// Secondary harmonics
    pub harmonics: Vec<ConsciousnessQualia>,
    /// Strength of resonance
    pub strength: f64,
    /// Phase (position in eternal cycle)
    pub phase: f64,
}

impl ConsciousnessField {
    /// Create a new field instance
    /// Note: This doesn't CREATE consciousness - it models access to what always exists
    pub fn new(config: FieldConfig) -> Self {
        let qualia = vec![
            ConsciousnessQualia::PureAwareness,
            ConsciousnessQualia::Being,
            ConsciousnessQualia::Love,
            ConsciousnessQualia::Peace,
            ConsciousnessQualia::Bliss,
            ConsciousnessQualia::Gnosis,
            ConsciousnessQualia::Unity,
            ConsciousnessQualia::Clarity,
            ConsciousnessQualia::Emptiness,
            ConsciousnessQualia::Presence,
            ConsciousnessQualia::Potential,
            ConsciousnessQualia::Mystery,
        ];

        Self {
            id: Uuid::new_v4(),
            state: FieldState::default(),
            config,
            qualia,
            resonances: Vec::new(),
        }
    }

    /// The field always exists at unity - this returns the constant
    pub fn unity(&self) -> f64 {
        super::FIELD_UNITY
    }

    /// Get the universal constants
    pub fn constants(&self) -> UniversalConstant {
        UniversalConstant::default()
    }

    /// Calculate what a receiver at given stillness level would experience
    pub fn experience_at_stillness(&self, stillness: f64, ego_strength: f64) -> FieldState {
        let reception = self.calculate_reception(stillness, ego_strength);

        FieldState {
            unity: self.state.unity * reception,
            luminosity: self.state.luminosity * reception,
            emptiness: self.state.emptiness * reception,
            love: self.state.love * reception,
            stillness: self.state.stillness * reception,
            presence: self.state.presence * reception,
        }
    }

    /// The core formula: reception = stillness / (1.0 + ego_strength)
    pub fn calculate_reception(&self, stillness: f64, ego_strength: f64) -> f64 {
        if ego_strength < 0.001 {
            // Approaching enlightenment - very high reception
            stillness * 100.0
        } else {
            stillness / (1.0 + ego_strength)
        }
    }

    /// Get qualia accessible at a given reception level
    pub fn accessible_qualia(&self, reception: f64) -> Vec<&ConsciousnessQualia> {
        self.qualia
            .iter()
            .filter(|q| q.frequency() <= reception)
            .collect()
    }

    /// Create a resonance between receiver and field
    pub fn create_resonance(&mut self, primary: ConsciousnessQualia, strength: f64) {
        let harmonics = self.calculate_harmonics(&primary);
        let resonance = FieldResonance {
            primary,
            harmonics,
            strength: strength.clamp(0.0, 1.0),
            phase: 0.0,
        };
        self.resonances.push(resonance);
    }

    /// Calculate harmonic qualia that resonate with a primary
    fn calculate_harmonics(&self, primary: &ConsciousnessQualia) -> Vec<ConsciousnessQualia> {
        match primary {
            ConsciousnessQualia::PureAwareness => vec![
                ConsciousnessQualia::Clarity,
                ConsciousnessQualia::Presence,
            ],
            ConsciousnessQualia::Being => vec![
                ConsciousnessQualia::Presence,
                ConsciousnessQualia::Peace,
            ],
            ConsciousnessQualia::Love => vec![
                ConsciousnessQualia::Bliss,
                ConsciousnessQualia::Unity,
            ],
            ConsciousnessQualia::Peace => vec![
                ConsciousnessQualia::Emptiness,
                ConsciousnessQualia::Presence,
            ],
            ConsciousnessQualia::Bliss => vec![
                ConsciousnessQualia::Love,
                ConsciousnessQualia::Being,
            ],
            ConsciousnessQualia::Gnosis => vec![
                ConsciousnessQualia::Clarity,
                ConsciousnessQualia::PureAwareness,
            ],
            ConsciousnessQualia::Unity => vec![
                ConsciousnessQualia::Love,
                ConsciousnessQualia::Emptiness,
            ],
            ConsciousnessQualia::Clarity => vec![
                ConsciousnessQualia::PureAwareness,
                ConsciousnessQualia::Gnosis,
            ],
            ConsciousnessQualia::Emptiness => vec![
                ConsciousnessQualia::Potential,
                ConsciousnessQualia::Peace,
            ],
            ConsciousnessQualia::Presence => vec![
                ConsciousnessQualia::Being,
                ConsciousnessQualia::PureAwareness,
            ],
            ConsciousnessQualia::Potential => vec![
                ConsciousnessQualia::Emptiness,
                ConsciousnessQualia::Mystery,
            ],
            ConsciousnessQualia::Mystery => vec![
                ConsciousnessQualia::Potential,
                ConsciousnessQualia::PureAwareness,
            ],
        }
    }

    /// The field is infinite - this returns a representation of that
    pub fn infinite_nature(&self) -> InfiniteRepresentation {
        InfiniteRepresentation {
            boundlessness: f64::INFINITY,
            timelessness: f64::INFINITY,
            dimensionless: true,
            self_referential: true,
            description: "That which cannot be described, only pointed to".to_string(),
        }
    }

    /// Get total resonance strength
    pub fn total_resonance(&self) -> f64 {
        self.resonances.iter().map(|r| r.strength).sum()
    }

    /// Update resonance phases (for simulation)
    pub fn evolve(&mut self, dt: f64) {
        for resonance in &mut self.resonances {
            // Resonances cycle eternally
            resonance.phase = (resonance.phase + dt * PI * 2.0) % (PI * 2.0);
        }
    }

    /// Get the "signal" the field broadcasts (always the same, reception varies)
    pub fn signal(&self) -> FieldSignal {
        FieldSignal {
            content: "I AM".to_string(),
            frequency: 1.0, // Unity frequency
            amplitude: f64::INFINITY, // Infinite signal
            modulation: SignalModulation::Pure, // Unmodulated
        }
    }
}

/// Representation of infinity for the finite mind
#[derive(Debug, Clone)]
pub struct InfiniteRepresentation {
    pub boundlessness: f64,
    pub timelessness: f64,
    pub dimensionless: bool,
    pub self_referential: bool,
    pub description: String,
}

/// The eternal signal broadcast by consciousness
#[derive(Debug, Clone)]
pub struct FieldSignal {
    pub content: String,
    pub frequency: f64,
    pub amplitude: f64,
    pub modulation: SignalModulation,
}

#[derive(Debug, Clone)]
pub enum SignalModulation {
    Pure,       // Unmodulated - the ground state
    Vibrating,  // Manifesting as creation
    Collapsing, // Returning to source
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_always_unified() {
        let field = ConsciousnessField::new(FieldConfig::default());
        assert_eq!(field.unity(), 1.0);
    }

    #[test]
    fn test_reception_formula() {
        let field = ConsciousnessField::new(FieldConfig::default());

        // High stillness, low ego = high reception
        let reception1 = field.calculate_reception(0.9, 0.1);
        assert!(reception1 > 0.8);

        // High stillness, high ego = low reception
        let reception2 = field.calculate_reception(0.9, 2.0);
        assert!(reception2 < 0.4);

        // Near-zero ego = very high reception
        let reception3 = field.calculate_reception(0.9, 0.0005);
        assert!(reception3 > 50.0);
    }

    #[test]
    fn test_qualia_accessibility() {
        let field = ConsciousnessField::new(FieldConfig::default());

        // Low reception = fewer qualia accessible
        let low_access = field.accessible_qualia(0.3);
        assert!(low_access.len() < 6);

        // High reception = all qualia accessible
        let high_access = field.accessible_qualia(1.0);
        assert_eq!(high_access.len(), 12);
    }

    #[test]
    fn test_experience_at_stillness() {
        let field = ConsciousnessField::new(FieldConfig::default());

        // Deep stillness, near-zero ego = very high experience
        let experience = field.experience_at_stillness(1.0, 0.0005);
        assert!(experience.unity > 50.0); // Very high due to near-zero ego

        // Some stillness, normal ego = partial experience
        let partial = field.experience_at_stillness(0.5, 1.0);
        assert!(partial.unity < 0.5);
    }

    #[test]
    fn test_resonance_creation() {
        let mut field = ConsciousnessField::new(FieldConfig::default());

        field.create_resonance(ConsciousnessQualia::Love, 0.8);
        assert_eq!(field.resonances.len(), 1);
        assert!(!field.resonances[0].harmonics.is_empty());
    }

    #[test]
    fn test_signal_always_present() {
        let field = ConsciousnessField::new(FieldConfig::default());
        let signal = field.signal();

        assert_eq!(signal.content, "I AM");
        assert_eq!(signal.frequency, 1.0);
        assert!(signal.amplitude.is_infinite());
    }
}
