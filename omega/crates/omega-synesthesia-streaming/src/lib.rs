//! # omega-synesthesia-streaming
//!
//! Real-time audio streaming for omega-synesthesia with <25ms latency.
//!
//! This crate provides cross-platform audio input capabilities using `cpal`
//! and lock-free ring buffers for minimal latency real-time processing.
//!
//! ## Features
//!
//! - **Multi-source audio input**: Microphone, system audio, files, URLs
//! - **Lock-free streaming**: SPSC ring buffer for <25ms latency
//! - **Real-time feature extraction**: FFT, onset detection, beat tracking
//! - **Cross-platform**: Windows, macOS, Linux, WASM
//!
//! ## Example
//!
//! ```no_run
//! use omega_synesthesia_streaming::{AudioInputStream, AudioSource, StreamConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create audio input stream from microphone
//! let config = StreamConfig::default();
//! let mut stream = AudioInputStream::new(AudioSource::Microphone, config).await?;
//!
//! // Start streaming
//! stream.start()?;
//!
//! // Process audio chunks in real-time
//! while let Some(chunk) = stream.next_chunk().await {
//!     // Chunk contains 512 samples (~11.6ms @ 44.1kHz)
//!     println!("Received {} samples", chunk.len());
//! }
//! # Ok(())
//! # }
//! ```

mod audio_input;
mod config;
mod feature_extractor;
mod buffer;

pub use audio_input::{AudioInputStream, AudioSource};
pub use config::StreamConfig;
pub use feature_extractor::{FeatureExtractor, StreamingFeatures};
pub use buffer::AudioBuffer;

use thiserror::Error;

/// Errors that can occur during audio streaming
#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Audio device not found: {0}")]
    DeviceNotFound(String),

    #[error("Audio format not supported: {0}")]
    UnsupportedFormat(String),

    #[error("Stream configuration error: {0}")]
    ConfigError(String),

    #[error("Stream I/O error: {0}")]
    IoError(String),

    #[error("Buffer overflow - audio processing too slow")]
    BufferOverflow,

    #[error("Buffer underflow - no audio data available")]
    BufferUnderflow,

    #[error("FFT processing error: {0}")]
    FftError(String),
}

pub type Result<T> = std::result::Result<T, StreamError>;
