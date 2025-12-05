# ExoGenesis Omega - Crate User Guides

This directory contains detailed user guides for each crate in the ExoGenesis Omega workspace.

## Available Guides

1. **[omega-core](./omega-core.md)** - Core types and traits foundation
2. **[omega-agentdb](./omega-agentdb.md)** - SIMD-optimized vector database with HNSW (13-41x speedup)
3. **[omega-memory](./omega-memory.md)** - 12-tier cosmic memory system (Instant → Omega)
4. **[omega-loops](./omega-loops.md)** - 7 temporal cognitive loops (100ms → 10 years)
5. **[omega-meta-sona](./omega-meta-sona.md)** - Self-organizing neural architecture (MCTS + PPO)
6. **[omega-runtime](./omega-runtime.md)** - Production runtime orchestrator with health monitoring
7. **[omega-persistence](./omega-persistence.md)** - SQLite persistence layer with ACID guarantees

## Quick Navigation

### By Use Case

**Building a New Intelligence System?**
- Start with [omega-runtime](./omega-runtime.md) for the main orchestrator
- Use [omega-meta-sona](./omega-meta-sona.md) to design architectures
- Reference [omega-core](./omega-core.md) for type definitions

**Implementing Memory Systems?**
- Read [omega-memory](./omega-memory.md) for the 12-tier system
- Check [omega-agentdb](./omega-agentdb.md) for SIMD-optimized vector storage
- Review [omega-persistence](./omega-persistence.md) for durability

**Working with Temporal Processing?**
- Study [omega-loops](./omega-loops.md) for the 7-loop system
- See [omega-core](./omega-core.md) for loop types

**Optimizing Performance?**
- See [omega-agentdb](./omega-agentdb.md) for SIMD optimization details
- Check [omega-runtime](./omega-runtime.md) for circuit breakers and retry policies
- Review [omega-memory](./omega-memory.md) for memory consolidation strategies

### By Developer Experience

**Beginners**: Start with omega-runtime → omega-memory → omega-core
**Intermediate**: omega-loops → omega-meta-sona → omega-agentdb
**Advanced**: omega-persistence → Design docs → Source code

## Documentation Structure

Each crate guide includes:
- **Overview**: Purpose, key features, and performance characteristics
- **Installation**: Setup instructions and dependency requirements
- **Core Concepts**: Key abstractions, patterns, and architecture
- **API Reference**: Complete API with types, traits, functions, and examples
- **Common Patterns**: Real-world usage patterns and best practices
- **Best Practices**: DO/DON'T guidelines and recommendations
- **Error Handling**: Error types and recovery strategies
- **Performance Optimization**: Benchmarks, tuning, and optimization tips
- **Integration Examples**: Working with other Omega crates
- **Testing**: Unit test examples and testing strategies
- **Advanced Topics**: Deep dives into complex features

## Additional Resources

- **Main User Guide**: [../user-guides/00-MAIN-USER-GUIDE.md](../user-guides/00-MAIN-USER-GUIDE.md)
- **Design Documents**: [../../design-docs/](../../design-docs/)
- **Examples**: See each crate's `examples/` directory
- **Tests**: See each crate's `src/tests.rs` or `tests/` directory

## Quick Reference

### Performance Highlights

- **omega-agentdb**: 13-41x SIMD speedup (AVX2/AVX-512)
- **omega-memory**: 12-tier system with auto-consolidation
- **omega-loops**: 7 concurrent temporal loops (100ms to 10 years)
- **omega-meta-sona**: 86.42% fitness in META-SONA simulations
- **omega-runtime**: Circuit breakers, retry policies, graceful degradation

### System Requirements

- **Rust**: 1.70+ (2021 edition)
- **OS**: Linux, macOS, Windows
- **CPU**: x86_64 with AVX2 (AVX-512 for best performance)
- **RAM**: 4GB minimum, 16GB+ recommended for large datasets
- **Storage**: SQLite 3.35+ (bundled)

### Test Coverage

All 7 crates: **228 tests passing (100% core API coverage)**

---

**Status**: ✅ All guides complete (v0.1.0)
**Last Updated**: 2025-01-05
