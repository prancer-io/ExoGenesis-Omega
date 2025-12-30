//! Music Understanding Engine
//!
//! The core of SYNESTHESIA - understanding music at three layers:
//! 1. Signal: Raw audio features (FFT, beats, energy)
//! 2. Theory: Musical concepts (key, chords, tempo, structure)
//! 3. Narrative: Emotional story (arc, climax, scenes)

mod signal;
mod theory;
mod structure;
mod emotion;
mod analysis;

pub use signal::{SignalAnalyzer, SignalFeatures};
pub use theory::{TheoryAnalyzer, TheoryFeatures, Key, Mode, Chord};
pub use structure::{StructureAnalyzer, Section, SectionType};
pub use emotion::{EmotionMapper, Emotion, EmotionState};
pub use analysis::{MusicAnalysis, OfflineAnalyzer};

/// Complete music understanding at a point in time
#[derive(Debug, Clone)]
pub struct MusicUnderstanding {
    /// Current playback time
    pub time: f64,

    // ─────────────────────────────────────────────────────────────
    // Layer 1: Signal (real-time, <10ms latency)
    // ─────────────────────────────────────────────────────────────
    pub signal: SignalFeatures,

    // ─────────────────────────────────────────────────────────────
    // Layer 2: Music Theory (real-time + pre-analyzed)
    // ─────────────────────────────────────────────────────────────
    pub theory: TheoryFeatures,

    // ─────────────────────────────────────────────────────────────
    // Layer 3: Structure & Narrative (pre-analyzed)
    // ─────────────────────────────────────────────────────────────

    /// Current section of the song
    pub section: Section,

    /// Progress through current section (0.0 - 1.0)
    pub section_progress: f32,

    /// Are we approaching a section change?
    pub approaching_change: bool,

    /// Distance to next climax (seconds, negative if past)
    pub climax_distance: f32,

    /// Are we currently at a climax?
    pub is_climax: bool,

    /// Position in overall energy arc (0.0 - 1.0)
    pub energy_arc_position: f32,

    /// Current narrative beat
    pub narrative_beat: NarrativeBeat,

    /// How many times we've heard similar content
    pub repetition_count: u8,

    // ─────────────────────────────────────────────────────────────
    // Emotion (derived from all layers)
    // ─────────────────────────────────────────────────────────────

    /// Current emotional state
    pub emotion: EmotionState,
}

/// Where we are in the story arc
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NarrativeBeat {
    /// Setting the scene, establishing mood
    Establishment,
    /// Building tension, adding elements
    RisingAction,
    /// Peak moment, maximum intensity
    Climax,
    /// Coming down, processing
    FallingAction,
    /// Ending, closure
    Resolution,
}

impl NarrativeBeat {
    /// Get visual intensity multiplier
    pub fn intensity_multiplier(&self) -> f32 {
        match self {
            Self::Establishment => 0.3,
            Self::RisingAction => 0.6,
            Self::Climax => 1.0,
            Self::FallingAction => 0.5,
            Self::Resolution => 0.2,
        }
    }
}

/// The main music understanding engine
pub struct MusicEngine {
    /// Real-time signal analyzer
    signal_analyzer: SignalAnalyzer,

    /// Music theory analyzer
    theory_analyzer: TheoryAnalyzer,

    /// Pre-computed analysis (loaded from .synth file)
    offline_analysis: Option<MusicAnalysis>,

    /// Emotion mapper
    emotion_mapper: EmotionMapper,

    /// Sample rate
    sample_rate: u32,

    /// Current time
    current_time: f64,
}

impl MusicEngine {
    /// Create new music engine
    pub fn new(sample_rate: u32) -> Self {
        Self {
            signal_analyzer: SignalAnalyzer::new(sample_rate),
            theory_analyzer: TheoryAnalyzer::new(sample_rate),
            offline_analysis: None,
            emotion_mapper: EmotionMapper::new(),
            sample_rate,
            current_time: 0.0,
        }
    }

    /// Load pre-computed analysis
    pub fn load_analysis(&mut self, analysis: MusicAnalysis) {
        self.offline_analysis = Some(analysis);
    }

    /// Process audio samples and return understanding
    pub fn process(&mut self, samples: &[f32], time: f64) -> MusicUnderstanding {
        self.current_time = time;

        // Layer 1: Real-time signal analysis
        let signal = self.signal_analyzer.analyze(samples);

        // Layer 2: Music theory (real-time + lookup)
        let theory = self.get_theory_features(time, &signal);

        // Layer 3: Structure (from pre-analysis)
        let (section, section_progress, approaching_change) = self.get_section_info(time);
        let (climax_distance, is_climax) = self.get_climax_info(time);
        let energy_arc_position = self.get_energy_arc_position(time);
        let narrative_beat = self.get_narrative_beat(time, &section, energy_arc_position);
        let repetition_count = self.get_repetition_count(time, &section);

        // Emotion: Derived from all layers
        let emotion = self.emotion_mapper.map(&theory, &section, energy_arc_position);

        MusicUnderstanding {
            time,
            signal,
            theory,
            section,
            section_progress,
            approaching_change,
            climax_distance,
            is_climax,
            energy_arc_position,
            narrative_beat,
            repetition_count,
            emotion,
        }
    }

    /// Get theory features (real-time + pre-analyzed)
    fn get_theory_features(&self, time: f64, signal: &SignalFeatures) -> TheoryFeatures {
        if let Some(ref analysis) = self.offline_analysis {
            // Use pre-analyzed key, chord, etc.
            let chord = analysis.chord_at(time);
            let tension = analysis.tension_at(time);

            TheoryFeatures {
                key: analysis.key,
                mode: analysis.mode,
                key_confidence: analysis.key_confidence,
                chord,
                chord_tension: tension,
                tempo: analysis.tempo_at(time),
                tempo_confidence: 0.9,
                tempo_derivative: analysis.tempo_derivative_at(time),
                timbre_brightness: signal.spectral_centroid / 5000.0,
                timbre_roughness: signal.spectral_flux,
            }
        } else {
            // Real-time only (less accurate)
            self.theory_analyzer.analyze_realtime(signal)
        }
    }

    /// Get current section info
    fn get_section_info(&self, time: f64) -> (Section, f32, bool) {
        if let Some(ref analysis) = self.offline_analysis {
            let section = analysis.section_at(time);
            let progress = analysis.section_progress_at(time);
            let approaching = analysis.approaching_section_change(time, 2.0); // 2 sec lookahead
            (section, progress, approaching)
        } else {
            (Section::default(), 0.0, false)
        }
    }

    /// Get climax info
    fn get_climax_info(&self, time: f64) -> (f32, bool) {
        if let Some(ref analysis) = self.offline_analysis {
            let distance = analysis.distance_to_climax(time);
            let is_climax = distance.abs() < 0.5; // Within 0.5 seconds
            (distance, is_climax)
        } else {
            (f32::MAX, false)
        }
    }

    /// Get position in energy arc
    fn get_energy_arc_position(&self, time: f64) -> f32 {
        if let Some(ref analysis) = self.offline_analysis {
            analysis.energy_arc_position(time)
        } else {
            (time / 180.0).min(1.0) as f32 // Assume 3 min song
        }
    }

    /// Get narrative beat
    fn get_narrative_beat(
        &self,
        time: f64,
        section: &Section,
        energy_arc: f32,
    ) -> NarrativeBeat {
        if let Some(ref analysis) = self.offline_analysis {
            // Check for climax
            if analysis.distance_to_climax(time).abs() < 1.0 {
                return NarrativeBeat::Climax;
            }
        }

        // Derive from section and energy
        match section.section_type {
            SectionType::Intro => NarrativeBeat::Establishment,
            SectionType::Verse => {
                if energy_arc < 0.3 {
                    NarrativeBeat::Establishment
                } else {
                    NarrativeBeat::RisingAction
                }
            }
            SectionType::PreChorus => NarrativeBeat::RisingAction,
            SectionType::Chorus => {
                if energy_arc > 0.8 {
                    NarrativeBeat::Climax
                } else {
                    NarrativeBeat::RisingAction
                }
            }
            SectionType::Bridge => NarrativeBeat::FallingAction,
            SectionType::Breakdown => NarrativeBeat::FallingAction,
            SectionType::Buildup => NarrativeBeat::RisingAction,
            SectionType::Drop => NarrativeBeat::Climax,
            SectionType::Outro => NarrativeBeat::Resolution,
            SectionType::Instrumental => {
                if energy_arc > 0.7 {
                    NarrativeBeat::Climax
                } else {
                    NarrativeBeat::RisingAction
                }
            }
            SectionType::Unknown => NarrativeBeat::Establishment,
        }
    }

    /// Get repetition count for current section
    fn get_repetition_count(&self, time: f64, section: &Section) -> u8 {
        if let Some(ref analysis) = self.offline_analysis {
            analysis.section_repetition_at(time)
        } else {
            0
        }
    }

    /// Get current time
    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    /// Check if we have offline analysis loaded
    pub fn has_analysis(&self) -> bool {
        self.offline_analysis.is_some()
    }
}

impl Default for MusicEngine {
    fn default() -> Self {
        Self::new(44100)
    }
}
