//! Dynamic Lighting System
//!
//! Point lights, spotlights, and area lights that react to music.

use crate::features::{MusicalFeatures, EmotionalValence};
use glam::Vec3;

/// Light types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Point,
    Spot,
    Directional,
    Area,
}

/// A dynamic light in the world
#[derive(Debug, Clone)]
pub struct SynLight {
    /// Unique identifier
    pub id: String,
    /// Light type
    pub light_type: LightType,
    /// Position in world space
    pub position: Vec3,
    /// Direction (for spot/directional)
    pub direction: Vec3,
    /// Color (RGB)
    pub color: [f32; 3],
    /// Intensity
    pub intensity: f32,
    /// Range (for point/spot)
    pub range: f32,
    /// Spot angle (for spot lights)
    pub spot_angle: f32,
    /// Spot softness
    pub spot_softness: f32,
    /// Area size (for area lights)
    pub area_size: [f32; 2],
    /// Cast shadows
    pub cast_shadows: bool,
    /// Audio timestamp this light was created from
    pub timestamp: f32,
    /// Beat-reactive (pulses on beats)
    pub beat_reactive: bool,
    /// Base intensity (for animation)
    pub base_intensity: f32,
}

impl SynLight {
    /// Create point light
    pub fn point(id: &str, position: Vec3, color: [f32; 3], intensity: f32, range: f32) -> Self {
        Self {
            id: id.to_string(),
            light_type: LightType::Point,
            position,
            direction: Vec3::NEG_Y,
            color,
            intensity,
            range,
            spot_angle: 0.0,
            spot_softness: 0.0,
            area_size: [0.0, 0.0],
            cast_shadows: true,
            timestamp: 0.0,
            beat_reactive: false,
            base_intensity: intensity,
        }
    }

    /// Create spot light
    pub fn spot(
        id: &str,
        position: Vec3,
        direction: Vec3,
        color: [f32; 3],
        intensity: f32,
        range: f32,
        angle: f32,
    ) -> Self {
        Self {
            id: id.to_string(),
            light_type: LightType::Spot,
            position,
            direction: direction.normalize(),
            color,
            intensity,
            range,
            spot_angle: angle,
            spot_softness: 0.2,
            area_size: [0.0, 0.0],
            cast_shadows: true,
            timestamp: 0.0,
            beat_reactive: false,
            base_intensity: intensity,
        }
    }

    /// Create directional light (sun)
    pub fn directional(id: &str, direction: Vec3, color: [f32; 3], intensity: f32) -> Self {
        Self {
            id: id.to_string(),
            light_type: LightType::Directional,
            position: Vec3::ZERO,
            direction: direction.normalize(),
            color,
            intensity,
            range: f32::INFINITY,
            spot_angle: 0.0,
            spot_softness: 0.0,
            area_size: [0.0, 0.0],
            cast_shadows: true,
            timestamp: 0.0,
            beat_reactive: false,
            base_intensity: intensity,
        }
    }

    /// Make light beat-reactive
    pub fn with_beat_reactive(mut self) -> Self {
        self.beat_reactive = true;
        self
    }

    /// Update intensity based on musical features
    pub fn update_from_features(&mut self, feature: &MusicalFeatures) {
        if self.beat_reactive {
            let beat_boost = if feature.is_beat {
                1.0 + feature.onset_strength
            } else {
                1.0
            };
            self.intensity = self.base_intensity * beat_boost;

            // Color shift based on emotion
            let emotion_color = feature.emotion.color();
            self.color = [
                self.color[0] * 0.7 + emotion_color[0] * 0.3,
                self.color[1] * 0.7 + emotion_color[1] * 0.3,
                self.color[2] * 0.7 + emotion_color[2] * 0.3,
            ];
        }
    }

    /// Convert to glTF light extension
    pub fn to_gltf_extension(&self) -> serde_json::Value {
        match self.light_type {
            LightType::Point => serde_json::json!({
                "type": "point",
                "color": self.color,
                "intensity": self.intensity,
                "range": self.range
            }),
            LightType::Spot => serde_json::json!({
                "type": "spot",
                "color": self.color,
                "intensity": self.intensity,
                "range": self.range,
                "spot": {
                    "innerConeAngle": self.spot_angle * 0.8,
                    "outerConeAngle": self.spot_angle
                }
            }),
            LightType::Directional => serde_json::json!({
                "type": "directional",
                "color": self.color,
                "intensity": self.intensity
            }),
            LightType::Area => serde_json::json!({
                "type": "point", // glTF doesn't support area lights, approximate as point
                "color": self.color,
                "intensity": self.intensity,
                "range": self.range
            }),
        }
    }
}

/// Light generator from musical features
pub struct LightGenerator {
    /// Maximum lights per chunk
    pub max_lights_per_chunk: usize,
    /// Light range multiplier
    pub range_multiplier: f32,
    /// Intensity multiplier
    pub intensity_multiplier: f32,
    /// Beat light boost factor
    pub beat_boost: f32,
}

impl LightGenerator {
    pub fn new() -> Self {
        Self {
            max_lights_per_chunk: 16,
            range_multiplier: 1.0,
            intensity_multiplier: 1.0,
            beat_boost: 2.0,
        }
    }

    /// Generate lights from musical features
    pub fn generate_from_features(
        &self,
        features: &[MusicalFeatures],
        time_scale: f32,
    ) -> Vec<SynLight> {
        let mut lights = Vec::new();

        // Create lights on beats
        for feature in features {
            if feature.is_beat && feature.onset_strength > 0.5 {
                let position = Vec3::new(
                    feature.timestamp as f32 * time_scale,
                    5.0 + feature.loudness * 10.0,
                    0.0,
                );

                let color = feature.emotion.color();
                let intensity = feature.onset_strength * self.intensity_multiplier * self.beat_boost;
                let range = 20.0 * self.range_multiplier;

                let mut light = SynLight::point(
                    &format!("beat_light_{:.3}", feature.timestamp),
                    position,
                    color,
                    intensity,
                    range,
                );
                light.timestamp = feature.timestamp as f32;
                light.beat_reactive = true;

                lights.push(light);
            }
        }

        // Add ambient fill lights based on emotion regions
        let mut last_emotion: Option<EmotionalValence> = None;
        let mut last_light_time = -10.0f64;

        for feature in features {
            if feature.timestamp - last_light_time > 5.0 {
                let emotion_changed = last_emotion.map(|e| e != feature.emotion).unwrap_or(true);

                if emotion_changed {
                    let position = Vec3::new(
                        feature.timestamp as f32 * time_scale,
                        15.0,
                        -10.0,
                    );

                    let color = feature.emotion.color();
                    let intensity = 0.5 * self.intensity_multiplier;

                    let light = SynLight::point(
                        &format!("ambient_light_{:.3}", feature.timestamp),
                        position,
                        color,
                        intensity,
                        50.0 * self.range_multiplier,
                    );

                    lights.push(light);
                    last_light_time = feature.timestamp;
                    last_emotion = Some(feature.emotion);
                }
            }
        }

        lights
    }

    /// Generate spotlight for landmark element
    pub fn generate_landmark_spotlight(
        &self,
        position: Vec3,
        feature: &MusicalFeatures,
    ) -> SynLight {
        let color = feature.emotion.color();
        let intensity = feature.loudness * self.intensity_multiplier * 2.0;

        let mut light = SynLight::spot(
            &format!("landmark_spot_{:.3}", feature.timestamp),
            position + Vec3::new(0.0, 10.0, -5.0),
            Vec3::new(0.0, -1.0, 0.2).normalize(),
            color,
            intensity,
            30.0 * self.range_multiplier,
            0.5, // ~30 degrees
        );
        light.timestamp = feature.timestamp as f32;
        light.beat_reactive = true;

        light
    }

    /// Generate rim lights for dramatic effect
    pub fn generate_rim_lights(
        &self,
        position: Vec3,
        radius: f32,
        color: [f32; 3],
    ) -> Vec<SynLight> {
        let mut lights = Vec::new();

        for i in 0..3 {
            let angle = (i as f32 / 3.0) * std::f32::consts::PI * 2.0;
            let offset = Vec3::new(angle.cos() * radius, 2.0, angle.sin() * radius);

            let direction = -offset.normalize();

            let light = SynLight::spot(
                &format!("rim_light_{}", i),
                position + offset,
                direction,
                color,
                0.8 * self.intensity_multiplier,
                radius * 2.0,
                0.4,
            );

            lights.push(light);
        }

        lights
    }
}

impl Default for LightGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Light manager for world
#[derive(Debug, Default)]
pub struct LightManager {
    /// All lights in the world
    pub lights: Vec<SynLight>,
    /// Main directional light (sun)
    pub sun: Option<SynLight>,
    /// Ambient light color
    pub ambient_color: [f32; 3],
    /// Ambient intensity
    pub ambient_intensity: f32,
}

impl LightManager {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            sun: None,
            ambient_color: [0.1, 0.1, 0.15],
            ambient_intensity: 0.3,
        }
    }

    /// Add light
    pub fn add(&mut self, light: SynLight) {
        self.lights.push(light);
    }

    /// Set sun light
    pub fn set_sun(&mut self, direction: Vec3, color: [f32; 3], intensity: f32) {
        self.sun = Some(SynLight::directional("sun", direction, color, intensity));
    }

    /// Get lights near position
    pub fn get_lights_near(&self, position: Vec3, radius: f32) -> Vec<&SynLight> {
        self.lights.iter()
            .filter(|l| (l.position - position).length() < radius + l.range)
            .collect()
    }

    /// Update all beat-reactive lights
    pub fn update(&mut self, features: &[MusicalFeatures]) {
        for light in &mut self.lights {
            // Find closest feature by timestamp
            if let Some(feature) = features.iter().min_by(|a, b| {
                (a.timestamp as f32 - light.timestamp).abs()
                    .partial_cmp(&(b.timestamp as f32 - light.timestamp).abs())
                    .unwrap()
            }) {
                light.update_from_features(feature);
            }
        }
    }

    /// Get total light count
    pub fn count(&self) -> usize {
        self.lights.len() + if self.sun.is_some() { 1 } else { 0 }
    }

    /// Export lights to glTF extensions
    pub fn to_gltf_extensions(&self) -> Vec<serde_json::Value> {
        let mut extensions = Vec::new();

        if let Some(ref sun) = self.sun {
            extensions.push(sun.to_gltf_extension());
        }

        for light in &self.lights {
            extensions.push(light.to_gltf_extension());
        }

        extensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::ChordType;

    fn create_test_feature(timestamp: f64, is_beat: bool) -> MusicalFeatures {
        MusicalFeatures {
            timestamp,
            pitch: 440.0,
            midi_note: 69,
            pitch_class: 9,
            octave: 4,
            onset_strength: if is_beat { 0.9 } else { 0.2 },
            is_beat,
            tempo: 120.0,
            beat_phase: 0.0,
            loudness: 0.6,
            dynamics_delta: 0.0,
            brightness: 0.5,
            roughness: 0.3,
            warmth: 0.5,
            sharpness: 0.4,
            key: 0,
            chord_type: ChordType::Major,
            tension: 0.3,
            valence: 0.5,
            arousal: 0.6,
            emotion: EmotionalValence::Joy,
            mfcc: [0.0; 13],
            chroma: [0.0; 12],
        }
    }

    #[test]
    fn test_point_light() {
        let light = SynLight::point("test", Vec3::ZERO, [1.0, 1.0, 1.0], 1.0, 10.0);
        assert_eq!(light.light_type, LightType::Point);
        assert_eq!(light.intensity, 1.0);
    }

    #[test]
    fn test_spot_light() {
        let light = SynLight::spot(
            "test",
            Vec3::new(0.0, 10.0, 0.0),
            Vec3::NEG_Y,
            [1.0, 1.0, 1.0],
            1.0,
            20.0,
            0.5,
        );
        assert_eq!(light.light_type, LightType::Spot);
    }

    #[test]
    fn test_light_generation() {
        let gen = LightGenerator::new();

        let features: Vec<_> = (0..10)
            .map(|i| create_test_feature(i as f64 * 0.5, i % 2 == 0))
            .collect();

        let lights = gen.generate_from_features(&features, 10.0);
        assert!(!lights.is_empty());
    }

    #[test]
    fn test_beat_reactive_update() {
        let mut light = SynLight::point("test", Vec3::ZERO, [1.0, 1.0, 1.0], 1.0, 10.0)
            .with_beat_reactive();

        let feature = create_test_feature(0.0, true);
        light.update_from_features(&feature);

        assert!(light.intensity > light.base_intensity);
    }

    #[test]
    fn test_light_manager() {
        let mut manager = LightManager::new();

        manager.add(SynLight::point("p1", Vec3::ZERO, [1.0, 0.0, 0.0], 1.0, 10.0));
        manager.add(SynLight::point("p2", Vec3::new(20.0, 0.0, 0.0), [0.0, 1.0, 0.0], 1.0, 10.0));

        let nearby = manager.get_lights_near(Vec3::ZERO, 5.0);
        assert_eq!(nearby.len(), 1);
    }

    #[test]
    fn test_gltf_extension() {
        let light = SynLight::point("test", Vec3::ZERO, [1.0, 0.8, 0.6], 2.0, 15.0);
        let ext = light.to_gltf_extension();

        assert_eq!(ext["type"], "point");
        assert_eq!(ext["intensity"], 2.0);
    }
}
