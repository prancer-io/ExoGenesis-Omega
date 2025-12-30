//! Whisper Transcription
//!
//! Real-time speech-to-text using Whisper.

use serde::{Deserialize, Serialize};

/// A transcribed word with timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscribedWord {
    /// The transcribed text
    pub text: String,

    /// Start time in seconds
    pub start_time: f64,

    /// End time in seconds
    pub end_time: f64,

    /// Confidence score 0-1
    pub confidence: f32,
}

impl TranscribedWord {
    /// Create a new transcribed word
    pub fn new(text: String, start_time: f64, end_time: f64, confidence: f32) -> Self {
        Self {
            text,
            start_time,
            end_time,
            confidence,
        }
    }
}

/// Whisper transcription engine
pub struct WhisperEngine {
    // TODO: Add whisper-rs context when implementing
    _sample_rate: u32,
}

impl WhisperEngine {
    /// Create a new Whisper engine
    pub fn new(model_path: &str) -> anyhow::Result<Self> {
        log::info!("Loading Whisper model from: {}", model_path);
        // TODO: Load actual whisper model
        Ok(Self {
            _sample_rate: 16000,
        })
    }

    /// Transcribe audio samples
    pub fn transcribe(&mut self, _samples: &[f32]) -> Vec<TranscribedWord> {
        // TODO: Implement actual transcription
        Vec::new()
    }
}
