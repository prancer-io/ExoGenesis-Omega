# ExoGenesis Omega - Crate User Guides

This directory contains detailed user guides for each crate in the ExoGenesis Omega workspace.

## Available Guides

1. **[omega-core](./01-omega-core.md)** - Core types and traits foundation
2. **[omega-agentdb](./02-omega-agentdb.md)** - Vector database with HNSW indexing
3. **[omega-memory](./03-omega-memory.md)** - 12-tier hierarchical memory system
4. **[omega-loops](./04-omega-loops.md)** - 7 temporal loop execution engine
5. **[omega-meta-sona](./05-omega-meta-sona.md)** - Self-optimizing architecture design
6. **[omega-runtime](./06-omega-runtime.md)** - Production runtime orchestrator
7. **[omega-persistence](./07-omega-persistence.md)** - SQLite persistence layer

## Quick Navigation

### By Use Case

**Building a New Intelligence System?**
- Start with [omega-runtime](./06-omega-runtime.md) for the main orchestrator
- Use [omega-meta-sona](./05-omega-meta-sona.md) to design architectures
- Reference [omega-core](./01-omega-core.md) for type definitions

**Implementing Memory Systems?**
- Read [omega-memory](./03-omega-memory.md) for the tier system
- Check [omega-agentdb](./02-omega-agentdb.md) for vector storage
- Review [omega-persistence](./07-omega-persistence.md) for durability

**Working with Temporal Processing?**
- Study [omega-loops](./04-omega-loops.md) for the 7-loop system
- See [omega-core](./01-omega-core.md) for loop types

### By Developer Experience

**Beginners**: Start with omega-runtime → omega-memory → omega-core
**Intermediate**: omega-loops → omega-meta-sona → omega-agentdb
**Advanced**: omega-persistence → Design docs → Source code

## Documentation Structure

Each crate guide includes:
- **Overview**: Purpose and key features
- **Installation**: Setup and dependencies
- **Core Concepts**: Key abstractions and patterns
- **API Reference**: Main types, traits, and functions
- **Usage Examples**: Real-world code samples
- **Best Practices**: Recommended patterns
- **Troubleshooting**: Common issues and solutions
- **Performance**: Optimization tips

## Additional Resources

- **Main User Guide**: [../user-guides/00-MAIN-USER-GUIDE.md](../user-guides/00-MAIN-USER-GUIDE.md)
- **Design Documents**: [../../design-docs/](../../design-docs/)
- **Examples**: See each crate's `examples/` directory
- **Tests**: See each crate's `src/tests.rs` or `tests/` directory

---

Last Updated: 2025-12-05
