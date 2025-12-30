import { useState, useEffect, useRef } from 'react'
import { Canvas } from '@react-three/fiber'
import { OrbitControls, PerspectiveCamera, Stats } from '@react-three/drei'
import ProceduralWorldVisualizer from './components/ProceduralWorldVisualizer'
import ControlPanel from './components/ControlPanel'
import PerformanceMetrics from './components/PerformanceMetrics'
import { AudioProcessor, AudioSource } from './utils/audioProcessor'
import { MicrophoneCapture } from './utils/microphoneCapture'
import './App.css'

export type Genre = 'electronic' | 'classical' | 'jazz' | 'metal' | 'ambient'
export type CameraMode = 'orbit' | 'tracking' | 'cinematic' | 'firstPerson'

function App() {
  const [genre, setGenre] = useState<Genre>('electronic')
  const [cameraMode, setCameraMode] = useState<CameraMode>('tracking')
  const [isPlaying, setIsPlaying] = useState(true)
  const [speed, setSpeed] = useState(1.0)
  const [audioSource, setAudioSource] = useState<AudioSource>('generated')
  const [microphoneActive, setMicrophoneActive] = useState(false)
  const [audioFeatures, setAudioFeatures] = useState({
    spectralCentroid: 1000,
    rms: 0.3,
    zcr: 0.1,
    dominantFreq: 440,
    spectralFlux: 0.2,
    beatConfidence: 0.5,
    tempo: 120,
  })
  const [metrics, setMetrics] = useState({
    fps: 60,
    latency: 0,
    chunksGenerated: 0,
  })

  const audioProcessorRef = useRef<AudioProcessor | null>(null)
  const microphoneCaptureRef = useRef<MicrophoneCapture | null>(null)
  const frameCountRef = useRef(0)
  const lastFpsUpdateRef = useRef(Date.now())

  // Initialize audio processor
  useEffect(() => {
    audioProcessorRef.current = new AudioProcessor(audioSource)
  }, [])

  // Handle microphone toggle
  const toggleMicrophone = async () => {
    if (!microphoneActive) {
      try {
        console.log('🎤 Initializing microphone...')
        const mic = new MicrophoneCapture()
        await mic.initialize()
        microphoneCaptureRef.current = mic
        setMicrophoneActive(true)
        setAudioSource('microphone')
        if (audioProcessorRef.current) {
          audioProcessorRef.current.setAudioSource('microphone')
        }
        console.log('✅ Microphone activated, audioSource set to:', 'microphone')
      } catch (error) {
        console.error('❌ Failed to initialize microphone:', error)
        alert('Failed to access microphone. Please check permissions.')
      }
    } else {
      console.log('🛑 Stopping microphone...')
      if (microphoneCaptureRef.current) {
        microphoneCaptureRef.current.stop()
        microphoneCaptureRef.current = null
      }
      setMicrophoneActive(false)
      setAudioSource('generated')
      if (audioProcessorRef.current) {
        audioProcessorRef.current.setAudioSource('generated')
      }
      console.log('✅ Microphone deactivated, audioSource set to:', 'generated')
    }
  }

  // Audio processing loop - ONLY when microphone is active
  useEffect(() => {
    console.log('🔄 Audio loop restarted - micActive:', microphoneActive, 'isPlaying:', isPlaying)

    // ONLY process audio when microphone is ON
    if (!microphoneActive) {
      console.log('⏸️ Microphone OFF - no audio processing')
      return
    }

    // Start processing audio
    const interval = setInterval(() => {
      if (isPlaying && audioProcessorRef.current) {
        const start = window.performance.now()

        // Get microphone samples
        let micSamples: Float32Array | undefined
        if (microphoneCaptureRef.current) {
          const samples = microphoneCaptureRef.current.getTimeDomainData()
          if (samples) {
            micSamples = samples.slice(0, 512) // Use first 512 samples
          } else {
            console.warn('⚠️ Microphone active but getTimeDomainData returned null!')
          }
        }

        // Only process if we have microphone samples
        if (micSamples) {
          const features = audioProcessorRef.current.getFeatures(micSamples)
          const latency = window.performance.now() - start

          setAudioFeatures(features)
          setMetrics(prev => ({
            ...prev,
            latency,
            chunksGenerated: prev.chunksGenerated + 1,
          }))
        }
      }
    }, 16 * speed) // ~60fps adjusted by speed

    return () => clearInterval(interval)
  }, [isPlaying, speed, microphoneActive])

  // FPS tracking
  useEffect(() => {
    const updateFPS = () => {
      frameCountRef.current++
      const now = Date.now()
      const elapsed = now - lastFpsUpdateRef.current

      if (elapsed >= 1000) {
        const fps = (frameCountRef.current / elapsed) * 1000
        setMetrics(prev => ({ ...prev, fps }))
        frameCountRef.current = 0
        lastFpsUpdateRef.current = now
      }

      requestAnimationFrame(updateFPS)
    }

    const id = requestAnimationFrame(updateFPS)
    return () => cancelAnimationFrame(id)
  }, [])

  return (
    <div className="app-container">
      <div className="header">
        <h1>🎵 omega-synesthesia Web Viewer</h1>
        <p className="subtitle">Real-Time Music Visualization - V1.0.0</p>
      </div>

      <div className="main-content">
        <div className="canvas-container">
          <Canvas shadows>
            <PerspectiveCamera makeDefault position={[0, 5, 10]} />
            <OrbitControls
              enabled={cameraMode === 'orbit'}
              enableDamping
              dampingFactor={0.05}
            />

            {/* Procedural World Visualization - Genre-specific 3D worlds! */}
            <ProceduralWorldVisualizer
              audioFeatures={audioFeatures}
              genre={genre}
              cameraMode={cameraMode}
              isPlaying={isPlaying}
            />

            {/* Ground plane for walking */}
            <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, -0.5, 0]} receiveShadow>
              <planeGeometry args={[1000, 1000]} />
              <meshStandardMaterial color="#1a1a2e" />
            </mesh>

            <Stats />
          </Canvas>
        </div>

        <ControlPanel
          genre={genre}
          onGenreChange={setGenre}
          cameraMode={cameraMode}
          onCameraModeChange={setCameraMode}
          isPlaying={isPlaying}
          onPlayPauseToggle={() => setIsPlaying(!isPlaying)}
          speed={speed}
          onSpeedChange={setSpeed}
          audioSource={audioSource}
          microphoneActive={microphoneActive}
          onToggleMicrophone={toggleMicrophone}
          audioLevel={audioFeatures.rms}
          onReset={() => {
            setMetrics({ fps: 60, latency: 0, chunksGenerated: 0 })
            if (audioProcessorRef.current) {
              audioProcessorRef.current.reset()
            }
            // Force re-render to reset world
            window.location.reload()
          }}
        />

        <PerformanceMetrics
          fps={metrics.fps}
          latency={metrics.latency}
          chunksGenerated={metrics.chunksGenerated}
          audioFeatures={audioFeatures}
        />
      </div>

      <div className="footer">
        <p>🚀 Transform music into worlds. Experience sound spatially.</p>
        <p className="info">
          Performance: {metrics.latency.toFixed(2)}ms latency |
          {metrics.fps.toFixed(1)} FPS |
          {metrics.chunksGenerated} chunks generated
        </p>
      </div>
    </div>
  )
}

export default App
