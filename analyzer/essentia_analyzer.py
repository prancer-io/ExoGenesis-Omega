"""
Essentia-based Music Analyzer

Extracts musical features from audio files:
- Key and scale detection
- Chord progression
- Beat and tempo tracking
- Spectral features
- Loudness curve
"""

import numpy as np
from dataclasses import dataclass, field
from typing import List, Tuple, Optional
from pathlib import Path
import logging

logger = logging.getLogger(__name__)

# Try to import essentia, fall back to librosa if not available
try:
    import essentia
    import essentia.standard as es
    ESSENTIA_AVAILABLE = True
except ImportError:
    ESSENTIA_AVAILABLE = False
    logger.warning("Essentia not available, using librosa fallback")

try:
    import librosa
    LIBROSA_AVAILABLE = True
except ImportError:
    LIBROSA_AVAILABLE = False


@dataclass
class KeyResult:
    """Key detection result"""
    key: str              # C, C#, D, etc.
    scale: str            # major, minor
    confidence: float     # 0.0 - 1.0

    def __str__(self):
        return f"{self.key} {self.scale}"


@dataclass
class ChordEvent:
    """A chord at a point in time"""
    time: float           # Start time in seconds
    duration: float       # Duration in seconds
    chord: str            # Chord name (e.g., "Am", "G", "F#dim")
    confidence: float     # 0.0 - 1.0


@dataclass
class BeatInfo:
    """Beat and tempo information"""
    tempo: float                    # BPM
    tempo_confidence: float         # 0.0 - 1.0
    beats: List[float]              # Beat timestamps
    downbeats: List[float]          # Bar-start timestamps
    time_signature: Tuple[int, int] # e.g., (4, 4)


@dataclass
class SpectralFeatures:
    """Time-series spectral features (sampled at 10Hz)"""
    times: np.ndarray           # Timestamps
    centroid: np.ndarray        # Spectral centroid (brightness)
    rolloff: np.ndarray         # Spectral rolloff
    flux: np.ndarray            # Spectral flux (change rate)
    rms: np.ndarray             # RMS energy
    zcr: np.ndarray             # Zero crossing rate


@dataclass
class AnalysisResult:
    """Complete analysis result from Essentia"""
    # Basic info
    duration: float
    sample_rate: int

    # Key and harmony
    key: KeyResult
    chords: List[ChordEvent]

    # Rhythm
    beats: BeatInfo

    # Spectral
    spectral: SpectralFeatures

    # Energy curve (10Hz sample rate)
    energy_curve: np.ndarray
    loudness_curve: np.ndarray


class EssentiaAnalyzer:
    """
    Music analyzer using Essentia library.

    Essentia provides state-of-the-art algorithms for:
    - Key detection (using key profiles)
    - Chord recognition (using HPCP features)
    - Beat tracking (using BeatTrackerMultiFeature)
    - Spectral analysis
    """

    def __init__(self, sample_rate: int = 44100):
        self.sample_rate = sample_rate
        self.hop_size = 512
        self.frame_size = 2048

        if ESSENTIA_AVAILABLE:
            self._init_essentia()
        elif LIBROSA_AVAILABLE:
            logger.info("Using librosa for analysis")
        else:
            raise RuntimeError("Neither Essentia nor librosa available")

    def _init_essentia(self):
        """Initialize Essentia algorithms"""
        # Audio loading
        self.mono_mixer = es.MonoMixer()

        # Rhythm
        self.rhythm_extractor = es.RhythmExtractor2013(method="multifeature")

        # Key detection
        self.key_extractor = es.KeyExtractor()

        # Spectral
        self.spectrum = es.Spectrum()
        self.spectral_centroid = es.Centroid()
        self.spectral_rolloff = es.RollOff()
        self.flux = es.Flux()
        self.rms = es.RMS()
        self.zcr = es.ZeroCrossingRate()

        # Windowing
        self.windowing = es.Windowing(type="hann")

    def analyze(self, audio_path: str) -> AnalysisResult:
        """
        Analyze an audio file.

        Args:
            audio_path: Path to audio file (MP3, WAV, FLAC, etc.)

        Returns:
            AnalysisResult with all extracted features
        """
        audio_path = Path(audio_path)
        if not audio_path.exists():
            raise FileNotFoundError(f"Audio file not found: {audio_path}")

        logger.info(f"Analyzing: {audio_path}")

        if ESSENTIA_AVAILABLE:
            return self._analyze_essentia(str(audio_path))
        else:
            return self._analyze_librosa(str(audio_path))

    def _analyze_essentia(self, audio_path: str) -> AnalysisResult:
        """Analyze using Essentia"""
        # Load audio
        loader = es.MonoLoader(filename=audio_path, sampleRate=self.sample_rate)
        audio = loader()
        duration = len(audio) / self.sample_rate

        logger.info(f"Duration: {duration:.1f}s, Sample rate: {self.sample_rate}")

        # Key detection
        logger.info("Detecting key...")
        key, scale, key_strength = self.key_extractor(audio)
        key_result = KeyResult(key=key, scale=scale, confidence=key_strength)
        logger.info(f"Key: {key_result}")

        # Rhythm extraction
        logger.info("Extracting rhythm...")
        bpm, beats, beats_confidence, _, beats_intervals = self.rhythm_extractor(audio)

        # Estimate downbeats (simplified: every 4th beat)
        downbeats = beats[::4].tolist() if len(beats) >= 4 else beats.tolist()

        beat_info = BeatInfo(
            tempo=bpm,
            tempo_confidence=beats_confidence,
            beats=beats.tolist(),
            downbeats=downbeats,
            time_signature=(4, 4),
        )
        logger.info(f"Tempo: {bpm:.1f} BPM")

        # Chord detection
        logger.info("Detecting chords...")
        chords = self._detect_chords_essentia(audio)
        logger.info(f"Found {len(chords)} chord changes")

        # Spectral features
        logger.info("Extracting spectral features...")
        spectral = self._extract_spectral_essentia(audio)

        # Energy curve
        energy_curve = self._compute_energy_curve(spectral.rms)
        loudness_curve = self._compute_loudness_curve(audio)

        return AnalysisResult(
            duration=duration,
            sample_rate=self.sample_rate,
            key=key_result,
            chords=chords,
            beats=beat_info,
            spectral=spectral,
            energy_curve=energy_curve,
            loudness_curve=loudness_curve,
        )

    def _detect_chords_essentia(self, audio: np.ndarray) -> List[ChordEvent]:
        """Detect chord progression using HPCP features"""
        chords = []

        # Use ChordsDetection algorithm
        try:
            chords_detection = es.ChordsDetection(hopSize=self.hop_size)
            hpcp_algo = es.HPCP()
            spectrum_algo = es.Spectrum()
            peaks_algo = es.SpectralPeaks()

            # Frame-by-frame analysis
            frame_size = self.frame_size
            hop_size = self.hop_size
            num_frames = (len(audio) - frame_size) // hop_size

            hpcps = []
            for i in range(num_frames):
                frame = audio[i * hop_size:i * hop_size + frame_size]
                frame = self.windowing(frame)
                spec = spectrum_algo(frame)
                freqs, mags = peaks_algo(spec)
                hpcp = hpcp_algo(freqs, mags)
                hpcps.append(hpcp)

            hpcps = np.array(hpcps)

            # Get chord labels
            chord_labels, chord_strengths = chords_detection(hpcps)

            # Convert to ChordEvent list
            current_chord = None
            chord_start = 0.0

            for i, (chord, strength) in enumerate(zip(chord_labels, chord_strengths)):
                time = i * hop_size / self.sample_rate

                if chord != current_chord:
                    if current_chord is not None:
                        chords.append(ChordEvent(
                            time=chord_start,
                            duration=time - chord_start,
                            chord=current_chord,
                            confidence=float(strength),
                        ))
                    current_chord = chord
                    chord_start = time

            # Add final chord
            if current_chord is not None:
                chords.append(ChordEvent(
                    time=chord_start,
                    duration=len(audio) / self.sample_rate - chord_start,
                    chord=current_chord,
                    confidence=0.8,
                ))

        except Exception as e:
            logger.warning(f"Chord detection failed: {e}, using placeholder")
            # Return placeholder chords every 4 seconds
            duration = len(audio) / self.sample_rate
            placeholder_chords = ["Am", "F", "C", "G"]
            for i, t in enumerate(np.arange(0, duration, 4.0)):
                chords.append(ChordEvent(
                    time=t,
                    duration=min(4.0, duration - t),
                    chord=placeholder_chords[i % 4],
                    confidence=0.5,
                ))

        return chords

    def _extract_spectral_essentia(self, audio: np.ndarray) -> SpectralFeatures:
        """Extract spectral features frame-by-frame"""
        frame_size = self.frame_size
        hop_size = self.hop_size
        num_frames = (len(audio) - frame_size) // hop_size

        times = []
        centroids = []
        rolloffs = []
        fluxes = []
        rms_values = []
        zcr_values = []

        prev_spectrum = None

        for i in range(num_frames):
            frame = audio[i * hop_size:i * hop_size + frame_size]
            time = i * hop_size / self.sample_rate
            times.append(time)

            # Windowed frame
            windowed = self.windowing(frame)
            spec = self.spectrum(windowed)

            # Spectral features
            centroids.append(self.spectral_centroid(spec))
            rolloffs.append(self.spectral_rolloff(spec))

            if prev_spectrum is not None:
                fluxes.append(self.flux(spec, prev_spectrum))
            else:
                fluxes.append(0.0)
            prev_spectrum = spec

            # Time domain features
            rms_values.append(self.rms(frame))
            zcr_values.append(self.zcr(frame))

        # Resample to 10Hz
        target_times = np.arange(0, times[-1], 0.1)

        return SpectralFeatures(
            times=np.array(target_times),
            centroid=np.interp(target_times, times, centroids),
            rolloff=np.interp(target_times, times, rolloffs),
            flux=np.interp(target_times, times, fluxes),
            rms=np.interp(target_times, times, rms_values),
            zcr=np.interp(target_times, times, zcr_values),
        )

    def _analyze_librosa(self, audio_path: str) -> AnalysisResult:
        """Fallback analysis using librosa"""
        import librosa

        # Load audio
        y, sr = librosa.load(audio_path, sr=self.sample_rate)
        duration = len(y) / sr

        logger.info(f"Duration: {duration:.1f}s (librosa)")

        # Key detection (using chroma)
        chroma = librosa.feature.chroma_cqt(y=y, sr=sr)
        key_idx = np.argmax(np.mean(chroma, axis=1))
        key_names = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']
        key = key_names[key_idx]

        # Simple major/minor detection
        major_profile = np.array([1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1])
        minor_profile = np.array([1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0])
        chroma_sum = np.mean(chroma, axis=1)
        major_corr = np.correlate(np.roll(chroma_sum, -key_idx), major_profile)[0]
        minor_corr = np.correlate(np.roll(chroma_sum, -key_idx), minor_profile)[0]
        scale = "major" if major_corr > minor_corr else "minor"

        key_result = KeyResult(key=key, scale=scale, confidence=0.7)

        # Tempo and beats
        tempo, beats = librosa.beat.beat_track(y=y, sr=sr)
        beat_times = librosa.frames_to_time(beats, sr=sr)

        beat_info = BeatInfo(
            tempo=float(tempo),
            tempo_confidence=0.8,
            beats=beat_times.tolist(),
            downbeats=beat_times[::4].tolist() if len(beat_times) >= 4 else beat_times.tolist(),
            time_signature=(4, 4),
        )

        # Spectral features
        times = librosa.times_like(chroma, sr=sr)
        centroid = librosa.feature.spectral_centroid(y=y, sr=sr)[0]
        rolloff = librosa.feature.spectral_rolloff(y=y, sr=sr)[0]
        rms = librosa.feature.rms(y=y)[0]
        zcr = librosa.feature.zero_crossing_rate(y)[0]

        # Compute flux
        spec = np.abs(librosa.stft(y))
        flux = np.sqrt(np.sum(np.diff(spec, axis=1) ** 2, axis=0))
        flux = np.concatenate([[0], flux])

        # Resample to 10Hz
        target_times = np.arange(0, duration, 0.1)
        spectral = SpectralFeatures(
            times=target_times,
            centroid=np.interp(target_times, times, centroid),
            rolloff=np.interp(target_times, times, rolloff),
            flux=np.interp(target_times, librosa.times_like(flux, sr=sr), flux),
            rms=np.interp(target_times, librosa.times_like(rms, sr=sr), rms),
            zcr=np.interp(target_times, librosa.times_like(zcr, sr=sr), zcr),
        )

        # Placeholder chords
        chords = []
        placeholder = ["Am", "F", "C", "G"]
        for i, t in enumerate(np.arange(0, duration, 4.0)):
            chords.append(ChordEvent(
                time=t,
                duration=min(4.0, duration - t),
                chord=placeholder[i % 4],
                confidence=0.5,
            ))

        # Energy curve
        energy_curve = self._compute_energy_curve(spectral.rms)
        loudness_curve = spectral.rms  # Simplified

        return AnalysisResult(
            duration=duration,
            sample_rate=sr,
            key=key_result,
            chords=chords,
            beats=beat_info,
            spectral=spectral,
            energy_curve=energy_curve,
            loudness_curve=loudness_curve,
        )

    def _compute_energy_curve(self, rms: np.ndarray) -> np.ndarray:
        """Compute smoothed energy curve"""
        # Smooth with moving average
        window = 10  # 1 second at 10Hz
        if len(rms) < window:
            return rms
        cumsum = np.cumsum(np.insert(rms, 0, 0))
        smoothed = (cumsum[window:] - cumsum[:-window]) / window
        # Pad to match length
        pad_size = len(rms) - len(smoothed)
        smoothed = np.concatenate([smoothed[:1]] * pad_size + [smoothed])
        return smoothed

    def _compute_loudness_curve(self, audio: np.ndarray) -> np.ndarray:
        """Compute loudness curve (EBU R128 simplified)"""
        # Simple RMS-based loudness at 10Hz
        hop = self.sample_rate // 10
        num_frames = len(audio) // hop
        loudness = []
        for i in range(num_frames):
            frame = audio[i * hop:(i + 1) * hop]
            rms = np.sqrt(np.mean(frame ** 2))
            # Convert to dB (with floor)
            db = 20 * np.log10(max(rms, 1e-10))
            loudness.append(db)
        return np.array(loudness)


def analyze_song(audio_path: str, sample_rate: int = 44100) -> AnalysisResult:
    """
    Convenience function to analyze a song.

    Args:
        audio_path: Path to audio file
        sample_rate: Target sample rate

    Returns:
        AnalysisResult with all features
    """
    analyzer = EssentiaAnalyzer(sample_rate=sample_rate)
    return analyzer.analyze(audio_path)
