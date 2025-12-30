import { useRef, useMemo, useEffect } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import type { Genre, CameraMode } from '../App'

interface AdvancedVisualizerProps {
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

function AdvancedVisualizer({ audioFeatures, genre, cameraMode, isPlaying }: AdvancedVisualizerProps) {
  const groupRef = useRef<THREE.Group>(null)
  const particlesRef = useRef<THREE.Points>(null)
  const tunnelRef = useRef<THREE.Mesh>(null)
  const energyFieldRef = useRef<THREE.Mesh>(null)
  const timeRef = useRef(0)
  const beatsRef = useRef<number[]>([])

  // Genre-specific color schemes
  const colorScheme = useMemo(() => {
    const schemes = {
      electronic: { primary: 0x00ffff, secondary: 0xff00ff, accent: 0xffff00 },
      classical: { primary: 0xffd700, secondary: 0xffffff, accent: 0x87ceeb },
      jazz: { primary: 0xff6b35, secondary: 0xf7931e, accent: 0xc9ada7 },
      metal: { primary: 0xff0000, secondary: 0xff6666, accent: 0xffffff },
      ambient: { primary: 0x9370db, secondary: 0x48d1cc, accent: 0xba55d3 },
    }
    return schemes[genre]
  }, [genre])

  // Create particle system (10000 particles!)
  const particleGeometry = useMemo(() => {
    const geometry = new THREE.BufferGeometry()
    const particleCount = 10000
    const positions = new Float32Array(particleCount * 3)
    const colors = new Float32Array(particleCount * 3)
    const sizes = new Float32Array(particleCount)

    for (let i = 0; i < particleCount; i++) {
      // Distribute particles in a tunnel/spiral
      const angle = (i / particleCount) * Math.PI * 20
      const radius = 20 + Math.random() * 10
      const z = -i * 0.5

      positions[i * 3] = Math.cos(angle) * radius
      positions[i * 3 + 1] = Math.sin(angle) * radius
      positions[i * 3 + 2] = z

      // Random colors
      const color = new THREE.Color(colorScheme.primary)
      colors[i * 3] = color.r
      colors[i * 3 + 1] = color.g
      colors[i * 3 + 2] = color.b

      sizes[i] = Math.random() * 2 + 0.5
    }

    geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3))
    geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3))
    geometry.setAttribute('size', new THREE.BufferAttribute(sizes, 1))

    return geometry
  }, [colorScheme])

  const particleMaterial = useMemo(() => {
    return new THREE.PointsMaterial({
      size: 2,
      sizeAttenuation: true,
      vertexColors: true,
      transparent: true,
      opacity: 0.8,
      blending: THREE.AdditiveBlending,
    })
  }, [])

  // Create energy tunnel
  const tunnelGeometry = useMemo(() => {
    return new THREE.CylinderGeometry(25, 25, 500, 64, 64, true)
  }, [])

  const tunnelMaterial = useMemo(() => {
    return new THREE.MeshStandardMaterial({
      color: colorScheme.primary,
      wireframe: true,
      transparent: true,
      opacity: 0.15,
      emissive: new THREE.Color(colorScheme.primary),
      emissiveIntensity: 0.5,
      side: THREE.BackSide,
    })
  }, [colorScheme])

  // Create energy field sphere
  const energyFieldGeometry = useMemo(() => {
    return new THREE.IcosahedronGeometry(15, 4)
  }, [])

  const energyFieldMaterial = useMemo(() => {
    return new THREE.MeshStandardMaterial({
      color: colorScheme.secondary,
      wireframe: true,
      transparent: true,
      opacity: 0.3,
      emissive: new THREE.Color(colorScheme.secondary),
      emissiveIntensity: 1.0,
    })
  }, [colorScheme])

  // Track beats for visual pulses
  useEffect(() => {
    if (audioFeatures.beatConfidence > 0.7) {
      beatsRef.current.push(Date.now())
      // Keep only recent beats
      beatsRef.current = beatsRef.current.filter(t => Date.now() - t < 500)
    }
  }, [audioFeatures.beatConfidence])

  useFrame((state, delta) => {
    if (!isPlaying) return

    timeRef.current += delta

    // Animate particles based on audio
    if (particlesRef.current) {
      const positions = particlesRef.current.geometry.attributes.position.array as Float32Array
      const colors = particlesRef.current.geometry.attributes.color.array as Float32Array

      for (let i = 0; i < positions.length; i += 3) {
        // Move particles forward (create flow effect)
        positions[i + 2] += audioFeatures.rms * 2 + 0.5

        // Reset particles that moved too far
        if (positions[i + 2] > 50) {
          positions[i + 2] = -250
        }

        // Pulse particles with beat
        const beatPulse = beatsRef.current.length > 0 ? 1.5 : 1.0

        // Rotate particles based on frequency
        const angle = timeRef.current * 0.5 + audioFeatures.spectralCentroid * 0.0001
        const radius = 20 + Math.sin(i * 0.1 + timeRef.current) * 5 * audioFeatures.rms
        positions[i] = Math.cos(angle + i * 0.01) * radius * beatPulse
        positions[i + 1] = Math.sin(angle + i * 0.01) * radius * beatPulse

        // Color based on audio features
        const hue = (audioFeatures.spectralCentroid / 8000 + timeRef.current * 0.1) % 1
        const color = new THREE.Color().setHSL(hue, 1.0, 0.5 + audioFeatures.rms * 0.5)
        colors[i] = color.r
        colors[i + 1] = color.g
        colors[i + 2] = color.b
      }

      particlesRef.current.geometry.attributes.position.needsUpdate = true
      particlesRef.current.geometry.attributes.color.needsUpdate = true
    }

    // Animate tunnel rotation and pulse
    if (tunnelRef.current) {
      tunnelRef.current.rotation.z = timeRef.current * 0.2
      tunnelRef.current.position.z = -250

      // Pulse tunnel on beat
      const beatScale = 1 + audioFeatures.beatConfidence * 0.3
      tunnelRef.current.scale.set(beatScale, 1, beatScale)

      // Update tunnel material
      const material = tunnelRef.current.material as THREE.MeshStandardMaterial
      material.emissiveIntensity = 0.3 + audioFeatures.rms * 2
    }

    // Animate energy field
    if (energyFieldRef.current) {
      energyFieldRef.current.rotation.x = timeRef.current * 0.5
      energyFieldRef.current.rotation.y = timeRef.current * 0.3

      // Scale with audio
      const scale = 1 + audioFeatures.rms * 3
      energyFieldRef.current.scale.set(scale, scale, scale)

      // Update material
      const material = energyFieldRef.current.material as THREE.MeshStandardMaterial
      material.emissiveIntensity = 1.0 + audioFeatures.beatConfidence * 3
      material.opacity = 0.3 + audioFeatures.spectralFlux
    }

    // Dynamic camera based on mode
    const beatInfluence = beatsRef.current.length > 0 ? 2 : 0

    switch (cameraMode) {
      case 'tracking':
        // Follow particles flowing forward
        state.camera.position.set(
          Math.sin(timeRef.current * 0.3) * 10,
          Math.cos(timeRef.current * 0.2) * 10 + 5,
          20 + beatInfluence
        )
        state.camera.lookAt(0, 0, -50)
        break

      case 'cinematic':
        // Dramatic sweeping shots
        state.camera.position.set(
          Math.cos(timeRef.current * 0.4) * 40,
          Math.sin(timeRef.current * 0.3) * 20 + 10,
          Math.sin(timeRef.current * 0.2) * 30
        )
        state.camera.lookAt(0, 0, -100)
        break

      case 'firstPerson':
        // Fly through the tunnel
        state.camera.position.set(
          Math.sin(timeRef.current * 0.5) * 5,
          Math.cos(timeRef.current * 0.5) * 5,
          10 - timeRef.current * 5
        )
        state.camera.lookAt(0, 0, -200)
        break

      // orbit handled by OrbitControls
    }
  })

  return (
    <group ref={groupRef}>
      {/* Particle system */}
      <points ref={particlesRef} geometry={particleGeometry} material={particleMaterial} />

      {/* Energy tunnel */}
      <mesh ref={tunnelRef} geometry={tunnelGeometry} material={tunnelMaterial} />

      {/* Central energy field */}
      <mesh ref={energyFieldRef} geometry={energyFieldGeometry} material={energyFieldMaterial} />

      {/* Ambient rotating lights */}
      <pointLight
        position={[Math.cos(timeRef.current) * 20, 10, -50]}
        color={colorScheme.primary}
        intensity={2 + audioFeatures.rms * 5}
        distance={100}
      />
      <pointLight
        position={[Math.sin(timeRef.current) * 20, -10, -50]}
        color={colorScheme.secondary}
        intensity={2 + audioFeatures.beatConfidence * 5}
        distance={100}
      />
    </group>
  )
}

export default AdvancedVisualizer
