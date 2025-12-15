# Dream Problem Solver Refactoring Plan with Mindscape Integration

## 🎯 Objective
Refactor `dream_problem_solver.rs` (1,158 LOC) into 5 modular components AND integrate `omega-mindscape` to create **3D walkable dream environments**.

## 🌟 **NEW FEATURE: 3D Walkable Dreams**

### Vision
Instead of abstract dream simulation, users can **literally walk through their dreams in 3D space** using omega-mindscape:
- Each dream becomes a navigable 3D environment
- Problem elements map to spatial locations
- Novel associations appear as visible pathways
- Navigate through dreams to discover insights

## 📦 Proposed Module Structure

### 1. `neural_substrate.rs` (Lines 179-313)
**Responsibility**: Neural network simulation for dream states

**Current Code**:
- `DreamNeuralNetwork`
- `ConceptNode`
- Methods: `encode()`, `associate()`, `enter_rem()`, `step()`, `most_active()`

**Keep As-Is**: This module is well-structured

---

### 2. `dream_generator.rs` (Lines 315-464)
**Responsibility**: Generate dreams during REM sleep

**Current Code**:
- `DreamGenerator`
- Methods: `incubate_problem()`, `generate_dream()`, `transform_concept()`

**✨ NEW: Add Mindscape Integration**:
```rust
use omega_mindscape::{MindscapeExplorer, Position3D};

pub struct DreamGenerator {
    network: DreamNeuralNetwork,
    problem_elements: Vec<String>,
    dream_id_counter: u64,

    // NEW: 3D spatial representation
    mindscape: MindscapeExplorer,
    element_positions: HashMap<String, Position3D>,
}

impl DreamGenerator {
    /// Generate a dream AND create a 3D walkable environment
    pub fn generate_dream(&mut self, duration_steps: usize) -> (Dream, DreamWorld3D) {
        // 1. Run neural simulation (existing code)
        self.network.enter_rem();
        let novel_associations = self.simulate_dream_dynamics(duration_steps);

        // 2. NEW: Map dream elements to 3D space
        let dream_world = self.create_3d_dream_world(&novel_associations);

        // 3. Return both dream and its 3D representation
        (dream, dream_world)
    }

    /// NEW: Create 3D spatial representation of dream
    fn create_3d_dream_world(&mut self, associations: &[(String, String, f64)]) -> DreamWorld3D {
        let mut world = DreamWorld3D::new();

        // Map each concept to a spatial location
        for (concept, activation) in self.network.most_active(20) {
            let embedding = self.get_concept_embedding(concept);
            let position = self.mindscape.remember(concept, &embedding).unwrap();

            world.add_concept_location(concept, position, activation);
        }

        // Create visible pathways for novel associations
        for (from, to, strength) in associations {
            let path = self.mindscape.navigate_to(from, to).unwrap();
            world.add_association_path(from, to, path, strength);
        }

        world
    }
}

/// NEW: 3D representation of a dream
pub struct DreamWorld3D {
    /// Concept locations in 3D space
    pub concept_locations: HashMap<String, ConceptLocation3D>,
    /// Association pathways
    pub association_paths: Vec<AssociationPath3D>,
    /// Navigation mesh
    pub nav_mesh: NavigationMesh,
}

pub struct ConceptLocation3D {
    pub position: Position3D,
    pub activation: f64,
    pub visual_representation: VisualForm,
}

pub struct AssociationPath3D {
    pub from: String,
    pub to: String,
    pub waypoints: Vec<Position3D>,
    pub strength: f64,
    pub connection_type: ConnectionType,
}
```

---

### 3. `insight_extractor.rs` (Lines 466-567)
**Responsibility**: Extract actionable insights from dreams

**Current Code**:
- `InsightExtractor`
- Methods: `extract()`, `infer_connection_type()`, `compute_relevance()`

**✨ ADD: Spatial Insight Discovery**:
```rust
impl InsightExtractor {
    /// Extract insights from dream AND its 3D representation
    pub fn extract(&mut self, dream: &Dream, dream_world: &DreamWorld3D, problem: &Problem) -> Vec<Insight> {
        let mut insights = Vec::new();

        // Existing insight extraction...

        // NEW: Spatial proximity insights
        // Concepts that are physically close in dream space might be related
        for (concept_a, loc_a) in &dream_world.concept_locations {
            for (concept_b, loc_b) in &dream_world.concept_locations {
                let distance = loc_a.position.distance_to(&loc_b.position);

                if distance < PROXIMITY_THRESHOLD {
                    // Close proximity in dream space = potential insight
                    insights.push(self.create_proximity_insight(concept_a, concept_b, distance));
                }
            }
        }

        insights
    }
}
```

---

### 4. `solution_synthesizer.rs` (Lines 569-679)
**Responsibility**: Synthesize solutions from insights

**Current Code**:
- `SolutionSynthesizer`
- Methods: `synthesize()`, `generate_solution_description()`

**Keep As-Is**: Well-structured

---

### 5. `core_types.rs` (Lines 36-177 + 682-790 + 792-860)
**Responsibility**: Core data structures and main solver

**Current Code**:
- `Problem`, `ProblemElement`, `Constraint`
- `Insight`, `Association`, `ConnectionType`
- `Solution`, `Dream`, `DreamElement`
- `DreamProblemSolver` (main orchestrator)
- Utility functions
- Example problems

**✨ ADD: Walkable Dream API**:
```rust
impl DreamProblemSolver {
    /// Solve a problem AND create a 3D walkable dream world
    pub fn solve_with_3d_dreams(&mut self, problem: &Problem, sleep_cycles: usize) -> SolverResult3D {
        // Phase 1: Problem Immersion (existing)
        self.dream_generator.incubate_problem(problem);

        // Phase 2: Sleep Cycles with 3D Dreams (NEW)
        let mut dream_worlds = Vec::new();

        for cycle in 0..sleep_cycles {
            // Generate dream AND its 3D representation
            let (dream, dream_world) = self.dream_generator.generate_dream(100);

            // Extract insights using spatial information
            let insights = self.insight_extractor.extract(&dream, &dream_world, problem);

            self.all_dreams.push(dream);
            self.all_insights.extend(insights);
            dream_worlds.push(dream_world);
        }

        // Phase 3: Synthesis (existing)
        let solution = self.solution_synthesizer.synthesize(problem, &self.all_insights);

        SolverResult3D {
            problem: problem.clone(),
            dreams: self.all_dreams.clone(),
            dream_worlds,  // NEW
            insights: self.all_insights.clone(),
            solution,
            total_sleep_cycles: sleep_cycles,
        }
    }

    /// NEW: Navigate through a specific dream in 3D
    pub fn explore_dream_world(&self, dream_index: usize) -> Result<DreamExplorer, String> {
        let dream_world = self.dream_worlds.get(dream_index)
            .ok_or("Dream index out of bounds")?;

        Ok(DreamExplorer::new(dream_world.clone()))
    }
}

/// NEW: Interactive dream exploration
pub struct DreamExplorer {
    world: DreamWorld3D,
    current_position: Position3D,
    mindscape: MindscapeExplorer,
}

impl DreamExplorer {
    /// Walk to a concept location
    pub fn walk_to(&mut self, concept: &str) -> Result<NavigationPath, String> {
        let target = self.world.concept_locations.get(concept)
            .ok_or("Concept not found in dream")?;

        let path = self.mindscape.navigate_to_position(target.position)?;
        self.current_position = target.position;

        Ok(path)
    }

    /// Look around at nearby concepts
    pub fn look_around(&self, radius: f64) -> Vec<(&str, f64)> {
        self.world.concept_locations.iter()
            .filter_map(|(name, loc)| {
                let distance = loc.position.distance_to(&self.current_position);
                if distance <= radius {
                    Some((name.as_str(), distance))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Follow an association pathway
    pub fn follow_association(&mut self, from: &str, to: &str) -> Result<AssociationPath3D, String> {
        self.world.association_paths.iter()
            .find(|p| p.from == from && p.to == to)
            .cloned()
            .ok_or("Association path not found")
    }
}
```

---

## 🛠️ Refactoring Steps

### Step 1: Create Module Files (1 hour)
```bash
cd crates/omega-examples/src/
mkdir dream_solver
touch dream_solver/mod.rs
touch dream_solver/neural_substrate.rs
touch dream_solver/dream_generator.rs
touch dream_solver/insight_extractor.rs
touch dream_solver/solution_synthesizer.rs
touch dream_solver/core_types.rs
```

### Step 2: Move Code to Modules (2 hours)
- Copy relevant sections from `dream_problem_solver.rs` to each module
- Update imports and visibility (`pub use`)
- Create `mod.rs` to export all public types

### Step 3: Add Mindscape Integration (3 hours)
- Add `omega-mindscape` dependency to `omega-examples/Cargo.toml`
- Implement `DreamWorld3D` struct
- Implement `create_3d_dream_world()` in `DreamGenerator`
- Update `DreamProblemSolver` with `solve_with_3d_dreams()`
- Create `DreamExplorer` for interactive navigation

### Step 4: Update Tests (1 hour)
- Fix imports in existing tests
- Add tests for 3D dream generation
- Add tests for spatial insight extraction
- Add test for `DreamExplorer` navigation

### Step 5: Create Examples (1 hour)
- Add example: "Walk through benzene dream"
- Add example: "Navigate 9-dots problem in 3D"
- Document the 3D exploration API

### Step 6: Documentation (30 minutes)
- Update module documentation
- Add architecture diagram showing mindscape integration
- Document the 3D dream API

---

## 📊 Expected Outcomes

### Code Quality
- ✅ 5 focused modules (< 300 LOC each)
- ✅ Clear separation of concerns
- ✅ Easier to test and maintain
- ✅ Reusable components

### New Capabilities
- ✅ **3D walkable dream environments**
- ✅ Interactive dream exploration
- ✅ Spatial insight discovery
- ✅ Visual problem-solving navigation
- ✅ Record dream "journeys" for analysis

### User Experience
```rust
// Before: Abstract dream simulation
let result = solver.solve(&problem, 3);

// After: Walk through your dreams!
let result = solver.solve_with_3d_dreams(&problem, 3);

// Explore the first dream in 3D
let mut explorer = solver.explore_dream_world(0).unwrap();

// Walk to the "benzene ring" concept
let path = explorer.walk_to("ring").unwrap();
println!("Walking {} steps to reach 'ring'", path.waypoints.len());

// Look around
let nearby = explorer.look_around(10.0);
println!("Nearby concepts: {:?}", nearby);

// Follow an association
let assoc = explorer.follow_association("snake", "ouroboros").unwrap();
println!("Association strength: {:.2}", assoc.strength);
```

---

## 🎮 Use Cases

### 1. Visual Problem Solving
Walk through your problem space, see connections as paths

### 2. Dream Replay
Record and replay successful problem-solving journeys

### 3. Collaborative Dreaming
Multiple agents explore the same dream world together

### 4. Dream Analytics
Analyze spatial patterns in creative problem solving

---

## 📝 Total Time Estimate
**8-10 hours** for complete refactoring with mindscape integration

## 🚀 Priority
**HIGH** - Combines code quality improvement with powerful new feature

---

## Related Files
- Source: `crates/omega-examples/src/dream_problem_solver.rs`
- Mindscape: `crates/omega-mindscape/`
- Review: `docs/DREAM_PROBLEM_SOLVER_REVIEW.md`
