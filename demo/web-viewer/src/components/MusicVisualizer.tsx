import { useRef, useState, useEffect } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import type { Genre, CameraMode } from '../App'
import { WorldGenerator, type WorldElement } from '../utils/worldGenerator'

interface MusicVisualizerProps {
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

function MusicVisualizer({ audioFeatures, genre, cameraMode, isPlaying }: MusicVisualizerProps) {
  const groupRef = useRef<THREE.Group>(null)
  const timeRef = useRef(0)
  const worldGenRef = useRef<WorldGenerator | null>(null)
  const [worldElements, setWorldElements] = useState<WorldElement[]>([])
  const frameCountRef = useRef(0)

  // Initialize world generator
  useEffect(() => {
    worldGenRef.current = new WorldGenerator(genre)
  }, [])

  // Update genre
  useEffect(() => {
    if (worldGenRef.current) {
      worldGenRef.current.setGenre(genre)
    }
  }, [genre])

  // Process audio features and generate world
  useEffect(() => {
    if (isPlaying && worldGenRef.current) {
      // Add features every frame (simulating real-time)
      worldGenRef.current.addFeatures(audioFeatures)

      // Update elements list
      const allElements = worldGenRef.current.getAllElements()
      setWorldElements(allElements)
    }
  }, [audioFeatures, isPlaying])

  // Render world elements
  const renderElement = (element: WorldElement) => {
    const geometry = element.type === 'cube' ? (
      <boxGeometry args={element.scale} />
    ) : element.type === 'sphere' ? (
      <sphereGeometry args={[element.scale[0], 16, 16]} />
    ) : element.type === 'cone' ? (
      <coneGeometry args={[element.scale[0], element.scale[1], 8]} />
    ) : (
      <cylinderGeometry args={[element.scale[0], element.scale[0], element.scale[1], 12]} />
    )

    return (
      <mesh
        key={element.id}
        position={element.position}
        rotation={element.rotation}
        castShadow
      >
        {geometry}
        <meshStandardMaterial
          color={element.color}
          metalness={element.metalness}
          roughness={element.roughness}
          emissive={element.color}
          emissiveIntensity={element.emissiveIntensity}
        />
      </mesh>
    )
  }

  useFrame((state, delta) => {
    if (!isPlaying || !groupRef.current || !worldGenRef.current) return

    timeRef.current += delta
    const timeline = worldGenRef.current.getTimelinePosition()

    // Camera animations based on mode - More dynamic!
    switch (cameraMode) {
      case 'tracking':
        state.camera.position.set(
          Math.sin(timeRef.current * 0.1) * 3,
          8 + Math.sin(timeRef.current * 0.5) * 3,
          15
        )
        state.camera.lookAt(0, 5, -timeline * 1.5)
        break
      case 'cinematic':
        state.camera.position.set(
          Math.sin(timeRef.current * 0.3) * 20,
          8 + Math.cos(timeRef.current * 0.2) * 8,
          15 + Math.sin(timeRef.current * 0.15) * 10
        )
        state.camera.lookAt(0, 5, -timeline * 1.5)
        break
      case 'firstPerson':
        state.camera.position.set(
          Math.sin(timeRef.current * 0.2) * 2,
          5,
          8 - timeline * 1.5
        )
        state.camera.lookAt(0, 5, -timeline * 1.5 - 15)
        break
      // 'orbit' is handled by OrbitControls
    }
  })

  return (
    <group ref={groupRef}>
      {/* Render all generated world elements */}
      {worldElements.map(element => renderElement(element))}

      {/* Path line to show trajectory */}
      {worldElements.length > 1 && (
        <line>
          <bufferGeometry>
            <bufferAttribute
              attach="attributes-position"
              count={worldElements.length}
              array={new Float32Array(worldElements.flatMap(e => e.position))}
              itemSize={3}
            />
          </bufferGeometry>
          <lineBasicMaterial color="#444444" opacity={0.3} transparent />
        </line>
      )}
    </group>
  )
}

export default MusicVisualizer
