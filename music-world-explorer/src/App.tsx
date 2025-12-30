/**
 * Music World Explorer - Main Application
 * Transform sound into stunning 3D worlds
 */

import { useState, useEffect, useCallback, Suspense } from 'react'
import { Canvas } from '@react-three/fiber'
import { Loader } from '@react-three/drei'
import { useStore, type Genre } from './store'
import { audioEngine } from './audio/AudioEngine'
import { Scene } from './components/Scene'
import { UI, LoadingScreen } from './components/UI'

function App() {
  const [isLoaded, setIsLoaded] = useState(false)
  const genre = useStore((s) => s.genre)
  const setGenre = useStore((s) => s.setGenre)
  const setAudioFeatures = useStore((s) => s.setAudioFeatures)
  const isPlaying = useStore((s) => s.isPlaying)
  const setIsPlaying = useStore((s) => s.setIsPlaying)

  // Audio engine loop
  useEffect(() => {
    let animationId: number
    let lastTime = performance.now()

    const tick = () => {
      const now = performance.now()
      const delta = (now - lastTime) / 1000
      lastTime = now

      if (isPlaying) {
        const features = audioEngine.generateFeatures(delta)
        setAudioFeatures(features)
      }

      animationId = requestAnimationFrame(tick)
    }

    animationId = requestAnimationFrame(tick)

    return () => cancelAnimationFrame(animationId)
  }, [isPlaying, setAudioFeatures])

  // Update audio engine when genre changes
  useEffect(() => {
    audioEngine.setGenre(genre)
    audioEngine.reset()
  }, [genre])

  // Keyboard controls
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
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

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [setGenre, isPlaying, setIsPlaying])

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
    </div>
  )
}

export default App
