//! Example demonstrating the Omega Temporal Loops system

use omega_loops::LoopEngine;
use omega_core::{LoopType, CycleInput};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== Omega Temporal Loops System Demo ===\n");

    // Create and initialize the loop engine
    let mut engine = LoopEngine::new();
    println!("Created Loop Engine");

    engine.initialize().await?;
    println!("Initialized {} temporal loops\n", omega_core::LoopType::all_loops().len());

    // Display all loop types
    println!("Temporal Loops:");
    for loop_type in LoopType::all_loops() {
        println!("  - {:?} ({}): {}",
            loop_type,
            format_duration(loop_type.cycle_duration()),
            loop_type.description()
        );
    }
    println!();

    // Execute a cycle in the Reflexive loop (fastest)
    println!("Executing Reflexive loop cycle...");
    let input = CycleInput {
        data: HashMap::from([
            ("sensory_input".to_string(), serde_json::json!("Test data")),
        ]),
        context: "Demo sensory input".to_string(),
        objectives: vec!["Process sensory data".to_string()],
    };

    let output = engine.execute_cycle(LoopType::Reflexive, input).await?;
    println!("Reflexive cycle completed:");
    println!("  Results: {} items", output.results.len());
    println!("  Insights: {}", output.insights.join(", "));
    println!("  Actions: {}", output.actions.len());
    println!();

    // Execute a cycle in the Cognitive loop
    println!("Executing Adaptive loop cycle...");
    let input2 = CycleInput {
        data: HashMap::from([
            ("learning_data".to_string(), serde_json::json!({"experiences": 10})),
        ]),
        context: "Demo learning session".to_string(),
        objectives: vec!["Adapt behavior".to_string()],
    };

    let output2 = engine.execute_cycle(LoopType::Adaptive, input2).await?;
    println!("Adaptive cycle completed:");
    println!("  Results: {} items", output2.results.len());
    println!("  Insights: {}", output2.insights.join(", "));
    println!();

    // Get statistics
    println!("Loop Statistics:");
    let stats = engine.get_stats().await;
    for (loop_type, loop_stats) in stats {
        println!("  {:?}: {} cycles completed, {:.2}% success rate",
            loop_type,
            loop_stats.cycles_completed,
            loop_stats.success_rate * 100.0
        );
    }
    println!();

    // Shutdown
    engine.shutdown().await?;
    println!("Loop Engine shut down successfully");

    Ok(())
}

fn format_duration(duration: chrono::Duration) -> String {
    if duration.num_milliseconds() < 1000 {
        format!("{}ms", duration.num_milliseconds())
    } else if duration.num_seconds() < 60 {
        format!("{}s", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!("{}min", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h", duration.num_hours())
    } else if duration.num_days() < 365 {
        format!("{}d", duration.num_days())
    } else {
        format!("{}y", duration.num_days() / 365)
    }
}
