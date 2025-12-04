# ExoGenesis Omega Swarm Agents

## Overview
This directory contains agent definitions for the ExoGenesis Omega development swarm.

## Agent Hierarchy

```
                    ┌─────────────────┐
                    │   Coordinator   │
                    │   (orchestrates)│
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│   Architect   │  │    Memory     │  │    Loops      │
│  (META-SONA)  │  │  (AgentDB)    │  │  (7 loops)    │
└───────────────┘  └───────────────┘  └───────────────┘
        │                    │                    │
        ▼                    ▼                    ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│ Verification  │  │  Integration  │  │   Researcher  │
│  (alignment)  │  │    (APIs)     │  │    (docs)     │
└───────────────┘  └───────────────┘  └───────────────┘
                             │
                             ▼
                    ┌───────────────┐
                    │    Tester     │
                    │     (QA)      │
                    └───────────────┘
```

## Agents

| Agent | Role | Specialization |
|-------|------|----------------|
| coordinator | Orchestration | Task distribution, monitoring |
| architect | Specialist | META-SONA, architecture design |
| memory | Specialist | Cosmic Memory, AgentDB |
| loops | Specialist | Temporal loops 1-7 |
| verification | Specialist | Alignment, verification |
| integration | Specialist | APIs, protocols |
| researcher | Researcher | Documentation, analysis |
| tester | Tester | Quality assurance |

## Commands

```bash
# Start swarm
./claude-flow swarm start

# Check status
./claude-flow swarm status

# Spawn specific agent with task
./claude-flow swarm spawn --agent architect --task "Implement MCTS"

# Monitor swarm activity
./claude-flow swarm monitor

# Run hive-mind consensus
./claude-flow hive-mind consensus --topic "Architecture decision"
```

## Configuration
See `/.claude-flow/swarm-config.json` for full swarm configuration.

## Phase Mapping

| Phase | Primary Agents | Focus |
|-------|---------------|-------|
| 0 | Memory, Integration | AgentDB setup |
| 1 | Loops, Memory | Loops 1-4, Tier 1-4 |
| 2 | Architect, Verification | META-SONA |
| 3 | All | Higher loops, distribution |
