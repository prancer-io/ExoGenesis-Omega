# Rust Best Practices - Code Review Checklist for Omega Codebase

> **Compiled by**: Researcher Agent #2 (Hive Mind Swarm)
> **Session**: swarm-1765776508885-0wtq9b7ve
> **Date**: 2025-12-14
> **Purpose**: Comprehensive review guidelines for multi-crate Rust projects

---

## Table of Contents

1. [API Design & Idioms](#1-api-design--idioms)
2. [Safety & Security Patterns](#2-safety--security-patterns)
3. [Performance Optimization](#3-performance-optimization)
4. [Error Handling](#4-error-handling)
5. [Testing Strategies](#5-testing-strategies)
6. [Documentation Standards](#6-documentation-standards)
7. [Common Anti-Patterns](#7-common-anti-patterns)
8. [Dependency Management](#8-dependency-management)
9. [Concurrency & Async Patterns](#9-concurrency--async-patterns)
10. [Type System & Trait Design](#10-type-system--trait-design)

---

## 1. API Design & Idioms

### 1.1 Module Organization

**Best Practices:**
- ✅ Use `pub mod` for public modules, private for internal implementation
- ✅ Re-export commonly used types at crate root via `pub use`
- ✅ Organize by feature/domain, not by type (avoid separate `models/`, `utils/` directories)
- ✅ Use `mod.rs` or module-name file pattern consistently

**Review Checkpoints:**
```rust
// ✅ GOOD: Clear re-exports at crate root
pub mod types;
pub mod traits;
pub use types::*;
pub use traits::*;

// ❌ AVOID: Forcing users to import from deep paths
// User forced to: use omega_core::types::memory::MemoryTier;
// Better: use omega_core::MemoryTier;
```

**Omega Codebase Context:**
- Each crate (`omega-core`, `omega-brain`, etc.) should expose clear public API
- Core types should be accessible without deep imports
- Internal implementation details should remain private

---

### 1.2 Builder Pattern & Type-State

**Best Practices:**
- ✅ Use builder pattern for complex constructors (>3 parameters)
- ✅ Implement type-state pattern for compile-time state enforcement
- ✅ Provide sensible defaults via `Default` trait

**Review Checkpoints:**
```rust
// ✅ GOOD: Builder with type-state
pub struct MemoryBuilder<State = Unvalidated> {
    tier: Option<MemoryTier>,
    content: Option<MemoryContent>,
    importance: f64,
    _state: PhantomData<State>,
}

impl MemoryBuilder<Unvalidated> {
    pub fn tier(mut self, tier: MemoryTier) -> Self { /* ... */ }
    pub fn validate(self) -> MemoryBuilder<Validated> { /* ... */ }
}

impl MemoryBuilder<Validated> {
    pub fn build(self) -> Memory { /* ... */ }
}

// ❌ AVOID: Too many constructor parameters
pub fn new_memory(tier: MemoryTier, type_: MemoryType, content: MemoryContent,
                  importance: f64, confidence: f64, ttl: Option<Duration>,
                  metadata: HashMap<String, Value>) -> Memory
```

**Omega Specific:**
- Complex types like `Intelligence`, `Architecture` are good builder candidates
- Memory creation with metadata should use builder pattern

---

### 1.3 Method Naming Conventions

**Best Practices:**
- ✅ Use `new()` for constructors, `default()` for Default trait
- ✅ Prefix predicates with `is_`, `has_`, `can_`
- ✅ Use `as_ref()`, `as_mut()` for conversions
- ✅ Use `into_*()` for consuming conversions
- ✅ Use `to_*()` for expensive conversions that clone
- ✅ Use `try_*()` for fallible operations that return `Result`

**Review Checkpoints:**
```rust
// ✅ GOOD: Clear naming
impl Memory {
    pub fn new(tier: MemoryTier, ...) -> Self
    pub fn is_expired(&self) -> bool
    pub fn as_bytes(&self) -> &[u8]
    pub fn into_parts(self) -> (MemoryTier, MemoryContent)
    pub fn to_json(&self) -> Result<String, Error>  // Clones data
    pub fn try_consolidate(&mut self) -> Result<(), Error>
}

// ❌ AVOID: Unclear naming
pub fn check_expired(&self) -> bool  // Use is_expired
pub fn get_bytes(&self) -> &[u8]     // Use as_bytes
pub fn convert(self) -> String        // Use into_string or to_string
```

---

### 1.4 Iterator Patterns

**Best Practices:**
- ✅ Implement `IntoIterator` for container types
- ✅ Provide `iter()`, `iter_mut()` methods
- ✅ Return impl Iterator when appropriate for zero-cost abstractions
- ✅ Use iterator adapters instead of manual loops

**Review Checkpoints:**
```rust
// ✅ GOOD: Iterator implementation
impl Intelligence {
    pub fn capabilities(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.iter()
    }

    pub fn capabilities_mut(&mut self) -> impl Iterator<Item = &mut Capability> {
        self.capabilities.iter_mut()
    }
}

// ✅ GOOD: Using iterator adapters
let active_loops: Vec<_> = coordinator.loops()
    .filter(|loop_| loop_.is_active())
    .map(|loop_| loop_.id())
    .collect();

// ❌ AVOID: Manual loops when iterators suffice
let mut active = Vec::new();
for loop_ in coordinator.loops() {
    if loop_.is_active() {
        active.push(loop_.id());
    }
}
```

---

## 2. Safety & Security Patterns

### 2.1 Unsafe Code Usage

**Best Practices:**
- ✅ Minimize `unsafe` blocks - justify with safety comments
- ✅ Encapsulate `unsafe` in safe abstractions
- ✅ Document invariants that must be maintained
- ✅ Use `#[deny(unsafe_code)]` at crate level when possible

**Review Checkpoints:**
```rust
// ✅ GOOD: Justified unsafe with safety comment
/// # Safety
/// This is safe because:
/// 1. The pointer is guaranteed to be valid for the lifetime 'a
/// 2. The memory is properly aligned for T
/// 3. No mutable aliases exist during this operation
unsafe fn from_raw_parts<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    std::slice::from_raw_parts(ptr, len)
}

// ❌ AVOID: Unsafe without documentation or justification
unsafe {
    let data = std::mem::transmute::<&[u8], &str>(bytes);
    // Why is this safe? What invariants are assumed?
}
```

**Omega Specific:**
- SIMD operations in `omega-agentdb` may require unsafe
- Neural network operations should avoid unsafe when possible
- Document all safety invariants clearly

---

### 2.2 Bounds Checking & Panics

**Best Practices:**
- ✅ Prefer `get()` over indexing for potential out-of-bounds
- ✅ Use `expect()` with descriptive messages over `unwrap()`
- ✅ Return `Result` instead of panicking in library code
- ✅ Document panic conditions in function docs

**Review Checkpoints:**
```rust
// ✅ GOOD: Safe access with Result
pub fn get_memory(&self, index: usize) -> Option<&Memory> {
    self.memories.get(index)
}

pub fn memory_at(&self, index: usize) -> Result<&Memory, MemoryError> {
    self.memories.get(index)
        .ok_or_else(|| MemoryError::IndexOutOfBounds {
            index,
            len: self.memories.len()
        })
}

// ❌ AVOID: Unwrap in library code
pub fn get_memory(&self, index: usize) -> &Memory {
    self.memories.get(index).unwrap()  // Can panic!
}

// ⚠️ ACCEPTABLE: Expect with clear message in application code
let config = Config::load()
    .expect("Failed to load config - ensure config.toml exists");
```

---

### 2.3 Integer Overflow & Arithmetic

**Best Practices:**
- ✅ Use checked arithmetic in production code
- ✅ Enable overflow checks in release builds when appropriate
- ✅ Use saturating/wrapping operations explicitly when needed
- ✅ Validate user inputs that affect calculations

**Review Checkpoints:**
```rust
// ✅ GOOD: Safe arithmetic
pub fn calculate_importance(&self) -> Result<f64, ArithmeticError> {
    let base_score = self.access_count
        .checked_mul(100)?
        .checked_add(self.confidence_boost)?;
    Ok(base_score as f64 / 1000.0)
}

// ✅ GOOD: Explicit saturation when appropriate
pub fn increment_generation(&mut self) {
    self.generation = self.generation.saturating_add(1);
}

// ❌ AVOID: Unchecked arithmetic
pub fn calculate_score(&self) -> u64 {
    self.value * 100 + self.bonus  // Can overflow!
}
```

---

### 2.4 Resource Management & Cleanup

**Best Practices:**
- ✅ Implement `Drop` for resources requiring cleanup
- ✅ Use RAII pattern for automatic resource management
- ✅ Avoid `mem::forget` unless absolutely necessary
- ✅ Document drop order dependencies

**Review Checkpoints:**
```rust
// ✅ GOOD: RAII pattern with Drop
pub struct RuntimeGuard {
    runtime: Arc<Mutex<Runtime>>,
}

impl Drop for RuntimeGuard {
    fn drop(&mut self) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.shutdown();
            tracing::info!("Runtime shutdown completed");
        }
    }
}

// ✅ GOOD: Scoped resource management
{
    let _guard = coordinator.start_cycle(input)?;
    // Cycle automatically completes when guard drops
    perform_operations();
}  // Guard dropped here, cleanup happens
```

---

## 3. Performance Optimization

### 3.1 Allocation & Memory Management

**Best Practices:**
- ✅ Pre-allocate collections with known capacity
- ✅ Reuse allocations when possible
- ✅ Use `Vec::with_capacity()` over repeated `push()`
- ✅ Prefer stack allocation over heap when size is known
- ✅ Use `Box<[T]>` instead of `Vec<T>` for fixed-size data
- ✅ Consider `SmallVec` or `ArrayVec` for small collections

**Review Checkpoints:**
```rust
// ✅ GOOD: Pre-allocated capacity
pub fn process_batch(&self, items: &[Item]) -> Vec<Result> {
    let mut results = Vec::with_capacity(items.len());
    for item in items {
        results.push(self.process(item));
    }
    results
}

// ✅ GOOD: Reusing allocation
pub fn update_batch(&mut self, updates: Vec<Update>) {
    self.buffer.clear();
    self.buffer.extend(updates);
}

// ❌ AVOID: Repeated allocations
let mut results = Vec::new();  // Capacity 0, will reallocate multiple times
for i in 0..1000 {
    results.push(process(i));  // Many reallocations
}
```

**Omega Specific:**
- Neural network layers should pre-allocate weight matrices
- Memory consolidation should batch allocations
- Loop iterations should reuse buffers when possible

---

### 3.2 Cloning & Copying

**Best Practices:**
- ✅ Implement `Copy` for small types (≤128 bits)
- ✅ Use `Clone` for larger types
- ✅ Avoid unnecessary clones - prefer borrowing
- ✅ Use `Cow` (Clone-on-Write) for conditional ownership
- ✅ Mark expensive clones with `#[must_use]`

**Review Checkpoints:**
```rust
// ✅ GOOD: Small type with Copy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTier {
    Immediate,
    ShortTerm,
    // ... (12 variants, small enum)
}

// ✅ GOOD: Borrowing instead of cloning
pub fn analyze_intelligence(&self, intelligence: &Intelligence) -> Report {
    Report {
        name: intelligence.name.clone(),  // Only clone when necessary
        capabilities: &intelligence.capabilities,  // Borrow
    }
}

// ✅ GOOD: Clone-on-Write
use std::borrow::Cow;

pub fn process_data<'a>(&self, data: Cow<'a, str>) -> String {
    if needs_modification(&data) {
        let mut owned = data.into_owned();
        modify(&mut owned);
        owned
    } else {
        data.into_owned()
    }
}

// ❌ AVOID: Unnecessary cloning
pub fn get_name(&self) -> String {
    self.name.clone()  // Returns owned - prefer &str
}

// ✅ BETTER:
pub fn name(&self) -> &str {
    &self.name
}
```

---

### 3.3 String Handling

**Best Practices:**
- ✅ Use `&str` for function parameters instead of `String`
- ✅ Use `String` only when ownership is needed
- ✅ Consider `Cow<str>` for conditional ownership
- ✅ Use `format!` macro sparingly - prefer string concatenation for simple cases
- ✅ Use `write!` into a buffer for complex string building

**Review Checkpoints:**
```rust
// ✅ GOOD: Accepting string slices
pub fn create_intelligence(&mut self, name: &str) -> Intelligence {
    Intelligence {
        name: name.to_string(),
        // ...
    }
}

// ✅ GOOD: Efficient string building
use std::fmt::Write;

pub fn build_report(&self) -> String {
    let mut report = String::with_capacity(1024);
    writeln!(&mut report, "Intelligence Report").unwrap();
    writeln!(&mut report, "Name: {}", self.name).unwrap();
    report
}

// ❌ AVOID: Taking String when &str suffices
pub fn set_name(&mut self, name: String) {  // Forces caller to clone
    self.name = name;
}

// ❌ AVOID: Inefficient concatenation
let report = "Name: ".to_string() + &self.name + "\n" + "Status: " + &self.status;
```

---

### 3.4 Iterator Optimization

**Best Practices:**
- ✅ Use iterator chains instead of collecting intermediate results
- ✅ Leverage `size_hint()` for pre-allocation
- ✅ Use `fold()` and `collect()` efficiently
- ✅ Avoid collecting when streaming is sufficient
- ✅ Use `into_iter()` to consume and avoid clones

**Review Checkpoints:**
```rust
// ✅ GOOD: Iterator chain without intermediate collections
pub fn active_memory_count(&self) -> usize {
    self.memories.iter()
        .filter(|m| !m.is_expired())
        .filter(|m| m.importance > 0.5)
        .count()
}

// ✅ GOOD: Consuming iterator to avoid clones
pub fn take_capabilities(self) -> impl Iterator<Item = Capability> {
    self.capabilities.into_iter()
}

// ❌ AVOID: Unnecessary intermediate collections
let expired: Vec<_> = self.memories.iter()
    .filter(|m| m.is_expired())
    .collect();
let count = expired.len();  // Don't need to collect just for count!

// ✅ BETTER:
let count = self.memories.iter()
    .filter(|m| m.is_expired())
    .count();
```

---

## 4. Error Handling

### 4.1 Error Type Design

**Best Practices:**
- ✅ Use `thiserror` for error type definitions
- ✅ Implement `std::error::Error` trait
- ✅ Use enum for multiple error variants
- ✅ Include context in error messages
- ✅ Use `#[from]` for automatic conversion from underlying errors

**Review Checkpoints:**
```rust
// ✅ GOOD: Well-designed error type (from omega-runtime)
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("State transition error: current={current}, attempted={attempted}")]
    InvalidStateTransition {
        current: String,
        attempted: String
    },

    #[error("Component error - {component}: {error}")]
    Component {
        component: String,
        error: String
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// ❌ AVOID: String-based errors
pub type MyResult<T> = Result<T, String>;

// ❌ AVOID: Generic error without context
pub type MyResult<T> = Result<T, Box<dyn Error>>;
```

**Omega Specific:**
- Each crate should define its own error type
- Runtime errors should include component context
- Memory errors should include tier information
- Loop errors should include loop type and cycle ID

---

### 4.2 Result Propagation

**Best Practices:**
- ✅ Use `?` operator for error propagation
- ✅ Use `map_err()` to add context when converting errors
- ✅ Avoid `unwrap()` and `expect()` in library code
- ✅ Use `Result` return type for fallible operations
- ✅ Consider `anyhow` for application code, `thiserror` for libraries

**Review Checkpoints:**
```rust
// ✅ GOOD: Error propagation with context
pub async fn initialize(&mut self) -> Result<(), RuntimeError> {
    self.load_config()?;

    self.init_memory()
        .await
        .map_err(|e| RuntimeError::Component {
            component: "memory".to_string(),
            error: e.to_string(),
        })?;

    self.start_loops().await?;

    Ok(())
}

// ✅ GOOD: Chaining with map_err for context
pub fn parse_architecture(data: &str) -> Result<Architecture, ParseError> {
    serde_json::from_str(data)
        .map_err(|e| ParseError::InvalidJson {
            field: "architecture",
            source: e,
        })
}

// ❌ AVOID: Losing error context
pub fn load_data(&self) -> Result<Data, Error> {
    let content = std::fs::read_to_string(&self.path)?;
    let data = serde_json::from_str(&content)?;  // Which file? What field?
    Ok(data)
}
```

---

### 4.3 Custom Result Types

**Best Practices:**
- ✅ Define type aliases for common Result types in each crate
- ✅ Use `pub type Result<T> = std::result::Result<T, CrateError>` pattern
- ✅ Makes code more readable and maintainable

**Review Checkpoints:**
```rust
// ✅ GOOD: Custom Result types (from omega-runtime)
pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type ConfigResult<T> = Result<T, ConfigError>;
pub type APIResult<T> = Result<T, APIError>;

// Usage:
pub async fn start(&mut self) -> RuntimeResult<()> {
    // Much cleaner than Result<(), RuntimeError>
}

// ✅ GOOD: Crate-level Result type
// In lib.rs:
pub mod error;
pub use error::OmegaError;
pub type Result<T> = std::result::Result<T, OmegaError>;

// In other files:
use crate::Result;

pub fn process(&self) -> Result<Output> {
    // ...
}
```

---

### 4.4 Error Recovery & Fallbacks

**Best Practices:**
- ✅ Provide fallback mechanisms for non-critical operations
- ✅ Use `unwrap_or()`, `unwrap_or_else()`, `unwrap_or_default()` appropriately
- ✅ Log errors even when recovered
- ✅ Consider circuit breaker pattern for external dependencies

**Review Checkpoints:**
```rust
// ✅ GOOD: Graceful degradation
pub fn get_cached_or_compute(&mut self, key: &str) -> Value {
    self.cache.get(key)
        .cloned()
        .unwrap_or_else(|| {
            tracing::warn!("Cache miss for key: {}", key);
            self.compute_expensive(key)
        })
}

// ✅ GOOD: Circuit breaker pattern (from omega-runtime)
pub struct CircuitBreaker {
    state: CircuitState,
    failure_threshold: u32,
    timeout: Duration,
}

impl CircuitBreaker {
    pub async fn call<F, T>(&mut self, f: F) -> Result<T, CircuitError>
    where
        F: Future<Output = Result<T, Error>>,
    {
        match self.state {
            CircuitState::Open => Err(CircuitError::Open),
            CircuitState::Closed | CircuitState::HalfOpen => {
                match f.await {
                    Ok(result) => {
                        self.on_success();
                        Ok(result)
                    }
                    Err(e) => {
                        self.on_failure();
                        Err(CircuitError::Underlying(e))
                    }
                }
            }
        }
    }
}
```

---

## 5. Testing Strategies

### 5.1 Unit Testing

**Best Practices:**
- ✅ Test public API, not private implementation
- ✅ Use `#[cfg(test)]` module for tests
- ✅ One assertion per test when possible
- ✅ Use descriptive test names: `test_<scenario>_<expected_behavior>`
- ✅ Follow Arrange-Act-Assert (AAA) pattern
- ✅ Use test helpers to reduce duplication

**Review Checkpoints:**
```rust
// ✅ GOOD: Well-structured unit tests (from omega-core)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_create_intelligence_sets_initial_status() {
        // Arrange
        let architecture = create_test_architecture();

        // Act
        let intelligence = Intelligence::new("Test AI".to_string(), architecture);

        // Assert
        assert_eq!(intelligence.status, IntelligenceStatus::Initializing);
    }

    #[test]
    fn test_memory_tiers_returns_all_twelve_tiers() {
        // Act
        let tiers = MemoryTier::all_tiers();

        // Assert
        assert_eq!(tiers.len(), 12);
        assert_eq!(tiers[0], MemoryTier::Immediate);
        assert_eq!(tiers[11], MemoryTier::Cosmic);
    }

    // Test helper
    fn create_test_architecture() -> Architecture {
        Architecture {
            id: "test-arch".to_string(),
            name: "Test Architecture".to_string(),
            paradigm: Paradigm::Neural,
            substrate: SubstrateType::Digital,
            fitness: None,
            lineage: vec![],
            created_at: Utc::now(),
        }
    }
}

// ❌ AVOID: Testing private implementation
#[test]
fn test_internal_cache_structure() {
    // Don't test private fields or methods
}

// ❌ AVOID: Multiple unrelated assertions
#[test]
fn test_everything() {
    let intel = create_intelligence();
    assert_eq!(intel.status, IntelligenceStatus::Initializing);
    assert_eq!(intel.generation, 0);
    assert!(intel.capabilities.is_empty());
    // Split into separate tests
}
```

---

### 5.2 Integration Testing

**Best Practices:**
- ✅ Place integration tests in `tests/` directory
- ✅ Test interactions between modules/crates
- ✅ Use realistic scenarios
- ✅ Set up and tear down test fixtures properly
- ✅ Use `#[tokio::test]` for async integration tests

**Review Checkpoints:**
```rust
// ✅ GOOD: Integration test (from omega-loops tests)
#[tokio::test]
async fn test_full_loop_hierarchy() {
    // Arrange
    let mut coordinator = LoopCoordinator::new();
    coordinator.register_loop(Box::new(QuantumLoop::new()));
    coordinator.register_loop(Box::new(NeuralLoop::new()));
    coordinator.register_loop(Box::new(CognitiveLoop::new()));

    // Act
    coordinator.start().await.unwrap();
    let input = OmegaInput::new("Test sensory input");
    let output = coordinator.process(input).await.unwrap();

    // Assert
    assert!(coordinator.is_running().await);
    assert!(!output.results.is_empty());

    // Cleanup
    coordinator.stop().await.unwrap();
}

// ✅ GOOD: Testing error conditions
#[tokio::test]
async fn test_coordinator_fails_when_not_started() {
    let mut coordinator = LoopCoordinator::new();
    let input = OmegaInput::new("Test");

    let result = coordinator.process(input).await;
    assert!(result.is_err());
}
```

---

### 5.3 Property-Based Testing

**Best Practices:**
- ✅ Use `proptest` or `quickcheck` for property-based testing
- ✅ Test invariants that should always hold
- ✅ Generate random inputs to discover edge cases
- ✅ Particularly useful for parsers, encoders, mathematical operations

**Review Checkpoints:**
```rust
// ✅ GOOD: Property-based test
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_memory_serialization_roundtrip(
            tier in any::<MemoryTier>(),
            importance in 0.0f64..1.0f64,
        ) {
            // Arrange
            let memory = Memory::new(
                tier,
                MemoryType::Knowledge,
                MemoryContent::Text("test".to_string()),
                importance,
            );

            // Act
            let serialized = serde_json::to_string(&memory).unwrap();
            let deserialized: Memory = serde_json::from_str(&serialized).unwrap();

            // Assert - roundtrip property
            assert_eq!(memory.tier, deserialized.tier);
            assert_eq!(memory.metadata.importance, deserialized.metadata.importance);
        }

        #[test]
        fn test_importance_always_bounded(importance in any::<f64>()) {
            let normalized = normalize_importance(importance);

            // Property: normalized importance is always in [0, 1]
            assert!(normalized >= 0.0 && normalized <= 1.0);
        }
    }
}
```

**Omega Specific:**
- Test memory consolidation preserves data
- Test neural network weight updates don't explode
- Test loop coordination maintains consistency
- Test architecture evolution produces valid mutations

---

### 5.4 Benchmark Testing

**Best Practices:**
- ✅ Use `criterion` crate for benchmarking
- ✅ Place benchmarks in `benches/` directory
- ✅ Benchmark critical paths and hot loops
- ✅ Use realistic data sizes
- ✅ Compare alternatives with micro-benchmarks

**Review Checkpoints:**
```rust
// ✅ GOOD: Benchmark structure (example for omega-snn)
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use omega_snn::*;

fn benchmark_spike_propagation(c: &mut Criterion) {
    let mut network = SpikingNetwork::new(1000, 10000);

    c.bench_function("spike_propagation_1000_neurons", |b| {
        b.iter(|| {
            network.propagate_spikes(black_box(0.001))
        });
    });
}

fn benchmark_memory_consolidation(c: &mut Criterion) {
    let mut memory_system = MemorySystem::with_capacity(10000);

    // Setup
    for i in 0..10000 {
        memory_system.store(create_test_memory(i));
    }

    c.bench_function("consolidate_10k_memories", |b| {
        b.iter(|| {
            memory_system.consolidate(
                black_box(MemoryTier::ShortTerm),
                black_box(MemoryTier::Episodic)
            )
        });
    });
}

criterion_group!(benches, benchmark_spike_propagation, benchmark_memory_consolidation);
criterion_main!(benches);
```

**Omega Specific:**
- Benchmark SIMD operations in `omega-agentdb`
- Benchmark memory queries across tiers
- Benchmark loop tick performance
- Benchmark neural network forward/backward passes

---

### 5.5 Test Organization

**Best Practices:**
- ✅ Unit tests: In same file as code under `#[cfg(test)]`
- ✅ Integration tests: In `tests/` directory at crate root
- ✅ Benchmarks: In `benches/` directory
- ✅ Examples: In `examples/` directory
- ✅ Test fixtures: In `tests/fixtures/` or use `rstest` crate

**Directory Structure:**
```
omega-core/
├── src/
│   ├── lib.rs              // Unit tests in #[cfg(test)] mod
│   ├── types/
│   │   └── memory.rs       // Unit tests in #[cfg(test)] mod
│   └── traits/
│       └── core.rs         // Unit tests in #[cfg(test)] mod
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/
│       └── test_data.json
├── benches/
│   └── memory_benchmarks.rs
└── examples/
    └── basic_usage.rs
```

---

## 6. Documentation Standards

### 6.1 Crate-Level Documentation

**Best Practices:**
- ✅ Add module-level doc comment at top of `lib.rs` with `//!`
- ✅ Include overview, architecture, usage examples
- ✅ Link to related crates and modules
- ✅ Use markdown formatting
- ✅ Include examples in doc comments with ```` ```rust ````

**Review Checkpoints:**
```rust
// ✅ GOOD: Comprehensive crate docs (from omega-core)
//! # ExoGenesis Omega - Core Types and Traits
//!
//! This crate provides the foundational types and traits for the ExoGenesis Omega
//! universal intelligence orchestration system.
//!
//! ## Overview
//!
//! ExoGenesis Omega enables the orchestration of intelligence at all scales...
//!
//! ## Architecture
//!
//! ### Intelligence Types
//! The [`Intelligence`] type represents any form of intelligence...
//!
//! ## Usage
//!
//! ```rust
//! use omega_core::*;
//!
//! let intelligence = Intelligence::new(
//!     "My AI".to_string(),
//!     architecture,
//! );
//! ```

// ❌ AVOID: Missing or minimal crate docs
//! omega-core
```

---

### 6.2 Function & Method Documentation

**Best Practices:**
- ✅ Document all public APIs with `///` doc comments
- ✅ Include purpose, parameters, return value, errors, examples
- ✅ Use sections: `# Arguments`, `# Returns`, `# Errors`, `# Examples`, `# Panics`, `# Safety`
- ✅ Link to related functions with square brackets: `[`function_name`]`
- ✅ Mark deprecated items with `#[deprecated]` and migration path

**Review Checkpoints:**
```rust
// ✅ GOOD: Well-documented function
/// Stores a memory in the appropriate tier.
///
/// This function validates the memory's metadata and assigns it to the
/// specified tier. If the tier is full, older low-importance memories
/// may be evicted according to the tier's retention policy.
///
/// # Arguments
///
/// * `memory` - The memory to store
///
/// # Returns
///
/// Returns the unique `MemoryId` assigned to the stored memory.
///
/// # Errors
///
/// This function will return an error if:
/// - The memory tier is invalid
/// - The memory content exceeds the tier's size limit
/// - Serialization fails
///
/// # Examples
///
/// ```rust
/// use omega_memory::{Memory, MemoryTier, MemoryType, MemoryContent};
///
/// let mut manager = MemoryManager::new();
/// let memory = Memory::new(
///     MemoryTier::Semantic,
///     MemoryType::Knowledge,
///     MemoryContent::Text("Important fact".to_string()),
///     0.9,
/// );
///
/// let id = manager.store_memory(memory).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # See Also
///
/// - [`get_memory`] - Retrieve a stored memory
/// - [`query_memories`] - Search across memory tiers
pub async fn store_memory(&mut self, memory: Memory) -> Result<MemoryId, MemoryError> {
    // Implementation
}

// ❌ AVOID: Missing or minimal docs
pub async fn store_memory(&mut self, memory: Memory) -> Result<MemoryId, MemoryError> {
    // No documentation
}
```

---

### 6.3 Type Documentation

**Best Practices:**
- ✅ Document structs, enums, and their fields
- ✅ Explain what the type represents
- ✅ Document invariants and constraints
- ✅ Provide construction examples
- ✅ Use `#[doc(hidden)]` for internal types in public API

**Review Checkpoints:**
```rust
// ✅ GOOD: Documented struct
/// Represents an intelligence entity in the Omega system.
///
/// An `Intelligence` combines an architectural blueprint with runtime state,
/// capabilities, and evolutionary lineage. Each intelligence can evolve through
/// generations, acquiring new capabilities and optimizing its architecture.
///
/// # Invariants
///
/// - `generation` must be non-negative
/// - `created_at` must not be in the future
/// - `capabilities` contains unique capability IDs
///
/// # Examples
///
/// ```rust
/// use omega_core::{Intelligence, Architecture, Paradigm, SubstrateType};
///
/// let architecture = Architecture {
///     // ...
/// };
///
/// let intelligence = Intelligence::new("My AI".to_string(), architecture);
/// assert_eq!(intelligence.generation, 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intelligence {
    /// Unique identifier for this intelligence
    pub id: IntelligenceId,

    /// Human-readable name
    pub name: String,

    /// Current evolutionary generation (0 for first generation)
    pub generation: u32,

    /// Current operational status
    pub status: IntelligenceStatus,

    /// Architectural blueprint defining structure and capabilities
    pub architecture: Architecture,

    /// Active capabilities (learned or configured)
    pub capabilities: Vec<Capability>,

    /// Timestamp of creation
    pub created_at: DateTime<Utc>,
}

// ✅ GOOD: Documented enum
/// Memory tier representing different temporal scales of storage.
///
/// The Omega system uses 12 tiers spanning from milliseconds to cosmic timescales.
/// Each tier has different retention policies, consolidation rules, and access patterns.
///
/// # Tier Hierarchy
///
/// Tiers are ordered from shortest to longest duration:
/// 1. Immediate (milliseconds)
/// 2. ShortTerm (seconds to minutes)
/// ...
/// 12. Cosmic (billions of years)
///
/// # See Also
///
/// - [`Memory`] - Individual memory entries
/// - [`MemoryManager::consolidate_memories`] - Moving memories between tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryTier {
    /// Immediate sensory buffer (milliseconds)
    Immediate,
    /// Short-term working memory (seconds to minutes)
    ShortTerm,
    // ... (document each variant)
}
```

---

### 6.4 Example Code

**Best Practices:**
- ✅ Provide working examples in `examples/` directory
- ✅ Include examples in doc comments
- ✅ Use `# Ok::<(), ErrorType>(())` for infallible examples
- ✅ Show common use cases
- ✅ Keep examples simple and focused

**Review Checkpoints:**
```rust
// ✅ GOOD: examples/basic_usage.rs
//! Basic usage example for omega-memory
//!
//! This example demonstrates:
//! - Creating a memory system
//! - Storing memories across different tiers
//! - Querying and consolidating memories

use omega_memory::*;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize memory system
    let mut memory_system = MemorySystem::new();

    // Store immediate memory
    let immediate = Memory::new(
        MemoryTier::Immediate,
        MemoryType::Sensory,
        MemoryContent::Text("Saw a flash of light".to_string()),
        0.7,
    );
    memory_system.store(immediate).await?;

    // Store semantic knowledge
    let semantic = Memory::new(
        MemoryTier::Semantic,
        MemoryType::Knowledge,
        MemoryContent::Text("The sky is blue".to_string()),
        0.95,
    );
    memory_system.store(semantic).await?;

    // Query memories
    let query = MemoryQuery::new()
        .tier(MemoryTier::Semantic)
        .min_importance(0.8);
    let results = memory_system.query(query).await?;

    println!("Found {} semantic memories", results.len());

    Ok(())
}
```

---

## 7. Common Anti-Patterns

### 7.1 Ownership Anti-Patterns

**Anti-Patterns to Avoid:**
- ❌ Unnecessary cloning to satisfy the borrow checker
- ❌ Using `Rc<RefCell<T>>` when `&mut T` would work
- ❌ Over-use of `Arc<Mutex<T>>` in single-threaded code
- ❌ Returning references to temporary values

**Review Checkpoints:**
```rust
// ❌ AVOID: Clone to bypass borrow checker
pub fn process_data(&mut self) {
    let data = self.data.clone();  // Unnecessary clone
    self.transform(&data);
    self.data = data;
}

// ✅ BETTER: Use split borrows
pub fn process_data(&mut self) {
    let result = Self::transform_static(&self.data);
    self.data = result;
}

// ❌ AVOID: Rc<RefCell> for single-threaded mutable access
use std::rc::Rc;
use std::cell::RefCell;

struct Manager {
    data: Rc<RefCell<Data>>,
}

// ✅ BETTER: Direct ownership
struct Manager {
    data: Data,
}

// ❌ AVOID: Returning reference to temporary
pub fn get_name(&self) -> &str {
    &format!("{}-{}", self.prefix, self.id)  // Temporary value!
}

// ✅ BETTER: Return owned String
pub fn get_name(&self) -> String {
    format!("{}-{}", self.prefix, self.id)
}
```

---

### 7.2 Error Handling Anti-Patterns

**Anti-Patterns to Avoid:**
- ❌ Using `unwrap()` or `expect()` in library code
- ❌ Swallowing errors with `let _ = ...`
- ❌ Returning `Option` when `Result` provides more context
- ❌ Using panics for error conditions

**Review Checkpoints:**
```rust
// ❌ AVOID: Unwrap in library code
pub fn load_config(&self) -> Config {
    let content = std::fs::read_to_string("config.toml").unwrap();
    toml::from_str(&content).unwrap()
}

// ✅ BETTER: Return Result
pub fn load_config(&self) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string("config.toml")
        .map_err(|e| ConfigError::Io(e))?;
    toml::from_str(&content)
        .map_err(|e| ConfigError::Parse(e.to_string()))
}

// ❌ AVOID: Swallowing errors
pub fn cleanup(&mut self) {
    let _ = self.flush();  // Error ignored!
    let _ = self.close();
}

// ✅ BETTER: Log or propagate
pub fn cleanup(&mut self) -> Result<(), Error> {
    self.flush()?;
    self.close()?;
    Ok(())
}

// OR log if recovery is appropriate:
pub fn cleanup(&mut self) {
    if let Err(e) = self.flush() {
        tracing::error!("Failed to flush: {}", e);
    }
    if let Err(e) = self.close() {
        tracing::error!("Failed to close: {}", e);
    }
}
```

---

### 7.3 Concurrency Anti-Patterns

**Anti-Patterns to Avoid:**
- ❌ Holding locks across `.await` points
- ❌ Using `std::sync::Mutex` in async code
- ❌ Spawning unbounded tasks
- ❌ Not handling task panics

**Review Checkpoints:**
```rust
// ❌ AVOID: Holding lock across await
use std::sync::Mutex;

async fn bad_async(&self) -> Result<(), Error> {
    let guard = self.data.lock().unwrap();
    let result = fetch_data().await;  // Lock held across await!
    process(&guard, result);
    Ok(())
}

// ✅ BETTER: Use tokio::sync::Mutex or drop guard before await
use tokio::sync::Mutex;

async fn good_async(&self) -> Result<(), Error> {
    let result = fetch_data().await;
    let mut guard = self.data.lock().await;  // tokio Mutex
    process(&guard, result);
    Ok(())
}

// OR: Drop guard explicitly
async fn good_async_drop(&self) -> Result<(), Error> {
    let data = {
        let guard = self.data.lock().unwrap();
        guard.clone()  // Release lock
    };  // Guard dropped here

    let result = fetch_data().await;
    process(&data, result);
    Ok(())
}

// ❌ AVOID: Unbounded task spawning
for item in items {
    tokio::spawn(process_item(item));  // Could spawn millions!
}

// ✅ BETTER: Use bounded concurrency
use futures::stream::{self, StreamExt};

stream::iter(items)
    .for_each_concurrent(10, |item| async move {
        process_item(item).await
    })
    .await;

// ✅ BETTER: Use semaphore for backpressure
use tokio::sync::Semaphore;

let semaphore = Arc::new(Semaphore::new(10));
for item in items {
    let permit = semaphore.clone().acquire_owned().await?;
    tokio::spawn(async move {
        let _permit = permit;  // Hold permit
        process_item(item).await
    });
}
```

---

### 7.4 Type System Anti-Patterns

**Anti-Patterns to Avoid:**
- ❌ Stringly-typed APIs (using `String` for everything)
- ❌ Boolean parameters (use enums instead)
- ❌ Ignoring type safety with `as` casts
- ❌ Over-use of `Box<dyn Trait>` when concrete types work

**Review Checkpoints:**
```rust
// ❌ AVOID: Stringly-typed
pub fn create_memory(&mut self, tier: &str, type_: &str) -> Result<MemoryId, Error> {
    // String comparison, prone to typos
    match tier {
        "immediate" => { /* ... */ }
        "short_term" => { /* ... */ }
        _ => return Err(Error::InvalidTier(tier.to_string())),
    }
}

// ✅ BETTER: Use enums
pub fn create_memory(&mut self, tier: MemoryTier, type_: MemoryType) -> Result<MemoryId, Error> {
    match tier {
        MemoryTier::Immediate => { /* ... */ }
        MemoryTier::ShortTerm => { /* ... */ }
        // Compiler ensures exhaustiveness
    }
}

// ❌ AVOID: Boolean parameters
pub fn initialize(&mut self, enable_cache: bool, auto_consolidate: bool,
                  strict_mode: bool) -> Result<(), Error> {
    // What does true/false mean for each?
}

// ✅ BETTER: Use descriptive types
pub struct InitOptions {
    pub cache_mode: CacheMode,
    pub consolidation: ConsolidationPolicy,
    pub validation: ValidationMode,
}

pub fn initialize(&mut self, options: InitOptions) -> Result<(), Error> {
    // Self-documenting
}

// ❌ AVOID: Unchecked casts
let value = some_u64 as usize;  // What if this truncates?

// ✅ BETTER: Use try_into or handle explicitly
let value: usize = some_u64.try_into()
    .map_err(|_| Error::ValueOutOfRange)?;
```

---

## 8. Dependency Management

### 8.1 Workspace Configuration

**Best Practices:**
- ✅ Use workspace for multi-crate projects
- ✅ Define shared dependencies in `[workspace.dependencies]`
- ✅ Use consistent versioning across workspace crates
- ✅ Set workspace-level metadata (authors, license, etc.)

**Review Checkpoints:**
```toml
# ✅ GOOD: Workspace configuration (from Omega)
[workspace]
resolver = "2"
members = [
    "crates/omega-core",
    "crates/omega-memory",
    "crates/omega-loops",
    # ...
]

[workspace.package]
version = "1.0.0"
edition = "2021"
authors = ["ExoGenesis Omega Team"]
license = "MIT"
repository = "https://github.com/prancer-io/ExoGenesis-Omega"

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# ... shared dependencies

# Individual crate Cargo.toml:
[package]
name = "omega-core"
version.workspace = true
edition.workspace = true
authors.workspace = true

[dependencies]
tokio.workspace = true
serde.workspace = true
```

---

### 8.2 Dependency Selection

**Best Practices:**
- ✅ Minimize dependencies - each adds compile time and security surface
- ✅ Choose well-maintained crates (recent updates, good docs)
- ✅ Check dependency tree with `cargo tree`
- ✅ Avoid duplicate dependencies (different versions of same crate)
- ✅ Use `cargo-deny` to enforce policy

**Review Checkpoints:**
```bash
# Check dependency tree
cargo tree

# Check for duplicate dependencies
cargo tree --duplicates

# Check for outdated dependencies
cargo outdated

# Audit dependencies for security vulnerabilities
cargo audit
```

**Omega Specific:**
```toml
# ✅ GOOD: Minimal, focused dependencies
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
async-trait = { workspace = true }

# ❌ AVOID: Kitchen sink dependencies
[dependencies]
everything = "1.0"  # Avoid crates that re-export everything
```

---

### 8.3 Feature Flags

**Best Practices:**
- ✅ Use feature flags for optional functionality
- ✅ Make features additive (enabling doesn't break API)
- ✅ Document features in Cargo.toml and README
- ✅ Use `default` feature carefully
- ✅ Test with different feature combinations in CI

**Review Checkpoints:**
```toml
# ✅ GOOD: Feature flags
[features]
default = ["runtime"]

# Core features
runtime = ["tokio/rt-multi-thread"]
persistence = ["dep:sqlx", "dep:rusqlite"]

# Optional neural capabilities
neural = ["dep:ndarray", "dep:candle-core"]
simd = ["dep:simsimd"]

# Development features
visualize = ["dep:plotters"]

[dependencies]
tokio = { workspace = true }

# Optional dependencies (enabled by features)
sqlx = { version = "0.7", optional = true }
ndarray = { version = "0.15", optional = true }
plotters = { version = "0.3", optional = true }

# Example usage in code:
#[cfg(feature = "neural")]
pub mod neural {
    // Neural network code
}
```

**Testing features:**
```bash
# Test with different feature combinations
cargo test --no-default-features
cargo test --all-features
cargo test --features "neural,simd"
```

---

### 8.4 Versioning & SemVer

**Best Practices:**
- ✅ Follow semantic versioning (SemVer)
- ✅ Document breaking changes in CHANGELOG
- ✅ Use `~` for patch updates, `^` for minor updates
- ✅ Pin exact versions in applications, use ranges in libraries
- ✅ Test against minimum supported Rust version (MSRV)

**Review Checkpoints:**
```toml
# ✅ GOOD: Library versioning (flexible for users)
[dependencies]
serde = "1.0"  # Equivalent to "^1.0", allows 1.x updates
tokio = "1.35"  # Allows 1.35-1.x

# ✅ GOOD: Application versioning (reproducible builds)
[dependencies]
serde = "=1.0.195"  # Exact version
tokio = "=1.35.1"

# Specify MSRV in Cargo.toml
[package]
rust-version = "1.75"  # Minimum supported Rust version
```

**SemVer Guidelines:**
- **MAJOR** (1.0.0 → 2.0.0): Breaking API changes
- **MINOR** (1.0.0 → 1.1.0): New features, backwards compatible
- **PATCH** (1.0.0 → 1.0.1): Bug fixes, backwards compatible

---

## 9. Concurrency & Async Patterns

### 9.1 Async/Await Best Practices

**Best Practices:**
- ✅ Use `async-trait` for async trait methods
- ✅ Prefer `tokio::sync` primitives over `std::sync` in async code
- ✅ Use structured concurrency (join handles, futures)
- ✅ Handle cancellation gracefully with `tokio::select!`
- ✅ Avoid blocking operations in async functions

**Review Checkpoints:**
```rust
// ✅ GOOD: Async trait (from omega-core)
use async_trait::async_trait;

#[async_trait]
pub trait MemoryManager: Send + Sync {
    async fn store_memory(&mut self, memory: Memory) -> Result<MemoryId, Error>;
    async fn query_memories(&self, query: MemoryQuery) -> Result<Vec<Memory>, Error>;
}

// ✅ GOOD: Tokio sync primitives
use tokio::sync::{Mutex, RwLock, Semaphore};

pub struct SharedState {
    data: Arc<Mutex<Data>>,          // For mutable access
    config: Arc<RwLock<Config>>,     // For read-heavy access
    limiter: Arc<Semaphore>,         // For rate limiting
}

// ✅ GOOD: Structured concurrency
async fn process_batch(&self, items: Vec<Item>) -> Result<Vec<Output>, Error> {
    let tasks: Vec<_> = items.into_iter()
        .map(|item| tokio::spawn(self.process_item(item)))
        .collect();

    let mut results = Vec::new();
    for task in tasks {
        results.push(task.await??);  // Wait for all, propagate errors
    }
    Ok(results)
}

// ✅ GOOD: Graceful cancellation
async fn run_with_timeout(&self) -> Result<Output, Error> {
    tokio::select! {
        result = self.long_operation() => {
            result
        }
        _ = tokio::time::sleep(Duration::from_secs(30)) => {
            Err(Error::Timeout)
        }
        _ = self.shutdown_signal.recv() => {
            Err(Error::Cancelled)
        }
    }
}

// ❌ AVOID: Blocking in async context
async fn bad_async(&self) -> Result<(), Error> {
    std::thread::sleep(Duration::from_secs(5));  // Blocks executor!
    Ok(())
}

// ✅ BETTER: Use async sleep
async fn good_async(&self) -> Result<(), Error> {
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(())
}

// OR: Spawn blocking work
async fn process_cpu_intensive(&self, data: Data) -> Result<Output, Error> {
    tokio::task::spawn_blocking(move || {
        // CPU-intensive work here
        expensive_computation(data)
    }).await?
}
```

---

### 9.2 Send + Sync Bounds

**Best Practices:**
- ✅ Understand `Send` (can transfer between threads) and `Sync` (can be shared)
- ✅ Add `+ Send + Sync` bounds to async trait objects
- ✅ Use `Arc` for shared ownership across threads
- ✅ Document when types are NOT `Send` or `Sync`

**Review Checkpoints:**
```rust
// ✅ GOOD: Proper bounds on async traits
#[async_trait]
pub trait Processor: Send + Sync {
    async fn process(&self, input: Input) -> Result<Output, Error>;
}

// ✅ GOOD: Thread-safe shared state
#[derive(Clone)]
pub struct RuntimeHandle {
    state: Arc<Mutex<RuntimeState>>,  // Arc for shared ownership
}

// Both Send and Sync are automatically derived

// ⚠️ DOCUMENT: Types that are NOT Send/Sync
/// Thread-local cache - not Send or Sync
pub struct ThreadLocalCache {
    data: std::rc::Rc<Data>,  // Rc is not Send
    _marker: PhantomData<*const ()>,  // Explicitly not Send/Sync
}

// ✅ GOOD: Using PhantomData for variance
pub struct Owned<T> {
    data: *mut T,
    _marker: PhantomData<T>,  // Acts like owning T for Send/Sync
}

unsafe impl<T: Send> Send for Owned<T> {}
unsafe impl<T: Sync> Sync for Owned<T> {}
```

---

### 9.3 Channel Patterns

**Best Practices:**
- ✅ Use `tokio::sync::mpsc` for async message passing
- ✅ Use `tokio::sync::broadcast` for fan-out
- ✅ Use `tokio::sync::watch` for state updates
- ✅ Use `tokio::sync::oneshot` for single-value responses
- ✅ Handle channel errors (sender dropped, receiver dropped)

**Review Checkpoints:**
```rust
use tokio::sync::{mpsc, broadcast, watch, oneshot};

// ✅ GOOD: MPSC for task queue
pub struct TaskQueue {
    sender: mpsc::Sender<Task>,
}

impl TaskQueue {
    pub fn new(buffer: usize) -> (Self, mpsc::Receiver<Task>) {
        let (sender, receiver) = mpsc::channel(buffer);
        (Self { sender }, receiver)
    }

    pub async fn submit(&self, task: Task) -> Result<(), Error> {
        self.sender.send(task).await
            .map_err(|_| Error::QueueClosed)
    }
}

// ✅ GOOD: Broadcast for events
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn publish(&self, event: Event) -> Result<(), Error> {
        let _ = self.sender.send(event);  // OK to ignore if no receivers
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

// ✅ GOOD: Watch for configuration updates
pub struct ConfigManager {
    config: watch::Sender<Config>,
}

impl ConfigManager {
    pub fn update(&mut self, new_config: Config) {
        let _ = self.config.send(new_config);
    }

    pub fn subscribe(&self) -> watch::Receiver<Config> {
        self.config.subscribe()
    }
}

// ✅ GOOD: Oneshot for request-response
pub async fn compute_async(value: u64) -> Result<u64, Error> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let result = expensive_computation(value);
        let _ = tx.send(result);  // Receiver might be dropped
    });

    rx.await.map_err(|_| Error::Cancelled)
}
```

---

### 9.4 Task Management

**Best Practices:**
- ✅ Use `JoinHandle` to track spawned tasks
- ✅ Use `tokio::task::JoinSet` for managing multiple tasks
- ✅ Handle task panics with `JoinError`
- ✅ Implement graceful shutdown
- ✅ Use task-local data with `tokio::task_local!`

**Review Checkpoints:**
```rust
use tokio::task::{JoinSet, JoinHandle};

// ✅ GOOD: Managing multiple tasks
pub struct Coordinator {
    tasks: JoinSet<Result<(), Error>>,
}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            tasks: JoinSet::new(),
        }
    }

    pub fn spawn_loop(&mut self, loop_: Box<dyn TemporalLoop>) {
        self.tasks.spawn(async move {
            loop_.run().await
        });
    }

    pub async fn shutdown(mut self) -> Result<(), Error> {
        // Cancel all tasks
        self.tasks.shutdown().await;

        // Wait for completion and collect errors
        let mut errors = Vec::new();
        while let Some(result) = self.tasks.join_next().await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => errors.push(e),
                Err(e) if e.is_panic() => {
                    tracing::error!("Task panicked: {:?}", e);
                }
                Err(e) => {
                    tracing::warn!("Task cancelled: {:?}", e);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::MultipleErrors(errors))
        }
    }
}

// ✅ GOOD: Graceful shutdown pattern
pub struct Runtime {
    shutdown_tx: broadcast::Sender<()>,
    tasks: Vec<JoinHandle<()>>,
}

impl Runtime {
    pub async fn run(&mut self) -> Result<(), Error> {
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutdown signal received");
                    break;
                }
                result = self.process_next() => {
                    result?;
                }
            }
        }

        // Wait for all tasks to complete
        for task in self.tasks.drain(..) {
            let _ = task.await;
        }

        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }
}
```

---

## 10. Type System & Trait Design

### 10.1 Trait Design Principles

**Best Practices:**
- ✅ Design traits for specific capabilities, not kitchen sink
- ✅ Use associated types for output types tied to the trait
- ✅ Use generic parameters for input types
- ✅ Provide default implementations where reasonable
- ✅ Consider trait coherence and orphan rules

**Review Checkpoints:**
```rust
// ✅ GOOD: Focused trait with associated types
pub trait Storage {
    type Item;
    type Error;

    fn store(&mut self, item: Self::Item) -> Result<(), Self::Error>;
    fn load(&self, id: &str) -> Result<Option<Self::Item>, Self::Error>;
}

// ✅ GOOD: Default implementations
pub trait Processor {
    type Input;
    type Output;

    fn process(&self, input: Self::Input) -> Self::Output;

    // Default implementation using process
    fn process_batch(&self, inputs: Vec<Self::Input>) -> Vec<Self::Output> {
        inputs.into_iter()
            .map(|input| self.process(input))
            .collect()
    }
}

// ❌ AVOID: Kitchen sink trait
pub trait EverythingManager {
    fn create(&mut self, data: Data);
    fn read(&self, id: &str) -> Data;
    fn update(&mut self, id: &str, data: Data);
    fn delete(&mut self, id: &str);
    fn search(&self, query: Query) -> Vec<Data>;
    fn export(&self, format: Format) -> Vec<u8>;
    fn import(&mut self, data: Vec<u8>);
    // Too many unrelated responsibilities!
}

// ✅ BETTER: Split into focused traits
pub trait Repository {
    type Item;
    fn create(&mut self, item: Self::Item) -> Result<Id, Error>;
    fn get(&self, id: &Id) -> Result<Option<Self::Item>, Error>;
    fn update(&mut self, id: &Id, item: Self::Item) -> Result<(), Error>;
    fn delete(&mut self, id: &Id) -> Result<(), Error>;
}

pub trait Searchable {
    type Query;
    type Item;
    fn search(&self, query: Self::Query) -> Result<Vec<Self::Item>, Error>;
}

pub trait Serializable {
    fn export(&self, format: Format) -> Result<Vec<u8>, Error>;
    fn import(&mut self, data: &[u8], format: Format) -> Result<(), Error>;
}
```

---

### 10.2 Generic Type Parameters

**Best Practices:**
- ✅ Use meaningful names for type parameters (not just `T`, `U`)
- ✅ Add trait bounds in impl blocks, not struct definitions
- ✅ Use `PhantomData` for unused type parameters
- ✅ Consider using `where` clauses for complex bounds

**Review Checkpoints:**
```rust
// ✅ GOOD: Meaningful type parameter names
pub struct Repository<Item, Storage> {
    storage: Storage,
    _marker: PhantomData<Item>,
}

// ✅ GOOD: Bounds in impl, not definition
impl<Item, S> Repository<Item, S>
where
    Item: Serialize + Deserialize,
    S: Storage<Item = Item>,
{
    pub fn store(&mut self, item: Item) -> Result<(), Error> {
        self.storage.store(item)
    }
}

// ✅ GOOD: Using where clause for readability
pub fn complex_operation<T, U, V>(
    input: T,
    transformer: U,
    validator: V,
) -> Result<Output, Error>
where
    T: Clone + Serialize + Send + 'static,
    U: Fn(T) -> V + Send + Sync,
    V: Validate + DeserializeOwned,
{
    // Implementation
}

// ❌ AVOID: Bounds in struct definition (limits flexibility)
pub struct Repository<Item: Serialize, Storage: Send> {
    storage: Storage,
    _marker: PhantomData<Item>,
}
// Now all impls require these bounds, even if not needed

// ❌ AVOID: Unclear type parameter names
pub struct Processor<T, U, V, W> {
    // What do T, U, V, W represent?
}

// ✅ BETTER:
pub struct Processor<Input, Output, State, Config> {
    // Clear what each represents
}
```

---

### 10.3 Trait Objects vs Generics

**Best Practices:**
- ✅ Use generics for static dispatch (zero-cost abstraction)
- ✅ Use trait objects (`dyn Trait`) for dynamic dispatch
- ✅ Add `+ Send + Sync` bounds to trait objects used across threads
- ✅ Make traits object-safe when needed (no `Self` sized requirements)

**Review Checkpoints:**
```rust
// ✅ GOOD: Generics for static dispatch (faster)
pub fn process_items<I, P>(items: I, processor: P) -> Vec<Output>
where
    I: Iterator<Item = Input>,
    P: Processor,
{
    items.map(|item| processor.process(item)).collect()
}

// ✅ GOOD: Trait objects for heterogeneous collections
pub struct LoopCoordinator {
    loops: Vec<Box<dyn TemporalLoop + Send + Sync>>,
}

impl LoopCoordinator {
    pub fn register_loop(&mut self, loop_: Box<dyn TemporalLoop + Send + Sync>) {
        self.loops.push(loop_);
    }
}

// ✅ GOOD: Object-safe trait (no Self: Sized)
pub trait TemporalLoop: Send + Sync {
    fn tick(&mut self) -> Result<(), Error>;
    fn name(&self) -> &str;

    // Object-safe: uses &self, not Self
    fn as_any(&self) -> &dyn Any;
}

// ❌ NOT object-safe (can't use as trait object)
pub trait NotObjectSafe {
    fn clone_box(&self) -> Self;  // Returns Self by value
    fn new() -> Self;              // Associated function without self
    fn generic_method<T>(&self, t: T);  // Generic method
}

// ✅ GOOD: Make cloneable trait object-safe
pub trait CloneableLoop: TemporalLoop {
    fn clone_box(&self) -> Box<dyn CloneableLoop>;
}

impl Clone for Box<dyn CloneableLoop> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
```

**When to use what:**
- **Generics**: When you know types at compile time, want best performance
- **Trait Objects**: When you need runtime polymorphism, heterogeneous collections

---

### 10.4 Newtype Pattern

**Best Practices:**
- ✅ Use newtypes for type safety (wrapping primitives)
- ✅ Implement `Deref` for transparent access when appropriate
- ✅ Don't implement `Deref` if it hides important invariants
- ✅ Use derive macros for common traits

**Review Checkpoints:**
```rust
// ✅ GOOD: Newtype for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(Uuid);

impl MemoryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

// ✅ GOOD: Newtype for units
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Milliseconds(f64);

impl Milliseconds {
    pub fn as_seconds(&self) -> f64 {
        self.0 / 1000.0
    }
}

// ✅ GOOD: Implement Display for nice output
impl std::fmt::Display for MemoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mem:{}", self.0)
    }
}

// ⚠️ CAREFUL: Only implement Deref if transparent access makes sense
use std::ops::Deref;

impl Deref for MemoryId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Now can use MemoryId anywhere Uuid is expected, but loses type safety!

// ❌ AVOID: Just using primitives
pub fn get_memory(id: String) -> Result<Memory, Error> {
    // Could pass any string, no type safety
}

// ✅ BETTER:
pub fn get_memory(id: MemoryId) -> Result<Memory, Error> {
    // Can only pass valid MemoryId
}
```

---

### 10.5 Type State Pattern

**Best Practices:**
- ✅ Use type states to enforce state machines at compile time
- ✅ Make invalid states unrepresentable
- ✅ Use `PhantomData` for zero-cost type states
- ✅ Provide clear transitions between states

**Review Checkpoints:**
```rust
// ✅ GOOD: Type state pattern for configuration
use std::marker::PhantomData;

// State markers
pub struct Uninitialized;
pub struct Initialized;
pub struct Running;

pub struct Runtime<State = Uninitialized> {
    config: Config,
    state: Option<RuntimeState>,
    _marker: PhantomData<State>,
}

// Only available in Uninitialized state
impl Runtime<Uninitialized> {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: None,
            _marker: PhantomData,
        }
    }

    pub fn initialize(mut self) -> Result<Runtime<Initialized>, Error> {
        let state = RuntimeState::new(&self.config)?;
        Ok(Runtime {
            config: self.config,
            state: Some(state),
            _marker: PhantomData,
        })
    }
}

// Only available in Initialized state
impl Runtime<Initialized> {
    pub async fn start(self) -> Result<Runtime<Running>, Error> {
        let mut runtime = Runtime {
            config: self.config,
            state: self.state,
            _marker: PhantomData,
        };
        runtime.start_internal().await?;
        Ok(runtime)
    }
}

// Only available in Running state
impl Runtime<Running> {
    pub async fn shutdown(mut self) -> Result<Runtime<Initialized>, Error> {
        self.shutdown_internal().await?;
        Ok(Runtime {
            config: self.config,
            state: self.state,
            _marker: PhantomData,
        })
    }

    pub async fn process(&mut self, input: Input) -> Result<Output, Error> {
        // Only callable when running
        self.state.as_mut().unwrap().process(input).await
    }
}

// Usage:
let runtime = Runtime::new(config)
    .initialize()?        // Uninitialized -> Initialized
    .start().await?;      // Initialized -> Running

runtime.process(input).await?;  // Only works in Running state
// runtime.initialize();  // ❌ Compile error! Already initialized
```

---

## Summary Checklist

### Quick Review Guide

**Architecture & Organization**
- [ ] Clear module structure with appropriate visibility
- [ ] Workspace configuration for multi-crate projects
- [ ] Logical separation of concerns
- [ ] Minimal dependencies

**API Design**
- [ ] Builder pattern for complex constructors
- [ ] Clear, idiomatic naming conventions
- [ ] Iterator support for collections
- [ ] Type-safe APIs (avoid stringly-typed)

**Safety**
- [ ] No unnecessary `unsafe` blocks
- [ ] Proper bounds checking
- [ ] Safe arithmetic (checked/saturating)
- [ ] RAII for resource management

**Performance**
- [ ] Pre-allocated collections
- [ ] Avoid unnecessary clones
- [ ] Efficient string handling
- [ ] Iterator chains over manual loops

**Error Handling**
- [ ] Custom error types with `thiserror`
- [ ] `Result` types for fallible operations
- [ ] Context in error messages
- [ ] No `unwrap()` in library code

**Testing**
- [ ] Unit tests for all public APIs
- [ ] Integration tests for workflows
- [ ] Property-based tests for invariants
- [ ] Benchmarks for critical paths

**Documentation**
- [ ] Crate-level documentation
- [ ] Function/method documentation with examples
- [ ] Type documentation
- [ ] README with usage examples

**Concurrency**
- [ ] Proper async/await usage
- [ ] `tokio::sync` primitives
- [ ] Graceful shutdown
- [ ] Bounded task spawning

**Type System**
- [ ] Focused, composable traits
- [ ] Appropriate use of generics vs trait objects
- [ ] Newtype pattern for type safety
- [ ] Type states where beneficial

---

## Omega-Specific Review Focus

### Multi-Crate Consistency
- [ ] Consistent error handling across all crates
- [ ] Shared types in `omega-core`
- [ ] Consistent async patterns
- [ ] Workspace dependency versions aligned

### Domain-Specific Patterns
- [ ] Memory tier operations are safe and efficient
- [ ] Loop coordination is deadlock-free
- [ ] Neural operations handle numerical stability
- [ ] SIMD operations are properly unsafe-wrapped

### Performance Characteristics
- [ ] Memory allocations in hot paths minimized
- [ ] Lock contention in async code avoided
- [ ] Database queries optimized
- [ ] Batch operations where applicable

### Testing Coverage
- [ ] Core types have comprehensive tests
- [ ] Integration tests cover cross-crate interactions
- [ ] Examples are tested and working
- [ ] Benchmarks track performance regressions

---

## References & Further Reading

**Official Rust Resources:**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Rustonomicon](https://doc.rust-lang.org/nomicon/) (unsafe Rust)

**Ecosystem Best Practices:**
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Serde Documentation](https://serde.rs/)
- [Error Handling (thiserror)](https://github.com/dtolnay/thiserror)

**Testing & Quality:**
- [Criterion Benchmarking](https://github.com/bheisler/criterion.rs)
- [PropTest](https://github.com/proptest-rs/proptest)
- [Cargo Audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)

**Performance:**
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rust Async Performance](https://tokio.rs/tokio/topics/performance)

---

**END OF DOCUMENT**

*This checklist should be referenced during code reviews to ensure comprehensive coverage of Rust best practices for the Omega codebase.*
