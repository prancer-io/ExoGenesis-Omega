"""
Emotion Mapping

Maps musical features to emotional states using:
- Circumplex model (valence + arousal)
- Music psychology research
- Genre-specific adjustments
"""

import numpy as np
from dataclasses import dataclass
from typing import List, Optional, Dict
from enum import Enum
import logging

logger = logging.getLogger(__name__)


class Emotion(Enum):
    """Emotion categories based on circumplex model"""
    # High arousal, positive valence
    JOY = "joy"
    TRIUMPH = "triumph"
    EXCITEMENT = "excitement"
    EUPHORIA = "euphoria"

    # High arousal, negative valence
    ANGER = "anger"
    INTENSITY = "intensity"
    URGENCY = "urgency"
    CHAOS = "chaos"

    # Low arousal, positive valence
    PEACE = "peace"
    TENDERNESS = "tenderness"
    HOPE = "hope"
    NOSTALGIA = "nostalgia"

    # Low arousal, negative valence
    SADNESS = "sadness"
    MELANCHOLY = "melancholy"
    TENSION = "tension"
    DREAD = "dread"

    NEUTRAL = "neutral"


@dataclass
class EmotionPoint:
    """Emotion at a point in time"""
    time: float
    emotion: Emotion
    intensity: float
    valence: float   # -1 (negative) to +1 (positive)
    arousal: float   # -1 (calm) to +1 (excited)


@dataclass
class EmotionArc:
    """Complete emotional arc of the song"""
    points: List[EmotionPoint]
    dominant_emotion: Emotion
    emotional_range: float  # How much variety in emotions
    overall_valence: float
    overall_arousal: float


# Mappings from music theory to emotion dimensions
KEY_VALENCE = {
    # Major keys tend positive
    'C': 0.3, 'G': 0.35, 'D': 0.4, 'A': 0.3, 'E': 0.25, 'B': 0.2,
    'F': 0.25, 'Bb': 0.15, 'Eb': 0.1, 'Ab': 0.0, 'Db': -0.05, 'F#': 0.15,
}

MODE_VALENCE = {
    'major': 0.3,
    'minor': -0.3,
    'dorian': -0.1,
    'phrygian': -0.4,
    'lydian': 0.4,
    'mixolydian': 0.2,
    'aeolian': -0.25,
    'locrian': -0.5,
}

CHORD_TENSION = {
    # Chord type to tension level
    '': 0.0,      # Major
    'm': 0.1,     # Minor
    '7': 0.3,     # Dominant 7
    'maj7': 0.15, # Major 7
    'm7': 0.2,    # Minor 7
    'dim': 0.6,   # Diminished
    'aug': 0.5,   # Augmented
    'sus2': 0.2,  # Suspended 2
    'sus4': 0.25, # Suspended 4
    '5': 0.05,    # Power chord
}


class EmotionMapper:
    """
    Maps musical features to emotions.

    Uses the circumplex model:
    - Valence: Positive ↔ Negative (happy ↔ sad)
    - Arousal: High ↔ Low (excited ↔ calm)

    These two dimensions map to specific emotions.
    """

    def __init__(self):
        self.smoothing = 0.3  # Temporal smoothing factor

    def map(
        self,
        key: str,
        mode: str,
        tempo: float,
        energy_curve: np.ndarray,
        sections: List,
        duration: float,
        chords: Optional[List] = None,
    ) -> EmotionArc:
        """
        Map musical features to an emotional arc.

        Args:
            key: Musical key (C, D, etc.)
            mode: Scale mode (major, minor)
            tempo: Tempo in BPM
            energy_curve: Energy over time (10Hz sample rate)
            sections: List of song sections
            duration: Song duration in seconds
            chords: Optional chord progression

        Returns:
            EmotionArc with points over time
        """
        logger.info("Mapping emotions...")

        # Compute base valence from key and mode
        base_valence = self._compute_base_valence(key, mode)

        # Generate emotion points at 1Hz
        points = []
        sample_rate = 10  # Energy curve sample rate

        for i in range(int(duration)):
            time = float(i)

            # Get energy at this time
            energy_idx = min(i * sample_rate // 10, len(energy_curve) - 1)
            energy = energy_curve[energy_idx] if len(energy_curve) > 0 else 0.5

            # Get section at this time
            section = self._get_section_at(time, sections)

            # Get chord tension at this time
            chord_tension = self._get_chord_tension_at(time, chords) if chords else 0.0

            # Compute valence and arousal
            valence = self._compute_valence(
                base_valence, section, chord_tension, energy
            )
            arousal = self._compute_arousal(
                tempo, energy, section
            )

            # Map to specific emotion
            emotion = self._map_to_emotion(valence, arousal)
            intensity = (abs(valence) + abs(arousal)) / 2

            points.append(EmotionPoint(
                time=time,
                emotion=emotion,
                intensity=intensity,
                valence=valence,
                arousal=arousal,
            ))

        # Apply temporal smoothing
        points = self._smooth_emotions(points)

        # Compute summary statistics
        dominant = self._find_dominant_emotion(points)
        valences = [p.valence for p in points]
        arousals = [p.arousal for p in points]

        emotional_range = (
            np.std(valences) + np.std(arousals)
        ) / 2 if points else 0.0

        return EmotionArc(
            points=points,
            dominant_emotion=dominant,
            emotional_range=emotional_range,
            overall_valence=np.mean(valences) if valences else 0.0,
            overall_arousal=np.mean(arousals) if arousals else 0.0,
        )

    def _compute_base_valence(self, key: str, mode: str) -> float:
        """Compute base valence from key and mode"""
        key_val = KEY_VALENCE.get(key, 0.0)
        mode_val = MODE_VALENCE.get(mode.lower(), 0.0)
        return np.clip(key_val + mode_val, -1.0, 1.0)

    def _get_section_at(self, time: float, sections: List) -> Optional[any]:
        """Get the section at a given time"""
        for section in sections:
            if hasattr(section, 'start_time') and hasattr(section, 'end_time'):
                if section.start_time <= time < section.end_time:
                    return section
        return None

    def _get_chord_tension_at(self, time: float, chords: List) -> float:
        """Get chord tension at a given time"""
        for chord in chords:
            if hasattr(chord, 'time') and hasattr(chord, 'duration'):
                if chord.time <= time < chord.time + chord.duration:
                    # Parse chord type
                    chord_name = chord.chord
                    for suffix, tension in sorted(
                        CHORD_TENSION.items(), key=lambda x: -len(x[0])
                    ):
                        if chord_name.endswith(suffix):
                            return tension
        return 0.0

    def _compute_valence(
        self,
        base_valence: float,
        section: Optional[any],
        chord_tension: float,
        energy: float,
    ) -> float:
        """Compute valence at a point in time"""
        valence = base_valence

        # Section modifiers
        if section:
            section_type = getattr(section, 'section_type', None)
            if section_type:
                section_name = section_type.value if hasattr(section_type, 'value') else str(section_type)
                if 'chorus' in section_name or 'drop' in section_name:
                    valence += 0.2
                elif 'breakdown' in section_name:
                    valence -= 0.1
                elif 'bridge' in section_name:
                    valence -= 0.05

        # Chord tension pulls toward negative
        valence -= chord_tension * 0.3

        # High energy can push either direction
        if energy > 0.7:
            valence += 0.1 * np.sign(valence)

        return np.clip(valence, -1.0, 1.0)

    def _compute_arousal(
        self,
        tempo: float,
        energy: float,
        section: Optional[any],
    ) -> float:
        """Compute arousal at a point in time"""
        arousal = 0.0

        # Tempo contribution
        if tempo > 140:
            arousal += 0.4
        elif tempo > 120:
            arousal += 0.2
        elif tempo > 100:
            arousal += 0.0
        elif tempo > 80:
            arousal -= 0.2
        else:
            arousal -= 0.4

        # Energy contribution
        arousal += (energy - 0.5) * 0.8

        # Section modifiers
        if section:
            section_type = getattr(section, 'section_type', None)
            if section_type:
                section_name = section_type.value if hasattr(section_type, 'value') else str(section_type)
                if 'drop' in section_name:
                    arousal += 0.3
                elif 'chorus' in section_name:
                    arousal += 0.2
                elif 'buildup' in section_name:
                    arousal += 0.15
                elif 'breakdown' in section_name or 'intro' in section_name:
                    arousal -= 0.2
                elif 'outro' in section_name:
                    arousal -= 0.15

        return np.clip(arousal, -1.0, 1.0)

    def _map_to_emotion(self, valence: float, arousal: float) -> Emotion:
        """Map valence/arousal to specific emotion"""
        if arousal > 0.3:
            # High arousal
            if valence > 0.3:
                if arousal > 0.7:
                    return Emotion.EUPHORIA
                elif valence > 0.6:
                    return Emotion.JOY
                else:
                    return Emotion.EXCITEMENT
            elif valence < -0.3:
                if arousal > 0.7:
                    return Emotion.CHAOS
                elif valence < -0.6:
                    return Emotion.ANGER
                else:
                    return Emotion.INTENSITY
            else:
                return Emotion.URGENCY
        elif arousal < -0.3:
            # Low arousal
            if valence > 0.3:
                if valence > 0.6:
                    return Emotion.PEACE
                else:
                    return Emotion.TENDERNESS
            elif valence < -0.3:
                if valence < -0.6:
                    return Emotion.DREAD
                else:
                    return Emotion.SADNESS
            else:
                return Emotion.MELANCHOLY
        else:
            # Medium arousal
            if valence > 0.3:
                return Emotion.HOPE if valence > 0.5 else Emotion.NOSTALGIA
            elif valence < -0.3:
                return Emotion.TENSION
            else:
                return Emotion.NEUTRAL

    def _smooth_emotions(self, points: List[EmotionPoint]) -> List[EmotionPoint]:
        """Apply temporal smoothing to emotion values"""
        if len(points) < 2:
            return points

        smoothed = []
        for i, p in enumerate(points):
            if i == 0:
                smoothed.append(p)
            else:
                prev = smoothed[-1]
                new_valence = prev.valence * self.smoothing + p.valence * (1 - self.smoothing)
                new_arousal = prev.arousal * self.smoothing + p.arousal * (1 - self.smoothing)

                # Re-map emotion after smoothing
                emotion = self._map_to_emotion(new_valence, new_arousal)

                smoothed.append(EmotionPoint(
                    time=p.time,
                    emotion=emotion,
                    intensity=(abs(new_valence) + abs(new_arousal)) / 2,
                    valence=new_valence,
                    arousal=new_arousal,
                ))

        return smoothed

    def _find_dominant_emotion(self, points: List[EmotionPoint]) -> Emotion:
        """Find the most common emotion"""
        if not points:
            return Emotion.NEUTRAL

        emotion_counts: Dict[Emotion, float] = {}
        for p in points:
            emotion_counts[p.emotion] = emotion_counts.get(p.emotion, 0) + p.intensity

        return max(emotion_counts, key=emotion_counts.get)


def map_emotions(
    key: str,
    mode: str,
    tempo: float,
    energy_curve: np.ndarray,
    sections: List,
    duration: float,
    chords: Optional[List] = None,
) -> EmotionArc:
    """Convenience function for emotion mapping"""
    mapper = EmotionMapper()
    return mapper.map(key, mode, tempo, energy_curve, sections, duration, chords)
