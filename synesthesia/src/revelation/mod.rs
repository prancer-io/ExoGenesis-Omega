//! Revelation Engine
//!
//! Implements the "Painter's Algorithm" - progressive semantic clarity
//! where meaning emerges gradually like a painting coming to life.

mod accumulator;
mod clarity;
mod generator;

pub use accumulator::SemanticAccumulator;
pub use clarity::{ClarityLevel, ClarityCalculator};
pub use generator::{RevelationGenerator, GenerationParams, VideoFrame};

use crate::audio::{AudioFeatures, TranscribedWord};
use crate::ai::SemanticScene;

/// The Revelation Engine - core system for progressive visual emergence
pub struct RevelationEngine {
    /// Semantic context accumulator
    accumulator: SemanticAccumulator,

    /// Clarity calculator
    clarity_calc: ClarityCalculator,

    /// Video frame generator
    generator: RevelationGenerator,

    /// Current clarity level (0.0 = abstract, 1.0 = revealed)
    pub clarity: f32,

    /// Generation parameters
    params: GenerationParams,

    /// Time since song start
    elapsed_time: f64,
}

impl RevelationEngine {
    /// Create new revelation engine
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            accumulator: SemanticAccumulator::new(),
            clarity_calc: ClarityCalculator::new(),
            generator: RevelationGenerator::new()?,
            clarity: 0.0,
            params: GenerationParams::default(),
            elapsed_time: 0.0,
        })
    }

    /// Process new transcribed word
    pub fn process_word(&mut self, word: TranscribedWord) {
        self.accumulator.add_word(word);
        self.update_clarity();
    }

    /// Process semantic scene from AI
    pub fn process_scene(&mut self, scene: SemanticScene) {
        self.accumulator.add_scene(scene);
        self.update_clarity();
    }

    /// Update the engine state
    pub fn update(&mut self, delta: f64, audio: &AudioFeatures) {
        self.elapsed_time += delta;

        // Natural clarity increase over time (even without lyrics)
        let time_clarity = (self.elapsed_time / 120.0).min(0.3) as f32;
        self.clarity = (self.clarity + time_clarity * delta as f32 * 0.1).min(1.0);

        // Audio energy can reveal more
        let energy_boost = audio.rms * 0.01 * delta as f32;
        self.clarity = (self.clarity + energy_boost).min(1.0);

        // Update generation parameters based on clarity
        self.params = self.calculate_generation_params();
    }

    /// Calculate generation parameters based on current clarity
    fn calculate_generation_params(&self) -> GenerationParams {
        GenerationParams {
            noise_level: 1.0 - self.clarity,
            guidance_scale: 3.0 + (self.clarity * 12.0),
            num_inference_steps: 10 + (self.clarity * 40.0) as usize,
            fps: 24,
            num_frames: 6,
        }
    }

    /// Update clarity based on accumulated context
    fn update_clarity(&mut self) {
        let semantic_clarity = self.clarity_calc.calculate(&self.accumulator);
        // Blend with current clarity for smooth transition
        self.clarity = self.clarity * 0.9 + semantic_clarity * 0.1;
    }

    /// Get the current clarity level enum
    pub fn clarity_level(&self) -> ClarityLevel {
        ClarityLevel::from_value(self.clarity)
    }

    /// Generate prompt for current state
    pub fn generate_prompt(&self, audio: &AudioFeatures) -> String {
        match self.clarity_level() {
            ClarityLevel::Abstract => self.abstract_prompt(audio),
            ClarityLevel::Emerging => self.emerging_prompt(audio),
            ClarityLevel::Forming => self.forming_prompt(audio),
            ClarityLevel::Clarifying => self.clarifying_prompt(audio),
            ClarityLevel::Revealed => self.revealed_prompt(audio),
        }
    }

    /// Abstract prompt - pure audio reaction
    fn abstract_prompt(&self, audio: &AudioFeatures) -> String {
        let energy = if audio.rms > 0.6 { "explosive" }
                     else if audio.rms > 0.3 { "flowing" }
                     else { "gentle" };

        let rhythm = if audio.is_beat { "pulsating" } else { "drifting" };

        format!(
            "abstract flowing colors, {} energy, {} rhythm, \
             dreamlike particles, cosmic nebula, no definite shapes, \
             pure sensation, synesthetic experience",
            energy, rhythm
        )
    }

    /// Emerging prompt - hints of forms
    fn emerging_prompt(&self, audio: &AudioFeatures) -> String {
        let mood = self.accumulator.primary_mood().unwrap_or("mysterious".to_string());
        let hints = self.accumulator.theme_hints().join(", ");

        format!(
            "abstract forms becoming shapes, {} atmosphere, \
             hints of {}, emerging from primordial mist, \
             dreamlike, ethereal, soft edges dissolving, \
             intensity: {:.0}%",
            mood, hints, audio.rms * 100.0
        )
    }

    /// Forming prompt - shapes visible
    fn forming_prompt(&self, audio: &AudioFeatures) -> String {
        let setting = self.accumulator.detected_setting()
            .unwrap_or("ethereal landscape".to_string());
        let mood = self.accumulator.primary_mood()
            .unwrap_or("contemplative".to_string());

        format!(
            "{} taking shape, {} mood, \
             forms solidifying from abstract, semi-visible details, \
             cinematic lighting breaking through, \
             beat intensity: {:.0}%",
            setting, mood, audio.beat_intensity * 100.0
        )
    }

    /// Clarifying prompt - scene forming
    fn clarifying_prompt(&self, audio: &AudioFeatures) -> String {
        let scene = self.accumulator.synthesize_scene();

        format!(
            "{} in {}, {} time, {} atmosphere, \
             scene becoming clear, details emerging, \
             {} action, cinematic quality, \
             emotional intensity: {:.0}%",
            scene.subjects,
            scene.location,
            scene.time_of_day,
            scene.mood,
            scene.action,
            audio.rms * 100.0
        )
    }

    /// Revealed prompt - full visual fidelity
    fn revealed_prompt(&self, audio: &AudioFeatures) -> String {
        let narrative = self.accumulator.generate_narrative();

        format!(
            "cinematic masterpiece: {} {} {}. \
             Style: {}, Lighting: {}, Camera: {}, \
             ultra detailed, photorealistic, \
             emotional peak: {:.0}%",
            narrative.setting,
            narrative.action,
            narrative.emotional_context,
            narrative.visual_style,
            narrative.lighting,
            narrative.camera,
            audio.beat_intensity * 100.0
        )
    }

    /// Get current generation parameters
    pub fn params(&self) -> &GenerationParams {
        &self.params
    }

    /// Reset for new song
    pub fn reset(&mut self) {
        self.accumulator = SemanticAccumulator::new();
        self.clarity = 0.0;
        self.elapsed_time = 0.0;
        self.params = GenerationParams::default();
    }
}

impl Default for RevelationEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create revelation engine")
    }
}
