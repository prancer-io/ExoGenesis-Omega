/**
 * ENHANCED Procedural 3D World Generators - WOW FACTOR!
 * Massively improved with complex geometry, particles, and atmosphere
 */

import * as THREE from 'three'
import type { Genre } from '../App'

export interface WorldChunk {
  meshes: THREE.Mesh[]
  particles: THREE.Points[]
  lights: THREE.PointLight[]
  position: THREE.Vector3
  genre: Genre
}

// Classical: GRAND Marble Cathedral with Particle Dust
export class ClassicalWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []
    const particles: THREE.Points[] = []
    const lights: THREE.PointLight[] = []

    // MASSIVE marble floor (20x20 instead of 5x5)
    const floorTile = new THREE.Mesh(
      new THREE.BoxGeometry(20, 0.5, 20),
      new THREE.MeshStandardMaterial({
        color: 0xf5f5dc,
        metalness: 0.4,
        roughness: 0.3,
      })
    )
    floorTile.position.copy(position)
    floorTile.receiveShadow = true
    meshes.push(floorTile)

    // Checkered marble pattern (secondary tiles)
    for (let i = -2; i <= 2; i++) {
      for (let j = -2; j <= 2; j++) {
        if ((i + j) % 2 === 0) {
          const tile = new THREE.Mesh(
            new THREE.BoxGeometry(3.5, 0.3, 3.5),
            new THREE.MeshStandardMaterial({
              color: 0xffffff,
              metalness: 0.5,
              roughness: 0.2,
            })
          )
          tile.position.set(position.x + i * 4, 0.3, position.z + j * 4)
          meshes.push(tile)
        }
      }
    }

    // TOWERING pillars - 3-6 pillars per chunk (optimized)
    const pillarCount = Math.floor(audioFeatures.beatConfidence * 3) + 3
    for (let i = 0; i < pillarCount; i++) {
      const pillarHeight = 20 + audioFeatures.rms * 30 // 20-50 units tall!
      const angle = (i / pillarCount) * Math.PI * 2
      const radius = 8

      // Main pillar shaft (reduced segments)
      const pillar = new THREE.Mesh(
        new THREE.CylinderGeometry(0.8, 1, pillarHeight, 8),
        new THREE.MeshStandardMaterial({
          color: 0xffffff,
          metalness: 0.3,
          roughness: 0.3,
        })
      )
      pillar.position.set(
        position.x + Math.cos(angle) * radius,
        pillarHeight / 2,
        position.z + Math.sin(angle) * radius
      )
      pillar.castShadow = true
      meshes.push(pillar)

      // Ornate capital (top)
      const capital = new THREE.Mesh(
        new THREE.CylinderGeometry(1.3, 0.8, 1.5, 8),
        new THREE.MeshStandardMaterial({
          color: 0xffd700,
          metalness: 0.8,
          roughness: 0.2,
          emissive: new THREE.Color(0xffd700),
          emissiveIntensity: 0.3,
        })
      )
      capital.position.set(
        position.x + Math.cos(angle) * radius,
        pillarHeight + 0.75,
        position.z + Math.sin(angle) * radius
      )
      meshes.push(capital)

      // Base
      const base = new THREE.Mesh(
        new THREE.CylinderGeometry(1, 1.2, 1, 8),
        new THREE.MeshStandardMaterial({ color: 0xffffff })
      )
      base.position.set(
        position.x + Math.cos(angle) * radius,
        0.5,
        position.z + Math.sin(angle) * radius
      )
      meshes.push(base)
    }

    // GRAND fountain at center (when high frequency)
    if (audioFeatures.spectralCentroid > 2500) {
      // Large fountain base
      const fountainBase = new THREE.Mesh(
        new THREE.CylinderGeometry(3, 4, 1, 8),
        new THREE.MeshStandardMaterial({
          color: 0xb0c4de,
          metalness: 0.6,
          roughness: 0.2,
        })
      )
      fountainBase.position.set(position.x, 0.5, position.z)
      meshes.push(fountainBase)

      // Multi-tier fountain
      const tiers = 3
      for (let t = 0; t < tiers; t++) {
        const tierRadius = 2.5 - t * 0.7
        const tierHeight = 1 + t * 1.5
        const tier = new THREE.Mesh(
          new THREE.CylinderGeometry(tierRadius, tierRadius + 0.3, 0.3, 8),
          new THREE.MeshStandardMaterial({
            color: 0x87ceeb,
            metalness: 0.5,
            roughness: 0.3,
          })
        )
        tier.position.set(position.x, tierHeight, position.z)
        meshes.push(tier)
      }

      // Water particles (50 particles shooting up - optimized)
      const waterParticleCount = 50
      const waterGeometry = new THREE.BufferGeometry()
      const waterPositions = new Float32Array(waterParticleCount * 3)
      const waterColors = new Float32Array(waterParticleCount * 3)

      for (let i = 0; i < waterParticleCount; i++) {
        const angle = Math.random() * Math.PI * 2
        const radius = Math.random() * 2
        const height = Math.random() * (5 + audioFeatures.rms * 10)

        waterPositions[i * 3] = position.x + Math.cos(angle) * radius
        waterPositions[i * 3 + 1] = height
        waterPositions[i * 3 + 2] = position.z + Math.sin(angle) * radius

        waterColors[i * 3] = 0.5
        waterColors[i * 3 + 1] = 0.8
        waterColors[i * 3 + 2] = 1.0
      }

      waterGeometry.setAttribute('position', new THREE.BufferAttribute(waterPositions, 3))
      waterGeometry.setAttribute('color', new THREE.BufferAttribute(waterColors, 3))

      const waterParticles = new THREE.Points(
        waterGeometry,
        new THREE.PointsMaterial({
          size: 0.4, // Larger to compensate
          vertexColors: true,
          transparent: true,
          opacity: 0.8,
          blending: THREE.AdditiveBlending,
        })
      )
      particles.push(waterParticles)

      // Single bright light for fountain
      const fountainLight = new THREE.PointLight(0x87ceeb, 3, 15)
      fountainLight.position.set(position.x, 4, position.z)
      lights.push(fountainLight)
    }

    // Ambient dust particles (150 floating dust motes - optimized)
    const dustCount = 150
    const dustGeometry = new THREE.BufferGeometry()
    const dustPositions = new Float32Array(dustCount * 3)
    const dustColors = new Float32Array(dustCount * 3)

    for (let i = 0; i < dustCount; i++) {
      dustPositions[i * 3] = position.x + (Math.random() - 0.5) * 25
      dustPositions[i * 3 + 1] = Math.random() * 20
      dustPositions[i * 3 + 2] = position.z + (Math.random() - 0.5) * 25

      dustColors[i * 3] = 1.0
      dustColors[i * 3 + 1] = 0.95
      dustColors[i * 3 + 2] = 0.8
    }

    dustGeometry.setAttribute('position', new THREE.BufferAttribute(dustPositions, 3))
    dustGeometry.setAttribute('color', new THREE.BufferAttribute(dustColors, 3))

    const dust = new THREE.Points(
      dustGeometry,
      new THREE.PointsMaterial({
        size: 0.2, // Slightly larger to compensate
        vertexColors: true,
        transparent: true,
        opacity: 0.5,
      })
    )
    particles.push(dust)

    // Warm golden light from above (single light for whole chunk)
    const light = new THREE.PointLight(0xffd700, 3 + audioFeatures.rms * 4, 60)
    light.position.set(position.x, 15, position.z)
    lights.push(light)

    return { meshes, particles, lights, position, genre: 'classical' }
  }
}

// Rock: EXTREME Volcanic Hellscape with Lava Particles
export class RockWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []
    const particles: THREE.Points[] = []
    const lights: THREE.PointLight[] = []

    // MASSIVE jagged terrain (30x30)
    const groundGeometry = new THREE.PlaneGeometry(30, 30, 40, 40)
    const vertices = groundGeometry.attributes.position.array as Float32Array

    for (let i = 0; i < vertices.length; i += 3) {
      const x = vertices[i]
      const y = vertices[i + 1]
      // Extreme roughness with Perlin-like noise
      vertices[i + 2] =
        Math.sin(x * 0.3) * 3 +
        Math.cos(y * 0.3) * 3 +
        Math.random() * 4 +
        audioFeatures.zcr * 8
    }
    groundGeometry.computeVertexNormals()

    const ground = new THREE.Mesh(
      groundGeometry,
      new THREE.MeshStandardMaterial({
        color: 0x8b0000,
        metalness: 0.2,
        roughness: 1.0,
        emissive: new THREE.Color(0x330000),
        emissiveIntensity: 1 + audioFeatures.rms * 3,
      })
    )
    ground.rotation.x = -Math.PI / 2
    ground.position.copy(position)
    ground.receiveShadow = true
    meshes.push(ground)

    // MASSIVE crystal formations (5-10 crystals - optimized)
    const crystalCount = Math.floor(audioFeatures.beatConfidence * 5) + 5
    for (let i = 0; i < crystalCount; i++) {
      const crystalHeight = 10 + audioFeatures.rms * 25 // 10-35 units!
      const x = position.x + (Math.random() - 0.5) * 20
      const z = position.z + (Math.random() - 0.5) * 20

      // Main crystal spike
      const crystal = new THREE.Mesh(
        new THREE.ConeGeometry(1 + Math.random(), crystalHeight, 6),
        new THREE.MeshStandardMaterial({
          color: 0xff4500,
          metalness: 0.9,
          roughness: 0.1,
          emissive: new THREE.Color(0xff0000),
          emissiveIntensity: 2 + audioFeatures.beatConfidence * 4,
          transparent: true,
          opacity: 0.9,
        })
      )
      crystal.position.set(x, crystalHeight / 2, z)
      crystal.rotation.y = Math.random() * Math.PI
      crystal.rotation.z = (Math.random() - 0.5) * 0.3
      crystal.castShadow = true
      meshes.push(crystal)

      // Only add light for tallest crystals (performance)
      if (i % 2 === 0) {
        const crystalLight = new THREE.PointLight(
          0xff4500,
          4 + audioFeatures.beatConfidence * 6,
          20
        )
        crystalLight.position.set(x, crystalHeight / 2, z)
        lights.push(crystalLight)
      }
    }

    // Multiple lava pools with bubbling effect (reduced)
    const poolCount = Math.floor(audioFeatures.spectralFlux * 4) + 2
    for (let p = 0; p < poolCount; p++) {
      const poolX = position.x + (Math.random() - 0.5) * 25
      const poolZ = position.z + (Math.random() - 0.5) * 25
      const poolRadius = 2 + Math.random() * 3

      const lava = new THREE.Mesh(
        new THREE.CircleGeometry(poolRadius, 16),
        new THREE.MeshStandardMaterial({
          color: 0xff6600,
          emissive: new THREE.Color(0xff3300),
          emissiveIntensity: 5,
          metalness: 0.3,
          roughness: 0.4,
        })
      )
      lava.rotation.x = -Math.PI / 2
      lava.position.set(poolX, 0.1, poolZ)
      meshes.push(lava)

      // Lava bubble light (only for some pools)
      if (p % 2 === 0) {
        const lavaLight = new THREE.PointLight(0xff3300, 5, 25)
        lavaLight.position.set(poolX, 2, poolZ)
        lights.push(lavaLight)
      }
    }

    // Volcanic ember particles (300 flying embers - optimized)
    const emberCount = 300
    const emberGeometry = new THREE.BufferGeometry()
    const emberPositions = new Float32Array(emberCount * 3)
    const emberColors = new Float32Array(emberCount * 3)
    const emberSizes = new Float32Array(emberCount)

    for (let i = 0; i < emberCount; i++) {
      emberPositions[i * 3] = position.x + (Math.random() - 0.5) * 30
      emberPositions[i * 3 + 1] = Math.random() * 30
      emberPositions[i * 3 + 2] = position.z + (Math.random() - 0.5) * 30

      const heat = Math.random()
      emberColors[i * 3] = 1.0
      emberColors[i * 3 + 1] = heat * 0.5
      emberColors[i * 3 + 2] = 0.0

      emberSizes[i] = Math.random() * 0.5 + 0.1
    }

    emberGeometry.setAttribute('position', new THREE.BufferAttribute(emberPositions, 3))
    emberGeometry.setAttribute('color', new THREE.BufferAttribute(emberColors, 3))
    emberGeometry.setAttribute('size', new THREE.BufferAttribute(emberSizes, 1))

    const embers = new THREE.Points(
      emberGeometry,
      new THREE.PointsMaterial({
        size: 0.6, // Larger to compensate
        vertexColors: true,
        transparent: true,
        opacity: 0.9,
        blending: THREE.AdditiveBlending,
      })
    )
    particles.push(embers)

    return { meshes, particles, lights, position, genre: 'metal' }
  }
}

// Jazz: COZY Club with Atmospheric Smoke
export class JazzWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []
    const particles: THREE.Points[] = []
    const lights: THREE.PointLight[] = []

    // Large smooth wavy floor
    const floorGeometry = new THREE.PlaneGeometry(25, 25, 30, 30)
    const vertices = floorGeometry.attributes.position.array as Float32Array
    const time = Date.now() * 0.001

    for (let i = 0; i < vertices.length; i += 3) {
      const x = vertices[i]
      const y = vertices[i + 1]
      vertices[i + 2] =
        Math.sin(x * 0.4 + time) * 1 +
        Math.cos(y * 0.4 + time) * 1 +
        audioFeatures.rms * 3
    }
    floorGeometry.computeVertexNormals()

    const floor = new THREE.Mesh(
      floorGeometry,
      new THREE.MeshStandardMaterial({
        color: 0xb8860b,
        metalness: 0.5,
        roughness: 0.5,
        emissive: new THREE.Color(0x4a3400),
        emissiveIntensity: 0.4,
      })
    )
    floor.rotation.x = -Math.PI / 2
    floor.position.copy(position)
    floor.receiveShadow = true
    meshes.push(floor)

    // Multiple curved arches (3-5 arches - optimized)
    const archCount = 3 + Math.floor(audioFeatures.spectralCentroid * 0.0007)
    for (let i = 0; i < archCount; i++) {
      const archHeight = 8 + audioFeatures.spectralCentroid * 0.003
      const archAngle = (i / archCount) * Math.PI * 2
      const archRadius = 10

      const arch = new THREE.Mesh(
        new THREE.TorusGeometry(4, 0.4, 16, 32, Math.PI),
        new THREE.MeshStandardMaterial({
          color: 0xcd853f,
          metalness: 0.4,
          roughness: 0.4,
        })
      )
      arch.rotation.x = Math.PI / 2
      arch.position.set(
        position.x + Math.cos(archAngle) * archRadius,
        archHeight,
        position.z + Math.sin(archAngle) * archRadius
      )
      meshes.push(arch)
    }

    // Warm hanging lamps (5-8 lamps - optimized)
    const lampCount = 5 + Math.floor(audioFeatures.beatConfidence * 3)
    for (let i = 0; i < lampCount; i++) {
      const lampX = position.x + (Math.random() - 0.5) * 20
      const lampZ = position.z + (Math.random() - 0.5) * 20
      const lampHeight = 6 + Math.random() * 4

      // Lamp shade
      const lamp = new THREE.Mesh(
        new THREE.SphereGeometry(0.6, 16, 16),
        new THREE.MeshStandardMaterial({
          color: 0xffa500,
          emissive: new THREE.Color(0xff8c00),
          emissiveIntensity: 2 + audioFeatures.beatConfidence * 3,
          transparent: true,
          opacity: 0.85,
        })
      )
      lamp.position.set(lampX, lampHeight, lampZ)
      meshes.push(lamp)

      // Warm glow light (every other lamp)
      if (i % 2 === 0) {
        const lampLight = new THREE.PointLight(
          0xff8c00,
          3 + audioFeatures.beatConfidence * 5,
          15
        )
        lampLight.position.set(lampX, lampHeight, lampZ)
        lights.push(lampLight)
      }

      // Hanging chain
      const chain = new THREE.Mesh(
        new THREE.CylinderGeometry(0.05, 0.05, lampHeight - 0.5, 8),
        new THREE.MeshStandardMaterial({ color: 0x8b7355 })
      )
      chain.position.set(lampX, lampHeight + (lampHeight - 0.5) / 2, lampZ)
      meshes.push(chain)
    }

    // Cigarette smoke particles (atmospheric - optimized!)
    const smokeCount = 250
    const smokeGeometry = new THREE.BufferGeometry()
    const smokePositions = new Float32Array(smokeCount * 3)
    const smokeColors = new Float32Array(smokeCount * 3)
    const smokeSizes = new Float32Array(smokeCount)

    for (let i = 0; i < smokeCount; i++) {
      smokePositions[i * 3] = position.x + (Math.random() - 0.5) * 25
      smokePositions[i * 3 + 1] = Math.random() * 15
      smokePositions[i * 3 + 2] = position.z + (Math.random() - 0.5) * 25

      const gray = 0.6 + Math.random() * 0.3
      smokeColors[i * 3] = gray
      smokeColors[i * 3 + 1] = gray * 0.9
      smokeColors[i * 3 + 2] = gray * 0.8

      smokeSizes[i] = Math.random() * 2 + 0.5
    }

    smokeGeometry.setAttribute('position', new THREE.BufferAttribute(smokePositions, 3))
    smokeGeometry.setAttribute('color', new THREE.BufferAttribute(smokeColors, 3))
    smokeGeometry.setAttribute('size', new THREE.BufferAttribute(smokeSizes, 1))

    const smoke = new THREE.Points(
      smokeGeometry,
      new THREE.PointsMaterial({
        size: 2.0, // Larger particles
        vertexColors: true,
        transparent: true,
        opacity: 0.35,
      })
    )
    particles.push(smoke)

    return { meshes, particles, lights, position, genre: 'jazz' }
  }
}

// Electronic: MASSIVE Neon City with Light Trails
export class ElectronicWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []
    const particles: THREE.Points[] = []
    const lights: THREE.PointLight[] = []

    // Large metallic grid floor
    const grid = new THREE.Mesh(
      new THREE.PlaneGeometry(30, 30, 15, 15),
      new THREE.MeshStandardMaterial({
        color: 0x000033,
        metalness: 1.0,
        roughness: 0.1,
        wireframe: true,
        emissive: new THREE.Color(0x0000ff),
        emissiveIntensity: 1 + audioFeatures.rms * 2,
      })
    )
    grid.rotation.x = -Math.PI / 2
    grid.position.copy(position)
    meshes.push(grid)

    // Solid floor underneath
    const solidFloor = new THREE.Mesh(
      new THREE.PlaneGeometry(30, 30),
      new THREE.MeshStandardMaterial({
        color: 0x000022,
        metalness: 0.8,
        roughness: 0.3,
      })
    )
    solidFloor.rotation.x = -Math.PI / 2
    solidFloor.position.set(position.x, -0.1, position.z)
    meshes.push(solidFloor)

    // MANY neon structures (8-15 buildings - optimized!)
    const structureCount = 8 + Math.floor(audioFeatures.beatConfidence * 7)

    for (let i = 0; i < structureCount; i++) {
      const structureType = Math.floor(Math.random() * 3)
      let height = 5 + audioFeatures.rms * 15 + Math.random() * 10

      const x = position.x + (Math.random() - 0.5) * 25
      const z = position.z + (Math.random() - 0.5) * 25

      const hue = (audioFeatures.spectralCentroid / 8000 + i * 0.1 + Math.random() * 0.2) % 1
      const color = new THREE.Color().setHSL(hue, 1.0, 0.5)

      let geometry: THREE.BufferGeometry

      if (structureType === 0) {
        // Tall neon cube/tower
        geometry = new THREE.BoxGeometry(2, height, 2)
      } else if (structureType === 1) {
        // Neon pyramid
        geometry = new THREE.ConeGeometry(1.5, height, 4)
      } else {
        // Neon cylinder
        geometry = new THREE.CylinderGeometry(1, 1, height, 8)
      }

      const structure = new THREE.Mesh(
        geometry,
        new THREE.MeshStandardMaterial({
          color: color,
          metalness: 1.0,
          roughness: 0.0,
          emissive: color,
          emissiveIntensity: 3,
          wireframe: true,
        })
      )

      structure.position.set(x, height / 2, z)
      structure.castShadow = true
      meshes.push(structure)

      // Neon light at top (every other structure for performance)
      if (i % 2 === 0) {
        const structureLight = new THREE.PointLight(color.getHex(), 5, 20)
        structureLight.position.set(x, height, z)
        lights.push(structureLight)
      }
    }

    // Vertical light beams (5-8 beams - optimized)
    const beamCount = 5 + Math.floor(audioFeatures.beatConfidence * 3)
    for (let i = 0; i < beamCount; i++) {
      const beamX = position.x + (Math.random() - 0.5) * 28
      const beamZ = position.z + (Math.random() - 0.5) * 28
      const beamHeight = 15 + Math.random() * 15

      const beam = new THREE.Mesh(
        new THREE.CylinderGeometry(0.15, 0.15, beamHeight, 8),
        new THREE.MeshStandardMaterial({
          color: 0x00ffff,
          emissive: new THREE.Color(0x00ffff),
          emissiveIntensity: 4,
          transparent: true,
          opacity: 0.8,
        })
      )
      beam.position.set(beamX, beamHeight / 2, beamZ)
      meshes.push(beam)
    }

    // Neon light trail particles (400 particles - optimized!)
    const trailCount = 400
    const trailGeometry = new THREE.BufferGeometry()
    const trailPositions = new Float32Array(trailCount * 3)
    const trailColors = new Float32Array(trailCount * 3)
    const trailSizes = new Float32Array(trailCount)

    for (let i = 0; i < trailCount; i++) {
      trailPositions[i * 3] = position.x + (Math.random() - 0.5) * 35
      trailPositions[i * 3 + 1] = Math.random() * 25
      trailPositions[i * 3 + 2] = position.z + (Math.random() - 0.5) * 35

      const hue = Math.random()
      const color = new THREE.Color().setHSL(hue, 1.0, 0.5)
      trailColors[i * 3] = color.r
      trailColors[i * 3 + 1] = color.g
      trailColors[i * 3 + 2] = color.b

      trailSizes[i] = Math.random() * 0.8 + 0.2
    }

    trailGeometry.setAttribute('position', new THREE.BufferAttribute(trailPositions, 3))
    trailGeometry.setAttribute('color', new THREE.BufferAttribute(trailColors, 3))
    trailGeometry.setAttribute('size', new THREE.BufferAttribute(trailSizes, 1))

    const trails = new THREE.Points(
      trailGeometry,
      new THREE.PointsMaterial({
        size: 0.7, // Larger to compensate
        vertexColors: true,
        transparent: true,
        opacity: 0.95,
        blending: THREE.AdditiveBlending,
      })
    )
    particles.push(trails)

    return { meshes, particles, lights, position, genre: 'electronic' }
  }
}

// Ambient: ETHEREAL Dreamscape with Magical Wisps
export class AmbientWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []
    const particles: THREE.Points[] = []
    const lights: THREE.PointLight[] = []

    // Massive rolling terrain (40x40)
    const terrainGeometry = new THREE.PlaneGeometry(40, 40, 50, 50)
    const vertices = terrainGeometry.attributes.position.array as Float32Array
    const time = Date.now() * 0.0003

    for (let i = 0; i < vertices.length; i += 3) {
      const x = vertices[i]
      const y = vertices[i + 1]
      // Gentle rolling hills with multiple frequencies
      vertices[i + 2] =
        Math.sin(x * 0.2 + time) * 2.5 +
        Math.cos(y * 0.2 - time) * 2.5 +
        Math.sin(x * 0.1) * 4 +
        Math.cos(y * 0.1) * 4 +
        audioFeatures.rms * 5
    }
    terrainGeometry.computeVertexNormals()

    const terrain = new THREE.Mesh(
      terrainGeometry,
      new THREE.MeshStandardMaterial({
        color: 0x9370db,
        metalness: 0.2,
        roughness: 0.9,
        transparent: true,
        opacity: 0.85,
        emissive: new THREE.Color(0x483d8b),
        emissiveIntensity: 0.3,
      })
    )
    terrain.rotation.x = -Math.PI / 2
    terrain.position.copy(position)
    terrain.receiveShadow = true
    meshes.push(terrain)

    // Floating ethereal orbs (10-18 orbs - optimized!)
    const orbCount = 10 + Math.floor(audioFeatures.spectralFlux * 8)

    for (let i = 0; i < orbCount; i++) {
      const orbSize = 0.8 + Math.random() * 1.5
      const orbX = position.x + (Math.random() - 0.5) * 35
      const orbY = 3 + Math.random() * 15 + audioFeatures.rms * 5
      const orbZ = position.z + (Math.random() - 0.5) * 35

      const orb = new THREE.Mesh(
        new THREE.SphereGeometry(orbSize, 16, 16),
        new THREE.MeshStandardMaterial({
          color: 0x48d1cc,
          transparent: true,
          opacity: 0.5 + audioFeatures.rms * 0.4,
          emissive: new THREE.Color(0x48d1cc),
          emissiveIntensity: 2,
          metalness: 0.1,
          roughness: 0.9,
        })
      )

      orb.position.set(orbX, orbY, orbZ)
      meshes.push(orb)

      // Light inside some orbs (every other one for performance)
      if (i % 2 === 0) {
        const orbLight = new THREE.PointLight(0x48d1cc, 3, 12)
        orbLight.position.set(orbX, orbY, orbZ)
        lights.push(orbLight)
      }
    }

    // Multiple mist layers (3 layers - optimized)
    for (let layer = 0; layer < 3; layer++) {
      const mist = new THREE.Mesh(
        new THREE.PlaneGeometry(35, 10),
        new THREE.MeshStandardMaterial({
          color: 0xf0e68c,
          transparent: true,
          opacity: 0.12 + audioFeatures.zcr * 0.15,
          emissive: new THREE.Color(0xf0e68c),
          emissiveIntensity: 0.4,
          side: THREE.DoubleSide,
        })
      )
      mist.position.set(
        position.x + (Math.random() - 0.5) * 10,
        4 + layer * 2.5,
        position.z + (Math.random() - 0.5) * 10
      )
      mist.rotation.x = -Math.PI / 4 + (Math.random() - 0.5) * 0.5
      mist.rotation.y = Math.random() * Math.PI
      meshes.push(mist)
    }

    // Magical floating wisps/fireflies (350 particles - optimized!)
    const wispCount = 350
    const wispGeometry = new THREE.BufferGeometry()
    const wispPositions = new Float32Array(wispCount * 3)
    const wispColors = new Float32Array(wispCount * 3)
    const wispSizes = new Float32Array(wispCount)

    for (let i = 0; i < wispCount; i++) {
      wispPositions[i * 3] = position.x + (Math.random() - 0.5) * 40
      wispPositions[i * 3 + 1] = Math.random() * 20
      wispPositions[i * 3 + 2] = position.z + (Math.random() - 0.5) * 40

      const colorChoice = Math.random()
      if (colorChoice < 0.5) {
        // Turquoise
        wispColors[i * 3] = 0.28
        wispColors[i * 3 + 1] = 0.82
        wispColors[i * 3 + 2] = 0.8
      } else {
        // Lavender
        wispColors[i * 3] = 0.73
        wispColors[i * 3 + 1] = 0.44
        wispColors[i * 3 + 2] = 0.86
      }

      wispSizes[i] = Math.random() * 0.6 + 0.2
    }

    wispGeometry.setAttribute('position', new THREE.BufferAttribute(wispPositions, 3))
    wispGeometry.setAttribute('color', new THREE.BufferAttribute(wispColors, 3))
    wispGeometry.setAttribute('size', new THREE.BufferAttribute(wispSizes, 1))

    const wisps = new THREE.Points(
      wispGeometry,
      new THREE.PointsMaterial({
        size: 0.6, // Larger to compensate
        vertexColors: true,
        transparent: true,
        opacity: 0.8,
        blending: THREE.AdditiveBlending,
      })
    )
    particles.push(wisps)

    return { meshes, particles, lights, position, genre: 'ambient' }
  }
}

export function getWorldGenerator(genre: Genre) {
  switch (genre) {
    case 'classical':
      return new ClassicalWorldGenerator()
    case 'metal':
      return new RockWorldGenerator()
    case 'jazz':
      return new JazzWorldGenerator()
    case 'electronic':
      return new ElectronicWorldGenerator()
    case 'ambient':
      return new AmbientWorldGenerator()
    default:
      return new ElectronicWorldGenerator()
  }
}
