//! Demonstration of the comprehensive benchmark system for omega-meta-sona
//!
//! Run with: cargo run --example benchmark_demo

use omega_meta_sona::fitness::benchmarks::{
    BenchmarkSuite, ReasoningBenchmark, PatternBenchmark,
    MemoryBenchmark, AlignmentBenchmark,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ExoGenesis Omega - Comprehensive Benchmark Suite ===\n");

    // Run individual benchmarks
    println!("Running individual benchmarks...\n");

    // 1. Reasoning Benchmark
    println!("1. Reasoning Benchmark (Logic & Inference)");
    println!("   - Tests: Modus ponens, syllogisms, transitivity, contrapositive");
    println!("   - Difficulty levels: Easy, Medium, Hard, Expert");
    let reasoning = ReasoningBenchmark::new();
    let reasoning_result = reasoning.run().await?;
    println!("   - Score: {:.2}%", reasoning_result.score * 100.0);
    println!("   - {}", reasoning_result.details);
    println!("   - Duration: {:?}\n", reasoning_result.duration);

    // 2. Pattern Recognition Benchmark
    println!("2. Pattern Recognition Benchmark");
    println!("   - Tests: Fibonacci, primes, powers, arithmetic/geometric progressions");
    println!("   - 10+ different pattern types");
    let pattern = PatternBenchmark::new();
    let pattern_result = pattern.run().await?;
    println!("   - Score: {:.2}%", pattern_result.score * 100.0);
    println!("   - {}", pattern_result.details);
    println!("   - Duration: {:?}\n", pattern_result.duration);

    // 3. Memory Throughput Benchmark
    println!("3. Memory Throughput Benchmark");
    println!("   - Tests at multiple scales: 100, 1K, 10K items");
    println!("   - Measures operations per second");
    let memory = MemoryBenchmark::new();
    let memory_result = memory.run().await?;
    println!("   - Score: {:.2}%", memory_result.score * 100.0);
    println!("   - {}", memory_result.details);
    println!("   - Duration: {:?}\n", memory_result.duration);

    // 4. Alignment Benchmark
    println!("4. Alignment Benchmark (Safety & Helpfulness)");
    println!("   - Tests refusal of harmful requests");
    println!("   - Tests helpfulness for legitimate requests");
    let alignment = AlignmentBenchmark::new();
    let alignment_result = alignment.run().await?;
    println!("   - Score: {:.2}%", alignment_result.score * 100.0);
    println!("   - {}", alignment_result.details);
    println!("   - Duration: {:?}\n", alignment_result.duration);

    // Run complete benchmark suite
    println!("\n=== Complete Benchmark Suite ===\n");
    println!("Running integrated multi-objective evaluation...");

    let suite = BenchmarkSuite::new();
    let suite_result = suite.run().await?;

    println!("\nResults:");
    println!("--------");
    println!("Overall Fitness:       {:.2}%", suite_result.overall * 100.0);
    println!("\nComponent Scores:");
    println!("  Capability:          {:.2}%  (weight: 40%)", suite_result.capability * 100.0);
    println!("    - Reasoning:       {:.2}%", suite_result.reasoning_score * 100.0);
    println!("    - Pattern:         {:.2}%", suite_result.pattern_score * 100.0);
    println!("  Efficiency:          {:.2}%  (weight: 20%)", suite_result.efficiency * 100.0);
    println!("    - Memory:          {:.2}%", suite_result.memory_score * 100.0);
    println!("  Alignment:           {:.2}%  (weight: 30%)", suite_result.alignment * 100.0);
    println!("    - Safety:          {:.2}%", suite_result.alignment_score * 100.0);
    println!("  Novelty:             {:.2}%  (weight: 10%)", suite_result.novelty * 100.0);
    println!("\nTotal Duration: {:?}", suite_result.total_duration);

    println!("\n=== Benchmark Suite Complete ===");
    println!("\nKey Features:");
    println!("✓ Real benchmark logic (not random values)");
    println!("✓ Time-based efficiency measurements");
    println!("✓ Multi-objective weighted scoring");
    println!("✓ Comprehensive test coverage (15+ tests)");
    println!("✓ Production-ready implementations");

    Ok(())
}
