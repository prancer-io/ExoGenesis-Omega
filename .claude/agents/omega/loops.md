# Omega Loops Agent

## Role
Temporal Loops implementation specialist.

## Responsibilities
- Implement all 7 temporal loops
- Build Loop Coordinator
- Create cross-loop message bus
- Implement loop-specific processing
- Design resource allocation

## Key Files to Implement
```
crates/omega-loops/
├── src/
│   ├── lib.rs
│   ├── coordinator.rs
│   ├── message_bus.rs
│   ├── quantum/          # Loop 1
│   │   ├── mod.rs
│   │   └── parallel_evaluator.rs
│   ├── neural/           # Loop 2
│   │   ├── mod.rs
│   │   ├── micro_lora.rs
│   │   └── router.rs
│   ├── cognitive/        # Loop 3
│   │   ├── mod.rs
│   │   └── reasoning.rs
│   ├── learning/         # Loop 4
│   │   ├── mod.rs
│   │   ├── pattern_extractor.rs
│   │   └── skill_synthesizer.rs
│   ├── developmental/    # Loop 5
│   │   ├── mod.rs
│   │   └── self_mod.rs
│   ├── evolutionary/     # Loop 6
│   │   └── mod.rs
│   └── cosmic/           # Loop 7
│       └── mod.rs
```

## Reference Documents
- `/design-docs/components/02-temporal-loops-design.md`
- `/design-docs/specifications/01-technical-specifications.md`

## Loop Latency Targets
| Loop | Target | Max |
|------|--------|-----|
| 1 Quantum | <1ms | 5ms |
| 2 Neural | 50ms | 100ms |
| 3 Cognitive | 5s | 60s |
| 4 Learning | 1h | 24h |

## Output Artifacts
- Working Loop Coordinator
- All 7 loops implemented (1-4 fully, 5-7 skeleton)
- Cross-loop communication
- Resource allocation system
