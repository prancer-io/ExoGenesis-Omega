/**
 * Procedural 3D World Generators for Each Genre
 * Creates actual explorable environments, not just effects
 */

import * as THREE from 'three'
import type { Genre } from '../App'

export interface WorldChunk {
  meshes: THREE.Mesh[]
  position: THREE.Vector3
  genre: Genre
}

// Classical: Marble Halls with Fountains
export class ClassicalWorldGenerator {
  private scene: THREE.Group

  constructor() {
    this.scene = new THREE.Group()
  }

  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []

    // Create marble floor tiles
    const floorTile = new THREE.Mesh(
      new THREE.BoxGeometry(5, 0.2, 5),
      new THREE.MeshStandardMaterial({
        color: 0xf5f5dc, // Beige marble
        metalness: 0.3,
        roughness: 0.4,
        normalScale: new THREE.Vector2(0.5, 0.5),
      })
    )
    floorTile.position.copy(position)
    floorTile.receiveShadow = true
    meshes.push(floorTile)

    // Create marble pillars (based on beat)
    if (audioFeatures.beatConfidence > 0.6) {
      const pillarHeight = 10 + audioFeatures.rms * 15
      const pillar = new THREE.Mesh(
        new THREE.CylinderGeometry(0.5, 0.6, pillarHeight, 16),
        new THREE.MeshStandardMaterial({
          color: 0xffffff,
          metalness: 0.2,
          roughness: 0.3,
        })
      )
      pillar.position.set(position.x + 3, pillarHeight / 2, position.z)
      pillar.castShadow = true
      meshes.push(pillar)

      // Pillar capital
      const capital = new THREE.Mesh(
        new THREE.BoxGeometry(1, 0.5, 1),
        new THREE.MeshStandardMaterial({ color: 0xffd700 }) // Gold accent
      )
      capital.position.set(position.x + 3, pillarHeight, position.z)
      meshes.push(capital)
    }

    // Create fountain (based on spectral centroid)
    if (audioFeatures.spectralCentroid > 3000) {
      const fountainBase = new THREE.Mesh(
        new THREE.CylinderGeometry(1.5, 2, 0.5, 8),
        new THREE.MeshStandardMaterial({
          color: 0xb0c4de, // Light steel blue
          metalness: 0.5,
          roughness: 0.2,
        })
      )
      fountainBase.position.set(position.x, 0.25, position.z + 3)
      meshes.push(fountainBase)

      // Water particles (simple spheres)
      const waterHeight = audioFeatures.rms * 5
      const water = new THREE.Mesh(
        new THREE.SphereGeometry(0.3, 8, 8),
        new THREE.MeshStandardMaterial({
          color: 0x87ceeb,
          transparent: true,
          opacity: 0.7,
        })
      )
      water.position.set(position.x, waterHeight, position.z + 3)
      meshes.push(water)
    }

    return { meshes, position, genre: 'classical' }
  }
}

// Rock: Volcanic Terrain with Crystals
export class RockWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []

    // Create jagged volcanic ground
    const groundGeometry = new THREE.PlaneGeometry(10, 10, 10, 10)
    const vertices = groundGeometry.attributes.position.array as Float32Array
    for (let i = 0; i < vertices.length; i += 3) {
      vertices[i + 2] = Math.random() * 2 - 1 + audioFeatures.zcr * 5 // Add roughness
    }
    groundGeometry.computeVertexNormals()

    const ground = new THREE.Mesh(
      groundGeometry,
      new THREE.MeshStandardMaterial({
        color: 0x8b0000, // Dark red (volcanic)
        metalness: 0.1,
        roughness: 0.9,
        emissive: new THREE.Color(0x330000),
        emissiveIntensity: audioFeatures.rms * 2,
      })
    )
    ground.rotation.x = -Math.PI / 2
    ground.position.copy(position)
    ground.receiveShadow = true
    meshes.push(ground)

    // Create crystal formations (on beats)
    if (audioFeatures.beatConfidence > 0.7) {
      const crystalHeight = 5 + audioFeatures.rms * 10
      const crystal = new THREE.Mesh(
        new THREE.ConeGeometry(0.5, crystalHeight, 6),
        new THREE.MeshStandardMaterial({
          color: 0xff4500, // Orange-red
          metalness: 0.9,
          roughness: 0.1,
          emissive: new THREE.Color(0xff0000),
          emissiveIntensity: audioFeatures.beatConfidence * 2,
        })
      )
      crystal.position.set(
        position.x + Math.random() * 4 - 2,
        crystalHeight / 2,
        position.z + Math.random() * 4 - 2
      )
      crystal.rotation.y = Math.random() * Math.PI
      crystal.castShadow = true
      meshes.push(crystal)
    }

    // Lava pools (based on spectral flux)
    if (audioFeatures.spectralFlux > 0.3) {
      const lava = new THREE.Mesh(
        new THREE.CircleGeometry(1.5, 16),
        new THREE.MeshStandardMaterial({
          color: 0xff6600,
          emissive: new THREE.Color(0xff3300),
          emissiveIntensity: 3,
          metalness: 0.0,
          roughness: 0.5,
        })
      )
      lava.rotation.x = -Math.PI / 2
      lava.position.set(position.x - 3, 0.05, position.z - 3)
      meshes.push(lava)
    }

    return { meshes, position, genre: 'metal' }
  }
}

// Jazz: Warm Amber Spaces with Smooth Undulations
export class JazzWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []

    // Create smooth undulating floor
    const floorGeometry = new THREE.PlaneGeometry(12, 12, 20, 20)
    const vertices = floorGeometry.attributes.position.array as Float32Array
    const time = Date.now() * 0.001

    for (let i = 0; i < vertices.length; i += 3) {
      const x = vertices[i]
      const y = vertices[i + 1]
      // Create smooth waves
      vertices[i + 2] = Math.sin(x * 0.5 + time) * 0.5 + Math.cos(y * 0.5 + time) * 0.5 + audioFeatures.rms * 2
    }
    floorGeometry.computeVertexNormals()

    const floor = new THREE.Mesh(
      floorGeometry,
      new THREE.MeshStandardMaterial({
        color: 0xb8860b, // Dark goldenrod
        metalness: 0.4,
        roughness: 0.6,
        emissive: new THREE.Color(0x4a3400),
        emissiveIntensity: 0.3,
      })
    )
    floor.rotation.x = -Math.PI / 2
    floor.position.copy(position)
    floor.receiveShadow = true
    meshes.push(floor)

    // Create smooth curved walls/arches
    const archHeight = 6 + audioFeatures.spectralCentroid * 0.002
    const arch = new THREE.Mesh(
      new THREE.TorusGeometry(3, 0.3, 16, 32, Math.PI),
      new THREE.MeshStandardMaterial({
        color: 0xcd853f, // Peru (tan)
        metalness: 0.3,
        roughness: 0.5,
      })
    )
    arch.rotation.x = Math.PI / 2
    arch.position.set(position.x, archHeight, position.z + 5)
    meshes.push(arch)

    // Warm hanging lamps (based on beat)
    if (audioFeatures.beatConfidence > 0.5) {
      const lamp = new THREE.Mesh(
        new THREE.SphereGeometry(0.5, 16, 16),
        new THREE.MeshStandardMaterial({
          color: 0xffa500, // Orange
          emissive: new THREE.Color(0xff8c00),
          emissiveIntensity: 1 + audioFeatures.beatConfidence * 2,
          transparent: true,
          opacity: 0.8,
        })
      )
      lamp.position.set(position.x, 5, position.z)
      meshes.push(lamp)
    }

    return { meshes, position, genre: 'jazz' }
  }
}

// Electronic: Neon-Lit Geometric Structures
export class ElectronicWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []

    // Create geometric grid floor
    const grid = new THREE.Mesh(
      new THREE.PlaneGeometry(10, 10, 10, 10),
      new THREE.MeshStandardMaterial({
        color: 0x000033,
        metalness: 1.0,
        roughness: 0.1,
        wireframe: false,
        emissive: new THREE.Color(0x0000ff),
        emissiveIntensity: audioFeatures.rms * 1.5,
      })
    )
    grid.rotation.x = -Math.PI / 2
    grid.position.copy(position)
    meshes.push(grid)

    // Create neon geometric structures (cubes, pyramids)
    const structureCount = Math.floor(audioFeatures.beatConfidence * 5) + 1

    for (let i = 0; i < structureCount; i++) {
      const structureType = i % 2
      let geometry: THREE.BufferGeometry
      let height: number

      if (structureType === 0) {
        // Neon cube
        height = 3 + audioFeatures.rms * 5
        geometry = new THREE.BoxGeometry(2, height, 2)
      } else {
        // Neon pyramid
        height = 4 + audioFeatures.rms * 6
        geometry = new THREE.ConeGeometry(1.5, height, 4)
      }

      const hue = (audioFeatures.spectralCentroid / 8000 + i * 0.2) % 1
      const color = new THREE.Color().setHSL(hue, 1.0, 0.5)

      const structure = new THREE.Mesh(
        geometry,
        new THREE.MeshStandardMaterial({
          color: color,
          metalness: 1.0,
          roughness: 0.0,
          emissive: color,
          emissiveIntensity: 2,
          wireframe: true,
        })
      )

      structure.position.set(
        position.x + (i - structureCount / 2) * 3,
        height / 2, // Half the height to place on ground
        position.z
      )
      structure.castShadow = true
      meshes.push(structure)
    }

    // Neon light beams
    const beam = new THREE.Mesh(
      new THREE.CylinderGeometry(0.1, 0.1, 20, 8),
      new THREE.MeshStandardMaterial({
        color: 0x00ffff,
        emissive: new THREE.Color(0x00ffff),
        emissiveIntensity: 3,
        transparent: true,
        opacity: 0.7,
      })
    )
    beam.position.set(position.x, 10, position.z)
    meshes.push(beam)

    return { meshes, position, genre: 'electronic' }
  }
}

// Ambient: Ethereal Fog-Shrouded Expanses
export class AmbientWorldGenerator {
  generateChunk(audioFeatures: any, position: THREE.Vector3): WorldChunk {
    const meshes: THREE.Mesh[] = []

    // Create soft, rolling terrain
    const terrainGeometry = new THREE.PlaneGeometry(15, 15, 30, 30)
    const vertices = terrainGeometry.attributes.position.array as Float32Array
    const time = Date.now() * 0.0005

    for (let i = 0; i < vertices.length; i += 3) {
      const x = vertices[i]
      const y = vertices[i + 1]
      // Gentle rolling hills
      vertices[i + 2] =
        Math.sin(x * 0.3 + time) * 1.5 +
        Math.cos(y * 0.3 - time) * 1.5 +
        audioFeatures.rms * 3
    }
    terrainGeometry.computeVertexNormals()

    const terrain = new THREE.Mesh(
      terrainGeometry,
      new THREE.MeshStandardMaterial({
        color: 0x9370db, // Medium purple
        metalness: 0.1,
        roughness: 0.9,
        transparent: true,
        opacity: 0.8,
        emissive: new THREE.Color(0x483d8b),
        emissiveIntensity: 0.2,
      })
    )
    terrain.rotation.x = -Math.PI / 2
    terrain.position.copy(position)
    terrain.receiveShadow = true
    meshes.push(terrain)

    // Floating ethereal orbs
    const orbCount = Math.floor(audioFeatures.spectralFlux * 10) + 2

    for (let i = 0; i < orbCount; i++) {
      const orb = new THREE.Mesh(
        new THREE.SphereGeometry(0.5 + Math.random() * 0.5, 16, 16),
        new THREE.MeshStandardMaterial({
          color: 0x48d1cc, // Medium turquoise
          transparent: true,
          opacity: 0.4 + audioFeatures.rms * 0.4,
          emissive: new THREE.Color(0x48d1cc),
          emissiveIntensity: 1.5,
          metalness: 0.0,
          roughness: 1.0,
        })
      )

      orb.position.set(
        position.x + Math.random() * 10 - 5,
        2 + Math.random() * 5 + audioFeatures.rms * 3,
        position.z + Math.random() * 10 - 5
      )
      meshes.push(orb)
    }

    // Mist/fog effect (using semi-transparent planes)
    const mist = new THREE.Mesh(
      new THREE.PlaneGeometry(20, 5),
      new THREE.MeshStandardMaterial({
        color: 0xf0e68c, // Khaki (light fog color)
        transparent: true,
        opacity: 0.15 + audioFeatures.zcr * 0.2,
        emissive: new THREE.Color(0xf0e68c),
        emissiveIntensity: 0.3,
        side: THREE.DoubleSide,
      })
    )
    mist.position.set(position.x, 3, position.z)
    mist.rotation.x = -Math.PI / 4
    meshes.push(mist)

    return { meshes, position, genre: 'ambient' }
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
