//! Audio Module
//!
//! Handles audio playback, feature extraction, and transcription.

mod player;
mod analyzer;
mod whisper;

pub use player::AudioPlayer;
pub use analyzer::{AudioFeatures, AudioAnalyzer};
pub use whisper::TranscribedWord;

/// Default audio sample rate
pub const SAMPLE_RATE: u32 = 44100;

/// Audio buffer size for analysis
pub const BUFFER_SIZE: usize = 2048;
