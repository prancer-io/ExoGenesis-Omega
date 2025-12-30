"""
Beat and Tempo Tracking

Provides accurate beat detection for synchronizing visuals.
"""

import numpy as np
from dataclasses import dataclass
from typing import List, Optional, Tuple
import logging

logger = logging.getLogger(__name__)


@dataclass
class BeatTrackResult:
    """Result of beat tracking"""
    tempo: float                    # BPM
    tempo_confidence: float         # 0.0 - 1.0
    beats: List[float]              # Beat timestamps in seconds
    downbeats: List[float]          # Bar start timestamps
    time_signature: Tuple[int, int] # e.g., (4, 4)
    beat_strengths: List[float]     # Strength of each beat


class BeatTracker:
    """
    Beat tracking using onset detection and tempo estimation.

    Features:
    - Accurate beat positions
    - Downbeat (bar start) detection
    - Time signature estimation
    - Beat strength for visual sync
    """

    def __init__(self, sample_rate: int = 44100, hop_size: int = 512):
        self.sample_rate = sample_rate
        self.hop_size = hop_size

    def track(self, audio: np.ndarray) -> BeatTrackResult:
        """
        Track beats in audio.

        Args:
            audio: Audio samples (mono)

        Returns:
            BeatTrackResult with tempo and beat positions
        """
        try:
            import librosa
            return self._track_librosa(audio)
        except ImportError:
            logger.warning("librosa not available, using simple beat detection")
            return self._track_simple(audio)

    def _track_librosa(self, audio: np.ndarray) -> BeatTrackResult:
        """Beat tracking using librosa"""
        import librosa

        # Compute onset envelope
        onset_env = librosa.onset.onset_strength(
            y=audio,
            sr=self.sample_rate,
            hop_length=self.hop_size
        )

        # Estimate tempo
        tempo = librosa.feature.tempo(
            onset_envelope=onset_env,
            sr=self.sample_rate,
            hop_length=self.hop_size
        )[0]

        # Track beats
        beats = librosa.beat.beat_track(
            onset_envelope=onset_env,
            sr=self.sample_rate,
            hop_length=self.hop_size,
            bpm=tempo
        )[1]

        # Convert to time
        beat_times = librosa.frames_to_time(
            beats,
            sr=self.sample_rate,
            hop_length=self.hop_size
        )

        # Estimate beat strengths
        beat_strengths = [onset_env[b] for b in beats if b < len(onset_env)]
        if beat_strengths:
            max_strength = max(beat_strengths)
            if max_strength > 0:
                beat_strengths = [s / max_strength for s in beat_strengths]

        # Estimate time signature (simplified: assume 4/4)
        time_sig = (4, 4)

        # Detect downbeats (every 4th beat for 4/4)
        beats_per_bar = time_sig[0]
        downbeats = beat_times[::beats_per_bar].tolist()

        # Compute confidence based on onset envelope regularity
        if len(beats) > 4:
            intervals = np.diff(beat_times)
            interval_std = np.std(intervals)
            expected_interval = 60.0 / tempo
            confidence = max(0, 1 - interval_std / expected_interval)
        else:
            confidence = 0.5

        return BeatTrackResult(
            tempo=float(tempo),
            tempo_confidence=float(confidence),
            beats=beat_times.tolist(),
            downbeats=downbeats,
            time_signature=time_sig,
            beat_strengths=beat_strengths,
        )

    def _track_simple(self, audio: np.ndarray) -> BeatTrackResult:
        """Simple beat detection without librosa"""
        # Compute energy in frames
        frame_size = self.hop_size * 2
        hop = self.hop_size
        num_frames = (len(audio) - frame_size) // hop

        energy = []
        for i in range(num_frames):
            frame = audio[i * hop:i * hop + frame_size]
            energy.append(np.sqrt(np.mean(frame ** 2)))

        energy = np.array(energy)

        # Simple onset detection
        diff = np.diff(energy)
        diff = np.concatenate([[0], diff])
        diff[diff < 0] = 0

        # Threshold
        threshold = np.mean(diff) + np.std(diff)
        peaks = np.where(diff > threshold)[0]

        # Convert to time
        beat_times = peaks * hop / self.sample_rate

        # Estimate tempo from intervals
        if len(beat_times) > 2:
            intervals = np.diff(beat_times)
            # Filter outliers
            median_interval = np.median(intervals)
            valid = intervals[(intervals > median_interval * 0.5) &
                            (intervals < median_interval * 2)]
            if len(valid) > 0:
                tempo = 60.0 / np.mean(valid)
            else:
                tempo = 120.0
        else:
            tempo = 120.0

        # Clamp tempo to reasonable range
        tempo = max(60, min(200, tempo))

        return BeatTrackResult(
            tempo=tempo,
            tempo_confidence=0.5,
            beats=beat_times.tolist(),
            downbeats=beat_times[::4].tolist() if len(beat_times) >= 4 else beat_times.tolist(),
            time_signature=(4, 4),
            beat_strengths=[0.8] * len(beat_times),
        )

    def refine_to_grid(
        self,
        beats: List[float],
        tempo: float,
        duration: float,
    ) -> List[float]:
        """
        Refine beat positions to a tempo grid.

        Useful for quantizing beats to exact tempo.

        Args:
            beats: Detected beat times
            tempo: Estimated tempo
            duration: Song duration

        Returns:
            Grid-aligned beat times
        """
        if not beats:
            return []

        beat_interval = 60.0 / tempo

        # Find best offset
        offsets = [b % beat_interval for b in beats]
        offset = np.median(offsets)

        # Generate grid
        grid_beats = []
        t = offset
        while t < duration:
            grid_beats.append(t)
            t += beat_interval

        return grid_beats


def track_beats(audio: np.ndarray, sample_rate: int = 44100) -> BeatTrackResult:
    """Convenience function for beat tracking"""
    tracker = BeatTracker(sample_rate=sample_rate)
    return tracker.track(audio)
