/**
 * User Interface Components
 * Minimal, beautiful overlay UI
 */

import { useEffect, useRef } from 'react'
import { useStore, type Genre } from '../store'

const genreLabels: Record<Genre, { name: string; icon: string }> = {
  electronic: { name: 'ELECTRONIC', icon: '⚡' },
  classical: { name: 'CLASSICAL', icon: '🎻' },
  metal: { name: 'METAL', icon: '🔥' },
  ambient: { name: 'AMBIENT', icon: '🌙' },
  jazz: { name: 'JAZZ', icon: '🎷' },
}

export function UI() {
  const genre = useStore((s) => s.genre)
  const setGenre = useStore((s) => s.setGenre)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const showUI = useStore((s) => s.showUI)
  const setShowUI = useStore((s) => s.setShowUI)

  // Toggle UI with 'H' key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'h' || e.key === 'H') {
        setShowUI(!showUI)
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [showUI, setShowUI])

  if (!showUI) return null

  return (
    <div className="ui-overlay">
      {/* Title */}
      <div className="title-bar">
        <h1>MUSIC WORLD EXPLORER</h1>
        <p className="subtitle">Transform Sound Into Worlds</p>
      </div>

      {/* Audio Visualizer */}
      <AudioVisualizer frequencies={audioFeatures.frequencyBands} />

      {/* Genre Selector */}
      <div className="genre-selector">
        {(Object.keys(genreLabels) as Genre[]).map((g) => (
          <button
            key={g}
            className={`genre-btn ${g} ${genre === g ? 'active' : ''}`}
            onClick={() => setGenre(g)}
          >
            {genreLabels[g].icon} {genreLabels[g].name}
          </button>
        ))}
      </div>

      {/* Controls hint */}
      <div className="controls-hint">
        <p><kbd>H</kbd> Toggle UI</p>
        <p><kbd>1-5</kbd> Switch Genre</p>
        <p><kbd>SPACE</kbd> Pause/Play</p>
      </div>

      {/* Stats */}
      <StatsPanel />
    </div>
  )
}

// Audio frequency visualizer
function AudioVisualizer({ frequencies }: { frequencies: number[] }) {
  return (
    <div className="audio-visualizer">
      {frequencies.slice(0, 32).map((freq, i) => (
        <div
          key={i}
          className="audio-bar"
          style={{
            height: `${Math.max(4, freq * 60)}px`,
            opacity: 0.5 + freq * 0.5,
          }}
        />
      ))}
    </div>
  )
}

// Performance stats
function StatsPanel() {
  const audioFeatures = useStore((s) => s.audioFeatures)
  const genre = useStore((s) => s.genre)

  return (
    <div className="stats-panel">
      <p>Genre: {genre.toUpperCase()}</p>
      <p>BPM: {audioFeatures.bpm}</p>
      <p>Energy: {(audioFeatures.rms * 100).toFixed(0)}%</p>
      <p>Bass: {(audioFeatures.bass * 100).toFixed(0)}%</p>
    </div>
  )
}

// Loading screen
export function LoadingScreen({ progress }: { progress: number }) {
  return (
    <div className="loading-screen">
      <h1>LOADING</h1>
      <div className="loading-bar">
        <div className="loading-bar-fill" style={{ width: `${progress}%` }} />
      </div>
    </div>
  )
}
