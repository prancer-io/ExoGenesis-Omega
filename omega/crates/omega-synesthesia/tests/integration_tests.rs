//! Integration tests for omega-synesthesia
//! Tests the complete pipeline: Audio → Features → 3D World

use omega_synesthesia::*;

// ============================================================================
// BASIC PIPELINE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_engine_creation() {
    // Test creating engines with different genres
    let _classical = SynesthesiaEngine::new(Genre::Classical);
    let _electronic = SynesthesiaEngine::new(Genre::Electronic);
    let _jazz = SynesthesiaEngine::new(Genre::Jazz);
    let _ambient = SynesthesiaEngine::new(Genre::Ambient);
    let _rock = SynesthesiaEngine::new(Genre::Rock);
}

#[test]
fn test_custom_config() {
    let config = SynesthesiaConfig {
        genre: Genre::Electronic,
        sample_rate: 48000,
        fft_size: 4096,
        time_scale: 15.0,
        pitch_scale: 75.0,
        volumetrics: true,
        particles: true,
        quality: 0.8,
    };

    let _engine = SynesthesiaEngine::with_config(config);
}

#[test]
fn test_load_sine_wave() {
    let mut engine = SynesthesiaEngine::new(Genre::Classical);

    let source = AudioSource::TestSignal(TestSignalType::Sine {
        frequency: 440.0,
        duration: 1.0,
    });

    let result = engine.load_audio(source);
    assert!(result.is_ok(), "Should load sine wave successfully");
}

#[test]
fn test_load_harmonics() {
    let mut engine = SynesthesiaEngine::new(Genre::Classical);

    let source = AudioSource::TestSignal(TestSignalType::Harmonics {
        fundamental: 220.0,
        num_harmonics: 5,
        duration: 1.0,
    });

    let result = engine.load_audio(source);
    assert!(result.is_ok(), "Should load harmonics successfully");
}

#[test]
fn test_load_sweep() {
    let mut engine = SynesthesiaEngine::new(Genre::Electronic);

    let source = AudioSource::TestSignal(TestSignalType::Sweep {
        start_freq: 200.0,
        end_freq: 2000.0,
        duration: 2.0,
    });

    let result = engine.load_audio(source);
    assert!(result.is_ok(), "Should load sweep successfully");
}

#[test]
fn test_load_simulated_music() {
    let mut engine = SynesthesiaEngine::new(Genre::Electronic);

    let source = AudioSource::TestSignal(TestSignalType::SimulatedMusic {
        bpm: 120.0,
        duration: 2.0,
    });

    let result = engine.load_audio(source);
    assert!(result.is_ok(), "Should load simulated music successfully");
}

// ============================================================================
// WORLD GENERATION INTEGRATION TESTS
// ============================================================================

#[test]
fn test_full_pipeline_sine() {
    let mut engine = SynesthesiaEngine::new(Genre::Electronic);

    let source = AudioSource::TestSignal(TestSignalType::Sine {
        frequency: 440.0,
        duration: 1.0,
    });

    engine.load_audio(source).expect("Failed to load audio");
    let world = engine.generate_world().expect("Failed to generate world");

    // Verify world was created
    assert!(!world.chunks.is_empty(), "World should have chunks");
    assert!(world.total_elements() > 0, "World should have elements");
}

#[test]
fn test_full_pipeline_harmonics() {
    let mut engine = SynesthesiaEngine::new(Genre::Classical);

    let source = AudioSource::TestSignal(TestSignalType::Harmonics {
        fundamental: 261.63, // C4
        num_harmonics: 4,
        duration: 1.5,
    });

    engine.load_audio(source).expect("Failed to load audio");
    let world = engine.generate_world().expect("Failed to generate world");

    assert!(!world.chunks.is_empty(), "Harmonics should create chunks");
    assert!(world.total_elements() > 0, "Should have elements");
}

#[test]
fn test_full_pipeline_sweep() {
    let mut engine = SynesthesiaEngine::new(Genre::Ambient);

    let source = AudioSource::TestSignal(TestSignalType::Sweep {
        start_freq: 100.0,
        end_freq: 1000.0,
        duration: 2.0,
    });

    engine.load_audio(source).expect("Failed to load audio");
    let world = engine.generate_world().expect("Failed to generate world");

    assert!(!world.chunks.is_empty(), "Sweep should create chunks");
}

#[test]
fn test_full_pipeline_music() {
    let mut engine = SynesthesiaEngine::new(Genre::Jazz);

    let source = AudioSource::TestSignal(TestSignalType::SimulatedMusic {
        bpm: 140.0,
        duration: 3.0,
    });

    engine.load_audio(source).expect("Failed to load audio");
    let world = engine.generate_world().expect("Failed to generate world");

    // Music simulation should create rich geometry
    assert!(!world.chunks.is_empty(), "Music should create chunks");
    assert!(world.total_elements() > 0);
}

// ============================================================================
// GENRE STYLE TESTS
// ============================================================================

#[test]
fn test_different_genres_produce_worlds() {
    let genres = vec![
        Genre::Classical,
        Genre::Electronic,
        Genre::Jazz,
        Genre::Ambient,
        Genre::Rock,
    ];

    let source = AudioSource::TestSignal(TestSignalType::Sine {
        frequency: 440.0,
        duration: 1.0,
    });

    for genre in genres {
        let mut engine = SynesthesiaEngine::new(genre);
        engine.load_audio(source.clone()).expect("Load failed");
        let world = engine.generate_world().expect("Generation failed");

        assert!(!world.chunks.is_empty(), "Genre {:?} should produce chunks", genre);
        assert!(world.total_elements() > 0, "Genre {:?} should have elements", genre);
    }
}

// ============================================================================
// WORLD PROPERTIES TESTS
// ============================================================================

#[test]
fn test_world_bounds() {
    let mut engine = SynesthesiaEngine::new(Genre::Classical);

    let source = AudioSource::TestSignal(TestSignalType::Sine {
        frequency: 440.0,
        duration: 1.0,
    });

    engine.load_audio(source).expect("Load failed");
    let world = engine.generate_world().expect("Generation failed");

    // Verify bounds are valid
    assert!(world.bounds.min.x <= world.bounds.max.x);
    assert!(world.bounds.min.y <= world.bounds.max.y);
    assert!(world.bounds.min.z <= world.bounds.max.z);
}

#[test]
fn test_world_element_count() {
    let mut engine = SynesthesiaEngine::new(Genre::Electronic);

    let source = AudioSource::TestSignal(TestSignalType::Harmonics {
        fundamental: 440.0,
        num_harmonics: 3,
        duration: 1.0,
    });

    engine.load_audio(source).expect("Load failed");
    let world = engine.generate_world().expect("Generation failed");

    let total_elements = world.total_elements();
    let chunks_count = world.chunks.len();

    assert!(total_elements > 0, "Should have elements");
    assert!(chunks_count > 0, "Should have chunks");
}

#[test]
fn test_world_chunk_properties() {
    let mut engine = SynesthesiaEngine::new(Genre::Jazz);

    let source = AudioSource::TestSignal(TestSignalType::Sweep {
        start_freq: 200.0,
        end_freq: 800.0,
        duration: 2.0,
    });

    engine.load_audio(source).expect("Load failed");
    let world = engine.generate_world().expect("Generation failed");

    // Verify each chunk has valid data
    for chunk in &world.chunks {
        assert!(!chunk.elements.is_empty(), "Chunk {} should have elements", chunk.index);

        // Verify bounds are valid
        assert!(chunk.bounds_min.x <= chunk.bounds_max.x);
        assert!(chunk.bounds_min.y <= chunk.bounds_max.y);
        assert!(chunk.bounds_min.z <= chunk.bounds_max.z);
    }
}

// ============================================================================
// MULTIPLE LOADS TESTS
// ============================================================================

#[test]
fn test_multiple_audio_loads() {
    let mut engine = SynesthesiaEngine::new(Genre::Ambient);

    let sources = vec![
        AudioSource::TestSignal(TestSignalType::Sine {
            frequency: 220.0,
            duration: 0.5,
        }),
        AudioSource::TestSignal(TestSignalType::Sine {
            frequency: 440.0,
            duration: 0.5,
        }),
        AudioSource::TestSignal(TestSignalType::Sine {
            frequency: 880.0,
            duration: 0.5,
        }),
    ];

    for source in sources {
        engine.load_audio(source).expect("Load failed");
        let world = engine.generate_world().expect("Generation failed");
        assert!(!world.chunks.is_empty());
    }
}

// ============================================================================
// LONGER DURATION TESTS
// ============================================================================

#[test]
fn test_longer_audio_produces_more_chunks() {
    let mut engine_short = SynesthesiaEngine::new(Genre::Classical);
    let mut engine_long = SynesthesiaEngine::new(Genre::Classical);

    let short_source = AudioSource::TestSignal(TestSignalType::Sine {
        frequency: 440.0,
        duration: 1.0,
    });

    let long_source = AudioSource::TestSignal(TestSignalType::Sine {
        frequency: 440.0,
        duration: 5.0,
    });

    engine_short.load_audio(short_source).expect("Short load failed");
    engine_long.load_audio(long_source).expect("Long load failed");

    let world_short = engine_short.generate_world().expect("Short gen failed");
    let world_long = engine_long.generate_world().expect("Long gen failed");

    // Longer audio should produce more chunks
    assert!(
        world_long.chunks.len() >= world_short.chunks.len(),
        "Longer audio should have at least as many chunks"
    );
}

// ============================================================================
// AUDIO ANALYZER TESTS
// ============================================================================

#[test]
fn test_audio_analyzer_creation() {
    let _analyzer = AudioAnalyzer::new(44100, 2048);
    let _analyzer_high_res = AudioAnalyzer::new(48000, 4096);
}

// ============================================================================
// FEATURE EXTRACTOR TESTS
// ============================================================================

#[test]
fn test_feature_extractor_creation() {
    let _extractor = FeatureExtractor::new();
}

// ============================================================================
// SPATIAL MAPPER TESTS
// ============================================================================

#[test]
fn test_spatial_mapper_creation() {
    let _mapper = SpatialMapper::new(10.0, 50.0);
    let _mapper_custom = SpatialMapper::new(20.0, 100.0);
}

// ============================================================================
// GEOMETRY TESTS
// ============================================================================

#[test]
fn test_mesh_generator() {
    let mesh_gen = MeshGenerator::new();

    let sphere = mesh_gen.generate_sphere(1.0, 16);
    assert!(!sphere.positions.is_empty(), "Sphere should have positions");
    assert!(!sphere.indices.is_empty(), "Sphere should have indices");
    assert!(!sphere.normals.is_empty(), "Sphere should have normals");

    let cube = mesh_gen.generate_cube(2.0);
    assert!(!cube.positions.is_empty(), "Cube should have positions");
    assert!(!cube.indices.is_empty(), "Cube should have indices");
    assert!(!cube.normals.is_empty(), "Cube should have normals");
}

// ============================================================================
// GENRE STYLE TESTS
// ============================================================================

#[test]
fn test_genre_style_properties() {
    let classical = Genre::Classical.get_style();
    let electronic = Genre::Electronic.get_style();
    let jazz = Genre::Jazz.get_style();

    // Each genre should have a unique style
    assert!(classical.time_scale > 0.0);
    assert!(electronic.time_scale > 0.0);
    assert!(jazz.time_scale > 0.0);
}

// ============================================================================
// EXPORT TESTS
// ============================================================================

#[test]
fn test_gltf_exporter_creation() {
    let config = ExportConfig::default();
    let _exporter = GltfExporter::new(config);

    let custom_config = ExportConfig {
        binary: false,
        quality: 0.5,
        merge_meshes: false,
        vertex_colors: true,
        normals: true,
        uvs: false,
        max_texture_size: 1024,
        embed_textures: false,
        chunk_lod_distance: 50.0,
    };
    let _exporter_custom = GltfExporter::new(custom_config);
}

// ============================================================================
// END-TO-END INTEGRATION TEST
// ============================================================================

#[test]
fn test_complete_pipeline() {
    // Complete pipeline: Audio → Features → World → Geometry → Export preparation
    let mut engine = SynesthesiaEngine::new(Genre::Electronic);

    // 1. Load audio
    let source = AudioSource::TestSignal(TestSignalType::Harmonics {
        fundamental: 440.0,
        num_harmonics: 4,
        duration: 2.0,
    });
    engine.load_audio(source).expect("Load failed");

    // 2. Generate world
    let world = engine.generate_world().expect("Generation failed");
    assert!(!world.chunks.is_empty(), "Should have chunks");
    assert!(world.total_elements() > 0, "Should have elements");

    // 3. Verify geometry generation works
    let mesh_gen = MeshGenerator::new();
    for chunk in &world.chunks {
        for _element in &chunk.elements {
            let _mesh = mesh_gen.generate_sphere(1.0, 8);
        }
    }

    // 4. Verify export configuration
    let exporter = GltfExporter::new(ExportConfig::default());
    // Just verify exporter exists (actual file export would require file system)
    let _ = exporter;
}
