//! Core Types and Main Orchestrator with 3D Dream Explorer
//!
//! This module contains all core data structures, the main DreamProblemSolver,
//! and the NEW DreamExplorer API for navigating dreams in 3D space.

use super::dream_generator::{DreamGenerator, DreamWorld3D, Position3D};
use super::insight_extractor::InsightExtractor;
use super::solution_synthesizer::SolutionSynthesizer;
use std::collections::HashSet;

// ============================================================================
// CORE DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone)]
pub struct Problem {
    /// Unique identifier
    pub id: String,
    /// Problem description
    pub description: String,
    /// Key elements/concepts in the problem
    pub elements: Vec<ProblemElement>,
    /// Known constraints
    pub constraints: Vec<Constraint>,
    /// Previous failed approaches
    pub failed_approaches: Vec<String>,
    /// Domain(s) the problem belongs to
    pub domains: Vec<String>,
    /// Embedding vector for semantic matching
    pub embedding: Vec<f64>,
}

/// An element of a problem
#[derive(Debug, Clone)]
pub struct ProblemElement {
    pub name: String,
    pub concept: String,
    pub embedding: Vec<f64>,
    pub importance: f64,
    pub relations: Vec<(String, f64)>, // (related_element, strength)
}

/// A constraint on solutions
#[derive(Debug, Clone)]
pub struct Constraint {
    pub description: String,
    pub hard: bool, // Hard constraints must be satisfied; soft can be relaxed
    pub check: fn(&Solution) -> bool,
}

/// A creative insight from dreams
#[derive(Debug, Clone)]
pub struct Insight {
    /// Unique identifier
    pub id: String,
    /// The novel association discovered
    pub association: Association,
    /// Source dream that generated this
    pub source_dream_id: String,
    /// Bizarreness score (higher = more creative leap)
    pub bizarreness: f64,
    /// How relevant to original problem
    pub relevance: f64,
    /// Confidence in the insight
    pub confidence: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// A novel association between concepts
#[derive(Debug, Clone)]
pub struct Association {
    /// First concept
    pub from: String,
    /// Second concept
    pub to: String,
    /// Nature of the connection
    pub connection_type: ConnectionType,
    /// Embedding of the bridging concept
    pub bridge: Vec<f64>,
    /// Strength of association
    pub strength: f64,
}

/// Types of creative connections
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionType {
    /// A is like B (structural similarity)
    Analogy,
    /// A transforms into B
    Transformation,
    /// A and B combine to form C
    Synthesis,
    /// A is opposite of B (useful for inversion)
    Inversion,
    /// A causes B (novel causal link)
    CausalDiscovery,
    /// A and B share hidden property C
    CommonGround,
    /// NEW: A and B are spatially proximate in dream space
    SpatialProximity,
}

/// A potential solution
#[derive(Debug, Clone)]
pub struct Solution {
    /// Solution description
    pub description: String,
    /// Key insights used
    pub insights_used: Vec<String>,
    /// Novel elements introduced
    pub novel_elements: Vec<String>,
    /// Confidence score
    pub confidence: f64,
    /// Novelty score
    pub novelty: f64,
    /// Feasibility score
    pub feasibility: f64,
}

/// Dream content during REM
#[derive(Debug, Clone)]
pub struct Dream {
    pub id: String,
    /// Elements appearing in dream
    pub elements: Vec<DreamElement>,
    /// Narrative fragments
    pub narrative_fragments: Vec<String>,
    /// Emotional valence (-1 to 1)
    pub valence: f64,
    /// How bizarre/surreal (0 to 1)
    pub bizarreness: f64,
    /// Vividness (0 to 1)
    pub vividness: f64,
    /// Which problem elements appeared
    pub problem_elements_present: HashSet<String>,
    /// Novel combinations formed
    pub novel_combinations: Vec<(String, String)>,
}

/// Element in a dream
#[derive(Debug, Clone)]
pub struct DreamElement {
    pub original_concept: String,
    pub transformed_form: String,
    pub transformation_type: TransformationType,
    pub activation: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransformationType {
    Literal,           // Appears as-is
    Symbolic,          // Represented symbolically
    Condensed,         // Multiple concepts merged
    Displaced,         // Appears as something else
    Visualized,        // Abstract made concrete
}

// ============================================================================
// MAIN DREAM PROBLEM SOLVER
// ============================================================================

/// Complete dream-based problem solver with 3D capabilities
pub struct DreamProblemSolver {
    dream_generator: DreamGenerator,
    insight_extractor: InsightExtractor,
    solution_synthesizer: SolutionSynthesizer,
    all_dreams: Vec<Dream>,
    all_insights: Vec<Insight>,
    // NEW: Store 3D dream worlds
    dream_worlds: Vec<DreamWorld3D>,
}

impl Default for DreamProblemSolver {
    fn default() -> Self {
        Self {
            dream_generator: DreamGenerator::new(),
            insight_extractor: InsightExtractor::new(),
            solution_synthesizer: SolutionSynthesizer::new(),
            all_dreams: Vec::new(),
            all_insights: Vec::new(),
            dream_worlds: Vec::new(),
        }
    }
}

impl DreamProblemSolver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Solve a problem using dream incubation (standard version)
    pub fn solve(&mut self, problem: &Problem, sleep_cycles: usize) -> SolverResult {
        println!("=== Dream Problem Solver ===\n");
        println!("Problem: {}\n", problem.description);
        println!("Elements: {:?}\n", problem.elements.iter().map(|e| &e.name).collect::<Vec<_>>());

        // Phase 1: Problem Immersion
        println!("Phase 1: Immersing in problem...");
        self.dream_generator.incubate_problem(problem);

        // Phase 2: Sleep Cycles
        println!("Phase 2: Entering sleep cycles...\n");

        for cycle in 0..sleep_cycles {
            println!("  Sleep cycle {}/{}:", cycle + 1, sleep_cycles);

            // REM phase - generate dream
            let dream = self.dream_generator.generate_dream(100);

            println!("    Dream generated:");
            println!("      Bizarreness: {:.2}", dream.bizarreness);
            println!("      Elements present: {:?}", dream.problem_elements_present);
            println!("      Novel combinations: {:?}", dream.novel_combinations);

            // Extract insights
            let insights = self.insight_extractor.extract(&dream, problem);
            println!("      Insights extracted: {}", insights.len());

            for insight in &insights {
                println!("        - {:?}: {} ↔ {} (confidence: {:.2})",
                    insight.association.connection_type,
                    insight.association.from,
                    insight.association.to,
                    insight.confidence
                );
            }

            self.all_dreams.push(dream);
            self.all_insights.extend(insights);

            println!();
        }

        // Phase 3: Synthesis
        println!("Phase 3: Synthesizing solution...\n");

        let solution = self.solution_synthesizer.synthesize(problem, &self.all_insights);

        if let Some(ref sol) = solution {
            println!("Solution found:");
            println!("{}", sol.description);
            println!("Novelty: {:.2}", sol.novelty);
            println!("Confidence: {:.2}", sol.confidence);
            println!("Feasibility: {:.2}", sol.feasibility);
        } else {
            println!("No solution could be synthesized. More sleep cycles may help.");
        }

        SolverResult {
            problem: problem.clone(),
            dreams: self.all_dreams.clone(),
            insights: self.all_insights.clone(),
            solution,
            total_sleep_cycles: sleep_cycles,
        }
    }

    /// NEW: Solve a problem AND create 3D walkable dream worlds
    pub fn solve_with_3d_dreams(&mut self, problem: &Problem, sleep_cycles: usize) -> SolverResult3D {
        println!("=== Dream Problem Solver (3D Mode) ===\n");
        println!("Problem: {}\n", problem.description);
        println!("Elements: {:?}\n", problem.elements.iter().map(|e| &e.name).collect::<Vec<_>>());

        // Phase 1: Problem Immersion
        println!("Phase 1: Immersing in problem...");
        self.dream_generator.incubate_problem(problem);

        // Phase 2: Sleep Cycles with 3D Dreams
        println!("Phase 2: Entering sleep cycles (generating 3D dream worlds)...\n");

        for cycle in 0..sleep_cycles {
            println!("  Sleep cycle {}/{}:", cycle + 1, sleep_cycles);

            // REM phase - generate dream AND its 3D representation
            let (dream, dream_world) = self.dream_generator.generate_dream_with_3d(100);

            println!("    Dream generated:");
            println!("      Bizarreness: {:.2}", dream.bizarreness);
            println!("      Elements present: {:?}", dream.problem_elements_present);
            println!("      Novel combinations: {:?}", dream.novel_combinations);
            println!("      3D world: {} concepts, {} paths",
                dream_world.concept_locations.len(),
                dream_world.association_paths.len()
            );

            // Extract insights using spatial information
            let insights = self.insight_extractor.extract_with_spatial(&dream, Some(&dream_world), problem);
            println!("      Insights extracted: {} (including spatial)", insights.len());

            for insight in &insights {
                println!("        - {:?}: {} ↔ {} (confidence: {:.2})",
                    insight.association.connection_type,
                    insight.association.from,
                    insight.association.to,
                    insight.confidence
                );
            }

            self.all_dreams.push(dream);
            self.all_insights.extend(insights);
            self.dream_worlds.push(dream_world);

            println!();
        }

        // Phase 3: Synthesis
        println!("Phase 3: Synthesizing solution...\n");

        let solution = self.solution_synthesizer.synthesize(problem, &self.all_insights);

        if let Some(ref sol) = solution {
            println!("Solution found:");
            println!("{}", sol.description);
            println!("Novelty: {:.2}", sol.novelty);
            println!("Confidence: {:.2}", sol.confidence);
            println!("Feasibility: {:.2}", sol.feasibility);
            println!("\n💡 TIP: Use explore_dream_world() to navigate the 3D dream spaces!");
        } else {
            println!("No solution could be synthesized. More sleep cycles may help.");
        }

        SolverResult3D {
            problem: problem.clone(),
            dreams: self.all_dreams.clone(),
            dream_worlds: self.dream_worlds.clone(),
            insights: self.all_insights.clone(),
            solution,
            total_sleep_cycles: sleep_cycles,
        }
    }

    /// NEW: Navigate through a specific dream in 3D
    pub fn explore_dream_world(&self, dream_index: usize) -> Result<DreamExplorer, String> {
        let dream_world = self.dream_worlds.get(dream_index)
            .ok_or_else(|| format!("Dream index {} out of bounds (have {} dreams)", dream_index, self.dream_worlds.len()))?;

        Ok(DreamExplorer::new(dream_world.clone()))
    }

    /// Get dream log for analysis
    pub fn dream_log(&self) -> &[Dream] {
        &self.all_dreams
    }

    /// Get all insights
    pub fn insights(&self) -> &[Insight] {
        &self.all_insights
    }

    /// NEW: Get all 3D dream worlds
    pub fn dream_worlds(&self) -> &[DreamWorld3D] {
        &self.dream_worlds
    }
}

// ============================================================================
// DREAM EXPLORER - Navigate 3D Dreams
// ============================================================================

/// NEW: Interactive dream exploration in 3D space
pub struct DreamExplorer {
    world: DreamWorld3D,
    current_position: Position3D,
    // TODO: Add mindscape when available
    // mindscape: MindscapeExplorer,
}

impl DreamExplorer {
    pub fn new(world: DreamWorld3D) -> Self {
        // Start at origin
        Self {
            world,
            current_position: Position3D { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    /// Walk to a concept location in the dream
    pub fn walk_to(&mut self, concept: &str) -> Result<NavigationPath, String> {
        let target = self.world.concept_locations.get(concept)
            .ok_or_else(|| format!("Concept '{}' not found in dream", concept))?;

        // TODO: Use mindscape.navigate_to_position() when available
        // let path = self.mindscape.navigate_to_position(target.position)?;

        // For now, create simple direct path
        let path = NavigationPath {
            start: self.current_position.clone(),
            end: target.position.clone(),
            waypoints: vec![self.current_position.clone(), target.position.clone()],
            distance: self.current_position.distance_to(&target.position),
        };

        self.current_position = target.position.clone();

        Ok(path)
    }

    /// Look around at nearby concepts within radius
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

    /// Follow an association pathway between concepts
    pub fn follow_association(&mut self, from: &str, to: &str) -> Result<AssociationJourney, String> {
        let path = self.world.association_paths.iter()
            .find(|p| p.from == from && p.to == to)
            .ok_or_else(|| format!("No association path from '{}' to '{}'", from, to))?;

        // Walk along the path
        if let Some(start_pos) = path.waypoints.first() {
            self.current_position = start_pos.clone();
        }
        if let Some(end_pos) = path.waypoints.last() {
            self.current_position = end_pos.clone();
        }

        Ok(AssociationJourney {
            from: from.to_string(),
            to: to.to_string(),
            path: path.waypoints.clone(),
            strength: path.strength,
            connection_type: path.connection_type,
        })
    }

    /// Get current position in dream space
    pub fn current_position(&self) -> &Position3D {
        &self.current_position
    }

    /// List all concepts in the dream
    pub fn list_concepts(&self) -> Vec<&str> {
        self.world.concept_locations.keys()
            .map(|s| s.as_str())
            .collect()
    }

    /// List all association paths
    pub fn list_associations(&self) -> Vec<(&str, &str, f64)> {
        self.world.association_paths.iter()
            .map(|p| (p.from.as_str(), p.to.as_str(), p.strength))
            .collect()
    }
}

/// Path taken while navigating dream space
#[derive(Debug, Clone)]
pub struct NavigationPath {
    pub start: Position3D,
    pub end: Position3D,
    pub waypoints: Vec<Position3D>,
    pub distance: f64,
}

/// Journey along an association path
#[derive(Debug, Clone)]
pub struct AssociationJourney {
    pub from: String,
    pub to: String,
    pub path: Vec<Position3D>,
    pub strength: f64,
    pub connection_type: super::dream_generator::ConnectionType,
}

// ============================================================================
// SOLVER RESULTS
// ============================================================================

/// Result of solving attempt (standard)
pub struct SolverResult {
    pub problem: Problem,
    pub dreams: Vec<Dream>,
    pub insights: Vec<Insight>,
    pub solution: Option<Solution>,
    pub total_sleep_cycles: usize,
}

impl SolverResult {
    pub fn success(&self) -> bool {
        self.solution.is_some()
    }

    pub fn novelty_score(&self) -> f64 {
        self.solution.as_ref().map(|s| s.novelty).unwrap_or(0.0)
    }

    pub fn print_report(&self) {
        println!("\n=== DREAM SOLVING REPORT ===\n");
        println!("Problem: {}", self.problem.description);
        println!("Sleep cycles: {}", self.total_sleep_cycles);
        println!("Dreams generated: {}", self.dreams.len());
        println!("Insights extracted: {}", self.insights.len());
        println!("Solution found: {}", self.success());

        if let Some(ref sol) = self.solution {
            println!("\nSolution Summary:");
            println!("  Novel elements: {:?}", sol.novel_elements);
            println!("  Confidence: {:.1}%", sol.confidence * 100.0);
            println!("  Novelty: {:.1}%", sol.novelty * 100.0);
            println!("  Feasibility: {:.1}%", sol.feasibility * 100.0);
        }

        println!("\nDream Statistics:");
        let avg_bizarreness: f64 = self.dreams.iter()
            .map(|d| d.bizarreness)
            .sum::<f64>() / self.dreams.len() as f64;
        println!("  Average bizarreness: {:.2}", avg_bizarreness);

        let total_novel_combos: usize = self.dreams.iter()
            .map(|d| d.novel_combinations.len())
            .sum();
        println!("  Total novel combinations: {}", total_novel_combos);
    }
}

/// NEW: Result with 3D dream worlds
pub struct SolverResult3D {
    pub problem: Problem,
    pub dreams: Vec<Dream>,
    pub dream_worlds: Vec<DreamWorld3D>,
    pub insights: Vec<Insight>,
    pub solution: Option<Solution>,
    pub total_sleep_cycles: usize,
}

impl SolverResult3D {
    pub fn success(&self) -> bool {
        self.solution.is_some()
    }

    pub fn novelty_score(&self) -> f64 {
        self.solution.as_ref().map(|s| s.novelty).unwrap_or(0.0)
    }

    pub fn print_report(&self) {
        println!("\n=== DREAM SOLVING REPORT (3D) ===\n");
        println!("Problem: {}", self.problem.description);
        println!("Sleep cycles: {}", self.total_sleep_cycles);
        println!("Dreams generated: {}", self.dreams.len());
        println!("3D worlds created: {}", self.dream_worlds.len());
        println!("Insights extracted: {}", self.insights.len());
        println!("Solution found: {}", self.success());

        if let Some(ref sol) = self.solution {
            println!("\nSolution Summary:");
            println!("  Novel elements: {:?}", sol.novel_elements);
            println!("  Confidence: {:.1}%", sol.confidence * 100.0);
            println!("  Novelty: {:.1}%", sol.novelty * 100.0);
            println!("  Feasibility: {:.1}%", sol.feasibility * 100.0);
        }

        println!("\nDream Statistics:");
        let avg_bizarreness: f64 = self.dreams.iter()
            .map(|d| d.bizarreness)
            .sum::<f64>() / self.dreams.len() as f64;
        println!("  Average bizarreness: {:.2}", avg_bizarreness);

        let total_novel_combos: usize = self.dreams.iter()
            .map(|d| d.novel_combinations.len())
            .sum();
        println!("  Total novel combinations: {}", total_novel_combos);

        println!("\n3D World Statistics:");
        let avg_concepts = self.dream_worlds.iter()
            .map(|w| w.concept_locations.len())
            .sum::<usize>() as f64 / self.dream_worlds.len() as f64;
        let avg_paths = self.dream_worlds.iter()
            .map(|w| w.association_paths.len())
            .sum::<usize>() as f64 / self.dream_worlds.len() as f64;
        println!("  Average concepts per world: {:.1}", avg_concepts);
        println!("  Average association paths: {:.1}", avg_paths);
    }
}
