"""
Video Segment Generator

Generates pre-rendered video segments for music visualization.
Uses open-source video generation models (CogVideoX, Open-Sora, etc.)
or falls back to procedural generation for faster prototyping.

This is the OFFLINE component - runs during analysis, not playback.
"""

import logging
import numpy as np
from pathlib import Path
from dataclasses import dataclass
from typing import List, Optional, Dict, Any, Tuple
from enum import Enum
import json
import subprocess
import tempfile
import shutil

logger = logging.getLogger(__name__)


class GenerationMode(Enum):
    """Video generation mode"""
    PROCEDURAL = "procedural"      # Fast, CPU-based procedural visuals
    COGVIDEO = "cogvideo"          # CogVideoX model
    OPEN_SORA = "open_sora"        # Open-Sora model
    MOCHI = "mochi"                # Mochi model
    STATIC = "static"              # Static images with Ken Burns


@dataclass
class VideoSegment:
    """A generated video segment"""
    segment_id: int
    start_time: float
    end_time: float
    video_path: str
    mood: str
    clarity_level: float
    base_hue: float
    saturation: float
    brightness: float
    motion_speed: float
    prompt: str


@dataclass
class GenerationConfig:
    """Configuration for video generation"""
    mode: GenerationMode = GenerationMode.PROCEDURAL
    output_dir: str = "segments"
    width: int = 1920
    height: int = 1080
    fps: int = 30
    quality: str = "high"  # low, medium, high
    model_path: Optional[str] = None
    device: str = "cuda"  # cuda, cpu


class VideoGenerator:
    """
    Generates video segments based on music analysis.

    Primary approach: Procedural generation with shaders
    Optional: AI video generation (requires separate model loading)
    """

    def __init__(self, config: GenerationConfig = None):
        self.config = config or GenerationConfig()
        self.output_dir = Path(self.config.output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)

        # Check available modes
        self._available_modes = self._detect_available_modes()

        if self.config.mode not in self._available_modes:
            logger.warning(f"Mode {self.config.mode} not available, falling back to PROCEDURAL")
            self.config.mode = GenerationMode.PROCEDURAL

    def _detect_available_modes(self) -> List[GenerationMode]:
        """Detect which generation modes are available"""
        available = [GenerationMode.PROCEDURAL, GenerationMode.STATIC]

        # Check for ffmpeg (required for all video output)
        if shutil.which("ffmpeg"):
            logger.debug("ffmpeg found")
        else:
            logger.warning("ffmpeg not found - video generation will be limited")

        # Check for AI models (optional)
        try:
            import torch
            if torch.cuda.is_available():
                logger.debug(f"CUDA available: {torch.cuda.get_device_name(0)}")
                # Add AI modes if models are available
                # available.append(GenerationMode.COGVIDEO)
        except ImportError:
            logger.debug("PyTorch not available - AI modes disabled")

        return available

    def generate_segments(
        self,
        sections: List[Dict],
        emotion_arc: List[Dict],
        energy_curve: List[float],
        tempo: float,
        key: str,
        mode: str,
        duration: float,
    ) -> List[VideoSegment]:
        """
        Generate video segments for each section of the song.

        Args:
            sections: List of song sections from structure detector
            emotion_arc: Emotion points over time
            energy_curve: Energy values (10Hz sample rate)
            tempo: Song tempo in BPM
            key: Musical key
            mode: Musical mode (major/minor)
            duration: Total song duration

        Returns:
            List of VideoSegment objects
        """
        segments = []

        for i, section in enumerate(sections):
            segment = self._generate_segment(
                segment_id=i,
                section=section,
                emotion_arc=emotion_arc,
                energy_curve=energy_curve,
                tempo=tempo,
                key=key,
                mode=mode,
            )
            segments.append(segment)
            logger.info(f"Generated segment {i}: {section.get('type', 'unknown')} ({segment.video_path})")

        return segments

    def _generate_segment(
        self,
        segment_id: int,
        section: Dict,
        emotion_arc: List[Dict],
        energy_curve: List[float],
        tempo: float,
        key: str,
        mode: str,
    ) -> VideoSegment:
        """Generate a single video segment"""

        start_time = section.get('start', 0)
        end_time = section.get('end', start_time + 10)
        section_type = section.get('type', 'unknown')
        energy = section.get('energy', 0.5)

        # Find emotion at this section
        emotion = self._find_emotion_at(emotion_arc, start_time)

        # Calculate visual parameters
        clarity = self._calculate_clarity(section_type, section.get('repetition', 0))
        hue = self._emotion_to_hue(emotion)
        saturation = 0.5 + energy * 0.3
        brightness = 0.4 + energy * 0.3
        motion = tempo / 120.0  # Normalized to 120 BPM

        # Generate prompt
        prompt = self._generate_prompt(
            section_type, emotion, key, mode, energy, clarity
        )

        # Generate video based on mode
        if self.config.mode == GenerationMode.PROCEDURAL:
            video_path = self._generate_procedural(
                segment_id, end_time - start_time, hue, saturation, brightness, motion
            )
        elif self.config.mode == GenerationMode.STATIC:
            video_path = self._generate_static(
                segment_id, end_time - start_time, hue, saturation, brightness
            )
        else:
            # Fallback to procedural
            video_path = self._generate_procedural(
                segment_id, end_time - start_time, hue, saturation, brightness, motion
            )

        return VideoSegment(
            segment_id=segment_id,
            start_time=start_time,
            end_time=end_time,
            video_path=str(video_path),
            mood=emotion,
            clarity_level=clarity,
            base_hue=hue,
            saturation=saturation,
            brightness=brightness,
            motion_speed=motion,
            prompt=prompt,
        )

    def _find_emotion_at(self, emotion_arc: List[Dict], time: float) -> str:
        """Find emotion at a given time"""
        emotion = "neutral"
        for point in emotion_arc:
            if point.get('time', 0) <= time:
                emotion = point.get('emotion', 'neutral')
            else:
                break
        return emotion

    def _calculate_clarity(self, section_type: str, repetition: int) -> float:
        """Calculate clarity level for section"""
        base_clarity = {
            'intro': 0.1,
            'verse': 0.25,
            'pre_chorus': 0.35,
            'chorus': 0.6,
            'drop': 0.8,
            'bridge': 0.4,
            'breakdown': 0.3,
            'buildup': 0.5,
            'outro': 0.7,
        }.get(section_type, 0.3)

        # Repetition increases clarity
        repetition_bonus = min(repetition * 0.1, 0.2)

        return min(base_clarity + repetition_bonus, 1.0)

    def _emotion_to_hue(self, emotion: str) -> float:
        """Convert emotion to base hue"""
        hue_map = {
            'joy': 45.0,
            'triumph': 30.0,
            'excitement': 15.0,
            'euphoria': 300.0,
            'anger': 0.0,
            'intensity': 350.0,
            'urgency': 20.0,
            'chaos': 280.0,
            'peace': 180.0,
            'tenderness': 330.0,
            'hope': 60.0,
            'nostalgia': 35.0,
            'sadness': 220.0,
            'melancholy': 250.0,
            'tension': 270.0,
            'dread': 260.0,
            'neutral': 200.0,
        }
        return hue_map.get(emotion.lower(), 200.0)

    def _generate_prompt(
        self,
        section_type: str,
        emotion: str,
        key: str,
        mode: str,
        energy: float,
        clarity: float,
    ) -> str:
        """Generate a video prompt based on musical context"""

        # Section mood
        section_moods = {
            'intro': 'emerging from darkness, mysterious beginning',
            'verse': 'narrative unfolding, steady progression',
            'pre_chorus': 'building anticipation, rising tension',
            'chorus': 'powerful revelation, emotional peak',
            'drop': 'explosive transformation, maximum impact',
            'bridge': 'perspective shift, contemplative moment',
            'breakdown': 'stripped down, intimate focus',
            'buildup': 'ascending intensity, growing power',
            'outro': 'resolution, fading into peace',
        }
        mood = section_moods.get(section_type, 'abstract motion')

        # Energy descriptor
        energy_desc = 'explosive' if energy > 0.8 else 'dynamic' if energy > 0.5 else 'gentle' if energy > 0.3 else 'calm'

        # Mode feeling
        mode_feel = 'bright, hopeful' if mode == 'major' else 'deep, introspective'

        # Clarity style
        if clarity < 0.3:
            clarity_style = 'abstract, undefined shapes, pure color and motion'
        elif clarity < 0.6:
            clarity_style = 'forms emerging, dreamlike quality'
        else:
            clarity_style = 'clear vision, detailed cinematic scene'

        return f"{mood}, {emotion} atmosphere, {energy_desc} {mode_feel} in {key}, {clarity_style}"

    def _generate_procedural(
        self,
        segment_id: int,
        duration: float,
        hue: float,
        saturation: float,
        brightness: float,
        motion: float,
    ) -> Path:
        """Generate procedural video using ffmpeg filters"""

        output_path = self.output_dir / f"segment_{segment_id:03d}.mp4"

        # Convert HSL to RGB for ffmpeg
        r, g, b = self._hsl_to_rgb(hue / 360.0, saturation, brightness)

        # Build ffmpeg filter for procedural animation
        # Uses gradients, noise, and color effects
        filter_complex = f"""
        [0:v]format=yuv420p,
        geq=r='128+64*sin(2*PI*T*{motion})+{int(r*127)}*sin(X/W*PI)*sin(Y/H*PI)':
            g='128+64*sin(2*PI*T*{motion}+2)+{int(g*127)}*sin(X/W*PI)*sin(Y/H*PI)':
            b='128+64*sin(2*PI*T*{motion}+4)+{int(b*127)}*sin(X/W*PI)*sin(Y/H*PI)',
        hue=h={hue}:s={saturation*2}:b={brightness*2-1},
        eq=brightness={brightness-0.5}:saturation={saturation*2}
        """

        # Create base video with color
        cmd = [
            "ffmpeg", "-y",
            "-f", "lavfi",
            "-i", f"color=c=black:s={self.config.width}x{self.config.height}:d={duration}:r={self.config.fps}",
            "-filter_complex", filter_complex.replace('\n', '').replace('  ', ''),
            "-c:v", "libx264",
            "-preset", "fast" if self.config.quality == "low" else "medium",
            "-crf", "23" if self.config.quality == "high" else "28",
            "-t", str(duration),
            str(output_path),
        ]

        try:
            subprocess.run(cmd, capture_output=True, check=True)
        except subprocess.CalledProcessError as e:
            logger.error(f"FFmpeg error: {e.stderr.decode() if e.stderr else str(e)}")
            # Create a simple fallback
            self._generate_fallback(output_path, duration, hue, brightness)
        except FileNotFoundError:
            logger.warning("ffmpeg not found, creating placeholder")
            self._generate_fallback(output_path, duration, hue, brightness)

        return output_path

    def _generate_static(
        self,
        segment_id: int,
        duration: float,
        hue: float,
        saturation: float,
        brightness: float,
    ) -> Path:
        """Generate static image with Ken Burns effect"""

        output_path = self.output_dir / f"segment_{segment_id:03d}.mp4"

        # Generate gradient image, then apply zoom/pan
        filter_complex = f"""
        color=c=black:s={self.config.width}x{self.config.height}:d={duration},
        gradients=c0=hsl({hue},1,0.3):c1=hsl({(hue+180)%360},1,0.3):x0=0:y0=0:x1={self.config.width}:y1={self.config.height},
        zoompan=z='1.0+0.1*sin(2*PI*time/{duration})':x='iw/2-iw/zoom/2':y='ih/2-ih/zoom/2':d={int(duration*self.config.fps)}:s={self.config.width}x{self.config.height}:fps={self.config.fps}
        """

        cmd = [
            "ffmpeg", "-y",
            "-f", "lavfi",
            "-i", filter_complex.replace('\n', '').replace('  ', ''),
            "-c:v", "libx264",
            "-preset", "fast",
            "-crf", "23",
            "-t", str(duration),
            str(output_path),
        ]

        try:
            subprocess.run(cmd, capture_output=True, check=True)
        except (subprocess.CalledProcessError, FileNotFoundError):
            self._generate_fallback(output_path, duration, hue, brightness)

        return output_path

    def _generate_fallback(self, output_path: Path, duration: float, hue: float, brightness: float):
        """Create a simple placeholder video"""
        # Just create a simple colored video
        r, g, b = self._hsl_to_rgb(hue / 360.0, 0.5, brightness)
        color = f"#{int(r*255):02x}{int(g*255):02x}{int(b*255):02x}"

        cmd = [
            "ffmpeg", "-y",
            "-f", "lavfi",
            "-i", f"color=c={color}:s={self.config.width}x{self.config.height}:d={duration}:r={self.config.fps}",
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-crf", "28",
            "-t", str(duration),
            str(output_path),
        ]

        try:
            subprocess.run(cmd, capture_output=True, check=True)
        except Exception as e:
            logger.error(f"Could not generate fallback video: {e}")
            # Create empty file as last resort
            output_path.touch()

    def _hsl_to_rgb(self, h: float, s: float, l: float) -> Tuple[float, float, float]:
        """Convert HSL to RGB (0-1 range)"""
        if s == 0:
            return (l, l, l)

        def hue_to_rgb(p, q, t):
            if t < 0: t += 1
            if t > 1: t -= 1
            if t < 1/6: return p + (q - p) * 6 * t
            if t < 1/2: return q
            if t < 2/3: return p + (q - p) * (2/3 - t) * 6
            return p

        q = l * (1 + s) if l < 0.5 else l + s - l * s
        p = 2 * l - q

        r = hue_to_rgb(p, q, h + 1/3)
        g = hue_to_rgb(p, q, h)
        b = hue_to_rgb(p, q, h - 1/3)

        return (r, g, b)


def generate_segments_for_song(
    sections: List[Dict],
    emotion_arc: List[Dict],
    energy_curve: List[float],
    tempo: float,
    key: str,
    mode: str,
    duration: float,
    output_dir: str = "segments",
    generation_mode: str = "procedural",
) -> List[Dict]:
    """
    Convenience function to generate all segments for a song.

    Returns list of segment dictionaries for .synth file.
    """
    config = GenerationConfig(
        mode=GenerationMode(generation_mode),
        output_dir=output_dir,
    )

    generator = VideoGenerator(config)
    segments = generator.generate_segments(
        sections=sections,
        emotion_arc=emotion_arc,
        energy_curve=energy_curve,
        tempo=tempo,
        key=key,
        mode=mode,
        duration=duration,
    )

    return [
        {
            'segment_id': s.segment_id,
            'start_time': s.start_time,
            'end_time': s.end_time,
            'video_path': s.video_path,
            'mood': s.mood,
            'clarity_level': s.clarity_level,
            'base_hue': s.base_hue,
            'saturation': s.saturation,
            'brightness': s.brightness,
            'motion_speed': s.motion_speed,
        }
        for s in segments
    ]
