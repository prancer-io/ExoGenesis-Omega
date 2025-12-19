# omega-synesthesia-streaming

Real-time audio streaming for omega-synesthesia with <25ms latency.

## Overview

omega-synesthesia-streaming enables live music visualization by providing:

- **Real-time audio input** from microphone, system audio, or audio files
- **Lock-free streaming** with <25ms latency using SPSC ring buffers
- **Real-time feature extraction** (FFT, onset detection, beat tracking, tempo estimation)
- **Cross-platform support** (Windows, macOS, Linux, WASM via cpal)

## Features

- 🎤 **Multi-source audio input**: Microphone, system audio (loopback), specific devices
- ⚡ **<25ms latency**: 512-sample chunks @ 44.1kHz = 11.6ms processing time
- 🔒 **Lock-free buffering**: SPSC ring buffer for real-time safety
- 🎵 **Musical feature extraction**: Spectral analysis, beat detection, tempo estimation
- 🎛️ **Automatic Gain Control**: Adaptive RMS normalization
- 🌍 **Cross-platform**: cpal supports all major platforms

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
omega-synesthesia-streaming = { path = "../omega-synesthesia-streaming" }

# Optional: Enable real-time audio input (requires system audio libraries)
[features]
audio-input = ["omega-synesthesia-streaming/audio-input"]
```

### System Requirements

**Audio input requires system audio libraries:**

- **Linux**: `sudo apt-get install libalsa-ocaml-dev pkg-config`
- **macOS**: Built-in CoreAudio (no additional packages)
- **Windows**: Built-in WASAPI (no additional packages)

**Or compile without audio input:**
```bash
cargo build --no-default-features
```

## Quick Start

### Basic Audio Streaming

```rust
use omega_synesthesia_streaming::{
    AudioInputStream, AudioSource, StreamConfig, FeatureExtractor
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create stream configuration (44.1kHz, 2 channels, 512 samples)
    let config = StreamConfig::low_latency();

    // Create audio input from microphone
    let mut stream = AudioInputStream::new(
        AudioSource::Microphone,
        config.clone()
    ).await?;

    // Create feature extractor for real-time analysis
    let mut extractor = FeatureExtractor::new(
        config.sample_rate,
        config.chunk_size
    );

    // Start streaming
    stream.start()?;

    // Process audio chunks in real-time
    while let Some(chunk) = stream.next_chunk().await {
        // Extract musical features
        let features = extractor.extract(&chunk)?;

        println!(
            "RMS: {:.3}, Centroid: {:.1} Hz, Dominant: {:.1} Hz, BPM: {:?}",
            features.rms_energy,
            features.spectral_centroid,
            features.dominant_frequency,
            features.tempo_bpm
        );
    }

    Ok(())
}
```

### System Audio Capture

```rust
use omega_synesthesia_streaming::{AudioInputStream, AudioSource, StreamConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = StreamConfig::default();

    // Capture system audio output (loopback)
    let mut stream = AudioInputStream::new(
        AudioSource::SystemAudio,
        config
    ).await?;

    stream.start()?;

    // Process what's playing on the system
    while let Some(chunk) = stream.next_chunk().await {
        // Visualize or analyze system audio
    }

    Ok(())
}
```

### Custom Device

```rust
let stream = AudioInputStream::new(
    AudioSource::Device("USB Microphone".to_string()),
    config
).await?;
```

## Architecture

```
Audio Source → cpal → Ring Buffer → Feature Extractor → Musical Features
   (Mic)       (I/O)    (Lock-free)      (FFT)          (Real-time)
```

### Audio Input Pipeline

1. **cpal audio callback** (runs on audio thread)
   - Captures audio samples from device
   - Applies automatic gain control (AGC)
   - Lock-free push to ring buffer (<0.1ms)

2. **Consumer thread** (your application)
   - Async pop from ring buffer
   - Waits for full chunk (512 samples)
   - Returns audio data for processing

3. **Feature extraction** (real-time FFT)
   - Hann window application
   - FFT computation (rustfft)
   - Spectral analysis
   - Onset detection
   - Tempo estimation

### Latency Breakdown

```
Chunk Size: 512 samples @ 44.1kHz = 11.6ms
Ring Buffer Overhead: <0.1ms (lock-free)
Feature Extraction: 8-12ms (FFT + analysis)
Total Latency: ~23ms ✅ (<25ms target)
```

## API Reference

### `StreamConfig`

Configuration for audio streaming.

```rust
pub struct StreamConfig {
    pub sample_rate: u32,       // Hz (e.g., 44100, 48000)
    pub channels: u16,          // 1 = mono, 2 = stereo
    pub chunk_size: usize,      // Samples per chunk
    pub buffer_capacity: usize, // Total ring buffer size
    pub auto_gain: bool,        // Enable AGC
    pub target_rms: f32,        // AGC target level
}

impl StreamConfig {
    pub fn low_latency() -> Self;    // <25ms latency
    pub fn high_quality() -> Self;   // Higher quality, more latency
    pub fn latency_ms(&self) -> f32; // Calculate latency
    pub fn validate(&self) -> Result<()>;
}
```

### `AudioInputStream`

Real-time audio input stream.

```rust
pub struct AudioInputStream { /* ... */ }

impl AudioInputStream {
    pub async fn new(source: AudioSource, config: StreamConfig) -> Result<Self>;
    pub fn start(&mut self) -> Result<()>;
    pub fn stop(&mut self) -> Result<()>;
    pub async fn next_chunk(&self) -> Option<Vec<f32>>;
    pub fn buffer_fill_ratio(&self) -> f32;
    pub fn is_running(&self) -> bool;
    pub fn set_gain(&mut self, gain: f32);
}
```

### `FeatureExtractor`

Real-time musical feature extraction.

```rust
pub struct FeatureExtractor { /* ... */ }

impl FeatureExtractor {
    pub fn new(sample_rate: u32, fft_size: usize) -> Self;
    pub fn extract(&mut self, samples: &[f32]) -> Result<StreamingFeatures>;
    pub fn reset(&mut self);
}

pub struct StreamingFeatures {
    pub spectral_centroid: f32,   // Brightness (Hz)
    pub rms_energy: f32,          // Loudness (0.0-1.0)
    pub zero_crossing_rate: f32,  // Noisiness
    pub dominant_frequency: f32,  // Peak frequency (Hz)
    pub spectral_flux: f32,       // Rate of change
    pub beat_confidence: f32,     // Beat strength (0.0-1.0)
    pub tempo_bpm: Option<f32>,   // Estimated tempo
    pub spectrum: Vec<f32>,       // Magnitude spectrum
    pub timestamp: u64,           // Sample counter
}
```

## Performance

### Latency Targets

| Configuration | Chunk Size | Latency | Use Case |
|---------------|------------|---------|----------|
| Low Latency   | 512 @ 44.1kHz | 11.6ms | Live performance, VR |
| Default       | 512 @ 44.1kHz | 11.6ms | Real-time visualization |
| High Quality  | 1024 @ 48kHz | 21.3ms | Studio quality, less critical timing |

### Memory Usage

```
Ring Buffer: 44,100 samples * 4 bytes = 176 KB
Feature History: 50 frames * 512 floats * 4 bytes = 100 KB
Total: ~300 KB
```

### CPU Usage

- **Audio callback**: <1% (lock-free push)
- **Feature extraction**: 5-10% (single core @ 44.1kHz)
- **Total**: <15% on modern CPUs

## Advanced Features

### Automatic Gain Control (AGC)

```rust
let mut config = StreamConfig::default();
config.auto_gain = true;
config.target_rms = 0.1;  // Target -20 dB

let mut stream = AudioInputStream::new(AudioSource::Microphone, config).await?;
```

AGC automatically adjusts input volume to maintain consistent levels.

### Beat Detection

```rust
let mut extractor = FeatureExtractor::new(44100, 512);

while let Some(chunk) = stream.next_chunk().await {
    let features = extractor.extract(&chunk)?;

    if features.beat_confidence > 0.7 {
        println!("Beat detected! Strength: {:.2}", features.beat_confidence);
    }
}
```

### Tempo Estimation

```rust
if let Some(bpm) = features.tempo_bpm {
    println!("Tempo: {:.1} BPM", bpm);
}
```

Tempo estimation uses onset interval analysis and requires ~8 beats for accuracy.

## Integration with omega-synesthesia

```rust
use omega_synesthesia::{WorldGenerator, MusicalFeatures};
use omega_synesthesia_streaming::{AudioInputStream, FeatureExtractor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = StreamConfig::low_latency();
    let mut stream = AudioInputStream::new(AudioSource::Microphone, config.clone()).await?;
    let mut extractor = FeatureExtractor::new(config.sample_rate, config.chunk_size);
    let mut world_gen = WorldGenerator::new();

    stream.start()?;

    while let Some(chunk) = stream.next_chunk().await {
        // Extract features
        let features = extractor.extract(&chunk)?;

        // Convert to MusicalFeatures for world generation
        let musical_features = MusicalFeatures {
            spectral_centroid: features.spectral_centroid,
            rms_energy: features.rms_energy,
            // ... map other fields
        };

        // Generate 3D world chunk
        let world_chunk = world_gen.generate_chunk(&musical_features)?;

        // Render or export...
    }

    Ok(())
}
```

## Examples

See `omega-examples/` for:
- `realtime_microphone.rs` - Live microphone visualization
- `system_audio_capture.rs` - Visualize what's playing
- `beat_detection.rs` - Real-time beat tracking
- `tempo_estimator.rs` - BPM detection

## Troubleshooting

### "Audio input requires the 'audio-input' feature"

Enable the feature in `Cargo.toml`:
```toml
omega-synesthesia-streaming = { version = "1.0", features = ["audio-input"] }
```

### "ALSA lib not found" (Linux)

Install ALSA development libraries:
```bash
sudo apt-get install libalsa-ocaml-dev pkg-config
```

### High CPU usage

- Increase chunk size (less frequent processing)
- Reduce sample rate (44.1kHz → 22.05kHz)
- Disable AGC if not needed

### Audio glitches/dropouts

- Increase ring buffer capacity
- Lower system audio load
- Use low-latency kernel (Linux)

## License

MIT License - See LICENSE file for details

## Contributing

Contributions welcome! Please see CONTRIBUTING.md for guidelines.

## References

- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio I/O
- [rustfft](https://github.com/ejmahler/RustFFT) - FFT implementation
- [ringbuf](https://github.com/agerasev/ringbuf) - Lock-free ring buffer
