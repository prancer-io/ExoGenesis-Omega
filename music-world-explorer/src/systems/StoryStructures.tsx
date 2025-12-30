/**
 * Story-Driven Structures
 * Each genre tells a narrative through evolving 3D scenes
 * Structures change and progress based on time and audio intensity
 */

import { useRef, useMemo, useState, useEffect } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import { useStore, type Genre } from '../store'

// Story chapter definitions
interface StoryChapter {
  name: string
  duration: number // seconds
  intensity: number // 0-1 scale for structure complexity
}

const storyChapters: Record<Genre, StoryChapter[]> = {
  electronic: [
    { name: 'awakening', duration: 30, intensity: 0.2 },
    { name: 'pulse', duration: 45, intensity: 0.5 },
    { name: 'surge', duration: 60, intensity: 0.8 },
    { name: 'transcendence', duration: 45, intensity: 1.0 },
    { name: 'echo', duration: 30, intensity: 0.6 },
  ],
  classical: [
    { name: 'prelude', duration: 40, intensity: 0.3 },
    { name: 'rising', duration: 50, intensity: 0.5 },
    { name: 'crescendo', duration: 60, intensity: 0.9 },
    { name: 'resolution', duration: 50, intensity: 0.7 },
    { name: 'coda', duration: 30, intensity: 0.4 },
  ],
  metal: [
    { name: 'rumble', duration: 25, intensity: 0.4 },
    { name: 'eruption', duration: 40, intensity: 0.7 },
    { name: 'inferno', duration: 50, intensity: 1.0 },
    { name: 'destruction', duration: 45, intensity: 0.9 },
    { name: 'ashes', duration: 30, intensity: 0.5 },
  ],
  ambient: [
    { name: 'stillness', duration: 60, intensity: 0.2 },
    { name: 'awakening', duration: 50, intensity: 0.4 },
    { name: 'floating', duration: 70, intensity: 0.6 },
    { name: 'enlightenment', duration: 60, intensity: 0.8 },
    { name: 'peace', duration: 50, intensity: 0.5 },
  ],
  jazz: [
    { name: 'twilight', duration: 35, intensity: 0.3 },
    { name: 'gathering', duration: 45, intensity: 0.5 },
    { name: 'improvisation', duration: 55, intensity: 0.8 },
    { name: 'crescendo', duration: 50, intensity: 1.0 },
    { name: 'nightcap', duration: 40, intensity: 0.4 },
  ],
}

// Hook to track story progression
function useStoryProgression(genre: Genre) {
  const [chapterIndex, setChapterIndex] = useState(0)
  const [chapterProgress, setChapterProgress] = useState(0)
  const startTimeRef = useRef(0)
  const chapters = storyChapters[genre]

  useEffect(() => {
    startTimeRef.current = Date.now()
    setChapterIndex(0)
    setChapterProgress(0)
  }, [genre])

  useFrame(() => {
    const elapsed = (Date.now() - startTimeRef.current) / 1000
    let totalTime = 0

    for (let i = 0; i < chapters.length; i++) {
      if (elapsed < totalTime + chapters[i].duration) {
        if (i !== chapterIndex) setChapterIndex(i)
        const progress = (elapsed - totalTime) / chapters[i].duration
        setChapterProgress(progress)
        return
      }
      totalTime += chapters[i].duration
    }

    // Loop back to beginning
    startTimeRef.current = Date.now()
    setChapterIndex(0)
  })

  return {
    chapter: chapters[chapterIndex],
    chapterIndex,
    chapterProgress,
    totalChapters: chapters.length,
  }
}

// Electronic: Digital Awakening Story
export function ElectronicStory() {
  const groupRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const { chapter, chapterProgress } = useStoryProgression('electronic')

  // Dynamic building heights based on chapter
  const buildings = useMemo(() => {
    const items: JSX.Element[] = []
    const gridSize = 12
    const spacing = 14

    for (let x = -gridSize; x <= gridSize; x++) {
      for (let z = -gridSize; z <= gridSize; z++) {
        if (Math.random() > 0.55) continue
        const baseHeight = 8 + Math.random() * 35
        const width = 3 + Math.random() * 4
        const hue = (x + z) * 0.02 + 0.5

        items.push(
          <StoryBuilding
            key={`building-${x}-${z}`}
            position={[x * spacing, 0, z * spacing]}
            baseHeight={baseHeight}
            width={width}
            hue={hue}
          />
        )
      }
    }
    return items
  }, [])

  // Data streams that appear in later chapters
  const dataStreams = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 20; i++) {
      const angle = (i / 20) * Math.PI * 2
      const radius = 50 + Math.random() * 40
      items.push(
        <DataStream
          key={`stream-${i}`}
          startPos={[Math.cos(angle) * radius, -10, Math.sin(angle) * radius]}
          endPos={[Math.cos(angle) * radius * 0.3, 80, Math.sin(angle) * radius * 0.3]}
        />
      )
    }
    return items
  }, [])

  // Central nexus that grows with story
  const nexusScale = 0.5 + chapter.intensity * 1.5

  return (
    <group ref={groupRef}>
      {buildings}

      {/* Data streams visible after chapter 1 */}
      {chapter.intensity > 0.4 && dataStreams}

      {/* Central Nexus - grows with story */}
      <group scale={nexusScale}>
        <mesh position={[0, 30, 0]}>
          <icosahedronGeometry args={[10, 2]} />
          <meshStandardMaterial
            color="#001133"
            emissive="#00ffff"
            emissiveIntensity={0.5 + audioFeatures.beatIntensity}
            wireframe={chapter.intensity < 0.8}
            metalness={0.9}
            roughness={0.1}
          />
        </mesh>

        {/* Energy rings */}
        {[1, 2, 3].map((ring) => (
          <mesh key={ring} position={[0, 30, 0]} rotation={[Math.PI / 2, 0, 0]}>
            <torusGeometry args={[15 + ring * 8, 0.3, 8, 64]} />
            <meshStandardMaterial
              color="#00ffff"
              emissive="#00ffff"
              emissiveIntensity={chapter.intensity * 2}
              transparent
              opacity={0.3 + chapter.intensity * 0.4}
            />
          </mesh>
        ))}
      </group>

      {/* Ground grid pulses with chapter */}
      <gridHelper
        args={[350, 70,
          new THREE.Color().setHSL(0.5 + chapter.intensity * 0.1, 1, 0.5),
          '#001122'
        ]}
        position={[0, 0.1, 0]}
      />

      {/* Story title overlay hint */}
      <ChapterIndicator name={chapter.name} progress={chapterProgress} />
    </group>
  )
}

// Animated building that responds to story
function StoryBuilding({ position, baseHeight, width, hue }: {
  position: [number, number, number]
  baseHeight: number
  width: number
  hue: number
}) {
  const meshRef = useRef<THREE.Mesh>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const { chapter } = useStoryProgression('electronic')

  useFrame((state) => {
    if (meshRef.current) {
      const height = baseHeight * (0.3 + chapter.intensity * 0.7)
      meshRef.current.scale.y = 0.5 + height / baseHeight * 0.5 + audioFeatures.bass * 0.2
      meshRef.current.position.y = (height * meshRef.current.scale.y) / 2
    }
  })

  return (
    <group position={position}>
      <mesh ref={meshRef}>
        <boxGeometry args={[width, baseHeight, width]} />
        <meshStandardMaterial
          color={new THREE.Color().setHSL(hue, 0.8, 0.1)}
          emissive={new THREE.Color().setHSL(hue, 1, 0.4)}
          emissiveIntensity={0.3 + chapter.intensity * 0.5}
          metalness={0.9}
          roughness={0.1}
        />
      </mesh>
    </group>
  )
}

// Data stream particles
function DataStream({ startPos, endPos }: { startPos: number[], endPos: number[] }) {
  const ref = useRef<THREE.Points>(null)

  const { positions, colors } = useMemo(() => {
    const count = 50
    const pos = new Float32Array(count * 3)
    const col = new Float32Array(count * 3)

    for (let i = 0; i < count; i++) {
      const t = i / count
      pos[i * 3] = startPos[0] + (endPos[0] - startPos[0]) * t
      pos[i * 3 + 1] = startPos[1] + (endPos[1] - startPos[1]) * t
      pos[i * 3 + 2] = startPos[2] + (endPos[2] - startPos[2]) * t

      col[i * 3] = 0
      col[i * 3 + 1] = 1
      col[i * 3 + 2] = 1
    }

    return { positions: pos, colors: col }
  }, [startPos, endPos])

  useFrame((state) => {
    if (ref.current) {
      const pos = ref.current.geometry.attributes.position.array as Float32Array
      for (let i = 0; i < pos.length / 3; i++) {
        pos[i * 3 + 1] += 0.5
        if (pos[i * 3 + 1] > endPos[1]) {
          pos[i * 3 + 1] = startPos[1]
        }
      }
      ref.current.geometry.attributes.position.needsUpdate = true
    }
  })

  return (
    <points ref={ref}>
      <bufferGeometry>
        <bufferAttribute attach="attributes-position" args={[positions, 3]} />
        <bufferAttribute attach="attributes-color" args={[colors, 3]} />
      </bufferGeometry>
      <pointsMaterial size={0.5} vertexColors transparent opacity={0.8} />
    </points>
  )
}

// Classical: Symphony of Light Story
export function ClassicalStory() {
  const groupRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const { chapter, chapterProgress } = useStoryProgression('classical')

  // Cathedral pillars that rise with the story
  const pillars = useMemo(() => {
    const items: JSX.Element[] = []
    const count = 24
    const radius = 45

    for (let i = 0; i < count; i++) {
      const angle = (i / count) * Math.PI * 2
      items.push(
        <RisingPillar
          key={`pillar-${i}`}
          position={[Math.cos(angle) * radius, 0, Math.sin(angle) * radius]}
          delay={i * 0.1}
          maxHeight={25 + Math.sin(angle * 4) * 10}
        />
      )
    }
    return items
  }, [])

  // Angelic figures that appear in crescendo
  const angels = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 8; i++) {
      const angle = (i / 8) * Math.PI * 2
      items.push(
        <AngelicFigure
          key={`angel-${i}`}
          position={[Math.cos(angle) * 25, 35, Math.sin(angle) * 25]}
          rotation={[0, -angle + Math.PI, 0]}
        />
      )
    }
    return items
  }, [])

  // Light beams from above
  const lightBeams = chapter.intensity > 0.5

  return (
    <group ref={groupRef}>
      {pillars}

      {/* Angels appear during crescendo */}
      {chapter.intensity > 0.7 && angels}

      {/* Central altar grows */}
      <group scale={0.5 + chapter.intensity * 0.8}>
        <mesh position={[0, 5, 0]}>
          <cylinderGeometry args={[8, 10, 10, 8]} />
          <meshStandardMaterial
            color="#f5f5dc"
            emissive="#ffd700"
            emissiveIntensity={chapter.intensity * 0.5}
            metalness={0.4}
            roughness={0.3}
          />
        </mesh>

        {/* Sacred flame */}
        <mesh position={[0, 12, 0]}>
          <coneGeometry args={[2, 8, 8]} />
          <meshStandardMaterial
            color="#ffd700"
            emissive="#ffa500"
            emissiveIntensity={1 + audioFeatures.beatIntensity * 2}
            transparent
            opacity={0.8}
          />
        </mesh>
      </group>

      {/* Light beams from heaven */}
      {lightBeams && (
        <group>
          {[0, 1, 2, 3].map((i) => {
            const angle = (i / 4) * Math.PI * 2
            return (
              <mesh
                key={`beam-${i}`}
                position={[Math.cos(angle) * 15, 40, Math.sin(angle) * 15]}
                rotation={[0.2 * Math.cos(angle), 0, 0.2 * Math.sin(angle)]}
              >
                <cylinderGeometry args={[0.5, 3, 80, 8]} />
                <meshStandardMaterial
                  color="#ffd700"
                  emissive="#ffd700"
                  emissiveIntensity={2}
                  transparent
                  opacity={0.3 + chapter.intensity * 0.3}
                />
              </mesh>
            )
          })}
        </group>
      )}

      {/* Marble floor */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.01, 0]}>
        <circleGeometry args={[100, 64]} />
        <meshStandardMaterial color="#f5f5dc" metalness={0.3} roughness={0.4} />
      </mesh>

      <ChapterIndicator name={chapter.name} progress={chapterProgress} />
    </group>
  )
}

// Rising pillar animation
function RisingPillar({ position, delay, maxHeight }: {
  position: [number, number, number]
  delay: number
  maxHeight: number
}) {
  const groupRef = useRef<THREE.Group>(null)
  const { chapter } = useStoryProgression('classical')

  useFrame((state) => {
    if (groupRef.current) {
      const t = Math.max(0, state.clock.elapsedTime - delay)
      const targetHeight = chapter.intensity * maxHeight
      const currentHeight = THREE.MathUtils.lerp(
        groupRef.current.scale.y,
        0.1 + (targetHeight / maxHeight),
        0.02
      )
      groupRef.current.scale.y = currentHeight
    }
  })

  return (
    <group ref={groupRef} position={position}>
      {/* Column */}
      <mesh position={[0, maxHeight / 2, 0]}>
        <cylinderGeometry args={[1.2, 1.5, maxHeight, 12]} />
        <meshStandardMaterial color="#fffaf0" metalness={0.2} roughness={0.4} />
      </mesh>

      {/* Capital */}
      <mesh position={[0, maxHeight + 1, 0]}>
        <cylinderGeometry args={[2, 1.2, 2, 8]} />
        <meshStandardMaterial
          color="#ffd700"
          emissive="#ffd700"
          emissiveIntensity={0.3}
          metalness={0.8}
          roughness={0.2}
        />
      </mesh>
    </group>
  )
}

// Angelic figure silhouette
function AngelicFigure({ position, rotation }: {
  position: [number, number, number]
  rotation: [number, number, number]
}) {
  const ref = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  useFrame((state) => {
    if (ref.current) {
      ref.current.position.y = position[1] + Math.sin(state.clock.elapsedTime) * 2
    }
  })

  return (
    <group ref={ref} position={position} rotation={rotation}>
      {/* Body */}
      <mesh>
        <coneGeometry args={[2, 6, 8]} />
        <meshStandardMaterial
          color="#ffd700"
          emissive="#ffd700"
          emissiveIntensity={1 + audioFeatures.rms}
          transparent
          opacity={0.6}
        />
      </mesh>

      {/* Wings */}
      <mesh position={[-2, 2, 0]} rotation={[0, 0, 0.3]}>
        <planeGeometry args={[4, 3]} />
        <meshStandardMaterial
          color="#ffffff"
          emissive="#ffd700"
          emissiveIntensity={0.5}
          transparent
          opacity={0.5}
          side={THREE.DoubleSide}
        />
      </mesh>
      <mesh position={[2, 2, 0]} rotation={[0, 0, -0.3]}>
        <planeGeometry args={[4, 3]} />
        <meshStandardMaterial
          color="#ffffff"
          emissive="#ffd700"
          emissiveIntensity={0.5}
          transparent
          opacity={0.5}
          side={THREE.DoubleSide}
        />
      </mesh>
    </group>
  )
}

// Metal: Forge of Destruction Story
export function MetalStory() {
  const groupRef = useRef<THREE.Group>(null)
  const { chapter, chapterProgress } = useStoryProgression('metal')
  const audioFeatures = useStore((s) => s.audioFeatures)

  // Volcanic eruption intensity
  const eruptionIntensity = chapter.intensity

  // Lava shader
  const lavaMaterial = useMemo(() => {
    return new THREE.ShaderMaterial({
      uniforms: {
        uTime: { value: 0 },
        uIntensity: { value: 0.5 },
      },
      vertexShader: `
        varying vec2 vUv;
        varying float vElevation;
        uniform float uTime;
        uniform float uIntensity;

        void main() {
          vUv = uv;
          vec3 pos = position;

          // Bubbling effect
          float bubble = sin(pos.x * 3.0 + uTime * 2.0) * sin(pos.y * 3.0 + uTime * 1.5);
          pos.z += bubble * 0.5 * uIntensity;
          vElevation = bubble;

          gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
        }
      `,
      fragmentShader: `
        uniform float uTime;
        uniform float uIntensity;
        varying vec2 vUv;
        varying float vElevation;

        void main() {
          vec2 uv = vUv;

          float flow = sin(uv.x * 8.0 + uTime * 3.0) * cos(uv.y * 8.0 + uTime * 2.0);
          flow = flow * 0.5 + 0.5;

          vec3 hotColor = vec3(1.0, 0.9, 0.3);
          vec3 coolColor = vec3(0.8, 0.2, 0.0);
          vec3 lavaColor = mix(coolColor, hotColor, flow * uIntensity + vElevation * 0.5);

          lavaColor += vec3(0.3, 0.1, 0.0) * uIntensity;

          gl_FragColor = vec4(lavaColor, 1.0);
        }
      `,
    })
  }, [])

  useFrame((state) => {
    lavaMaterial.uniforms.uTime.value = state.clock.elapsedTime
    lavaMaterial.uniforms.uIntensity.value = eruptionIntensity + audioFeatures.bass * 0.3
  })

  // Erupting rocks
  const eruptingRocks = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 40; i++) {
      items.push(
        <EruptingRock
          key={`rock-${i}`}
          delay={i * 0.5}
          spread={80}
        />
      )
    }
    return items
  }, [])

  return (
    <group ref={groupRef}>
      {/* Main volcano */}
      <mesh position={[0, 20, 0]}>
        <coneGeometry args={[30, 40, 8, 1, true]} />
        <meshStandardMaterial
          color="#1a0500"
          emissive="#330000"
          emissiveIntensity={0.2 + eruptionIntensity * 0.5}
          side={THREE.DoubleSide}
        />
      </mesh>

      {/* Crater lava */}
      <mesh position={[0, 38, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <circleGeometry args={[12, 32]} />
        <primitive object={lavaMaterial} attach="material" />
      </mesh>

      {/* Lava rivers */}
      {[0, 1, 2, 3].map((i) => {
        const angle = (i / 4) * Math.PI * 2 + 0.3
        return (
          <mesh
            key={`river-${i}`}
            position={[Math.cos(angle) * 30, 1, Math.sin(angle) * 30]}
            rotation={[-Math.PI / 2, 0, angle]}
          >
            <planeGeometry args={[6, 60]} />
            <primitive object={lavaMaterial.clone()} attach="material" />
          </mesh>
        )
      })}

      {/* Erupting rocks during inferno */}
      {eruptionIntensity > 0.6 && eruptingRocks}

      {/* Smoke/ash clouds */}
      <SmokeCloud intensity={eruptionIntensity} />

      {/* Rock spires */}
      {Array.from({ length: 30 }).map((_, i) => {
        const angle = Math.random() * Math.PI * 2
        const radius = 40 + Math.random() * 60
        return (
          <mesh
            key={`spire-${i}`}
            position={[Math.cos(angle) * radius, Math.random() * 15, Math.sin(angle) * radius]}
            rotation={[(Math.random() - 0.5) * 0.3, 0, (Math.random() - 0.5) * 0.3]}
          >
            <coneGeometry args={[2 + Math.random() * 3, 10 + Math.random() * 20, 6]} />
            <meshStandardMaterial
              color="#1a0500"
              emissive="#330000"
              emissiveIntensity={0.1}
            />
          </mesh>
        )
      })}

      <ChapterIndicator name={chapter.name} progress={chapterProgress} />
    </group>
  )
}

// Erupting rock particle
function EruptingRock({ delay, spread }: { delay: number; spread: number }) {
  const ref = useRef<THREE.Mesh>(null)
  const velocityRef = useRef({ x: 0, y: 0, z: 0 })
  const { chapter } = useStoryProgression('metal')

  useFrame((state) => {
    if (!ref.current) return

    const t = state.clock.elapsedTime

    // Reset and launch
    if (ref.current.position.y < 0 && chapter.intensity > 0.5) {
      ref.current.position.set(
        (Math.random() - 0.5) * 10,
        40,
        (Math.random() - 0.5) * 10
      )
      velocityRef.current = {
        x: (Math.random() - 0.5) * 3,
        y: 5 + Math.random() * 10,
        z: (Math.random() - 0.5) * 3,
      }
    }

    // Physics
    velocityRef.current.y -= 0.15 // gravity
    ref.current.position.x += velocityRef.current.x * 0.1
    ref.current.position.y += velocityRef.current.y * 0.1
    ref.current.position.z += velocityRef.current.z * 0.1
    ref.current.rotation.x += 0.1
    ref.current.rotation.z += 0.05
  })

  return (
    <mesh ref={ref} position={[0, -20, 0]}>
      <dodecahedronGeometry args={[1 + Math.random(), 0]} />
      <meshStandardMaterial
        color="#1a0500"
        emissive="#ff4500"
        emissiveIntensity={1}
      />
    </mesh>
  )
}

// Smoke cloud effect
function SmokeCloud({ intensity }: { intensity: number }) {
  const ref = useRef<THREE.Points>(null)

  const { positions, sizes } = useMemo(() => {
    const count = 200
    const pos = new Float32Array(count * 3)
    const sz = new Float32Array(count)

    for (let i = 0; i < count; i++) {
      const angle = Math.random() * Math.PI * 2
      const radius = Math.random() * 20
      pos[i * 3] = Math.cos(angle) * radius
      pos[i * 3 + 1] = 40 + Math.random() * 40
      pos[i * 3 + 2] = Math.sin(angle) * radius
      sz[i] = 2 + Math.random() * 4
    }

    return { positions: pos, sizes: sz }
  }, [])

  useFrame((state) => {
    if (!ref.current) return
    const pos = ref.current.geometry.attributes.position.array as Float32Array

    for (let i = 0; i < pos.length / 3; i++) {
      pos[i * 3 + 1] += 0.2 * intensity
      if (pos[i * 3 + 1] > 100) {
        pos[i * 3 + 1] = 40
      }
      pos[i * 3] += (Math.random() - 0.5) * 0.5
      pos[i * 3 + 2] += (Math.random() - 0.5) * 0.5
    }
    ref.current.geometry.attributes.position.needsUpdate = true
  })

  return (
    <points ref={ref}>
      <bufferGeometry>
        <bufferAttribute attach="attributes-position" args={[positions, 3]} />
        <bufferAttribute attach="attributes-size" args={[sizes, 1]} />
      </bufferGeometry>
      <pointsMaterial
        size={3}
        color="#333333"
        transparent
        opacity={0.3 * intensity}
        sizeAttenuation
      />
    </points>
  )
}

// Ambient: Journey to Enlightenment Story
export function AmbientStory() {
  const groupRef = useRef<THREE.Group>(null)
  const { chapter, chapterProgress } = useStoryProgression('ambient')
  const audioFeatures = useStore((s) => s.audioFeatures)

  // Floating islands appear progressively
  const islands = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 20; i++) {
      items.push(
        <FloatingIsland
          key={`island-${i}`}
          basePosition={[
            (Math.random() - 0.5) * 150,
            10 + Math.random() * 50,
            (Math.random() - 0.5) * 150,
          ]}
          appearThreshold={i / 20}
          scale={1 + Math.random() * 2}
        />
      )
    }
    return items
  }, [])

  // Wisdom orbs
  const wisdomOrbs = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 12; i++) {
      const angle = (i / 12) * Math.PI * 2
      items.push(
        <WisdomOrb
          key={`orb-${i}`}
          position={[Math.cos(angle) * 30, 25 + Math.sin(angle * 2) * 10, Math.sin(angle) * 30]}
          color={new THREE.Color().setHSL(i / 12, 0.5, 0.6)}
        />
      )
    }
    return items
  }, [])

  return (
    <group ref={groupRef}>
      {islands}

      {/* Wisdom orbs appear during enlightenment */}
      {chapter.intensity > 0.6 && wisdomOrbs}

      {/* Central tree of life */}
      <group scale={0.5 + chapter.intensity * 0.8}>
        {/* Trunk */}
        <mesh position={[0, 15, 0]}>
          <cylinderGeometry args={[2, 4, 30, 8]} />
          <meshStandardMaterial
            color="#4a2545"
            emissive="#9370db"
            emissiveIntensity={0.2}
          />
        </mesh>

        {/* Canopy layers */}
        {[0, 1, 2].map((layer) => (
          <mesh key={layer} position={[0, 30 + layer * 8, 0]}>
            <sphereGeometry args={[10 - layer * 2, 16, 16]} />
            <meshStandardMaterial
              color={new THREE.Color().setHSL(0.8 - layer * 0.05, 0.5, 0.4)}
              emissive={new THREE.Color().setHSL(0.8 - layer * 0.05, 0.8, 0.4)}
              emissiveIntensity={0.3 + chapter.intensity * 0.5}
              transparent
              opacity={0.7}
            />
          </mesh>
        ))}
      </group>

      {/* Ethereal mist layers */}
      {[5, 15, 25].map((height, i) => (
        <mesh
          key={`mist-${i}`}
          position={[0, height, 0]}
          rotation={[-Math.PI / 2, 0, 0]}
        >
          <planeGeometry args={[200, 200]} />
          <meshStandardMaterial
            color="#9370db"
            transparent
            opacity={0.05 + chapter.intensity * 0.05}
            side={THREE.DoubleSide}
          />
        </mesh>
      ))}

      {/* Starfield below (reflection) */}
      <points>
        <bufferGeometry>
          <bufferAttribute
            attach="attributes-position"
            args={[new Float32Array(
              Array.from({ length: 500 * 3 }, (_, i) => {
                const idx = Math.floor(i / 3)
                if (i % 3 === 0) return (Math.random() - 0.5) * 200
                if (i % 3 === 1) return -5 - Math.random() * 20
                return (Math.random() - 0.5) * 200
              })
            ), 3]}
          />
        </bufferGeometry>
        <pointsMaterial
          size={0.5}
          color="#9370db"
          transparent
          opacity={0.5}
        />
      </points>

      <ChapterIndicator name={chapter.name} progress={chapterProgress} />
    </group>
  )
}

// Floating island with progressive appearance
function FloatingIsland({ basePosition, appearThreshold, scale }: {
  basePosition: number[]
  appearThreshold: number
  scale: number
}) {
  const ref = useRef<THREE.Group>(null)
  const { chapter } = useStoryProgression('ambient')
  const visible = chapter.intensity >= appearThreshold

  useFrame((state) => {
    if (ref.current && visible) {
      ref.current.position.y = basePosition[1] + Math.sin(state.clock.elapsedTime * 0.5 + appearThreshold * 10) * 3
      ref.current.rotation.y += 0.001
    }
  })

  if (!visible) return null

  return (
    <group ref={ref} position={basePosition as [number, number, number]} scale={scale}>
      {/* Island body */}
      <mesh>
        <sphereGeometry args={[3, 12, 12, 0, Math.PI * 2, 0, Math.PI / 2]} />
        <meshStandardMaterial
          color="#9370db"
          emissive="#4b0082"
          emissiveIntensity={0.3}
        />
      </mesh>

      {/* Crystal on top */}
      <mesh position={[0, 1.5, 0]}>
        <octahedronGeometry args={[1, 0]} />
        <meshStandardMaterial
          color="#48d1cc"
          emissive="#48d1cc"
          emissiveIntensity={0.5}
          transparent
          opacity={0.8}
        />
      </mesh>
    </group>
  )
}

// Wisdom orb with pulsing glow
function WisdomOrb({ position, color }: {
  position: [number, number, number]
  color: THREE.Color
}) {
  const ref = useRef<THREE.Mesh>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  useFrame((state) => {
    if (ref.current) {
      ref.current.position.y = position[1] + Math.sin(state.clock.elapsedTime + position[0]) * 2
      const scale = 1 + audioFeatures.rms * 0.3
      ref.current.scale.setScalar(scale)
    }
  })

  return (
    <mesh ref={ref} position={position}>
      <sphereGeometry args={[3, 32, 32]} />
      <meshStandardMaterial
        color={color}
        emissive={color}
        emissiveIntensity={1}
        transparent
        opacity={0.5}
      />
    </mesh>
  )
}

// Jazz: Night at the Club Story
export function JazzStory() {
  const groupRef = useRef<THREE.Group>(null)
  const { chapter, chapterProgress } = useStoryProgression('jazz')
  const audioFeatures = useStore((s) => s.audioFeatures)

  // Crowd density increases with story
  const crowdDensity = Math.floor(10 + chapter.intensity * 20)

  // Stage spotlight colors change
  const spotlightHue = chapterProgress * 0.2

  return (
    <group ref={groupRef}>
      {/* Club floor */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0, 0]}>
        <planeGeometry args={[120, 120]} />
        <meshStandardMaterial color="#1a1008" roughness={0.8} />
      </mesh>

      {/* Stage */}
      <group position={[0, 0, -30]}>
        <mesh position={[0, 1, 0]}>
          <boxGeometry args={[30, 2, 15]} />
          <meshStandardMaterial color="#2a1810" />
        </mesh>

        {/* Piano */}
        <mesh position={[-8, 3.5, 2]}>
          <boxGeometry args={[6, 3, 3]} />
          <meshStandardMaterial color="#0a0a0a" />
        </mesh>

        {/* Drummer */}
        <mesh position={[8, 3, 0]}>
          <cylinderGeometry args={[2, 2, 1.5, 16]} />
          <meshStandardMaterial color="#b8860b" metalness={0.8} />
        </mesh>

        {/* Bass player silhouette */}
        <mesh position={[0, 5, 0]}>
          <capsuleGeometry args={[0.5, 4, 4, 8]} />
          <meshStandardMaterial color="#1a1a1a" />
        </mesh>

        {/* Spotlights */}
        <pointLight
          position={[-8, 10, 5]}
          color={new THREE.Color().setHSL(0.08 + spotlightHue, 1, 0.5)}
          intensity={30 + audioFeatures.beatIntensity * 50}
          distance={25}
        />
        <pointLight
          position={[0, 10, 5]}
          color={new THREE.Color().setHSL(0.12 + spotlightHue, 0.8, 0.6)}
          intensity={40 + audioFeatures.rms * 30}
          distance={25}
        />
        <pointLight
          position={[8, 10, 5]}
          color={new THREE.Color().setHSL(0.06 + spotlightHue, 1, 0.5)}
          intensity={30 + audioFeatures.beatIntensity * 50}
          distance={25}
        />
      </group>

      {/* Tables with candles */}
      {Array.from({ length: crowdDensity }).map((_, i) => {
        const x = (Math.random() - 0.5) * 80
        const z = 10 + Math.random() * 50
        if (Math.abs(x) < 10 && z < 20) return null

        return (
          <group key={`table-${i}`} position={[x, 0, z]}>
            {/* Table */}
            <mesh position={[0, 2.5, 0]}>
              <cylinderGeometry args={[2, 2, 0.2, 12]} />
              <meshStandardMaterial color="#5c4033" />
            </mesh>
            <mesh position={[0, 1.25, 0]}>
              <cylinderGeometry args={[0.2, 0.2, 2.5, 8]} />
              <meshStandardMaterial color="#3d2817" />
            </mesh>

            {/* Candle */}
            <CandleFlame position={[0, 2.8, 0]} />

            {/* Glasses */}
            {chapter.intensity > 0.4 && (
              <>
                <mesh position={[0.5, 2.7, 0.3]}>
                  <cylinderGeometry args={[0.15, 0.1, 0.4, 8]} />
                  <meshStandardMaterial color="#4a3728" transparent opacity={0.6} />
                </mesh>
                <mesh position={[-0.4, 2.7, -0.2]}>
                  <cylinderGeometry args={[0.15, 0.1, 0.4, 8]} />
                  <meshStandardMaterial color="#4a3728" transparent opacity={0.6} />
                </mesh>
              </>
            )}
          </group>
        )
      })}

      {/* Bar */}
      <mesh position={[0, 2, 50]}>
        <boxGeometry args={[40, 4, 3]} />
        <meshStandardMaterial color="#4a3728" metalness={0.3} />
      </mesh>

      {/* Bar bottles */}
      {Array.from({ length: 15 }).map((_, i) => (
        <mesh key={`bottle-${i}`} position={[-15 + i * 2, 5, 49]}>
          <cylinderGeometry args={[0.2, 0.2, 1.2, 8]} />
          <meshStandardMaterial
            color={new THREE.Color().setHSL(Math.random() * 0.1 + 0.05, 0.8, 0.3)}
            transparent
            opacity={0.8}
          />
        </mesh>
      ))}

      {/* Hanging lamps */}
      {Array.from({ length: 8 }).map((_, i) => {
        const x = (Math.random() - 0.5) * 60
        const z = Math.random() * 60
        return (
          <group key={`lamp-${i}`} position={[x, 10, z]}>
            <mesh>
              <sphereGeometry args={[0.6, 12, 12]} />
              <meshStandardMaterial
                color="#ff8c00"
                emissive="#ff6600"
                emissiveIntensity={0.8 + Math.sin(chapter.intensity * Math.PI) * 0.5}
                transparent
                opacity={0.9}
              />
            </mesh>
            <pointLight
              color="#ff8c00"
              intensity={15 + audioFeatures.rms * 10}
              distance={12}
            />
          </group>
        )
      })}

      {/* Smoke effect during later chapters */}
      {chapter.intensity > 0.5 && (
        <mesh position={[0, 4, 20]} rotation={[-Math.PI / 2, 0, 0]}>
          <planeGeometry args={[100, 80]} />
          <meshStandardMaterial
            color="#333333"
            transparent
            opacity={0.1 * chapter.intensity}
            side={THREE.DoubleSide}
          />
        </mesh>
      )}

      <ChapterIndicator name={chapter.name} progress={chapterProgress} />
    </group>
  )
}

// Candle flame with flicker
function CandleFlame({ position }: { position: [number, number, number] }) {
  const ref = useRef<THREE.Mesh>(null)
  const lightRef = useRef<THREE.PointLight>(null)

  useFrame((state) => {
    if (ref.current && lightRef.current) {
      const flicker = Math.sin(state.clock.elapsedTime * 15) * 0.1 +
                      Math.sin(state.clock.elapsedTime * 23) * 0.05
      ref.current.scale.y = 1 + flicker
      lightRef.current.intensity = 3 + flicker * 5
    }
  })

  return (
    <group position={position}>
      <mesh ref={ref}>
        <coneGeometry args={[0.1, 0.3, 8]} />
        <meshStandardMaterial
          color="#ffa500"
          emissive="#ff6600"
          emissiveIntensity={2}
        />
      </mesh>
      <pointLight ref={lightRef} color="#ff8c00" intensity={3} distance={5} />
    </group>
  )
}

// Chapter indicator (subtle 3D text position marker)
function ChapterIndicator({ name, progress }: { name: string; progress: number }) {
  // This is a placeholder - the actual chapter name is shown in the UI
  // Here we just create a subtle visual marker
  return (
    <mesh position={[0, 0.05, 0]} rotation={[-Math.PI / 2, 0, 0]}>
      <ringGeometry args={[95 + progress * 5, 96 + progress * 5, 64]} />
      <meshStandardMaterial
        color="#ffffff"
        emissive="#ffffff"
        emissiveIntensity={0.3}
        transparent
        opacity={0.2}
      />
    </mesh>
  )
}

// Main story structure selector
export function StoryStructures() {
  const genre = useStore((s) => s.genre)

  switch (genre) {
    case 'electronic':
      return <ElectronicStory />
    case 'classical':
      return <ClassicalStory />
    case 'metal':
      return <MetalStory />
    case 'ambient':
      return <AmbientStory />
    case 'jazz':
      return <JazzStory />
    default:
      return <ElectronicStory />
  }
}
