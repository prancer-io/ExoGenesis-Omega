/**
 * Main Scene - Combines all visual systems with post-processing
 */

import { useRef, useEffect } from 'react'
import { useFrame, useThree } from '@react-three/fiber'
import { Environment, Sky, Stars } from '@react-three/drei'
import {
  EffectComposer,
  Bloom,
  ChromaticAberration,
  Vignette,
  Noise,
  ToneMapping,
} from '@react-three/postprocessing'
import { BlendFunction, ToneMappingMode } from 'postprocessing'
import * as THREE from 'three'
import { useStore, type Genre } from '../store'
import { Terrain } from '../systems/Terrain'
import { ParticleSystem, AtmosphericParticles } from '../systems/Particles'
import { DynamicStoryWorld } from '../systems/DynamicStoryWorld'

// Genre-specific environment settings
const envConfigs: Record<Genre, {
  skyColor: string
  groundColor: string
  fogColor: string
  fogNear: number
  fogFar: number
  bloomIntensity: number
  bloomThreshold: number
  starCount: number
}> = {
  electronic: {
    skyColor: '#000022',
    groundColor: '#001133',
    fogColor: '#000033',
    fogNear: 30,
    fogFar: 200,
    bloomIntensity: 1.5,
    bloomThreshold: 0.2,
    starCount: 5000,
  },
  classical: {
    skyColor: '#1a1510',
    groundColor: '#332200',
    fogColor: '#1a1510',
    fogNear: 50,
    fogFar: 250,
    bloomIntensity: 0.8,
    bloomThreshold: 0.4,
    starCount: 2000,
  },
  metal: {
    skyColor: '#0a0000',
    groundColor: '#330000',
    fogColor: '#1a0500',
    fogNear: 20,
    fogFar: 150,
    bloomIntensity: 2.0,
    bloomThreshold: 0.1,
    starCount: 1000,
  },
  ambient: {
    skyColor: '#0a0a15',
    groundColor: '#1a0033',
    fogColor: '#0a0a20',
    fogNear: 40,
    fogFar: 300,
    bloomIntensity: 1.2,
    bloomThreshold: 0.3,
    starCount: 8000,
  },
  jazz: {
    skyColor: '#0a0500',
    groundColor: '#1a1000',
    fogColor: '#1a0800',
    fogNear: 15,
    fogFar: 100,
    bloomIntensity: 0.6,
    bloomThreshold: 0.5,
    starCount: 500,
  },
}

// Cinematic camera controller - more dynamic
function CinematicCamera() {
  const { camera } = useThree()
  const audioFeatures = useStore((s) => s.audioFeatures)
  const cameraMode = useStore((s) => s.cameraMode)
  const targetRef = useRef(new THREE.Vector3(0, 20, 0))
  const positionRef = useRef(new THREE.Vector3(0, 30, 60))
  const shakeRef = useRef({ x: 0, y: 0, z: 0 })
  const zoomRef = useRef(60)
  const lastBeatTimeRef = useRef(0)

  useFrame((state, delta) => {
    const time = state.clock.elapsedTime

    // Dynamic zoom based on energy
    const targetZoom = 50 + audioFeatures.rms * 30 - audioFeatures.bass * 20
    zoomRef.current = THREE.MathUtils.lerp(zoomRef.current, targetZoom, 0.05)

    if (cameraMode === 'cinematic') {
      // More dramatic sweeping motion
      const baseRadius = zoomRef.current + 20
      const radius = baseRadius + Math.sin(time * 0.2) * 25
      const height = 20 + Math.sin(time * 0.15) * 20 + audioFeatures.bass * 15
      const angle = time * 0.12 + audioFeatures.beatIntensity * 0.5

      positionRef.current.set(
        Math.cos(angle) * radius,
        height,
        Math.sin(angle) * radius
      )

      // Target follows the energy
      targetRef.current.set(
        Math.sin(time * 0.08) * 15,
        15 + audioFeatures.mid * 10,
        Math.cos(time * 0.08) * 15
      )
    } else if (cameraMode === 'fly') {
      // Flying through with energy-based speed
      const speed = 15 + audioFeatures.rms * 20
      positionRef.current.z -= delta * speed
      positionRef.current.x = Math.sin(time * 0.3) * 40
      positionRef.current.y = 20 + Math.sin(time * 0.2) * 15 + audioFeatures.bass * 10

      targetRef.current.set(
        positionRef.current.x + Math.sin(time * 0.5) * 15,
        15 + audioFeatures.high * 10,
        positionRef.current.z - 60
      )

      if (positionRef.current.z < -500) {
        positionRef.current.z = 100
      }
    }

    // Dramatic camera shake on beats
    if (audioFeatures.isBeat && time - lastBeatTimeRef.current > 0.1) {
      const shakeIntensity = audioFeatures.beatIntensity * 4
      shakeRef.current.x = (Math.random() - 0.5) * shakeIntensity
      shakeRef.current.y = (Math.random() - 0.5) * shakeIntensity
      shakeRef.current.z = (Math.random() - 0.5) * shakeIntensity * 0.5
      lastBeatTimeRef.current = time
    } else {
      shakeRef.current.x *= 0.85
      shakeRef.current.y *= 0.85
      shakeRef.current.z *= 0.85
    }

    // Smooth camera movement
    camera.position.lerp(positionRef.current, delta * 3)
    camera.position.x += shakeRef.current.x
    camera.position.y += shakeRef.current.y
    camera.position.z += shakeRef.current.z

    // Dynamic FOV
    if ('fov' in camera) {
      const targetFov = 55 + audioFeatures.beatIntensity * 10
      ;(camera as THREE.PerspectiveCamera).fov = THREE.MathUtils.lerp(
        (camera as THREE.PerspectiveCamera).fov,
        targetFov,
        0.1
      )
      ;(camera as THREE.PerspectiveCamera).updateProjectionMatrix()
    }

    camera.lookAt(targetRef.current)
  })

  return null
}

// Dynamic lighting system
function DynamicLighting() {
  const genre = useStore((s) => s.genre)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const config = envConfigs[genre]

  const lightRef1 = useRef<THREE.DirectionalLight>(null)
  const lightRef2 = useRef<THREE.PointLight>(null)

  useFrame((state) => {
    if (lightRef1.current) {
      // Main light pulses with audio
      lightRef1.current.intensity = 0.5 + audioFeatures.rms * 0.5
    }
    if (lightRef2.current) {
      // Accent light moves
      const time = state.clock.elapsedTime
      lightRef2.current.position.x = Math.sin(time * 0.5) * 50
      lightRef2.current.position.z = Math.cos(time * 0.5) * 50
      lightRef2.current.intensity = 50 + audioFeatures.beatIntensity * 100
    }
  })

  return (
    <>
      {/* Main directional light */}
      <directionalLight
        ref={lightRef1}
        position={[50, 100, 50]}
        intensity={0.5}
        color={config.skyColor}
        castShadow
        shadow-mapSize={[2048, 2048]}
        shadow-camera-far={200}
        shadow-camera-left={-100}
        shadow-camera-right={100}
        shadow-camera-top={100}
        shadow-camera-bottom={-100}
      />

      {/* Moving accent light */}
      <pointLight
        ref={lightRef2}
        position={[0, 30, 0]}
        color={genre === 'electronic' ? '#00ffff' :
               genre === 'metal' ? '#ff4500' :
               genre === 'classical' ? '#ffd700' :
               genre === 'ambient' ? '#9370db' : '#ff8c00'}
        intensity={50}
        distance={100}
      />

      {/* Ambient fill */}
      <ambientLight intensity={0.1} color={config.groundColor} />

      {/* Hemisphere for sky/ground color */}
      <hemisphereLight
        color={config.skyColor}
        groundColor={config.groundColor}
        intensity={0.3}
      />
    </>
  )
}

// Post-processing effects - more dramatic
function PostProcessing() {
  const genre = useStore((s) => s.genre)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const config = envConfigs[genre]

  // More intense bloom during high energy
  const dynamicBloom = config.bloomIntensity + audioFeatures.beatIntensity * 1.5 + audioFeatures.bass * 0.5

  return (
    <EffectComposer>
      {/* Intense bloom for glow effects */}
      <Bloom
        intensity={dynamicBloom}
        luminanceThreshold={config.bloomThreshold - audioFeatures.rms * 0.1}
        luminanceSmoothing={0.8}
        mipmapBlur
      />

      {/* Strong chromatic aberration on beats */}
      <ChromaticAberration
        offset={new THREE.Vector2(
          audioFeatures.beatIntensity * 0.005 + audioFeatures.bass * 0.002,
          audioFeatures.beatIntensity * 0.005 + audioFeatures.bass * 0.002
        )}
        radialModulation={false}
        modulationOffset={0.5}
      />

      {/* Dynamic vignette */}
      <Vignette
        offset={0.2 + audioFeatures.beatIntensity * 0.1}
        darkness={0.6 + audioFeatures.bass * 0.3}
        blendFunction={BlendFunction.NORMAL}
      />

      {/* Subtle noise for film grain */}
      <Noise
        opacity={0.03 + audioFeatures.high * 0.02}
        blendFunction={BlendFunction.OVERLAY}
      />

      {/* Tone mapping */}
      <ToneMapping
        mode={ToneMappingMode.ACES_FILMIC}
      />
    </EffectComposer>
  )
}

// Main scene component
export function Scene() {
  const genre = useStore((s) => s.genre)
  const config = envConfigs[genre]
  const { scene } = useThree()

  // Update fog when genre changes
  useEffect(() => {
    scene.fog = new THREE.Fog(config.fogColor, config.fogNear, config.fogFar)
    scene.background = new THREE.Color(config.skyColor)
  }, [genre, config, scene])

  return (
    <>
      {/* Camera controller */}
      <CinematicCamera />

      {/* Dynamic lighting */}
      <DynamicLighting />

      {/* Starfield */}
      <Stars
        radius={200}
        depth={100}
        count={config.starCount}
        factor={4}
        saturation={0.5}
        fade
        speed={0.5}
      />

      {/* Procedural terrain */}
      <Terrain size={300} resolution={150} />

      {/* Dynamic story world - main visual show */}
      <DynamicStoryWorld />

      {/* Enhanced particle systems */}
      <ParticleSystem count={6000} />
      <AtmosphericParticles count={1200} />

      {/* Post-processing */}
      <PostProcessing />
    </>
  )
}
