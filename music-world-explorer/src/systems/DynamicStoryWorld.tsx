/**
 * Dynamic Story World - Cinematic Music Visualization
 * Every beat creates dramatic visual events
 * Structures morph, explode, transform based on audio
 */

import { useRef, useMemo, useState, useEffect } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import { useStore, type Genre } from '../store'

// Story scene types
type ScenePhase = 'intro' | 'rising' | 'climax' | 'falling' | 'outro'

// Dynamic story config
interface StoryMoment {
  threshold: number // energy threshold to trigger
  duration: number // seconds
  name: string
}

const storyMoments: Record<Genre, StoryMoment[]> = {
  electronic: [
    { threshold: 0.3, duration: 20, name: 'awakening' },
    { threshold: 0.5, duration: 30, name: 'pulse' },
    { threshold: 0.7, duration: 40, name: 'surge' },
    { threshold: 0.9, duration: 30, name: 'transcendence' },
  ],
  classical: [
    { threshold: 0.2, duration: 30, name: 'prelude' },
    { threshold: 0.4, duration: 40, name: 'adagio' },
    { threshold: 0.7, duration: 50, name: 'crescendo' },
    { threshold: 0.5, duration: 30, name: 'resolution' },
  ],
  metal: [
    { threshold: 0.4, duration: 15, name: 'rumble' },
    { threshold: 0.6, duration: 25, name: 'fury' },
    { threshold: 0.9, duration: 35, name: 'inferno' },
    { threshold: 0.7, duration: 25, name: 'aftermath' },
  ],
  ambient: [
    { threshold: 0.2, duration: 40, name: 'stillness' },
    { threshold: 0.3, duration: 50, name: 'drift' },
    { threshold: 0.5, duration: 60, name: 'transcend' },
    { threshold: 0.3, duration: 40, name: 'peace' },
  ],
  jazz: [
    { threshold: 0.3, duration: 25, name: 'nightfall' },
    { threshold: 0.5, duration: 35, name: 'groove' },
    { threshold: 0.8, duration: 45, name: 'solo' },
    { threshold: 0.4, duration: 30, name: 'cool-down' },
  ],
}

// Main Dynamic World Component
export function DynamicStoryWorld() {
  const genre = useStore((s) => s.genre)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const setStoryState = useStore((s) => s.setStoryState)

  const [scenePhase, setScenePhase] = useState<ScenePhase>('intro')
  const [momentIndex, setMomentIndex] = useState(0)
  const [intensity, setIntensity] = useState(0)
  const energyHistoryRef = useRef<number[]>([])
  const lastBeatRef = useRef(0)
  const sceneTimeRef = useRef(0)

  // Calculate scene phase based on energy
  useFrame((state, delta) => {
    const energy = audioFeatures.rms * 0.4 + audioFeatures.bass * 0.4 + audioFeatures.beatIntensity * 0.2
    energyHistoryRef.current.push(energy)
    if (energyHistoryRef.current.length > 60) energyHistoryRef.current.shift()

    const avgEnergy = energyHistoryRef.current.reduce((a, b) => a + b, 0) / energyHistoryRef.current.length

    // Smooth intensity for visual elements
    setIntensity(prev => THREE.MathUtils.lerp(prev, avgEnergy, 0.1))

    // Update scene time
    sceneTimeRef.current += delta

    // Determine phase based on energy
    if (avgEnergy > 0.8) setScenePhase('climax')
    else if (avgEnergy > 0.5) setScenePhase('rising')
    else if (avgEnergy > 0.3) setScenePhase('falling')
    else setScenePhase('intro')

    // Update story state for UI
    const moments = storyMoments[genre]
    const currentMoment = moments[momentIndex % moments.length]
    if (avgEnergy > currentMoment.threshold && sceneTimeRef.current > currentMoment.duration) {
      setMomentIndex(prev => (prev + 1) % moments.length)
      sceneTimeRef.current = 0
    }

    setStoryState({
      chapterName: currentMoment.name,
      chapterIndex: momentIndex,
      chapterProgress: Math.min(sceneTimeRef.current / currentMoment.duration, 1),
      totalChapters: moments.length,
    })
  })

  // Reset on genre change
  useEffect(() => {
    setMomentIndex(0)
    sceneTimeRef.current = 0
    energyHistoryRef.current = []
  }, [genre])

  return (
    <group>
      {/* Beat explosions */}
      <BeatExplosions />

      {/* Dynamic central structure */}
      <CentralMonument intensity={intensity} phase={scenePhase} />

      {/* Orbiting entities */}
      <OrbitingEntities intensity={intensity} />

      {/* Rising pillars */}
      <DynamicPillars intensity={intensity} phase={scenePhase} />

      {/* Particle trails */}
      <ParticleTrails intensity={intensity} />

      {/* Energy waves */}
      <EnergyWaves />

      {/* Floating fragments */}
      <FloatingFragments intensity={intensity} />

      {/* Light beams */}
      <DynamicLightBeams phase={scenePhase} />

      {/* Ground effects */}
      <GroundEffects intensity={intensity} />
    </group>
  )
}

// Beat-reactive explosions
function BeatExplosions() {
  const audioFeatures = useStore((s) => s.audioFeatures)
  const explosionsRef = useRef<Array<{
    position: THREE.Vector3
    scale: number
    life: number
    color: THREE.Color
  }>>([])
  const meshRef = useRef<THREE.InstancedMesh>(null)
  const lastBeatRef = useRef(false)

  const count = 50
  const dummy = useMemo(() => new THREE.Object3D(), [])
  const colors = useMemo(() => new Float32Array(count * 3), [])

  useFrame((state, delta) => {
    if (!meshRef.current) return

    // Spawn explosion on beat
    if (audioFeatures.isBeat && !lastBeatRef.current) {
      const angle = Math.random() * Math.PI * 2
      const radius = 10 + Math.random() * 40
      explosionsRef.current.push({
        position: new THREE.Vector3(
          Math.cos(angle) * radius,
          5 + Math.random() * 30,
          Math.sin(angle) * radius
        ),
        scale: 0.1,
        life: 1,
        color: new THREE.Color().setHSL(Math.random(), 1, 0.6),
      })
    }
    lastBeatRef.current = audioFeatures.isBeat

    // Update explosions
    explosionsRef.current = explosionsRef.current.filter((exp, i) => {
      if (i >= count) return false

      exp.life -= delta * 2
      exp.scale += delta * 30 * (1 - exp.life)

      dummy.position.copy(exp.position)
      dummy.scale.setScalar(exp.scale * exp.life)
      dummy.updateMatrix()
      meshRef.current!.setMatrixAt(i, dummy.matrix)

      colors[i * 3] = exp.color.r
      colors[i * 3 + 1] = exp.color.g
      colors[i * 3 + 2] = exp.color.b

      return exp.life > 0
    })

    // Hide unused instances
    for (let i = explosionsRef.current.length; i < count; i++) {
      dummy.scale.setScalar(0)
      dummy.updateMatrix()
      meshRef.current.setMatrixAt(i, dummy.matrix)
    }

    meshRef.current.instanceMatrix.needsUpdate = true
    meshRef.current.geometry.setAttribute('color', new THREE.InstancedBufferAttribute(colors, 3))
  })

  return (
    <instancedMesh ref={meshRef} args={[undefined, undefined, count]}>
      <sphereGeometry args={[1, 16, 16]} />
      <meshStandardMaterial
        emissive="white"
        emissiveIntensity={3}
        transparent
        opacity={0.6}
        vertexColors
      />
    </instancedMesh>
  )
}

// Central monument that transforms
function CentralMonument({ intensity, phase }: { intensity: number; phase: ScenePhase }) {
  const groupRef = useRef<THREE.Group>(null)
  const coreRef = useRef<THREE.Mesh>(null)
  const ringsRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const genre = useStore((s) => s.genre)

  const genreColors: Record<Genre, [string, string]> = {
    electronic: ['#00ffff', '#ff00ff'],
    classical: ['#ffd700', '#ffffff'],
    metal: ['#ff4500', '#ff0000'],
    ambient: ['#9370db', '#48d1cc'],
    jazz: ['#ff8c00', '#ffa500'],
  }

  const [primary, secondary] = genreColors[genre]

  useFrame((state) => {
    if (!groupRef.current || !coreRef.current || !ringsRef.current) return

    const t = state.clock.elapsedTime

    // Core pulsing
    const pulseScale = 1 + audioFeatures.bass * 0.5 + audioFeatures.beatIntensity * 0.8
    coreRef.current.scale.setScalar(pulseScale * (phase === 'climax' ? 1.5 : 1))

    // Core rotation accelerates with energy
    coreRef.current.rotation.y += 0.01 + intensity * 0.05
    coreRef.current.rotation.x = Math.sin(t * 0.5) * 0.2

    // Rings orbit speed based on energy
    ringsRef.current.rotation.y += 0.005 + intensity * 0.03
    ringsRef.current.rotation.x = Math.sin(t * 0.3) * 0.1
    ringsRef.current.rotation.z = Math.cos(t * 0.4) * 0.1

    // Float height based on phase
    const targetY = phase === 'climax' ? 40 : phase === 'rising' ? 30 : 20
    groupRef.current.position.y = THREE.MathUtils.lerp(groupRef.current.position.y, targetY, 0.02)
  })

  return (
    <group ref={groupRef} position={[0, 20, 0]}>
      {/* Core crystal */}
      <mesh ref={coreRef}>
        <octahedronGeometry args={[5, 2]} />
        <meshStandardMaterial
          color={primary}
          emissive={primary}
          emissiveIntensity={1 + intensity * 2}
          metalness={0.9}
          roughness={0.1}
          wireframe={phase !== 'climax'}
        />
      </mesh>

      {/* Orbiting rings */}
      <group ref={ringsRef}>
        {[8, 12, 16].map((radius, i) => (
          <mesh key={i} rotation={[Math.PI / 2 + i * 0.3, i * 0.5, 0]}>
            <torusGeometry args={[radius, 0.2 + intensity * 0.3, 8, 64]} />
            <meshStandardMaterial
              color={i % 2 === 0 ? primary : secondary}
              emissive={i % 2 === 0 ? primary : secondary}
              emissiveIntensity={0.5 + intensity}
              transparent
              opacity={0.7 + intensity * 0.3}
            />
          </mesh>
        ))}
      </group>

      {/* Point light from core */}
      <pointLight
        color={primary}
        intensity={50 + intensity * 100}
        distance={80}
      />
    </group>
  )
}

// Orbiting entities
function OrbitingEntities({ intensity }: { intensity: number }) {
  const groupRef = useRef<THREE.Group>(null)
  const entitiesRef = useRef<THREE.Mesh[]>([])
  const audioFeatures = useStore((s) => s.audioFeatures)

  const entities = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 12; i++) {
      const orbitRadius = 25 + (i % 3) * 15
      items.push(
        <OrbitingEntity
          key={i}
          index={i}
          orbitRadius={orbitRadius}
          speed={0.2 + (i % 4) * 0.1}
          intensity={intensity}
        />
      )
    }
    return items
  }, [intensity])

  useFrame((state) => {
    if (groupRef.current) {
      groupRef.current.rotation.y += 0.001 + intensity * 0.01
    }
  })

  return <group ref={groupRef}>{entities}</group>
}

function OrbitingEntity({ index, orbitRadius, speed, intensity }: {
  index: number
  orbitRadius: number
  speed: number
  intensity: number
}) {
  const meshRef = useRef<THREE.Mesh>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const genre = useStore((s) => s.genre)

  const shapes = ['box', 'octahedron', 'tetrahedron', 'icosahedron']
  const shape = shapes[index % shapes.length]

  useFrame((state) => {
    if (!meshRef.current) return

    const t = state.clock.elapsedTime
    const angle = t * speed + index * (Math.PI * 2 / 12)

    // Orbit position
    meshRef.current.position.x = Math.cos(angle) * orbitRadius
    meshRef.current.position.z = Math.sin(angle) * orbitRadius
    meshRef.current.position.y = 20 + Math.sin(t * 2 + index) * 10 * intensity

    // Rotate entity
    meshRef.current.rotation.x += 0.02 + audioFeatures.bass * 0.05
    meshRef.current.rotation.y += 0.01

    // Scale on beat
    const beatScale = 1 + audioFeatures.beatIntensity * 0.5
    meshRef.current.scale.setScalar(beatScale * (1 + intensity * 0.5))
  })

  const Geometry = () => {
    switch (shape) {
      case 'octahedron': return <octahedronGeometry args={[2, 0]} />
      case 'tetrahedron': return <tetrahedronGeometry args={[2, 0]} />
      case 'icosahedron': return <icosahedronGeometry args={[2, 0]} />
      default: return <boxGeometry args={[2, 2, 2]} />
    }
  }

  return (
    <mesh ref={meshRef}>
      <Geometry />
      <meshStandardMaterial
        color={new THREE.Color().setHSL(index / 12, 0.8, 0.5)}
        emissive={new THREE.Color().setHSL(index / 12, 1, 0.4)}
        emissiveIntensity={0.5 + intensity}
        metalness={0.8}
        roughness={0.2}
      />
    </mesh>
  )
}

// Dynamic pillars that rise with energy
function DynamicPillars({ intensity, phase }: { intensity: number; phase: ScenePhase }) {
  const pillarsRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  const pillars = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 24; i++) {
      const angle = (i / 24) * Math.PI * 2
      const radius = 50 + (i % 3) * 20
      items.push(
        <DynamicPillar
          key={i}
          position={[Math.cos(angle) * radius, 0, Math.sin(angle) * radius]}
          index={i}
          maxHeight={15 + (i % 5) * 10}
        />
      )
    }
    return items
  }, [])

  return <group ref={pillarsRef}>{pillars}</group>
}

function DynamicPillar({ position, index, maxHeight }: {
  position: [number, number, number]
  index: number
  maxHeight: number
}) {
  const meshRef = useRef<THREE.Mesh>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const genre = useStore((s) => s.genre)

  useFrame((state) => {
    if (!meshRef.current) return

    const t = state.clock.elapsedTime
    const bandIndex = index % 32
    const bandEnergy = audioFeatures.frequencyBands[bandIndex] || 0

    // Height based on frequency band
    const targetHeight = 2 + bandEnergy * maxHeight + audioFeatures.bass * 10
    meshRef.current.scale.y = THREE.MathUtils.lerp(meshRef.current.scale.y, targetHeight / 10, 0.15)
    meshRef.current.position.y = meshRef.current.scale.y * 5

    // Color pulse on beat
    const material = meshRef.current.material as THREE.MeshStandardMaterial
    material.emissiveIntensity = 0.3 + audioFeatures.beatIntensity * 2
  })

  return (
    <mesh ref={meshRef} position={position}>
      <cylinderGeometry args={[1, 1.5, 10, 6]} />
      <meshStandardMaterial
        color={new THREE.Color().setHSL(index / 24, 0.7, 0.3)}
        emissive={new THREE.Color().setHSL(index / 24, 1, 0.5)}
        emissiveIntensity={0.3}
        metalness={0.6}
        roughness={0.4}
      />
    </mesh>
  )
}

// Particle trails
function ParticleTrails({ intensity }: { intensity: number }) {
  const pointsRef = useRef<THREE.Points>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  const count = 2000
  const { positions, velocities, colors, lives } = useMemo(() => {
    const pos = new Float32Array(count * 3)
    const vel = new Float32Array(count * 3)
    const col = new Float32Array(count * 3)
    const lif = new Float32Array(count)

    for (let i = 0; i < count; i++) {
      pos[i * 3] = (Math.random() - 0.5) * 200
      pos[i * 3 + 1] = Math.random() * 100
      pos[i * 3 + 2] = (Math.random() - 0.5) * 200

      vel[i * 3] = (Math.random() - 0.5) * 0.5
      vel[i * 3 + 1] = Math.random() * 2
      vel[i * 3 + 2] = (Math.random() - 0.5) * 0.5

      const hue = Math.random()
      const color = new THREE.Color().setHSL(hue, 1, 0.6)
      col[i * 3] = color.r
      col[i * 3 + 1] = color.g
      col[i * 3 + 2] = color.b

      lif[i] = Math.random()
    }

    return { positions: pos, velocities: vel, colors: col, lives: lif }
  }, [])

  useFrame((state, delta) => {
    if (!pointsRef.current) return

    const pos = pointsRef.current.geometry.attributes.position.array as Float32Array
    const col = pointsRef.current.geometry.attributes.color.array as Float32Array

    for (let i = 0; i < count; i++) {
      // Update position
      pos[i * 3] += velocities[i * 3] * (1 + intensity * 2)
      pos[i * 3 + 1] += velocities[i * 3 + 1] * (1 + audioFeatures.bass * 3)
      pos[i * 3 + 2] += velocities[i * 3 + 2] * (1 + intensity * 2)

      // Update life
      lives[i] -= delta * 0.5

      // Respawn
      if (lives[i] <= 0 || pos[i * 3 + 1] > 100) {
        const angle = Math.random() * Math.PI * 2
        const radius = 20 + Math.random() * 60
        pos[i * 3] = Math.cos(angle) * radius
        pos[i * 3 + 1] = 0
        pos[i * 3 + 2] = Math.sin(angle) * radius
        lives[i] = 1

        // New velocity on beat
        if (audioFeatures.isBeat) {
          velocities[i * 3 + 1] = 2 + Math.random() * 5
        }
      }

      // Color based on height and beat
      const hue = (pos[i * 3 + 1] / 100 + audioFeatures.beatIntensity * 0.2) % 1
      const color = new THREE.Color().setHSL(hue, 1, 0.5 + audioFeatures.beatIntensity * 0.3)
      col[i * 3] = color.r
      col[i * 3 + 1] = color.g
      col[i * 3 + 2] = color.b
    }

    pointsRef.current.geometry.attributes.position.needsUpdate = true
    pointsRef.current.geometry.attributes.color.needsUpdate = true
  })

  return (
    <points ref={pointsRef}>
      <bufferGeometry>
        <bufferAttribute attach="attributes-position" args={[positions, 3]} />
        <bufferAttribute attach="attributes-color" args={[colors, 3]} />
      </bufferGeometry>
      <pointsMaterial
        size={0.8 + intensity}
        vertexColors
        transparent
        opacity={0.8}
        sizeAttenuation
      />
    </points>
  )
}

// Energy waves that pulse outward
function EnergyWaves() {
  const wavesRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const genre = useStore((s) => s.genre)

  const waves = useMemo(() => {
    const items: { scale: number; opacity: number }[] = []
    for (let i = 0; i < 5; i++) {
      items.push({ scale: 1 + i * 20, opacity: 1 - i * 0.2 })
    }
    return items
  }, [])

  const waveRefs = useRef<THREE.Mesh[]>([])
  const lastBeatRef = useRef(false)

  useFrame((state, delta) => {
    // Spawn new wave on beat
    if (audioFeatures.isBeat && !lastBeatRef.current) {
      waveRefs.current.forEach((wave) => {
        if (wave) {
          wave.scale.setScalar(1)
          const mat = wave.material as THREE.MeshStandardMaterial
          mat.opacity = 0.8
        }
      })
    }
    lastBeatRef.current = audioFeatures.isBeat

    // Expand waves
    waveRefs.current.forEach((wave) => {
      if (wave) {
        wave.scale.x += delta * 30 * (1 + audioFeatures.bass)
        wave.scale.z = wave.scale.x
        const material = wave.material as THREE.MeshStandardMaterial
        material.opacity = Math.max(0, material.opacity - delta * 0.5)
      }
    })
  })

  return (
    <group ref={wavesRef} position={[0, 0.5, 0]} rotation={[-Math.PI / 2, 0, 0]}>
      {waves.map((_, i) => (
        <mesh
          key={i}
          ref={(el) => { if (el) waveRefs.current[i] = el }}
        >
          <ringGeometry args={[0.9, 1, 64]} />
          <meshStandardMaterial
            color="#00ffff"
            emissive="#00ffff"
            emissiveIntensity={1}
            transparent
            opacity={0}
            side={THREE.DoubleSide}
          />
        </mesh>
      ))}
    </group>
  )
}

// Floating fragments
function FloatingFragments({ intensity }: { intensity: number }) {
  const groupRef = useRef<THREE.Group>(null)
  const fragmentsRef = useRef<THREE.Mesh[]>([])
  const audioFeatures = useStore((s) => s.audioFeatures)

  const fragments = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 30; i++) {
      items.push(
        <FloatingFragment
          key={i}
          index={i}
          initialPosition={[
            (Math.random() - 0.5) * 100,
            10 + Math.random() * 50,
            (Math.random() - 0.5) * 100,
          ]}
        />
      )
    }
    return items
  }, [])

  return <group ref={groupRef}>{fragments}</group>
}

function FloatingFragment({ index, initialPosition }: {
  index: number
  initialPosition: [number, number, number]
}) {
  const meshRef = useRef<THREE.Mesh>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  useFrame((state) => {
    if (!meshRef.current) return

    const t = state.clock.elapsedTime

    // Float movement
    meshRef.current.position.x = initialPosition[0] + Math.sin(t * 0.5 + index) * 5
    meshRef.current.position.y = initialPosition[1] + Math.sin(t * 0.3 + index * 0.5) * 10
    meshRef.current.position.z = initialPosition[2] + Math.cos(t * 0.4 + index) * 5

    // Rotation
    meshRef.current.rotation.x += 0.01 + audioFeatures.mid * 0.02
    meshRef.current.rotation.y += 0.02

    // Scale pulse
    const scale = 1 + audioFeatures.beatIntensity * 0.5
    meshRef.current.scale.setScalar(scale * (0.5 + Math.random() * 0.5))
  })

  return (
    <mesh ref={meshRef} position={initialPosition}>
      <dodecahedronGeometry args={[1 + Math.random(), 0]} />
      <meshStandardMaterial
        color={new THREE.Color().setHSL(index / 30, 0.6, 0.4)}
        emissive={new THREE.Color().setHSL(index / 30, 1, 0.3)}
        emissiveIntensity={0.5}
        metalness={0.7}
        roughness={0.3}
        transparent
        opacity={0.8}
      />
    </mesh>
  )
}

// Dynamic light beams
function DynamicLightBeams({ phase }: { phase: ScenePhase }) {
  const groupRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  useFrame((state) => {
    if (!groupRef.current) return

    groupRef.current.rotation.y += 0.01 + audioFeatures.bass * 0.02

    groupRef.current.children.forEach((child, i) => {
      if (child instanceof THREE.Mesh) {
        const material = child.material as THREE.MeshStandardMaterial
        material.opacity = 0.1 + audioFeatures.beatIntensity * 0.4
        material.emissiveIntensity = 0.5 + audioFeatures.bass * 2

        child.scale.y = 1 + audioFeatures.high * 0.5
      }
    })
  })

  if (phase !== 'climax' && phase !== 'rising') return null

  return (
    <group ref={groupRef}>
      {[0, 1, 2, 3, 4, 5].map((i) => {
        const angle = (i / 6) * Math.PI * 2
        return (
          <mesh
            key={i}
            position={[Math.cos(angle) * 30, 40, Math.sin(angle) * 30]}
            rotation={[0, 0, Math.cos(angle) * 0.3]}
          >
            <cylinderGeometry args={[0.3, 2, 80, 8]} />
            <meshStandardMaterial
              color={new THREE.Color().setHSL(i / 6, 1, 0.5)}
              emissive={new THREE.Color().setHSL(i / 6, 1, 0.5)}
              emissiveIntensity={1}
              transparent
              opacity={0.3}
            />
          </mesh>
        )
      })}
    </group>
  )
}

// Ground effects
function GroundEffects({ intensity }: { intensity: number }) {
  const meshRef = useRef<THREE.Mesh>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const genre = useStore((s) => s.genre)

  const genreColors: Record<Genre, string> = {
    electronic: '#00ffff',
    classical: '#ffd700',
    metal: '#ff4500',
    ambient: '#9370db',
    jazz: '#ff8c00',
  }

  useFrame((state) => {
    if (!meshRef.current) return

    const material = meshRef.current.material as THREE.MeshStandardMaterial
    material.emissiveIntensity = 0.1 + audioFeatures.bass * 0.5
    material.opacity = 0.3 + intensity * 0.4
  })

  return (
    <mesh ref={meshRef} position={[0, 0.05, 0]} rotation={[-Math.PI / 2, 0, 0]}>
      <circleGeometry args={[150, 64]} />
      <meshStandardMaterial
        color={genreColors[genre]}
        emissive={genreColors[genre]}
        emissiveIntensity={0.2}
        transparent
        opacity={0.3}
      />
    </mesh>
  )
}
