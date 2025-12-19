//! Stream configuration for audio input

use crate::Result;

/// Configuration for audio streaming
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Sample rate in Hz (e.g., 44100, 48000)
    pub sample_rate: u32,

    /// Number of audio channels (1 = mono, 2 = stereo)
    pub channels: u16,

    /// Chunk size in samples (affects latency)
    /// 512 samples @ 44.1kHz = ~11.6ms latency
    pub chunk_size: usize,

    /// Ring buffer capacity in samples (e.g., 1 second = sample_rate)
    pub buffer_capacity: usize,

    /// Enable automatic gain control
    pub auto_gain: bool,

    /// Target RMS level for auto gain (0.0 - 1.0)
    pub target_rms: f32,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            chunk_size: 512,  // ~11.6ms @ 44.1kHz
            buffer_capacity: 44100,  // 1 second buffer
            auto_gain: true,
            target_rms: 0.1,  // -20 dB
        }
    }
}

impl StreamConfig {
    /// Create a low-latency configuration (<25ms)
    pub fn low_latency() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            chunk_size: 512,  // ~11.6ms
            buffer_capacity: 44100,
            auto_gain: true,
            target_rms: 0.1,
        }
    }

    /// Create a high-quality configuration (lower latency tolerance)
    pub fn high_quality() -> Self {
        Self {
            sample_rate: 48000,
            channels: 2,
            chunk_size: 1024,  // ~21.3ms
            buffer_capacity: 96000,  // 2 second buffer
            auto_gain: false,
            target_rms: 0.1,
        }
    }

    /// Calculate latency in milliseconds
    pub fn latency_ms(&self) -> f32 {
        (self.chunk_size as f32 / self.sample_rate as f32) * 1000.0
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        use crate::StreamError;

        if self.sample_rate < 8000 || self.sample_rate > 192000 {
            return Err(StreamError::ConfigError(
                format!("Invalid sample rate: {}", self.sample_rate)
            ));
        }

        if self.channels == 0 || self.channels > 8 {
            return Err(StreamError::ConfigError(
                format!("Invalid channel count: {}", self.channels)
            ));
        }

        if self.chunk_size < 64 || self.chunk_size > 8192 {
            return Err(StreamError::ConfigError(
                format!("Invalid chunk size: {}", self.chunk_size)
            ));
        }

        if self.buffer_capacity < self.chunk_size * 2 {
            return Err(StreamError::ConfigError(
                "Buffer capacity must be at least 2x chunk size".to_string()
            ));
        }

        if self.target_rms < 0.0 || self.target_rms > 1.0 {
            return Err(StreamError::ConfigError(
                format!("Invalid target RMS: {}", self.target_rms)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = StreamConfig::default();
        assert_eq!(config.sample_rate, 44100);
        assert_eq!(config.channels, 2);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_latency_calculation() {
        let config = StreamConfig::default();
        let latency = config.latency_ms();
        assert!((latency - 11.6).abs() < 0.1);  // ~11.6ms
    }

    #[test]
    fn test_low_latency_config() {
        let config = StreamConfig::low_latency();
        assert!(config.latency_ms() < 25.0);  // <25ms target
    }

    #[test]
    fn test_invalid_sample_rate() {
        let mut config = StreamConfig::default();
        config.sample_rate = 1000;  // Too low
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_buffer_capacity() {
        let mut config = StreamConfig::default();
        config.buffer_capacity = 100;  // Too small
        assert!(config.validate().is_err());
    }
}
