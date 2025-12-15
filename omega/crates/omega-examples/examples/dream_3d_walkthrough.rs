//! 3D Dream Walkthrough Example
//!
//! This example demonstrates the NEW 3D walkable dream functionality.
//! It solves the benzene problem and then lets you explore the dream world
//! in 3D space, walking between concepts and following associations.

use omega_examples::dream_problem_solver::{DreamProblemSolver, benzene_problem};

fn main() {
    println!("=== 3D Dream Exploration Demo ===\n");

    // Create the benzene problem
    let problem = benzene_problem();
    println!("Problem: {}", problem.description);
    println!("Elements: {:?}\n", problem.elements.iter().map(|e| &e.name).collect::<Vec<_>>());

    // Create solver and generate 3D dreams
    let mut solver = DreamProblemSolver::new();
    println!("Phase 1: Generating 3D dream worlds...\n");
    let result = solver.solve_with_3d_dreams(&problem, 3);

    println!("Generated {} dreams with 3D representations\n", result.dreams.len());

    // Explore the first dream
    println!("=== Exploring Dream #1 in 3D ===\n");
    let mut explorer = solver.explore_dream_world(0)
        .expect("Failed to create dream explorer");

    // List all concepts in the dream (collect to owned strings to avoid borrow issues)
    let concepts: Vec<String> = explorer.list_concepts().iter().map(|s| s.to_string()).collect();
    println!("Concepts in this dream:");
    for concept in &concepts {
        println!("  - {}", concept);
    }
    println!();

    // Walk to the first concept if available
    if let Some(first_concept) = concepts.first() {
        println!("Walking to concept: '{}'", first_concept);
        match explorer.walk_to(&first_concept) {
            Ok(path) => {
                println!("  ✓ Walked {} meters", path.distance);
                println!("  ✓ Path had {} waypoints", path.waypoints.len());
            }
            Err(e) => println!("  ✗ Failed to walk: {}", e),
        }
        println!();

        // Look around
        println!("Looking around within 20 meters...");
        let nearby = explorer.look_around(20.0);
        if nearby.is_empty() {
            println!("  No nearby concepts found");
        } else {
            println!("  Found {} nearby concepts:", nearby.len());
            for (concept, distance) in nearby.iter().take(5) {
                println!("    - {} ({:.2}m away)", concept, distance);
            }
        }
        println!();
    }

    // Try to follow an association if we have at least 2 concepts
    if concepts.len() >= 2 {
        let from = &concepts[0];
        let to = &concepts[1];

        println!("Attempting to follow association: {} → {}", from, to);
        match explorer.follow_association(from, to) {
            Ok(journey) => {
                println!("  ✓ Successfully followed association!");
                println!("    Connection type: {:?}", journey.connection_type);
                println!("    Strength: {:.2}", journey.strength);
                println!("    Path waypoints: {}", journey.path.len());
            }
            Err(e) => println!("  ✗ No direct association found: {}", e),
        }
        println!();
    }

    // Display insights discovered
    println!("=== Insights Discovered ===\n");
    if result.insights.is_empty() {
        println!("No insights extracted from dreams");
    } else {
        println!("Found {} insights:", result.insights.len());
        for (i, insight) in result.insights.iter().take(5).enumerate() {
            println!("  {}. {} ↔ {} (confidence: {:.2}, bizarreness: {:.2})",
                i + 1,
                insight.association.from,
                insight.association.to,
                insight.confidence,
                insight.bizarreness
            );
        }
    }
    println!();

    // Display solution if found
    if let Some(solution) = &result.solution {
        println!("=== Solution Found ===\n");
        println!("{}", solution.description);
        println!("\nNovelty: {:.2}", solution.novelty);
        println!("Feasibility: {:.2}", solution.feasibility);
        println!("Confidence: {:.2}", solution.confidence);
    } else {
        println!("=== No Solution Synthesized ===");
    }

    println!("\n=== Dream Statistics ===");
    println!("Total sleep cycles: {}", result.total_sleep_cycles);
    println!("Dreams generated: {}", result.dreams.len());
    println!("3D worlds created: {}", result.dream_worlds.len());
    println!("Total insights: {}", result.insights.len());
}
