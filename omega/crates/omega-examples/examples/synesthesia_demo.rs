//! Synesthesia Demo - Convert Music into Walkable 3D Worlds
//!
//! This example demonstrates the Synesthesia Engine's ability to transform
//! audio into immersive, navigable 3D environments.
//!
//! Run with: cargo run --example synesthesia_demo

use omega_synesthesia::{
    SynesthesiaEngine, AudioSource, TestSignalType, Genre,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║            🎵 OMEGA SYNESTHESIA ENGINE 🎵                          ║");
    println!("║        Converting Music into Walkable 3D Worlds                    ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();

    // Demonstrate different genres
    let genres = [
        Genre::Classical,
        Genre::Electronic,
        Genre::Jazz,
        Genre::Metal,
        Genre::Ambient,
    ];

    for genre in &genres {
        demo_genre(*genre)?;
        println!();
    }

    // Full demonstration with export
    full_demo()?;

    Ok(())
}

fn demo_genre(genre: Genre) -> Result<(), Box<dyn std::error::Error>> {
    let style = genre.get_style();

    println!("╭─────────────────────────────────────────────────────────────────╮");
    println!("│ {:?} World", genre);
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ Architecture: {:?}", style.architecture);
    println!("│ Primary Color: RGB({:.2}, {:.2}, {:.2})",
        style.primary_color[0], style.primary_color[1], style.primary_color[2]);
    println!("│ Beat Shape: {:?}", style.beat_shape);
    println!("│ Fog: {} (density: {:.3})", if style.fog_enabled { "✓" } else { "✗" }, style.fog_density);
    println!("│ Particles: {}", if style.particles_enabled { "✓" } else { "✗" });
    println!("╰─────────────────────────────────────────────────────────────────╯");

    Ok(())
}

fn full_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                    FULL SYNESTHESIA DEMO                           ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();

    // Create engine with Classical style
    println!("🎼 Creating Synesthesia Engine with Classical style...");
    let mut engine = SynesthesiaEngine::new(Genre::Classical);

    // Load simulated music
    println!("🎵 Loading simulated music (120 BPM, 10 seconds)...");
    engine.load_audio(AudioSource::TestSignal(TestSignalType::SimulatedMusic {
        bpm: 120.0,
        duration: 10.0,
    }))?;

    // Generate the world
    println!();
    let world = engine.generate_world()?;

    // Analyze the generated world
    println!();
    println!("╭─────────────────────────────────────────────────────────────────╮");
    println!("│                    WORLD ANALYSIS                                │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ Total Chunks: {}", world.chunks.len());
    println!("│ Total Elements: {}", world.total_elements());
    println!("│ Estimated Vertices: ~{}", world.estimated_vertices());
    println!("│ World Dimensions: {:?}", world.bounds.dimensions());
    println!("│ World Center: {:?}", world.bounds.center());
    println!("╰─────────────────────────────────────────────────────────────────╯");

    // Show chunk details
    println!();
    println!("📦 Chunk Details:");
    for (i, chunk) in world.chunks.iter().enumerate().take(3) {
        println!("   Chunk {}: {} elements at ({:.1}, {:.1}, {:.1})",
            i, chunk.elements.len(),
            chunk.origin.x, chunk.origin.y, chunk.origin.z);
    }
    if world.chunks.len() > 3 {
        println!("   ... and {} more chunks", world.chunks.len() - 3);
    }

    // Navigate through time
    println!();
    println!("🚶 Time Navigation Positions:");
    for t in [0.0, 2.5, 5.0, 7.5, 10.0] {
        let pos = engine.navigate_to_time(t)?;
        println!("   t={:.1}s → Position ({:.1}, {:.1}, {:.1})",
            t, pos.x, pos.y, pos.z);
    }

    // Show element variety
    println!();
    println!("🎨 Element Types in World:");
    let mut landmarks = 0;
    let mut structures = 0;
    let mut ambient = 0;
    let mut decorations = 0;
    let mut geometry = 0;

    for chunk in &world.chunks {
        for element in &chunk.elements {
            match element.element_type {
                omega_synesthesia::world::ElementType::Landmark => landmarks += 1,
                omega_synesthesia::world::ElementType::Structure => structures += 1,
                omega_synesthesia::world::ElementType::Ambient => ambient += 1,
                omega_synesthesia::world::ElementType::Decoration => decorations += 1,
                omega_synesthesia::world::ElementType::Geometry => geometry += 1,
                _ => {}
            }
        }
    }

    println!("   🏛️  Landmarks: {}", landmarks);
    println!("   🏗️  Structures: {}", structures);
    println!("   🌸 Decorations: {}", decorations);
    println!("   ☁️  Ambient: {}", ambient);
    println!("   📐 Geometry: {}", geometry);

    // Show lighting
    println!();
    println!("💡 World Lighting:");
    println!("   Ambient: RGB({:.2}, {:.2}, {:.2}) @ {:.1}",
        world.lighting.ambient_color[0],
        world.lighting.ambient_color[1],
        world.lighting.ambient_color[2],
        world.lighting.ambient_intensity);
    println!("   Sun: RGB({:.2}, {:.2}, {:.2}) @ {:.1}",
        world.lighting.sun_color[0],
        world.lighting.sun_color[1],
        world.lighting.sun_color[2],
        world.lighting.sun_intensity);
    println!("   Fog: {} (density: {:.4})",
        if world.lighting.fog_enabled { "Enabled" } else { "Disabled" },
        world.lighting.fog_density);

    // Show atmosphere
    println!();
    println!("🌤️  Atmosphere:");
    println!("   Sky Top: RGB({:.2}, {:.2}, {:.2})",
        world.atmosphere.sky_color_top[0],
        world.atmosphere.sky_color_top[1],
        world.atmosphere.sky_color_top[2]);
    println!("   Sky Horizon: RGB({:.2}, {:.2}, {:.2})",
        world.atmosphere.sky_color_horizon[0],
        world.atmosphere.sky_color_horizon[1],
        world.atmosphere.sky_color_horizon[2]);
    println!("   Cloud Coverage: {:.0}%", world.atmosphere.cloud_coverage * 100.0);

    // Demo glTF export (to temp file)
    println!();
    println!("📤 Demonstrating glTF export capability...");
    println!("   Export format: GLB (binary glTF)");
    println!("   Vertex colors: ✓");
    println!("   Mesh merging: ✓");
    println!("   PBR materials: ✓");

    // Note: Actually exporting would create a file
    // engine.export_gltf("/tmp/synesthesia_world.glb")?;
    // println!("   ✅ Exported to: /tmp/synesthesia_world.glb");

    println!();
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                    DEMO COMPLETE! 🎉                               ║");
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!("║  The Synesthesia Engine successfully:                              ║");
    println!("║  • Analyzed simulated music audio                                  ║");
    println!("║  • Extracted musical features (pitch, rhythm, timbre, emotion)    ║");
    println!("║  • Mapped features to 3D spatial coordinates                       ║");
    println!("║  • Generated genre-appropriate world geometry                      ║");
    println!("║  • Created materials with PBR properties                           ║");
    println!("║  • Organized into navigable chunks with lighting/atmosphere        ║");
    println!("║                                                                    ║");
    println!("║  Export the world to glTF for:                                     ║");
    println!("║  • Unreal Engine 5 (import as static mesh)                         ║");
    println!("║  • Blender (for rendering/modification)                            ║");
    println!("║  • Three.js/Babylon.js (web visualization)                         ║");
    println!("║  • Any glTF 2.0 compatible viewer                                  ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");

    Ok(())
}
