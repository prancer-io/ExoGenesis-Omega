# omega-synesthesia Web Viewer

Interactive web-based demo of omega-synesthesia real-time music visualization.

## Features

- ✨ **Real-Time 3D Visualization** - Three.js powered 3D graphics
- 🎵 **Multiple Genres** - Electronic, Classical, Jazz, Metal, Ambient
- 📷 **Camera Modes** - Orbit, Tracking, Cinematic, First-Person
- ⚡ **Performance Metrics** - FPS, latency, and audio features
- 🎮 **Interactive Controls** - Full UI and keyboard shortcuts

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

## Usage

1. Open `http://localhost:5173` in your browser
2. Use the control panel to:
   - Switch between genres
   - Change camera modes
   - Play/pause visualization
   - Adjust playback speed
3. Watch real-time performance metrics
4. Use keyboard shortcuts for quick controls

## Keyboard Shortcuts

- `Space` - Play/Pause
- `1-4` - Switch camera modes
- `G` - Cycle through genres
- `+/-` - Adjust speed
- `R` - Reset visualization

## Architecture

### Components

- **App.tsx** - Main application container
- **MusicVisualizer.tsx** - Three.js 3D visualization
- **ControlPanel.tsx** - Interactive controls UI
- **PerformanceMetrics.tsx** - Real-time metrics display
- **audioProcessor.ts** - Audio feature extraction (simulated)

### Tech Stack

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Three.js** - 3D graphics
- **React Three Fiber** - React renderer for Three.js
- **Vite** - Build tool

## How It Works

This web demo simulates the omega-synesthesia Rust engine in the browser:

1. **Audio Generation** - Synthetic audio with musical patterns
2. **Feature Extraction** - FFT, spectral centroid, RMS, beat detection
3. **World Generation** - Procedural 3D geometry based on audio features
4. **Rendering** - WebGL 2.0 powered 3D visualization
5. **Camera Animation** - Automatic camera movements matching music

## Performance

- Target: 60 FPS
- Latency: ~16ms (browser rendering)
- Chunk Generation: Real-time

Compare with native Rust:
- Native latency: 2.79ms (5.7x faster)
- Native FPS: 60+ sustained
- Native GPU: Vulkan/Metal/DX12 (higher quality)

## Browser Compatibility

- ✅ Chrome 90+
- ✅ Firefox 88+
- ✅ Safari 15+
- ✅ Edge 90+

Requires WebGL 2.0 support.

## Development

```bash
# Run type checking
npm run tsc

# Run linter
npm run lint

# Preview production build
npm run build && npm run preview
```

## Deployment

### GitHub Pages

```bash
npm run build
# Deploy dist/ folder to GitHub Pages
```

### Netlify/Vercel

```bash
# Connect your repository
# Build command: npm run build
# Publish directory: dist
```

## License

Same as omega-synesthesia parent project.

## Related

- [omega-synesthesia](../../omega/crates/omega-synesthesia/) - Native Rust engine
- [Performance Reports](../../docs/) - Technical documentation
- [Examples](../../omega/crates/omega-examples/) - Native Rust examples
