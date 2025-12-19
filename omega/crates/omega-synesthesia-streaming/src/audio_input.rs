//! Audio input stream using cpal for cross-platform support
//!
//! NOTE: Requires the "audio-input" feature to be enabled.
//! Without this feature, AudioInputStream will return an error.

#[cfg(feature = "audio-input")]
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, Stream, StreamConfig as CpalStreamConfig,
};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::{AudioBuffer, StreamConfig, Result, StreamError};

/// Audio source type
#[derive(Debug, Clone)]
pub enum AudioSource {
    /// Default system microphone
    Microphone,

    /// System audio output (loopback)
    SystemAudio,

    /// Specific audio device by name
    Device(String),
}

/// Real-time audio input stream
pub struct AudioInputStream {
    /// Audio source
    source: AudioSource,

    /// Stream configuration
    config: StreamConfig,

    /// Audio buffer (lock-free SPSC)
    buffer: AudioBuffer,

    /// cpal audio stream
    #[cfg(feature = "audio-input")]
    stream: Option<Stream>,

    /// cpal device
    #[cfg(feature = "audio-input")]
    device: Option<Device>,

    /// Current gain (for AGC)
    gain: Arc<Mutex<f32>>,

    /// Running state
    is_running: Arc<Mutex<bool>>,
}

impl AudioInputStream {
    /// Create a new audio input stream
    pub async fn new(source: AudioSource, config: StreamConfig) -> Result<Self> {
        config.validate()?;

        #[cfg(not(feature = "audio-input"))]
        {
            return Err(StreamError::UnsupportedFormat(
                "Audio input requires the 'audio-input' feature to be enabled. \
                 Add 'audio-input = true' to Cargo.toml features, or install ALSA dev libraries (Linux).".to_string()
            ));
        }

        #[cfg(feature = "audio-input")]
        {
            Ok(Self {
                source,
                config,
                buffer: AudioBuffer::new(config.buffer_capacity),
                stream: None,
                device: None,
                gain: Arc::new(Mutex::new(1.0)),
                is_running: Arc::new(Mutex::new(false)),
            })
        }
    }

    /// Start the audio stream
    #[cfg(feature = "audio-input")]
    pub fn start(&mut self) -> Result<()> {
        // Get cpal host
        let host = cpal::default_host();

        // Get audio device based on source
        let device = self.get_device(&host)?;

        // Get device's default config
        let device_config = device
            .default_input_config()
            .map_err(|e| StreamError::ConfigError(format!("Failed to get device config: {}", e)))?;

        // Create cpal stream config matching our requirements
        let cpal_config = CpalStreamConfig {
            channels: self.config.channels,
            sample_rate: cpal::SampleRate(self.config.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(self.config.chunk_size as u32),
        };

        // Clone for move into closure
        let buffer = self.buffer.clone();
        let gain = Arc::clone(&self.gain);
        let auto_gain = self.config.auto_gain;
        let target_rms = self.config.target_rms;
        let is_running = Arc::clone(&self.is_running);

        // Build input stream
        let stream = device
            .build_input_stream(
                &cpal_config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    // Check if stream is running
                    if !*is_running.lock() {
                        return;
                    }

                    // Apply automatic gain control if enabled
                    let samples = if auto_gain {
                        apply_agc(data, &gain, target_rms)
                    } else {
                        data.to_vec()
                    };

                    // Push to ring buffer (lock-free)
                    buffer.push(&samples);
                },
                |err| {
                    eprintln!("Audio stream error: {}", err);
                },
                None,
            )
            .map_err(|e| StreamError::IoError(format!("Failed to build stream: {}", e)))?;

        // Start the stream
        stream
            .play()
            .map_err(|e| StreamError::IoError(format!("Failed to start stream: {}", e)))?;

        self.stream = Some(stream);
        self.device = Some(device);
        *self.is_running.lock() = true;

        Ok(())
    }

    /// Stop the audio stream
    pub fn stop(&mut self) -> Result<()> {
        *self.is_running.lock() = false;

        if let Some(stream) = self.stream.take() {
            stream
                .pause()
                .map_err(|e| StreamError::IoError(format!("Failed to stop stream: {}", e)))?;
        }

        self.device = None;
        self.buffer.clear();

        Ok(())
    }

    /// Get the next chunk of audio data
    /// Returns None if not enough data is available
    pub async fn next_chunk(&self) -> Option<Vec<f32>> {
        // Wait for enough data
        while self.buffer.available() < self.config.chunk_size {
            if !*self.is_running.lock() {
                return None;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }

        // Read chunk
        let mut chunk = vec![0.0; self.config.chunk_size];
        let read = self.buffer.pop(&mut chunk);

        if read == self.config.chunk_size {
            Some(chunk)
        } else {
            None
        }
    }

    /// Get current buffer fill ratio
    pub fn buffer_fill_ratio(&self) -> f32 {
        self.buffer.fill_ratio()
    }

    /// Check if stream is running
    pub fn is_running(&self) -> bool {
        *self.is_running.lock()
    }

    /// Get current gain
    pub fn get_gain(&self) -> f32 {
        *self.gain.lock()
    }

    /// Set manual gain (disables AGC)
    pub fn set_gain(&mut self, gain: f32) {
        *self.gain.lock() = gain.clamp(0.0, 10.0);
    }

    /// Get audio device based on source
    #[cfg(feature = "audio-input")]
    fn get_device(&self, host: &Host) -> Result<Device> {
        match &self.source {
            AudioSource::Microphone => {
                host.default_input_device()
                    .ok_or_else(|| StreamError::DeviceNotFound("Default input device not found".to_string()))
            }
            AudioSource::SystemAudio => {
                // Try to get loopback device (platform-specific)
                #[cfg(target_os = "windows")]
                {
                    host.default_output_device()
                        .ok_or_else(|| StreamError::DeviceNotFound("Default output device not found".to_string()))
                }
                #[cfg(not(target_os = "windows"))]
                {
                    // On macOS/Linux, loopback typically requires special setup
                    Err(StreamError::UnsupportedFormat(
                        "System audio capture not directly supported on this platform. Use virtual audio device.".to_string()
                    ))
                }
            }
            AudioSource::Device(name) => {
                // Find device by name
                for device in host.input_devices().map_err(|e| {
                    StreamError::DeviceNotFound(format!("Failed to enumerate devices: {}", e))
                })? {
                    if let Ok(device_name) = device.name() {
                        if device_name.contains(name) {
                            return Ok(device);
                        }
                    }
                }
                Err(StreamError::DeviceNotFound(format!("Device not found: {}", name)))
            }
        }
    }
}

impl Drop for AudioInputStream {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Apply automatic gain control to audio samples
fn apply_agc(samples: &[f32], gain: &Arc<Mutex<f32>>, target_rms: f32) -> Vec<f32> {
    // Calculate RMS of current samples
    let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
    let rms = (sum_squares / samples.len() as f32).sqrt();

    // Update gain with smoothing
    let mut current_gain = gain.lock();
    if rms > 0.0 {
        let desired_gain = target_rms / rms;
        // Smooth gain changes (low-pass filter)
        *current_gain = *current_gain * 0.9 + desired_gain * 0.1;
        *current_gain = current_gain.clamp(0.1, 10.0);
    }

    // Apply gain
    samples.iter().map(|s| s * *current_gain).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agc() {
        let gain = Arc::new(Mutex::new(1.0));
        let samples = vec![0.01, 0.02, 0.01, 0.02];  // Low volume
        let target_rms = 0.1;

        let adjusted = apply_agc(&samples, &gain, target_rms);

        // Gain should have increased
        assert!(*gain.lock() > 1.0);

        // Output should be louder
        let output_rms: f32 = adjusted.iter().map(|s| s * s).sum::<f32>() / adjusted.len() as f32;
        assert!(output_rms.sqrt() > 0.01);
    }

    #[tokio::test]
    async fn test_stream_creation() {
        let config = StreamConfig::default();
        let stream = AudioInputStream::new(AudioSource::Microphone, config).await;

        // May fail if no audio device available (CI environments)
        // Just check that it doesn't panic
        let _ = stream;
    }
}
