/**
 * Microphone Audio Capture using Web Audio API
 */

export class MicrophoneCapture {
  private audioContext: AudioContext | null = null
  private analyser: AnalyserNode | null = null
  private microphone: MediaStreamAudioSourceNode | null = null
  private gainNode: GainNode | null = null
  private dataArray: Float32Array | null = null
  private isCapturing: boolean = false

  async initialize(): Promise<void> {
    try {
      // Request microphone access
      const stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: false,
          noiseSuppression: false,
          autoGainControl: false,
        }
      })

      console.log('🎤 Microphone stream obtained, tracks:', stream.getAudioTracks().length)

      // Create audio context
      this.audioContext = new AudioContext({ sampleRate: 44100 })
      console.log('🔊 AudioContext state:', this.audioContext.state)

      // Resume context if suspended (Chrome requires this after user interaction)
      if (this.audioContext.state === 'suspended') {
        await this.audioContext.resume()
        console.log('▶️ AudioContext resumed, new state:', this.audioContext.state)
      }

      // Create analyser node
      this.analyser = this.audioContext.createAnalyser()
      this.analyser.fftSize = 2048
      this.analyser.smoothingTimeConstant = 0.3 // Reduced from 0.8 for more responsive

      // Create gain node for amplification (10x boost!)
      this.gainNode = this.audioContext.createGain()
      this.gainNode.gain.value = 10.0 // Amplify microphone by 10x

      // Create microphone source
      this.microphone = this.audioContext.createMediaStreamSource(stream)

      // Connect: microphone → gain → analyser
      this.microphone.connect(this.gainNode)
      this.gainNode.connect(this.analyser)

      // Create data array for samples
      this.dataArray = new Float32Array(this.analyser.frequencyBinCount)

      this.isCapturing = true

      // Verify stream is active
      const audioTrack = stream.getAudioTracks()[0]
      console.log('✅ Microphone initialized - Track enabled:', audioTrack.enabled, 'readyState:', audioTrack.readyState)
    } catch (error) {
      console.error('❌ Microphone access denied:', error)
      throw new Error('Microphone access denied. Please allow microphone access.')
    }
  }

  getTimeDomainData(): Float32Array | null {
    if (!this.analyser || !this.dataArray || !this.isCapturing) {
      console.warn('⚠️ Mic not ready - analyser:', !!this.analyser, 'dataArray:', !!this.dataArray, 'capturing:', this.isCapturing)
      return null
    }

    // Check if audio context is running
    if (this.audioContext && this.audioContext.state !== 'running') {
      console.warn('⚠️ AudioContext not running, state:', this.audioContext.state)
      this.audioContext.resume()
    }

    this.analyser.getFloatTimeDomainData(this.dataArray)

    // Debug: Calculate RMS to verify we're getting audio
    let sum = 0
    for (let i = 0; i < Math.min(512, this.dataArray.length); i++) {
      sum += this.dataArray[i] * this.dataArray[i]
    }
    const rms = Math.sqrt(sum / Math.min(512, this.dataArray.length))

    // Only log if there's significant audio (reduce spam)
    if (rms > 0.001) {
      console.log('🎤 Mic RMS:', rms.toFixed(4))
    }

    return this.dataArray
  }

  getFrequencyData(): Float32Array | null {
    if (!this.analyser || !this.dataArray || !this.isCapturing) {
      return null
    }

    this.analyser.getFloatFrequencyData(this.dataArray)
    return this.dataArray
  }

  stop(): void {
    if (this.microphone) {
      this.microphone.disconnect()
      this.microphone = null
    }

    if (this.gainNode) {
      this.gainNode.disconnect()
      this.gainNode = null
    }

    if (this.audioContext) {
      this.audioContext.close()
      this.audioContext = null
    }

    this.isCapturing = false
    console.log('🛑 Microphone capture stopped')
  }

  setGain(value: number): void {
    if (this.gainNode) {
      this.gainNode.gain.value = value
      console.log('🎚️ Microphone gain set to:', value)
    }
  }

  isActive(): boolean {
    return this.isCapturing
  }

  getSampleRate(): number {
    return this.audioContext?.sampleRate || 44100
  }
}
