//! Animation System for Synesthesia Worlds
//!
//! Creates animations from musical features for glTF export.

use crate::features::MusicalFeatures;
use glam::{Vec3, Quat};

/// Animation clip containing keyframes
#[derive(Debug, Clone)]
pub struct AnimationClip {
    /// Clip name
    pub name: String,
    /// Duration in seconds
    pub duration: f32,
    /// Animation channels
    pub channels: Vec<AnimationChannel>,
}

impl AnimationClip {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            duration: 0.0,
            channels: Vec::new(),
        }
    }

    /// Add channel
    pub fn add_channel(&mut self, channel: AnimationChannel) {
        self.duration = self.duration.max(channel.keyframes.last()
            .map(|k| k.time)
            .unwrap_or(0.0));
        self.channels.push(channel);
    }

    /// Get total keyframe count
    pub fn keyframe_count(&self) -> usize {
        self.channels.iter().map(|c| c.keyframes.len()).sum()
    }
}

/// Animation channel for a single property
#[derive(Debug, Clone)]
pub struct AnimationChannel {
    /// Target node index
    pub target_node: usize,
    /// Property being animated
    pub property: AnimationProperty,
    /// Interpolation mode
    pub interpolation: Interpolation,
    /// Keyframes
    pub keyframes: Vec<Keyframe>,
}

impl AnimationChannel {
    pub fn new(target_node: usize, property: AnimationProperty) -> Self {
        Self {
            target_node,
            property,
            interpolation: Interpolation::Linear,
            keyframes: Vec::new(),
        }
    }

    /// Add keyframe
    pub fn add_keyframe(&mut self, time: f32, value: KeyframeValue) {
        self.keyframes.push(Keyframe { time, value });
    }

    /// Sort keyframes by time
    pub fn sort_keyframes(&mut self) {
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// Sample value at time
    pub fn sample(&self, time: f32) -> Option<KeyframeValue> {
        if self.keyframes.is_empty() {
            return None;
        }

        if time <= self.keyframes[0].time {
            return Some(self.keyframes[0].value.clone());
        }

        if time >= self.keyframes.last().unwrap().time {
            return Some(self.keyframes.last().unwrap().value.clone());
        }

        // Find surrounding keyframes
        for i in 0..self.keyframes.len() - 1 {
            let k0 = &self.keyframes[i];
            let k1 = &self.keyframes[i + 1];

            if time >= k0.time && time <= k1.time {
                let t = (time - k0.time) / (k1.time - k0.time);
                return Some(self.interpolate(&k0.value, &k1.value, t));
            }
        }

        None
    }

    /// Interpolate between values
    fn interpolate(&self, v0: &KeyframeValue, v1: &KeyframeValue, t: f32) -> KeyframeValue {
        match self.interpolation {
            Interpolation::Step => v0.clone(),
            Interpolation::Linear => v0.lerp(v1, t),
            Interpolation::CubicSpline => v0.lerp(v1, t), // Simplified
        }
    }
}

/// Animation property type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationProperty {
    Translation,
    Rotation,
    Scale,
    Weights, // Morph targets
}

/// Interpolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interpolation {
    Step,
    Linear,
    CubicSpline,
}

/// Single keyframe
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Time in seconds
    pub time: f32,
    /// Value at this keyframe
    pub value: KeyframeValue,
}

/// Keyframe value types
#[derive(Debug, Clone)]
pub enum KeyframeValue {
    Translation(Vec3),
    Rotation(Quat),
    Scale(Vec3),
    Weights(Vec<f32>),
}

impl KeyframeValue {
    /// Linearly interpolate between values
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Self::Translation(v0), Self::Translation(v1)) => {
                Self::Translation(v0.lerp(*v1, t))
            }
            (Self::Rotation(q0), Self::Rotation(q1)) => {
                Self::Rotation(q0.slerp(*q1, t))
            }
            (Self::Scale(v0), Self::Scale(v1)) => {
                Self::Scale(v0.lerp(*v1, t))
            }
            (Self::Weights(w0), Self::Weights(w1)) => {
                let weights: Vec<f32> = w0.iter()
                    .zip(w1.iter())
                    .map(|(a, b)| a + (b - a) * t)
                    .collect();
                Self::Weights(weights)
            }
            _ => self.clone(),
        }
    }
}

/// Animation generator from musical features
pub struct AnimationGenerator {
    /// Sample rate for keyframes (keyframes per second)
    pub keyframe_rate: f32,
    /// Scale multiplier for beat reactions
    pub beat_scale: f32,
    /// Amplitude for pitch-based movement
    pub pitch_amplitude: f32,
    /// Enable rotation animations
    pub enable_rotation: bool,
    /// Enable scale pulsing on beats
    pub enable_scale_pulse: bool,
}

impl AnimationGenerator {
    pub fn new() -> Self {
        Self {
            keyframe_rate: 30.0,
            beat_scale: 1.5,
            pitch_amplitude: 0.5,
            enable_rotation: true,
            enable_scale_pulse: true,
        }
    }

    /// Generate camera path animation from features
    pub fn generate_camera_path(&self, features: &[MusicalFeatures], time_scale: f32) -> AnimationClip {
        let mut clip = AnimationClip::new("CameraPath");
        let mut translation_channel = AnimationChannel::new(0, AnimationProperty::Translation);
        let mut rotation_channel = AnimationChannel::new(0, AnimationProperty::Rotation);

        let keyframe_interval = 1.0 / self.keyframe_rate;
        let mut last_time = -1.0f32;

        for feature in features {
            let time = feature.timestamp as f32;

            // Skip if too close to last keyframe
            if time - last_time < keyframe_interval {
                continue;
            }
            last_time = time;

            // Camera follows the music through time (X axis)
            let x = time * time_scale;
            // Height based on pitch
            let y = 5.0 + (feature.pitch / 440.0 - 1.0) * self.pitch_amplitude * 10.0;
            // Slight Z variation based on loudness
            let z = -10.0 + feature.loudness * 5.0;

            translation_channel.add_keyframe(
                time,
                KeyframeValue::Translation(Vec3::new(x, y, z)),
            );

            // Rotation based on brightness (look direction)
            let look_angle = (feature.brightness - 0.5) * 0.3;
            let rotation = Quat::from_euler(glam::EulerRot::XYZ, -0.2, look_angle, 0.0);
            rotation_channel.add_keyframe(
                time,
                KeyframeValue::Rotation(rotation),
            );
        }

        clip.add_channel(translation_channel);
        clip.add_channel(rotation_channel);
        clip
    }

    /// Generate element animation based on beat
    pub fn generate_beat_animation(
        &self,
        element_node: usize,
        features: &[MusicalFeatures],
        base_scale: f32,
    ) -> AnimationClip {
        let mut clip = AnimationClip::new(&format!("BeatPulse_{}", element_node));

        if !self.enable_scale_pulse {
            return clip;
        }

        let mut scale_channel = AnimationChannel::new(element_node, AnimationProperty::Scale);

        for feature in features {
            let time = feature.timestamp as f32;

            if feature.is_beat {
                // Start of beat - scale up
                scale_channel.add_keyframe(
                    time,
                    KeyframeValue::Scale(Vec3::splat(base_scale * self.beat_scale)),
                );

                // Return to normal after beat
                scale_channel.add_keyframe(
                    time + 0.1,
                    KeyframeValue::Scale(Vec3::splat(base_scale * 1.1)),
                );
                scale_channel.add_keyframe(
                    time + 0.2,
                    KeyframeValue::Scale(Vec3::splat(base_scale)),
                );
            }
        }

        clip.add_channel(scale_channel);
        clip
    }

    /// Generate rotation animation based on melodic contour
    pub fn generate_melodic_rotation(
        &self,
        element_node: usize,
        features: &[MusicalFeatures],
    ) -> AnimationClip {
        let mut clip = AnimationClip::new(&format!("MelodicRotation_{}", element_node));

        if !self.enable_rotation {
            return clip;
        }

        let mut rotation_channel = AnimationChannel::new(element_node, AnimationProperty::Rotation);
        let keyframe_interval = 1.0 / self.keyframe_rate;
        let mut last_time = -1.0f32;

        let mut cumulative_rotation = 0.0f32;

        for (i, feature) in features.iter().enumerate() {
            let time = feature.timestamp as f32;

            if time - last_time < keyframe_interval {
                continue;
            }
            last_time = time;

            // Pitch change drives rotation
            let pitch_delta = if i > 0 {
                (feature.pitch - features[i - 1].pitch) / 100.0
            } else {
                0.0
            };

            cumulative_rotation += pitch_delta * 0.1;

            // Add some rhythm-based wobble
            let wobble = if feature.is_beat { 0.1 } else { 0.0 };

            let rotation = Quat::from_euler(
                glam::EulerRot::XYZ,
                wobble,
                cumulative_rotation,
                0.0,
            );

            rotation_channel.add_keyframe(time, KeyframeValue::Rotation(rotation));
        }

        clip.add_channel(rotation_channel);
        clip
    }

    /// Generate emission pulse animation
    pub fn generate_emission_animation(
        &self,
        features: &[MusicalFeatures],
    ) -> Vec<(f32, f32)> {
        // Returns (time, intensity) pairs for emission pulsing
        let mut emissions = Vec::new();

        for feature in features {
            let time = feature.timestamp as f32;
            let intensity = if feature.is_beat {
                feature.onset_strength * 2.0
            } else {
                feature.loudness * 0.5
            };
            emissions.push((time, intensity.clamp(0.0, 2.0)));
        }

        emissions
    }

    /// Generate morph target weights animation
    pub fn generate_morph_animation(
        &self,
        element_node: usize,
        features: &[MusicalFeatures],
        num_targets: usize,
    ) -> AnimationClip {
        let mut clip = AnimationClip::new(&format!("Morph_{}", element_node));
        let mut weights_channel = AnimationChannel::new(element_node, AnimationProperty::Weights);

        let keyframe_interval = 1.0 / self.keyframe_rate;
        let mut last_time = -1.0f32;

        for feature in features {
            let time = feature.timestamp as f32;

            if time - last_time < keyframe_interval {
                continue;
            }
            last_time = time;

            // Generate weights based on chroma
            let mut weights = vec![0.0f32; num_targets];
            for i in 0..num_targets.min(12) {
                weights[i] = feature.chroma[i];
            }

            weights_channel.add_keyframe(time, KeyframeValue::Weights(weights));
        }

        clip.add_channel(weights_channel);
        clip
    }
}

impl Default for AnimationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Light animation data
#[derive(Debug, Clone)]
pub struct LightAnimation {
    /// Light node index
    pub node_index: usize,
    /// Intensity keyframes (time, intensity)
    pub intensity: Vec<(f32, f32)>,
    /// Color keyframes (time, rgb)
    pub color: Vec<(f32, [f32; 3])>,
}

impl LightAnimation {
    pub fn new(node_index: usize) -> Self {
        Self {
            node_index,
            intensity: Vec::new(),
            color: Vec::new(),
        }
    }

    /// Generate from features
    pub fn from_features(node_index: usize, features: &[MusicalFeatures]) -> Self {
        let mut anim = Self::new(node_index);

        for feature in features {
            let time = feature.timestamp as f32;

            // Intensity based on loudness and beats
            let intensity = if feature.is_beat {
                1.0 + feature.onset_strength
            } else {
                0.5 + feature.loudness * 0.5
            };
            anim.intensity.push((time, intensity));

            // Color based on emotion
            let emotion_color = feature.emotion.color();
            anim.color.push((time, emotion_color));
        }

        anim
    }

    /// Sample intensity at time
    pub fn sample_intensity(&self, time: f32) -> f32 {
        if self.intensity.is_empty() {
            return 1.0;
        }

        // Find surrounding keyframes
        let mut prev = &self.intensity[0];
        for kf in &self.intensity {
            if kf.0 > time {
                let t = (time - prev.0) / (kf.0 - prev.0).max(0.001);
                return prev.1 + (kf.1 - prev.1) * t;
            }
            prev = kf;
        }

        prev.1
    }
}

/// Animation exporter to glTF format
pub struct AnimationExporter {
    /// Sample rate for baking animations
    pub sample_rate: f32,
}

impl AnimationExporter {
    pub fn new() -> Self {
        Self {
            sample_rate: 30.0,
        }
    }

    /// Convert animation clip to glTF animation data
    pub fn to_gltf_data(&self, clip: &AnimationClip) -> GltfAnimationData {
        let mut data = GltfAnimationData {
            name: clip.name.clone(),
            channels: Vec::new(),
            samplers: Vec::new(),
        };

        for (i, channel) in clip.channels.iter().enumerate() {
            // Extract times and values
            let times: Vec<f32> = channel.keyframes.iter().map(|k| k.time).collect();
            let values: Vec<f32> = channel.keyframes.iter()
                .flat_map(|k| self.value_to_floats(&k.value))
                .collect();

            // Create sampler
            let sampler = GltfSampler {
                input: i * 2,     // Accessor index for times
                output: i * 2 + 1, // Accessor index for values
                interpolation: match channel.interpolation {
                    Interpolation::Step => "STEP".to_string(),
                    Interpolation::Linear => "LINEAR".to_string(),
                    Interpolation::CubicSpline => "CUBICSPLINE".to_string(),
                },
            };
            data.samplers.push(sampler);

            // Create channel
            let gltf_channel = GltfChannel {
                sampler: i,
                target: GltfTarget {
                    node: channel.target_node,
                    path: match channel.property {
                        AnimationProperty::Translation => "translation".to_string(),
                        AnimationProperty::Rotation => "rotation".to_string(),
                        AnimationProperty::Scale => "scale".to_string(),
                        AnimationProperty::Weights => "weights".to_string(),
                    },
                },
            };
            data.channels.push(GltfChannelWithData {
                channel: gltf_channel,
                times,
                values,
            });
        }

        data
    }

    /// Convert keyframe value to float array
    fn value_to_floats(&self, value: &KeyframeValue) -> Vec<f32> {
        match value {
            KeyframeValue::Translation(v) => vec![v.x, v.y, v.z],
            KeyframeValue::Rotation(q) => vec![q.x, q.y, q.z, q.w],
            KeyframeValue::Scale(v) => vec![v.x, v.y, v.z],
            KeyframeValue::Weights(w) => w.clone(),
        }
    }
}

impl Default for AnimationExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// glTF animation data for export
#[derive(Debug)]
pub struct GltfAnimationData {
    pub name: String,
    pub channels: Vec<GltfChannelWithData>,
    pub samplers: Vec<GltfSampler>,
}

#[derive(Debug)]
pub struct GltfChannelWithData {
    pub channel: GltfChannel,
    pub times: Vec<f32>,
    pub values: Vec<f32>,
}

#[derive(Debug)]
pub struct GltfChannel {
    pub sampler: usize,
    pub target: GltfTarget,
}

#[derive(Debug)]
pub struct GltfTarget {
    pub node: usize,
    pub path: String,
}

#[derive(Debug)]
pub struct GltfSampler {
    pub input: usize,
    pub output: usize,
    pub interpolation: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::{ChordType, EmotionalValence};

    fn create_test_feature(timestamp: f64, pitch: f32, is_beat: bool) -> MusicalFeatures {
        MusicalFeatures {
            timestamp,
            pitch,
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
    fn test_camera_path_generation() {
        let gen = AnimationGenerator::new();

        let features: Vec<_> = (0..100)
            .map(|i| create_test_feature(i as f64 * 0.1, 440.0, i % 4 == 0))
            .collect();

        let clip = gen.generate_camera_path(&features, 10.0);
        assert!(!clip.channels.is_empty());
        assert!(clip.duration > 0.0);
    }

    #[test]
    fn test_beat_animation() {
        let gen = AnimationGenerator::new();

        let features = vec![
            create_test_feature(0.0, 440.0, true),
            create_test_feature(0.5, 440.0, true),
            create_test_feature(1.0, 440.0, true),
        ];

        let clip = gen.generate_beat_animation(0, &features, 1.0);
        assert!(clip.keyframe_count() > 0);
    }

    #[test]
    fn test_animation_sampling() {
        let mut channel = AnimationChannel::new(0, AnimationProperty::Scale);
        channel.add_keyframe(0.0, KeyframeValue::Scale(Vec3::ONE));
        channel.add_keyframe(1.0, KeyframeValue::Scale(Vec3::splat(2.0)));

        let sampled = channel.sample(0.5);
        assert!(sampled.is_some());

        if let Some(KeyframeValue::Scale(v)) = sampled {
            assert!((v.x - 1.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_animation_export() {
        let gen = AnimationGenerator::new();
        let exporter = AnimationExporter::new();

        let features = vec![
            create_test_feature(0.0, 440.0, true),
            create_test_feature(1.0, 880.0, false),
        ];

        let clip = gen.generate_camera_path(&features, 10.0);
        let gltf_data = exporter.to_gltf_data(&clip);

        assert_eq!(gltf_data.name, "CameraPath");
        assert!(!gltf_data.channels.is_empty());
    }
}
