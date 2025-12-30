//! Synchronization Engine
//!
//! Combines pre-analyzed data with real-time audio features
//! to produce complete MusicUnderstanding.

use super::synth_loader::MusicAnalysis;
use super::audio_player::AudioFeatures;
use crate::music::{
    MusicUnderstanding, NarrativeBeat,
    SignalFeatures, TheoryFeatures, Section, SectionType,
    Key, Mode, Chord, ChordType, EmotionState, Emotion,
};

/// Synchronization engine that merges pre-analysis with real-time
pub struct SyncEngine {
    /// Pre-computed analysis
    analysis: Option<MusicAnalysis>,

    /// Previous understanding for smoothing
    prev_understanding: Option<MusicUnderstanding>,

    /// Smoothing factor
    smoothing: f32,
}

impl SyncEngine {
    pub fn new() -> Self {
        Self {
            analysis: None,
            prev_understanding: None,
            smoothing: 0.1,
        }
    }

    /// Load pre-computed analysis
    pub fn load_analysis(&mut self, analysis: &MusicAnalysis) {
        self.analysis = Some(analysis.clone());
        self.prev_understanding = None;
    }

    /// Get complete music understanding at a point in time
    pub fn get_understanding(
        &mut self,
        time: f64,
        audio: &AudioFeatures,
    ) -> MusicUnderstanding {
        // Build signal features from real-time audio
        let signal = self.build_signal_features(audio);

        // Build theory features from analysis + real-time
        let theory = self.build_theory_features(time, audio);

        // Get section from analysis
        let section = self.get_section(time);
        let section_progress = self.get_section_progress(time, &section);

        // Check for section change
        let approaching_change = self.is_approaching_change(time);

        // Climax info
        let (climax_distance, is_climax) = self.get_climax_info(time);

        // Energy arc position
        let energy_arc_position = self.get_energy_arc_position(time);

        // Narrative beat
        let narrative_beat = self.get_narrative_beat(time, &section, energy_arc_position);

        // Repetition count
        let repetition_count = self.get_repetition_count(time);

        // Emotion
        let emotion = self.get_emotion_state(time);

        let understanding = MusicUnderstanding {
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
        };

        self.prev_understanding = Some(understanding.clone());
        understanding
    }

    fn build_signal_features(&self, audio: &AudioFeatures) -> SignalFeatures {
        SignalFeatures {
            spectrum: audio.spectrum,
            rms: audio.rms,
            bass: audio.bass,
            mid: audio.mid,
            high: audio.high,
            is_beat: audio.is_beat,
            beat_strength: audio.beat_strength,
            is_onset: audio.spectral_flux > 0.3,
            onset_strength: audio.spectral_flux,
            spectral_centroid: audio.spectral_centroid,
            spectral_flux: audio.spectral_flux,
            zero_crossing_rate: 0.1, // Not computed in real-time
            instant_tempo: self.analysis.as_ref().map(|a| a.tempo).unwrap_or(120.0),
        }
    }

    fn build_theory_features(&self, time: f64, audio: &AudioFeatures) -> TheoryFeatures {
        if let Some(ref analysis) = self.analysis {
            // Parse key from analysis
            let key = Self::parse_key(&analysis.key);
            let mode = Self::parse_mode(&analysis.mode);

            // Get chord at time
            let chord = if let Some(c) = analysis.chord_at(time) {
                Self::parse_chord(&c.chord)
            } else {
                Chord::default()
            };

            // Get tension from curve
            let tension = analysis.tension_at(time);

            TheoryFeatures {
                key,
                mode,
                key_confidence: analysis.key_confidence,
                chord,
                chord_tension: tension,
                tempo: analysis.tempo,
                tempo_confidence: analysis.tempo_confidence,
                tempo_derivative: 0.0,
                timbre_brightness: audio.spectral_centroid / 5000.0,
                timbre_roughness: audio.spectral_flux,
            }
        } else {
            TheoryFeatures::default()
        }
    }

    fn get_section(&self, time: f64) -> Section {
        if let Some(ref analysis) = self.analysis {
            if let Some(s) = analysis.section_at(time) {
                return Section {
                    section_type: Self::parse_section_type(&s.section_type),
                    start_time: s.start,
                    end_time: s.end,
                    energy: s.energy,
                    repetition: s.repetition,
                    confidence: s.confidence,
                };
            }
        }
        Section::default()
    }

    fn get_section_progress(&self, time: f64, section: &Section) -> f32 {
        if section.end_time > section.start_time {
            ((time - section.start_time) / (section.end_time - section.start_time)) as f32
        } else {
            0.0
        }
    }

    fn is_approaching_change(&self, time: f64) -> bool {
        if let Some(ref analysis) = self.analysis {
            if let Some(section) = analysis.section_at(time) {
                let time_to_end = section.end - time;
                return time_to_end > 0.0 && time_to_end < 2.0;
            }
        }
        false
    }

    fn get_climax_info(&self, time: f64) -> (f32, bool) {
        if let Some(ref analysis) = self.analysis {
            if let Some((climax, distance)) = analysis.nearest_climax(time) {
                let is_climax = distance < 0.5;
                return (distance as f32, is_climax);
            }
        }
        (f32::MAX, false)
    }

    fn get_energy_arc_position(&self, time: f64) -> f32 {
        if let Some(ref analysis) = self.analysis {
            if analysis.duration > 0.0 {
                return (time / analysis.duration) as f32;
            }
        }
        0.0
    }

    fn get_narrative_beat(
        &self,
        time: f64,
        section: &Section,
        energy_arc: f32,
    ) -> NarrativeBeat {
        // Check for climax
        if let Some(ref analysis) = self.analysis {
            if let Some((_, distance)) = analysis.nearest_climax(time) {
                if distance < 1.0 {
                    return NarrativeBeat::Climax;
                }
            }
        }

        match section.section_type {
            SectionType::Intro => NarrativeBeat::Establishment,
            SectionType::Verse => {
                if energy_arc < 0.3 {
                    NarrativeBeat::Establishment
                } else {
                    NarrativeBeat::RisingAction
                }
            }
            SectionType::PreChorus | SectionType::Buildup => NarrativeBeat::RisingAction,
            SectionType::Chorus | SectionType::Drop => {
                if energy_arc > 0.8 {
                    NarrativeBeat::Climax
                } else {
                    NarrativeBeat::RisingAction
                }
            }
            SectionType::Bridge | SectionType::Breakdown => NarrativeBeat::FallingAction,
            SectionType::Outro => NarrativeBeat::Resolution,
            _ => NarrativeBeat::Establishment,
        }
    }

    fn get_repetition_count(&self, time: f64) -> u8 {
        if let Some(ref analysis) = self.analysis {
            if let Some(section) = analysis.section_at(time) {
                return section.repetition;
            }
        }
        0
    }

    fn get_emotion_state(&self, time: f64) -> EmotionState {
        if let Some(ref analysis) = self.analysis {
            if let Some(e) = analysis.emotion_at(time) {
                let emotion = Self::parse_emotion(&e.emotion);
                return EmotionState {
                    primary: emotion,
                    intensity: e.intensity,
                    secondary: None,
                    blend: 0.0,
                    valence: e.valence,
                    arousal: e.arousal,
                };
            }
        }
        EmotionState::default()
    }

    // Parsing helpers

    fn parse_key(s: &str) -> Key {
        match s.to_uppercase().as_str() {
            "C" => Key::C,
            "C#" | "DB" => Key::Cs,
            "D" => Key::D,
            "D#" | "EB" => Key::Ds,
            "E" => Key::E,
            "F" => Key::F,
            "F#" | "GB" => Key::Fs,
            "G" => Key::G,
            "G#" | "AB" => Key::Gs,
            "A" => Key::A,
            "A#" | "BB" => Key::As,
            "B" => Key::B,
            _ => Key::Unknown,
        }
    }

    fn parse_mode(s: &str) -> Mode {
        match s.to_lowercase().as_str() {
            "major" => Mode::Major,
            "minor" => Mode::Minor,
            "dorian" => Mode::Dorian,
            "phrygian" => Mode::Phrygian,
            "lydian" => Mode::Lydian,
            "mixolydian" => Mode::Mixolydian,
            "aeolian" => Mode::Aeolian,
            "locrian" => Mode::Locrian,
            _ => Mode::Major,
        }
    }

    fn parse_chord(s: &str) -> Chord {
        // Simple chord parsing
        let s = s.trim();
        if s.is_empty() {
            return Chord::default();
        }

        // Extract root
        let (root_str, suffix) = if s.len() > 1 && (s.chars().nth(1) == Some('#') || s.chars().nth(1) == Some('b')) {
            (&s[..2], &s[2..])
        } else {
            (&s[..1], &s[1..])
        };

        let root = Self::parse_key(root_str);

        let chord_type = if suffix.contains("dim") {
            ChordType::Diminished
        } else if suffix.contains("aug") {
            ChordType::Augmented
        } else if suffix.contains("m7") {
            ChordType::Minor7
        } else if suffix.contains("maj7") {
            ChordType::Major7
        } else if suffix.contains("7") {
            ChordType::Dominant7
        } else if suffix.contains("sus2") {
            ChordType::Sus2
        } else if suffix.contains("sus4") {
            ChordType::Sus4
        } else if suffix.contains("m") {
            ChordType::Minor
        } else {
            ChordType::Major
        };

        Chord {
            root,
            chord_type,
            confidence: 0.8,
        }
    }

    fn parse_section_type(s: &str) -> SectionType {
        match s.to_lowercase().as_str() {
            "intro" => SectionType::Intro,
            "verse" => SectionType::Verse,
            "pre_chorus" | "prechorus" => SectionType::PreChorus,
            "chorus" => SectionType::Chorus,
            "bridge" => SectionType::Bridge,
            "breakdown" => SectionType::Breakdown,
            "buildup" => SectionType::Buildup,
            "drop" => SectionType::Drop,
            "outro" => SectionType::Outro,
            "instrumental" => SectionType::Instrumental,
            _ => SectionType::Unknown,
        }
    }

    fn parse_emotion(s: &str) -> Emotion {
        match s.to_lowercase().as_str() {
            "joy" => Emotion::Joy,
            "triumph" => Emotion::Triumph,
            "excitement" => Emotion::Excitement,
            "euphoria" => Emotion::Euphoria,
            "anger" => Emotion::Anger,
            "intensity" => Emotion::Intensity,
            "urgency" => Emotion::Urgency,
            "chaos" => Emotion::Chaos,
            "peace" => Emotion::Peace,
            "tenderness" => Emotion::Tenderness,
            "hope" => Emotion::Hope,
            "nostalgia" => Emotion::Nostalgia,
            "sadness" => Emotion::Sadness,
            "melancholy" => Emotion::Melancholy,
            "tension" => Emotion::Tension,
            "dread" => Emotion::Dread,
            _ => Emotion::Neutral,
        }
    }
}

impl Default for SyncEngine {
    fn default() -> Self {
        Self::new()
    }
}
