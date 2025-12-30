/**
 * Genre-Specific Stunning Structures
 * Creates immersive environments for each music genre
 */

import { useRef, useMemo } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import { useStore, type Genre } from '../store'

// Electronic: Neon Cityscape
export function ElectronicCity() {
  const groupRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  const buildings = useMemo(() => {
    const items: JSX.Element[] = []
    const gridSize = 10
    const spacing = 15

    for (let x = -gridSize; x <= gridSize; x++) {
      for (let z = -gridSize; z <= gridSize; z++) {
        if (Math.random() > 0.6) continue

        const height = 10 + Math.random() * 40
        const width = 3 + Math.random() * 5
        const hue = Math.random()

        items.push(
          <group key={`building-${x}-${z}`} position={[x * spacing, height / 2, z * spacing]}>
            {/* Main building */}
            <mesh>
              <boxGeometry args={[width, height, width]} />
              <meshStandardMaterial
                color={new THREE.Color().setHSL(hue, 1, 0.1)}
                emissive={new THREE.Color().setHSL(hue, 1, 0.5)}
                emissiveIntensity={0.5}
                metalness={0.9}
                roughness={0.1}
              />
            </mesh>

            {/* Neon edges */}
            <lineSegments>
              <edgesGeometry args={[new THREE.BoxGeometry(width + 0.1, height + 0.1, width + 0.1)]} />
              <lineBasicMaterial color={new THREE.Color().setHSL(hue, 1, 0.7)} linewidth={2} />
            </lineSegments>

            {/* Top beacon */}
            <mesh position={[0, height / 2 + 1, 0]}>
              <sphereGeometry args={[0.5, 16, 16]} />
              <meshStandardMaterial
                color={new THREE.Color().setHSL(hue, 1, 0.5)}
                emissive={new THREE.Color().setHSL(hue, 1, 0.8)}
                emissiveIntensity={2}
              />
            </mesh>
          </group>
        )
      }
    }

    return items
  }, [])

  // Rotating light beams
  const beams = useMemo(() => {
    const items: JSX.Element[] = []
    for (let i = 0; i < 8; i++) {
      const angle = (i / 8) * Math.PI * 2
      const radius = 80
      items.push(
        <mesh
          key={`beam-${i}`}
          position={[Math.cos(angle) * radius, 25, Math.sin(angle) * radius]}
          rotation={[0, 0, Math.PI / 8]}
        >
          <cylinderGeometry args={[0.2, 0.2, 60, 8]} />
          <meshStandardMaterial
            color="#00ffff"
            emissive="#00ffff"
            emissiveIntensity={3}
            transparent
            opacity={0.6}
          />
        </mesh>
      )
    }
    return items
  }, [])

  useFrame((state) => {
    if (groupRef.current) {
      // Subtle rotation based on audio
      groupRef.current.rotation.y = state.clock.elapsedTime * 0.02
    }
  })

  return (
    <group ref={groupRef}>
      {buildings}
      {beams}

      {/* Ground grid */}
      <gridHelper args={[300, 60, '#00ffff', '#001133']} position={[0, 0.1, 0]} />

      {/* Central tower */}
      <mesh position={[0, 40, 0]}>
        <cylinderGeometry args={[5, 8, 80, 8]} />
        <meshStandardMaterial
          color="#0a0a1a"
          emissive="#00ffff"
          emissiveIntensity={0.3}
          metalness={1}
          roughness={0}
        />
      </mesh>
    </group>
  )
}

// Classical: Crystal Cathedral
export function ClassicalCathedral() {
  const groupRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  const pillars = useMemo(() => {
    const items: JSX.Element[] = []
    const count = 16
    const radius = 40

    for (let i = 0; i < count; i++) {
      const angle = (i / count) * Math.PI * 2
      const x = Math.cos(angle) * radius
      const z = Math.sin(angle) * radius
      const height = 30 + Math.sin(angle * 3) * 10

      items.push(
        <group key={`pillar-${i}`} position={[x, 0, z]}>
          {/* Base */}
          <mesh position={[0, 1, 0]}>
            <cylinderGeometry args={[2, 2.5, 2, 8]} />
            <meshStandardMaterial color="#f5f5dc" metalness={0.3} roughness={0.5} />
          </mesh>

          {/* Column */}
          <mesh position={[0, height / 2 + 2, 0]}>
            <cylinderGeometry args={[1.5, 1.5, height, 16]} />
            <meshStandardMaterial color="#fffaf0" metalness={0.2} roughness={0.4} />
          </mesh>

          {/* Capital */}
          <mesh position={[0, height + 2.5, 0]}>
            <cylinderGeometry args={[2.5, 1.5, 3, 8]} />
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

    return items
  }, [])

  // Floating crystals
  const crystals = useMemo(() => {
    const items: JSX.Element[] = []

    for (let i = 0; i < 30; i++) {
      const angle = Math.random() * Math.PI * 2
      const radius = 10 + Math.random() * 60
      const height = 15 + Math.random() * 30
      const scale = 0.5 + Math.random() * 2

      items.push(
        <mesh
          key={`crystal-${i}`}
          position={[Math.cos(angle) * radius, height, Math.sin(angle) * radius]}
          rotation={[Math.random(), Math.random(), Math.random()]}
          scale={scale}
        >
          <octahedronGeometry args={[1, 0]} />
          <meshStandardMaterial
            color="#ffd700"
            emissive="#ffd700"
            emissiveIntensity={0.5}
            transparent
            opacity={0.7}
            metalness={0.9}
            roughness={0.1}
          />
        </mesh>
      )
    }

    return items
  }, [])

  useFrame((state) => {
    if (groupRef.current) {
      // Crystals rotate gently
      groupRef.current.children.forEach((child, i) => {
        if (child.type === 'Mesh' && i > 20) {
          child.rotation.y += 0.005
          child.position.y += Math.sin(state.clock.elapsedTime + i) * 0.01
        }
      })
    }
  })

  return (
    <group ref={groupRef}>
      {pillars}
      {crystals}

      {/* Central fountain base */}
      <mesh position={[0, 1, 0]}>
        <cylinderGeometry args={[10, 12, 2, 16]} />
        <meshStandardMaterial color="#b0c4de" metalness={0.5} roughness={0.3} />
      </mesh>

      {/* Marble floor pattern */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.01, 0]}>
        <circleGeometry args={[80, 64]} />
        <meshStandardMaterial color="#f5f5dc" metalness={0.3} roughness={0.5} />
      </mesh>
    </group>
  )
}

// Metal: Volcanic Hellscape
export function MetalVolcano() {
  const groupRef = useRef<THREE.Group>(null)
  const materialRef = useRef<THREE.ShaderMaterial>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  // Lava pools shader
  const lavaMaterial = useMemo(() => {
    return new THREE.ShaderMaterial({
      uniforms: {
        uTime: { value: 0 },
        uBass: { value: 0.3 },
      },
      vertexShader: `
        varying vec2 vUv;
        void main() {
          vUv = uv;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader: `
        uniform float uTime;
        uniform float uBass;
        varying vec2 vUv;

        void main() {
          vec2 uv = vUv;

          // Animated lava flow
          float flow = sin(uv.x * 10.0 + uTime * 2.0) * cos(uv.y * 10.0 + uTime * 1.5);
          flow += sin(uv.x * 5.0 - uTime) * 0.5;

          // Color gradient from orange to yellow
          vec3 lavaColor = mix(
            vec3(1.0, 0.2, 0.0),
            vec3(1.0, 0.8, 0.0),
            flow * 0.5 + 0.5 + uBass * 0.3
          );

          // Add glow
          lavaColor += vec3(0.5, 0.1, 0.0) * (1.0 + uBass);

          gl_FragColor = vec4(lavaColor, 1.0);
        }
      `,
    })
  }, [])

  // Spiky rock formations
  const rocks = useMemo(() => {
    const items: JSX.Element[] = []

    for (let i = 0; i < 50; i++) {
      const angle = Math.random() * Math.PI * 2
      const radius = 20 + Math.random() * 80
      const height = 5 + Math.random() * 25
      const tilt = (Math.random() - 0.5) * 0.5

      items.push(
        <mesh
          key={`rock-${i}`}
          position={[Math.cos(angle) * radius, height / 2, Math.sin(angle) * radius]}
          rotation={[tilt, Math.random() * Math.PI, tilt]}
        >
          <coneGeometry args={[2 + Math.random() * 3, height, 6]} />
          <meshStandardMaterial
            color="#1a0500"
            emissive="#330000"
            emissiveIntensity={0.2}
            roughness={0.9}
            metalness={0.1}
          />
        </mesh>
      )
    }

    return items
  }, [])

  useFrame((state) => {
    if (lavaMaterial) {
      lavaMaterial.uniforms.uTime.value = state.clock.elapsedTime
      lavaMaterial.uniforms.uBass.value = audioFeatures.bass
    }
  })

  return (
    <group ref={groupRef}>
      {rocks}

      {/* Multiple lava pools */}
      {[
        { pos: [0, 0.2, 0], radius: 15 },
        { pos: [-30, 0.2, 20], radius: 8 },
        { pos: [40, 0.2, -10], radius: 10 },
        { pos: [-20, 0.2, -40], radius: 12 },
      ].map((pool, i) => (
        <mesh key={`lava-${i}`} position={pool.pos as [number, number, number]} rotation={[-Math.PI / 2, 0, 0]}>
          <circleGeometry args={[pool.radius, 32]} />
          <primitive object={lavaMaterial} attach="material" />
        </mesh>
      ))}

      {/* Central volcano */}
      <mesh position={[0, 15, 0]}>
        <coneGeometry args={[20, 30, 8, 1, true]} />
        <meshStandardMaterial
          color="#1a0500"
          emissive="#330000"
          emissiveIntensity={0.3}
          side={THREE.DoubleSide}
        />
      </mesh>
    </group>
  )
}

// Ambient: Ethereal Dreamscape
export function AmbientDreamscape() {
  const groupRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  // Floating islands
  const islands = useMemo(() => {
    const items: JSX.Element[] = []

    for (let i = 0; i < 15; i++) {
      const x = (Math.random() - 0.5) * 150
      const y = 10 + Math.random() * 40
      const z = (Math.random() - 0.5) * 150
      const scale = 1 + Math.random() * 3

      items.push(
        <group key={`island-${i}`} position={[x, y, z]} scale={scale}>
          {/* Island body */}
          <mesh>
            <sphereGeometry args={[3, 16, 16, 0, Math.PI * 2, 0, Math.PI / 2]} />
            <meshStandardMaterial
              color="#9370db"
              emissive="#4b0082"
              emissiveIntensity={0.3}
            />
          </mesh>

          {/* Glowing tree */}
          <mesh position={[0, 2, 0]}>
            <coneGeometry args={[1, 4, 8]} />
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

    return items
  }, [])

  // Giant glowing orbs
  const orbs = useMemo(() => {
    const items: JSX.Element[] = []

    for (let i = 0; i < 8; i++) {
      const x = (Math.random() - 0.5) * 100
      const y = 20 + Math.random() * 30
      const z = (Math.random() - 0.5) * 100
      const scale = 3 + Math.random() * 5

      items.push(
        <mesh key={`orb-${i}`} position={[x, y, z]}>
          <sphereGeometry args={[scale, 32, 32]} />
          <meshStandardMaterial
            color="#dda0dd"
            emissive="#dda0dd"
            emissiveIntensity={1}
            transparent
            opacity={0.4}
          />
        </mesh>
      )
    }

    return items
  }, [])

  useFrame((state) => {
    if (groupRef.current) {
      // Gentle floating animation
      groupRef.current.children.forEach((child, i) => {
        if (child.position) {
          child.position.y += Math.sin(state.clock.elapsedTime * 0.5 + i) * 0.02
        }
      })
    }
  })

  return (
    <group ref={groupRef}>
      {islands}
      {orbs}

      {/* Mist layers */}
      {[10, 20, 30].map((height, i) => (
        <mesh key={`mist-${i}`} position={[0, height, 0]} rotation={[-Math.PI / 2, 0, 0]}>
          <planeGeometry args={[200, 200]} />
          <meshStandardMaterial
            color="#9370db"
            transparent
            opacity={0.1}
            side={THREE.DoubleSide}
          />
        </mesh>
      ))}
    </group>
  )
}

// Jazz: Smoky Club
export function JazzClub() {
  const groupRef = useRef<THREE.Group>(null)
  const audioFeatures = useStore((s) => s.audioFeatures)

  // Tables and chairs
  const furniture = useMemo(() => {
    const items: JSX.Element[] = []

    for (let i = 0; i < 20; i++) {
      const x = (Math.random() - 0.5) * 60
      const z = (Math.random() - 0.5) * 60

      // Skip if too close to center (stage area)
      if (Math.abs(x) < 15 && Math.abs(z) < 15) continue

      items.push(
        <group key={`table-${i}`} position={[x, 0, z]}>
          {/* Table */}
          <mesh position={[0, 2.5, 0]}>
            <cylinderGeometry args={[2, 2, 0.2, 16]} />
            <meshStandardMaterial color="#8b4513" metalness={0.3} roughness={0.7} />
          </mesh>
          <mesh position={[0, 1.25, 0]}>
            <cylinderGeometry args={[0.3, 0.3, 2.5, 8]} />
            <meshStandardMaterial color="#654321" />
          </mesh>

          {/* Candle */}
          <mesh position={[0, 2.8, 0]}>
            <sphereGeometry args={[0.15, 8, 8]} />
            <meshStandardMaterial
              color="#ffa500"
              emissive="#ff6600"
              emissiveIntensity={2}
            />
          </mesh>
        </group>
      )
    }

    return items
  }, [])

  // Stage with instruments (silhouettes)
  const stage = useMemo(() => (
    <group position={[0, 0, -20]}>
      {/* Stage platform */}
      <mesh position={[0, 0.5, 0]}>
        <boxGeometry args={[20, 1, 10]} />
        <meshStandardMaterial color="#2a1810" metalness={0.2} roughness={0.8} />
      </mesh>

      {/* Piano silhouette */}
      <mesh position={[-5, 2.5, 0]}>
        <boxGeometry args={[4, 3, 2]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>

      {/* Drum kit silhouette */}
      <mesh position={[5, 2, 0]}>
        <cylinderGeometry args={[1.5, 1.5, 2, 16]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>

      {/* Spotlights */}
      <pointLight position={[-5, 8, 2]} color="#ff8c00" intensity={50} distance={20} />
      <pointLight position={[0, 8, 2]} color="#ffa500" intensity={50} distance={20} />
      <pointLight position={[5, 8, 2]} color="#ff8c00" intensity={50} distance={20} />
    </group>
  ), [])

  // Hanging lamps
  const lamps = useMemo(() => {
    const items: JSX.Element[] = []

    for (let i = 0; i < 12; i++) {
      const x = (Math.random() - 0.5) * 80
      const z = (Math.random() - 0.5) * 80

      items.push(
        <group key={`lamp-${i}`} position={[x, 8, z]}>
          <mesh>
            <sphereGeometry args={[0.8, 16, 16]} />
            <meshStandardMaterial
              color="#ff8c00"
              emissive="#ff6600"
              emissiveIntensity={1}
              transparent
              opacity={0.8}
            />
          </mesh>
          <pointLight color="#ff8c00" intensity={20} distance={15} />
        </group>
      )
    }

    return items
  }, [])

  useFrame((state) => {
    // Lamps flicker slightly
    if (groupRef.current) {
      groupRef.current.children.forEach((child, i) => {
        const light = child.children?.find(c => c.type === 'PointLight') as THREE.PointLight
        if (light) {
          light.intensity = 20 + Math.sin(state.clock.elapsedTime * 3 + i) * 5
        }
      })
    }
  })

  return (
    <group ref={groupRef}>
      {furniture}
      {stage}
      {lamps}

      {/* Wooden floor */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0, 0]}>
        <planeGeometry args={[100, 100]} />
        <meshStandardMaterial color="#3d2817" roughness={0.8} />
      </mesh>

      {/* Bar at the back */}
      <mesh position={[0, 2, 40]}>
        <boxGeometry args={[30, 4, 2]} />
        <meshStandardMaterial color="#4a3728" metalness={0.3} roughness={0.6} />
      </mesh>
    </group>
  )
}

// Main structure selector
export function GenreStructures() {
  const genre = useStore((s) => s.genre)

  switch (genre) {
    case 'electronic':
      return <ElectronicCity />
    case 'classical':
      return <ClassicalCathedral />
    case 'metal':
      return <MetalVolcano />
    case 'ambient':
      return <AmbientDreamscape />
    case 'jazz':
      return <JazzClub />
    default:
      return <ElectronicCity />
  }
}
