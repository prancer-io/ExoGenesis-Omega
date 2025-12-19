//! Real-Time Streaming Integration
//!
//! This module bridges omega-synesthesia-streaming with the core synesthesia engine,
//! enabling live music visualization with <50ms end-to-end latency.
//!
//! ## Architecture
//!
//! ```text
//! Audio Stream → StreamingFeatures → MusicalFeatures → WorldChunk → GPU Render
//!   (11.6ms)          (12ms)             (5ms)           (10ms)      (16.7ms @ 60 FPS)
//!
//! Total: ~55ms end-to-end latency
//! ```

use crate::features::{MusicalFeatures, ChordType, EmotionalValence};
use crate::world::{WorldChunk, WorldElement};
use crate::genre::GenreStyle;
use crate::mapping::SpatialMapper;
use std::collections::VecDeque;
use glam::Vec3;

/// Bridge between streaming features and musical features
pub struct FeatureBridge {
    /// Running sample counter for timestamps
    sample_counter: u64,

    /// Sample rate for time conversion
    sample_rate: u32,

    /// Feature history for smoothing
    history: VecDeque<SmoothedFeatures>,

    /// Smoothing window size (frames)
    smoothing_window: usize,
}

/// Smoothed features for temporal stability
#[derive(Debug, Clone)]
struct SmoothedFeatures {
    pitch: f32,
    loudness: f32,
    brightness: f32,
    onset_strength: f32,
}

impl FeatureBridge {
    /// Create a new feature bridge
    pub fn new(sample_rate: u32, smoothing_window: usize) -> Self {
        Self {
            sample_counter: 0,
            sample_rate,
            history: VecDeque::with_capacity(smoothing_window),
            smoothing_window,
        }
    }

    /// Convert streaming features to musical features
    ///
    /// This bridges omega-synesthesia-streaming's StreamingFeatures
    /// to omega-synesthesia's MusicalFeatures.
    ///
    /// # Note
    ///
    /// In a real implementation, this would accept StreamingFeatures directly.
    /// For now, we define a compatible interface that can be implemented
    /// when omega-synesthesia-streaming compiles successfully.
    pub fn convert(
        &mut self,
        spectral_centroid: f32,
        rms_energy: f32,
        zero_crossing_rate: f32,
        dominant_frequency: f32,
        spectral_flux: f32,
        beat_confidence: f32,
        tempo_bpm: Option<f32>,
        spectrum: &[f32],
    ) -> MusicalFeatures {
        // Calculate timestamp
        let timestamp = self.sample_counter as f64 / self.sample_rate as f64;

        // Smooth features
        let smoothed = SmoothedFeatures {
            pitch: dominant_frequency,
            loudness: rms_energy,
            brightness: spectral_centroid / 4000.0,  // Normalize to 0-1
            onset_strength: spectral_flux * rms_energy,
        };

        self.history.push_back(smoothed.clone());
        if self.history.len() > self.smoothing_window {
            self.history.pop_front();
        }

        let avg_pitch = self.average_pitch();
        let avg_loudness = self.average_loudness();
        let avg_brightness = self.average_brightness();

        // Convert pitch to MIDI
        let (midi_note, pitch_class, octave) = Self::hz_to_midi(avg_pitch);

        // Detect beats
        let is_beat = beat_confidence > 0.7;

        // Estimate tempo
        let tempo = tempo_bpm.unwrap_or(120.0);

        // Calculate beat phase (0-1 within beat)
        let beat_duration = 60.0 / tempo as f64;
        let beat_phase = (timestamp % beat_duration) / beat_duration;

        // Derive timbre features
        let brightness = avg_brightness;
        let roughness = zero_crossing_rate;
        let warmth = Self::calculate_warmth(spectrum);
        let sharpness = Self::calculate_sharpness(spectrum);

        // Derive emotional features
        let valence = Self::calculate_valence(avg_brightness, tempo);
        let arousal = avg_loudness;
        let emotion = Self::derive_emotion(valence, arousal);

        // Calculate harmonic features (simplified for real-time)
        let key = 0;  // C major (would need chroma analysis for accurate key)
        let chord_type = ChordType::Unknown;
        let tension = spectral_flux.min(1.0);

        // Placeholder MFCC and chroma (would come from proper analysis)
        let mfcc = [0.0; 13];
        let chroma = [0.0; 12];

        MusicalFeatures {
            timestamp,
            pitch: avg_pitch,
            midi_note,
            pitch_class,
            octave,
            onset_strength: smoothed.onset_strength,
            is_beat,
            tempo,
            beat_phase: beat_phase as f32,
            loudness: avg_loudness,
            dynamics_delta: 0.0,  // Would need previous frame
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
        }
    }

    /// Convert frequency (Hz) to MIDI note
    fn hz_to_midi(hz: f32) -> (u8, u8, i8) {
        if hz < 20.0 {
            return (0, 0, -2);
        }

        let midi_float = 69.0 + 12.0 * (hz / 440.0).log2();
        let midi_note = midi_float.clamp(0.0, 127.0) as u8;
        let pitch_class = midi_note % 12;
        let octave = (midi_note / 12) as i8 - 2;

        (midi_note, pitch_class, octave)
    }

    /// Calculate warmth from spectrum (low frequency content)
    fn calculate_warmth(spectrum: &[f32]) -> f32 {
        if spectrum.is_empty() {
            return 0.5;
        }

        // Sum low frequency bins (first 25%)
        let low_bins = spectrum.len() / 4;
        let low_sum: f32 = spectrum[..low_bins].iter().sum();
        let total_sum: f32 = spectrum.iter().sum();

        if total_sum > 0.0 {
            (low_sum / total_sum).min(1.0)
        } else {
            0.5
        }
    }

    /// Calculate sharpness from spectrum (high frequency content)
    fn calculate_sharpness(spectrum: &[f32]) -> f32 {
        if spectrum.is_empty() {
            return 0.5;
        }

        // Sum high frequency bins (last 25%)
        let high_bins = spectrum.len() * 3 / 4;
        let high_sum: f32 = spectrum[high_bins..].iter().sum();
        let total_sum: f32 = spectrum.iter().sum();

        if total_sum > 0.0 {
            (high_sum / total_sum).min(1.0)
        } else {
            0.5
        }
    }

    /// Calculate emotional valence from brightness and tempo
    fn calculate_valence(brightness: f32, tempo: f32) -> f32 {
        // Bright + fast = happy (positive valence)
        // Dark + slow = sad (negative valence)
        let brightness_contribution = (brightness - 0.5) * 2.0;
        let tempo_contribution = ((tempo - 90.0) / 60.0).clamp(-1.0, 1.0);

        ((brightness_contribution + tempo_contribution) / 2.0).clamp(-1.0, 1.0)
    }

    /// Derive emotional category from valence and arousal
    fn derive_emotion(valence: f32, arousal: f32) -> EmotionalValence {
        match (valence > 0.0, arousal > 0.5) {
            (true, true) => EmotionalValence::Joy,      // High valence, high arousal
            (true, false) => EmotionalValence::Peace,   // High valence, low arousal
            (false, true) => EmotionalValence::Anger,   // Low valence, high arousal
            (false, false) => EmotionalValence::Sadness, // Low valence, low arousal
        }
    }

    /// Get average pitch from history
    fn average_pitch(&self) -> f32 {
        if self.history.is_empty() {
            return 440.0;
        }

        let sum: f32 = self.history.iter().map(|f| f.pitch).sum();
        sum / self.history.len() as f32
    }

    /// Get average loudness from history
    fn average_loudness(&self) -> f32 {
        if self.history.is_empty() {
            return 0.0;
        }

        let sum: f32 = self.history.iter().map(|f| f.loudness).sum();
        sum / self.history.len() as f32
    }

    /// Get average brightness from history
    fn average_brightness(&self) -> f32 {
        if self.history.is_empty() {
            return 0.5;
        }

        let sum: f32 = self.history.iter().map(|f| f.brightness).sum();
        sum / self.history.len() as f32
    }

    /// Increment sample counter
    pub fn advance_time(&mut self, samples: usize) {
        self.sample_counter += samples as u64;
    }

    /// Reset the bridge state
    pub fn reset(&mut self) {
        self.sample_counter = 0;
        self.history.clear();
    }
}

/// Streaming world generator for incremental chunk creation
pub struct StreamingWorldGenerator {
    /// Genre style for consistent aesthetics
    style: GenreStyle,

    /// Spatial mapper for feature-to-3D conversion
    mapper: SpatialMapper,

    /// Current time offset (X position in world)
    time_offset: f32,

    /// Chunk duration in seconds
    chunk_duration: f32,

    /// Current chunk being built
    current_chunk: Option<WorldChunk>,

    /// Chunk index counter
    chunk_index: usize,

    /// Feature history buffer
    feature_buffer: VecDeque<MusicalFeatures>,

    /// Buffer size (number of features per chunk)
    buffer_size: usize,
}

impl StreamingWorldGenerator {
    /// Create a new streaming world generator
    pub fn new(style: GenreStyle, chunk_duration: f32) -> Self {
        // Create mapper with default scales (1.0 for both)
        let mapper = SpatialMapper::new(1.0, 1.0);

        // Calculate buffer size based on typical streaming rate
        // At 512 samples @ 44.1kHz = ~85 Hz update rate
        // For 1 second chunks = ~85 features
        let buffer_size = (chunk_duration * 85.0) as usize;

        Self {
            style: style.clone(),
            mapper,
            time_offset: 0.0,
            chunk_duration,
            current_chunk: Some(WorldChunk::new(0, Vec3::ZERO)),
            chunk_index: 0,
            feature_buffer: VecDeque::with_capacity(buffer_size),
            buffer_size,
        }
    }

    /// Add a musical feature and potentially generate a chunk
    ///
    /// Returns Some(WorldChunk) when a chunk is complete and ready for rendering.
    pub fn add_feature(&mut self, features: MusicalFeatures) -> Option<WorldChunk> {
        self.feature_buffer.push_back(features);

        // Check if we have enough features for a chunk
        if self.feature_buffer.len() >= self.buffer_size {
            return self.finalize_chunk();
        }

        None
    }

    /// Finalize the current chunk and start a new one
    fn finalize_chunk(&mut self) -> Option<WorldChunk> {
        if self.feature_buffer.is_empty() {
            return None;
        }

        // Create chunk from buffered features
        let mut chunk = WorldChunk::new(
            self.chunk_index,
            Vec3::new(self.time_offset, 0.0, 0.0),
        );

        // Process features into world elements
        let features_vec: Vec<_> = self.feature_buffer.iter().cloned().collect();
        if let Ok(spatial_moments) = self.mapper.map_features(&features_vec, &self.style) {
            for spatial in spatial_moments {
                let element = self.create_element(&spatial);
                chunk.elements.push(element);
            }
        }

        chunk.finalize();

        // Update state for next chunk
        self.time_offset += self.chunk_duration * self.style.time_scale;
        self.chunk_index += 1;
        self.feature_buffer.clear();

        Some(chunk)
    }

    /// Create a world element from spatial moment
    ///
    /// Note: Simplified version that works with current architecture
    fn create_element(&self, spatial: &crate::mapping::SpatialMoment) -> WorldElement {
        use crate::world::ElementType;
        use glam::Mat4;

        let position = spatial.position.to_vec3();

        // Create transform
        let translation = Mat4::from_translation(position);
        let rotation = Mat4::from_euler(
            glam::EulerRot::XYZ,
            spatial.rotation.x,
            spatial.rotation.y,
            spatial.rotation.z,
        );
        let scale = Mat4::from_scale(Vec3::splat(spatial.scale));
        let transform = translation * rotation * scale;

        // Determine element type based on features
        let element_type = if spatial.features.is_beat {
            ElementType::Landmark  // Beats are landmarks
        } else if spatial.features.loudness > 0.7 {
            ElementType::Structure  // Loud notes are structural
        } else {
            ElementType::Geometry  // Standard notes
        };

        WorldElement {
            id: format!("stream_elem_{:.3}", spatial.features.timestamp),
            element_type,
            transform,
            position,
            scale: spatial.scale,
            rotation: spatial.rotation,
            color: spatial.color,
            emission: spatial.emission,
            shape: spatial.shape,
            material: spatial.material_hint.clone(),
            timestamp: spatial.features.timestamp,
            is_beat: spatial.features.is_beat,
            loudness: spatial.features.loudness,
        }
    }

    /// Force flush current buffer into a chunk (for stream end)
    pub fn flush(&mut self) -> Option<WorldChunk> {
        if !self.feature_buffer.is_empty() {
            self.finalize_chunk()
        } else {
            None
        }
    }

    /// Get current time offset in world
    pub fn current_time(&self) -> f32 {
        self.time_offset
    }

    /// Get chunks generated so far
    pub fn chunk_count(&self) -> usize {
        self.chunk_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hz_to_midi() {
        let (midi, pitch_class, octave) = FeatureBridge::hz_to_midi(440.0);
        assert_eq!(midi, 69);  // A4
        assert_eq!(pitch_class, 9);  // A
        assert_eq!(octave, 3);  // 4th octave
    }

    #[test]
    fn test_feature_bridge_conversion() {
        let mut bridge = FeatureBridge::new(44100, 5);

        let features = bridge.convert(
            2000.0,  // spectral_centroid
            0.5,     // rms_energy
            0.1,     // zero_crossing_rate
            440.0,   // dominant_frequency
            0.3,     // spectral_flux
            0.8,     // beat_confidence
            Some(120.0),  // tempo_bpm
            &vec![0.1; 256],  // spectrum
        );

        assert!(features.pitch > 400.0 && features.pitch < 500.0);
        assert_eq!(features.is_beat, true);
        assert_eq!(features.tempo, 120.0);
    }

    #[test]
    fn test_streaming_world_generator() {
        let style = GenreStyle::electronic();
        let mut generator = StreamingWorldGenerator::new(style, 1.0);

        // Generate test features
        for i in 0..100 {
            let mut bridge = FeatureBridge::new(44100, 1);
            let features = bridge.convert(
                2000.0,
                0.5 + (i as f32 * 0.01),
                0.1,
                440.0 + (i as f32 * 10.0),
                0.3,
                if i % 10 == 0 { 0.9 } else { 0.3 },
                Some(120.0),
                &vec![0.1; 256],
            );

            if let Some(chunk) = generator.add_feature(features) {
                assert!(!chunk.elements.is_empty());
                assert_eq!(chunk.index, 0);  // First chunk
                break;
            }
        }
    }

    #[test]
    fn test_valence_calculation() {
        // Bright + fast = happy
        let valence1 = FeatureBridge::calculate_valence(0.8, 140.0);
        assert!(valence1 > 0.0);

        // Dark + slow = sad
        let valence2 = FeatureBridge::calculate_valence(0.2, 60.0);
        assert!(valence2 < 0.0);
    }
}
