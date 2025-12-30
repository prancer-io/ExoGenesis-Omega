/**
 * Procedural Terrain System
 * Creates stunning, audio-reactive terrain with Perlin noise
 */

import { useRef, useMemo } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import { createNoise2D, createNoise3D } from 'simplex-noise'
import { useStore, type Genre } from '../store'

const noise2D = createNoise2D()
const noise3D = createNoise3D()

interface TerrainProps {
  size?: number
  resolution?: number
}

// Genre-specific terrain configurations
const terrainConfigs: Record<Genre, {
  baseColor: string
  peakColor: string
  emissiveColor: string
  roughness: number
  metalness: number
  amplitude: number
  frequency: number
  octaves: number
  animated: boolean
}> = {
  electronic: {
    baseColor: '#0a0a1a',
    peakColor: '#00ffff',
    emissiveColor: '#001133',
    roughness: 0.1,
    metalness: 0.9,
    amplitude: 8,
    frequency: 0.02,
    octaves: 4,
    animated: true,
  },
  classical: {
    baseColor: '#1a1510',
    peakColor: '#ffd700',
    emissiveColor: '#331100',
    roughness: 0.6,
    metalness: 0.3,
    amplitude: 15,
    frequency: 0.015,
    octaves: 6,
    animated: false,
  },
  metal: {
    baseColor: '#1a0500',
    peakColor: '#ff4500',
    emissiveColor: '#330000',
    roughness: 0.8,
    metalness: 0.2,
    amplitude: 25,
    frequency: 0.03,
    octaves: 5,
    animated: true,
  },
  ambient: {
    baseColor: '#0a0a15',
    peakColor: '#9370db',
    emissiveColor: '#1a0033',
    roughness: 0.9,
    metalness: 0.1,
    amplitude: 12,
    frequency: 0.01,
    octaves: 8,
    animated: true,
  },
  jazz: {
    baseColor: '#1a1000',
    peakColor: '#ff8c00',
    emissiveColor: '#331a00',
    roughness: 0.5,
    metalness: 0.4,
    amplitude: 6,
    frequency: 0.025,
    octaves: 3,
    animated: false,
  },
}

export function Terrain({ size = 200, resolution = 128 }: TerrainProps) {
  const meshRef = useRef<THREE.Mesh>(null)
  const materialRef = useRef<THREE.ShaderMaterial>(null)
  const genre = useStore((s) => s.genre)
  const audioFeatures = useStore((s) => s.audioFeatures)

  const config = terrainConfigs[genre]

  // Create geometry with high resolution
  const geometry = useMemo(() => {
    const geo = new THREE.PlaneGeometry(size, size, resolution, resolution)
    geo.rotateX(-Math.PI / 2)

    // Store original positions
    const positions = geo.attributes.position.array as Float32Array
    geo.userData.originalPositions = positions.slice()

    return geo
  }, [size, resolution])

  // Custom shader for stunning terrain
  const shaderMaterial = useMemo(() => {
    return new THREE.ShaderMaterial({
      uniforms: {
        uTime: { value: 0 },
        uBass: { value: 0.3 },
        uMid: { value: 0.3 },
        uHigh: { value: 0.3 },
        uBeatIntensity: { value: 0 },
        uBaseColor: { value: new THREE.Color(config.baseColor) },
        uPeakColor: { value: new THREE.Color(config.peakColor) },
        uEmissiveColor: { value: new THREE.Color(config.emissiveColor) },
        uAmplitude: { value: config.amplitude },
      },
      vertexShader: `
        uniform float uTime;
        uniform float uBass;
        uniform float uAmplitude;

        varying vec3 vPosition;
        varying vec3 vNormal;
        varying float vElevation;

        // Simplex noise functions
        vec3 mod289(vec3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
        vec4 mod289(vec4 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
        vec4 permute(vec4 x) { return mod289(((x*34.0)+1.0)*x); }
        vec4 taylorInvSqrt(vec4 r) { return 1.79284291400159 - 0.85373472095314 * r; }

        float snoise(vec3 v) {
          const vec2 C = vec2(1.0/6.0, 1.0/3.0);
          const vec4 D = vec4(0.0, 0.5, 1.0, 2.0);

          vec3 i  = floor(v + dot(v, C.yyy));
          vec3 x0 = v - i + dot(i, C.xxx);

          vec3 g = step(x0.yzx, x0.xyz);
          vec3 l = 1.0 - g;
          vec3 i1 = min(g.xyz, l.zxy);
          vec3 i2 = max(g.xyz, l.zxy);

          vec3 x1 = x0 - i1 + C.xxx;
          vec3 x2 = x0 - i2 + C.yyy;
          vec3 x3 = x0 - D.yyy;

          i = mod289(i);
          vec4 p = permute(permute(permute(
                    i.z + vec4(0.0, i1.z, i2.z, 1.0))
                  + i.y + vec4(0.0, i1.y, i2.y, 1.0))
                  + i.x + vec4(0.0, i1.x, i2.x, 1.0));

          float n_ = 0.142857142857;
          vec3 ns = n_ * D.wyz - D.xzx;

          vec4 j = p - 49.0 * floor(p * ns.z * ns.z);

          vec4 x_ = floor(j * ns.z);
          vec4 y_ = floor(j - 7.0 * x_);

          vec4 x = x_ *ns.x + ns.yyyy;
          vec4 y = y_ *ns.x + ns.yyyy;
          vec4 h = 1.0 - abs(x) - abs(y);

          vec4 b0 = vec4(x.xy, y.xy);
          vec4 b1 = vec4(x.zw, y.zw);

          vec4 s0 = floor(b0)*2.0 + 1.0;
          vec4 s1 = floor(b1)*2.0 + 1.0;
          vec4 sh = -step(h, vec4(0.0));

          vec4 a0 = b0.xzyw + s0.xzyw*sh.xxyy;
          vec4 a1 = b1.xzyw + s1.xzyw*sh.zzww;

          vec3 p0 = vec3(a0.xy, h.x);
          vec3 p1 = vec3(a0.zw, h.y);
          vec3 p2 = vec3(a1.xy, h.z);
          vec3 p3 = vec3(a1.zw, h.w);

          vec4 norm = taylorInvSqrt(vec4(dot(p0,p0), dot(p1,p1), dot(p2,p2), dot(p3,p3)));
          p0 *= norm.x;
          p1 *= norm.y;
          p2 *= norm.z;
          p3 *= norm.w;

          vec4 m = max(0.6 - vec4(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3)), 0.0);
          m = m * m;
          return 42.0 * dot(m*m, vec4(dot(p0,x0), dot(p1,x1), dot(p2,x2), dot(p3,x3)));
        }

        float fbm(vec3 p) {
          float value = 0.0;
          float amplitude = 0.5;
          float frequency = 1.0;

          for (int i = 0; i < 6; i++) {
            value += amplitude * snoise(p * frequency);
            amplitude *= 0.5;
            frequency *= 2.0;
          }

          return value;
        }

        void main() {
          vec3 pos = position;

          // Multi-octave noise for terrain
          float freq = 0.02;
          float noiseVal = fbm(vec3(pos.x * freq, pos.z * freq, uTime * 0.1));

          // Audio-reactive displacement
          float audioInfluence = uBass * 0.5 + 0.5;
          float elevation = noiseVal * uAmplitude * audioInfluence;

          // Add ripples from bass
          float ripple = sin(length(pos.xz) * 0.1 - uTime * 2.0) * uBass * 3.0;
          elevation += ripple;

          pos.y += elevation;

          vPosition = pos;
          vNormal = normal;
          vElevation = elevation / uAmplitude;

          gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
        }
      `,
      fragmentShader: `
        uniform float uTime;
        uniform float uBass;
        uniform float uMid;
        uniform float uHigh;
        uniform float uBeatIntensity;
        uniform vec3 uBaseColor;
        uniform vec3 uPeakColor;
        uniform vec3 uEmissiveColor;

        varying vec3 vPosition;
        varying vec3 vNormal;
        varying float vElevation;

        void main() {
          // Height-based color gradient
          float heightMix = smoothstep(-0.5, 1.0, vElevation);
          vec3 color = mix(uBaseColor, uPeakColor, heightMix);

          // Grid lines (electronic style)
          float gridX = abs(fract(vPosition.x * 0.1) - 0.5) * 2.0;
          float gridZ = abs(fract(vPosition.z * 0.1) - 0.5) * 2.0;
          float grid = max(
            smoothstep(0.95, 1.0, gridX),
            smoothstep(0.95, 1.0, gridZ)
          );

          // Emissive glow on peaks and grid
          vec3 emissive = uEmissiveColor * (heightMix * 0.5 + grid * 0.5);
          emissive += uPeakColor * uBeatIntensity * 0.5;

          // Rim lighting effect
          vec3 viewDir = normalize(cameraPosition - vPosition);
          float rim = 1.0 - max(0.0, dot(viewDir, vNormal));
          rim = pow(rim, 3.0);
          emissive += uPeakColor * rim * 0.3;

          // Final color
          color += emissive;
          color += grid * uPeakColor * 0.3;

          // Beat pulse
          color += uPeakColor * uBeatIntensity * 0.2;

          gl_FragColor = vec4(color, 1.0);
        }
      `,
      side: THREE.DoubleSide,
    })
  }, [config])

  // Animate terrain
  useFrame((state, delta) => {
    if (materialRef.current) {
      materialRef.current.uniforms.uTime.value += delta
      materialRef.current.uniforms.uBass.value = audioFeatures.bass
      materialRef.current.uniforms.uMid.value = audioFeatures.mid
      materialRef.current.uniforms.uHigh.value = audioFeatures.high
      materialRef.current.uniforms.uBeatIntensity.value = audioFeatures.beatIntensity

      // Update colors when genre changes
      materialRef.current.uniforms.uBaseColor.value.set(config.baseColor)
      materialRef.current.uniforms.uPeakColor.value.set(config.peakColor)
      materialRef.current.uniforms.uEmissiveColor.value.set(config.emissiveColor)
      materialRef.current.uniforms.uAmplitude.value = config.amplitude
    }
  })

  return (
    <mesh ref={meshRef} geometry={geometry}>
      <primitive object={shaderMaterial} ref={materialRef} attach="material" />
    </mesh>
  )
}
