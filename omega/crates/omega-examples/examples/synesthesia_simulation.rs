//! Synesthesia Engine - Comprehensive Simulation Suite
//!
//! Runs multiple simulations across all genres and configurations,
//! generating statistics and comparative analysis.
//!
//! Run with: cargo run --example synesthesia_simulation

use omega_synesthesia::{
    SynesthesiaEngine, AudioSource, TestSignalType, Genre, ElementType,
};
use std::time::Instant;

/// Simulation result for a single run
#[derive(Debug)]
struct SimulationResult {
    genre: Genre,
    duration_secs: f64,
    bpm: f64,
    generation_time_ms: u128,
    total_elements: usize,
    landmarks: usize,
    structures: usize,
    decorations: usize,
    ambient: usize,
    geometry: usize,
    estimated_vertices: usize,
    world_width: f32,
    world_height: f32,
    world_depth: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║         🎵 OMEGA SYNESTHESIA - COMPREHENSIVE SIMULATION SUITE 🎵          ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Run simulations across all genres
    let genres = [
        Genre::Classical,
        Genre::Jazz,
        Genre::Rock,
        Genre::Electronic,
        Genre::Metal,
        Genre::Ambient,
        Genre::HipHop,
        Genre::Folk,
        Genre::Pop,
        Genre::Experimental,
    ];

    // Test configurations
    let configs = [
        (60.0, 10.0, "Slow & Short"),
        (120.0, 10.0, "Medium Tempo"),
        (180.0, 10.0, "Fast Tempo"),
        (120.0, 30.0, "Extended Duration"),
        (140.0, 5.0, "Quick Burst"),
    ];

    let mut all_results: Vec<SimulationResult> = Vec::new();

    // SIMULATION 1: All Genres at Standard Config
    println!("╭─────────────────────────────────────────────────────────────────────────────╮");
    println!("│  SIMULATION 1: Genre Comparison (120 BPM, 10 seconds)                       │");
    println!("╰─────────────────────────────────────────────────────────────────────────────╯");
    println!();

    for genre in &genres {
        let result = run_simulation(*genre, 120.0, 10.0)?;
        print_result_summary(&result);
        all_results.push(result);
    }

    // SIMULATION 2: Tempo Variation
    println!();
    println!("╭─────────────────────────────────────────────────────────────────────────────╮");
    println!("│  SIMULATION 2: Tempo Variation (Electronic genre)                           │");
    println!("╰─────────────────────────────────────────────────────────────────────────────╯");
    println!();

    for (bpm, duration, name) in &configs {
        println!("  Testing: {} ({}BPM, {}s)", name, bpm, duration);
        let result = run_simulation(Genre::Electronic, *bpm, *duration)?;
        print_result_summary(&result);
        all_results.push(result);
    }

    // SIMULATION 3: Extended World Generation
    println!();
    println!("╭─────────────────────────────────────────────────────────────────────────────╮");
    println!("│  SIMULATION 3: Extended World Generation (60 seconds)                       │");
    println!("╰─────────────────────────────────────────────────────────────────────────────╯");
    println!();

    for genre in &[Genre::Classical, Genre::Electronic, Genre::Metal] {
        let result = run_simulation(*genre, 120.0, 60.0)?;
        print_result_summary(&result);
        all_results.push(result);
    }

    // Generate comprehensive statistics
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                    SIMULATION RESULTS SUMMARY                              ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");

    print_statistics(&all_results);

    // Genre comparison table
    println!();
    println!("╭─────────────────────────────────────────────────────────────────────────────╮");
    println!("│                        GENRE COMPARISON TABLE                               │");
    println!("├─────────────────────────────────────────────────────────────────────────────┤");
    println!("│ Genre        │ Elements │ Vertices │ Width   │ Height  │ Gen Time (ms)     │");
    println!("├─────────────────────────────────────────────────────────────────────────────┤");

    for result in all_results.iter().filter(|r| r.duration_secs == 10.0 && r.bpm == 120.0) {
        println!(
            "│ {:12} │ {:8} │ {:8} │ {:7.1} │ {:7.1} │ {:17} │",
            format!("{:?}", result.genre),
            result.total_elements,
            result.estimated_vertices,
            result.world_width,
            result.world_height,
            result.generation_time_ms
        );
    }
    println!("╰─────────────────────────────────────────────────────────────────────────────╯");

    // Element distribution analysis
    println!();
    println!("╭─────────────────────────────────────────────────────────────────────────────╮");
    println!("│                     ELEMENT DISTRIBUTION ANALYSIS                           │");
    println!("├─────────────────────────────────────────────────────────────────────────────┤");

    for result in all_results.iter().filter(|r| r.duration_secs == 10.0 && r.bpm == 120.0) {
        let total = result.total_elements as f32;
        println!(
            "│ {:12} │ L:{:4.1}% S:{:4.1}% D:{:4.1}% A:{:4.1}% G:{:4.1}%",
            format!("{:?}", result.genre),
            result.landmarks as f32 / total * 100.0,
            result.structures as f32 / total * 100.0,
            result.decorations as f32 / total * 100.0,
            result.ambient as f32 / total * 100.0,
            result.geometry as f32 / total * 100.0
        );
    }
    println!("╰─────────────────────────────────────────────────────────────────────────────╯");

    // Performance benchmarks
    println!();
    println!("╭─────────────────────────────────────────────────────────────────────────────╮");
    println!("│                        PERFORMANCE BENCHMARKS                               │");
    println!("├─────────────────────────────────────────────────────────────────────────────┤");

    let total_elements: usize = all_results.iter().map(|r| r.total_elements).sum();
    let total_vertices: usize = all_results.iter().map(|r| r.estimated_vertices).sum();
    let total_time: u128 = all_results.iter().map(|r| r.generation_time_ms).sum();
    let avg_time = total_time / all_results.len() as u128;
    let min_time = all_results.iter().map(|r| r.generation_time_ms).min().unwrap_or(0);
    let max_time = all_results.iter().map(|r| r.generation_time_ms).max().unwrap_or(0);

    println!("│ Total Simulations Run:     {:6}                                         │", all_results.len());
    println!("│ Total Elements Generated:  {:6}                                         │", total_elements);
    println!("│ Total Vertices Generated:  {:6}                                         │", total_vertices);
    println!("│ Average Generation Time:   {:6}ms                                       │", avg_time);
    println!("│ Min Generation Time:       {:6}ms                                       │", min_time);
    println!("│ Max Generation Time:       {:6}ms                                       │", max_time);
    println!("│ Elements per Second:       {:6.0}                                         │",
        total_elements as f64 / (total_time as f64 / 1000.0));
    println!("╰─────────────────────────────────────────────────────────────────────────────╯");

    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                  ALL SIMULATIONS COMPLETED SUCCESSFULLY! 🎉               ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");

    Ok(())
}

fn run_simulation(genre: Genre, bpm: f64, duration: f64) -> Result<SimulationResult, Box<dyn std::error::Error>> {
    let start = Instant::now();

    // Create engine
    let mut engine = SynesthesiaEngine::new(genre);

    // Load simulated music
    engine.load_audio(AudioSource::TestSignal(TestSignalType::SimulatedMusic {
        bpm: bpm as f32,
        duration: duration as f32,
    }))?;

    // Generate world
    let world = engine.generate_world()?;

    let generation_time = start.elapsed().as_millis();

    // Count element types
    let mut landmarks = 0;
    let mut structures = 0;
    let mut ambient = 0;
    let mut decorations = 0;
    let mut geometry = 0;

    for chunk in &world.chunks {
        for element in &chunk.elements {
            match element.element_type {
                ElementType::Landmark => landmarks += 1,
                ElementType::Structure => structures += 1,
                ElementType::Ambient => ambient += 1,
                ElementType::Decoration => decorations += 1,
                ElementType::Geometry => geometry += 1,
                _ => {}
            }
        }
    }

    let dims = world.bounds.dimensions();

    Ok(SimulationResult {
        genre,
        duration_secs: duration,
        bpm,
        generation_time_ms: generation_time,
        total_elements: world.total_elements(),
        landmarks,
        structures,
        decorations,
        ambient,
        geometry,
        estimated_vertices: world.estimated_vertices(),
        world_width: dims.x,
        world_height: dims.y,
        world_depth: dims.z,
    })
}

fn print_result_summary(result: &SimulationResult) {
    println!(
        "  {:12} │ {:5} elements │ ~{:6} verts │ {:3}ms │ {}x{}x{}",
        format!("{:?}", result.genre),
        result.total_elements,
        result.estimated_vertices,
        result.generation_time_ms,
        result.world_width as i32,
        result.world_height as i32,
        result.world_depth as i32
    );
}

fn print_statistics(results: &[SimulationResult]) {
    let total_elements: usize = results.iter().map(|r| r.total_elements).sum();
    let avg_elements = total_elements / results.len();

    let total_vertices: usize = results.iter().map(|r| r.estimated_vertices).sum();
    let avg_vertices = total_vertices / results.len();

    println!("║  Total Simulations: {:5}                                                  ║", results.len());
    println!("║  Total Elements:    {:7}                                                ║", total_elements);
    println!("║  Avg Elements/Run:  {:7}                                                ║", avg_elements);
    println!("║  Total Vertices:    {:7}                                                ║", total_vertices);
    println!("║  Avg Vertices/Run:  {:7}                                                ║", avg_vertices);
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
}
