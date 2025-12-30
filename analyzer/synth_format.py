"""
.synth File Format

Binary format for storing music analysis + pre-rendered video.
Designed to be efficient to load and easy to parse in Rust.

Format structure:
  - Header (magic bytes, version, flags)
  - Music analysis (msgpack encoded)
  - Video segments (optional, compressed)
  - Shader parameters (per-segment)
"""

import struct
import msgpack
import numpy as np
from dataclasses import dataclass, field, asdict
from typing import List, Optional, Dict, Any, BinaryIO
from pathlib import Path
import hashlib
import logging
import json

logger = logging.getLogger(__name__)

# Magic bytes: "SYNTH" + version
MAGIC = b'SYNTH\x00\x01\x00'  # SYNTH v1.0
FORMAT_VERSION = (1, 0)


@dataclass
class MusicAnalysis:
    """Complete music analysis for a song"""
    # Basic info
    duration: float
    sample_rate: int
    audio_hash: str  # SHA256 of original audio

    # Key and harmony
    key: str
    mode: str
    key_confidence: float

    # Tempo and rhythm
    tempo: float
    tempo_confidence: float
    beats: List[float]
    downbeats: List[float]
    time_signature: tuple

    # Chords
    chords: List[Dict[str, Any]]  # [{time, duration, chord, confidence}]

    # Structure
    sections: List[Dict[str, Any]]  # [{type, start, end, energy, repetition}]
    climaxes: List[Dict[str, Any]]  # [{time, intensity, type}]

    # Curves (sampled at 10Hz)
    energy_curve: List[float]
    tension_curve: List[float]
    loudness_curve: List[float]

    # Emotion arc
    emotion_arc: List[Dict[str, Any]]  # [{time, emotion, intensity, valence, arousal}]

    # Spectral features (optional, for real-time enhancement)
    spectral_centroid: Optional[List[float]] = None
    spectral_flux: Optional[List[float]] = None


@dataclass
class VideoSegment:
    """A pre-rendered video segment"""
    segment_id: int
    start_time: float
    end_time: float
    video_path: str  # Path to video file (relative)
    mood: str
    clarity_level: float

    # Shader parameters for this segment
    base_hue: float
    saturation: float
    brightness: float
    motion_speed: float


@dataclass
class TransitionPoint:
    """A transition between segments"""
    time: float
    from_segment: int
    to_segment: int
    transition_type: str  # "cut", "crossfade", "beat_wipe", "morph"
    duration: float


@dataclass
class ShaderCurves:
    """Animation curves for shader parameters (10Hz)"""
    times: List[float]
    bloom_intensity: List[float]
    chromatic_amount: List[float]
    vignette_strength: List[float]
    grain_amount: List[float]
    color_shift: List[float]


@dataclass
class SynthFile:
    """Complete .synth file structure"""
    # Header
    version: tuple = FORMAT_VERSION
    created_at: str = ""

    # Core analysis
    analysis: MusicAnalysis = None

    # Video (optional)
    video_segments: List[VideoSegment] = field(default_factory=list)
    transitions: List[TransitionPoint] = field(default_factory=list)

    # Shader parameters
    shader_curves: Optional[ShaderCurves] = None

    # Style
    style_name: str = "default"
    style_params: Dict[str, Any] = field(default_factory=dict)

    def save(self, path: str) -> None:
        """Save to .synth file"""
        path = Path(path)
        logger.info(f"Saving .synth file: {path}")

        with open(path, 'wb') as f:
            self._write(f)

        logger.info(f"Saved {path.stat().st_size / 1024:.1f} KB")

    def _write(self, f: BinaryIO) -> None:
        """Write binary format"""
        # Magic header
        f.write(MAGIC)

        # Flags (32-bit)
        flags = 0
        if self.video_segments:
            flags |= 0x01  # Has video
        if self.shader_curves:
            flags |= 0x02  # Has shader curves
        f.write(struct.pack('<I', flags))

        # Serialize analysis to msgpack
        analysis_data = self._serialize_analysis()
        analysis_bytes = msgpack.packb(analysis_data, use_bin_type=True)

        # Write analysis section
        f.write(struct.pack('<I', len(analysis_bytes)))
        f.write(analysis_bytes)

        # Write video segment metadata (if present)
        if self.video_segments:
            video_data = [asdict(seg) for seg in self.video_segments]
            video_bytes = msgpack.packb(video_data, use_bin_type=True)
            f.write(struct.pack('<I', len(video_bytes)))
            f.write(video_bytes)

            # Transitions
            trans_data = [asdict(t) for t in self.transitions]
            trans_bytes = msgpack.packb(trans_data, use_bin_type=True)
            f.write(struct.pack('<I', len(trans_bytes)))
            f.write(trans_bytes)

        # Write shader curves (if present)
        if self.shader_curves:
            shader_data = asdict(self.shader_curves)
            shader_bytes = msgpack.packb(shader_data, use_bin_type=True)
            f.write(struct.pack('<I', len(shader_bytes)))
            f.write(shader_bytes)

        # Style info
        style_data = {
            'name': self.style_name,
            'params': self.style_params,
        }
        style_bytes = msgpack.packb(style_data, use_bin_type=True)
        f.write(struct.pack('<I', len(style_bytes)))
        f.write(style_bytes)

    def _serialize_analysis(self) -> Dict:
        """Convert analysis to serializable dict"""
        if self.analysis is None:
            return {}

        return {
            'duration': self.analysis.duration,
            'sample_rate': self.analysis.sample_rate,
            'audio_hash': self.analysis.audio_hash,
            'key': self.analysis.key,
            'mode': self.analysis.mode,
            'key_confidence': self.analysis.key_confidence,
            'tempo': self.analysis.tempo,
            'tempo_confidence': self.analysis.tempo_confidence,
            'beats': self.analysis.beats,
            'downbeats': self.analysis.downbeats,
            'time_signature': list(self.analysis.time_signature),
            'chords': self.analysis.chords,
            'sections': self.analysis.sections,
            'climaxes': self.analysis.climaxes,
            'energy_curve': self.analysis.energy_curve,
            'tension_curve': self.analysis.tension_curve,
            'loudness_curve': self.analysis.loudness_curve,
            'emotion_arc': self.analysis.emotion_arc,
            'spectral_centroid': self.analysis.spectral_centroid,
            'spectral_flux': self.analysis.spectral_flux,
        }

    @classmethod
    def load(cls, path: str) -> 'SynthFile':
        """Load from .synth file"""
        path = Path(path)
        logger.info(f"Loading .synth file: {path}")

        with open(path, 'rb') as f:
            return cls._read(f)

    @classmethod
    def _read(cls, f: BinaryIO) -> 'SynthFile':
        """Read binary format"""
        # Check magic
        magic = f.read(8)
        if magic != MAGIC:
            raise ValueError(f"Invalid .synth file (bad magic: {magic!r})")

        # Read flags
        flags = struct.unpack('<I', f.read(4))[0]
        has_video = bool(flags & 0x01)
        has_shader = bool(flags & 0x02)

        # Read analysis
        analysis_size = struct.unpack('<I', f.read(4))[0]
        analysis_bytes = f.read(analysis_size)
        analysis_data = msgpack.unpackb(analysis_bytes, raw=False)

        # Convert to MusicAnalysis
        analysis = MusicAnalysis(
            duration=analysis_data['duration'],
            sample_rate=analysis_data['sample_rate'],
            audio_hash=analysis_data['audio_hash'],
            key=analysis_data['key'],
            mode=analysis_data['mode'],
            key_confidence=analysis_data['key_confidence'],
            tempo=analysis_data['tempo'],
            tempo_confidence=analysis_data['tempo_confidence'],
            beats=analysis_data['beats'],
            downbeats=analysis_data['downbeats'],
            time_signature=tuple(analysis_data['time_signature']),
            chords=analysis_data['chords'],
            sections=analysis_data['sections'],
            climaxes=analysis_data['climaxes'],
            energy_curve=analysis_data['energy_curve'],
            tension_curve=analysis_data['tension_curve'],
            loudness_curve=analysis_data['loudness_curve'],
            emotion_arc=analysis_data['emotion_arc'],
            spectral_centroid=analysis_data.get('spectral_centroid'),
            spectral_flux=analysis_data.get('spectral_flux'),
        )

        result = cls(analysis=analysis)

        # Read video segments (if present)
        if has_video:
            video_size = struct.unpack('<I', f.read(4))[0]
            video_bytes = f.read(video_size)
            video_data = msgpack.unpackb(video_bytes, raw=False)
            result.video_segments = [VideoSegment(**seg) for seg in video_data]

            trans_size = struct.unpack('<I', f.read(4))[0]
            trans_bytes = f.read(trans_size)
            trans_data = msgpack.unpackb(trans_bytes, raw=False)
            result.transitions = [TransitionPoint(**t) for t in trans_data]

        # Read shader curves (if present)
        if has_shader:
            shader_size = struct.unpack('<I', f.read(4))[0]
            shader_bytes = f.read(shader_size)
            shader_data = msgpack.unpackb(shader_bytes, raw=False)
            result.shader_curves = ShaderCurves(**shader_data)

        # Read style
        style_size = struct.unpack('<I', f.read(4))[0]
        style_bytes = f.read(style_size)
        style_data = msgpack.unpackb(style_bytes, raw=False)
        result.style_name = style_data.get('name', 'default')
        result.style_params = style_data.get('params', {})

        return result

    def to_json(self) -> str:
        """Export as JSON for debugging/inspection"""
        data = {
            'version': self.version,
            'created_at': self.created_at,
            'analysis': self._serialize_analysis(),
            'video_segments': [asdict(seg) for seg in self.video_segments],
            'transitions': [asdict(t) for t in self.transitions],
            'shader_curves': asdict(self.shader_curves) if self.shader_curves else None,
            'style_name': self.style_name,
            'style_params': self.style_params,
        }
        return json.dumps(data, indent=2)


def create_synth_file(
    analysis_result,  # AnalysisResult from essentia_analyzer
    structure_result: tuple,  # (sections, climaxes) from structure_detector
    emotion_arc,  # EmotionArc from emotion_mapper
    audio_path: str,
) -> SynthFile:
    """
    Create a SynthFile from analysis results.

    Args:
        analysis_result: Result from EssentiaAnalyzer.analyze()
        structure_result: (sections, climaxes) from StructureDetector.detect()
        emotion_arc: EmotionArc from EmotionMapper.map()
        audio_path: Path to original audio (for hash)

    Returns:
        SynthFile ready to save
    """
    from datetime import datetime

    # Compute audio hash
    with open(audio_path, 'rb') as f:
        audio_hash = hashlib.sha256(f.read()).hexdigest()

    sections, climaxes = structure_result

    # Convert sections to dicts
    section_dicts = []
    for s in sections:
        section_dicts.append({
            'type': s.section_type.value if hasattr(s.section_type, 'value') else str(s.section_type),
            'start': s.start_time,
            'end': s.end_time,
            'energy': s.energy,
            'repetition': s.repetition,
            'confidence': s.confidence,
        })

    # Convert climaxes to dicts
    climax_dicts = []
    for c in climaxes:
        climax_dicts.append({
            'time': c.time,
            'intensity': c.intensity,
            'type': c.type,
        })

    # Convert chords to dicts
    chord_dicts = []
    for c in analysis_result.chords:
        chord_dicts.append({
            'time': c.time,
            'duration': c.duration,
            'chord': c.chord,
            'confidence': c.confidence,
        })

    # Convert emotion arc to dicts
    emotion_dicts = []
    for e in emotion_arc.points:
        emotion_dicts.append({
            'time': e.time,
            'emotion': e.emotion.value if hasattr(e.emotion, 'value') else str(e.emotion),
            'intensity': e.intensity,
            'valence': e.valence,
            'arousal': e.arousal,
        })

    # Build MusicAnalysis
    analysis = MusicAnalysis(
        duration=analysis_result.duration,
        sample_rate=analysis_result.sample_rate,
        audio_hash=audio_hash,
        key=analysis_result.key.key,
        mode=analysis_result.key.scale,
        key_confidence=analysis_result.key.confidence,
        tempo=analysis_result.beats.tempo,
        tempo_confidence=analysis_result.beats.tempo_confidence,
        beats=analysis_result.beats.beats,
        downbeats=analysis_result.beats.downbeats,
        time_signature=analysis_result.beats.time_signature,
        chords=chord_dicts,
        sections=section_dicts,
        climaxes=climax_dicts,
        energy_curve=analysis_result.energy_curve.tolist(),
        tension_curve=analysis_result.spectral.flux.tolist(),
        loudness_curve=analysis_result.loudness_curve.tolist(),
        emotion_arc=emotion_dicts,
        spectral_centroid=analysis_result.spectral.centroid.tolist(),
        spectral_flux=analysis_result.spectral.flux.tolist(),
    )

    # Create shader curves from analysis
    shader_curves = ShaderCurves(
        times=analysis_result.spectral.times.tolist(),
        bloom_intensity=[e * 0.5 for e in analysis_result.energy_curve.tolist()],
        chromatic_amount=[e * 0.02 for e in analysis_result.energy_curve.tolist()],
        vignette_strength=[0.3] * len(analysis_result.spectral.times),
        grain_amount=[max(0, 0.1 - e * 0.1) for e in analysis_result.energy_curve.tolist()],
        color_shift=[0.0] * len(analysis_result.spectral.times),
    )

    return SynthFile(
        created_at=datetime.now().isoformat(),
        analysis=analysis,
        shader_curves=shader_curves,
    )
