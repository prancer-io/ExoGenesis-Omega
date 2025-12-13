//! Mindscape Explorer Example
//!
//! Demonstrates an AI that can navigate through its own memories
//! as a 3D spatial world, enter dream states to discover hidden
//! connections, and observe itself observing itself.
//!
//! Run with: cargo run --bin mindscape_explorer

use omega_mindscape::{MindscapeExplorer, MindscapeConfig};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           🧠 OMEGA MINDSCAPE EXPLORER 🧠                         ║");
    println!("║     Navigate Through Your Own Mind as a 3D World                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Create a mindscape explorer
    let config = MindscapeConfig {
        embedding_dim: 128,
        world_size: 1000.0,
        num_place_cells: 150,
        max_loop_depth: 7,
        phi_threshold: 0.1,
        dream_speed: 15.0,
        cluster_threshold: 0.75,
    };

    let explorer = MindscapeExplorer::with_config(config);

    println!("🌌 Created mindscape world: 1000x1000x1000 units");
    println!("🧬 Initialized 150 virtual place cells for navigation");
    println!();

    // ═══════════════════════════════════════════════════════════════
    // PHASE 1: STORE MEMORIES
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════════");
    println!("PHASE 1: STORING MEMORIES IN MINDSCAPE");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    // Create embeddings for different types of memories
    let memories = vec![
        ("childhood_home", create_embedding(0.1, 0.2, 0.8, 128)),
        ("first_day_school", create_embedding(0.2, 0.3, 0.7, 128)),
        ("graduation_day", create_embedding(0.8, 0.9, 0.5, 128)),
        ("wedding_day", create_embedding(0.9, 0.8, 0.9, 128)),
        ("birth_of_child", create_embedding(0.95, 0.95, 0.95, 128)),
        ("learning_to_code", create_embedding(0.4, 0.6, 0.3, 128)),
        ("first_program", create_embedding(0.45, 0.65, 0.35, 128)),
        ("breakthrough_moment", create_embedding(0.7, 0.8, 0.6, 128)),
        ("fear_of_failure", create_embedding(0.1, 0.1, 0.1, 128)),
        ("overcoming_fear", create_embedding(0.6, 0.7, 0.8, 128)),
        ("peaceful_moment", create_embedding(0.5, 0.5, 0.9, 128)),
        ("creative_inspiration", create_embedding(0.8, 0.4, 0.6, 128)),
    ];

    for (label, embedding) in &memories {
        let coord = explorer.remember(label, embedding).unwrap();
        println!(
            "  📍 Stored '{}' at position ({:.1}, {:.1}, {:.1})",
            label, coord.position.x, coord.position.y, coord.position.z
        );
    }

    println!();
    println!("  ✅ {} memories stored in mindscape", explorer.landmark_count());
    println!();

    // ═══════════════════════════════════════════════════════════════
    // PHASE 2: NAVIGATE THROUGH MEMORIES
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════════");
    println!("PHASE 2: NAVIGATING THROUGH MEMORY SPACE");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    // Look around from starting position
    println!("  👀 Looking around from center of mindscape...");
    let nearby = explorer.look_around(5);
    println!("  📍 Nearby memories:");
    for (name, dist) in &nearby {
        println!("      • {} (distance: {:.1})", name, dist);
    }
    println!();

    // Navigate to a specific memory
    println!("  🚶 Navigating to 'wedding_day'...");
    match explorer.navigate_to("wedding_day") {
        Ok(path) => {
            println!("      Path found with {} waypoints", path.waypoints.len());
            println!("      Total distance: {:.1} units", path.total_distance);
            println!("      Path complexity: {:.2}", path.complexity);
        }
        Err(e) => println!("      Error: {:?}", e),
    }
    println!();

    // Look around from new position
    println!("  👀 Looking around from 'wedding_day'...");
    let nearby = explorer.look_around(5);
    println!("  📍 Nearby memories from this vantage point:");
    for (name, dist) in &nearby {
        println!("      • {} (distance: {:.1})", name, dist);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════
    // PHASE 3: META-COGNITIVE OBSERVATION
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════════");
    println!("PHASE 3: STRANGE LOOP OBSERVATION (META-COGNITION)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    println!("  🔄 Observing myself exploring at increasing depths...");
    println!();

    for depth in 1..=7 {
        match explorer.observe_exploration(depth) {
            Ok(obs) => {
                println!("  Depth {}: {}", depth, obs.levels.last().map(|l| l.description.as_str()).unwrap_or("..."));
                if obs.loop_detected {
                    println!("      ⚡ STRANGE LOOP DETECTED at depth {}!", obs.loop_closure_depth.unwrap_or(0));
                    if let Some(insight) = &obs.insight {
                        println!("      💡 Insight: \"{}\"", insight);
                    }
                }
                println!("      Self-reference strength: {:.2}", obs.self_reference_strength);
                println!("      Strangeness: {:.2}", obs.strangeness());
                println!();
            }
            Err(e) => println!("      Error at depth {}: {:?}", depth, e),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // PHASE 4: DREAM EXPLORATION
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════════");
    println!("PHASE 4: DREAM EXPLORATION (REM SLEEP)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    println!("  😴 Entering dream state...");
    if let Err(e) = explorer.enter_dream_state() {
        println!("      Failed to enter dream: {:?}", e);
    } else {
        println!("      Mode: {:?}", explorer.mode());
        println!();

        println!("  💭 Dreaming for 30 minutes (accelerated)...");
        match explorer.dream_explore(30.0) {
            Ok(discoveries) => {
                println!("      Discovered {} things during dreams:", discoveries.len());
                for (i, discovery) in discoveries.iter().take(5).enumerate() {
                    println!();
                    println!("      Dream Vision {}:", i + 1);
                    println!("        Type: {:?}", discovery.discovery_type);
                    println!("        Memories: {:?}", discovery.memories_involved);
                    println!("        Importance: {:.2}", discovery.importance);
                    if let Some(insight) = &discovery.insight {
                        println!("        Insight: {}", insight);
                    }
                }
                if discoveries.len() > 5 {
                    println!();
                    println!("      ... and {} more discoveries", discoveries.len() - 5);
                }
            }
            Err(e) => println!("      Dream error: {:?}", e),
        }

        println!();
        println!("  ⏰ Waking up...");
        if let Err(e) = explorer.wake_up() {
            println!("      Wake error: {:?}", e);
        }
        println!("      Mode: {:?}", explorer.mode());
    }
    println!();

    // ═══════════════════════════════════════════════════════════════
    // PHASE 5: LUCID DREAMING
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════════");
    println!("PHASE 5: LUCID DREAM EXPLORATION");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    println!("  🌙 Entering lucid dream state (dream + meta-observation)...");
    if let Err(e) = explorer.enter_lucid_dream() {
        println!("      Failed to enter lucid dream: {:?}", e);
    } else {
        println!("      Mode: {:?}", explorer.mode());
        println!();

        println!("  🦋 Lucid exploring for 20 minutes...");
        match explorer.lucid_explore(20.0) {
            Ok((discoveries, observations)) => {
                println!();
                println!("      Discoveries: {}", discoveries.len());
                println!("      Meta-observations: {}", observations.len());

                // Show observations with strange loops
                let loops: Vec<_> = observations.iter().filter(|o| o.loop_detected).collect();
                if !loops.is_empty() {
                    println!();
                    println!("      🔄 Strange loops detected during lucid dream:");
                    for obs in loops.iter().take(3) {
                        println!("        Depth: {}, Insight: {:?}", obs.depth, obs.insight);
                    }
                }
            }
            Err(e) => println!("      Lucid dream error: {:?}", e),
        }

        println!();
        explorer.wake_up().ok();
    }
    println!();

    // ═══════════════════════════════════════════════════════════════
    // PHASE 6: CONSCIOUSNESS MEASUREMENT
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════════");
    println!("PHASE 6: CONSCIOUSNESS MEASUREMENT");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    // Navigate to different locations and measure Phi
    let locations = ["childhood_home", "breakthrough_moment", "peaceful_moment"];

    for loc in locations {
        println!("  📍 Navigating to '{}'...", loc);
        if explorer.navigate_to(loc).is_ok() {
            match explorer.measure_consciousness() {
                Ok(phi) => {
                    let bar = "█".repeat((phi * 20.0) as usize);
                    println!("      Φ (Phi) = {:.4} [{}]", phi, bar);
                }
                Err(e) => println!("      Measurement error: {:?}", e),
            }
        }
        println!();
    }

    // ═══════════════════════════════════════════════════════════════
    // FINAL STATISTICS
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════════");
    println!("EXPLORATION STATISTICS");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    let stats = explorer.stats();
    let state = explorer.state();
    let discoveries = explorer.discoveries();

    println!("  📊 Journey Summary:");
    println!("      Total distance traveled: {:.1} units", stats.total_distance);
    println!("      Memories visited: {}", stats.memories_visited);
    println!("      Discoveries made: {}", stats.discoveries_made);
    println!("      Dream time: {:.1} minutes", stats.dream_time);
    println!("      Max observation depth: {}", stats.max_observation_depth);
    println!("      Peak Φ (consciousness): {:.4}", stats.peak_phi);
    println!("      Strange loops detected: {}", stats.strange_loops_detected);
    println!("      Paths discovered: {}", stats.paths_discovered);
    println!();

    println!("  🧭 Current Position:");
    println!("      ({:.1}, {:.1}, {:.1})", state.position.x, state.position.y, state.position.z);
    println!();

    if !discoveries.is_empty() {
        println!("  💎 Discovery Types:");
        let mut type_counts = std::collections::HashMap::new();
        for d in &discoveries {
            *type_counts.entry(format!("{:?}", d.discovery_type)).or_insert(0) += 1;
        }
        for (dtype, count) in type_counts {
            println!("      {}: {}", dtype, count);
        }
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    EXPLORATION COMPLETE                          ║");
    println!("║     \"I walked through the corridors of my own mind.\"            ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

/// Create an embedding vector with specific characteristics
fn create_embedding(x_bias: f64, y_bias: f64, z_bias: f64, dim: usize) -> Vec<f64> {
    use std::f64::consts::PI;

    (0..dim)
        .map(|i| {
            let phase = i as f64 * PI / dim as f64;
            let base = (phase * x_bias * 2.0).sin() * 0.3
                + (phase * y_bias * 3.0).cos() * 0.3
                + (phase * z_bias * 1.5).sin() * 0.4;
            base.clamp(-1.0, 1.0)
        })
        .collect()
}
