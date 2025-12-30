/**
 * User Interface Components
 * Minimal, beautiful overlay UI with audio controls
 */

import { useEffect, useRef, useCallback } from 'react'
import { useStore, type Genre, type AudioMode } from '../store'

const genreLabels: Record<Genre, { name: string; icon: string; story: string }> = {
  electronic: { name: 'ELECTRONIC', icon: '⚡', story: 'Digital Awakening' },
  classical: { name: 'CLASSICAL', icon: '🎻', story: 'Symphony of Light' },
  metal: { name: 'METAL', icon: '🔥', story: 'Forge of Destruction' },
  ambient: { name: 'AMBIENT', icon: '🌙', story: 'Journey to Enlightenment' },
  jazz: { name: 'JAZZ', icon: '🎷', story: 'Night at the Club' },
}

export function UI() {
  const genre = useStore((s) => s.genre)
  const setGenre = useStore((s) => s.setGenre)
  const audioFeatures = useStore((s) => s.audioFeatures)
  const showUI = useStore((s) => s.showUI)
  const setShowUI = useStore((s) => s.setShowUI)
  const storyState = useStore((s) => s.storyState)
  const audioMode = useStore((s) => s.audioMode)
  const setAudioMode = useStore((s) => s.setAudioMode)
  const setAudioFile = useStore((s) => s.setAudioFile)
  const setAudioFileName = useStore((s) => s.setAudioFileName)
  const audioFileName = useStore((s) => s.audioFileName)
  const volume = useStore((s) => s.volume)
  const setVolume = useStore((s) => s.setVolume)
  const isPlaying = useStore((s) => s.isPlaying)
  const setIsPlaying = useStore((s) => s.setIsPlaying)

  const fileInputRef = useRef<HTMLInputElement>(null)

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

  const handleFileSelect = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file) {
      setAudioFile(file)
      setAudioFileName(file.name)
      setAudioMode('file')
    }
  }, [setAudioFile, setAudioFileName, setAudioMode])

  const handleModeToggle = useCallback(() => {
    const newMode: AudioMode = audioMode === 'synth' ? 'file' : 'synth'
    setAudioMode(newMode)
  }, [audioMode, setAudioMode])

  if (!showUI) return null

  return (
    <div className="ui-overlay">
      {/* Title */}
      <div className="title-bar">
        <h1>MUSIC WORLD EXPLORER</h1>
        <p className="subtitle">Transform Sound Into Worlds</p>
      </div>

      {/* Story Chapter Display */}
      <div className="story-display">
        <div className="story-title">{genreLabels[genre].story}</div>
        <div className="chapter-name">Chapter: {storyState.chapterName}</div>
        <div className="chapter-progress">
          <div
            className="chapter-progress-fill"
            style={{ width: `${storyState.chapterProgress * 100}%` }}
          />
        </div>
      </div>

      {/* Audio Visualizer */}
      <AudioVisualizer frequencies={audioFeatures.frequencyBands} />

      {/* Audio Controls */}
      <div className="audio-controls">
        <div className="audio-mode-toggle">
          <button
            className={`mode-btn ${audioMode === 'synth' ? 'active' : ''}`}
            onClick={() => setAudioMode('synth')}
          >
            🎹 Synth
          </button>
          <button
            className={`mode-btn ${audioMode === 'file' ? 'active' : ''}`}
            onClick={() => fileInputRef.current?.click()}
          >
            📁 Your Music
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept="audio/*"
            onChange={handleFileSelect}
            style={{ display: 'none' }}
          />
        </div>

        {audioFileName && audioMode === 'file' && (
          <div className="file-name">Playing: {audioFileName}</div>
        )}

        <div className="playback-controls">
          <button
            className="play-btn"
            onClick={() => setIsPlaying(!isPlaying)}
          >
            {isPlaying ? '⏸' : '▶'}
          </button>

          <div className="volume-control">
            <span>🔊</span>
            <input
              type="range"
              min="0"
              max="1"
              step="0.05"
              value={volume}
              onChange={(e) => setVolume(parseFloat(e.target.value))}
              className="volume-slider"
            />
          </div>
        </div>
      </div>

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
  const audioMode = useStore((s) => s.audioMode)

  return (
    <div className="stats-panel">
      <p>Genre: {genre.toUpperCase()}</p>
      <p>Mode: {audioMode === 'synth' ? 'Synthesizer' : 'Audio File'}</p>
      <p>BPM: {audioFeatures.bpm || '—'}</p>
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
