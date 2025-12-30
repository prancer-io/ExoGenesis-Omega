# SYNESTHESIA - Full System Build

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SYNESTHESIA SYSTEM                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────┐     ┌─────────────────────┐                   │
│  │   OFFLINE PIPELINE  │     │   RUNTIME PLAYER    │                   │
│  │   (Python/Essentia) │────▶│   (Rust/wgpu)       │                   │
│  └─────────────────────┘     └─────────────────────┘                   │
│           │                            │                                │
│           ▼                            ▼                                │
│  ┌─────────────────────┐     ┌─────────────────────┐                   │
│  │   .synth FILE       │     │   60 FPS OUTPUT     │                   │
│  │   (Analysis + Video)│     │   (Window/VR)       │                   │
│  └─────────────────────┘     └─────────────────────┘                   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
synesthesia/
├── analyzer/           # Python - Offline music analysis
│   ├── __init__.py
│   ├── essentia_analyzer.py   # Key, chords, structure
│   ├── beat_tracker.py        # Beat/tempo detection
│   ├── emotion_mapper.py      # Music → emotion
│   └── cli.py                 # Command-line interface
│
├── generator/          # Python - Video segment generation
│   ├── __init__.py
│   ├── prompt_builder.py      # Music → prompts
│   ├── cogvideo_gen.py        # CogVideoX integration
│   ├── segment_stitcher.py    # Combine segments
│   └── cli.py
│
├── common/             # Shared formats
│   ├── synth_format.py        # .synth file spec (Python)
│   └── synth_format.rs        # .synth file spec (Rust)
│
├── synesthesia/        # Rust - Runtime player
│   ├── src/
│   │   ├── main.rs
│   │   ├── player/            # Audio + video sync
│   │   ├── shaders/           # Reactive WGSL shaders
│   │   ├── transitions/       # Beat-synced transitions
│   │   └── ui/                # egui interface
│   └── Cargo.toml
│
├── demo/               # Web demo (HTML/JS)
│   └── synesthesia-demo.html
│
└── cli/                # Unified CLI
    └── synth           # Main command-line tool
```

## Quick Start

```bash
# 1. Analyze a song (creates .synth file)
synth analyze song.mp3 --output song.synth

# 2. Generate video segments (optional, enhances visuals)
synth generate song.synth --model cogvideo

# 3. Play the visualization
synth play song.synth

# Or all in one:
synth run song.mp3
```

## Component Status

| Component | Status | Description |
|-----------|--------|-------------|
| analyzer/ | 🔨 Building | Essentia-based music analysis |
| generator/ | 📋 Planned | AI video segment generation |
| common/ | 🔨 Building | .synth file format |
| synesthesia/ | ✅ Scaffold | Rust runtime player |
| demo/ | ✅ Complete | Browser demo |
| cli/ | 📋 Planned | Unified command-line |
