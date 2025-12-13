//! Musical Feature Extraction
//!
//! Extracts high-level musical features from raw audio analysis.

use crate::audio::AudioAnalyzer;
use crate::Result;

/// High-level musical features for a moment in time
#[derive(Debug, Clone)]
pub struct MusicalFeatures {
    /// Timestamp in seconds
    pub timestamp: f64,

    // === Pitch Features ===
    /// Dominant pitch in Hz
    pub pitch: f32,
    /// Pitch in MIDI note number (0-127)
    pub midi_note: u8,
    /// Pitch class (0-11, C=0)
    pub pitch_class: u8,
    /// Octave number
    pub octave: i8,

    // === Rhythm Features ===
    /// Onset strength (beat likelihood)
    pub onset_strength: f32,
    /// Is this a beat?
    pub is_beat: bool,
    /// Estimated tempo in BPM
    pub tempo: f32,
    /// Beat phase (0-1)
    pub beat_phase: f32,

    // === Dynamics ===
    /// Loudness (0-1)
    pub loudness: f32,
    /// Dynamic change rate
    pub dynamics_delta: f32,

    // === Timbre ===
    /// Brightness (spectral centroid normalized)
    pub brightness: f32,
    /// Roughness/noisiness (spectral flatness)
    pub roughness: f32,
    /// Warmth (low frequency content)
    pub warmth: f32,
    /// Sharpness (high frequency content)
    pub sharpness: f32,

    // === Harmony ===
    /// Estimated key (0-11, major) or (12-23, minor)
    pub key: u8,
    /// Chord type estimate
    pub chord_type: ChordType,
    /// Harmonic tension (0-1)
    pub tension: f32,

    // === Emotion ===
    /// Emotional valence (-1 to 1, sad to happy)
    pub valence: f32,
    /// Arousal/energy level (0-1)
    pub arousal: f32,
    /// Overall emotional color
    pub emotion: EmotionalValence,

    // === Raw spectral data for advanced use ===
    /// Mel-frequency bands (13 bands)
    pub mfcc: [f32; 13],
    /// Chroma features (12 pitch classes)
    pub chroma: [f32; 12],
}

impl MusicalFeatures {
    /// Convert to embedding vector for mindscape storage
    pub fn to_embedding(&self) -> Vec<f64> {
        let mut embedding = Vec::with_capacity(64);

        // Pitch features
        embedding.push(self.pitch as f64 / 20000.0);
        embedding.push(self.midi_note as f64 / 127.0);
        embedding.push(self.pitch_class as f64 / 12.0);

        // Rhythm
        embedding.push(self.onset_strength as f64);
        embedding.push(if self.is_beat { 1.0 } else { 0.0 });
        embedding.push(self.tempo as f64 / 200.0);
        embedding.push(self.beat_phase as f64);

        // Dynamics
        embedding.push(self.loudness as f64);
        embedding.push((self.dynamics_delta + 1.0) as f64 / 2.0);

        // Timbre
        embedding.push(self.brightness as f64);
        embedding.push(self.roughness as f64);
        embedding.push(self.warmth as f64);
        embedding.push(self.sharpness as f64);

        // Harmony
        embedding.push(self.key as f64 / 24.0);
        embedding.push(self.tension as f64);

        // Emotion
        embedding.push((self.valence + 1.0) as f64 / 2.0);
        embedding.push(self.arousal as f64);

        // MFCC
        for mfcc in &self.mfcc {
            embedding.push((*mfcc).clamp(-1.0, 1.0) as f64);
        }

        // Chroma
        for chroma in &self.chroma {
            embedding.push(*chroma as f64);
        }

        // Pad to 64 dimensions
        while embedding.len() < 64 {
            embedding.push(0.0);
        }

        embedding
    }
}

/// Chord type estimation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
    Suspended,
    Dominant7,
    Major7,
    Minor7,
    Unknown,
}

/// Emotional valence categories
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmotionalValence {
    /// Joyful, bright, uplifting
    Joy,
    /// Sad, melancholic, reflective
    Sadness,
    /// Angry, aggressive, intense
    Anger,
    /// Calm, peaceful, serene
    Peace,
    /// Fearful, tense, anxious
    Fear,
    /// Surprised, unexpected, exciting
    Surprise,
    /// Neutral, balanced
    Neutral,
}

impl EmotionalValence {
    /// Get color associated with this emotion (RGB)
    pub fn color(&self) -> [f32; 3] {
        match self {
            Self::Joy => [1.0, 0.85, 0.2],      // Golden yellow
            Self::Sadness => [0.2, 0.3, 0.6],    // Blue
            Self::Anger => [0.8, 0.1, 0.1],      // Red
            Self::Peace => [0.3, 0.7, 0.5],      // Green
            Self::Fear => [0.4, 0.2, 0.5],       // Purple
            Self::Surprise => [1.0, 0.5, 0.0],   // Orange
            Self::Neutral => [0.5, 0.5, 0.5],    // Gray
        }
    }
}

/// Feature extractor for musical analysis
pub struct FeatureExtractor {
    /// Previous frame for delta calculations
    prev_frame: Option<MusicalFeatures>,
    /// Onset detection buffer (for future advanced detection)
    #[allow(dead_code)]
    onset_buffer: Vec<f32>,
    /// Beat tracking state (for future advanced tracking)
    #[allow(dead_code)]
    beat_tracker: BeatTracker,
    /// Key detection state (for future advanced detection)
    #[allow(dead_code)]
    key_detector: KeyDetector,
}

impl FeatureExtractor {
    pub fn new() -> Self {
        Self {
            prev_frame: None,
            onset_buffer: Vec::new(),
            beat_tracker: BeatTracker::new(),
            key_detector: KeyDetector::new(),
        }
    }

    /// Extract features from all audio frames
    pub fn extract_all(&mut self, analyzer: &AudioAnalyzer) -> Result<Vec<MusicalFeatures>> {
        let hop_size = 512;  // ~11.6ms at 44100Hz

        // We need to get the audio frames - for now use a simpler approach
        // In production, we'd have direct access to the analyzer's internal state

        let duration = analyzer.duration();
        let hop_duration = hop_size as f64 / analyzer.sample_rate() as f64;

        let mut features = Vec::new();
        let mut timestamp = 0.0;

        // Generate features based on duration (simplified for now)
        while timestamp < duration {
            let feature = self.extract_at_time(timestamp, duration);
            features.push(feature);
            timestamp += hop_duration;
        }

        Ok(features)
    }

    /// Extract features at a specific timestamp (simplified version)
    fn extract_at_time(&mut self, timestamp: f64, total_duration: f64) -> MusicalFeatures {
        // Simulate realistic music features based on time
        let progress = timestamp / total_duration;

        // Simulate pitch variation
        let base_pitch = 261.63; // C4
        let pitch_variation = (timestamp * 2.0).sin() as f32 * 200.0;
        let pitch = (base_pitch + pitch_variation).max(80.0);

        // Convert to MIDI
        let midi_note = (12.0 * (pitch / 440.0).log2() + 69.0).round() as u8;
        let pitch_class = midi_note % 12;
        let octave = (midi_note / 12) as i8 - 1;

        // Simulate beats
        let bpm = 120.0;
        let beat_period = 60.0 / bpm;
        let beat_phase = ((timestamp % beat_period as f64) / beat_period as f64) as f32;
        let is_beat = beat_phase < 0.1;
        let onset_strength = if is_beat { 0.8 + 0.2 * rand::random::<f32>() } else { 0.1 * rand::random::<f32>() };

        // Simulate dynamics
        let loudness = 0.5 + 0.3 * (timestamp * 0.5).sin() as f32;
        let prev_loudness = self.prev_frame.as_ref().map(|f| f.loudness).unwrap_or(loudness);
        let dynamics_delta = loudness - prev_loudness;

        // Simulate timbre
        let brightness = 0.5 + 0.3 * (timestamp * 1.5).cos() as f32;
        let roughness = 0.2 + 0.1 * rand::random::<f32>();
        let warmth = 1.0 - brightness;
        let sharpness = brightness * 0.8;

        // Simulate harmony
        let key = ((timestamp / 10.0) as u8 * 5) % 12;
        let chord_type = if progress < 0.3 {
            ChordType::Major
        } else if progress < 0.6 {
            ChordType::Minor
        } else {
            ChordType::Dominant7
        };
        let tension = (0.3 + 0.4 * (timestamp * 0.3).sin() as f32).clamp(0.0, 1.0);

        // Calculate emotion
        let valence = brightness - tension + 0.2;
        let arousal = loudness * 0.7 + onset_strength * 0.3;
        let emotion = Self::classify_emotion(valence, arousal);

        // Simulate MFCC and chroma
        let mut mfcc = [0.0f32; 13];
        for (i, m) in mfcc.iter_mut().enumerate() {
            *m = (timestamp * (i + 1) as f64 * 0.1).sin() as f32 * 0.5;
        }

        let mut chroma = [0.0f32; 12];
        chroma[pitch_class as usize] = 1.0;
        chroma[((pitch_class + 4) % 12) as usize] = 0.6;
        chroma[((pitch_class + 7) % 12) as usize] = 0.8;

        let feature = MusicalFeatures {
            timestamp,
            pitch,
            midi_note,
            pitch_class,
            octave,
            onset_strength,
            is_beat,
            tempo: bpm,
            beat_phase,
            loudness,
            dynamics_delta,
            brightness,
            roughness,
            warmth,
            sharpness,
            key,
            chord_type,
            tension,
            valence,
            arousal,
            emotion,
            mfcc,
            chroma,
        };

        self.prev_frame = Some(feature.clone());
        feature
    }

    /// Classify emotion based on valence and arousal
    fn classify_emotion(valence: f32, arousal: f32) -> EmotionalValence {
        if arousal > 0.7 {
            if valence > 0.3 {
                EmotionalValence::Joy
            } else if valence < -0.3 {
                EmotionalValence::Anger
            } else {
                EmotionalValence::Surprise
            }
        } else if arousal < 0.3 {
            if valence > 0.3 {
                EmotionalValence::Peace
            } else if valence < -0.3 {
                EmotionalValence::Sadness
            } else {
                EmotionalValence::Neutral
            }
        } else {
            if valence < -0.5 {
                EmotionalValence::Fear
            } else {
                EmotionalValence::Neutral
            }
        }
    }
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Beat tracking state machine (for future advanced tracking)
#[allow(dead_code)]
struct BeatTracker {
    tempo_estimate: f32,
    phase: f32,
    confidence: f32,
}

impl BeatTracker {
    fn new() -> Self {
        Self {
            tempo_estimate: 120.0,
            phase: 0.0,
            confidence: 0.0,
        }
    }
}

/// Key detection using chroma features (for future advanced detection)
#[allow(dead_code)]
struct KeyDetector {
    chroma_history: Vec<[f32; 12]>,
    current_key: u8,
}

impl KeyDetector {
    fn new() -> Self {
        Self {
            chroma_history: Vec::new(),
            current_key: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_creation() {
        let extractor = FeatureExtractor::new();
        assert!(extractor.prev_frame.is_none());
    }

    #[test]
    fn test_emotion_classification() {
        assert_eq!(FeatureExtractor::classify_emotion(0.8, 0.9), EmotionalValence::Joy);
        assert_eq!(FeatureExtractor::classify_emotion(-0.8, 0.1), EmotionalValence::Sadness);
        assert_eq!(FeatureExtractor::classify_emotion(0.8, 0.1), EmotionalValence::Peace);
    }

    #[test]
    fn test_to_embedding() {
        let feature = MusicalFeatures {
            timestamp: 0.0,
            pitch: 440.0,
            midi_note: 69,
            pitch_class: 9,
            octave: 4,
            onset_strength: 0.5,
            is_beat: true,
            tempo: 120.0,
            beat_phase: 0.0,
            loudness: 0.7,
            dynamics_delta: 0.0,
            brightness: 0.6,
            roughness: 0.2,
            warmth: 0.4,
            sharpness: 0.5,
            key: 0,
            chord_type: ChordType::Major,
            tension: 0.3,
            valence: 0.5,
            arousal: 0.6,
            emotion: EmotionalValence::Joy,
            mfcc: [0.0; 13],
            chroma: [0.0; 12],
        };

        let embedding = feature.to_embedding();
        assert_eq!(embedding.len(), 64);
    }
}
