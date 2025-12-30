import { create } from 'zustand'

export type Genre = 'electronic' | 'classical' | 'metal' | 'ambient' | 'jazz'
export type AudioMode = 'synth' | 'file'

export interface AudioFeatures {
  // Core features
  rms: number           // 0-1 overall loudness
  bass: number          // 0-1 low frequency energy
  mid: number           // 0-1 mid frequency energy
  high: number          // 0-1 high frequency energy

  // Beat detection
  isBeat: boolean       // true on beat
  beatIntensity: number // 0-1 beat strength
  bpm: number           // estimated BPM

  // Spectral
  spectralCentroid: number  // brightness (Hz)
  spectralFlux: number      // change rate

  // Frequency bands (32 bands)
  frequencyBands: number[]

  // Time
  time: number
}

export interface StoryState {
  chapterName: string
  chapterIndex: number
  chapterProgress: number
  totalChapters: number
}

interface AppState {
  // Genre
  genre: Genre
  setGenre: (genre: Genre) => void

  // Audio mode
  audioMode: AudioMode
  setAudioMode: (mode: AudioMode) => void

  // Audio file
  audioFile: File | null
  setAudioFile: (file: File | null) => void
  audioFileName: string
  setAudioFileName: (name: string) => void

  // Volume
  volume: number
  setVolume: (volume: number) => void

  // Audio
  audioFeatures: AudioFeatures
  setAudioFeatures: (features: Partial<AudioFeatures>) => void

  // Playback
  isPlaying: boolean
  setIsPlaying: (playing: boolean) => void

  // Story
  storyState: StoryState
  setStoryState: (state: Partial<StoryState>) => void

  // Camera
  cameraMode: 'orbit' | 'fly' | 'cinematic'
  setCameraMode: (mode: 'orbit' | 'fly' | 'cinematic') => void

  // Performance
  quality: 'low' | 'medium' | 'high' | 'ultra'
  setQuality: (quality: 'low' | 'medium' | 'high' | 'ultra') => void

  // UI
  showUI: boolean
  setShowUI: (show: boolean) => void
}

const defaultAudioFeatures: AudioFeatures = {
  rms: 0.3,
  bass: 0.3,
  mid: 0.3,
  high: 0.3,
  isBeat: false,
  beatIntensity: 0,
  bpm: 120,
  spectralCentroid: 2000,
  spectralFlux: 0.1,
  frequencyBands: new Array(32).fill(0.2),
  time: 0,
}

const defaultStoryState: StoryState = {
  chapterName: 'awakening',
  chapterIndex: 0,
  chapterProgress: 0,
  totalChapters: 5,
}

export const useStore = create<AppState>((set) => ({
  genre: 'electronic',
  setGenre: (genre) => set({ genre }),

  audioMode: 'synth',
  setAudioMode: (audioMode) => set({ audioMode }),

  audioFile: null,
  setAudioFile: (audioFile) => set({ audioFile }),
  audioFileName: '',
  setAudioFileName: (audioFileName) => set({ audioFileName }),

  volume: 0.7,
  setVolume: (volume) => set({ volume }),

  audioFeatures: defaultAudioFeatures,
  setAudioFeatures: (features) => set((state) => ({
    audioFeatures: { ...state.audioFeatures, ...features }
  })),

  isPlaying: true,
  setIsPlaying: (isPlaying) => set({ isPlaying }),

  storyState: defaultStoryState,
  setStoryState: (state) => set((prev) => ({
    storyState: { ...prev.storyState, ...state }
  })),

  cameraMode: 'cinematic',
  setCameraMode: (cameraMode) => set({ cameraMode }),

  quality: 'high',
  setQuality: (quality) => set({ quality }),

  showUI: true,
  setShowUI: (showUI) => set({ showUI }),
}))
