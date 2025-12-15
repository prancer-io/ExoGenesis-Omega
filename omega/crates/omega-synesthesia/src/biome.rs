//! Biome System for Synesthesia Worlds
//!
//! Creates distinct regions based on musical characteristics.

use crate::features::{MusicalFeatures, EmotionalValence};
use crate::genre::GenreStyle;
use glam::Vec3;

/// A biome represents a distinct region in the synesthesia world
#[derive(Debug, Clone)]
pub struct Biome {
    /// Biome type
    pub biome_type: BiomeType,
    /// Center position
    pub center: Vec3,
    /// Approximate radius
    pub radius: f32,
    /// Dominant emotion
    pub emotion: EmotionalValence,
    /// Visual density (0-1)
    pub density: f32,
    /// Height variation multiplier
    pub height_scale: f32,
    /// Color tint
    pub color_tint: [f32; 3],
    /// Fog density multiplier
    pub fog_multiplier: f32,
    /// Particle intensity
    pub particle_intensity: f32,
}

/// Types of biomes based on musical characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiomeType {
    /// High energy, major key - vibrant colors, tall structures
    Euphoria,
    /// Low energy, minor key - muted colors, flowing shapes
    Melancholy,
    /// High energy, dissonant - sharp angles, red tints
    Tension,
    /// Low energy, consonant - soft shapes, calm colors
    Serenity,
    /// Rhythmic focus - geometric patterns, pulsing elements
    Rhythmic,
    /// Melodic focus - flowing paths, harmonic colors
    Melodic,
    /// Chaotic/complex - fragmented shapes, varied textures
    Chaos,
    /// Simple/minimal - clean lines, sparse elements
    Minimal,
    /// Transition zone between biomes
    Transition,
}

impl BiomeType {
    /// Get base color for this biome type
    pub fn base_color(&self) -> [f32; 3] {
        match self {
            Self::Euphoria => [1.0, 0.85, 0.3],    // Golden
            Self::Melancholy => [0.3, 0.4, 0.7],   // Blue-gray
            Self::Tension => [0.8, 0.2, 0.2],      // Red
            Self::Serenity => [0.4, 0.8, 0.6],     // Soft green
            Self::Rhythmic => [0.9, 0.5, 0.1],     // Orange
            Self::Melodic => [0.6, 0.4, 0.9],      // Purple
            Self::Chaos => [0.7, 0.3, 0.5],        // Magenta
            Self::Minimal => [0.9, 0.9, 0.95],     // Near white
            Self::Transition => [0.6, 0.6, 0.6],   // Gray
        }
    }

    /// Get fog density for this biome
    pub fn fog_density(&self) -> f32 {
        match self {
            Self::Euphoria => 0.01,
            Self::Melancholy => 0.08,
            Self::Tension => 0.03,
            Self::Serenity => 0.05,
            Self::Rhythmic => 0.02,
            Self::Melodic => 0.04,
            Self::Chaos => 0.06,
            Self::Minimal => 0.005,
            Self::Transition => 0.04,
        }
    }

    /// Get element density for this biome
    pub fn element_density(&self) -> f32 {
        match self {
            Self::Euphoria => 1.2,
            Self::Melancholy => 0.8,
            Self::Tension => 1.5,
            Self::Serenity => 0.6,
            Self::Rhythmic => 1.0,
            Self::Melodic => 0.9,
            Self::Chaos => 1.8,
            Self::Minimal => 0.3,
            Self::Transition => 0.7,
        }
    }
}

/// Biome generator that creates regions from musical features
pub struct BiomeGenerator {
    /// Minimum region duration (seconds)
    pub min_region_duration: f32,
    /// Smoothing window size
    pub smoothing_window: usize,
    /// Transition blend distance
    pub transition_distance: f32,
}

impl BiomeGenerator {
    pub fn new() -> Self {
        Self {
            min_region_duration: 5.0,
            smoothing_window: 50,
            transition_distance: 10.0,
        }
    }

    /// Generate biomes from musical features
    pub fn generate(&self, features: &[MusicalFeatures], style: &GenreStyle) -> Vec<Biome> {
        if features.is_empty() {
            return Vec::new();
        }

        let mut biomes = Vec::new();
        let mut current_biome_start = 0;
        let mut current_type = self.classify_region(&features[0..1.min(features.len())]);

        // Smooth features for region detection
        let smoothed_features = self.smooth_features(features);

        // Detect biome boundaries
        for (i, feature) in smoothed_features.iter().enumerate() {
            let new_type = self.classify_single(feature);

            // Check if we should start a new biome
            let duration = feature.timestamp - features[current_biome_start].timestamp;

            if new_type != current_type && duration >= self.min_region_duration as f64 {
                // Create biome for previous region
                let biome = self.create_biome(
                    &features[current_biome_start..i],
                    current_type,
                    style,
                );
                biomes.push(biome);

                current_biome_start = i;
                current_type = new_type;
            }
        }

        // Create final biome
        if current_biome_start < features.len() {
            let biome = self.create_biome(
                &features[current_biome_start..],
                current_type,
                style,
            );
            biomes.push(biome);
        }

        // Add transition zones
        self.add_transitions(&mut biomes);

        biomes
    }

    /// Smooth features for more stable region detection
    fn smooth_features(&self, features: &[MusicalFeatures]) -> Vec<MusicalFeatures> {
        if features.len() <= self.smoothing_window {
            return features.to_vec();
        }

        let half_window = self.smoothing_window / 2;
        let mut smoothed = Vec::with_capacity(features.len());

        for i in 0..features.len() {
            let start = i.saturating_sub(half_window);
            let end = (i + half_window + 1).min(features.len());
            let window = &features[start..end];

            let mut avg = features[i].clone();
            avg.arousal = window.iter().map(|f| f.arousal).sum::<f32>() / window.len() as f32;
            avg.valence = window.iter().map(|f| f.valence).sum::<f32>() / window.len() as f32;
            avg.loudness = window.iter().map(|f| f.loudness).sum::<f32>() / window.len() as f32;
            avg.brightness = window.iter().map(|f| f.brightness).sum::<f32>() / window.len() as f32;
            avg.tension = window.iter().map(|f| f.tension).sum::<f32>() / window.len() as f32;

            smoothed.push(avg);
        }

        smoothed
    }

    /// Classify a region of features into a biome type
    fn classify_region(&self, features: &[MusicalFeatures]) -> BiomeType {
        if features.is_empty() {
            return BiomeType::Minimal;
        }

        // Average the key metrics
        let avg_arousal = features.iter().map(|f| f.arousal).sum::<f32>() / features.len() as f32;
        let avg_valence = features.iter().map(|f| f.valence).sum::<f32>() / features.len() as f32;
        let avg_brightness = features.iter().map(|f| f.brightness).sum::<f32>() / features.len() as f32;
        let avg_tension = features.iter().map(|f| f.tension).sum::<f32>() / features.len() as f32;
        let avg_onset = features.iter().map(|f| f.onset_strength).sum::<f32>() / features.len() as f32;

        self.classify_metrics(avg_arousal, avg_valence, avg_brightness, avg_tension, avg_onset)
    }

    /// Classify a single feature
    fn classify_single(&self, feature: &MusicalFeatures) -> BiomeType {
        self.classify_metrics(
            feature.arousal,
            feature.valence,
            feature.brightness,
            feature.tension,
            feature.onset_strength,
        )
    }

    /// Classify based on metrics
    fn classify_metrics(
        &self,
        arousal: f32,
        valence: f32,
        brightness: f32,
        tension: f32,
        onset_strength: f32,
    ) -> BiomeType {
        // High arousal, positive valence = Euphoria
        if arousal > 0.7 && valence > 0.3 {
            return BiomeType::Euphoria;
        }

        // Low arousal, negative valence = Melancholy
        if arousal < 0.4 && valence < -0.2 {
            return BiomeType::Melancholy;
        }

        // High tension = Tension biome
        if tension > 0.7 {
            return BiomeType::Tension;
        }

        // Low arousal, positive valence = Serenity
        if arousal < 0.4 && valence > 0.2 {
            return BiomeType::Serenity;
        }

        // Strong rhythm = Rhythmic
        if onset_strength > 0.6 {
            return BiomeType::Rhythmic;
        }

        // Bright, moderate energy = Melodic
        if brightness > 0.6 && arousal > 0.4 && arousal < 0.7 {
            return BiomeType::Melodic;
        }

        // High variation/complexity = Chaos
        if brightness > 0.5 && tension > 0.5 && arousal > 0.5 {
            return BiomeType::Chaos;
        }

        // Low everything = Minimal
        if arousal < 0.3 && brightness < 0.4 {
            return BiomeType::Minimal;
        }

        // Default
        BiomeType::Melodic
    }

    /// Create a biome from a region of features
    fn create_biome(
        &self,
        features: &[MusicalFeatures],
        biome_type: BiomeType,
        style: &GenreStyle,
    ) -> Biome {
        if features.is_empty() {
            return Biome {
                biome_type,
                center: Vec3::ZERO,
                radius: 10.0,
                emotion: EmotionalValence::Neutral,
                density: 0.5,
                height_scale: 1.0,
                color_tint: biome_type.base_color(),
                fog_multiplier: 1.0,
                particle_intensity: 0.5,
            };
        }

        // Calculate center position (time-based X, average pitch Y)
        let avg_timestamp = features.iter().map(|f| f.timestamp).sum::<f64>() / features.len() as f64;
        let avg_pitch = features.iter().map(|f| f.pitch).sum::<f32>() / features.len() as f32;
        let avg_loudness = features.iter().map(|f| f.loudness).sum::<f32>() / features.len() as f32;

        let center = Vec3::new(
            avg_timestamp as f32 * style.time_scale,
            (avg_pitch / 20000.0) * style.vertical_scale,
            0.0,
        );

        // Calculate radius based on duration
        let duration = features.last().unwrap().timestamp - features.first().unwrap().timestamp;
        let radius = (duration as f32 * style.time_scale / 2.0).max(10.0);

        // Get dominant emotion
        let emotions: Vec<_> = features.iter().map(|f| f.emotion).collect();
        let emotion = Self::mode_emotion(&emotions);

        // Calculate density based on loudness and onset strength
        let avg_onset = features.iter().map(|f| f.onset_strength).sum::<f32>() / features.len() as f32;
        let density = (avg_loudness * 0.5 + avg_onset * 0.5).clamp(0.1, 1.0);

        // Height scale based on pitch range
        let min_pitch = features.iter().map(|f| f.pitch).fold(f32::MAX, f32::min);
        let max_pitch = features.iter().map(|f| f.pitch).fold(f32::MIN, f32::max);
        let height_scale = ((max_pitch - min_pitch) / 500.0).clamp(0.5, 2.0);

        // Color tint blends biome color with emotion color
        let biome_color = biome_type.base_color();
        let emotion_color = emotion.color();
        let color_tint = [
            biome_color[0] * 0.6 + emotion_color[0] * 0.4,
            biome_color[1] * 0.6 + emotion_color[1] * 0.4,
            biome_color[2] * 0.6 + emotion_color[2] * 0.4,
        ];

        // Fog and particles
        let avg_brightness = features.iter().map(|f| f.brightness).sum::<f32>() / features.len() as f32;
        let fog_multiplier = 1.0 + (1.0 - avg_brightness) * 0.5;
        let particle_intensity = avg_onset * density;

        Biome {
            biome_type,
            center,
            radius,
            emotion,
            density,
            height_scale,
            color_tint,
            fog_multiplier,
            particle_intensity,
        }
    }

    /// Add transition zones between biomes
    fn add_transitions(&self, biomes: &mut Vec<Biome>) {
        if biomes.len() < 2 {
            return;
        }

        let mut transitions = Vec::new();

        for i in 0..biomes.len() - 1 {
            let b1 = &biomes[i];
            let b2 = &biomes[i + 1];

            // Create transition biome between them
            let transition_center = (b1.center + b2.center) * 0.5;
            let transition = Biome {
                biome_type: BiomeType::Transition,
                center: transition_center,
                radius: self.transition_distance,
                emotion: EmotionalValence::Neutral,
                density: (b1.density + b2.density) * 0.5,
                height_scale: (b1.height_scale + b2.height_scale) * 0.5,
                color_tint: [
                    (b1.color_tint[0] + b2.color_tint[0]) * 0.5,
                    (b1.color_tint[1] + b2.color_tint[1]) * 0.5,
                    (b1.color_tint[2] + b2.color_tint[2]) * 0.5,
                ],
                fog_multiplier: (b1.fog_multiplier + b2.fog_multiplier) * 0.5,
                particle_intensity: (b1.particle_intensity + b2.particle_intensity) * 0.5,
            };

            transitions.push(transition);
        }

        // Insert transitions
        for (i, transition) in transitions.into_iter().enumerate() {
            biomes.insert(i * 2 + 1, transition);
        }
    }

    /// Get the most common emotion
    fn mode_emotion(emotions: &[EmotionalValence]) -> EmotionalValence {
        use std::collections::HashMap;
        let mut counts: HashMap<u8, usize> = HashMap::new();

        for e in emotions {
            let key = match e {
                EmotionalValence::Joy => 0,
                EmotionalValence::Sadness => 1,
                EmotionalValence::Anger => 2,
                EmotionalValence::Peace => 3,
                EmotionalValence::Fear => 4,
                EmotionalValence::Surprise => 5,
                EmotionalValence::Neutral => 6,
            };
            *counts.entry(key).or_default() += 1;
        }

        let (max_key, _) = counts.into_iter().max_by_key(|(_, v)| *v).unwrap_or((6, 0));
        match max_key {
            0 => EmotionalValence::Joy,
            1 => EmotionalValence::Sadness,
            2 => EmotionalValence::Anger,
            3 => EmotionalValence::Peace,
            4 => EmotionalValence::Fear,
            5 => EmotionalValence::Surprise,
            _ => EmotionalValence::Neutral,
        }
    }

    /// Get biome at position
    pub fn get_biome_at<'a>(&self, biomes: &'a [Biome], position: Vec3) -> Option<&'a Biome> {
        for biome in biomes {
            let dist = (position - biome.center).length();
            if dist < biome.radius {
                return Some(biome);
            }
        }

        // Find closest biome
        biomes.iter()
            .min_by(|a, b| {
                let dist_a = (position - a.center).length();
                let dist_b = (position - b.center).length();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }

    /// Get blend factor between biomes at position
    pub fn get_biome_blend(&self, biomes: &[Biome], position: Vec3) -> Vec<(usize, f32)> {
        let mut blends = Vec::new();
        let mut total_weight = 0.0f32;

        for (i, biome) in biomes.iter().enumerate() {
            let dist = (position - biome.center).length();
            let influence = 1.0 / (dist + 1.0);
            blends.push((i, influence));
            total_weight += influence;
        }

        // Normalize weights
        for (_, weight) in &mut blends {
            *weight /= total_weight;
        }

        // Sort by weight descending and keep top 3
        blends.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        blends.truncate(3);

        blends
    }
}

impl Default for BiomeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::ChordType;

    fn create_test_feature(timestamp: f64, arousal: f32, valence: f32) -> MusicalFeatures {
        MusicalFeatures {
            timestamp,
            pitch: 440.0,
            midi_note: 69,
            pitch_class: 9,
            octave: 4,
            onset_strength: 0.5,
            is_beat: false,
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
            valence,
            arousal,
            emotion: EmotionalValence::Neutral,
            mfcc: [0.0; 13],
            chroma: [0.0; 12],
        }
    }

    #[test]
    fn test_biome_classification() {
        let gen = BiomeGenerator::new();

        // High arousal, positive valence = Euphoria
        let euphoria = vec![create_test_feature(0.0, 0.9, 0.6)];
        let biome_type = gen.classify_region(&euphoria);
        assert_eq!(biome_type, BiomeType::Euphoria);

        // Low arousal, negative valence = Melancholy
        let melancholy = vec![create_test_feature(0.0, 0.2, -0.5)];
        let biome_type = gen.classify_region(&melancholy);
        assert_eq!(biome_type, BiomeType::Melancholy);
    }

    #[test]
    fn test_biome_generation() {
        let gen = BiomeGenerator::new();
        let style = crate::genre::Genre::Classical.get_style();

        let features: Vec<_> = (0..100)
            .map(|i| create_test_feature(i as f64 * 0.1, 0.5, 0.0))
            .collect();

        let biomes = gen.generate(&features, &style);
        assert!(!biomes.is_empty());
    }
}
