/**
 * Audio Engine - Procedural audio generation with beat detection
 * Creates stunning reactive audio without requiring microphone
 */

import type { AudioFeatures } from '../store'

export class AudioEngine {
  private audioContext: AudioContext | null = null
  private analyser: AnalyserNode | null = null
  private oscillators: OscillatorNode[] = []
  private gains: GainNode[] = []
  private time: number = 0
  private lastBeatTime: number = 0
  private beatThreshold: number = 0.6
  private prevRms: number = 0

  // Procedural music parameters by genre
  private genreParams = {
    electronic: {
      bpm: 128,
      baseFreqs: [55, 110, 220, 440, 880],
      beatStrength: 0.9,
      bassWeight: 0.8,
    },
    classical: {
      bpm: 72,
      baseFreqs: [261.63, 329.63, 392.00, 523.25],
      beatStrength: 0.4,
      bassWeight: 0.3,
    },
    metal: {
      bpm: 160,
      baseFreqs: [82.41, 110, 164.81, 220],
      beatStrength: 1.0,
      bassWeight: 0.9,
    },
    ambient: {
      bpm: 60,
      baseFreqs: [130.81, 196.00, 261.63, 392.00],
      beatStrength: 0.2,
      bassWeight: 0.4,
    },
    jazz: {
      bpm: 96,
      baseFreqs: [146.83, 185.00, 220.00, 277.18, 349.23],
      beatStrength: 0.6,
      bassWeight: 0.5,
    },
  }

  private currentGenre: keyof typeof this.genreParams = 'electronic'

  constructor() {
    // AudioContext created on user interaction
  }

  setGenre(genre: keyof typeof this.genreParams) {
    this.currentGenre = genre
  }

  /**
   * Generate audio features procedurally (no actual audio needed)
   * This creates rich, reactive data for visualization
   */
  generateFeatures(deltaTime: number): AudioFeatures {
    this.time += deltaTime

    const params = this.genreParams[this.currentGenre]
    const beatInterval = 60 / params.bpm

    // Beat detection simulation
    const timeSinceBeat = this.time - this.lastBeatTime
    const isBeat = timeSinceBeat >= beatInterval

    if (isBeat) {
      this.lastBeatTime = this.time
    }

    // Create smooth oscillating values
    const t = this.time

    // Bass - slow, powerful oscillation
    const bass = 0.3 + 0.4 * Math.sin(t * 0.5) * Math.sin(t * 0.7)
      + (isBeat ? 0.3 * params.bassWeight : 0)

    // Mid - medium frequency movement
    const mid = 0.3 + 0.3 * Math.sin(t * 1.2) * Math.cos(t * 0.8)
      + 0.2 * Math.sin(t * 2.1)

    // High - faster, more erratic
    const high = 0.2 + 0.3 * Math.sin(t * 3.5) * Math.sin(t * 2.7)
      + 0.15 * Math.cos(t * 5.2)

    // RMS - overall energy
    const rms = (bass * params.bassWeight + mid * 0.5 + high * 0.3) / 1.5
      + (isBeat ? 0.2 * params.beatStrength : 0)

    // Beat intensity decay
    const beatDecay = Math.max(0, 1 - timeSinceBeat / (beatInterval * 0.5))
    const beatIntensity = isBeat ? params.beatStrength : beatDecay * params.beatStrength * 0.5

    // Spectral centroid (brightness) - varies with genre and time
    const basecentroid = this.currentGenre === 'electronic' ? 3000 :
      this.currentGenre === 'metal' ? 2500 :
      this.currentGenre === 'classical' ? 1500 :
      this.currentGenre === 'jazz' ? 2000 : 1000

    const spectralCentroid = basecentroid + 1000 * Math.sin(t * 0.3) + 500 * high

    // Spectral flux - rate of change
    const spectralFlux = Math.abs(rms - this.prevRms) * 10 + 0.1 * Math.random()
    this.prevRms = rms

    // Generate 32 frequency bands
    const frequencyBands: number[] = []
    for (let i = 0; i < 32; i++) {
      const bandFreq = 0.1 + i * 0.3
      const bandValue =
        (i < 8 ? bass : i < 20 ? mid : high) *
        (0.5 + 0.5 * Math.sin(t * bandFreq + i * 0.5)) *
        (isBeat && i < 12 ? 1.3 : 1)

      frequencyBands.push(Math.min(1, Math.max(0, bandValue)))
    }

    return {
      rms: Math.min(1, Math.max(0, rms)),
      bass: Math.min(1, Math.max(0, bass)),
      mid: Math.min(1, Math.max(0, mid)),
      high: Math.min(1, Math.max(0, high)),
      isBeat,
      beatIntensity: Math.min(1, Math.max(0, beatIntensity)),
      bpm: params.bpm,
      spectralCentroid,
      spectralFlux: Math.min(1, Math.max(0, spectralFlux)),
      frequencyBands,
      time: this.time,
    }
  }

  reset() {
    this.time = 0
    this.lastBeatTime = 0
    this.prevRms = 0
  }
}

export const audioEngine = new AudioEngine()
