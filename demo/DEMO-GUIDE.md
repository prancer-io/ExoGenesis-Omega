# omega-synesthesia Demo Guide

## Quick Start Options

We provide **3 ways** to experience omega-synesthesia for third-party validation:

### Option 1: Web Demo (Easiest - Browser Only) 🌐

**Best for**: Third-party reviewers, quick testing, sharing demos

```bash
cd demo/web-viewer
npm install
npm run dev
```

Then open `http://localhost:5173` in your browser.

**Features**:
- Real-time 3D visualization
- Interactive controls (genre, camera, speed)
- Performance metrics
- Works on any platform with a modern browser
- No Rust installation required

---

### Option 2: Native Rust Example (Best Performance) ⚡

**Best for**: Technical evaluation, performance validation

```bash
cd omega
cargo run --example week3_final_integration --release
```

**Features**:
- Full omega-synesthesia engine
- GPU-accelerated rendering (wgpu)
- <3ms latency
- Real-time audio processing

---

### Option 3: Interactive Demo (Full Control) 🎮

**Best for**: Live demonstrations, presentations

```bash
cd omega
cargo run --example interactive_demo --release
```

**Controls**:
- `SPACE` - Play/Pause
- `1-4` - Camera modes
- `G` - Cycle genres
- `+/-` - Adjust speed
- `R` - Reset
- `H` - Help

---

## Web Demo Features

The web demo (Option 1) provides a complete browser-based experience:

### Real-Time Visualization
- 3D music world rendering with Three.js
- Procedural geometry generation
- PBR-style materials
- Camera animations

### Interactive Controls
- **Genre Selection**: Classical, Jazz, Electronic, Metal, Ambient
- **Camera Modes**: Orbit, Tracking, Cinematic, First-Person
- **Playback Controls**: Play/Pause, Speed adjustment
- **Audio Input**: Microphone, File upload, or generated demo audio

### Performance Metrics
- FPS counter
- Latency measurements
- Chunk generation stats
- Real-time graphs

### Visual Features
- Beat-reactive effects
- Genre-specific color palettes
- Dynamic lighting
- Smooth animations

---

## System Requirements

### Web Demo (Option 1)
- Modern browser (Chrome, Firefox, Safari, Edge)
- WebGL 2.0 support
- Node.js 18+ (for development only)

### Native Examples (Options 2 & 3)
- Rust 1.70+
- GPU with Vulkan/Metal/DX12 support
- Linux/macOS/Windows

---

## Performance Comparison

| Metric | Web Demo | Native (Rust) |
|--------|----------|---------------|
| Latency | ~16ms (60 FPS) | 2.79ms |
| FPS | 60 | 60+ |
| GPU | WebGL 2.0 | Vulkan/Metal/DX12 |
| Platform | Any (browser) | Desktop |
| Setup | 5 minutes | 10 minutes |

---

## Demo Scenarios

### Scenario 1: Quick Validation (5 minutes)
1. Run web demo
2. Test different genres
3. Check performance metrics
4. Try camera modes

### Scenario 2: Technical Evaluation (15 minutes)
1. Run native example
2. Measure latency benchmarks
3. Test with different audio inputs
4. Review code architecture

### Scenario 3: Live Presentation (30 minutes)
1. Run interactive demo
2. Connect live audio input
3. Demonstrate real-time capabilities
4. Show genre switching
5. Display performance metrics

---

## Recording a Demo Video

### Using OBS Studio

```bash
# 1. Start the demo
cargo run --example interactive_demo --release

# 2. In OBS:
# - Add Window Capture
# - Add Audio Input Capture
# - Set recording quality to 1080p60

# 3. Record your session showing:
# - Genre changes
# - Camera modes
# - Performance metrics
# - Real-time responsiveness
```

### Command-line Recording (Linux)

```bash
# Start demo
cargo run --example interactive_demo --release &

# Record with ffmpeg
ffmpeg -video_size 1920x1080 -framerate 60 \
  -f x11grab -i :0.0+0,0 \
  -f pulse -ac 2 -i default \
  -c:v libx264 -preset ultrafast -c:a aac \
  omega-synesthesia-demo.mp4
```

---

## Sharing with Third Parties

### Option A: Send Web Demo
1. Build the web demo: `npm run build`
2. Host on GitHub Pages / Netlify / Vercel
3. Share the URL - anyone can test instantly

### Option B: Send Executable
```bash
# Build release binary
cargo build --release --example interactive_demo

# Binary location:
./target/release/examples/interactive_demo

# Package with instructions
```

### Option C: Send Video
1. Record a demo video (see above)
2. Upload to YouTube/Vimeo
3. Share link with documentation

---

## Troubleshooting

### Web Demo Issues

**Error**: "WebGL not supported"
- **Fix**: Update your browser or try a different one

**Error**: "Audio input not available"
- **Fix**: Use generated demo audio or upload a music file

### Native Demo Issues

**Error**: "No suitable GPU adapter found"
- **Fix**: Update graphics drivers

**Error**: "Failed to create surface"
- **Fix**: On Linux, install `libwayland-dev` or `libxcb-dev`

---

## Next Steps

After testing the demo:

1. **Read the Documentation**: See `/docs/RELEASE-NOTES-V1.0.0.md`
2. **Review Performance**: See `/docs/V1.0.0-VALIDATION-REPORT.md`
3. **Explore Code**: See `/docs/WEEK-3-COMPLETION-REPORT.md`
4. **Check Examples**: See `/omega/crates/omega-examples/examples/`

---

## Contact & Support

- **Issues**: https://github.com/prancer-io/ExoGenesis-Omega/issues
- **Documentation**: `/docs/`
- **Examples**: `/omega/crates/omega-examples/`

---

**omega-synesthesia V1.0.0**
Transform music into worlds. Experience sound spatially. Walk through a symphony.
