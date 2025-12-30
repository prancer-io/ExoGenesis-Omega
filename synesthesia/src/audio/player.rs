//! Audio Player
//!
//! Handles audio file loading and playback.

use anyhow::Result;
use crossbeam_channel::Sender;
use std::sync::Arc;
use parking_lot::RwLock;

use super::{AudioAnalyzer, AudioFeatures, TranscribedWord, SAMPLE_RATE};

/// Audio player with integrated analysis
pub struct AudioPlayer {
    analyzer: AudioAnalyzer,
    current_features: Arc<RwLock<Option<AudioFeatures>>>,
    word_tx: Sender<TranscribedWord>,
    is_playing: bool,
    current_time: f64,
}

impl AudioPlayer {
    /// Create a new audio player
    pub fn new(word_tx: Sender<TranscribedWord>) -> Result<Self> {
        Ok(Self {
            analyzer: AudioAnalyzer::new(SAMPLE_RATE),
            current_features: Arc::new(RwLock::new(None)),
            word_tx,
            is_playing: false,
            current_time: 0.0,
        })
    }

    /// Load an audio file
    pub fn load(&mut self, path: &str) -> Result<()> {
        log::info!("Loading audio file: {}", path);
        // TODO: Implement actual audio loading with symphonia
        // For now, this is a stub
        Ok(())
    }

    /// Start playback
    pub fn play(&mut self) {
        log::info!("Starting playback");
        self.is_playing = true;
    }

    /// Pause playback
    pub fn pause(&mut self) {
        log::info!("Pausing playback");
        self.is_playing = false;
    }

    /// Stop playback
    pub fn stop(&mut self) {
        log::info!("Stopping playback");
        self.is_playing = false;
        self.current_time = 0.0;
    }

    /// Get current audio features
    pub fn get_features(&self) -> Option<AudioFeatures> {
        self.current_features.read().clone()
    }

    /// Process audio samples (called from audio thread)
    pub fn process_samples(&mut self, samples: &[f32], delta_time: f64) {
        if !self.is_playing {
            return;
        }

        self.current_time += delta_time;
        let features = self.analyzer.analyze(samples, self.current_time);
        *self.current_features.write() = Some(features);
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Get current playback time
    pub fn current_time(&self) -> f64 {
        self.current_time
    }
}
