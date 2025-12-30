"""
SYNESTHESIA Analyzer

Offline music analysis pipeline using Essentia for:
- Key detection
- Chord progression
- Beat/tempo tracking
- Song structure segmentation
- Emotion mapping

Usage:
    from analyzer import analyze_song
    analysis = analyze_song("song.mp3")
    analysis.save("song.synth")
"""

from .essentia_analyzer import EssentiaAnalyzer, analyze_song
from .beat_tracker import BeatTracker
from .structure_detector import StructureDetector
from .emotion_mapper import EmotionMapper
from .synth_format import SynthFile, MusicAnalysis

__version__ = "0.1.0"
__all__ = [
    "EssentiaAnalyzer",
    "analyze_song",
    "BeatTracker",
    "StructureDetector",
    "EmotionMapper",
    "SynthFile",
    "MusicAnalysis",
]
