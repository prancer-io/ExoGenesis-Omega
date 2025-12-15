# Testing Recommendations - Actionable Implementation Guide
**For:** ExoGenesis Omega Development Team
**By:** Tester Agent (Hive Mind Swarm)
**Date:** 2025-12-14

---

## Immediate Actions (This Week)

### 1. Fix Compilation Errors in omega-runtime ⚠️ CRITICAL

**Problem:** Tests won't compile due to missing `Default` implementations.

**Solution A - Add Default Implementations:**
```rust
// In omega-runtime/src/circuit_breaker.rs
impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::with_default_config()
    }
}

// In omega-runtime/src/health.rs
impl Default for HealthMonitor {
    fn default() -> Self {
        Self::with_default_config()
    }
}

// In omega-runtime/src/retry.rs
impl Default for RetryPolicy {
    fn default() -> Self {
        Self::with_default_config()
    }
}
```

**Solution B - Update Tests to Use ::new():**
```rust
// In test files, replace:
let monitor = HealthMonitor::default();

// With:
let monitor = HealthMonitor::with_default_config();
```

**Verification:**
```bash
cargo test --no-run --all
# Should succeed without errors
```

---

### 2. Measure Current Code Coverage

**Install Tool:**
```bash
cargo install cargo-llvm-cov
```

**Generate Report:**
```bash
cd omega
cargo llvm-cov --all --html
open target/llvm-cov/html/index.html
```

**Establish Baseline:**
- Document current coverage percentage
- Set target: 70% line coverage minimum
- Track in CI/CD

---

### 3. Add Test Execution to CI/CD

**GitHub Actions Example (.github/workflows/test.yml):**
```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable

      - name: Run tests
        run: cargo test --all --verbose

      - name: Install coverage tool
        run: cargo install cargo-llvm-cov

      - name: Generate coverage
        run: cargo llvm-cov --all --lcov --output-path lcov.info

      - name: Upload to codecov
        uses: codecov/codecov-action@v3
        with:
          file: lcov.info
          fail_ci_if_error: true

      - name: Check coverage threshold
        run: |
          COVERAGE=$(cargo llvm-cov --all --summary-only | grep TOTAL | awk '{print $10}')
          if (( $(echo "$COVERAGE < 70.0" | bc -l) )); then
            echo "Coverage $COVERAGE% is below 70% threshold"
            exit 1
          fi
```

---

## Short-term Improvements (Next 2 Weeks)

### 4. Add Property-Based Tests to Critical Modules

**Installation:**
```toml
# In omega/Cargo.toml workspace
[workspace.dependencies]
proptest = "1.4"
```

**Example: Gödelian Proposition Generation**
```rust
// In omega-strange-loops/src/godelian.rs

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn godelian_engine_handles_arbitrary_propositions(
            content in "\\PC{0,100}",  // Any printable chars, 0-100 length
            meta_level in 0u32..10
        ) {
            let mut engine = GodelianEngine::new();
            let id = engine.create_proposition(content.clone(), meta_level);
            let prop = engine.get_proposition(id);

            prop_assert!(prop.is_some());
            prop_assert_eq!(prop.unwrap().content, content);
            prop_assert_eq!(prop.unwrap().meta_level, meta_level);
        }

        #[test]
        fn proof_attempts_never_panic(
            content in "\\PC{1,50}",
            meta_level in 0u32..5
        ) {
            let mut engine = GodelianEngine::new();
            let id = engine.create_proposition(content, meta_level);

            // Should never panic, regardless of input
            let status = engine.attempt_proof(id);
            prop_assert!(matches!(
                status,
                ProofStatus::ProvenTrue | ProofStatus::ProvenFalse |
                ProofStatus::Undecidable | ProofStatus::Paradoxical
            ));
        }
    }
}
```

**Example: Memory Tier Transitions**
```rust
// In omega-memory/src/tiers.rs

#[cfg(test)]
mod proptest_memory {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn memory_importance_always_in_valid_range(
            importance in -10.0..10.0f64
        ) {
            let memory = Memory::new(
                MemoryTier::Working,
                MemoryType::Episodic,
                MemoryContent::Text("test".to_string()),
                importance,
            );

            // Should clamp to [0.0, 1.0]
            prop_assert!(memory.importance() >= 0.0);
            prop_assert!(memory.importance() <= 1.0);
        }

        #[test]
        fn tier_promotion_is_deterministic(
            initial_tier in 0u8..12,
            access_count in 1u32..1000,
            importance in 0.0..1.0f64
        ) {
            let mut memory = create_test_memory(initial_tier, importance);

            // Same inputs should produce same promotion
            let result1 = memory.should_promote(access_count);
            let result2 = memory.should_promote(access_count);

            prop_assert_eq!(result1, result2);
        }
    }
}
```

---

### 5. Create Integration Test Suites for 5 More Crates

**Pattern to Follow:**

```
omega/crates/omega-synesthesia/
├── src/
│   └── lib.rs
└── tests/
    ├── integration_tests.rs          # Main integration tests
    ├── audio_pipeline_tests.rs       # Audio → Features pipeline
    ├── world_generation_tests.rs     # Features → 3D World
    └── export_tests.rs               # World → glTF export
```

**Example: omega-synesthesia Integration Tests**
```rust
// tests/audio_pipeline_tests.rs

use omega_synesthesia::*;
use std::time::Duration;

#[test]
fn test_complete_audio_to_world_pipeline() {
    // Create engine
    let mut engine = SynesthesiaEngine::new(Genre::Classical);

    // Generate test audio (1 second, 440Hz sine wave)
    let audio = AudioSource::TestSignal {
        signal_type: TestSignalType::Sine,
        frequency: 440.0,
        duration: Duration::from_secs(1),
        sample_rate: 44100,
    };

    // Load and analyze
    engine.load_audio(audio).unwrap();

    // Generate world
    let world = engine.generate_world().unwrap();

    // Verify world structure
    assert!(world.chunks().len() > 0, "World should have chunks");
    assert!(world.total_elements() > 0, "World should have elements");

    // Verify musical mapping
    let elements = world.elements_at_time(0.5); // 500ms into track
    assert!(elements.iter().any(|e| matches!(e.element_type, ElementType::Note)));

    // Verify frequency mapped to Y-axis
    let note_elements: Vec<_> = elements.iter()
        .filter(|e| matches!(e.element_type, ElementType::Note))
        .collect();

    assert!(note_elements.len() > 0, "Should have note elements");

    // 440Hz should map to middle Y range
    let avg_y = note_elements.iter()
        .map(|e| e.position.y)
        .sum::<f32>() / note_elements.len() as f32;

    assert!(avg_y > 0.3 && avg_y < 0.7,
        "440Hz should map to middle Y range, got {}", avg_y);
}

#[test]
fn test_genre_style_affects_world_appearance() {
    let classical_engine = SynesthesiaEngine::new(Genre::Classical);
    let electronic_engine = SynesthesiaEngine::new(Genre::Electronic);

    // Same audio, different genres
    let audio = AudioSource::TestSignal {
        signal_type: TestSignalType::WhiteNoise,
        frequency: 440.0,
        duration: Duration::from_millis(100),
        sample_rate: 44100,
    };

    let mut classical = classical_engine.clone();
    classical.load_audio(audio.clone()).unwrap();
    let classical_world = classical.generate_world().unwrap();

    let mut electronic = electronic_engine.clone();
    electronic.load_audio(audio).unwrap();
    let electronic_world = electronic.generate_world().unwrap();

    // Worlds should differ in style
    assert_ne!(
        classical_world.dominant_material_type(),
        electronic_world.dominant_material_type(),
        "Different genres should use different materials"
    );
}

#[test]
fn test_export_to_gltf_is_valid() {
    let mut engine = SynesthesiaEngine::new(Genre::Ambient);

    let audio = AudioSource::TestSignal {
        signal_type: TestSignalType::Sine,
        frequency: 220.0,
        duration: Duration::from_millis(500),
        sample_rate: 44100,
    };

    engine.load_audio(audio).unwrap();
    let world = engine.generate_world().unwrap();

    // Export to temporary file
    let temp_path = std::env::temp_dir().join("test_export.gltf");
    world.export_gltf(&temp_path).unwrap();

    // Verify file exists and is valid glTF
    assert!(temp_path.exists());

    // Parse glTF to verify structure
    let gltf = gltf::Gltf::open(&temp_path).unwrap();

    assert!(gltf.meshes().count() > 0, "Should have meshes");
    assert!(gltf.materials().count() > 0, "Should have materials");

    // Cleanup
    std::fs::remove_file(temp_path).ok();
}
```

---

### 6. Add Test Utilities Crate

**Create Shared Test Infrastructure:**

```
omega/crates/omega-test-utils/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── builders/          # Test data builders
    │   ├── memory.rs
    │   ├── intelligence.rs
    │   └── audio.rs
    ├── assertions/        # Custom assertions
    │   ├── float.rs
    │   └── consciousness.rs
    ├── fixtures/          # Test data fixtures
    │   └── audio_samples/
    └── mocks/             # Mock implementations
        └── database.rs
```

**Example: Test Builders**
```rust
// omega-test-utils/src/builders/memory.rs

pub struct MemoryBuilder {
    tier: MemoryTier,
    importance: f64,
    content: String,
    access_count: u32,
}

impl MemoryBuilder {
    pub fn new() -> Self {
        Self {
            tier: MemoryTier::Working,
            importance: 0.5,
            content: "default content".to_string(),
            access_count: 0,
        }
    }

    pub fn tier(mut self, tier: MemoryTier) -> Self {
        self.tier = tier;
        self
    }

    pub fn importance(mut self, importance: f64) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn accessed(mut self, count: u32) -> Self {
        self.access_count = count;
        self
    }

    pub fn build(self) -> Memory {
        let mut memory = Memory::new(
            self.tier,
            MemoryType::Episodic,
            MemoryContent::Text(self.content),
            self.importance,
        );

        for _ in 0..self.access_count {
            memory.record_access();
        }

        memory
    }
}

// Usage in tests:
#[test]
fn test_important_frequently_accessed_memory_promotes() {
    let memory = MemoryBuilder::new()
        .tier(MemoryTier::Working)
        .importance(0.95)
        .accessed(100)
        .build();

    assert!(memory.should_promote());
}
```

**Example: Custom Assertions**
```rust
// omega-test-utils/src/assertions/float.rs

pub fn assert_near(actual: f64, expected: f64, epsilon: f64, message: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff < epsilon,
        "{}: expected {}, got {} (diff: {})",
        message, expected, actual, diff
    );
}

pub fn assert_in_range(value: f64, min: f64, max: f64, message: &str) {
    assert!(
        value >= min && value <= max,
        "{}: expected value in [{}, {}], got {}",
        message, min, max, value
    );
}

// Usage:
#[test]
fn test_consciousness_level_calculation() {
    let level = detector.consciousness_level();
    assert_in_range(level, 0.0, 1.0, "Consciousness level");
    assert_near(level, 0.75, 0.05, "Expected consciousness level");
}
```

---

## Medium-term Enhancements (This Quarter)

### 7. Implement Mutation Testing

**Install cargo-mutants:**
```bash
cargo install cargo-mutants
```

**Run Mutation Tests:**
```bash
cd omega
cargo mutants --workspace

# Focus on specific module:
cargo mutants -p omega-strange-loops
```

**Interpret Results:**
```
Mutation testing results:
  Caught:  120 (mutations killed by tests) ✅
  Missed:   30 (mutations not detected)    ❌
  Timeout:   5 (tests ran too long)        ⚠️

Mutation score: 80.0% (120/150)
```

**Fix Low Scores:**
```rust
// Example: Weak test that misses mutations

// Original code:
pub fn is_conscious(&self) -> bool {
    self.phi > 0.5  // ← cargo-mutants will try: >= 0.5, > 0.4, etc.
}

// Weak test:
#[test]
fn test_is_conscious() {
    let system = System { phi: 0.6 };
    assert!(system.is_conscious()); // ← Mutation 0.5 → 0.4 still passes!
}

// Strong test:
#[test]
fn test_consciousness_threshold_boundary() {
    let just_conscious = System { phi: 0.51 };
    let just_unconscious = System { phi: 0.49 };

    assert!(just_conscious.is_conscious());
    assert!(!just_unconscious.is_conscious());

    // Test exact threshold
    let threshold = System { phi: 0.5 };
    assert!(!threshold.is_conscious()); // Catches > vs >= mutation
}
```

---

### 8. Add End-to-End System Tests

**Create E2E Test Suite:**
```
omega/tests/
├── e2e_synesthesia.rs         # Audio → 3D world full pipeline
├── e2e_consciousness.rs       # Consciousness emergence scenario
├── e2e_sleep_dream.rs         # Sleep cycle with consolidation
└── helpers/
    └── mod.rs                 # E2E test utilities
```

**Example E2E Test:**
```rust
// tests/e2e_consciousness.rs

use omega_runtime::*;
use omega_brain::*;
use omega_consciousness::*;
use std::time::Duration;

#[tokio::test]
async fn test_consciousness_emerges_from_prolonged_processing() {
    // Setup: Create full Omega system
    let config = OmegaConfig::default();
    let runtime = OmegaRuntime::new(config).await.unwrap();
    runtime.start().await.unwrap();

    let brain = OmegaBrain::new(BrainConfig::default()).await.unwrap();

    // Simulate 1 hour of conscious processing (accelerated)
    let start_time = std::time::Instant::now();
    let mut consciousness_readings = Vec::new();

    for iteration in 0..3600 {
        // Process sensory input
        let input = generate_complex_sensory_data(iteration);
        let output = brain.process_conscious_cycle(&input).await.unwrap();

        // Measure consciousness
        let phi = output.integrated_information;
        consciousness_readings.push((iteration, phi));

        // Simulate 1 second passing
        tokio::time::sleep(Duration::from_millis(10)).await; // Accelerated
    }

    // Verify consciousness emergence
    let initial_phi = consciousness_readings.first().unwrap().1;
    let final_phi = consciousness_readings.last().unwrap().1;

    assert!(
        final_phi > initial_phi * 2.0,
        "Consciousness should increase significantly over time"
    );

    // Verify stability (no wild oscillations)
    let variance = calculate_variance(&consciousness_readings);
    assert!(
        variance < 0.1,
        "Consciousness should be stable, not oscillating wildly"
    );

    // Verify self-model coherence
    let self_model = brain.get_self_model().await.unwrap();
    assert!(
        self_model.coherence_score() > 0.7,
        "Self-model should be coherent after prolonged operation"
    );

    // Cleanup
    runtime.stop().await.unwrap();

    println!("E2E test completed in {:?}", start_time.elapsed());
}

fn generate_complex_sensory_data(iteration: usize) -> SensoryInput {
    // Generate realistic multi-modal input
    SensoryInput {
        visual: generate_visual_pattern(iteration),
        auditory: generate_audio_waveform(iteration),
        tactile: generate_touch_sensations(iteration),
        internal: generate_proprioception(iteration),
        timestamp: iteration as u64,
    }
}
```

---

### 9. Performance Testing Suite

**Benchmark Critical Operations:**
```rust
// benches/memory_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use omega_memory::*;

fn bench_memory_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_storage");

    // Benchmark different memory counts
    for count in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut system = MemorySystem::new();
                    for i in 0..count {
                        let memory = create_test_memory(i);
                        system.store(black_box(memory));
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_memory_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_query");

    // Pre-populate system
    let mut system = MemorySystem::new();
    for i in 0..10_000 {
        system.store(create_test_memory(i));
    }

    group.bench_function("query_by_tier", |b| {
        b.iter(|| {
            system.query_by_tier(black_box(MemoryTier::Semantic))
        });
    });

    group.bench_function("query_by_importance", |b| {
        b.iter(|| {
            system.query_by_importance(black_box(0.8))
        });
    });

    group.bench_function("similarity_search", |b| {
        let query_embedding = vec![0.5; 768];
        b.iter(|| {
            system.similarity_search(black_box(&query_embedding), 10)
        });
    });

    group.finish();
}

criterion_group!(benches, bench_memory_storage, bench_memory_query);
criterion_main!(benches);
```

**Set Performance Budgets:**
```yaml
# .github/workflows/performance.yml
- name: Run benchmarks
  run: cargo bench --workspace > bench_results.txt

- name: Check performance regressions
  run: |
    # Fail if any operation is >10% slower than baseline
    cargo bench --workspace -- --save-baseline main
    cargo bench --workspace -- --baseline main --load-baseline main
```

---

### 10. Documentation and Testing Standards

**Create TESTING.md:**
```markdown
# Testing Standards for ExoGenesis Omega

## Coverage Requirements

- **Minimum Line Coverage:** 70%
- **Minimum Branch Coverage:** 60%
- **Critical Modules:** 85%+ coverage
  - omega-runtime
  - omega-persistence
  - omega-brain
  - omega-consciousness

## Test Types

### Unit Tests (70% of tests)
- Test single functions/methods
- Mock external dependencies
- Fast (<10ms per test)
- Located in same file as code

### Integration Tests (20% of tests)
- Test inter-module communication
- Use real dependencies
- Moderate speed (<100ms per test)
- Located in `tests/` directory

### E2E Tests (10% of tests)
- Test complete user scenarios
- Full system initialization
- Slower (<5s per test)
- Run less frequently

## Writing Tests

### Test Naming
```rust
#[test]
fn given_[state]_when_[action]_then_[outcome]()
```

### Test Structure
```rust
#[test]
fn test_name() {
    // Arrange: Setup
    let system = create_test_system();

    // Act: Execute
    let result = system.do_something();

    // Assert: Verify
    assert_eq!(result, expected);
}
```

### Property-Based Tests
Use for:
- Input validation
- Invariant checking
- Edge case discovery

### Benchmarks
Required for:
- Database operations
- Neural network processing
- Memory management
- Audio/signal processing

## CI/CD Requirements

All pull requests must:
1. Pass all tests: `cargo test --all`
2. Maintain coverage: No decrease in %
3. Pass benchmarks: <10% regression
4. Pass mutation tests: >75% score (when enabled)

## Performance Budgets

| Operation | Time Budget | Memory Budget |
|-----------|-------------|---------------|
| Memory store | <1ms | <1KB |
| Memory query | <10ms | <100KB |
| Neural step | <5ms | <10MB |
| Consciousness update | <50ms | <50MB |
| Full cognitive cycle | <500ms | <500MB |

## Test Data

- Use builders for complex objects
- Use fixtures for common scenarios
- Never commit sensitive data
- Keep test data minimal
```

---

## Long-term Goals (Next 6 Months)

### 11. Fuzzing Infrastructure

```bash
cargo install cargo-fuzz
cargo fuzz init
```

**Create Fuzz Targets:**
```rust
// fuzz/fuzz_targets/godelian_proposition.rs

#![no_main]
use libfuzzer_sys::fuzz_target;
use omega_strange_loops::GodelianEngine;

fuzz_target!(|data: &[u8]| {
    if let Ok(content) = std::str::from_utf8(data) {
        let mut engine = GodelianEngine::new();

        // Should never panic
        let id = engine.create_proposition(content.to_string(), 0);
        let _ = engine.attempt_proof(id);
        let _ = engine.get_proposition(id);
    }
});
```

### 12. Test Metrics Dashboard

**Track Over Time:**
- Test count trend
- Coverage percentage
- Mutation score
- Benchmark results
- Flaky test rate

**Tools:**
- codecov.io for coverage
- bencher.dev for benchmarks
- Custom dashboard for holistic view

### 13. Automated Test Generation

**Use AI to Generate Tests:**
```rust
// Example: Use LLM to generate edge cases

// Given function:
pub fn calculate_phi(network: &Network) -> f64 {
    // Complex consciousness calculation
}

// Generated tests:
#[test]
fn test_phi_with_empty_network() { /* ... */ }

#[test]
fn test_phi_with_disconnected_nodes() { /* ... */ }

#[test]
fn test_phi_with_fully_connected_network() { /* ... */ }

#[test]
fn test_phi_with_cyclic_network() { /* ... */ }
```

---

## Summary Checklist

### Week 1
- [ ] Fix omega-runtime compilation errors
- [ ] Run `cargo test --all` successfully
- [ ] Install cargo-llvm-cov
- [ ] Generate baseline coverage report
- [ ] Document current coverage %

### Week 2
- [ ] Add property-based tests to 3 modules
- [ ] Create test utilities crate
- [ ] Write 2 integration test suites
- [ ] Set up CI/CD test gates

### Month 1
- [ ] Reach 70% code coverage
- [ ] Add mutation testing
- [ ] Create 5 integration test suites
- [ ] Write TESTING.md guide

### Quarter 1
- [ ] Reach 80% code coverage
- [ ] Mutation score >75%
- [ ] 10+ E2E tests
- [ ] Performance budgets enforced
- [ ] Test metrics dashboard live

---

**Next Steps:**
1. Review this document with team
2. Prioritize recommendations
3. Assign owners for each task
4. Track progress in project board
5. Iterate based on results

**Questions?** Contact Tester Agent via swarm memory system.
