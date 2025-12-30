/**
 * Music World Explorer - Main Application
 * Transform sound into stunning 3D worlds with real audio
 */

import { useState, useEffect, useCallback, Suspense } from 'react'
import { Canvas } from '@react-three/fiber'
import { Loader } from '@react-three/drei'
import { useStore } from './store'
import { audioEngine } from './audio/AudioEngine'
import { synthEngine } from './audio/SynthEngine'
import { fileAnalyzer } from './audio/FileAnalyzer'
import { Scene } from './components/Scene'
import { UI, LoadingScreen } from './components/UI'

function App() {
  const [isLoaded, setIsLoaded] = useState(false)
  const [audioInitialized, setAudioInitialized] = useState(false)
  const genre = useStore((s) => s.genre)
  const setGenre = useStore((s) => s.setGenre)
  const setAudioFeatures = useStore((s) => s.setAudioFeatures)
  const isPlaying = useStore((s) => s.isPlaying)
  const setIsPlaying = useStore((s) => s.setIsPlaying)
  const audioMode = useStore((s) => s.audioMode)
  const audioFile = useStore((s) => s.audioFile)
  const volume = useStore((s) => s.volume)

  // Initialize audio on first user interaction
  const initAudio = useCallback(async () => {
    if (audioInitialized) return

    try {
      await synthEngine.init()
      await fileAnalyzer.init()
      setAudioInitialized(true)

      if (audioMode === 'synth') {
        synthEngine.setGenre(genre)
        synthEngine.start()
      }
    } catch (err) {
      console.error('Failed to initialize audio:', err)
    }
  }, [audioInitialized, audioMode, genre])

  // Audio engine loop
  useEffect(() => {
    let animationId: number
    let lastTime = performance.now()

    const tick = () => {
      const now = performance.now()
      const delta = (now - lastTime) / 1000
      lastTime = now

      if (isPlaying) {
        if (audioMode === 'synth' && audioInitialized) {
          // Get analysis from synthesizer
          const analysis = synthEngine.getAnalysisData()
          const simulated = audioEngine.generateFeatures(delta)

          setAudioFeatures({
            ...simulated,
            bass: analysis.bass > 0 ? analysis.bass : simulated.bass,
            mid: analysis.mid > 0 ? analysis.mid : simulated.mid,
            high: analysis.high > 0 ? analysis.high : simulated.high,
            frequencyBands: Array.from(analysis.frequencyData).slice(0, 32).map(v => v / 255),
          })
        } else if (audioMode === 'file' && audioFile) {
          // Get analysis from file analyzer
          const analysis = fileAnalyzer.analyze()
          setAudioFeatures({
            rms: analysis.rms,
            bass: analysis.bass,
            mid: analysis.mid,
            high: analysis.high,
            isBeat: analysis.isBeat,
            beatIntensity: analysis.beatIntensity,
            bpm: 0, // Would need BPM detection algorithm
            spectralCentroid: analysis.spectralCentroid * 10000,
            spectralFlux: analysis.spectralFlux,
            frequencyBands: Array.from(analysis.frequencyData).slice(0, 32).map(v => v / 255),
            time: fileAnalyzer.getCurrentTime(),
          })
        } else {
          // Fallback to simulated features
          const features = audioEngine.generateFeatures(delta)
          setAudioFeatures(features)
        }
      }

      animationId = requestAnimationFrame(tick)
    }

    animationId = requestAnimationFrame(tick)

    return () => cancelAnimationFrame(animationId)
  }, [isPlaying, setAudioFeatures, audioMode, audioFile, audioInitialized])

  // Update audio engine when genre changes
  useEffect(() => {
    audioEngine.setGenre(genre)
    audioEngine.reset()

    if (audioInitialized && audioMode === 'synth') {
      synthEngine.setGenre(genre)
    }
  }, [genre, audioInitialized, audioMode])

  // Handle play/pause
  useEffect(() => {
    if (!audioInitialized) return

    if (audioMode === 'synth') {
      if (isPlaying) {
        synthEngine.start()
      } else {
        synthEngine.stop()
      }
    } else if (audioMode === 'file') {
      if (isPlaying) {
        fileAnalyzer.play()
      } else {
        fileAnalyzer.pause()
      }
    }
  }, [isPlaying, audioMode, audioInitialized])

  // Handle volume changes
  useEffect(() => {
    if (!audioInitialized) return

    synthEngine.setVolume(volume)
    fileAnalyzer.setVolume(volume)
  }, [volume, audioInitialized])

  // Handle file changes
  useEffect(() => {
    if (!audioFile || !audioInitialized) return

    const loadFile = async () => {
      try {
        synthEngine.stop()
        await fileAnalyzer.loadFile(audioFile)
        if (isPlaying) {
          fileAnalyzer.play()
        }
      } catch (err) {
        console.error('Failed to load audio file:', err)
      }
    }

    loadFile()
  }, [audioFile, audioInitialized, isPlaying])

  // Keyboard controls
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Initialize audio on any key press
      initAudio()

      switch (e.key) {
        case '1':
          setGenre('electronic')
          break
        case '2':
          setGenre('classical')
          break
        case '3':
          setGenre('metal')
          break
        case '4':
          setGenre('ambient')
          break
        case '5':
          setGenre('jazz')
          break
        case ' ':
          e.preventDefault()
          setIsPlaying(!isPlaying)
          break
      }
    }

    // Also init on click
    const handleClick = () => {
      initAudio()
    }

    window.addEventListener('keydown', handleKeyDown)
    window.addEventListener('click', handleClick, { once: true })

    return () => {
      window.removeEventListener('keydown', handleKeyDown)
      window.removeEventListener('click', handleClick)
    }
  }, [setGenre, isPlaying, setIsPlaying, initAudio])

  // Simulate loading
  useEffect(() => {
    const timer = setTimeout(() => setIsLoaded(true), 1500)
    return () => clearTimeout(timer)
  }, [])

  return (
    <div className="app-container">
      {/* 3D Canvas */}
      <div className="canvas-wrapper">
        <Canvas
          camera={{
            fov: 60,
            near: 0.1,
            far: 1000,
            position: [0, 20, 50],
          }}
          shadows
          dpr={[1, 2]}
          gl={{
            antialias: true,
            alpha: false,
            powerPreference: 'high-performance',
          }}
        >
          <Suspense fallback={null}>
            <Scene />
          </Suspense>
        </Canvas>

        {/* Three.js loader */}
        <Loader
          containerStyles={{
            background: 'rgba(0, 0, 0, 0.9)',
          }}
          barStyles={{
            background: 'linear-gradient(90deg, #00ffff, #ff00ff)',
          }}
          dataStyles={{
            color: 'white',
            fontSize: '0.8rem',
          }}
        />
      </div>

      {/* UI Overlay */}
      {isLoaded && <UI />}

      {/* Loading screen */}
      {!isLoaded && <LoadingScreen progress={75} />}

      {/* Audio init hint */}
      {isLoaded && !audioInitialized && (
        <div className="audio-hint">
          Click anywhere or press any key to start audio
        </div>
      )}
    </div>
  )
}

export default App
