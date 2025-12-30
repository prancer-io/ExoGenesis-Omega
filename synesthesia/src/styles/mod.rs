//! Style/Theme System
//!
//! Defines visual styles that can be applied to music visualization.
//! Each style affects color palettes, shader parameters, and transition behavior.

mod presets;

pub use presets::*;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::music::Emotion;

/// A complete visual style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    /// Style identifier
    pub name: String,

    /// Human-readable description
    pub description: String,

    /// Color palette
    pub palette: ColorPalette,

    /// Shader effect parameters
    pub effects: EffectParams,

    /// Transition preferences
    pub transitions: TransitionPrefs,

    /// Per-emotion color overrides
    pub emotion_colors: HashMap<String, ColorMapping>,

    /// Base clarity curve modifier
    pub clarity_modifier: f32,

    /// Overall intensity multiplier
    pub intensity: f32,
}

/// Color palette for a style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary color (HSL)
    pub primary: [f32; 3],
    /// Secondary color (HSL)
    pub secondary: [f32; 3],
    /// Accent color (HSL)
    pub accent: [f32; 3],
    /// Background color (HSL)
    pub background: [f32; 3],
    /// Highlight color (HSL)
    pub highlight: [f32; 3],
    /// Shadow color (HSL)
    pub shadow: [f32; 3],
}

impl ColorPalette {
    /// Get primary as RGB (0-1)
    pub fn primary_rgb(&self) -> [f32; 3] {
        hsl_to_rgb(self.primary[0], self.primary[1], self.primary[2])
    }

    /// Get secondary as RGB
    pub fn secondary_rgb(&self) -> [f32; 3] {
        hsl_to_rgb(self.secondary[0], self.secondary[1], self.secondary[2])
    }

    /// Get accent as RGB
    pub fn accent_rgb(&self) -> [f32; 3] {
        hsl_to_rgb(self.accent[0], self.accent[1], self.accent[2])
    }

    /// Interpolate between two palettes
    pub fn lerp(&self, other: &ColorPalette, t: f32) -> ColorPalette {
        ColorPalette {
            primary: lerp_hsl(self.primary, other.primary, t),
            secondary: lerp_hsl(self.secondary, other.secondary, t),
            accent: lerp_hsl(self.accent, other.accent, t),
            background: lerp_hsl(self.background, other.background, t),
            highlight: lerp_hsl(self.highlight, other.highlight, t),
            shadow: lerp_hsl(self.shadow, other.shadow, t),
        }
    }
}

/// Shader effect parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectParams {
    /// Base bloom intensity (0-1)
    pub bloom: f32,
    /// Chromatic aberration amount (0-0.1)
    pub chromatic_aberration: f32,
    /// Vignette strength (0-1)
    pub vignette: f32,
    /// Film grain amount (0-0.2)
    pub grain: f32,
    /// Motion blur strength (0-1)
    pub motion_blur: f32,
    /// Contrast adjustment (0.5-2.0)
    pub contrast: f32,
    /// Saturation adjustment (0-2.0)
    pub saturation: f32,
    /// Gamma correction (0.5-2.0)
    pub gamma: f32,
    /// Glow radius
    pub glow_radius: f32,
    /// Beat reactivity multiplier
    pub beat_reactivity: f32,
}

impl Default for EffectParams {
    fn default() -> Self {
        Self {
            bloom: 0.3,
            chromatic_aberration: 0.01,
            vignette: 0.3,
            grain: 0.05,
            motion_blur: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            gamma: 0.85,
            glow_radius: 1.0,
            beat_reactivity: 1.0,
        }
    }
}

/// Transition preferences for a style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionPrefs {
    /// Default transition type
    pub default_type: String,
    /// Default duration (seconds)
    pub duration: f32,
    /// Use beat-synced transitions
    pub beat_sync: bool,
    /// Flash intensity on drops
    pub drop_flash: f32,
    /// Transition on section change
    pub section_transitions: bool,
}

impl Default for TransitionPrefs {
    fn default() -> Self {
        Self {
            default_type: "crossfade".to_string(),
            duration: 0.5,
            beat_sync: true,
            drop_flash: 0.8,
            section_transitions: true,
        }
    }
}

/// Color mapping for a specific emotion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorMapping {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

/// Style manager
pub struct StyleManager {
    /// Available styles
    styles: HashMap<String, Style>,
    /// Current active style
    current: String,
    /// Target style (for transitions)
    target: Option<String>,
    /// Transition progress
    transition_progress: f32,
}

impl StyleManager {
    /// Create new style manager with default styles
    pub fn new() -> Self {
        let mut styles = HashMap::new();

        // Load preset styles
        styles.insert("default".to_string(), presets::default_style());
        styles.insert("neon".to_string(), presets::neon_style());
        styles.insert("ethereal".to_string(), presets::ethereal_style());
        styles.insert("cinematic".to_string(), presets::cinematic_style());
        styles.insert("retro".to_string(), presets::retro_style());
        styles.insert("minimal".to_string(), presets::minimal_style());
        styles.insert("psychedelic".to_string(), presets::psychedelic_style());
        styles.insert("noir".to_string(), presets::noir_style());

        Self {
            styles,
            current: "default".to_string(),
            target: None,
            transition_progress: 0.0,
        }
    }

    /// Get current style
    pub fn current(&self) -> &Style {
        self.styles.get(&self.current).unwrap()
    }

    /// Get style by name
    pub fn get(&self, name: &str) -> Option<&Style> {
        self.styles.get(name)
    }

    /// Set active style (immediate)
    pub fn set(&mut self, name: &str) -> bool {
        if self.styles.contains_key(name) {
            self.current = name.to_string();
            self.target = None;
            self.transition_progress = 0.0;
            true
        } else {
            false
        }
    }

    /// Transition to a new style
    pub fn transition_to(&mut self, name: &str, duration: f32) -> bool {
        if self.styles.contains_key(name) {
            self.target = Some(name.to_string());
            self.transition_progress = 0.0;
            true
        } else {
            false
        }
    }

    /// Update style transition
    pub fn update(&mut self, delta: f32) {
        if let Some(ref target) = self.target {
            self.transition_progress += delta / 2.0; // 2 second default transition
            if self.transition_progress >= 1.0 {
                self.current = target.clone();
                self.target = None;
                self.transition_progress = 0.0;
            }
        }
    }

    /// Get interpolated style (during transitions)
    pub fn interpolated(&self) -> Style {
        if let Some(ref target) = self.target {
            let from = self.styles.get(&self.current).unwrap();
            let to = self.styles.get(target).unwrap();
            interpolate_styles(from, to, self.transition_progress)
        } else {
            self.current().clone()
        }
    }

    /// Add custom style
    pub fn add_style(&mut self, style: Style) {
        self.styles.insert(style.name.clone(), style);
    }

    /// List available styles
    pub fn list(&self) -> Vec<&str> {
        self.styles.keys().map(|s| s.as_str()).collect()
    }

    /// Get color for emotion based on current style
    pub fn emotion_color(&self, emotion: &Emotion) -> [f32; 3] {
        let style = self.current();
        let key = format!("{:?}", emotion).to_lowercase();

        if let Some(mapping) = style.emotion_colors.get(&key) {
            [mapping.hue, mapping.saturation, mapping.lightness]
        } else {
            // Default emotion colors
            match emotion {
                Emotion::Joy => [45.0, 0.85, 0.6],
                Emotion::Triumph => [30.0, 0.8, 0.55],
                Emotion::Excitement => [15.0, 0.85, 0.55],
                Emotion::Euphoria => [300.0, 0.9, 0.6],
                Emotion::Anger => [0.0, 0.9, 0.4],
                Emotion::Intensity => [350.0, 0.85, 0.45],
                Emotion::Urgency => [20.0, 0.8, 0.5],
                Emotion::Chaos => [280.0, 0.85, 0.45],
                Emotion::Peace => [180.0, 0.5, 0.6],
                Emotion::Tenderness => [330.0, 0.4, 0.65],
                Emotion::Hope => [60.0, 0.6, 0.6],
                Emotion::Nostalgia => [35.0, 0.4, 0.5],
                Emotion::Sadness => [220.0, 0.6, 0.35],
                Emotion::Melancholy => [250.0, 0.5, 0.4],
                Emotion::Tension => [270.0, 0.6, 0.35],
                Emotion::Dread => [260.0, 0.7, 0.25],
                Emotion::Neutral => [200.0, 0.3, 0.5],
            }
        }
    }
}

impl Default for StyleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Interpolate between two styles
fn interpolate_styles(from: &Style, to: &Style, t: f32) -> Style {
    Style {
        name: to.name.clone(),
        description: to.description.clone(),
        palette: from.palette.lerp(&to.palette, t),
        effects: EffectParams {
            bloom: lerp(from.effects.bloom, to.effects.bloom, t),
            chromatic_aberration: lerp(from.effects.chromatic_aberration, to.effects.chromatic_aberration, t),
            vignette: lerp(from.effects.vignette, to.effects.vignette, t),
            grain: lerp(from.effects.grain, to.effects.grain, t),
            motion_blur: lerp(from.effects.motion_blur, to.effects.motion_blur, t),
            contrast: lerp(from.effects.contrast, to.effects.contrast, t),
            saturation: lerp(from.effects.saturation, to.effects.saturation, t),
            gamma: lerp(from.effects.gamma, to.effects.gamma, t),
            glow_radius: lerp(from.effects.glow_radius, to.effects.glow_radius, t),
            beat_reactivity: lerp(from.effects.beat_reactivity, to.effects.beat_reactivity, t),
        },
        transitions: to.transitions.clone(),
        emotion_colors: to.emotion_colors.clone(),
        clarity_modifier: lerp(from.clarity_modifier, to.clarity_modifier, t),
        intensity: lerp(from.intensity, to.intensity, t),
    }
}

/// Linear interpolation
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// HSL interpolation
fn lerp_hsl(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    // Handle hue wraparound
    let mut h_diff = b[0] - a[0];
    if h_diff > 180.0 {
        h_diff -= 360.0;
    } else if h_diff < -180.0 {
        h_diff += 360.0;
    }
    let h = (a[0] + h_diff * t + 360.0) % 360.0;

    [h, lerp(a[1], b[1], t), lerp(a[2], b[2], t)]
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 3] {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [r + m, g + m, b + m]
}
