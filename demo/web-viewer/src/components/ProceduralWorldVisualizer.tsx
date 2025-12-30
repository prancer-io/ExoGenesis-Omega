import { useRef, useEffect, useState } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import type { Genre, CameraMode } from '../App'
import { getWorldGenerator, WorldChunk } from './EnhancedWorldGenerators'

interface ProceduralWorldVisualizerProps {
  audioFeatures: {
    spectralCentroid: number
    rms: number
    zcr: number
    dominantFreq: number
    spectralFlux: number
    beatConfidence: number
    tempo: number
  }
  genre: Genre
  cameraMode: CameraMode
  isPlaying: boolean
}

function ProceduralWorldVisualizer({
  audioFeatures,
  genre,
  cameraMode,
  isPlaying,
}: ProceduralWorldVisualizerProps) {
  const groupRef = useRef<THREE.Group>(null)
  const [chunks, setChunks] = useState<WorldChunk[]>([])
  const generatorRef = useRef(getWorldGenerator(genre))
  const chunkPositionRef = useRef(0)
  const timeRef = useRef(0)
  const lastGenTimeRef = useRef(0)
  const particleUpdateCounterRef = useRef(0) // Update particles less frequently

  // Update generator when genre changes
  useEffect(() => {
    console.log(`🎨 Switching to ${genre} world generator`)
    generatorRef.current = getWorldGenerator(genre)
    setChunks([]) // Clear old chunks
    chunkPositionRef.current = 0
    lastGenTimeRef.current = 0
  }, [genre])

  // Generate world chunks based on audio
  useEffect(() => {
    if (!isPlaying || audioFeatures.rms < 0.01) {
      return
    }

    const now = Date.now()
    // Generate new chunk every 3 seconds when audio is active (optimized)
    if (now - lastGenTimeRef.current > 3000) {
      const position = new THREE.Vector3(
        chunkPositionRef.current * 10,
        0,
        0
      )

      const newChunk = generatorRef.current.generateChunk(audioFeatures, position)

      setChunks((prev) => {
        // Keep only last 10 chunks (optimized for performance)
        const updated = [...prev, newChunk]
        if (updated.length > 10) {
          // Clean up old meshes, particles, and lights
          const removed = updated.shift()
          removed?.meshes.forEach(mesh => {
            mesh.geometry.dispose()
            if (Array.isArray(mesh.material)) {
              mesh.material.forEach(mat => mat.dispose())
            } else {
              mesh.material.dispose()
            }
          })
          removed?.particles.forEach(particle => {
            particle.geometry.dispose()
            if (Array.isArray(particle.material)) {
              particle.material.forEach(mat => mat.dispose())
            } else {
              particle.material.dispose()
            }
          })
        }
        return updated
      })

      chunkPositionRef.current++
      lastGenTimeRef.current = now

      console.log(`🏗️ Generated ${genre} chunk #${chunkPositionRef.current} at x=${position.x}`)
    }
  }, [audioFeatures, isPlaying, genre])

  // Animate camera based on mode
  useFrame((state, delta) => {
    if (!isPlaying) return

    timeRef.current += delta

    // Camera movement based on mode
    switch (cameraMode) {
      case 'tracking':
        // Follow along the generated world
        state.camera.position.set(
          chunkPositionRef.current * 10 - 20 + Math.sin(timeRef.current * 0.3) * 5,
          5 + Math.sin(timeRef.current * 0.5) * 3,
          15
        )
        state.camera.lookAt(chunkPositionRef.current * 10 - 10, 0, 0)
        break

      case 'cinematic':
        // Sweeping aerial view
        const angle = timeRef.current * 0.3
        const radius = 30
        state.camera.position.set(
          chunkPositionRef.current * 10 + Math.cos(angle) * radius,
          15 + Math.sin(timeRef.current * 0.2) * 10,
          Math.sin(angle) * radius
        )
        state.camera.lookAt(chunkPositionRef.current * 10, 0, 0)
        break

      case 'firstPerson':
        // Walk through the world
        state.camera.position.set(
          chunkPositionRef.current * 10 - 15 + timeRef.current * 2,
          2 + Math.sin(timeRef.current * 2) * 0.5, // Gentle head bob
          Math.sin(timeRef.current * 0.5) * 3
        )
        state.camera.lookAt(chunkPositionRef.current * 10, 2, 0)
        break

      // orbit handled by OrbitControls
    }

    // Animate meshes, particles, and lights within chunks
    chunks.forEach((chunk, chunkIndex) => {
      // Animate meshes
      chunk.meshes.forEach((mesh, meshIndex) => {
        // Gentle floating animation for spheres (orbs, lamps)
        if (mesh.geometry instanceof THREE.SphereGeometry) {
          mesh.position.y += Math.sin(timeRef.current * 2 + meshIndex * 0.5) * 0.02
        }

        // Rotate crystals, pyramids, structures
        if (mesh.geometry instanceof THREE.ConeGeometry) {
          mesh.rotation.y += delta * 0.3
        }

        // Pulse emissive materials with audio
        const material = mesh.material as THREE.MeshStandardMaterial
        if (material.emissive && material.emissive.r > 0) {
          const baseIntensity = material.userData.baseEmissiveIntensity || material.emissiveIntensity
          material.userData.baseEmissiveIntensity = baseIntensity
          material.emissiveIntensity = baseIntensity * (1 + audioFeatures.beatConfidence * 0.8)
        }
      })

      // Animate particles (only every 3rd frame for performance)
      particleUpdateCounterRef.current++
      if (particleUpdateCounterRef.current % 3 === 0) {
        chunk.particles.forEach((particleSystem, particleIndex) => {
          const positions = particleSystem.geometry.attributes.position.array as Float32Array

          // Different animation based on particle type (genre)
          // Update fewer particles per frame for better performance
          const step = 6 // Update every 6th particle

          if (chunk.genre === 'classical') {
            // Dust floats slowly upward
            for (let i = 1; i < positions.length; i += step) {
              positions[i] += delta * 1.5 // Compensate for less frequent updates
              if (positions[i] > 25) positions[i] = 0
            }
          } else if (chunk.genre === 'metal') {
            // Embers rise and drift
            for (let i = 0; i < positions.length; i += step) {
              positions[i + 1] += delta * 6
              positions[i] += Math.sin(timeRef.current + i) * delta * 1.5
              if (positions[i + 1] > 35) positions[i + 1] = 0
            }
          } else if (chunk.genre === 'jazz') {
            // Smoke drifts lazily
            for (let i = 0; i < positions.length; i += step) {
              positions[i + 1] += delta * 0.9
              positions[i] += Math.sin(timeRef.current * 0.5 + i) * delta * 0.9
              positions[i + 2] += Math.cos(timeRef.current * 0.5 + i) * delta * 0.9
              if (positions[i + 1] > 18) positions[i + 1] = 0
            }
          } else if (chunk.genre === 'electronic') {
            // Light trails zoom around
            for (let i = 0; i < positions.length; i += step) {
              positions[i] += Math.sin(timeRef.current + i * 0.1) * delta * 9
              positions[i + 2] += Math.cos(timeRef.current + i * 0.1) * delta * 9
            }
          } else if (chunk.genre === 'ambient') {
            // Wisps float gently
            for (let i = 0; i < positions.length; i += step) {
              positions[i] += Math.sin(timeRef.current + i * 0.01) * delta * 1.5
              positions[i + 1] += Math.cos(timeRef.current * 0.5 + i * 0.01) * delta * 0.9
              positions[i + 2] += Math.sin(timeRef.current * 0.7 + i * 0.01) * delta * 1.5
            }
          }

          particleSystem.geometry.attributes.position.needsUpdate = true
        })
      }

      // Pulse lights with audio
      chunk.lights.forEach(light => {
        const baseIntensity = light.userData.baseIntensity || light.intensity
        light.userData.baseIntensity = baseIntensity
        light.intensity = baseIntensity * (1 + audioFeatures.beatConfidence * 0.5)
      })
    })
  })

  return (
    <group ref={groupRef}>
      {/* Render all chunk meshes, particles, and lights */}
      {chunks.map((chunk, chunkIndex) => (
        <group key={`chunk-${chunkIndex}`}>
          {/* Meshes */}
          {chunk.meshes.map((mesh, meshIndex) => (
            <primitive
              key={`chunk-${chunkIndex}-mesh-${meshIndex}`}
              object={mesh}
            />
          ))}

          {/* Particles */}
          {chunk.particles.map((particle, particleIndex) => (
            <primitive
              key={`chunk-${chunkIndex}-particle-${particleIndex}`}
              object={particle}
            />
          ))}

          {/* Lights */}
          {chunk.lights.map((light, lightIndex) => (
            <primitive
              key={`chunk-${chunkIndex}-light-${lightIndex}`}
              object={light}
            />
          ))}
        </group>
      ))}

      {/* Dynamic directional lighting based on genre */}
      <directionalLight
        position={[15, 20, 10]}
        intensity={0.8 + audioFeatures.rms * 1.5}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
      />
      <directionalLight
        position={[-15, 15, -10]}
        intensity={0.4 + audioFeatures.beatConfidence * 0.8}
        color={
          genre === 'metal' ? '#ff4500' :
          genre === 'jazz' ? '#ff8c00' :
          genre === 'classical' ? '#ffd700' :
          genre === 'electronic' ? '#00ffff' :
          '#9370db'
        }
      />

      {/* Ambient lighting - darker for more dramatic effect */}
      <ambientLight intensity={0.15} />
      <hemisphereLight
        color={
          genre === 'ambient' ? '#9370db' :
          genre === 'classical' ? '#ffd700' :
          genre === 'jazz' ? '#ff8c00' :
          '#ffffff'
        }
        groundColor={
          genre === 'metal' ? '#330000' :
          genre === 'electronic' ? '#000033' :
          '#1a1a2e'
        }
        intensity={0.4}
      />

      {/* Atmospheric fog */}
      <fog
        attach="fog"
        args={[
          genre === 'metal' ? '#330000' :
          genre === 'jazz' ? '#4a3400' :
          genre === 'ambient' ? '#483d8b' :
          genre === 'electronic' ? '#000033' :
          '#0a0a1a',
          30,
          120
        ]}
      />
    </group>
  )
}

export default ProceduralWorldVisualizer
