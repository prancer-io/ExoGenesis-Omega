//! Dream Problem Solver - Now Modular with 3D Dreams!
//!
//! This file maintains backwards compatibility by re-exporting from the new
//! modular `dream_solver` architecture.
//!
//! ## What Changed?
//!
//! The 1,158-line monolithic file has been refactored into 5 focused modules:
//! - `dream_solver/neural_substrate.rs` - Neural network simulation
//! - `dream_solver/dream_generator.rs` - Dream generation + **3D mapping**
//! - `dream_solver/insight_extractor.rs` - Insight extraction + **spatial discovery**
//! - `dream_solver/solution_synthesizer.rs` - Solution synthesis
//! - `dream_solver/core_types.rs` - Types + main solver + **DreamExplorer API**
//!
//! ## NEW: 3D Walkable Dreams
//!
//! Dreams can now be explored as 3D spatial environments!
//!
//! ```rust,ignore
//! use omega_examples::dream_problem_solver::{DreamProblemSolver, Problem};
//!
//! let mut solver = DreamProblemSolver::new();
//! let problem = /* your problem */;
//!
//! // Generate dreams with 3D worlds
//! let result = solver.solve_with_3d_dreams(&problem, 3);
//!
//! // Walk through the first dream
//! let mut explorer = solver.explore_dream_world(0).unwrap();
//!
//! // Navigate to a concept
//! explorer.walk_to("benzene")?;
//!
//! // Look around
//! let nearby = explorer.look_around(10.0);
//!
//! // Follow associations
//! explorer.follow_association("snake", "ouroboros")?;
//! ```
//!
//! ## Original API Still Works
//!
//! All existing code continues to work:
//! ```rust,ignore
//! let result = solver.solve(&problem, 3); // Standard 2D solving
//! ```

// Declare the new modular structure
pub mod dream_solver;

// Re-export everything for backwards compatibility
pub use dream_solver::*;

// Keep example problems from original file

/// Example problem: The benzene ring discovery (Kekulé's dream)
pub fn benzene_problem() -> Problem {
    Problem {
        id: "benzene_1865".to_string(),
        description: "Determine the molecular structure of benzene (C6H6)".to_string(),
        elements: vec![
            ProblemElement {
                name: "carbon_atoms".to_string(),
                concept: "6 carbon atoms".to_string(),
                embedding: vec![0.8; 32],
                importance: 0.9,
                relations: vec![("bonds".to_string(), 0.9)],
            },
            ProblemElement {
                name: "hydrogen_atoms".to_string(),
                concept: "6 hydrogen atoms".to_string(),
                embedding: vec![0.6; 32],
                importance: 0.7,
                relations: vec![("bonds".to_string(), 0.8)],
            },
            ProblemElement {
                name: "bonds".to_string(),
                concept: "Chemical bonds between atoms".to_string(),
                embedding: vec![0.7; 32],
                importance: 0.95,
                relations: vec![
                    ("carbon_atoms".to_string(), 0.9),
                    ("hydrogen_atoms".to_string(), 0.8),
                ],
            },
            ProblemElement {
                name: "stability".to_string(),
                concept: "Unusual chemical stability".to_string(),
                embedding: vec![0.75; 32],
                importance: 0.85,
                relations: vec![("structure".to_string(), 0.9)],
            },
            ProblemElement {
                name: "structure".to_string(),
                concept: "Unknown molecular structure".to_string(),
                embedding: vec![0.9; 32],
                importance: 1.0,
                relations: vec![
                    ("carbon_atoms".to_string(), 0.95),
                    ("bonds".to_string(), 0.9),
                ],
            },
        ],
        constraints: vec![
            Constraint {
                description: "Must form closed structure".to_string(),
                hard: true,
                check: |_| true,
            },
            Constraint {
                description: "Carbon must have 4 bonds each".to_string(),
                hard: true,
                check: |_| true,
            },
        ],
        failed_approaches: vec![
            "Linear chain structure".to_string(),
            "Branched structure".to_string(),
        ],
        domains: vec!["chemistry".to_string(), "organic molecules".to_string()],
        embedding: vec![0.85; 32],
    }
}

/// Example problem: The 9-dots puzzle
pub fn nine_dots_problem() -> Problem {
    Problem {
        id: "nine_dots".to_string(),
        description: "Connect all 9 dots with 4 straight lines without lifting pen".to_string(),
        elements: vec![
            ProblemElement {
                name: "dots".to_string(),
                concept: "9 dots in 3x3 grid".to_string(),
                embedding: vec![0.7; 32],
                importance: 0.9,
                relations: vec![("constraints".to_string(), 0.8)],
            },
            ProblemElement {
                name: "constraints".to_string(),
                concept: "4 lines, continuous, straight".to_string(),
                embedding: vec![0.85; 32],
                importance: 0.95,
                relations: vec![("solution_space".to_string(), 0.9)],
            },
            ProblemElement {
                name: "boundary".to_string(),
                concept: "Implicit boundary around dots".to_string(),
                embedding: vec![0.75; 32],
                importance: 0.7,
                relations: vec![("assumptions".to_string(), 0.85)],
            },
            ProblemElement {
                name: "assumptions".to_string(),
                concept: "Assumed constraints".to_string(),
                embedding: vec![0.8; 32],
                importance: 0.8,
                relations: vec![("boundary".to_string(), 0.85)],
            },
        ],
        constraints: vec![
            Constraint {
                description: "Exactly 4 lines".to_string(),
                hard: true,
                check: |_| true,
            },
            Constraint {
                description: "Lines must be continuous".to_string(),
                hard: true,
                check: |_| true,
            },
        ],
        failed_approaches: vec![
            "Staying within dots boundary".to_string(),
            "Using only horizontal/vertical lines".to_string(),
        ],
        domains: vec!["puzzles".to_string(), "lateral thinking".to_string()],
        embedding: vec![0.75; 32],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_creation() {
        let solver = DreamProblemSolver::new();
        assert!(solver.dream_log().is_empty());
        assert!(solver.insights().is_empty());
    }

    #[test]
    fn test_benzene_problem() {
        let problem = benzene_problem();
        assert_eq!(problem.id, "benzene_1865");
        assert_eq!(problem.elements.len(), 5);
    }

    #[test]
    fn test_nine_dots_problem() {
        let problem = nine_dots_problem();
        assert_eq!(problem.id, "nine_dots");
        assert_eq!(problem.elements.len(), 4);
    }

    #[test]
    fn test_solve_benzene() {
        let mut solver = DreamProblemSolver::new();
        let problem = benzene_problem();
        let result = solver.solve(&problem, 2);
        assert_eq!(result.total_sleep_cycles, 2);
        assert!(!result.dreams.is_empty());
    }

    #[test]
    fn test_solve_with_3d_dreams() {
        let mut solver = DreamProblemSolver::new();
        let problem = benzene_problem();
        let result = solver.solve_with_3d_dreams(&problem, 2);
        assert_eq!(result.total_sleep_cycles, 2);
        assert!(!result.dreams.is_empty());
        assert_eq!(result.dream_worlds.len(), 2);
    }

    #[test]
    fn test_dream_explorer() {
        let mut solver = DreamProblemSolver::new();
        let problem = benzene_problem();
        let _result = solver.solve_with_3d_dreams(&problem, 1);

        // Explore the first dream
        let explorer = solver.explore_dream_world(0);
        assert!(explorer.is_ok());

        if let Ok(explorer) = explorer {
            let concepts = explorer.list_concepts();
            assert!(!concepts.is_empty());
        }
    }
}
