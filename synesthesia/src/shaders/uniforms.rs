//! Shader Uniforms
//!
//! Data structure passed to shaders each frame.

use bytemuck::{Pod, Zeroable};
use crate::music::{MusicUnderstanding, Emotion};
use crate::revelation::ClarityBreakdown;

/// Shader uniform buffer
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ShaderUniforms {
    // Time
    pub time: f32,
    pub delta_time: f32,
    pub resolution: [f32; 2],

    // Music understanding
    pub clarity: f32,
    pub energy: f32,
    pub bass: f32,
    pub mid: f32,
    pub high: f32,
    pub beat: f32,
    pub tempo: f32,
    pub _pad1: f32,

    // Emotion
    pub valence: f32,
    pub arousal: f32,
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub _pad2: [f32; 3],

    // Section
    pub section_type: u32,
    pub section_progress: f32,
    pub is_climax: u32,
    pub _pad3: u32,

    // Effects
    pub bloom_intensity: f32,
    pub chromatic_amount: f32,
    pub vignette_strength: f32,
    pub grain_amount: f32,
}

impl Default for ShaderUniforms {
    fn default() -> Self {
        Self {
            time: 0.0,
            delta_time: 0.016,
            resolution: [1920.0, 1080.0],

            clarity: 0.0,
            energy: 0.5,
            bass: 0.0,
            mid: 0.0,
            high: 0.0,
            beat: 0.0,
            tempo: 120.0,
            _pad1: 0.0,

            valence: 0.0,
            arousal: 0.0,
            hue: 240.0,
            saturation: 0.5,
            lightness: 0.5,
            _pad2: [0.0; 3],

            section_type: 0,
            section_progress: 0.0,
            is_climax: 0,
            _pad3: 0,

            bloom_intensity: 0.3,
            chromatic_amount: 0.01,
            vignette_strength: 0.3,
            grain_amount: 0.05,
        }
    }
}

impl ShaderUniforms {
    /// Update from music understanding
    pub fn update_from_music(&mut self, music: &MusicUnderstanding) {
        // Signal features
        self.energy = music.signal.rms;
        self.bass = music.signal.bass;
        self.mid = music.signal.mid;
        self.high = music.signal.high;
        self.beat = music.signal.beat_strength;
        self.tempo = music.theory.tempo;

        // Emotion
        self.valence = music.emotion.valence;
        self.arousal = music.emotion.arousal;

        // Map emotion to color
        let (h, s, l) = Self::emotion_to_hsl(&music.emotion.primary);
        self.hue = h;
        self.saturation = s * music.emotion.intensity;
        self.lightness = l;

        // Section
        self.section_type = music.section.section_type as u32;
        self.section_progress = music.section_progress;
        self.is_climax = if music.is_climax { 1 } else { 0 };

        // Dynamic effects based on music
        self.bloom_intensity = 0.2 + music.signal.beat_strength * 0.4;
        self.chromatic_amount = 0.005 + music.signal.rms * 0.02;
        self.vignette_strength = 0.25 + music.signal.beat_strength * 0.15;
        self.grain_amount = 0.03 * (1.0 - music.theory.key_confidence);
    }

    /// Update clarity from revelation engine
    pub fn update_clarity(&mut self, clarity: f32, _breakdown: &ClarityBreakdown) {
        self.clarity = clarity;

        // Adjust effects based on clarity
        self.grain_amount = 0.08 * (1.0 - clarity);
    }

    /// Update time
    pub fn update_time(&mut self, time: f32, delta: f32) {
        self.time = time;
        self.delta_time = delta;
    }

    /// Update resolution
    pub fn set_resolution(&mut self, width: f32, height: f32) {
        self.resolution = [width, height];
    }

    /// Map emotion to HSL color
    fn emotion_to_hsl(emotion: &Emotion) -> (f32, f32, f32) {
        match emotion {
            // High arousal, positive
            Emotion::Joy => (45.0, 0.85, 0.6),
            Emotion::Triumph => (30.0, 0.8, 0.55),
            Emotion::Excitement => (15.0, 0.85, 0.55),
            Emotion::Euphoria => (300.0, 0.9, 0.6),

            // High arousal, negative
            Emotion::Anger => (0.0, 0.9, 0.4),
            Emotion::Intensity => (350.0, 0.85, 0.45),
            Emotion::Urgency => (20.0, 0.8, 0.5),
            Emotion::Chaos => (280.0, 0.85, 0.45),

            // Low arousal, positive
            Emotion::Peace => (180.0, 0.5, 0.6),
            Emotion::Tenderness => (330.0, 0.4, 0.65),
            Emotion::Hope => (60.0, 0.6, 0.6),
            Emotion::Nostalgia => (35.0, 0.4, 0.5),

            // Low arousal, negative
            Emotion::Sadness => (220.0, 0.6, 0.35),
            Emotion::Melancholy => (250.0, 0.5, 0.4),
            Emotion::Tension => (270.0, 0.6, 0.35),
            Emotion::Dread => (260.0, 0.7, 0.25),

            Emotion::Neutral => (200.0, 0.3, 0.5),
        }
    }
}
