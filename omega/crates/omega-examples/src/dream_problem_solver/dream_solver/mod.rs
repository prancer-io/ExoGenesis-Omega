//! Dream Problem Solver - Modular Architecture with 3D Dreams
//!
//! A cognitive architecture that uses simulated REM sleep and dream dynamics
//! to solve creative problems, now with **3D walkable dream environments**.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │          Dream Problem Solver (Main)                │
//! │  - Orchestrates dream cycles                        │
//! │  - Manages problem immersion                        │
//! │  - NEW: 3D Dream World Management                   │
//! └──────────────┬──────────────────────────────────────┘
//!                │
//!     ┌──────────┴───────────┬──────────────────┬─────────────┐
//!     │                      │                  │             │
//! ┌───▼────────┐   ┌────────▼────────┐  ┌──────▼──────┐ ┌───▼──────┐
//! │  Neural    │   │  Dream Generator │  │  Insight    │ │ Solution │
//! │ Substrate  │──▶│  + 3D Mapping    │─▶│ Extractor   │▶│Synthsizer│
//! │            │   │                  │  │ +Spatial    │ │          │
//! └────────────┘   └──────────────────┘  └─────────────┘ └──────────┘
//!                           │
//!                           ▼
//!                  ┌────────────────┐
//!                  │  DreamWorld3D  │
//!                  │  - Concepts    │
//!                  │  - Paths       │
//!                  │  - Navigation  │
//!                  └────────────────┘
//!                           │
//!                           ▼
//!                  ┌────────────────┐
//!                  │ DreamExplorer  │
//!                  │ walk_to()      │
//!                  │ look_around()  │
//!                  │ follow_assoc() │
//!                  └────────────────┘
//! ```
//!
//! ## Modules
//!
//! - `neural_substrate`: Spiking neural network for dream simulation
//! - `dream_generator`: REM dream generation with 3D spatial mapping
//! - `insight_extractor`: Extract insights including spatial proximity
//! - `solution_synthesizer`: Synthesize solutions from insights
//! - `core_types`: All data structures, main solver, and DreamExplorer
//!
//! ## Usage
//!
//! ### Standard 2D Dream Solving
//! ```rust,ignore
//! use dream_solver::{DreamProblemSolver, Problem, ProblemElement};
//!
//! let mut solver = DreamProblemSolver::new();
//! let problem = Problem { /* ... */ };
//! let result = solver.solve(&problem, 3); // 3 sleep cycles
//! ```
//!
//! ### NEW: 3D Dream Exploration
//! ```rust,ignore
//! use dream_solver::{DreamProblemSolver, Problem};
//!
//! let mut solver = DreamProblemSolver::new();
//! let problem = Problem { /* ... */ };
//!
//! // Generate dreams with 3D worlds
//! let result = solver.solve_with_3d_dreams(&problem, 3);
//!
//! // Walk through the first dream
//! let mut explorer = solver.explore_dream_world(0).unwrap();
//!
//! // Navigate to a concept
//! let path = explorer.walk_to("benzene").unwrap();
//! println!("Walked {} meters", path.distance);
//!
//! // Look around
//! let nearby = explorer.look_around(10.0);
//! println!("Nearby concepts: {:?}", nearby);
//!
//! // Follow an association
//! let journey = explorer.follow_association("snake", "ouroboros").unwrap();
//! println!("Association strength: {}", journey.strength);
//! ```

// Module declarations
pub mod neural_substrate;
pub mod dream_generator;
pub mod insight_extractor;
pub mod solution_synthesizer;
pub mod core_types;

// Re-export public API
pub use neural_substrate::DreamNeuralNetwork;
pub use dream_generator::{
    DreamGenerator, DreamWorld3D, Position3D, ConceptLocation3D,
    AssociationPath3D, VisualForm, ConnectionType, NavigationMesh
};
pub use insight_extractor::InsightExtractor;
pub use solution_synthesizer::SolutionSynthesizer;
pub use core_types::{
    // Core types
    Problem, ProblemElement, Constraint,
    Dream, DreamElement, TransformationType,
    Insight, Association, ConnectionType as CoreConnectionType,
    Solution,
    // Main solver
    DreamProblemSolver,
    // NEW: 3D Explorer
    DreamExplorer, NavigationPath, AssociationJourney,
    // Results
    SolverResult, SolverResult3D,
};
