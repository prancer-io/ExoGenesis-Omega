import type { Genre, CameraMode } from '../App'
import type { AudioSource } from '../utils/audioProcessor'
import './ControlPanel.css'

interface ControlPanelProps {
  genre: Genre
  onGenreChange: (genre: Genre) => void
  cameraMode: CameraMode
  onCameraModeChange: (mode: CameraMode) => void
  isPlaying: boolean
  onPlayPauseToggle: () => void
  speed: number
  onSpeedChange: (speed: number) => void
  audioSource: AudioSource
  microphoneActive: boolean
  onToggleMicrophone: () => void
  onReset: () => void
  audioLevel: number // RMS audio level (0-1)
}

function ControlPanel({
  genre,
  onGenreChange,
  cameraMode,
  onCameraModeChange,
  isPlaying,
  onPlayPauseToggle,
  speed,
  onSpeedChange,
  audioSource,
  microphoneActive,
  onToggleMicrophone,
  onReset,
  audioLevel,
}: ControlPanelProps) {
  const genres: Genre[] = ['electronic', 'classical', 'jazz', 'metal', 'ambient']
  const cameraModes: CameraMode[] = ['orbit', 'tracking', 'cinematic', 'firstPerson']

  return (
    <div className="control-panel">
      <h2>Controls</h2>

      <div className="control-section">
        <h3>Genre</h3>
        <div className="button-group">
          {genres.map(g => (
            <button
              key={g}
              className={genre === g ? 'active' : ''}
              onClick={() => onGenreChange(g)}
            >
              {g.charAt(0).toUpperCase() + g.slice(1)}
            </button>
          ))}
        </div>
      </div>

      <div className="control-section">
        <h3>Camera Mode</h3>
        <div className="button-group">
          {cameraModes.map(mode => (
            <button
              key={mode}
              className={cameraMode === mode ? 'active' : ''}
              onClick={() => onCameraModeChange(mode)}
            >
              {mode.charAt(0).toUpperCase() + mode.slice(1)}
            </button>
          ))}
        </div>
      </div>

      <div className="control-section">
        <h3>Audio Input</h3>
        <button
          className={`mic-button ${microphoneActive ? 'active' : ''}`}
          onClick={onToggleMicrophone}
        >
          {microphoneActive ? '🎤 Microphone ON' : '🎤 Enable Microphone'}
        </button>

        {microphoneActive && (
          <>
            {/* Volume Bar - only shown when mic is active */}
            <div className="volume-bar-container">
              <div className="volume-bar-label">
                <span>Microphone Level:</span>
                <span className="volume-value">{(audioLevel * 100).toFixed(0)}%</span>
              </div>
              <div className="volume-bar-track">
                <div
                  className="volume-bar-fill"
                  style={{
                    width: `${Math.min(audioLevel * 100, 100)}%`,
                    backgroundColor: audioLevel > 0.8 ? '#ff4444' : audioLevel > 0.5 ? '#ffaa00' : '#00ff88'
                  }}
                />
              </div>
            </div>

            <div className="audio-source-info">
              🎤 Listening to microphone...
            </div>
          </>
        )}

        {!microphoneActive && (
          <div className="audio-source-info" style={{ opacity: 0.6 }}>
            Click "Enable Microphone" to start
          </div>
        )}
      </div>

      <div className="control-section">
        <h3>Controls</h3>
        {microphoneActive && (
          <>
            <button className="playback-button" onClick={onPlayPauseToggle}>
              {isPlaying ? '⏸ Pause Visualization' : '▶ Resume Visualization'}
            </button>

            <div className="slider-control">
              <label>Speed: {speed.toFixed(1)}x</label>
              <input
                type="range"
                min="0.1"
                max="2.0"
                step="0.1"
                value={speed}
                onChange={e => onSpeedChange(parseFloat(e.target.value))}
              />
            </div>
          </>
        )}

        <button className="reset-button" onClick={onReset}>
          🔄 Reset World
        </button>
      </div>

      <div className="control-section">
        <h3>Keyboard Shortcuts</h3>
        <div className="shortcuts">
          <kbd>Space</kbd> Play/Pause<br />
          <kbd>1-4</kbd> Camera Modes<br />
          <kbd>G</kbd> Cycle Genre<br />
          <kbd>+/-</kbd> Adjust Speed<br />
          <kbd>R</kbd> Reset
        </div>
      </div>
    </div>
  )
}

export default ControlPanel
