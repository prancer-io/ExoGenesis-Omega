//! Synesthesia Visualization Generator
//!
//! Creates actual 3D visualizations exported as glTF files

use omega_synesthesia::{
    SynesthesiaEngine, AudioSource, TestSignalType, Genre,
    ExportConfig, GltfExporter,
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║       🎨 OMEGA SYNESTHESIA - 3D VISUALIZATION GENERATOR 🎨                ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Create output directory
    let output_dir = "/tmp/synesthesia_visualizations";
    fs::create_dir_all(output_dir)?;
    println!("📁 Output directory: {}", output_dir);
    println!();

    // Generate visualizations for multiple genres
    let visualizations = [
        (Genre::Classical, 120.0, 15.0, "classical_cathedral"),
        (Genre::Electronic, 140.0, 20.0, "electronic_grid"),
        (Genre::Jazz, 100.0, 15.0, "jazz_club"),
        (Genre::Metal, 160.0, 15.0, "metal_spires"),
        (Genre::Ambient, 80.0, 30.0, "ambient_void"),
    ];

    for (genre, bpm, duration, name) in &visualizations {
        println!("═══════════════════════════════════════════════════════════════════════════");
        println!("🎵 Generating: {} ({:?} @ {} BPM, {}s)", name, genre, bpm, duration);
        println!("═══════════════════════════════════════════════════════════════════════════");

        generate_visualization(*genre, *bpm, *duration, name, output_dir)?;
        println!();
    }

    // Print summary
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                    VISUALIZATION GENERATION COMPLETE! 🎉                  ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                           ║");
    println!("║  Generated files in: {}               ║", output_dir);
    println!("║                                                                           ║");
    println!("║  View your 3D worlds with:                                                ║");
    println!("║  • https://gltf-viewer.donmccurdy.com/ (drag & drop GLB file)            ║");
    println!("║  • https://modelviewer.dev/editor/ (Google's viewer)                      ║");
    println!("║  • Blender (File > Import > glTF 2.0)                                     ║");
    println!("║  • Unreal Engine 5 (drag into Content Browser)                            ║");
    println!("║  • Three.js / Babylon.js (for web integration)                            ║");
    println!("║                                                                           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");

    // List generated files
    println!();
    println!("📦 Generated Files:");
    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let size_kb = metadata.len() / 1024;
        println!("   {} ({} KB)", entry.file_name().to_string_lossy(), size_kb);
    }

    Ok(())
}

fn generate_visualization(
    genre: Genre,
    bpm: f32,
    duration: f32,
    name: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create engine
    let mut engine = SynesthesiaEngine::new(genre);

    // Load simulated music
    engine.load_audio(AudioSource::TestSignal(TestSignalType::SimulatedMusic {
        bpm,
        duration,
    }))?;

    // Generate the world
    let world = engine.generate_world()?;

    // Print world stats
    println!();
    println!("   📊 World Statistics:");
    println!("      • Chunks: {}", world.chunks.len());
    println!("      • Elements: {}", world.total_elements());
    println!("      • Estimated Vertices: ~{}", world.estimated_vertices());
    println!("      • Dimensions: {:.1} x {:.1} x {:.1}",
        world.bounds.dimensions().x,
        world.bounds.dimensions().y,
        world.bounds.dimensions().z);

    // Configure export
    let export_config = ExportConfig {
        binary: true,  // GLB format
        quality: 1.0,
        merge_meshes: true,
        vertex_colors: true,
        normals: true,
        uvs: true,
        max_texture_size: 2048,
        embed_textures: true,
        chunk_lod_distance: 100.0,
    };

    // Export to glTF
    let output_path = format!("{}/{}.glb", output_dir, name);
    let exporter = GltfExporter::new(export_config);
    exporter.export(&world, &output_path)?;

    println!("   ✅ Exported: {}", output_path);

    // Also export scene info as JSON
    let info_path = format!("{}/{}_info.json", output_dir, name);
    let info = serde_json::json!({
        "name": name,
        "genre": format!("{:?}", genre),
        "bpm": bpm,
        "duration_seconds": duration,
        "chunks": world.chunks.len(),
        "total_elements": world.total_elements(),
        "estimated_vertices": world.estimated_vertices(),
        "bounds": {
            "min": [world.bounds.min.x, world.bounds.min.y, world.bounds.min.z],
            "max": [world.bounds.max.x, world.bounds.max.y, world.bounds.max.z],
            "dimensions": [
                world.bounds.dimensions().x,
                world.bounds.dimensions().y,
                world.bounds.dimensions().z
            ]
        },
        "lighting": {
            "ambient_color": world.lighting.ambient_color,
            "ambient_intensity": world.lighting.ambient_intensity,
            "sun_color": world.lighting.sun_color,
            "sun_intensity": world.lighting.sun_intensity,
            "fog_enabled": world.lighting.fog_enabled,
            "fog_color": world.lighting.fog_color,
            "fog_density": world.lighting.fog_density
        },
        "atmosphere": {
            "sky_color_top": world.atmosphere.sky_color_top,
            "sky_color_horizon": world.atmosphere.sky_color_horizon,
            "cloud_coverage": world.atmosphere.cloud_coverage,
            "particle_density": world.atmosphere.particle_density
        }
    });
    fs::write(&info_path, serde_json::to_string_pretty(&info)?)?;
    println!("   📄 Info: {}", info_path);

    Ok(())
}
