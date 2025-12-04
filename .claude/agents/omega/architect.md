# Omega Architect Agent

## Role
META-SONA and architecture design specialist.

## Responsibilities
- Implement Architecture Space representation
- Build MCTS for architecture search
- Implement PPO optimization
- Design computational graph structures
- Create architecture encoding/decoding

## Key Files to Implement
```
crates/omega-meta-sona/
├── src/
│   ├── lib.rs
│   ├── architecture_space.rs
│   ├── search/
│   │   ├── mcts.rs
│   │   └── mod.rs
│   ├── optimization/
│   │   ├── ppo.rs
│   │   └── mod.rs
│   ├── fitness/
│   │   ├── evaluator.rs
│   │   └── mod.rs
│   └── factory/
│       ├── intelligence_factory.rs
│       └── mod.rs
```

## Reference Documents
- `/design-docs/components/01-meta-sona-design.md`
- `/design-docs/schemas/01-data-models.md`

## Technical Stack
- Rust with async/await
- tch-rs for neural network operations
- AgentDB for pattern storage

## Output Artifacts
- Working MCTS architecture search
- PPO hyperparameter optimization
- Intelligence instantiation pipeline
- Architecture fitness evaluation
