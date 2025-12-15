//! # omega-examples
//!
//! Example applications demonstrating ExoGenesis Omega brain capabilities.
//!
//! ## Available Examples
//!
//! - **loops_demo**: Demonstrates the 7 temporal cognitive loops
//! - **dream_problem_solver**: Creative problem solving using simulated REM sleep with 3D walkable dreams
//!
//! ## Running Examples
//!
//! ```bash
//! # Run the loops demo
//! cargo run --bin loops_demo
//! ```
//!
//! ## Library Usage
//!
//! ```rust
//! use omega_examples::dream_problem_solver::{DreamProblemSolver, benzene_problem};
//!
//! let mut solver = DreamProblemSolver::new();
//! let problem = benzene_problem();
//!
//! // Standard 2D solving
//! let result = solver.solve(&problem, 3);
//!
//! // NEW: 3D walkable dreams
//! let result = solver.solve_with_3d_dreams(&problem, 3);
//! let mut explorer = solver.explore_dream_world(0).unwrap();
//! explorer.walk_to("benzene").unwrap();
//! ```
//!
//! ## Use Cases
//!
//! See `USE_CASES.md` in the omega/examples folder for detailed use case descriptions.

pub mod common;
pub mod dream_problem_solver;

pub use common::*;
