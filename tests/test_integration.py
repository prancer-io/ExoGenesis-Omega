#!/usr/bin/env python3
"""
Integration Tests for SYNESTHESIA Analyzer Pipeline

Tests the complete workflow from audio input to .synth file output.
"""

import unittest
import tempfile
import shutil
import numpy as np
from pathlib import Path
import struct


class TestSynthFormat(unittest.TestCase):
    """Test .synth file format creation and loading"""

    def test_synth_file_magic(self):
        """Verify .synth files have correct magic bytes"""
        from analyzer.synth_format import SynthFile, MusicAnalysisData

        # Create minimal analysis
        analysis = MusicAnalysisData(
            duration=180.0,
            sample_rate=44100,
            audio_hash="test123",
            key="C",
            mode="major",
            key_confidence=0.9,
            tempo=120.0,
            tempo_confidence=0.85,
            beats=[0.5, 1.0, 1.5, 2.0],
            downbeats=[0.5, 2.5],
            time_signature=(4, 4),
            chords=[],
            sections=[],
            climaxes=[],
            energy_curve=[0.5] * 100,
            tension_curve=[0.3] * 100,
            loudness_curve=[0.6] * 100,
            emotion_arc=[],
        )

        synth = SynthFile(analysis=analysis)

        with tempfile.NamedTemporaryFile(suffix='.synth', delete=False) as f:
            path = f.name

        try:
            synth.save(path)

            # Check magic bytes
            with open(path, 'rb') as f:
                magic = f.read(8)
                self.assertEqual(magic, b'SYNTH\x00\x01\x00')
        finally:
            Path(path).unlink(missing_ok=True)

    def test_synth_roundtrip(self):
        """Test save and load cycle"""
        from analyzer.synth_format import SynthFile, MusicAnalysisData, Section, EmotionPoint

        analysis = MusicAnalysisData(
            duration=200.0,
            sample_rate=44100,
            audio_hash="abc123",
            key="G",
            mode="minor",
            key_confidence=0.85,
            tempo=128.0,
            tempo_confidence=0.9,
            beats=list(np.arange(0, 200, 0.5)),
            downbeats=list(np.arange(0, 200, 2.0)),
            time_signature=(4, 4),
            chords=[],
            sections=[
                Section(
                    section_type="verse",
                    start_time=0.0,
                    end_time=30.0,
                    energy=0.5,
                    repetition=0,
                    confidence=0.8,
                ),
                Section(
                    section_type="chorus",
                    start_time=30.0,
                    end_time=60.0,
                    energy=0.8,
                    repetition=0,
                    confidence=0.9,
                ),
            ],
            climaxes=[],
            energy_curve=[0.5] * 200,
            tension_curve=[0.3] * 200,
            loudness_curve=[0.6] * 200,
            emotion_arc=[
                EmotionPoint(time=0.0, emotion="peace", intensity=0.5, valence=0.6, arousal=0.3),
                EmotionPoint(time=30.0, emotion="joy", intensity=0.8, valence=0.9, arousal=0.7),
            ],
        )

        synth = SynthFile(analysis=analysis, style_name="neon")

        with tempfile.NamedTemporaryFile(suffix='.synth', delete=False) as f:
            path = f.name

        try:
            synth.save(path)
            loaded = SynthFile.load(path)

            self.assertAlmostEqual(loaded.analysis.duration, 200.0, places=1)
            self.assertEqual(loaded.analysis.key, "G")
            self.assertEqual(loaded.analysis.mode, "minor")
            self.assertAlmostEqual(loaded.analysis.tempo, 128.0, places=1)
            self.assertEqual(len(loaded.analysis.sections), 2)
            self.assertEqual(loaded.style_name, "neon")
        finally:
            Path(path).unlink(missing_ok=True)


class TestEmotionMapper(unittest.TestCase):
    """Test emotion mapping from musical features"""

    def test_major_key_positive_valence(self):
        """Major key should produce positive valence"""
        from analyzer.emotion_mapper import EmotionMapper

        mapper = EmotionMapper()
        result = mapper.map(
            key="C",
            mode="major",
            tempo=120.0,
            energy_curve=np.array([0.5] * 100),
            sections=[],
            duration=100.0,
            chords=[],
        )

        # Major key should have positive valence
        self.assertGreater(result.dominant_valence, 0.5)

    def test_minor_key_negative_valence(self):
        """Minor key should produce lower valence"""
        from analyzer.emotion_mapper import EmotionMapper

        mapper = EmotionMapper()
        result = mapper.map(
            key="A",
            mode="minor",
            tempo=80.0,
            energy_curve=np.array([0.3] * 100),
            sections=[],
            duration=100.0,
            chords=[],
        )

        # Minor key + slow tempo should have lower valence
        self.assertLess(result.dominant_valence, 0.6)

    def test_high_tempo_high_arousal(self):
        """High tempo should produce high arousal"""
        from analyzer.emotion_mapper import EmotionMapper

        mapper = EmotionMapper()
        result = mapper.map(
            key="D",
            mode="major",
            tempo=180.0,
            energy_curve=np.array([0.8] * 100),
            sections=[],
            duration=100.0,
            chords=[],
        )

        # High tempo + high energy should have high arousal
        self.assertGreater(result.dominant_arousal, 0.6)


class TestStructureDetector(unittest.TestCase):
    """Test song structure detection"""

    def test_section_detection(self):
        """Test that sections are detected from features"""
        from analyzer.structure_detector import StructureDetector

        detector = StructureDetector()

        # Create synthetic features with distinct sections
        duration = 180.0
        fps = 10
        n_frames = int(duration * fps)

        # Section 1: Low energy (0-60s)
        # Section 2: High energy (60-120s)
        # Section 3: Medium energy (120-180s)
        energy = np.concatenate([
            np.ones(600) * 0.3,  # Verse
            np.ones(600) * 0.8,  # Chorus
            np.ones(600) * 0.5,  # Bridge
        ])

        spectral = np.column_stack([
            energy,  # centroid
            energy * 0.8,  # rolloff
            np.abs(np.diff(energy, prepend=energy[0])),  # flux
            energy,  # rms
        ])

        beats = list(np.arange(0, duration, 0.5))

        sections, climaxes = detector.detect(
            spectral_features=spectral,
            energy_curve=energy,
            beats=beats,
            tempo=120.0,
            duration=duration,
        )

        # Should detect multiple sections
        self.assertGreaterEqual(len(sections), 2)

        # Sections should cover the song
        self.assertLessEqual(sections[0].start_time, 5.0)
        self.assertGreaterEqual(sections[-1].end_time, duration - 5.0)


class TestVideoGenerator(unittest.TestCase):
    """Test video segment generation"""

    def test_procedural_generation_config(self):
        """Test procedural generation configuration"""
        from analyzer.video_generator import VideoGenerator, GenerationConfig, GenerationMode

        config = GenerationConfig(
            mode=GenerationMode.PROCEDURAL,
            output_dir="test_segments",
            width=640,
            height=360,
            fps=24,
        )

        generator = VideoGenerator(config)
        self.assertEqual(generator.config.width, 640)
        self.assertEqual(generator.config.height, 360)

    def test_emotion_to_hue(self):
        """Test emotion to hue color mapping"""
        from analyzer.video_generator import VideoGenerator, GenerationConfig

        generator = VideoGenerator(GenerationConfig())

        joy_hue = generator._emotion_to_hue("joy")
        sadness_hue = generator._emotion_to_hue("sadness")

        # Joy should be warm (yellow-orange)
        self.assertGreater(joy_hue, 20)
        self.assertLess(joy_hue, 60)

        # Sadness should be cool (blue)
        self.assertGreater(sadness_hue, 200)
        self.assertLess(sadness_hue, 260)


class TestPipelineIntegration(unittest.TestCase):
    """Test the complete pipeline integration"""

    def test_minimal_pipeline(self):
        """Test that all modules can be imported and instantiated"""
        # Import all modules
        from analyzer.essentia_analyzer import EssentiaAnalyzer
        from analyzer.structure_detector import StructureDetector
        from analyzer.emotion_mapper import EmotionMapper
        from analyzer.synth_format import SynthFile
        from analyzer.video_generator import VideoGenerator

        # Instantiate all components
        structure = StructureDetector()
        emotion = EmotionMapper()
        video = VideoGenerator()

        # Verify they exist
        self.assertIsNotNone(structure)
        self.assertIsNotNone(emotion)
        self.assertIsNotNone(video)


if __name__ == '__main__':
    unittest.main()
