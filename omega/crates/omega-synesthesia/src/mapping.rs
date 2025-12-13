//! Spatial Mapping Module
//!
//! Maps musical features to 3D coordinates and properties.

use crate::features::{MusicalFeatures, EmotionalValence};
use crate::genre::GenreStyle;
use crate::Result;
use glam::Vec3;

/// 3D coordinate in the synesthesia world
#[derive(Debug, Clone, Copy)]
pub struct Coordinate3D {
    pub x: f32,  // Time axis
    pub y: f32,  // Pitch/height axis
    pub z: f32,  // Intensity/width axis
}

impl Coordinate3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

/// Spatial data for a single moment in the world
#[derive(Debug, Clone)]
pub struct SpatialMoment {
    /// Position in 3D space
    pub position: Coordinate3D,
    /// Scale factor
    pub scale: f32,
    /// Rotation (Euler angles)
    pub rotation: Vec3,
    /// Color (RGB)
    pub color: [f32; 3],
    /// Emission intensity
    pub emission: f32,
    /// Shape type to generate
    pub shape: ShapeHint,
    /// Material properties
    pub material_hint: MaterialHint,
    /// Original features for reference
    pub features: MusicalFeatures,
}

/// Hint for what shape to generate
#[derive(Debug, Clone, Copy)]
pub enum ShapeHint {
    /// Smooth, rounded shapes
    Organic,
    /// Sharp, angular shapes
    Crystalline,
    /// Flowing, ribbon-like shapes
    Flowing,
    /// Particle cloud
    Particles,
    /// Solid block
    Block,
    /// Spire/tower
    Spire,
    /// Dome/arch
    Dome,
    /// Wave/ripple
    Wave,
}

/// Hint for material properties
#[derive(Debug, Clone)]
pub struct MaterialHint {
    /// Base color (RGB)
    pub color: [f32; 3],
    /// Metallic factor (0-1)
    pub metallic: f32,
    /// Roughness (0-1)
    pub roughness: f32,
    /// Emission color and intensity
    pub emission: [f32; 3],
    /// Transparency (0-1, 0 = opaque)
    pub transparency: f32,
    /// Texture type hint
    pub texture: TextureHint,
}

/// Hint for texture generation
#[derive(Debug, Clone, Copy)]
pub enum TextureHint {
    Smooth,
    Marble,
    Wood,
    Metal,
    Glass,
    Crystal,
    Fabric,
    Organic,
    Energy,
    Void,
}

/// Configuration for spatial mapping
#[derive(Debug, Clone)]
pub struct MappingConfig {
    pub time_scale: f32,
    pub pitch_scale: f32,
    pub intensity_scale: f32,
    pub beat_emphasis: f32,
    pub emotion_influence: f32,
}

impl Default for MappingConfig {
    fn default() -> Self {
        Self {
            time_scale: 10.0,
            pitch_scale: 50.0,
            intensity_scale: 20.0,
            beat_emphasis: 2.0,
            emotion_influence: 0.5,
        }
    }
}

/// Spatial mapper for converting features to 3D
pub struct SpatialMapper {
    config: MappingConfig,
}

impl SpatialMapper {
    pub fn new(time_scale: f32, pitch_scale: f32) -> Self {
        Self {
            config: MappingConfig {
                time_scale,
                pitch_scale,
                ..Default::default()
            },
        }
    }

    pub fn with_config(config: MappingConfig) -> Self {
        Self { config }
    }

    /// Map all features to spatial data
    pub fn map_features(&self, features: &[MusicalFeatures], style: &GenreStyle) -> Result<Vec<SpatialMoment>> {
        features
            .iter()
            .map(|f| self.map_single(f, style))
            .collect()
    }

    /// Map a single feature to spatial data
    fn map_single(&self, feature: &MusicalFeatures, style: &GenreStyle) -> Result<SpatialMoment> {
        // === POSITION ===
        // X = Time (walking forward = moving through song)
        let x = feature.timestamp as f32 * self.config.time_scale;

        // Y = Pitch (high notes = high up)
        let pitch_normalized = (feature.midi_note as f32 - 21.0) / 88.0; // Piano range
        let y = pitch_normalized * self.config.pitch_scale;

        // Z = Harmonic spread / stereo width
        let z = (feature.brightness - 0.5) * self.config.intensity_scale;

        let position = Coordinate3D::new(x, y, z);

        // === SCALE ===
        // Based on loudness and beat emphasis
        let base_scale = 0.5 + feature.loudness * 1.5;
        let beat_boost = if feature.is_beat { self.config.beat_emphasis } else { 1.0 };
        let scale = base_scale * beat_boost * style.scale_multiplier;

        // === ROTATION ===
        // Based on harmonic tension and rhythm phase
        let rotation = Vec3::new(
            feature.tension * std::f32::consts::PI * 0.25,
            feature.beat_phase * std::f32::consts::PI * 2.0,
            feature.brightness * std::f32::consts::PI * 0.1,
        );

        // === COLOR ===
        let base_color = self.emotion_to_color(feature.emotion, style);
        let color = self.apply_pitch_tint(base_color, feature.pitch_class, style);

        // === EMISSION ===
        let emission = feature.onset_strength * style.emission_intensity;

        // === SHAPE ===
        let shape = self.select_shape(feature, style);

        // === MATERIAL ===
        let material_hint = self.create_material(feature, style, color);

        Ok(SpatialMoment {
            position,
            scale,
            rotation,
            color,
            emission,
            shape,
            material_hint,
            features: feature.clone(),
        })
    }

    /// Convert time to world position
    pub fn time_to_position(&self, time_seconds: f64) -> Coordinate3D {
        Coordinate3D::new(
            time_seconds as f32 * self.config.time_scale,
            0.0,
            0.0,
        )
    }

    /// Map emotion to color
    fn emotion_to_color(&self, emotion: EmotionalValence, style: &GenreStyle) -> [f32; 3] {
        let base = emotion.color();

        // Blend with genre palette
        let genre_influence = 0.3;
        let style_color = style.primary_color;

        [
            base[0] * (1.0 - genre_influence) + style_color[0] * genre_influence,
            base[1] * (1.0 - genre_influence) + style_color[1] * genre_influence,
            base[2] * (1.0 - genre_influence) + style_color[2] * genre_influence,
        ]
    }

    /// Apply pitch-based color tinting
    fn apply_pitch_tint(&self, base: [f32; 3], pitch_class: u8, _style: &GenreStyle) -> [f32; 3] {
        // Create subtle hue shift based on pitch class (circle of fifths)
        let hue_shift = (pitch_class as f32 / 12.0) * 0.2;

        [
            (base[0] + hue_shift * 0.1).clamp(0.0, 1.0),
            (base[1] + hue_shift * 0.05).clamp(0.0, 1.0),
            (base[2] - hue_shift * 0.05).clamp(0.0, 1.0),
        ]
    }

    /// Select appropriate shape based on features
    fn select_shape(&self, feature: &MusicalFeatures, style: &GenreStyle) -> ShapeHint {
        // Beats get emphasized shapes
        if feature.is_beat && feature.onset_strength > 0.7 {
            return style.beat_shape;
        }

        // High tension = crystalline
        if feature.tension > 0.7 {
            return ShapeHint::Crystalline;
        }

        // Peaceful = organic
        if feature.emotion == EmotionalValence::Peace {
            return ShapeHint::Organic;
        }

        // High brightness = spires
        if feature.brightness > 0.7 {
            return ShapeHint::Spire;
        }

        // Default to style's primary shape
        style.primary_shape
    }

    /// Create material properties
    fn create_material(&self, feature: &MusicalFeatures, style: &GenreStyle, base_color: [f32; 3]) -> MaterialHint {
        let metallic = style.base_metallic + feature.brightness * 0.3;
        let roughness = style.base_roughness * (1.0 - feature.brightness * 0.5);

        let emission_intensity = feature.onset_strength * style.emission_intensity;
        let emission = [
            base_color[0] * emission_intensity,
            base_color[1] * emission_intensity,
            base_color[2] * emission_intensity,
        ];

        let transparency = if feature.emotion == EmotionalValence::Peace {
            0.2
        } else {
            style.base_transparency
        };

        let texture = self.select_texture(feature, style);

        MaterialHint {
            color: base_color,
            metallic,
            roughness,
            emission,
            transparency,
            texture,
        }
    }

    /// Select texture type
    fn select_texture(&self, feature: &MusicalFeatures, style: &GenreStyle) -> TextureHint {
        match feature.emotion {
            EmotionalValence::Joy => TextureHint::Energy,
            EmotionalValence::Sadness => TextureHint::Marble,
            EmotionalValence::Anger => TextureHint::Metal,
            EmotionalValence::Peace => TextureHint::Smooth,
            EmotionalValence::Fear => TextureHint::Void,
            EmotionalValence::Surprise => TextureHint::Crystal,
            EmotionalValence::Neutral => style.primary_texture,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genre::Genre;

    #[test]
    fn test_mapper_creation() {
        let mapper = SpatialMapper::new(10.0, 50.0);
        let pos = mapper.time_to_position(1.0);
        assert_eq!(pos.x, 10.0);
    }

    #[test]
    fn test_coordinate() {
        let coord = Coordinate3D::new(1.0, 2.0, 3.0);
        let vec = coord.to_vec3();
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }
}
