//! Music Theory Analysis - Layer 2
//!
//! Musical concepts: key, chords, tempo, timbre.
//! Combines real-time analysis with pre-computed data.

use super::signal::SignalFeatures;

/// Musical key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Key {
    C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B,
    #[default]
    Unknown,
}

impl Key {
    /// Get the key name
    pub fn name(&self) -> &'static str {
        match self {
            Key::C => "C",
            Key::Cs => "C#",
            Key::D => "D",
            Key::Ds => "D#",
            Key::E => "E",
            Key::F => "F",
            Key::Fs => "F#",
            Key::G => "G",
            Key::Gs => "G#",
            Key::A => "A",
            Key::As => "A#",
            Key::B => "B",
            Key::Unknown => "?",
        }
    }

    /// Get base hue for this key (0-360)
    pub fn base_hue(&self) -> f32 {
        match self {
            Key::C => 0.0,     // Red
            Key::G => 30.0,    // Orange
            Key::D => 60.0,    // Yellow
            Key::A => 90.0,    // Yellow-green
            Key::E => 120.0,   // Green
            Key::B => 150.0,   // Cyan-green
            Key::Fs => 180.0,  // Cyan
            Key::Cs => 210.0,  // Blue-cyan
            Key::Gs => 240.0,  // Blue
            Key::Ds => 270.0,  // Purple
            Key::As => 300.0,  // Magenta
            Key::F => 330.0,   // Pink
            Key::Unknown => 0.0,
        }
    }
}

/// Musical mode (major/minor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Major,
    Minor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    /// Is this a "happy" mode?
    pub fn is_bright(&self) -> bool {
        matches!(self, Mode::Major | Mode::Lydian | Mode::Mixolydian)
    }

    /// Saturation modifier
    pub fn saturation_modifier(&self) -> f32 {
        match self {
            Mode::Major => 0.8,
            Mode::Lydian => 0.9,
            Mode::Mixolydian => 0.7,
            Mode::Minor => 0.6,
            Mode::Dorian => 0.65,
            Mode::Aeolian => 0.55,
            Mode::Phrygian => 0.5,
            Mode::Locrian => 0.4,
        }
    }

    /// Lightness modifier
    pub fn lightness_modifier(&self) -> f32 {
        match self {
            Mode::Major => 0.6,
            Mode::Lydian => 0.7,
            Mode::Mixolydian => 0.55,
            Mode::Minor => 0.4,
            Mode::Dorian => 0.45,
            Mode::Aeolian => 0.35,
            Mode::Phrygian => 0.3,
            Mode::Locrian => 0.25,
        }
    }
}

/// Chord type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChordType {
    #[default]
    Major,
    Minor,
    Diminished,
    Augmented,
    Dominant7,
    Major7,
    Minor7,
    Sus2,
    Sus4,
    Power,
}

/// A chord
#[derive(Debug, Clone, Copy, Default)]
pub struct Chord {
    pub root: Key,
    pub chord_type: ChordType,
    pub confidence: f32,
}

impl Chord {
    /// Get tension level (0 = consonant, 1 = dissonant)
    pub fn tension(&self) -> f32 {
        match self.chord_type {
            ChordType::Major => 0.0,
            ChordType::Minor => 0.1,
            ChordType::Power => 0.05,
            ChordType::Sus2 => 0.2,
            ChordType::Sus4 => 0.25,
            ChordType::Major7 => 0.15,
            ChordType::Dominant7 => 0.4,
            ChordType::Minor7 => 0.3,
            ChordType::Diminished => 0.7,
            ChordType::Augmented => 0.6,
        }
    }

    /// Get descriptive name
    pub fn name(&self) -> String {
        let type_str = match self.chord_type {
            ChordType::Major => "",
            ChordType::Minor => "m",
            ChordType::Diminished => "dim",
            ChordType::Augmented => "aug",
            ChordType::Dominant7 => "7",
            ChordType::Major7 => "maj7",
            ChordType::Minor7 => "m7",
            ChordType::Sus2 => "sus2",
            ChordType::Sus4 => "sus4",
            ChordType::Power => "5",
        };
        format!("{}{}", self.root.name(), type_str)
    }
}

/// Music theory features at a point in time
#[derive(Debug, Clone, Default)]
pub struct TheoryFeatures {
    /// Detected key
    pub key: Key,

    /// Major/minor mode
    pub mode: Mode,

    /// Key detection confidence
    pub key_confidence: f32,

    /// Current chord
    pub chord: Chord,

    /// Harmonic tension (0 = resolved, 1 = tense)
    pub chord_tension: f32,

    /// Tempo (BPM)
    pub tempo: f32,

    /// Tempo confidence
    pub tempo_confidence: f32,

    /// Tempo derivative (positive = speeding up)
    pub tempo_derivative: f32,

    /// Timbre brightness (0 = dark, 1 = bright)
    pub timbre_brightness: f32,

    /// Timbre roughness (0 = smooth, 1 = harsh)
    pub timbre_roughness: f32,
}

/// Real-time theory analyzer (simplified - full version uses ML)
pub struct TheoryAnalyzer {
    sample_rate: u32,

    /// Running key estimate
    key_estimate: Key,
    key_confidence: f32,

    /// Running tempo estimate
    tempo_estimate: f32,
    tempo_history: Vec<f32>,
}

impl TheoryAnalyzer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            key_estimate: Key::Unknown,
            key_confidence: 0.0,
            tempo_estimate: 120.0,
            tempo_history: Vec::with_capacity(100),
        }
    }

    /// Analyze in real-time (less accurate than offline)
    pub fn analyze_realtime(&mut self, signal: &SignalFeatures) -> TheoryFeatures {
        // Update tempo estimate
        if signal.instant_tempo > 40.0 && signal.instant_tempo < 220.0 {
            self.tempo_history.push(signal.instant_tempo);
            if self.tempo_history.len() > 100 {
                self.tempo_history.remove(0);
            }
            self.tempo_estimate = self.tempo_history.iter().sum::<f32>()
                / self.tempo_history.len() as f32;
        }

        // Simplified key detection based on spectral features
        // Real implementation would use chroma features and key profiles
        let brightness = signal.spectral_centroid / 5000.0;

        TheoryFeatures {
            key: self.key_estimate,
            mode: if brightness > 0.5 { Mode::Major } else { Mode::Minor },
            key_confidence: self.key_confidence,
            chord: Chord::default(),
            chord_tension: signal.spectral_flux,
            tempo: self.tempo_estimate,
            tempo_confidence: if self.tempo_history.len() > 20 { 0.8 } else { 0.3 },
            tempo_derivative: 0.0,
            timbre_brightness: brightness.clamp(0.0, 1.0),
            timbre_roughness: signal.zero_crossing_rate.clamp(0.0, 1.0),
        }
    }
}
