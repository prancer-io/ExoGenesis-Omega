#!/usr/bin/env python3
"""
Full Pipeline Validation Test

Tests the complete SYNESTHESIA pipeline from synthetic audio to .synth file.
Uses mocked audio analysis to test all downstream components.
"""

import sys
import os
import tempfile
import struct
import json
from pathlib import Path

# Add analyzer to path
sys.path.insert(0, str(Path(__file__).parent.parent))

import numpy as np

print("=" * 60)
print("SYNESTHESIA FULL PIPELINE VALIDATION")
print("=" * 60)

# ─────────────────────────────────────────────────────────────────
# TEST 1: Structure Detector
# ─────────────────────────────────────────────────────────────────
print("\n[1/6] Testing Structure Detector...")

try:
    from analyzer.structure_detector import StructureDetector, Section, SectionType, ClimaxPoint

    detector = StructureDetector()

    # Create synthetic features simulating: Intro -> Verse -> Chorus -> Outro
    duration = 180.0
    fps = 10
    n_frames = int(duration * fps)

    # Energy pattern: low -> medium -> high -> medium
    energy = np.concatenate([
        np.linspace(0.2, 0.3, 300),   # Intro (0-30s)
        np.ones(450) * 0.5,           # Verse (30-75s)
        np.ones(450) * 0.85,          # Chorus (75-120s)
        np.linspace(0.85, 0.4, 300),  # Bridge (120-150s)
        np.linspace(0.4, 0.2, 300),   # Outro (150-180s)
    ])

    spectral = np.column_stack([
        energy * 2000 + 500,  # centroid
        energy * 0.8,         # rolloff
        np.abs(np.diff(energy, prepend=energy[0])) * 10,  # flux
        energy,               # rms
    ])

    beats = list(np.arange(0, duration, 0.5))  # 120 BPM

    sections, climaxes = detector.detect(
        spectral_features=spectral,
        energy_curve=energy,
        beats=beats,
        tempo=120.0,
        duration=duration,
    )

    print(f"   ✓ Detected {len(sections)} sections")
    for s in sections:
        print(f"      - {s.section_type.value}: {s.start_time:.1f}s - {s.end_time:.1f}s (energy={s.energy:.2f})")
    print(f"   ✓ Detected {len(climaxes)} climax points")

    assert len(sections) >= 2, "Should detect at least 2 sections"
    assert sections[0].start_time < 5.0, "First section should start near beginning"
    print("   ✓ Structure Detector PASSED")

except Exception as e:
    print(f"   ✗ Structure Detector FAILED: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

# ─────────────────────────────────────────────────────────────────
# TEST 2: Emotion Mapper
# ─────────────────────────────────────────────────────────────────
print("\n[2/6] Testing Emotion Mapper...")

try:
    from analyzer.emotion_mapper import EmotionMapper, Emotion

    mapper = EmotionMapper()

    # Test major key + fast tempo = high valence, high arousal
    result_happy = mapper.map(
        key="C",
        mode="major",
        tempo=140.0,
        energy_curve=np.array([0.7] * 100),
        sections=sections,
        duration=100.0,
        chords=[],
    )

    # Test minor key + slow tempo = low valence, low arousal
    result_sad = mapper.map(
        key="A",
        mode="minor",
        tempo=70.0,
        energy_curve=np.array([0.3] * 100),
        sections=[],
        duration=100.0,
        chords=[],
    )

    print(f"   ✓ Major/Fast → {result_happy.dominant_emotion.value} (V={result_happy.overall_valence:.2f}, A={result_happy.overall_arousal:.2f})")
    print(f"   ✓ Minor/Slow → {result_sad.dominant_emotion.value} (V={result_sad.overall_valence:.2f}, A={result_sad.overall_arousal:.2f})")

    assert result_happy.overall_valence > result_sad.overall_valence, "Major key should have higher valence"
    assert result_happy.overall_arousal > result_sad.overall_arousal, "Fast tempo should have higher arousal"
    print("   ✓ Emotion Mapper PASSED")

except Exception as e:
    print(f"   ✗ Emotion Mapper FAILED: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

# ─────────────────────────────────────────────────────────────────
# TEST 3: Synth Format (Create & Save)
# ─────────────────────────────────────────────────────────────────
print("\n[3/6] Testing Synth Format Creation...")

try:
    from analyzer.synth_format import (
        SynthFile, MusicAnalysis, VideoSegment, TransitionPoint
    )

    # Create full analysis data using Dict format as expected by the module
    analysis = MusicAnalysis(
        duration=180.0,
        sample_rate=44100,
        audio_hash="test_validation_12345",
        key="G",
        mode="major",
        key_confidence=0.92,
        tempo=128.0,
        tempo_confidence=0.88,
        beats=list(np.arange(0, 180, 60/128)),
        downbeats=list(np.arange(0, 180, 4 * 60/128)),
        time_signature=(4, 4),
        chords=[
            {"time": 0.0, "duration": 4.0, "chord": "G", "confidence": 0.9},
            {"time": 4.0, "duration": 4.0, "chord": "D", "confidence": 0.85},
            {"time": 8.0, "duration": 4.0, "chord": "Em", "confidence": 0.88},
            {"time": 12.0, "duration": 4.0, "chord": "C", "confidence": 0.92},
        ],
        sections=[
            {"type": "intro", "start": 0.0, "end": 16.0, "energy": 0.3, "repetition": 0, "confidence": 0.85},
            {"type": "verse", "start": 16.0, "end": 48.0, "energy": 0.5, "repetition": 0, "confidence": 0.9},
            {"type": "chorus", "start": 48.0, "end": 80.0, "energy": 0.85, "repetition": 0, "confidence": 0.92},
            {"type": "verse", "start": 80.0, "end": 112.0, "energy": 0.55, "repetition": 1, "confidence": 0.88},
            {"type": "chorus", "start": 112.0, "end": 144.0, "energy": 0.9, "repetition": 1, "confidence": 0.95},
            {"type": "outro", "start": 144.0, "end": 180.0, "energy": 0.35, "repetition": 0, "confidence": 0.8},
        ],
        climaxes=[
            {"time": 64.0, "intensity": 0.85, "type": "chorus_peak"},
            {"time": 128.0, "intensity": 0.95, "type": "final_chorus"},
        ],
        energy_curve=list(energy),
        tension_curve=list(np.sin(np.linspace(0, 4*np.pi, len(energy))) * 0.3 + 0.5),
        loudness_curve=list(energy * 0.9),
        emotion_arc=[
            {"time": 0.0, "emotion": "peace", "intensity": 0.4, "valence": 0.6, "arousal": 0.3},
            {"time": 48.0, "emotion": "joy", "intensity": 0.8, "valence": 0.85, "arousal": 0.75},
            {"time": 112.0, "emotion": "euphoria", "intensity": 0.95, "valence": 0.9, "arousal": 0.9},
            {"time": 150.0, "emotion": "peace", "intensity": 0.5, "valence": 0.65, "arousal": 0.35},
        ],
    )

    # Create video segments
    video_segments = [
        VideoSegment(segment_id=0, start_time=0.0, end_time=48.0, video_path="segments/seg_000.mp4",
                    mood="peace", clarity_level=0.3, base_hue=180.0, saturation=0.5, brightness=0.5, motion_speed=0.8),
        VideoSegment(segment_id=1, start_time=48.0, end_time=112.0, video_path="segments/seg_001.mp4",
                    mood="joy", clarity_level=0.7, base_hue=45.0, saturation=0.8, brightness=0.7, motion_speed=1.2),
        VideoSegment(segment_id=2, start_time=112.0, end_time=180.0, video_path="segments/seg_002.mp4",
                    mood="euphoria", clarity_level=0.9, base_hue=300.0, saturation=0.9, brightness=0.8, motion_speed=1.5),
    ]

    # Create transitions
    transitions = [
        TransitionPoint(time=48.0, from_segment=0, to_segment=1, transition_type="crossfade", duration=0.5),
        TransitionPoint(time=112.0, from_segment=1, to_segment=2, transition_type="flash_reveal", duration=0.3),
    ]

    synth = SynthFile(
        analysis=analysis,
        video_segments=video_segments,
        transitions=transitions,
        style_name="neon",
    )

    # Save to temp file
    with tempfile.NamedTemporaryFile(suffix='.synth', delete=False) as f:
        temp_path = f.name

    synth.save(temp_path)
    file_size = os.path.getsize(temp_path)

    print(f"   ✓ Created .synth file: {file_size} bytes")
    print(f"   ✓ Contains {len(analysis.sections)} sections")
    print(f"   ✓ Contains {len(analysis.chords)} chords")
    print(f"   ✓ Contains {len(video_segments)} video segments")
    print(f"   ✓ Contains {len(transitions)} transitions")
    print(f"   ✓ Style: {synth.style_name}")

    # Verify magic bytes
    with open(temp_path, 'rb') as f:
        magic = f.read(8)
        assert magic == b'SYNTH\x00\x01\x00', f"Invalid magic: {magic}"

    print("   ✓ Synth Format Creation PASSED")

except Exception as e:
    print(f"   ✗ Synth Format Creation FAILED: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

# ─────────────────────────────────────────────────────────────────
# TEST 4: Synth Format (Load & Verify)
# ─────────────────────────────────────────────────────────────────
print("\n[4/6] Testing Synth Format Loading...")

try:
    loaded = SynthFile.load(temp_path)

    assert abs(loaded.analysis.duration - 180.0) < 0.1, "Duration mismatch"
    assert loaded.analysis.key == "G", "Key mismatch"
    assert loaded.analysis.mode == "major", "Mode mismatch"
    assert abs(loaded.analysis.tempo - 128.0) < 0.1, "Tempo mismatch"
    assert len(loaded.analysis.sections) == 6, f"Section count mismatch: {len(loaded.analysis.sections)}"
    assert len(loaded.analysis.chords) == 4, "Chord count mismatch"
    assert len(loaded.analysis.emotion_arc) == 4, "Emotion arc mismatch"
    assert loaded.style_name == "neon", "Style mismatch"

    print(f"   ✓ Duration: {loaded.analysis.duration}s")
    print(f"   ✓ Key: {loaded.analysis.key} {loaded.analysis.mode}")
    print(f"   ✓ Tempo: {loaded.analysis.tempo} BPM")
    print(f"   ✓ Sections: {len(loaded.analysis.sections)}")
    print(f"   ✓ Chords: {len(loaded.analysis.chords)}")
    print(f"   ✓ Emotion points: {len(loaded.analysis.emotion_arc)}")

    # Verify section data (sections are dicts)
    first_section = loaded.analysis.sections[0]
    print(f"   ✓ First section: {first_section['type']} ({first_section['start']}s - {first_section['end']}s)")

    print("   ✓ Synth Format Loading PASSED")

except Exception as e:
    print(f"   ✗ Synth Format Loading FAILED: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

# ─────────────────────────────────────────────────────────────────
# TEST 5: Video Generator Configuration
# ─────────────────────────────────────────────────────────────────
print("\n[5/6] Testing Video Generator...")

try:
    from analyzer.video_generator import VideoGenerator, GenerationConfig, GenerationMode

    config = GenerationConfig(
        mode=GenerationMode.PROCEDURAL,
        output_dir=tempfile.mkdtemp(),
        width=640,
        height=360,
        fps=24,
    )

    generator = VideoGenerator(config)

    # Test emotion to hue mapping
    joy_hue = generator._emotion_to_hue("joy")
    sadness_hue = generator._emotion_to_hue("sadness")
    euphoria_hue = generator._emotion_to_hue("euphoria")

    print(f"   ✓ Joy hue: {joy_hue}° (warm yellow-orange)")
    print(f"   ✓ Sadness hue: {sadness_hue}° (cool blue)")
    print(f"   ✓ Euphoria hue: {euphoria_hue}° (magenta)")

    assert 30 < joy_hue < 60, "Joy should be warm"
    assert 200 < sadness_hue < 260, "Sadness should be cool"

    # Test clarity calculation
    intro_clarity = generator._calculate_clarity("intro", 0)
    chorus_clarity = generator._calculate_clarity("chorus", 0)
    chorus2_clarity = generator._calculate_clarity("chorus", 2)

    print(f"   ✓ Intro clarity: {intro_clarity:.2f}")
    print(f"   ✓ Chorus clarity: {chorus_clarity:.2f}")
    print(f"   ✓ Chorus (2nd time) clarity: {chorus2_clarity:.2f}")

    assert intro_clarity < chorus_clarity, "Chorus should have higher clarity than intro"
    assert chorus2_clarity > chorus_clarity, "Repeated sections should have higher clarity"

    # Test prompt generation
    prompt = generator._generate_prompt(
        section_type="chorus",
        emotion="euphoria",
        key="G",
        mode="major",
        energy=0.9,
        clarity=0.8,
    )

    print(f"   ✓ Generated prompt: '{prompt[:60]}...'")
    assert "euphoria" in prompt.lower() or "explosive" in prompt.lower(), "Prompt should reflect emotion/energy"

    print("   ✓ Video Generator PASSED")

except Exception as e:
    print(f"   ✗ Video Generator FAILED: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

# ─────────────────────────────────────────────────────────────────
# TEST 6: JSON Export
# ─────────────────────────────────────────────────────────────────
print("\n[6/6] Testing JSON Export...")

try:
    json_str = loaded.to_json()
    json_data = json.loads(json_str)

    assert "analysis" in json_data
    assert "style_name" in json_data
    assert json_data["style_name"] == "neon"
    assert json_data["analysis"]["key"] == "G"
    assert json_data["analysis"]["mode"] == "major"
    assert len(json_data["analysis"]["sections"]) == 6

    print(f"   ✓ JSON export size: {len(json_str)} bytes")
    print(f"   ✓ Contains all required fields")
    print("   ✓ JSON Export PASSED")

except Exception as e:
    print(f"   ✗ JSON Export FAILED: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

# ─────────────────────────────────────────────────────────────────
# CLEANUP
# ─────────────────────────────────────────────────────────────────
try:
    os.unlink(temp_path)
except:
    pass

# ─────────────────────────────────────────────────────────────────
# SUMMARY
# ─────────────────────────────────────────────────────────────────
print("\n" + "=" * 60)
print("✅ ALL PIPELINE TESTS PASSED!")
print("=" * 60)
print("""
Pipeline Components Validated:
  ✓ Structure Detector - Section & climax detection
  ✓ Emotion Mapper - Valence/arousal from musical features
  ✓ Synth Format - Binary file creation with msgpack
  ✓ Synth Loading - Roundtrip save/load verification
  ✓ Video Generator - Procedural generation config
  ✓ JSON Export - Inspection/debugging output

The pipeline is ready for production use!
""")
