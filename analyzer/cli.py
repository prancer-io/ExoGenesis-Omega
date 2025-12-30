#!/usr/bin/env python3
"""
SYNESTHESIA Analyzer CLI

Command-line interface for analyzing songs and creating .synth files.

Usage:
    synth-analyze song.mp3                    # Analyze and save song.synth
    synth-analyze song.mp3 -o output.synth    # Specify output path
    synth-analyze song.mp3 --json             # Also output JSON for inspection
    synth-analyze --batch playlist/           # Analyze all files in directory
"""

import click
import logging
import sys
from pathlib import Path
from typing import Optional
import json

# Setup logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
    datefmt='%H:%M:%S'
)
logger = logging.getLogger(__name__)


@click.group()
@click.version_option(version='0.1.0')
def cli():
    """SYNESTHESIA Music Analyzer - Create .synth files from audio"""
    pass


@cli.command()
@click.argument('audio_path', type=click.Path(exists=True))
@click.option('-o', '--output', type=click.Path(), help='Output .synth file path')
@click.option('--json', 'output_json', is_flag=True, help='Also output JSON for inspection')
@click.option('--sample-rate', default=44100, help='Sample rate for analysis')
@click.option('-v', '--verbose', is_flag=True, help='Verbose output')
def analyze(
    audio_path: str,
    output: Optional[str],
    output_json: bool,
    sample_rate: int,
    verbose: bool,
):
    """
    Analyze a song and create a .synth file.

    AUDIO_PATH: Path to audio file (MP3, WAV, FLAC, etc.)
    """
    if verbose:
        logging.getLogger().setLevel(logging.DEBUG)

    audio_path = Path(audio_path)
    if not output:
        output = audio_path.with_suffix('.synth')
    output = Path(output)

    click.echo(f"🎵 Analyzing: {audio_path.name}")
    click.echo(f"📁 Output: {output}")

    try:
        # Import here to avoid slow startup
        from .essentia_analyzer import EssentiaAnalyzer
        from .structure_detector import StructureDetector
        from .emotion_mapper import EmotionMapper
        from .synth_format import create_synth_file

        # Step 1: Audio analysis
        click.echo("\n⏳ Step 1/4: Extracting audio features...")
        analyzer = EssentiaAnalyzer(sample_rate=sample_rate)
        analysis = analyzer.analyze(str(audio_path))
        click.echo(f"   ✓ Duration: {analysis.duration:.1f}s")
        click.echo(f"   ✓ Key: {analysis.key}")
        click.echo(f"   ✓ Tempo: {analysis.beats.tempo:.1f} BPM")

        # Step 2: Structure detection
        click.echo("\n⏳ Step 2/4: Detecting song structure...")
        structure_detector = StructureDetector()

        # Combine spectral features for structure analysis
        import numpy as np
        spectral_features = np.column_stack([
            analysis.spectral.centroid,
            analysis.spectral.rolloff,
            analysis.spectral.flux,
            analysis.spectral.rms,
        ])

        sections, climaxes = structure_detector.detect(
            spectral_features,
            analysis.energy_curve,
            analysis.beats.beats,
            analysis.beats.tempo,
            analysis.duration,
        )
        click.echo(f"   ✓ Sections: {len(sections)}")
        for s in sections:
            click.echo(f"      {s.start_time:5.1f}s - {s.end_time:5.1f}s: {s.section_type.value}")
        click.echo(f"   ✓ Climax points: {len(climaxes)}")

        # Step 3: Emotion mapping
        click.echo("\n⏳ Step 3/4: Mapping emotions...")
        emotion_mapper = EmotionMapper()
        emotion_arc = emotion_mapper.map(
            key=analysis.key.key,
            mode=analysis.key.scale,
            tempo=analysis.beats.tempo,
            energy_curve=analysis.energy_curve,
            sections=sections,
            duration=analysis.duration,
            chords=analysis.chords,
        )
        click.echo(f"   ✓ Dominant emotion: {emotion_arc.dominant_emotion.value}")
        click.echo(f"   ✓ Emotional range: {emotion_arc.emotional_range:.2f}")

        # Step 4: Create .synth file
        click.echo("\n⏳ Step 4/4: Creating .synth file...")
        synth_file = create_synth_file(
            analysis_result=analysis,
            structure_result=(sections, climaxes),
            emotion_arc=emotion_arc,
            audio_path=str(audio_path),
        )
        synth_file.save(str(output))
        click.echo(f"   ✓ Saved: {output} ({output.stat().st_size / 1024:.1f} KB)")

        # Optional: Output JSON
        if output_json:
            json_path = output.with_suffix('.json')
            with open(json_path, 'w') as f:
                f.write(synth_file.to_json())
            click.echo(f"   ✓ JSON: {json_path}")

        click.echo("\n✅ Analysis complete!")

    except Exception as e:
        logger.exception("Analysis failed")
        click.echo(f"\n❌ Error: {e}", err=True)
        sys.exit(1)


@cli.command()
@click.argument('directory', type=click.Path(exists=True, file_okay=False))
@click.option('--ext', default='.mp3,.wav,.flac', help='File extensions to process')
@click.option('-v', '--verbose', is_flag=True, help='Verbose output')
def batch(directory: str, ext: str, verbose: bool):
    """
    Analyze all audio files in a directory.

    DIRECTORY: Path to directory containing audio files
    """
    from .essentia_analyzer import EssentiaAnalyzer
    from .structure_detector import StructureDetector
    from .emotion_mapper import EmotionMapper
    from .synth_format import create_synth_file
    import numpy as np

    extensions = [e.strip().lower() for e in ext.split(',')]
    directory = Path(directory)

    # Find all audio files
    audio_files = []
    for e in extensions:
        if not e.startswith('.'):
            e = '.' + e
        audio_files.extend(directory.glob(f'*{e}'))
        audio_files.extend(directory.glob(f'**/*{e}'))

    audio_files = sorted(set(audio_files))
    click.echo(f"Found {len(audio_files)} audio files")

    if not audio_files:
        click.echo("No audio files found!")
        return

    # Initialize analyzers
    analyzer = EssentiaAnalyzer()
    structure_detector = StructureDetector()
    emotion_mapper = EmotionMapper()

    success = 0
    failed = 0

    for i, audio_path in enumerate(audio_files, 1):
        click.echo(f"\n[{i}/{len(audio_files)}] {audio_path.name}")

        try:
            # Analyze
            analysis = analyzer.analyze(str(audio_path))

            # Structure
            spectral_features = np.column_stack([
                analysis.spectral.centroid,
                analysis.spectral.rolloff,
                analysis.spectral.flux,
                analysis.spectral.rms,
            ])
            sections, climaxes = structure_detector.detect(
                spectral_features,
                analysis.energy_curve,
                analysis.beats.beats,
                analysis.beats.tempo,
                analysis.duration,
            )

            # Emotion
            emotion_arc = emotion_mapper.map(
                key=analysis.key.key,
                mode=analysis.key.scale,
                tempo=analysis.beats.tempo,
                energy_curve=analysis.energy_curve,
                sections=sections,
                duration=analysis.duration,
                chords=analysis.chords,
            )

            # Save
            synth_file = create_synth_file(
                analysis_result=analysis,
                structure_result=(sections, climaxes),
                emotion_arc=emotion_arc,
                audio_path=str(audio_path),
            )
            output = audio_path.with_suffix('.synth')
            synth_file.save(str(output))

            click.echo(f"   ✓ {output.name}")
            success += 1

        except Exception as e:
            click.echo(f"   ✗ Error: {e}", err=True)
            failed += 1

    click.echo(f"\n{'=' * 40}")
    click.echo(f"Completed: {success} success, {failed} failed")


@cli.command()
@click.argument('synth_path', type=click.Path(exists=True))
def info(synth_path: str):
    """
    Display information about a .synth file.

    SYNTH_PATH: Path to .synth file
    """
    from .synth_format import SynthFile

    synth = SynthFile.load(synth_path)

    click.echo(f"\n📦 {Path(synth_path).name}")
    click.echo(f"{'=' * 40}")
    click.echo(f"Version: {synth.version}")
    click.echo(f"Created: {synth.created_at}")

    if synth.analysis:
        a = synth.analysis
        click.echo(f"\n🎵 Audio Analysis")
        click.echo(f"   Duration: {a.duration:.1f}s")
        click.echo(f"   Key: {a.key} {a.mode} (confidence: {a.key_confidence:.0%})")
        click.echo(f"   Tempo: {a.tempo:.1f} BPM")
        click.echo(f"   Beats: {len(a.beats)}")
        click.echo(f"   Chords: {len(a.chords)}")
        click.echo(f"   Sections: {len(a.sections)}")
        click.echo(f"   Climaxes: {len(a.climaxes)}")

        click.echo(f"\n📊 Sections:")
        for s in a.sections:
            click.echo(f"   {s['start']:5.1f}s - {s['end']:5.1f}s: {s['type']}")

    if synth.video_segments:
        click.echo(f"\n🎬 Video Segments: {len(synth.video_segments)}")

    click.echo(f"\n🎨 Style: {synth.style_name}")


@cli.command()
@click.argument('synth_path', type=click.Path(exists=True))
@click.argument('output_path', type=click.Path())
def export_json(synth_path: str, output_path: str):
    """
    Export .synth file as JSON.

    SYNTH_PATH: Input .synth file
    OUTPUT_PATH: Output JSON file
    """
    from .synth_format import SynthFile

    synth = SynthFile.load(synth_path)

    with open(output_path, 'w') as f:
        f.write(synth.to_json())

    click.echo(f"Exported to {output_path}")


def main():
    """Entry point"""
    cli()


if __name__ == '__main__':
    main()
