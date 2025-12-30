/**
 * Synthesized Audio Engine
 * Generates genre-specific procedural music using Web Audio API
 */

import type { Genre } from '../store'

interface SynthConfig {
  bpm: number
  baseFreq: number
  scale: number[]
  kickPattern: number[]
  bassPattern: number[]
  padNotes: number[]
  leadPattern: number[]
  useArpeggio: boolean
  reverbAmount: number
  filterFreq: number
  distortion: number
}

const synthConfigs: Record<Genre, SynthConfig> = {
  electronic: {
    bpm: 128,
    baseFreq: 55,
    scale: [0, 3, 5, 7, 10, 12, 15], // Minor pentatonic
    kickPattern: [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
    bassPattern: [1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0],
    padNotes: [0, 7, 12, 15],
    leadPattern: [1, 0, 1, 1, 0, 1, 0, 1],
    useArpeggio: true,
    reverbAmount: 0.3,
    filterFreq: 2000,
    distortion: 0.1,
  },
  classical: {
    bpm: 72,
    baseFreq: 65.41, // C2
    scale: [0, 2, 4, 5, 7, 9, 11, 12], // Major scale
    kickPattern: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    bassPattern: [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    padNotes: [0, 4, 7, 12, 16],
    leadPattern: [1, 0, 0, 1, 0, 0, 1, 0],
    useArpeggio: false,
    reverbAmount: 0.7,
    filterFreq: 8000,
    distortion: 0,
  },
  metal: {
    bpm: 140,
    baseFreq: 41.2, // E1
    scale: [0, 1, 3, 5, 6, 8, 10, 12], // Phrygian
    kickPattern: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1],
    bassPattern: [1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1],
    padNotes: [0, 7],
    leadPattern: [1, 1, 1, 0, 1, 1, 1, 0],
    useArpeggio: false,
    reverbAmount: 0.2,
    filterFreq: 4000,
    distortion: 0.8,
  },
  ambient: {
    bpm: 60,
    baseFreq: 55,
    scale: [0, 2, 4, 7, 9, 12, 14, 16], // Major pentatonic extended
    kickPattern: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    bassPattern: [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    padNotes: [0, 4, 7, 11, 14, 16],
    leadPattern: [0, 0, 1, 0, 0, 0, 0, 0],
    useArpeggio: false,
    reverbAmount: 0.9,
    filterFreq: 1500,
    distortion: 0,
  },
  jazz: {
    bpm: 95,
    baseFreq: 55,
    scale: [0, 2, 3, 5, 7, 9, 10, 12], // Dorian
    kickPattern: [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    bassPattern: [1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0],
    padNotes: [0, 3, 7, 10, 14],
    leadPattern: [1, 0, 1, 0, 0, 1, 1, 0],
    useArpeggio: false,
    reverbAmount: 0.5,
    filterFreq: 6000,
    distortion: 0.05,
  },
}

export class SynthEngine {
  private audioContext: AudioContext | null = null
  private masterGain: GainNode | null = null
  private compressor: DynamicsCompressorNode | null = null
  private reverb: ConvolverNode | null = null
  private analyser: AnalyserNode | null = null
  private genre: Genre = 'electronic'
  private isPlaying = false
  private schedulerTimer: number | null = null
  private currentStep = 0
  private nextNoteTime = 0
  private scheduleAheadTime = 0.1
  private lookAhead = 25 // ms

  // Oscillators
  private activeOscillators: OscillatorNode[] = []

  // Analysis data
  private frequencyData: Uint8Array = new Uint8Array(256)

  async init(): Promise<void> {
    if (this.audioContext) return

    this.audioContext = new AudioContext()

    // Create master chain
    this.masterGain = this.audioContext.createGain()
    this.masterGain.gain.value = 0.5

    this.compressor = this.audioContext.createDynamicsCompressor()
    this.compressor.threshold.value = -24
    this.compressor.knee.value = 30
    this.compressor.ratio.value = 12
    this.compressor.attack.value = 0.003
    this.compressor.release.value = 0.25

    // Create analyser for visualization
    this.analyser = this.audioContext.createAnalyser()
    this.analyser.fftSize = 512
    this.frequencyData = new Uint8Array(this.analyser.frequencyBinCount)

    // Create reverb
    this.reverb = await this.createReverb()

    // Connect chain
    this.masterGain.connect(this.compressor)
    this.compressor.connect(this.analyser)
    this.analyser.connect(this.audioContext.destination)
  }

  private async createReverb(): Promise<ConvolverNode> {
    const reverb = this.audioContext!.createConvolver()
    const sampleRate = this.audioContext!.sampleRate
    const length = sampleRate * 2 // 2 seconds
    const impulse = this.audioContext!.createBuffer(2, length, sampleRate)

    for (let channel = 0; channel < 2; channel++) {
      const channelData = impulse.getChannelData(channel)
      for (let i = 0; i < length; i++) {
        channelData[i] = (Math.random() * 2 - 1) * Math.pow(1 - i / length, 2)
      }
    }

    reverb.buffer = impulse
    return reverb
  }

  setGenre(genre: Genre): void {
    this.genre = genre
  }

  start(): void {
    if (!this.audioContext) return
    if (this.audioContext.state === 'suspended') {
      this.audioContext.resume()
    }

    this.isPlaying = true
    this.currentStep = 0
    this.nextNoteTime = this.audioContext.currentTime
    this.scheduler()
  }

  stop(): void {
    this.isPlaying = false
    if (this.schedulerTimer) {
      clearTimeout(this.schedulerTimer)
      this.schedulerTimer = null
    }

    // Stop all oscillators
    this.activeOscillators.forEach(osc => {
      try { osc.stop() } catch {}
    })
    this.activeOscillators = []
  }

  private scheduler(): void {
    if (!this.isPlaying || !this.audioContext) return

    while (this.nextNoteTime < this.audioContext.currentTime + this.scheduleAheadTime) {
      this.scheduleNote(this.currentStep, this.nextNoteTime)
      this.advanceNote()
    }

    this.schedulerTimer = window.setTimeout(() => this.scheduler(), this.lookAhead)
  }

  private advanceNote(): void {
    const config = synthConfigs[this.genre]
    const secondsPerBeat = 60.0 / config.bpm / 4 // 16th notes
    this.nextNoteTime += secondsPerBeat
    this.currentStep = (this.currentStep + 1) % 16
  }

  private scheduleNote(step: number, time: number): void {
    const config = synthConfigs[this.genre]

    // Kick drum
    if (config.kickPattern[step]) {
      this.playKick(time)
    }

    // Bass
    if (config.bassPattern[step]) {
      const noteIndex = Math.floor(Math.random() * 3)
      const freq = config.baseFreq * Math.pow(2, config.scale[noteIndex] / 12)
      this.playBass(freq, time)
    }

    // Lead/melody (every 2 steps)
    if (step % 2 === 0 && config.leadPattern[step / 2]) {
      const noteIndex = 2 + Math.floor(Math.random() * 4)
      const freq = config.baseFreq * 2 * Math.pow(2, config.scale[noteIndex] / 12)
      this.playLead(freq, time)
    }

    // Pad (chord, sustained)
    if (step === 0) {
      this.playPad(time, config)
    }

    // Arpeggio for electronic
    if (config.useArpeggio && step % 4 === 0) {
      const arpIndex = (step / 4) % config.padNotes.length
      const freq = config.baseFreq * 2 * Math.pow(2, config.padNotes[arpIndex] / 12)
      this.playArpeggio(freq, time)
    }
  }

  private playKick(time: number): void {
    if (!this.audioContext || !this.masterGain) return

    const osc = this.audioContext.createOscillator()
    const gain = this.audioContext.createGain()

    osc.type = 'sine'
    osc.frequency.setValueAtTime(150, time)
    osc.frequency.exponentialRampToValueAtTime(40, time + 0.1)

    gain.gain.setValueAtTime(0.8, time)
    gain.gain.exponentialRampToValueAtTime(0.01, time + 0.3)

    osc.connect(gain)
    gain.connect(this.masterGain)

    osc.start(time)
    osc.stop(time + 0.3)
    this.activeOscillators.push(osc)
  }

  private playBass(freq: number, time: number): void {
    if (!this.audioContext || !this.masterGain) return
    const config = synthConfigs[this.genre]

    const osc = this.audioContext.createOscillator()
    const gain = this.audioContext.createGain()
    const filter = this.audioContext.createBiquadFilter()

    osc.type = config.distortion > 0.5 ? 'sawtooth' : 'triangle'
    osc.frequency.setValueAtTime(freq, time)

    filter.type = 'lowpass'
    filter.frequency.setValueAtTime(config.filterFreq * 0.5, time)

    gain.gain.setValueAtTime(0.4, time)
    gain.gain.exponentialRampToValueAtTime(0.01, time + 0.2)

    osc.connect(filter)
    filter.connect(gain)
    gain.connect(this.masterGain)

    osc.start(time)
    osc.stop(time + 0.2)
    this.activeOscillators.push(osc)
  }

  private playLead(freq: number, time: number): void {
    if (!this.audioContext || !this.masterGain || !this.reverb) return
    const config = synthConfigs[this.genre]

    const osc = this.audioContext.createOscillator()
    const gain = this.audioContext.createGain()
    const filter = this.audioContext.createBiquadFilter()
    const reverbGain = this.audioContext.createGain()

    osc.type = this.genre === 'classical' ? 'sine' : 'square'
    osc.frequency.setValueAtTime(freq, time)

    filter.type = 'lowpass'
    filter.frequency.setValueAtTime(config.filterFreq, time)
    filter.frequency.exponentialRampToValueAtTime(config.filterFreq * 0.3, time + 0.3)

    gain.gain.setValueAtTime(0, time)
    gain.gain.linearRampToValueAtTime(0.15, time + 0.02)
    gain.gain.exponentialRampToValueAtTime(0.01, time + 0.3)

    reverbGain.gain.value = config.reverbAmount

    osc.connect(filter)
    filter.connect(gain)
    gain.connect(this.masterGain)
    gain.connect(reverbGain)
    reverbGain.connect(this.reverb)
    this.reverb.connect(this.masterGain)

    osc.start(time)
    osc.stop(time + 0.3)
    this.activeOscillators.push(osc)
  }

  private playPad(time: number, config: SynthConfig): void {
    if (!this.audioContext || !this.masterGain || !this.reverb) return

    const duration = 60 / config.bpm * 4 // 4 beats

    config.padNotes.forEach((note, i) => {
      const osc = this.audioContext!.createOscillator()
      const gain = this.audioContext!.createGain()
      const filter = this.audioContext!.createBiquadFilter()
      const reverbGain = this.audioContext!.createGain()

      const freq = config.baseFreq * Math.pow(2, note / 12)

      osc.type = 'sine'
      osc.frequency.setValueAtTime(freq, time)
      osc.detune.setValueAtTime((Math.random() - 0.5) * 10, time)

      filter.type = 'lowpass'
      filter.frequency.setValueAtTime(config.filterFreq * 0.5, time)

      gain.gain.setValueAtTime(0, time)
      gain.gain.linearRampToValueAtTime(0.08 / config.padNotes.length, time + 0.5)
      gain.gain.setValueAtTime(0.08 / config.padNotes.length, time + duration - 0.5)
      gain.gain.linearRampToValueAtTime(0.001, time + duration)

      reverbGain.gain.value = config.reverbAmount

      osc.connect(filter)
      filter.connect(gain)
      gain.connect(reverbGain)
      reverbGain.connect(this.reverb!)
      this.reverb!.connect(this.masterGain!)
      gain.connect(this.masterGain!)

      osc.start(time)
      osc.stop(time + duration)
      this.activeOscillators.push(osc)
    })
  }

  private playArpeggio(freq: number, time: number): void {
    if (!this.audioContext || !this.masterGain) return

    const osc = this.audioContext.createOscillator()
    const gain = this.audioContext.createGain()
    const filter = this.audioContext.createBiquadFilter()

    osc.type = 'sawtooth'
    osc.frequency.setValueAtTime(freq, time)

    filter.type = 'lowpass'
    filter.frequency.setValueAtTime(4000, time)
    filter.frequency.exponentialRampToValueAtTime(500, time + 0.15)

    gain.gain.setValueAtTime(0.1, time)
    gain.gain.exponentialRampToValueAtTime(0.01, time + 0.15)

    osc.connect(filter)
    filter.connect(gain)
    gain.connect(this.masterGain)

    osc.start(time)
    osc.stop(time + 0.15)
    this.activeOscillators.push(osc)
  }

  getAnalysisData(): { frequencyData: Uint8Array; bass: number; mid: number; high: number } {
    if (!this.analyser) {
      return { frequencyData: new Uint8Array(256), bass: 0, mid: 0, high: 0 }
    }

    const tempData = new Uint8Array(this.frequencyData.length)
    this.analyser.getByteFrequencyData(tempData)
    // Copy to our stored array
    this.frequencyData.set(tempData)

    // Calculate band averages
    const bassEnd = Math.floor(this.frequencyData.length * 0.1)
    const midEnd = Math.floor(this.frequencyData.length * 0.5)

    let bassSum = 0, midSum = 0, highSum = 0

    for (let i = 0; i < bassEnd; i++) {
      bassSum += this.frequencyData[i]
    }
    for (let i = bassEnd; i < midEnd; i++) {
      midSum += this.frequencyData[i]
    }
    for (let i = midEnd; i < this.frequencyData.length; i++) {
      highSum += this.frequencyData[i]
    }

    return {
      frequencyData: new Uint8Array(this.frequencyData),
      bass: bassSum / bassEnd / 255,
      mid: midSum / (midEnd - bassEnd) / 255,
      high: highSum / (this.frequencyData.length - midEnd) / 255,
    }
  }

  setVolume(volume: number): void {
    if (this.masterGain) {
      this.masterGain.gain.value = Math.max(0, Math.min(1, volume))
    }
  }

  isInitialized(): boolean {
    return this.audioContext !== null
  }
}

export const synthEngine = new SynthEngine()
