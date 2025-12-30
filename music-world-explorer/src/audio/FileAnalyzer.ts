/**
 * Audio File Analyzer
 * Analyzes uploaded audio files with real FFT beat detection
 */

export interface AnalysisResult {
  frequencyData: Uint8Array
  timeData: Uint8Array
  bass: number
  mid: number
  high: number
  rms: number
  isBeat: boolean
  beatIntensity: number
  spectralCentroid: number
  spectralFlux: number
}

export class FileAnalyzer {
  private audioContext: AudioContext | null = null
  private analyser: AnalyserNode | null = null
  private sourceNode: MediaElementAudioSourceNode | null = null
  private audioElement: HTMLAudioElement | null = null
  private frequencyData: Uint8Array = new Uint8Array(256)
  private timeData: Uint8Array = new Uint8Array(256)
  private prevSpectrum: Float32Array = new Float32Array(256)
  private beatThreshold = 0.15
  private beatDecay = 0.98
  private beatHoldTime = 0.1
  private lastBeatTime = 0
  private beatEnergy = 0
  private energyHistory: number[] = []
  private historySize = 43 // ~1 second at 43fps

  async init(): Promise<void> {
    if (this.audioContext) return
    this.audioContext = new AudioContext()

    this.analyser = this.audioContext.createAnalyser()
    this.analyser.fftSize = 512
    this.analyser.smoothingTimeConstant = 0.8

    this.frequencyData = new Uint8Array(this.analyser.frequencyBinCount)
    this.timeData = new Uint8Array(this.analyser.frequencyBinCount)
    this.prevSpectrum = new Float32Array(this.analyser.frequencyBinCount)
  }

  async loadFile(file: File): Promise<HTMLAudioElement> {
    if (!this.audioContext) await this.init()

    // Clean up previous source
    if (this.sourceNode) {
      this.sourceNode.disconnect()
    }
    if (this.audioElement) {
      this.audioElement.pause()
      URL.revokeObjectURL(this.audioElement.src)
    }

    // Create new audio element
    const url = URL.createObjectURL(file)
    this.audioElement = new Audio(url)
    this.audioElement.crossOrigin = 'anonymous'

    // Wait for the audio to be ready
    await new Promise<void>((resolve, reject) => {
      this.audioElement!.addEventListener('canplaythrough', () => resolve(), { once: true })
      this.audioElement!.addEventListener('error', (e) => reject(e), { once: true })
      this.audioElement!.load()
    })

    // Connect to analyzer
    if (this.audioContext!.state === 'suspended') {
      await this.audioContext!.resume()
    }

    this.sourceNode = this.audioContext!.createMediaElementSource(this.audioElement)
    this.sourceNode.connect(this.analyser!)
    this.analyser!.connect(this.audioContext!.destination)

    return this.audioElement
  }

  play(): void {
    if (this.audioElement) {
      this.audioElement.play()
    }
  }

  pause(): void {
    if (this.audioElement) {
      this.audioElement.pause()
    }
  }

  stop(): void {
    if (this.audioElement) {
      this.audioElement.pause()
      this.audioElement.currentTime = 0
    }
  }

  setVolume(volume: number): void {
    if (this.audioElement) {
      this.audioElement.volume = Math.max(0, Math.min(1, volume))
    }
  }

  seek(time: number): void {
    if (this.audioElement) {
      this.audioElement.currentTime = time
    }
  }

  getDuration(): number {
    return this.audioElement?.duration || 0
  }

  getCurrentTime(): number {
    return this.audioElement?.currentTime || 0
  }

  isPlaying(): boolean {
    return this.audioElement ? !this.audioElement.paused : false
  }

  analyze(): AnalysisResult {
    if (!this.analyser) {
      return {
        frequencyData: new Uint8Array(256),
        timeData: new Uint8Array(256),
        bass: 0,
        mid: 0,
        high: 0,
        rms: 0,
        isBeat: false,
        beatIntensity: 0,
        spectralCentroid: 0,
        spectralFlux: 0,
      }
    }

    this.analyser.getByteFrequencyData(this.frequencyData as Uint8Array<ArrayBuffer>)
    this.analyser.getByteTimeDomainData(this.timeData as Uint8Array<ArrayBuffer>)

    // Calculate frequency bands
    const bassEnd = Math.floor(this.frequencyData.length * 0.1)
    const midEnd = Math.floor(this.frequencyData.length * 0.5)

    let bassSum = 0, midSum = 0, highSum = 0
    let totalWeight = 0, weightedSum = 0

    for (let i = 0; i < this.frequencyData.length; i++) {
      const value = this.frequencyData[i] / 255

      if (i < bassEnd) {
        bassSum += value
      } else if (i < midEnd) {
        midSum += value
      } else {
        highSum += value
      }

      // For spectral centroid
      weightedSum += i * value
      totalWeight += value
    }

    const bass = bassSum / bassEnd
    const mid = midSum / (midEnd - bassEnd)
    const high = highSum / (this.frequencyData.length - midEnd)

    // RMS calculation
    let rmsSum = 0
    for (let i = 0; i < this.timeData.length; i++) {
      const sample = (this.timeData[i] - 128) / 128
      rmsSum += sample * sample
    }
    const rms = Math.sqrt(rmsSum / this.timeData.length)

    // Spectral centroid (brightness)
    const spectralCentroid = totalWeight > 0 ? weightedSum / totalWeight / this.frequencyData.length : 0

    // Spectral flux (change detection)
    let fluxSum = 0
    for (let i = 0; i < this.frequencyData.length; i++) {
      const current = this.frequencyData[i] / 255
      const diff = current - this.prevSpectrum[i]
      fluxSum += diff > 0 ? diff : 0
      this.prevSpectrum[i] = current
    }
    const spectralFlux = fluxSum / this.frequencyData.length

    // Beat detection using energy history
    const currentEnergy = bass * 0.6 + rms * 0.4
    this.energyHistory.push(currentEnergy)
    if (this.energyHistory.length > this.historySize) {
      this.energyHistory.shift()
    }

    const avgEnergy = this.energyHistory.reduce((a, b) => a + b, 0) / this.energyHistory.length
    const now = this.audioContext?.currentTime || 0

    let isBeat = false
    if (currentEnergy > avgEnergy * (1 + this.beatThreshold) &&
        now - this.lastBeatTime > this.beatHoldTime) {
      isBeat = true
      this.lastBeatTime = now
      this.beatEnergy = 1
    }

    // Decay beat intensity
    this.beatEnergy *= this.beatDecay

    return {
      frequencyData: new Uint8Array(this.frequencyData),
      timeData: new Uint8Array(this.timeData),
      bass,
      mid,
      high,
      rms,
      isBeat,
      beatIntensity: this.beatEnergy,
      spectralCentroid,
      spectralFlux,
    }
  }

  cleanup(): void {
    if (this.sourceNode) {
      this.sourceNode.disconnect()
      this.sourceNode = null
    }
    if (this.audioElement) {
      this.audioElement.pause()
      URL.revokeObjectURL(this.audioElement.src)
      this.audioElement = null
    }
    if (this.analyser) {
      this.analyser.disconnect()
    }
  }
}

export const fileAnalyzer = new FileAnalyzer()
