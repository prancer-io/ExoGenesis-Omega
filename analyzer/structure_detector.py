"""
Song Structure Detection

Segments songs into sections:
- Intro, Verse, Chorus, Bridge, Drop, Outro
- Uses self-similarity matrix and clustering
"""

import numpy as np
from dataclasses import dataclass
from typing import List, Optional
from enum import Enum
import logging

logger = logging.getLogger(__name__)


class SectionType(Enum):
    """Types of song sections"""
    INTRO = "intro"
    VERSE = "verse"
    PRE_CHORUS = "pre_chorus"
    CHORUS = "chorus"
    BRIDGE = "bridge"
    BREAKDOWN = "breakdown"
    BUILDUP = "buildup"
    DROP = "drop"
    OUTRO = "outro"
    INSTRUMENTAL = "instrumental"
    UNKNOWN = "unknown"


@dataclass
class Section:
    """A section of the song"""
    section_type: SectionType
    start_time: float
    end_time: float
    energy: float
    repetition: int  # 1st occurrence, 2nd, etc.
    confidence: float

    @property
    def duration(self) -> float:
        return self.end_time - self.start_time


@dataclass
class ClimaxPoint:
    """A peak/climax moment in the song"""
    time: float
    intensity: float  # 0.0 - 1.0
    type: str  # "energy", "drop", "emotional", "final"


class StructureDetector:
    """
    Detects song structure using audio features.

    Uses:
    - Self-similarity matrix (SSM) for finding repetitions
    - Novelty curve for finding boundaries
    - Energy profile for classifying sections
    """

    def __init__(self, sample_rate: int = 10):
        """
        Args:
            sample_rate: Sample rate of input features (default 10Hz)
        """
        self.sample_rate = sample_rate

    def detect(
        self,
        spectral_features: np.ndarray,
        energy_curve: np.ndarray,
        beats: List[float],
        tempo: float,
        duration: float,
    ) -> tuple[List[Section], List[ClimaxPoint]]:
        """
        Detect song structure.

        Args:
            spectral_features: Combined spectral features (n_frames, n_features)
            energy_curve: Energy over time (n_frames,)
            beats: Beat timestamps
            tempo: Tempo in BPM
            duration: Song duration in seconds

        Returns:
            Tuple of (sections, climax_points)
        """
        logger.info("Detecting song structure...")

        # Compute self-similarity matrix
        ssm = self._compute_ssm(spectral_features)

        # Find segment boundaries using novelty
        boundaries = self._detect_boundaries(ssm, beats, tempo, duration)

        # Classify each segment
        sections = self._classify_segments(
            boundaries, energy_curve, duration, tempo
        )

        # Find climax points
        climaxes = self._detect_climaxes(energy_curve, sections, duration)

        logger.info(f"Found {len(sections)} sections, {len(climaxes)} climax points")

        return sections, climaxes

    def _compute_ssm(self, features: np.ndarray) -> np.ndarray:
        """Compute self-similarity matrix using cosine similarity"""
        # Normalize features
        norms = np.linalg.norm(features, axis=1, keepdims=True)
        norms[norms == 0] = 1
        normalized = features / norms

        # Compute cosine similarity
        ssm = np.dot(normalized, normalized.T)

        return ssm

    def _detect_boundaries(
        self,
        ssm: np.ndarray,
        beats: List[float],
        tempo: float,
        duration: float,
    ) -> List[float]:
        """Detect segment boundaries using novelty curve"""
        n = ssm.shape[0]

        # Compute novelty curve (checkerboard kernel convolution)
        kernel_size = min(32, n // 4)
        novelty = np.zeros(n)

        for i in range(kernel_size, n - kernel_size):
            # Checkerboard kernel
            tl = np.mean(ssm[i - kernel_size:i, i - kernel_size:i])
            br = np.mean(ssm[i:i + kernel_size, i:i + kernel_size])
            tr = np.mean(ssm[i - kernel_size:i, i:i + kernel_size])
            bl = np.mean(ssm[i:i + kernel_size, i - kernel_size:i])
            novelty[i] = (tl + br) - (tr + bl)

        # Peak picking on novelty curve
        # Prefer boundaries on beats
        beat_frames = [int(b * self.sample_rate) for b in beats]

        # Find local maxima
        boundaries = [0.0]  # Start
        min_segment = int(4 * self.sample_rate)  # Minimum 4 seconds

        threshold = np.mean(novelty) + np.std(novelty) * 0.5

        i = min_segment
        while i < n - min_segment:
            if novelty[i] > threshold:
                # Find nearest beat
                if beat_frames:
                    nearest_beat = min(beat_frames, key=lambda b: abs(b - i))
                    if abs(nearest_beat - i) < self.sample_rate:  # Within 1 second
                        i = nearest_beat

                boundaries.append(i / self.sample_rate)
                i += min_segment
            else:
                i += 1

        boundaries.append(duration)  # End

        return sorted(set(boundaries))

    def _classify_segments(
        self,
        boundaries: List[float],
        energy_curve: np.ndarray,
        duration: float,
        tempo: float,
    ) -> List[Section]:
        """Classify each segment by its characteristics"""
        sections = []
        section_counts = {}  # Track repetitions

        for i in range(len(boundaries) - 1):
            start = boundaries[i]
            end = boundaries[i + 1]

            # Get energy for this segment
            start_idx = int(start * self.sample_rate)
            end_idx = int(end * self.sample_rate)
            end_idx = min(end_idx, len(energy_curve))

            if start_idx >= end_idx:
                continue

            segment_energy = np.mean(energy_curve[start_idx:end_idx])
            energy_var = np.var(energy_curve[start_idx:end_idx])

            # Classify based on position and energy
            section_type = self._classify_segment(
                start, end, duration, segment_energy, energy_var, tempo
            )

            # Track repetitions
            type_key = section_type.value
            section_counts[type_key] = section_counts.get(type_key, 0) + 1

            sections.append(Section(
                section_type=section_type,
                start_time=start,
                end_time=end,
                energy=segment_energy,
                repetition=section_counts[type_key],
                confidence=0.8,
            ))

        return sections

    def _classify_segment(
        self,
        start: float,
        end: float,
        duration: float,
        energy: float,
        energy_var: float,
        tempo: float,
    ) -> SectionType:
        """Classify a single segment based on features"""
        segment_duration = end - start
        position = start / duration  # 0.0 - 1.0

        # Normalize energy (assume 0-1 range)
        energy = min(max(energy, 0), 1)

        # Position-based heuristics
        if position < 0.1 and energy < 0.4:
            return SectionType.INTRO

        if position > 0.85 and energy < 0.5:
            return SectionType.OUTRO

        # Energy-based classification
        if energy > 0.8:
            # High energy
            if tempo > 130 and energy_var > 0.05:
                return SectionType.DROP
            return SectionType.CHORUS

        if energy > 0.6:
            # Medium-high energy
            if energy_var > 0.1:
                return SectionType.BUILDUP
            return SectionType.CHORUS

        if energy > 0.4:
            # Medium energy
            if segment_duration > 20:
                return SectionType.VERSE
            return SectionType.PRE_CHORUS

        if energy > 0.2:
            # Low-medium energy
            if energy_var < 0.02:
                return SectionType.BREAKDOWN
            return SectionType.VERSE

        # Low energy
        if position > 0.4 and position < 0.7:
            return SectionType.BRIDGE

        return SectionType.VERSE

    def _detect_climaxes(
        self,
        energy_curve: np.ndarray,
        sections: List[Section],
        duration: float,
    ) -> List[ClimaxPoint]:
        """Detect climax/peak moments"""
        climaxes = []

        # Find global energy peaks
        smoothed = np.convolve(energy_curve, np.ones(20) / 20, mode='same')

        # Peak detection
        peaks = []
        for i in range(1, len(smoothed) - 1):
            if smoothed[i] > smoothed[i - 1] and smoothed[i] > smoothed[i + 1]:
                if smoothed[i] > np.mean(smoothed) + np.std(smoothed):
                    peaks.append(i)

        # Classify peaks
        for peak in peaks:
            time = peak / self.sample_rate
            intensity = smoothed[peak]

            # Find what section this is in
            peak_section = None
            for section in sections:
                if section.start_time <= time <= section.end_time:
                    peak_section = section
                    break

            # Classify climax type
            if peak_section:
                if peak_section.section_type == SectionType.DROP:
                    climax_type = "drop"
                elif peak_section.section_type == SectionType.CHORUS:
                    climax_type = "emotional"
                else:
                    climax_type = "energy"
            else:
                climax_type = "energy"

            # Is this the final climax?
            if time > duration * 0.7 and not any(c.type == "final" for c in climaxes):
                if intensity > 0.8 * max(smoothed):
                    climax_type = "final"

            climaxes.append(ClimaxPoint(
                time=time,
                intensity=min(intensity, 1.0),
                type=climax_type,
            ))

        return climaxes


def detect_structure(
    spectral_features: np.ndarray,
    energy_curve: np.ndarray,
    beats: List[float],
    tempo: float,
    duration: float,
) -> tuple[List[Section], List[ClimaxPoint]]:
    """
    Convenience function for structure detection.
    """
    detector = StructureDetector()
    return detector.detect(
        spectral_features, energy_curve, beats, tempo, duration
    )
