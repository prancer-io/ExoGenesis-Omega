/**
 * World Generator - Creates 3D worlds from musical features
 * Simulates omega-synesthesia's StreamingWorldGenerator
 */

import type { Genre } from '../App'

export interface WorldElement {
  id: string
  type: 'cube' | 'sphere' | 'cone' | 'cylinder'
  position: [number, number, number]
  scale: [number, number, number]
  rotation: [number, number, number]
  color: string
  metalness: number
  roughness: number
  emissiveIntensity: number
  timestamp: number
}

export interface AudioFeatures {
  spectralCentroid: number
  rms: number
  zcr: number
  dominantFreq: number
  spectralFlux: number
  beatConfidence: number
  tempo: number
}

export class WorldGenerator {
  private chunks: WorldElement[][] = []
  private currentChunk: WorldElement[] = []
  private chunkSize: number = 10 // Elements per chunk
  private timelinePosition: number = 0
  private genre: Genre = 'electronic'
  private elementCounter: number = 0

  constructor(genre: Genre) {
    this.genre = genre
  }

  setGenre(genre: Genre) {
    this.genre = genre
  }

  /**
   * Add musical features and generate world elements
   */
  addFeatures(features: AudioFeatures): WorldElement[] | null {
    const elements = this.generateElements(features)
    this.currentChunk.push(...elements)

    // When chunk is full, finalize it
    if (this.currentChunk.length >= this.chunkSize) {
      const completedChunk = [...this.currentChunk]
      this.chunks.push(completedChunk)
      this.currentChunk = []
      return completedChunk
    }

    return null
  }

  /**
   * Generate world elements from audio features
   */
  private generateElements(features: AudioFeatures): WorldElement[] {
    const elements: WorldElement[] = []
    const z = -this.timelinePosition * 3

    // Select shape based on genre and frequency
    const shape = this.selectShape(features)

    // Calculate size based on RMS energy - MUCH BIGGER!
    const baseScale = 2 + features.rms * 8
    const height = 3 + features.rms * 15

    // Calculate position based on spectral centroid
    const xOffset = this.mapFrequencyToPosition(features.spectralCentroid)
    const yOffset = 2 + features.rms * 10

    // Get genre-specific colors
    const color = this.getColorFromFeatures(features)

    // Main element - with rotation and stronger glow
    elements.push({
      id: `element-${this.elementCounter++}`,
      type: shape,
      position: [xOffset, yOffset, z],
      scale: shape === 'cube' ? [baseScale, height, baseScale] : [baseScale * 1.5, baseScale * 1.5, baseScale * 1.5],
      rotation: [
        Math.sin(this.timelinePosition * 0.05) * 0.3,
        this.timelinePosition * 0.1,
        Math.cos(this.timelinePosition * 0.05) * 0.3
      ],
      color,
      metalness: this.getMetalness(),
      roughness: this.getRoughness(),
      emissiveIntensity: 0.3 + features.beatConfidence * 0.7, // Much brighter glow!
      timestamp: this.timelinePosition,
    })

    // Add harmonic elements for higher spectral flux - BIGGER and BRIGHTER!
    if (features.spectralFlux > 0.15) {
      const sideScale = baseScale * 0.8
      elements.push({
        id: `harmonic-l-${this.elementCounter++}`,
        type: 'sphere',
        position: [xOffset - 5, yOffset * 0.7, z],
        scale: [sideScale, sideScale, sideScale],
        rotation: [0, this.timelinePosition * 0.15, 0],
        color: this.getSecondaryColor(),
        metalness: this.getMetalness(),
        roughness: this.getRoughness(),
        emissiveIntensity: 0.5 + features.beatConfidence * 0.5,
        timestamp: this.timelinePosition,
      })

      elements.push({
        id: `harmonic-r-${this.elementCounter++}`,
        type: 'sphere',
        position: [xOffset + 5, yOffset * 0.7, z],
        scale: [sideScale, sideScale, sideScale],
        rotation: [0, -this.timelinePosition * 0.15, 0],
        color: this.getSecondaryColor(),
        metalness: this.getMetalness(),
        roughness: this.getRoughness(),
        emissiveIntensity: 0.5 + features.beatConfidence * 0.5,
        timestamp: this.timelinePosition,
      })
    }

    // Beat marker for high confidence - MUCH BIGGER and BRIGHTER!
    if (features.beatConfidence > 0.6) {
      const beatSize = 2 + features.rms * 3
      elements.push({
        id: `beat-${this.elementCounter++}`,
        type: 'sphere',
        position: [0, 12 + features.rms * 5, z],
        scale: [beatSize, beatSize, beatSize],
        rotation: [0, this.timelinePosition * 0.2, 0],
        color: '#ffff00', // Bright yellow
        metalness: 1.0,
        roughness: 0.0,
        emissiveIntensity: 2.0, // SUPER bright!
        timestamp: this.timelinePosition,
      })
    }

    this.timelinePosition++
    return elements
  }

  /**
   * Select shape based on genre and audio features
   */
  private selectShape(features: AudioFeatures): WorldElement['type'] {
    switch (this.genre) {
      case 'electronic':
        return features.zcr > 0.1 ? 'cube' : 'sphere'
      case 'classical':
        return features.spectralCentroid > 2000 ? 'cone' : 'sphere'
      case 'jazz':
        return features.beatConfidence > 0.6 ? 'cylinder' : 'cube'
      case 'metal':
        return 'cube'
      case 'ambient':
        return 'sphere'
      default:
        return 'cube'
    }
  }

  /**
   * Map frequency to X position
   */
  private mapFrequencyToPosition(centroid: number): number {
    // Map 0-8000 Hz to -5 to +5
    return ((centroid - 4000) / 4000) * 5
  }

  /**
   * Get color based on genre and features
   */
  private getColorFromFeatures(features: AudioFeatures): string {
    const colors = this.getGenreColors()

    // Select color based on spectral centroid
    if (features.spectralCentroid > 3000) {
      return colors.primary
    } else if (features.spectralCentroid > 1500) {
      return colors.secondary
    } else {
      return colors.tertiary
    }
  }

  private getSecondaryColor(): string {
    return this.getGenreColors().secondary
  }

  private getGenreColors() {
    const genreColors = {
      electronic: { primary: '#00ffff', secondary: '#ff00ff', tertiary: '#ffff00' },
      classical: { primary: '#ffd700', secondary: '#ffffff', tertiary: '#87ceeb' },
      jazz: { primary: '#b8860b', secondary: '#cd853f', tertiary: '#daa520' },
      metal: { primary: '#ff0000', secondary: '#ffffff', tertiary: '#808080' },
      ambient: { primary: '#9370db', secondary: '#48d1cc', tertiary: '#f0e68c' },
    }
    return genreColors[this.genre]
  }

  private getMetalness(): number {
    switch (this.genre) {
      case 'electronic': return 0.7
      case 'classical': return 0.2
      case 'jazz': return 0.5
      case 'metal': return 0.9
      case 'ambient': return 0.3
      default: return 0.5
    }
  }

  private getRoughness(): number {
    switch (this.genre) {
      case 'electronic': return 0.3
      case 'classical': return 0.6
      case 'jazz': return 0.5
      case 'metal': return 0.2
      case 'ambient': return 0.7
      default: return 0.5
    }
  }

  /**
   * Get all generated elements
   */
  getAllElements(): WorldElement[] {
    const all: WorldElement[] = []
    for (const chunk of this.chunks) {
      all.push(...chunk)
    }
    all.push(...this.currentChunk)
    return all
  }

  /**
   * Get timeline position for camera
   */
  getTimelinePosition(): number {
    return this.timelinePosition
  }

  /**
   * Reset world
   */
  reset() {
    this.chunks = []
    this.currentChunk = []
    this.timelinePosition = 0
    this.elementCounter = 0
  }
}
