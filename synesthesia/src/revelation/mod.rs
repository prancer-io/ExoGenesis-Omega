//! Revelation Engine
//!
//! Implements the "Painter's Algorithm" - progressive visual emergence
//! where meaning emerges gradually like a painting coming to life.
//!
//! ## Music-Driven Revelation
//!
//! The visualization clarity is driven by MUSICAL understanding, not just lyrics:
//! - Key detection → Color palette confidence
//! - Structure detection → Scene understanding
//! - Pattern repetition → Familiarity / revelation
//! - Climax proximity → Full revelation moments
//! - Emotional arc → Narrative coherence

mod accumulator;
mod clarity;
mod generator;

pub use accumulator::SemanticAccumulator;
pub use clarity::{ClarityLevel, ClarityCalculator};
pub use generator::{RevelationGenerator, GenerationParams, VideoFrame};

use crate::music::{MusicUnderstanding, NarrativeBeat, SectionType, Emotion, EmotionState};
use crate::audio::TranscribedWord;
use crate::ai::SemanticScene;

/// The Revelation Engine - core system for progressive visual emergence
/// Now driven primarily by MUSIC, with lyrics as optional enhancement
pub struct RevelationEngine {
    /// Semantic context accumulator (for lyrics, when available)
    accumulator: SemanticAccumulator,

    /// Video frame generator
    generator: RevelationGenerator,

    /// Current clarity level (0.0 = abstract, 1.0 = revealed)
    pub clarity: f32,

    /// Music-driven clarity components
    pub key_clarity: f32,
    pub structure_clarity: f32,
    pub pattern_clarity: f32,
    pub climax_clarity: f32,

    /// Generation parameters
    params: GenerationParams,

    /// Time since song start
    elapsed_time: f64,

    /// Has the first chorus/drop happened?
    first_peak_reached: bool,

    /// Smoothing factor for clarity transitions
    smoothing: f32,
}

impl RevelationEngine {
    /// Create new revelation engine
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            accumulator: SemanticAccumulator::new(),
            generator: RevelationGenerator::new()?,
            clarity: 0.0,
            key_clarity: 0.0,
            structure_clarity: 0.0,
            pattern_clarity: 0.0,
            climax_clarity: 0.0,
            params: GenerationParams::default(),
            elapsed_time: 0.0,
            first_peak_reached: false,
            smoothing: 0.05,
        })
    }

    /// Process new transcribed word (optional lyrics enhancement)
    pub fn process_word(&mut self, word: TranscribedWord) {
        self.accumulator.add_word(word);
        // Lyrics add a small boost to clarity
        self.clarity = (self.clarity + 0.01).min(1.0);
    }

    /// Process semantic scene from AI
    pub fn process_scene(&mut self, scene: SemanticScene) {
        self.accumulator.add_scene(scene);
    }

    /// Update the engine state based on MUSIC understanding
    pub fn update(&mut self, delta: f64, music: &MusicUnderstanding) {
        self.elapsed_time += delta;

        // ─────────────────────────────────────────────────────────────
        // MUSIC-DRIVEN CLARITY CALCULATION
        // ─────────────────────────────────────────────────────────────

        // 1. Key Detection Clarity (do we know the musical key?)
        let target_key = if music.theory.key_confidence > 0.7 {
            0.3  // Key detected = 30% clarity
        } else if music.theory.key_confidence > 0.4 {
            0.15
        } else {
            0.05
        };
        self.key_clarity = self.smooth(self.key_clarity, target_key);

        // 2. Structure Clarity (do we know where we are in the song?)
        let target_structure = match music.section.section_type {
            SectionType::Unknown => 0.0,
            SectionType::Intro => 0.1,
            SectionType::Verse => 0.2,
            SectionType::PreChorus => 0.25,
            SectionType::Chorus => 0.4,
            SectionType::Drop => 0.45,
            SectionType::Bridge => 0.3,
            SectionType::Breakdown => 0.2,
            SectionType::Buildup => 0.35,
            SectionType::Outro => 0.3,
            SectionType::Instrumental => 0.25,
        };
        self.structure_clarity = self.smooth(self.structure_clarity, target_structure);

        // 3. Pattern Clarity (have we heard this before?)
        let target_pattern = match music.repetition_count {
            0 => 0.0,
            1 => 0.1,  // First time = some recognition
            2 => 0.2,  // Second time = familiarity
            _ => 0.25, // More = strong familiarity
        };
        self.pattern_clarity = self.smooth(self.pattern_clarity, target_pattern);

        // 4. Climax Clarity (are we at/near a peak moment?)
        let target_climax = if music.is_climax {
            self.first_peak_reached = true;
            1.0  // FULL REVELATION at climax
        } else if music.climax_distance.abs() < 2.0 {
            0.8  // Near climax
        } else if music.climax_distance.abs() < 5.0 {
            0.5  // Approaching
        } else if self.first_peak_reached {
            0.3  // After first peak, maintain some revelation
        } else {
            0.0
        };
        self.climax_clarity = self.smooth(self.climax_clarity, target_climax);

        // ─────────────────────────────────────────────────────────────
        // COMBINE ALL CLARITY SOURCES
        // ─────────────────────────────────────────────────────────────

        let music_clarity = self.key_clarity
            + self.structure_clarity
            + self.pattern_clarity
            + self.climax_clarity.max(0.0);

        // Energy arc position adds baseline clarity as song progresses
        let arc_clarity = music.energy_arc_position * 0.15;

        // Narrative beat modulates clarity
        let narrative_mod = match music.narrative_beat {
            NarrativeBeat::Establishment => 0.0,
            NarrativeBeat::RisingAction => 0.1,
            NarrativeBeat::Climax => 0.3,
            NarrativeBeat::FallingAction => 0.05,
            NarrativeBeat::Resolution => 0.15,
        };

        // Final clarity (clamped to 0-1)
        let target_clarity = (music_clarity + arc_clarity + narrative_mod).clamp(0.0, 1.0);

        // Smooth transition to target
        self.clarity = self.smooth(self.clarity, target_clarity);

        // Update generation parameters based on clarity
        self.params = self.calculate_generation_params();
    }

    /// Smooth value transition
    fn smooth(&self, current: f32, target: f32) -> f32 {
        current + (target - current) * self.smoothing
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

    /// Get the current clarity level enum
    pub fn clarity_level(&self) -> ClarityLevel {
        ClarityLevel::from_value(self.clarity)
    }

    /// Generate prompt based on MUSIC understanding
    pub fn generate_prompt(&self, music: &MusicUnderstanding) -> String {
        match self.clarity_level() {
            ClarityLevel::Abstract => self.abstract_prompt(music),
            ClarityLevel::Emerging => self.emerging_prompt(music),
            ClarityLevel::Forming => self.forming_prompt(music),
            ClarityLevel::Clarifying => self.clarifying_prompt(music),
            ClarityLevel::Revealed => self.revealed_prompt(music),
        }
    }

    /// Abstract prompt - pure musical sensation
    fn abstract_prompt(&self, music: &MusicUnderstanding) -> String {
        let energy = match music.signal.rms {
            e if e > 0.7 => "explosive",
            e if e > 0.4 => "flowing",
            e if e > 0.2 => "gentle",
            _ => "whisper-quiet",
        };

        let texture = if music.theory.timbre_brightness > 0.6 {
            "shimmering, luminous"
        } else if music.theory.timbre_roughness > 0.5 {
            "gritty, textured"
        } else {
            "smooth, ethereal"
        };

        let motion = if music.signal.is_beat { "pulsating" } else { "drifting" };

        format!(
            "abstract {} colors, {} energy, {} rhythm, \
             no definite shapes, pure synesthetic sensation, \
             frequency: {} Hz centroid",
            texture, energy, motion,
            music.signal.spectral_centroid as u32
        )
    }

    /// Emerging prompt - mood becoming apparent
    fn emerging_prompt(&self, music: &MusicUnderstanding) -> String {
        let mood = self.emotion_to_atmosphere(&music.emotion);
        let mode_feel = if music.theory.mode.is_bright() {
            "warm light emerging"
        } else {
            "deep shadows forming"
        };

        let tempo_feel = match music.theory.tempo {
            t if t > 140.0 => "rapid motion",
            t if t > 100.0 => "steady pulse",
            t if t > 70.0 => "gentle sway",
            _ => "slow drift",
        };

        format!(
            "forms emerging from void, {} atmosphere, {}, \
             {}, dreamlike, edges undefined, \
             {} BPM heartbeat",
            mood, mode_feel, tempo_feel,
            music.theory.tempo as u32
        )
    }

    /// Forming prompt - structure becoming visible
    fn forming_prompt(&self, music: &MusicUnderstanding) -> String {
        let section_desc = match music.section.section_type {
            SectionType::Verse => "narrative unfolding",
            SectionType::Chorus => "themes crystallizing",
            SectionType::Bridge => "perspective shifting",
            SectionType::Buildup => "tension mounting",
            SectionType::Drop => "power unleashed",
            SectionType::Breakdown => "reflection moment",
            _ => "scene forming",
        };

        let key_palette = format!("{} {:?}",
            music.theory.key.name(),
            music.theory.mode
        );

        format!(
            "{}, {} palette, \
             shapes solidifying, {} section, \
             {} progress through moment, \
             tension level: {:.0}%",
            section_desc,
            key_palette,
            music.section.section_type.description(),
            (music.section_progress * 100.0) as u32,
            music.theory.chord_tension * 100.0
        )
    }

    /// Clarifying prompt - full scene emerging
    fn clarifying_prompt(&self, music: &MusicUnderstanding) -> String {
        let emotion = &music.emotion;
        let narrative = match music.narrative_beat {
            NarrativeBeat::Establishment => "world establishing",
            NarrativeBeat::RisingAction => "intensity building",
            NarrativeBeat::Climax => "peak moment",
            NarrativeBeat::FallingAction => "descent from heights",
            NarrativeBeat::Resolution => "finding peace",
        };

        let lighting = if emotion.lightness() > 0.5 {
            "bright, hopeful lighting"
        } else {
            "dramatic shadows, moody"
        };

        format!(
            "cinematic scene: {}, {}, \
             {:?} emotion at {:.0}% intensity, \
             {}, camera reveals details, \
             {} energy arc",
            narrative, lighting,
            emotion.primary, emotion.intensity * 100.0,
            music.section.section_type.description(),
            (music.energy_arc_position * 100.0) as u32
        )
    }

    /// Revealed prompt - full visual fidelity
    fn revealed_prompt(&self, music: &MusicUnderstanding) -> String {
        let emotion = &music.emotion;

        // Build rich scene description from all musical data
        let setting = self.emotion_to_setting(emotion);
        let action = match music.narrative_beat {
            NarrativeBeat::Climax => "explosive transformation",
            NarrativeBeat::RisingAction => "ascending motion",
            NarrativeBeat::FallingAction => "graceful descent",
            NarrativeBeat::Resolution => "peaceful settling",
            NarrativeBeat::Establishment => "grand reveal",
        };

        let style = match emotion.primary {
            Emotion::Euphoria | Emotion::Joy => "vibrant, maximalist",
            Emotion::Sadness | Emotion::Melancholy => "muted, painterly",
            Emotion::Anger | Emotion::Intensity => "sharp, high contrast",
            Emotion::Peace | Emotion::Tenderness => "soft, dreamy",
            Emotion::Tension | Emotion::Dread => "distorted, unsettling",
            _ => "cinematic, atmospheric",
        };

        let camera = if music.is_climax {
            "epic wide shot with dynamic movement"
        } else if music.signal.is_beat {
            "beat-synced camera pulse"
        } else {
            "smooth contemplative pan"
        };

        format!(
            "masterpiece: {}, {}, \
             style: {}, camera: {}, \
             full emotional revelation at {} key, \
             ultra detailed, maximum clarity",
            setting, action,
            style, camera,
            music.theory.key.name()
        )
    }

    /// Convert emotion to atmospheric description
    fn emotion_to_atmosphere(&self, emotion: &EmotionState) -> &'static str {
        match emotion.primary {
            Emotion::Joy => "jubilant, radiant",
            Emotion::Triumph => "victorious, powerful",
            Emotion::Excitement => "electric, energetic",
            Emotion::Euphoria => "transcendent, ecstatic",
            Emotion::Anger => "fierce, turbulent",
            Emotion::Intensity => "focused, driven",
            Emotion::Urgency => "pressing, immediate",
            Emotion::Chaos => "wild, unpredictable",
            Emotion::Peace => "serene, tranquil",
            Emotion::Tenderness => "gentle, intimate",
            Emotion::Hope => "uplifting, dawning",
            Emotion::Nostalgia => "wistful, golden",
            Emotion::Sadness => "melancholic, blue",
            Emotion::Melancholy => "pensive, grey",
            Emotion::Tension => "uneasy, suspended",
            Emotion::Dread => "ominous, dark",
            Emotion::Neutral => "balanced, neutral",
        }
    }

    /// Convert emotion to setting description
    fn emotion_to_setting(&self, emotion: &EmotionState) -> &'static str {
        match emotion.primary {
            Emotion::Joy | Emotion::Euphoria => "sunlit meadow with dancing light",
            Emotion::Triumph => "mountaintop at golden hour",
            Emotion::Excitement => "neon-lit cityscape at night",
            Emotion::Anger | Emotion::Intensity => "storm-wracked landscape",
            Emotion::Urgency => "racing through corridors",
            Emotion::Chaos => "fracturing reality, kaleidoscope world",
            Emotion::Peace => "still lake reflecting mountains",
            Emotion::Tenderness => "intimate room with warm glow",
            Emotion::Hope => "dawn breaking over horizon",
            Emotion::Nostalgia => "autumn scene, fading light",
            Emotion::Sadness => "rain on empty streets",
            Emotion::Melancholy => "foggy forest, muted colors",
            Emotion::Tension => "narrow space, shadows watching",
            Emotion::Dread => "void with distant lights",
            Emotion::Neutral => "abstract space, neutral forms",
        }
    }

    /// Get current generation parameters
    pub fn params(&self) -> &GenerationParams {
        &self.params
    }

    /// Get clarity breakdown for debugging/UI
    pub fn clarity_breakdown(&self) -> ClarityBreakdown {
        ClarityBreakdown {
            total: self.clarity,
            key: self.key_clarity,
            structure: self.structure_clarity,
            pattern: self.pattern_clarity,
            climax: self.climax_clarity,
        }
    }

    /// Reset for new song
    pub fn reset(&mut self) {
        self.accumulator = SemanticAccumulator::new();
        self.clarity = 0.0;
        self.key_clarity = 0.0;
        self.structure_clarity = 0.0;
        self.pattern_clarity = 0.0;
        self.climax_clarity = 0.0;
        self.elapsed_time = 0.0;
        self.first_peak_reached = false;
        self.params = GenerationParams::default();
    }
}

/// Clarity breakdown for debugging/UI
#[derive(Debug, Clone)]
pub struct ClarityBreakdown {
    pub total: f32,
    pub key: f32,
    pub structure: f32,
    pub pattern: f32,
    pub climax: f32,
}

impl Default for RevelationEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create revelation engine")
    }
}
