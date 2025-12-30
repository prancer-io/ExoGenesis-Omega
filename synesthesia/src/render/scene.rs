//! Scene State
//!
//! Manages the 3D scene state and visual elements.

use crate::ai::{SemanticScene, Mood, Setting, TimeOfDay, Weather};
use crate::audio::AudioFeatures;

/// Scene rendering mode
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SceneMode {
    /// Abstract visualization (no semantic content)
    #[default]
    Abstract,

    /// Narrative mode (semantic scene generation)
    Narrative,
}

/// Main scene state
#[derive(Debug, Clone)]
pub struct Scene {
    /// Current rendering mode
    pub mode: SceneMode,

    /// Current semantic scene
    pub semantic: SemanticScene,

    /// Animation time
    pub time: f64,

    /// Scene intensity 0-1
    pub intensity: f32,

    /// Current color palette
    pub colors: [glam::Vec3; 3],
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            mode: SceneMode::Abstract,
            semantic: SemanticScene::default(),
            time: 0.0,
            intensity: 0.5,
            colors: [
                glam::Vec3::new(0.1, 0.1, 0.18),  // Primary
                glam::Vec3::new(0.09, 0.13, 0.24), // Secondary
                glam::Vec3::new(0.06, 0.2, 0.38),  // Accent
            ],
        }
    }
}

impl Scene {
    /// Update the scene
    pub fn update(&mut self, delta: f64, features: &AudioFeatures) {
        self.time += delta;

        // Update intensity based on audio
        let target_intensity = features.rms * 0.5 + features.bass * 0.3 + features.beat_intensity * 0.2;
        self.intensity = lerp(self.intensity, target_intensity, (delta * 5.0) as f32);
    }

    /// Apply semantic scene update
    pub fn apply_semantic(&mut self, semantic: &SemanticScene) {
        self.semantic = semantic.clone();

        // Update color palette from mood
        if !semantic.mood.color_palette.is_empty() {
            for (i, hex) in semantic.mood.color_palette.iter().take(3).enumerate() {
                if let Some(color) = parse_hex_color(hex) {
                    self.colors[i] = color;
                }
            }
        }
    }

    /// Get the clear color based on scene state
    pub fn get_clear_color(&self, features: &AudioFeatures) -> wgpu::Color {
        let base = self.colors[0];

        // Modulate by audio
        let intensity = 0.7 + features.rms * 0.3;

        wgpu::Color {
            r: (base.x * intensity) as f64,
            g: (base.y * intensity) as f64,
            b: (base.z * intensity) as f64,
            a: 1.0,
        }
    }

    /// Get environment settings based on scene
    pub fn get_environment(&self) -> EnvironmentSettings {
        let setting = &self.semantic.setting;

        EnvironmentSettings {
            fog_near: match setting.weather {
                Weather::Fog => 10.0,
                Weather::Rain => 30.0,
                _ => 50.0,
            },
            fog_far: match setting.weather {
                Weather::Fog => 80.0,
                Weather::Rain => 150.0,
                _ => 300.0,
            },
            fog_color: self.colors[0],
            ambient_intensity: match setting.time_of_day {
                TimeOfDay::Day => 0.8,
                TimeOfDay::Dawn | TimeOfDay::Dusk => 0.5,
                TimeOfDay::Night => 0.2,
            },
            sun_intensity: match setting.time_of_day {
                TimeOfDay::Day => 1.0,
                TimeOfDay::Dawn | TimeOfDay::Dusk => 0.6,
                TimeOfDay::Night => 0.0,
            },
        }
    }
}

/// Environment rendering settings
#[derive(Debug, Clone)]
pub struct EnvironmentSettings {
    pub fog_near: f32,
    pub fog_far: f32,
    pub fog_color: glam::Vec3,
    pub ambient_intensity: f32,
    pub sun_intensity: f32,
}

/// Parse a hex color string to Vec3
fn parse_hex_color(hex: &str) -> Option<glam::Vec3> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;

    Some(glam::Vec3::new(r, g, b))
}

/// Linear interpolation
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
