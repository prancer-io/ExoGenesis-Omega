/**
 * Audio Processor - Simulates omega-synesthesia feature extraction
 * Supports both generated audio and real microphone input
 */

export type AudioSource = 'generated' | 'microphone' | 'file'

export class AudioProcessor {
  private sampleRate: number = 44100
  private chunkSize: number = 512
  private sampleCounter: number = 0
  private frequencyOffset: number = 0
  private audioSource: AudioSource = 'generated'

  constructor(audioSource: AudioSource = 'generated') {
    this.audioSource = audioSource
    this.reset()
  }

  reset() {
    this.sampleCounter = 0
    this.frequencyOffset = 0
  }

  setAudioSource(source: AudioSource) {
    this.audioSource = source
  }

  getAudioSource(): AudioSource {
    return this.audioSource
  }

  /**
   * Generate synthetic audio samples with MORE VARIATION
   */
  private generateSamples(): Float32Array {
    const samples = new Float32Array(this.chunkSize)

    // Create much more varied frequency content
    const baseFreq = 220 + this.frequencyOffset
    const time = this.sampleCounter / this.sampleRate

    for (let i = 0; i < this.chunkSize; i++) {
      const t = (this.sampleCounter + i) / this.sampleRate

      // Create musical pattern with more variation
      const fundamental = Math.sin(2 * Math.PI * baseFreq * t)
      const harmonic2 = 0.5 * Math.sin(2 * Math.PI * baseFreq * 2 * t)
      const harmonic3 = 0.25 * Math.sin(2 * Math.PI * baseFreq * 3 * t)

      // Add rhythm (120 BPM) with varying intensity
      const beatFreq = 2.0
      const beatEnvelope = Math.pow(Math.abs(Math.sin(2 * Math.PI * beatFreq * t)), 2)

      // Add much more dramatic melody variation
      const melody = Math.sin(2 * Math.PI * 0.5 * t) * 300 + Math.cos(2 * Math.PI * 0.3 * t) * 200
      const melodyTone = Math.sin(2 * Math.PI * (baseFreq + melody) * t) * 0.3

      // Add evolving amplitude envelope (gets louder and quieter over time)
      const amplitudeEnvelope = 0.5 + 0.3 * Math.sin(2 * Math.PI * 0.2 * time)

      // Add noise component for more spectral variation
      const noise = (Math.random() - 0.5) * 0.05 * Math.sin(2 * Math.PI * 10 * t)

      // Combine all components with higher amplitude
      samples[i] = ((fundamental + harmonic2 + harmonic3) * beatEnvelope * 0.6 + melodyTone * 0.4 + noise) * amplitudeEnvelope
    }

    this.sampleCounter += this.chunkSize
    // Increase frequency offset variation dramatically (was 0.5, now up to 50)
    this.frequencyOffset = (this.frequencyOffset + Math.random() * 50 + 10) % 800

    return samples
  }

  /**
   * Simplified FFT (using approximation for demo)
   */
  private simpleFft(samples: Float32Array): Float32Array {
    const fftSize = Math.min(samples.length, 512)
    const spectrum = new Float32Array(fftSize / 2)

    for (let k = 0; k < spectrum.length; k++) {
      let real = 0
      let imag = 0

      for (let n = 0; n < fftSize; n++) {
        const phase = (-2 * Math.PI * k * n) / fftSize
        real += samples[n] * Math.cos(phase)
        imag += samples[n] * Math.sin(phase)
      }

      spectrum[k] = Math.sqrt(real * real + imag * imag) / fftSize
    }

    return spectrum
  }

  /**
   * Calculate spectral centroid
   */
  private calculateCentroid(spectrum: Float32Array): number {
    let weightedSum = 0
    let sum = 0

    for (let i = 0; i < spectrum.length; i++) {
      weightedSum += i * spectrum[i]
      sum += spectrum[i]
    }

    if (sum === 0) return 1000

    const binWidth = this.sampleRate / (2 * spectrum.length)
    return (weightedSum / sum) * binWidth
  }

  /**
   * Find peak frequency
   */
  private findPeakFrequency(spectrum: Float32Array): number {
    let maxIdx = 0
    let maxVal = 0

    for (let i = 0; i < spectrum.length; i++) {
      if (spectrum[i] > maxVal) {
        maxVal = spectrum[i]
        maxIdx = i
      }
    }

    const binWidth = this.sampleRate / (2 * spectrum.length)
    return maxIdx * binWidth
  }

  /**
   * Extract all audio features (simulating omega-synesthesia)
   * Can process either generated or real microphone samples
   */
  getFeatures(microphoneSamples?: Float32Array) {
    // Use microphone samples if provided, otherwise generate synthetic audio
    const samples = microphoneSamples || this.generateSamples()
    const spectrum = this.simpleFft(samples)

    // RMS energy
    let rmsSum = 0
    for (let i = 0; i < samples.length; i++) {
      rmsSum += samples[i] * samples[i]
    }
    const rms = Math.sqrt(rmsSum / samples.length)

    // Zero crossing rate
    let zeroCrossings = 0
    for (let i = 1; i < samples.length; i++) {
      if (
        (samples[i - 1] >= 0 && samples[i] < 0) ||
        (samples[i - 1] < 0 && samples[i] >= 0)
      ) {
        zeroCrossings++
      }
    }
    const zcr = zeroCrossings / samples.length

    // Spectral flux
    let spectralFlux = 0
    for (let i = 0; i < spectrum.length; i++) {
      spectralFlux += spectrum[i]
    }
    spectralFlux /= spectrum.length

    // Beat detection (simple threshold-based)
    const beatConfidence = rms > 0.15 ? 0.9 : 0.2

    return {
      spectralCentroid: this.calculateCentroid(spectrum),
      rms,
      zcr,
      dominantFreq: this.findPeakFrequency(spectrum),
      spectralFlux,
      beatConfidence,
      tempo: 120, // Fixed for demo
    }
  }
}
