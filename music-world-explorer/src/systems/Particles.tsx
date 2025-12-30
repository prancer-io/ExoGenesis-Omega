/**
 * Stunning Particle Systems
 * Genre-specific particles that react to audio
 */

import { useRef, useMemo } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import { useStore, type Genre } from '../store'

interface ParticleSystemProps {
  count?: number
}

// Genre-specific particle configurations
const particleConfigs: Record<Genre, {
  colors: string[]
  size: number
  speed: number
  spread: number
  height: number
  glow: number
  behavior: 'rise' | 'float' | 'swarm' | 'rain' | 'spiral'
}> = {
  electronic: {
    colors: ['#00ffff', '#ff00ff', '#ffff00', '#00ff00'],
    size: 0.15,
    speed: 8,
    spread: 80,
    height: 50,
    glow: 1.5,
    behavior: 'swarm',
  },
  classical: {
    colors: ['#ffd700', '#ffffff', '#fffacd'],
    size: 0.08,
    speed: 1,
    spread: 100,
    height: 40,
    glow: 0.8,
    behavior: 'float',
  },
  metal: {
    colors: ['#ff4500', '#ff6600', '#ffff00', '#ff0000'],
    size: 0.12,
    speed: 6,
    spread: 60,
    height: 60,
    glow: 2.0,
    behavior: 'rise',
  },
  ambient: {
    colors: ['#9370db', '#48d1cc', '#98fb98', '#dda0dd'],
    size: 0.2,
    speed: 0.5,
    spread: 120,
    height: 30,
    glow: 1.0,
    behavior: 'float',
  },
  jazz: {
    colors: ['#ff8c00', '#ffa500', '#daa520', '#b8860b'],
    size: 0.25,
    speed: 0.8,
    spread: 50,
    height: 20,
    glow: 0.6,
    behavior: 'float',
  },
}

export function ParticleSystem({ count = 3000 }: ParticleSystemProps) {
  const pointsRef = useRef<THREE.Points>(null)
  const genre = useStore((s) => s.genre)
  const audioFeatures = useStore((s) => s.audioFeatures)

  const config = particleConfigs[genre]

  // Create particle geometry and material
  const { geometry, material } = useMemo(() => {
    const geo = new THREE.BufferGeometry()

    // Positions
    const positions = new Float32Array(count * 3)
    const colors = new Float32Array(count * 3)
    const sizes = new Float32Array(count)
    const velocities = new Float32Array(count * 3)
    const phases = new Float32Array(count)

    for (let i = 0; i < count; i++) {
      // Random position in a sphere
      const theta = Math.random() * Math.PI * 2
      const phi = Math.acos(2 * Math.random() - 1)
      const r = Math.random() * config.spread

      positions[i * 3] = r * Math.sin(phi) * Math.cos(theta)
      positions[i * 3 + 1] = Math.random() * config.height
      positions[i * 3 + 2] = r * Math.sin(phi) * Math.sin(theta)

      // Random color from palette
      const colorHex = config.colors[Math.floor(Math.random() * config.colors.length)]
      const color = new THREE.Color(colorHex)
      colors[i * 3] = color.r
      colors[i * 3 + 1] = color.g
      colors[i * 3 + 2] = color.b

      // Random sizes
      sizes[i] = config.size * (0.5 + Math.random())

      // Velocities
      velocities[i * 3] = (Math.random() - 0.5) * 2
      velocities[i * 3 + 1] = Math.random() * 2
      velocities[i * 3 + 2] = (Math.random() - 0.5) * 2

      // Phase for animation
      phases[i] = Math.random() * Math.PI * 2
    }

    geo.setAttribute('position', new THREE.BufferAttribute(positions, 3))
    geo.setAttribute('color', new THREE.BufferAttribute(colors, 3))
    geo.setAttribute('size', new THREE.BufferAttribute(sizes, 1))
    geo.userData.velocities = velocities
    geo.userData.phases = phases

    // Custom shader material for glowing particles
    const mat = new THREE.ShaderMaterial({
      uniforms: {
        uTime: { value: 0 },
        uBeatIntensity: { value: 0 },
        uBass: { value: 0.3 },
        uGlow: { value: config.glow },
      },
      vertexShader: `
        attribute float size;
        attribute vec3 color;

        uniform float uTime;
        uniform float uBeatIntensity;
        uniform float uBass;

        varying vec3 vColor;
        varying float vAlpha;

        void main() {
          vColor = color;

          // Pulse size with beat
          float pulseSize = size * (1.0 + uBeatIntensity * 0.5 + uBass * 0.3);

          vec4 mvPosition = modelViewMatrix * vec4(position, 1.0);

          // Size attenuation
          gl_PointSize = pulseSize * (300.0 / -mvPosition.z);
          gl_PointSize = clamp(gl_PointSize, 1.0, 50.0);

          gl_Position = projectionMatrix * mvPosition;

          // Distance-based alpha
          vAlpha = smoothstep(150.0, 0.0, length(position.xz));
        }
      `,
      fragmentShader: `
        uniform float uGlow;
        uniform float uBeatIntensity;

        varying vec3 vColor;
        varying float vAlpha;

        void main() {
          // Circular particle with glow
          vec2 center = gl_PointCoord - 0.5;
          float dist = length(center);

          if (dist > 0.5) discard;

          // Soft glow falloff
          float glow = 1.0 - smoothstep(0.0, 0.5, dist);
          glow = pow(glow, 1.5);

          // Beat pulse
          float beatGlow = uBeatIntensity * 0.5;

          vec3 finalColor = vColor * (uGlow + beatGlow);

          gl_FragColor = vec4(finalColor, glow * vAlpha);
        }
      `,
      transparent: true,
      blending: THREE.AdditiveBlending,
      depthWrite: false,
    })

    return { geometry: geo, material: mat }
  }, [count, config])

  // Animate particles
  useFrame((state, delta) => {
    if (!pointsRef.current) return

    const positions = geometry.attributes.position.array as Float32Array
    const velocities = geometry.userData.velocities as Float32Array
    const phases = geometry.userData.phases as Float32Array
    const time = state.clock.elapsedTime

    material.uniforms.uTime.value = time
    material.uniforms.uBeatIntensity.value = audioFeatures.beatIntensity
    material.uniforms.uBass.value = audioFeatures.bass

    const speed = config.speed * (1 + audioFeatures.rms)

    for (let i = 0; i < count; i++) {
      const i3 = i * 3
      const phase = phases[i]

      switch (config.behavior) {
        case 'rise':
          // Rise and reset (embers)
          positions[i3 + 1] += delta * speed * (0.5 + audioFeatures.bass)
          positions[i3] += Math.sin(time + phase) * delta * 2
          positions[i3 + 2] += Math.cos(time + phase) * delta * 2

          if (positions[i3 + 1] > config.height) {
            positions[i3 + 1] = 0
            positions[i3] = (Math.random() - 0.5) * config.spread
            positions[i3 + 2] = (Math.random() - 0.5) * config.spread
          }
          break

        case 'float':
          // Gentle floating (dust, wisps)
          positions[i3] += Math.sin(time * 0.5 + phase) * delta * speed * 0.5
          positions[i3 + 1] += Math.sin(time * 0.3 + phase * 2) * delta * speed * 0.3
          positions[i3 + 2] += Math.cos(time * 0.5 + phase) * delta * speed * 0.5
          break

        case 'swarm':
          // Fast swarming (electronic trails)
          const angle = time * 2 + phase
          const radius = 10 + Math.sin(time + phase) * 5
          positions[i3] += Math.cos(angle) * delta * speed
          positions[i3 + 1] += Math.sin(time * 3 + phase) * delta * speed * 0.5
          positions[i3 + 2] += Math.sin(angle) * delta * speed

          // Reset if too far
          if (Math.abs(positions[i3]) > config.spread) {
            positions[i3] = (Math.random() - 0.5) * 20
          }
          if (Math.abs(positions[i3 + 2]) > config.spread) {
            positions[i3 + 2] = (Math.random() - 0.5) * 20
          }
          break

        case 'spiral':
          // Spiral upward
          const spiralAngle = time + phase + positions[i3 + 1] * 0.1
          positions[i3] = Math.cos(spiralAngle) * (10 + positions[i3 + 1] * 0.2)
          positions[i3 + 1] += delta * speed
          positions[i3 + 2] = Math.sin(spiralAngle) * (10 + positions[i3 + 1] * 0.2)

          if (positions[i3 + 1] > config.height) {
            positions[i3 + 1] = 0
          }
          break

        case 'rain':
          // Fall downward
          positions[i3 + 1] -= delta * speed
          if (positions[i3 + 1] < 0) {
            positions[i3 + 1] = config.height
            positions[i3] = (Math.random() - 0.5) * config.spread
            positions[i3 + 2] = (Math.random() - 0.5) * config.spread
          }
          break
      }
    }

    geometry.attributes.position.needsUpdate = true
  })

  return (
    <points ref={pointsRef} geometry={geometry} material={material} />
  )
}

// Additional atmospheric particles (slower, ambient)
export function AtmosphericParticles({ count = 500 }: ParticleSystemProps) {
  const pointsRef = useRef<THREE.Points>(null)
  const genre = useStore((s) => s.genre)
  const audioFeatures = useStore((s) => s.audioFeatures)

  const config = particleConfigs[genre]

  const { geometry, material } = useMemo(() => {
    const geo = new THREE.BufferGeometry()
    const positions = new Float32Array(count * 3)
    const sizes = new Float32Array(count)

    for (let i = 0; i < count; i++) {
      positions[i * 3] = (Math.random() - 0.5) * 200
      positions[i * 3 + 1] = Math.random() * 80
      positions[i * 3 + 2] = (Math.random() - 0.5) * 200

      sizes[i] = 0.5 + Math.random() * 1.5
    }

    geo.setAttribute('position', new THREE.BufferAttribute(positions, 3))
    geo.setAttribute('size', new THREE.BufferAttribute(sizes, 1))

    const mat = new THREE.PointsMaterial({
      size: 0.5,
      color: new THREE.Color(config.colors[0]),
      transparent: true,
      opacity: 0.3,
      blending: THREE.AdditiveBlending,
      depthWrite: false,
    })

    return { geometry: geo, material: mat }
  }, [count, config])

  useFrame((state) => {
    if (pointsRef.current) {
      pointsRef.current.rotation.y = state.clock.elapsedTime * 0.01
      material.opacity = 0.2 + audioFeatures.rms * 0.3
    }
  })

  return <points ref={pointsRef} geometry={geometry} material={material} />
}
