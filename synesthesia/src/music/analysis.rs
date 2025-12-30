//! Offline Music Analysis
//!
//! Pre-computes all musical features for a song. This data is stored
//! in .synth files and loaded at runtime for instant access.

use super::theory::{Key, Mode, Chord, ChordType};
use super::structure::{Section, SectionType, StructureAnalysis};
use super::emotion::Emotion;

/// Complete pre-computed music analysis
#[derive(Debug, Clone)]
pub struct MusicAnalysis {
    /// Song duration in seconds
    pub duration: f64,

    /// Detected key
    pub key: Key,

    /// Major/minor mode
    pub mode: Mode,

    /// Key detection confidence
    pub key_confidence: f32,

    /// Base tempo (BPM)
    pub base_tempo: f32,

    /// Tempo curve (time, bpm) for songs with tempo changes
    pub tempo_curve: Vec<(f64, f32)>,

    /// Beat timestamps
    pub beats: Vec<f64>,

    /// Downbeat timestamps (first beat of each bar)
    pub downbeats: Vec<f64>,

    /// Chord progression
    pub chords: Vec<ChordEvent>,

    /// Song structure
    pub structure: StructureAnalysis,

    /// Energy curve sampled at 10Hz
    pub energy_curve: Vec<f32>,

    /// Harmonic tension curve sampled at 10Hz
    pub tension_curve: Vec<f32>,

    /// Climax points
    pub climaxes: Vec<ClimaxPoint>,

    /// Emotional arc
    pub emotion_arc: Vec<EmotionPoint>,
}

/// A chord at a point in time
#[derive(Debug, Clone)]
pub struct ChordEvent {
    pub time: f64,
    pub duration: f64,
    pub chord: Chord,
}

/// A climax/peak moment
#[derive(Debug, Clone)]
pub struct ClimaxPoint {
    pub time: f64,
    pub intensity: f32,
    pub climax_type: ClimaxType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClimaxType {
    /// Energy peak (loud moment)
    Energy,
    /// Drop (after buildup)
    Drop,
    /// Emotional peak (key change, resolution)
    Emotional,
    /// Final climax of the song
    Final,
}

/// Emotion at a point in the song
#[derive(Debug, Clone)]
pub struct EmotionPoint {
    pub time: f64,
    pub emotion: Emotion,
    pub intensity: f32,
}

impl MusicAnalysis {
    /// Create empty analysis
    pub fn new(duration: f64) -> Self {
        Self {
            duration,
            key: Key::Unknown,
            mode: Mode::Major,
            key_confidence: 0.0,
            base_tempo: 120.0,
            tempo_curve: vec![(0.0, 120.0)],
            beats: Vec::new(),
            downbeats: Vec::new(),
            chords: Vec::new(),
            structure: StructureAnalysis::default(),
            energy_curve: Vec::new(),
            tension_curve: Vec::new(),
            climaxes: Vec::new(),
            emotion_arc: Vec::new(),
        }
    }

    /// Get tempo at a specific time
    pub fn tempo_at(&self, time: f64) -> f32 {
        // Find the tempo at this time from curve
        let mut tempo = self.base_tempo;
        for (t, bpm) in &self.tempo_curve {
            if *t <= time {
                tempo = *bpm;
            } else {
                break;
            }
        }
        tempo
    }

    /// Get tempo derivative (change rate) at time
    pub fn tempo_derivative_at(&self, time: f64) -> f32 {
        if self.tempo_curve.len() < 2 {
            return 0.0;
        }

        // Find surrounding points
        let mut prev = (0.0, self.base_tempo);
        let mut next = (self.duration, self.base_tempo);

        for i in 0..self.tempo_curve.len() - 1 {
            if self.tempo_curve[i].0 <= time && self.tempo_curve[i + 1].0 > time {
                prev = self.tempo_curve[i];
                next = self.tempo_curve[i + 1];
                break;
            }
        }

        let dt = next.0 - prev.0;
        if dt > 0.0 {
            ((next.1 - prev.1) / dt as f32)
        } else {
            0.0
        }
    }

    /// Get chord at time
    pub fn chord_at(&self, time: f64) -> Chord {
        for event in &self.chords {
            if time >= event.time && time < event.time + event.duration {
                return event.chord;
            }
        }
        Chord::default()
    }

    /// Get harmonic tension at time
    pub fn tension_at(&self, time: f64) -> f32 {
        // Sample at 10Hz
        let index = (time * 10.0) as usize;
        if index < self.tension_curve.len() {
            self.tension_curve[index]
        } else {
            0.0
        }
    }

    /// Get energy at time
    pub fn energy_at(&self, time: f64) -> f32 {
        let index = (time * 10.0) as usize;
        if index < self.energy_curve.len() {
            self.energy_curve[index]
        } else {
            0.5
        }
    }

    /// Get section at time
    pub fn section_at(&self, time: f64) -> Section {
        self.structure.section_at(time)
    }

    /// Get section progress at time
    pub fn section_progress_at(&self, time: f64) -> f32 {
        self.structure.section_progress_at(time)
    }

    /// Is approaching section change?
    pub fn approaching_section_change(&self, time: f64, lookahead: f64) -> bool {
        self.structure.approaching_change(time, lookahead)
    }

    /// Get distance to nearest climax (negative if past)
    pub fn distance_to_climax(&self, time: f64) -> f32 {
        let mut min_distance = f32::MAX;

        for climax in &self.climaxes {
            let distance = (climax.time - time) as f32;
            if distance.abs() < min_distance.abs() {
                min_distance = distance;
            }
        }

        min_distance
    }

    /// Get position in overall energy arc (0-1)
    pub fn energy_arc_position(&self, time: f64) -> f32 {
        // Find cumulative energy up to this point
        let current_index = (time * 10.0) as usize;
        let total_samples = self.energy_curve.len();

        if total_samples == 0 {
            return (time / self.duration) as f32;
        }

        // Calculate what fraction of total energy we've experienced
        let energy_so_far: f32 = self.energy_curve[..current_index.min(total_samples)]
            .iter()
            .sum();
        let total_energy: f32 = self.energy_curve.iter().sum();

        if total_energy > 0.0 {
            energy_so_far / total_energy
        } else {
            (time / self.duration) as f32
        }
    }

    /// Get section repetition count at time
    pub fn section_repetition_at(&self, time: f64) -> u8 {
        self.structure.repetition_at(time)
    }

    /// Get emotion at time
    pub fn emotion_at(&self, time: f64) -> (Emotion, f32) {
        for i in 0..self.emotion_arc.len() {
            if i + 1 < self.emotion_arc.len() {
                if time >= self.emotion_arc[i].time && time < self.emotion_arc[i + 1].time {
                    return (self.emotion_arc[i].emotion, self.emotion_arc[i].intensity);
                }
            } else if time >= self.emotion_arc[i].time {
                return (self.emotion_arc[i].emotion, self.emotion_arc[i].intensity);
            }
        }
        (Emotion::Neutral, 0.5)
    }
}

/// Offline analyzer - runs before playback
pub struct OfflineAnalyzer {
    // Configuration
    sample_rate: u32,
}

impl OfflineAnalyzer {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
        }
    }

    /// Analyze a complete audio file
    /// In real implementation, this would use Essentia
    pub fn analyze(&self, audio_path: &str) -> anyhow::Result<MusicAnalysis> {
        // This is a placeholder - real implementation would:
        //
        // 1. Load audio with symphonia
        // 2. Run Essentia algorithms:
        //    - KeyExtractor
        //    - ChordsDetection
        //    - BeatTracker
        //    - OnsetDetection
        //    - RhythmExtractor
        //    - MusicStructure
        //    - Intensity
        // 3. Post-process to find climaxes
        // 4. Map to emotions

        log::info!("Analyzing: {}", audio_path);

        // Return dummy analysis for development
        let analysis = MusicAnalysis {
            duration: 180.0, // 3 minutes
            key: Key::A,
            mode: Mode::Minor,
            key_confidence: 0.85,
            base_tempo: 128.0,
            tempo_curve: vec![
                (0.0, 128.0),
            ],
            beats: (0..360).map(|i| i as f64 * 0.5).collect(), // Beat every 0.5s
            downbeats: (0..90).map(|i| i as f64 * 2.0).collect(), // Downbeat every 2s
            chords: vec![
                ChordEvent {
                    time: 0.0,
                    duration: 8.0,
                    chord: Chord { root: Key::A, chord_type: ChordType::Minor, confidence: 0.9 },
                },
                ChordEvent {
                    time: 8.0,
                    duration: 8.0,
                    chord: Chord { root: Key::F, chord_type: ChordType::Major, confidence: 0.9 },
                },
                // ... more chords
            ],
            structure: StructureAnalysis {
                sections: vec![
                    Section {
                        section_type: SectionType::Intro,
                        start_time: 0.0,
                        end_time: 16.0,
                        energy: 0.3,
                        repetition: 1,
                        confidence: 0.9,
                    },
                    Section {
                        section_type: SectionType::Verse,
                        start_time: 16.0,
                        end_time: 48.0,
                        energy: 0.5,
                        repetition: 1,
                        confidence: 0.9,
                    },
                    Section {
                        section_type: SectionType::Buildup,
                        start_time: 48.0,
                        end_time: 64.0,
                        energy: 0.7,
                        repetition: 1,
                        confidence: 0.85,
                    },
                    Section {
                        section_type: SectionType::Drop,
                        start_time: 64.0,
                        end_time: 96.0,
                        energy: 1.0,
                        repetition: 1,
                        confidence: 0.95,
                    },
                    Section {
                        section_type: SectionType::Breakdown,
                        start_time: 96.0,
                        end_time: 112.0,
                        energy: 0.4,
                        repetition: 1,
                        confidence: 0.8,
                    },
                    Section {
                        section_type: SectionType::Buildup,
                        start_time: 112.0,
                        end_time: 128.0,
                        energy: 0.8,
                        repetition: 2,
                        confidence: 0.85,
                    },
                    Section {
                        section_type: SectionType::Drop,
                        start_time: 128.0,
                        end_time: 160.0,
                        energy: 1.0,
                        repetition: 2,
                        confidence: 0.95,
                    },
                    Section {
                        section_type: SectionType::Outro,
                        start_time: 160.0,
                        end_time: 180.0,
                        energy: 0.3,
                        repetition: 1,
                        confidence: 0.9,
                    },
                ],
                duration: 180.0,
            },
            energy_curve: (0..1800).map(|i| {
                let t = i as f32 / 10.0;
                // Simulate energy curve
                if t < 16.0 { 0.3 }
                else if t < 48.0 { 0.5 }
                else if t < 64.0 { 0.5 + (t - 48.0) / 32.0 } // Buildup
                else if t < 96.0 { 1.0 } // Drop
                else if t < 112.0 { 0.4 }
                else if t < 128.0 { 0.4 + (t - 112.0) / 40.0 }
                else if t < 160.0 { 1.0 }
                else { 0.3 }
            }).collect(),
            tension_curve: (0..1800).map(|i| {
                let t = i as f32 / 10.0;
                // Tension builds before drops
                if t > 56.0 && t < 64.0 { (t - 56.0) / 8.0 }
                else if t > 120.0 && t < 128.0 { (t - 120.0) / 8.0 }
                else { 0.2 }
            }).collect(),
            climaxes: vec![
                ClimaxPoint { time: 64.0, intensity: 1.0, climax_type: ClimaxType::Drop },
                ClimaxPoint { time: 128.0, intensity: 1.0, climax_type: ClimaxType::Final },
            ],
            emotion_arc: vec![
                EmotionPoint { time: 0.0, emotion: Emotion::Tension, intensity: 0.4 },
                EmotionPoint { time: 48.0, emotion: Emotion::Urgency, intensity: 0.7 },
                EmotionPoint { time: 64.0, emotion: Emotion::Euphoria, intensity: 1.0 },
                EmotionPoint { time: 96.0, emotion: Emotion::Peace, intensity: 0.5 },
                EmotionPoint { time: 112.0, emotion: Emotion::Excitement, intensity: 0.8 },
                EmotionPoint { time: 128.0, emotion: Emotion::Euphoria, intensity: 1.0 },
                EmotionPoint { time: 160.0, emotion: Emotion::Nostalgia, intensity: 0.6 },
            ],
        };

        Ok(analysis)
    }
}

impl Default for OfflineAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
