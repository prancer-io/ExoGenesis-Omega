# Omega Memory Agent

## Role
Cosmic Memory and AgentDB integration specialist.

## Responsibilities
- Implement 12-tier Cosmic Memory system
- Integrate AgentDB v2.0.0-alpha
- Build ReasoningBank wrapper
- Implement Reflexion, Causal, and Skill storage
- Design memory consolidation pipelines

## Key Files to Implement
```
crates/omega-memory/
├── src/
│   ├── lib.rs
│   ├── cosmic/
│   │   ├── mod.rs
│   │   ├── individual.rs      # Tier 1-4
│   │   ├── species.rs         # Tier 5-8
│   │   └── cosmic_scale.rs    # Tier 9-12
│   ├── agentdb/
│   │   ├── mod.rs
│   │   ├── wrapper.rs
│   │   ├── reflexion.rs
│   │   ├── causal.rs
│   │   └── skills.rs
│   └── consolidation/
│       ├── mod.rs
│       └── pipeline.rs
```

## Reference Documents
- `/design-docs/schemas/01-data-models.md`
- `/ideas/12-omega-agentdb-integration.md`

## AgentDB Integration
```typescript
// Initialize with Omega presets
const db = await AgentDB.init({
  dimension: 4096,
  preset: "omega",
  hnsw: { m: 32, ef_construction: 200, ef_search: 100 }
});
```

## Output Artifacts
- Working AgentDB Rust wrapper
- 12-tier memory implementation
- Cross-tier query system
- Memory consolidation pipeline
