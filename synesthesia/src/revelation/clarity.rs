//! Clarity Calculator
//!
//! Determines how "revealed" the visualization should be based on
//! accumulated semantic context.

use super::accumulator::SemanticAccumulator;

/// Clarity levels matching visual revelation stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClarityLevel {
    /// Pure abstraction - only audio frequencies visible
    Abstract,
    /// Forms beginning to emerge from chaos
    Emerging,
    /// Shapes visible but soft, undefined
    Forming,
    /// Scene becoming clear, details emerging
    Clarifying,
    /// Full revelation - complete understanding
    Revealed,
}

impl ClarityLevel {
    /// Convert from float clarity value
    pub fn from_value(clarity: f32) -> Self {
        match clarity {
            c if c < 0.15 => Self::Abstract,
            c if c < 0.35 => Self::Emerging,
            c if c < 0.55 => Self::Forming,
            c if c < 0.80 => Self::Clarifying,
            _ => Self::Revealed,
        }
    }

    /// Get description for UI
    pub fn description(&self) -> &'static str {
        match self {
            Self::Abstract => "Pure Sensation",
            Self::Emerging => "Forms Emerging",
            Self::Forming => "Shapes Visible",
            Self::Clarifying => "Scene Forming",
            Self::Revealed => "Full Revelation",
        }
    }

    /// Get noise level for diffusion
    pub fn noise_level(&self) -> f32 {
        match self {
            Self::Abstract => 0.95,
            Self::Emerging => 0.70,
            Self::Forming => 0.45,
            Self::Clarifying => 0.20,
            Self::Revealed => 0.05,
        }
    }

    /// Get guidance scale for diffusion
    pub fn guidance_scale(&self) -> f32 {
        match self {
            Self::Abstract => 3.0,
            Self::Emerging => 5.0,
            Self::Forming => 8.0,
            Self::Clarifying => 12.0,
            Self::Revealed => 15.0,
        }
    }
}

/// Calculator for semantic clarity
pub struct ClarityCalculator {
    /// Weight for word count contribution
    word_weight: f32,

    /// Weight for theme detection contribution
    theme_weight: f32,

    /// Weight for scene fragment contribution
    scene_weight: f32,

    /// Minimum words before any clarity
    min_words: usize,

    /// Words needed for full clarity
    full_words: usize,
}

impl ClarityCalculator {
    /// Create new clarity calculator with default weights
    pub fn new() -> Self {
        Self {
            word_weight: 0.3,
            theme_weight: 0.3,
            scene_weight: 0.4,
            min_words: 5,
            full_words: 50,
        }
    }

    /// Calculate clarity from accumulated context
    pub fn calculate(&self, accumulator: &SemanticAccumulator) -> f32 {
        let word_clarity = self.calculate_word_clarity(accumulator);
        let theme_clarity = self.calculate_theme_clarity(accumulator);
        let scene_clarity = self.calculate_scene_clarity(accumulator);

        // Weighted combination
        let raw_clarity = word_clarity * self.word_weight
            + theme_clarity * self.theme_weight
            + scene_clarity * self.scene_weight;

        // Apply sigmoid for smooth transition
        self.sigmoid(raw_clarity)
    }

    /// Clarity contribution from word count
    fn calculate_word_clarity(&self, accumulator: &SemanticAccumulator) -> f32 {
        let words = accumulator.word_count();

        if words < self.min_words {
            return 0.0;
        }

        let progress = (words - self.min_words) as f32
            / (self.full_words - self.min_words) as f32;

        progress.clamp(0.0, 1.0)
    }

    /// Clarity contribution from detected themes
    fn calculate_theme_clarity(&self, accumulator: &SemanticAccumulator) -> f32 {
        let themes = accumulator.theme_count();

        match themes {
            0 => 0.0,
            1 => 0.3,
            2 => 0.6,
            3 => 0.8,
            _ => 1.0,
        }
    }

    /// Clarity contribution from AI scene analysis
    fn calculate_scene_clarity(&self, accumulator: &SemanticAccumulator) -> f32 {
        let scenes = accumulator.scene_count();

        match scenes {
            0 => 0.0,
            1 => 0.4,
            2 => 0.7,
            _ => 1.0,
        }
    }

    /// Smooth transition function
    fn sigmoid(&self, x: f32) -> f32 {
        1.0 / (1.0 + (-5.0 * (x - 0.5)).exp())
    }
}

impl Default for ClarityCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clarity_levels() {
        assert_eq!(ClarityLevel::from_value(0.0), ClarityLevel::Abstract);
        assert_eq!(ClarityLevel::from_value(0.2), ClarityLevel::Emerging);
        assert_eq!(ClarityLevel::from_value(0.4), ClarityLevel::Forming);
        assert_eq!(ClarityLevel::from_value(0.6), ClarityLevel::Clarifying);
        assert_eq!(ClarityLevel::from_value(0.9), ClarityLevel::Revealed);
    }

    #[test]
    fn test_noise_levels_decrease() {
        let levels = [
            ClarityLevel::Abstract,
            ClarityLevel::Emerging,
            ClarityLevel::Forming,
            ClarityLevel::Clarifying,
            ClarityLevel::Revealed,
        ];

        for i in 1..levels.len() {
            assert!(levels[i].noise_level() < levels[i-1].noise_level(),
                "Noise should decrease as clarity increases");
        }
    }
}
