import './PerformanceMetrics.css'

interface PerformanceMetricsProps {
  fps: number
  latency: number
  chunksGenerated: number
  audioFeatures: {
    spectralCentroid: number
    rms: number
    zcr: number
    dominantFreq: number
    spectralFlux: number
    beatConfidence: number
    tempo: number
  }
}

function PerformanceMetrics({
  fps,
  latency,
  chunksGenerated,
  audioFeatures,
}: PerformanceMetricsProps) {
  return (
    <div className="performance-metrics">
      <h2>Performance</h2>

      <div className="metrics-grid">
        <div className="metric">
          <div className="metric-label">FPS</div>
          <div className="metric-value">{fps.toFixed(1)}</div>
        </div>

        <div className="metric">
          <div className="metric-label">Latency</div>
          <div className="metric-value">{latency.toFixed(2)}ms</div>
        </div>

        <div className="metric">
          <div className="metric-label">Chunks</div>
          <div className="metric-value">{chunksGenerated}</div>
        </div>
      </div>

      <h3>Audio Features</h3>
      <div className="audio-features">
        <div className="feature">
          <span>Spectral Centroid:</span>
          <span>{audioFeatures.spectralCentroid.toFixed(1)} Hz</span>
        </div>
        <div className="feature">
          <span>RMS Energy:</span>
          <span>{audioFeatures.rms.toFixed(3)}</span>
        </div>
        <div className="feature">
          <span>Zero Crossing Rate:</span>
          <span>{audioFeatures.zcr.toFixed(3)}</span>
        </div>
        <div className="feature">
          <span>Dominant Frequency:</span>
          <span>{audioFeatures.dominantFreq.toFixed(1)} Hz</span>
        </div>
        <div className="feature">
          <span>Spectral Flux:</span>
          <span>{audioFeatures.spectralFlux.toFixed(3)}</span>
        </div>
        <div className="feature">
          <span>Beat Confidence:</span>
          <div className="confidence-bar">
            <div
              className="confidence-fill"
              style={{ width: `${audioFeatures.beatConfidence * 100}%` }}
            />
          </div>
        </div>
        <div className="feature">
          <span>Tempo:</span>
          <span>{audioFeatures.tempo} BPM</span>
        </div>
      </div>
    </div>
  )
}

export default PerformanceMetrics
