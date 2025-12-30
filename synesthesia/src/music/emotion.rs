//! Emotion Mapping
//!
//! Maps musical features to emotional states, which drive visual aesthetics.
//! Based on music psychology research and theory.

use super::theory::{TheoryFeatures, Mode};
use super::structure::{Section, SectionType};

/// Primary emotions that music evokes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Emotion {
    // High energy, positive
    Joy,
    Triumph,
    Excitement,
    Euphoria,

    // High energy, negative
    Anger,
    Intensity,
    Urgency,
    Chaos,

    // Low energy, positive
    Peace,
    Tenderness,
    Hope,
    Nostalgia,

    // Low energy, negative
    Sadness,
    Melancholy,
    Tension,
    Dread,

    #[default]
    Neutral,
}

impl Emotion {
    /// Get primary color hue (0-360)
    pub fn hue(&self) -> f32 {
        match self {
            // Warm colors for positive high energy
            Emotion::Joy => 45.0,        // Golden yellow
            Emotion::Triumph => 30.0,    // Orange
            Emotion::Excitement => 15.0, // Red-orange
            Emotion::Euphoria => 300.0,  // Magenta

            // Hot/dark for negative high energy
            Emotion::Anger => 0.0,       // Red
            Emotion::Intensity => 350.0, // Deep red
            Emotion::Urgency => 20.0,    // Red-orange
            Emotion::Chaos => 280.0,     // Violet

            // Cool soft for positive low energy
            Emotion::Peace => 180.0,     // Cyan
            Emotion::Tenderness => 330.0,// Soft pink
            Emotion::Hope => 60.0,       // Warm yellow
            Emotion::Nostalgia => 35.0,  // Sepia

            // Cool dark for negative low energy
            Emotion::Sadness => 220.0,   // Blue
            Emotion::Melancholy => 250.0,// Blue-purple
            Emotion::Tension => 270.0,   // Purple
            Emotion::Dread => 260.0,     // Dark purple

            Emotion::Neutral => 200.0,   // Neutral blue
        }
    }

    /// Get saturation (0-1)
    pub fn saturation(&self) -> f32 {
        match self {
            Emotion::Joy | Emotion::Triumph | Emotion::Excitement | Emotion::Euphoria => 0.85,
            Emotion::Anger | Emotion::Intensity | Emotion::Urgency | Emotion::Chaos => 0.9,
            Emotion::Peace | Emotion::Tenderness | Emotion::Hope => 0.5,
            Emotion::Nostalgia => 0.4,
            Emotion::Sadness | Emotion::Melancholy | Emotion::Tension => 0.6,
            Emotion::Dread => 0.7,
            Emotion::Neutral => 0.3,
        }
    }

    /// Get lightness (0-1)
    pub fn lightness(&self) -> f32 {
        match self {
            Emotion::Joy | Emotion::Hope | Emotion::Euphoria => 0.7,
            Emotion::Triumph | Emotion::Excitement => 0.6,
            Emotion::Peace | Emotion::Tenderness => 0.65,
            Emotion::Nostalgia => 0.5,
            Emotion::Anger | Emotion::Intensity | Emotion::Urgency => 0.4,
            Emotion::Chaos => 0.45,
            Emotion::Sadness | Emotion::Melancholy => 0.35,
            Emotion::Tension | Emotion::Dread => 0.25,
            Emotion::Neutral => 0.5,
        }
    }

    /// Motion speed multiplier
    pub fn motion_speed(&self) -> f32 {
        match self {
            Emotion::Joy | Emotion::Excitement | Emotion::Euphoria => 1.3,
            Emotion::Triumph => 1.1,
            Emotion::Anger | Emotion::Urgency | Emotion::Chaos => 1.5,
            Emotion::Intensity => 1.2,
            Emotion::Peace | Emotion::Tenderness => 0.5,
            Emotion::Hope => 0.7,
            Emotion::Nostalgia => 0.6,
            Emotion::Sadness | Emotion::Melancholy => 0.4,
            Emotion::Tension => 0.8,
            Emotion::Dread => 0.3,
            Emotion::Neutral => 1.0,
        }
    }

    /// Particle density multiplier
    pub fn particle_density(&self) -> f32 {
        match self {
            Emotion::Euphoria | Emotion::Chaos => 2.0,
            Emotion::Joy | Emotion::Excitement | Emotion::Triumph => 1.5,
            Emotion::Anger | Emotion::Intensity | Emotion::Urgency => 1.3,
            Emotion::Peace | Emotion::Tenderness | Emotion::Sadness => 0.5,
            Emotion::Melancholy | Emotion::Hope | Emotion::Nostalgia => 0.7,
            Emotion::Tension | Emotion::Dread => 0.8,
            Emotion::Neutral => 1.0,
        }
    }
}

/// Complete emotional state
#[derive(Debug, Clone, Default)]
pub struct EmotionState {
    /// Primary emotion
    pub primary: Emotion,

    /// Primary emotion intensity (0-1)
    pub intensity: f32,

    /// Secondary emotion (for blending)
    pub secondary: Option<Emotion>,

    /// Blend factor toward secondary (0-1)
    pub blend: f32,

    /// Valence: positive (1) to negative (-1)
    pub valence: f32,

    /// Arousal: high energy (1) to low energy (-1)
    pub arousal: f32,
}

impl EmotionState {
    /// Get blended hue
    pub fn hue(&self) -> f32 {
        if let Some(secondary) = self.secondary {
            let h1 = self.primary.hue();
            let h2 = secondary.hue();
            // Blend on hue wheel (handle wrap-around)
            let diff = h2 - h1;
            let adjusted_diff = if diff.abs() > 180.0 {
                if diff > 0.0 { diff - 360.0 } else { diff + 360.0 }
            } else {
                diff
            };
            (h1 + adjusted_diff * self.blend).rem_euclid(360.0)
        } else {
            self.primary.hue()
        }
    }

    /// Get blended saturation
    pub fn saturation(&self) -> f32 {
        if let Some(secondary) = self.secondary {
            self.primary.saturation() * (1.0 - self.blend)
                + secondary.saturation() * self.blend
        } else {
            self.primary.saturation()
        }
    }

    /// Get blended lightness
    pub fn lightness(&self) -> f32 {
        if let Some(secondary) = self.secondary {
            self.primary.lightness() * (1.0 - self.blend)
                + secondary.lightness() * self.blend
        } else {
            self.primary.lightness()
        }
    }
}

/// Maps musical features to emotions
pub struct EmotionMapper {
    /// Smoothing factor for emotion transitions
    smoothing: f32,

    /// Previous state for interpolation
    prev_state: EmotionState,
}

impl EmotionMapper {
    pub fn new() -> Self {
        Self {
            smoothing: 0.1,
            prev_state: EmotionState::default(),
        }
    }

    /// Map musical features to emotion
    pub fn map(
        &mut self,
        theory: &TheoryFeatures,
        section: &Section,
        energy_arc: f32,
    ) -> EmotionState {
        // Calculate valence and arousal
        let valence = self.calculate_valence(theory, section);
        let arousal = self.calculate_arousal(theory, section, energy_arc);

        // Map to primary emotion using circumplex model
        let primary = self.map_to_emotion(valence, arousal);
        let intensity = (valence.abs() + arousal.abs()) / 2.0;

        // Determine secondary emotion based on tension
        let secondary = if theory.chord_tension > 0.5 {
            Some(self.tension_emotion(valence, arousal))
        } else {
            None
        };

        let target = EmotionState {
            primary,
            intensity,
            secondary,
            blend: theory.chord_tension * 0.5,
            valence,
            arousal,
        };

        // Smooth transition
        self.smooth_transition(target)
    }

    /// Calculate valence (positive/negative)
    fn calculate_valence(&self, theory: &TheoryFeatures, section: &Section) -> f32 {
        let mut valence = 0.0;

        // Mode contributes significantly
        if theory.mode.is_bright() {
            valence += 0.4;
        } else {
            valence -= 0.3;
        }

        // Chord tension pulls toward negative
        valence -= theory.chord_tension * 0.3;

        // Brightness contributes to positivity
        valence += (theory.timbre_brightness - 0.5) * 0.3;

        // Section type modifiers
        match section.section_type {
            SectionType::Chorus | SectionType::Drop => valence += 0.2,
            SectionType::Breakdown => valence -= 0.1,
            SectionType::Outro => valence += 0.1,
            _ => {}
        }

        valence.clamp(-1.0, 1.0)
    }

    /// Calculate arousal (energy level)
    fn calculate_arousal(
        &self,
        theory: &TheoryFeatures,
        section: &Section,
        energy_arc: f32,
    ) -> f32 {
        let mut arousal = 0.0;

        // Tempo is a major factor
        if theory.tempo > 140.0 {
            arousal += 0.4;
        } else if theory.tempo > 100.0 {
            arousal += 0.1;
        } else if theory.tempo < 80.0 {
            arousal -= 0.3;
        }

        // Tempo change (speeding up = more arousal)
        arousal += theory.tempo_derivative * 0.01;

        // Roughness increases arousal
        arousal += theory.timbre_roughness * 0.2;

        // Section energy
        arousal += (section.energy - 0.5) * 0.4;

        // Overall song arc position
        if energy_arc > 0.7 {
            arousal += 0.2;
        } else if energy_arc < 0.2 {
            arousal -= 0.1;
        }

        arousal.clamp(-1.0, 1.0)
    }

    /// Map valence/arousal to specific emotion
    fn map_to_emotion(&self, valence: f32, arousal: f32) -> Emotion {
        if arousal > 0.3 {
            // High arousal
            if valence > 0.3 {
                if arousal > 0.7 { Emotion::Euphoria }
                else if valence > 0.6 { Emotion::Joy }
                else { Emotion::Excitement }
            } else if valence < -0.3 {
                if arousal > 0.7 { Emotion::Chaos }
                else if valence < -0.6 { Emotion::Anger }
                else { Emotion::Intensity }
            } else {
                Emotion::Urgency
            }
        } else if arousal < -0.3 {
            // Low arousal
            if valence > 0.3 {
                if valence > 0.6 { Emotion::Peace }
                else { Emotion::Tenderness }
            } else if valence < -0.3 {
                if valence < -0.6 { Emotion::Dread }
                else { Emotion::Sadness }
            } else {
                Emotion::Melancholy
            }
        } else {
            // Medium arousal
            if valence > 0.3 {
                if valence > 0.5 { Emotion::Hope } else { Emotion::Nostalgia }
            } else if valence < -0.3 {
                Emotion::Tension
            } else {
                Emotion::Neutral
            }
        }
    }

    /// Get tension-related secondary emotion
    fn tension_emotion(&self, valence: f32, arousal: f32) -> Emotion {
        if arousal > 0.0 {
            if valence > 0.0 { Emotion::Excitement } else { Emotion::Intensity }
        } else {
            if valence > 0.0 { Emotion::Nostalgia } else { Emotion::Tension }
        }
    }

    /// Smooth transition between emotional states
    fn smooth_transition(&mut self, target: EmotionState) -> EmotionState {
        let smoothed = EmotionState {
            primary: target.primary,
            intensity: self.prev_state.intensity * (1.0 - self.smoothing)
                + target.intensity * self.smoothing,
            secondary: target.secondary,
            blend: self.prev_state.blend * (1.0 - self.smoothing)
                + target.blend * self.smoothing,
            valence: self.prev_state.valence * (1.0 - self.smoothing)
                + target.valence * self.smoothing,
            arousal: self.prev_state.arousal * (1.0 - self.smoothing)
                + target.arousal * self.smoothing,
        };

        self.prev_state = smoothed.clone();
        smoothed
    }
}

impl Default for EmotionMapper {
    fn default() -> Self {
        Self::new()
    }
}
