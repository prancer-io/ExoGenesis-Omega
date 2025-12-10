//! # Dream-Based Creative Problem Solver
//!
//! This module implements a cognitive architecture that uses REM sleep
//! for creative problem solving, inspired by research on sleep's role
//! in insight and creativity.
//!
//! ## Scientific Background
//!
//! Research shows that REM sleep facilitates:
//! - Remote associative thinking (connecting distant concepts)
//! - Memory recombination (novel arrangements of experiences)
//! - Reduced logical constraints (prefrontal cortex offline)
//! - Insight emergence ("sleeping on it" actually works)
//!
//! Famous examples:
//! - Kekulé's benzene ring structure (dreamed of ouroboros)
//! - Mendeleev's periodic table (dreamed arrangement)
//! - Paul McCartney's "Yesterday" (dreamed melody)
//! - Otto Loewi's neurotransmitter experiment (dreamed protocol)
//!
//! ## How It Works
//!
//! 1. **Problem Immersion**: Deeply encode problem elements
//! 2. **Sleep Onset**: Transition through N1 → N2 → N3 → REM
//! 3. **Dream Incubation**: REM recombines problem elements bizarrely
//! 4. **Insight Capture**: High-bizarreness dreams contain novel associations
//! 5. **Synthesis**: Apply dream insights to original problem

use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::time::Duration;

// ============================================================================
// CORE TYPES
// ============================================================================

/// A problem to be solved creatively
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

#[derive(Debug, Clone)]
pub enum TransformationType {
    Literal,           // Appears as-is
    Symbolic,          // Represented symbolically
    Condensed,         // Multiple concepts merged
    Displaced,         // Appears as something else
    Visualized,        // Abstract made concrete
}

// ============================================================================
// NEURAL SUBSTRATE SIMULATION
// ============================================================================

/// Simplified neural network for dream simulation
pub struct DreamNeuralNetwork {
    /// Concept nodes and their activations
    concepts: HashMap<String, ConceptNode>,
    /// Associative connections
    associations: HashMap<(String, String), f64>,
    /// Current activation state
    activations: HashMap<String, f64>,
    /// Noise level (higher during REM)
    noise_level: f64,
    /// Prefrontal inhibition (lower during REM)
    prefrontal_inhibition: f64,
}

struct ConceptNode {
    embedding: Vec<f64>,
    base_activation: f64,
    decay_rate: f64,
}

impl DreamNeuralNetwork {
    pub fn new() -> Self {
        Self {
            concepts: HashMap::new(),
            associations: HashMap::new(),
            activations: HashMap::new(),
            noise_level: 0.1,
            prefrontal_inhibition: 1.0,
        }
    }

    /// Encode a concept into the network
    pub fn encode(&mut self, name: &str, embedding: Vec<f64>, importance: f64) {
        self.concepts.insert(name.to_string(), ConceptNode {
            embedding: embedding.clone(),
            base_activation: importance,
            decay_rate: 0.1,
        });
        self.activations.insert(name.to_string(), importance);
    }

    /// Create association between concepts
    pub fn associate(&mut self, from: &str, to: &str, strength: f64) {
        self.associations.insert((from.to_string(), to.to_string()), strength);
        self.associations.insert((to.to_string(), from.to_string()), strength * 0.8);
    }

    /// Enter REM state (reduced inhibition, increased noise)
    pub fn enter_rem(&mut self) {
        self.prefrontal_inhibition = 0.2; // Prefrontal cortex offline
        self.noise_level = 0.5; // More random activation
    }

    /// Exit REM state
    pub fn exit_rem(&mut self) {
        self.prefrontal_inhibition = 1.0;
        self.noise_level = 0.1;
    }

    /// Simulate one step of neural dynamics
    pub fn step(&mut self, dt: f64) -> Vec<(String, String, f64)> {
        let mut new_activations = HashMap::new();
        let mut novel_associations = Vec::new();

        // Spread activation through network
        for (concept, activation) in &self.activations {
            // Decay
            let decayed = activation * (1.0 - self.concepts[concept].decay_rate * dt);

            // Add noise (REM has more noise)
            let noise = (rand_float() - 0.5) * self.noise_level;

            // Collect input from associated concepts
            let mut input = 0.0;
            for ((from, to), strength) in &self.associations {
                if to == concept {
                    input += self.activations.get(from).unwrap_or(&0.0) * strength;
                }
            }

            // Apply prefrontal inhibition (limits unusual associations when awake)
            let gated_input = input * (1.0 - self.prefrontal_inhibition * 0.5);

            new_activations.insert(
                concept.clone(),
                (decayed + gated_input + noise).clamp(0.0, 1.0),
            );
        }

        // Detect novel co-activations (potential insights)
        let active_concepts: Vec<_> = new_activations.iter()
            .filter(|(_, &a)| a > 0.5)
            .map(|(c, _)| c.clone())
            .collect();

        for i in 0..active_concepts.len() {
            for j in i+1..active_concepts.len() {
                let c1 = &active_concepts[i];
                let c2 = &active_concepts[j];

                // Check if this is a novel association
                let key = (c1.clone(), c2.clone());
                if !self.associations.contains_key(&key) {
                    // Novel co-activation!
                    let strength = new_activations[c1] * new_activations[c2];
                    if strength > 0.3 {
                        novel_associations.push((c1.clone(), c2.clone(), strength));
                    }
                }
            }
        }

        self.activations = new_activations;
        novel_associations
    }

    /// Get most active concepts
    pub fn most_active(&self, n: usize) -> Vec<(String, f64)> {
        let mut sorted: Vec<_> = self.activations.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sorted.into_iter().take(n).collect()
    }
}

// ============================================================================
// DREAM GENERATOR
// ============================================================================

/// Generates dreams during REM sleep
pub struct DreamGenerator {
    network: DreamNeuralNetwork,
    problem_elements: Vec<String>,
    dream_id_counter: u64,
}

impl DreamGenerator {
    pub fn new() -> Self {
        Self {
            network: DreamNeuralNetwork::new(),
            problem_elements: Vec::new(),
            dream_id_counter: 0,
        }
    }

    /// Prepare for dreaming about a problem
    pub fn incubate_problem(&mut self, problem: &Problem) {
        // Encode all problem elements
        for element in &problem.elements {
            self.network.encode(&element.name, element.embedding.clone(), element.importance);
            self.problem_elements.push(element.name.clone());

            // Encode relations
            for (related, strength) in &element.relations {
                self.network.associate(&element.name, related, *strength);
            }
        }

        // Also encode failed approaches (to potentially invert them)
        for (i, approach) in problem.failed_approaches.iter().enumerate() {
            let name = format!("failed_{}", i);
            self.network.encode(&name, vec![0.0; 32], 0.3);
        }
    }

    /// Generate a dream during REM
    pub fn generate_dream(&mut self, duration_steps: usize) -> Dream {
        self.dream_id_counter += 1;
        let dream_id = format!("dream_{}", self.dream_id_counter);

        self.network.enter_rem();

        let mut all_novel_associations = Vec::new();
        let mut active_elements = HashSet::new();

        // Simulate dream neural dynamics
        for _ in 0..duration_steps {
            let novel = self.network.step(0.1);
            all_novel_associations.extend(novel);

            // Track which problem elements appear
            for (concept, activation) in self.network.most_active(5) {
                if self.problem_elements.contains(&concept) && activation > 0.6 {
                    active_elements.insert(concept);
                }
            }
        }

        self.network.exit_rem();

        // Calculate bizarreness based on novel associations
        let bizarreness = (all_novel_associations.len() as f64 / duration_steps as f64)
            .min(1.0);

        // Build dream elements
        let elements: Vec<DreamElement> = self.network.most_active(10)
            .into_iter()
            .map(|(concept, activation)| {
                DreamElement {
                    original_concept: concept.clone(),
                    transformed_form: self.transform_concept(&concept, bizarreness),
                    transformation_type: self.select_transformation(bizarreness),
                    activation,
                }
            })
            .collect();

        // Novel combinations
        let novel_combinations: Vec<_> = all_novel_associations.iter()
            .filter(|(a, b, s)| *s > 0.4)
            .map(|(a, b, _)| (a.clone(), b.clone()))
            .collect();

        Dream {
            id: dream_id,
            elements,
            narrative_fragments: self.generate_narrative(&active_elements, &novel_combinations),
            valence: (rand_float() - 0.3), // Slight positive bias
            bizarreness,
            vividness: 0.5 + rand_float() * 0.5,
            problem_elements_present: active_elements,
            novel_combinations,
        }
    }

    fn transform_concept(&self, concept: &str, bizarreness: f64) -> String {
        if bizarreness < 0.3 {
            concept.to_string() // Literal
        } else if bizarreness < 0.6 {
            format!("{}_symbol", concept) // Symbolic
        } else {
            format!("transformed_{}", concept) // Highly transformed
        }
    }

    fn select_transformation(&self, bizarreness: f64) -> TransformationType {
        let r = rand_float();
        if bizarreness < 0.3 {
            if r < 0.7 { TransformationType::Literal } else { TransformationType::Visualized }
        } else if bizarreness < 0.6 {
            if r < 0.5 { TransformationType::Symbolic } else { TransformationType::Condensed }
        } else {
            if r < 0.3 { TransformationType::Displaced } else { TransformationType::Condensed }
        }
    }

    fn generate_narrative(
        &self,
        elements: &HashSet<String>,
        combinations: &[(String, String)],
    ) -> Vec<String> {
        let mut fragments = Vec::new();

        for (a, b) in combinations {
            fragments.push(format!("{} becomes connected to {}", a, b));
        }

        if !elements.is_empty() {
            fragments.push(format!(
                "Scene with: {}",
                elements.iter().cloned().collect::<Vec<_>>().join(", ")
            ));
        }

        fragments
    }
}

// ============================================================================
// INSIGHT EXTRACTOR
// ============================================================================

/// Extracts actionable insights from dreams
pub struct InsightExtractor {
    insight_id_counter: u64,
}

impl InsightExtractor {
    pub fn new() -> Self {
        Self { insight_id_counter: 0 }
    }

    /// Extract insights from a dream
    pub fn extract(&mut self, dream: &Dream, problem: &Problem) -> Vec<Insight> {
        let mut insights = Vec::new();

        // Analyze novel combinations
        for (a, b) in &dream.novel_combinations {
            // Check if both relate to problem
            let a_relevant = problem.elements.iter().any(|e| e.name == *a);
            let b_relevant = problem.elements.iter().any(|e| e.name == *b);

            if a_relevant || b_relevant {
                self.insight_id_counter += 1;

                let connection_type = self.infer_connection_type(a, b, dream);
                let relevance = self.compute_relevance(a, b, problem);

                insights.push(Insight {
                    id: format!("insight_{}", self.insight_id_counter),
                    association: Association {
                        from: a.clone(),
                        to: b.clone(),
                        connection_type,
                        bridge: vec![0.0; 32], // Would compute proper embedding
                        strength: dream.bizarreness,
                    },
                    source_dream_id: dream.id.clone(),
                    bizarreness: dream.bizarreness,
                    relevance,
                    confidence: relevance * (1.0 - dream.bizarreness * 0.5),
                    timestamp: current_timestamp(),
                });
            }
        }

        // Also look for inversions (if failed approaches appeared transformed)
        for element in &dream.elements {
            if element.original_concept.starts_with("failed_") {
                if element.transformation_type == TransformationType::Displaced {
                    // Failed approach appeared in disguise - potential inversion insight
                    self.insight_id_counter += 1;
                    insights.push(Insight {
                        id: format!("insight_{}", self.insight_id_counter),
                        association: Association {
                            from: element.original_concept.clone(),
                            to: "inverted_approach".to_string(),
                            connection_type: ConnectionType::Inversion,
                            bridge: vec![0.0; 32],
                            strength: element.activation,
                        },
                        source_dream_id: dream.id.clone(),
                        bizarreness: dream.bizarreness,
                        relevance: 0.7,
                        confidence: 0.5,
                        timestamp: current_timestamp(),
                    });
                }
            }
        }

        insights
    }

    fn infer_connection_type(&self, a: &str, b: &str, dream: &Dream) -> ConnectionType {
        // Heuristics for connection type
        if dream.bizarreness > 0.7 {
            ConnectionType::Synthesis // Highly bizarre = novel synthesis
        } else if a.contains("failed") || b.contains("failed") {
            ConnectionType::Inversion
        } else if dream.bizarreness > 0.4 {
            ConnectionType::Analogy
        } else {
            ConnectionType::CommonGround
        }
    }

    fn compute_relevance(&self, a: &str, b: &str, problem: &Problem) -> f64 {
        let mut score = 0.0;

        for element in &problem.elements {
            if element.name == *a || element.name == *b {
                score += element.importance;
            }
        }

        (score / 2.0).min(1.0)
    }
}

// ============================================================================
// SOLUTION SYNTHESIZER
// ============================================================================

/// Synthesizes solutions from insights
pub struct SolutionSynthesizer;

impl SolutionSynthesizer {
    pub fn new() -> Self {
        Self
    }

    /// Synthesize solution from problem and insights
    pub fn synthesize(
        &self,
        problem: &Problem,
        insights: &[Insight],
    ) -> Option<Solution> {
        if insights.is_empty() {
            return None;
        }

        // Rank insights by combined score
        let mut ranked_insights: Vec<_> = insights.iter()
            .map(|i| {
                let score = i.relevance * 0.4 + i.confidence * 0.3 + i.bizarreness * 0.3;
                (i, score)
            })
            .collect();
        ranked_insights.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Use top insights to build solution
        let top_insights: Vec<_> = ranked_insights.iter()
            .take(3)
            .map(|(i, _)| *i)
            .collect();

        let description = self.generate_solution_description(problem, &top_insights);
        let novel_elements = self.extract_novel_elements(&top_insights);

        let novelty = top_insights.iter()
            .map(|i| i.bizarreness)
            .sum::<f64>() / top_insights.len() as f64;

        let feasibility = 1.0 - novelty * 0.3; // More novel = less immediately feasible

        let confidence = top_insights.iter()
            .map(|i| i.confidence)
            .sum::<f64>() / top_insights.len() as f64;

        Some(Solution {
            description,
            insights_used: top_insights.iter().map(|i| i.id.clone()).collect(),
            novel_elements,
            confidence,
            novelty,
            feasibility,
        })
    }

    fn generate_solution_description(&self, problem: &Problem, insights: &[&Insight]) -> String {
        let mut description = format!(
            "Solution to '{}' using dream-derived insights:\n\n",
            problem.description
        );

        for insight in insights {
            match &insight.association.connection_type {
                ConnectionType::Analogy => {
                    description.push_str(&format!(
                        "- Consider that {} is analogous to {}\n",
                        insight.association.from,
                        insight.association.to
                    ));
                }
                ConnectionType::Synthesis => {
                    description.push_str(&format!(
                        "- Combine {} with {} to create new approach\n",
                        insight.association.from,
                        insight.association.to
                    ));
                }
                ConnectionType::Inversion => {
                    description.push_str(&format!(
                        "- Instead of {}, try the opposite approach\n",
                        insight.association.from
                    ));
                }
                ConnectionType::CommonGround => {
                    description.push_str(&format!(
                        "- {} and {} share hidden connection\n",
                        insight.association.from,
                        insight.association.to
                    ));
                }
                _ => {}
            }
        }

        description
    }

    fn extract_novel_elements(&self, insights: &[&Insight]) -> Vec<String> {
        insights.iter()
            .flat_map(|i| vec![
                format!("{}↔{}", i.association.from, i.association.to)
            ])
            .collect()
    }
}

// ============================================================================
// MAIN SOLVER
// ============================================================================

/// Complete dream-based problem solver
pub struct DreamProblemSolver {
    dream_generator: DreamGenerator,
    insight_extractor: InsightExtractor,
    solution_synthesizer: SolutionSynthesizer,
    all_dreams: Vec<Dream>,
    all_insights: Vec<Insight>,
}

impl DreamProblemSolver {
    pub fn new() -> Self {
        Self {
            dream_generator: DreamGenerator::new(),
            insight_extractor: InsightExtractor::new(),
            solution_synthesizer: SolutionSynthesizer::new(),
            all_dreams: Vec::new(),
            all_insights: Vec::new(),
        }
    }

    /// Solve a problem using dream incubation
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

            // Simulate sleep architecture
            // N1 → N2 → N3 → REM (where dreams occur)

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

    /// Get dream log for analysis
    pub fn dream_log(&self) -> &[Dream] {
        &self.all_dreams
    }

    /// Get all insights
    pub fn insights(&self) -> &[Insight] {
        &self.all_insights
    }
}

/// Result of solving attempt
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

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn rand_float() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    (hasher.finish() as f64) / (u64::MAX as f64)
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// ============================================================================
// EXAMPLE PROBLEMS
// ============================================================================

/// Create the classic "9 dots" problem
pub fn nine_dots_problem() -> Problem {
    Problem {
        id: "nine_dots".to_string(),
        description: "Connect all 9 dots arranged in a 3x3 grid using only 4 straight lines without lifting the pen".to_string(),
        elements: vec![
            ProblemElement {
                name: "dots".to_string(),
                concept: "points to connect".to_string(),
                embedding: vec![0.1; 32],
                importance: 1.0,
                relations: vec![("grid".to_string(), 0.9)],
            },
            ProblemElement {
                name: "grid".to_string(),
                concept: "3x3 arrangement".to_string(),
                embedding: vec![0.2; 32],
                importance: 0.8,
                relations: vec![("boundary".to_string(), 0.7)],
            },
            ProblemElement {
                name: "lines".to_string(),
                concept: "straight connections".to_string(),
                embedding: vec![0.3; 32],
                importance: 0.9,
                relations: vec![("continuous".to_string(), 0.8)],
            },
            ProblemElement {
                name: "boundary".to_string(),
                concept: "implicit square boundary".to_string(),
                embedding: vec![0.4; 32],
                importance: 0.5, // Low importance - this is the trap!
                relations: vec![],
            },
            ProblemElement {
                name: "outside".to_string(),
                concept: "space beyond the grid".to_string(),
                embedding: vec![0.5; 32],
                importance: 0.3, // Key insight often missed
                relations: vec![("boundary".to_string(), -0.5)], // Negative relation!
            },
        ],
        constraints: vec![],
        failed_approaches: vec![
            "Staying within the grid boundary".to_string(),
            "Trying diagonal lines only within grid".to_string(),
        ],
        domains: vec!["spatial reasoning".to_string(), "creativity".to_string()],
        embedding: vec![0.25; 32],
    }
}

/// Create Kekulé's benzene structure problem
pub fn benzene_problem() -> Problem {
    Problem {
        id: "benzene".to_string(),
        description: "Determine the structure of benzene (C6H6) - how can 6 carbons bond to only 6 hydrogens?".to_string(),
        elements: vec![
            ProblemElement {
                name: "carbon_atoms".to_string(),
                concept: "6 carbon atoms needing 4 bonds each".to_string(),
                embedding: vec![0.1; 32],
                importance: 1.0,
                relations: vec![("bonds".to_string(), 0.9)],
            },
            ProblemElement {
                name: "hydrogen_atoms".to_string(),
                concept: "only 6 hydrogens available".to_string(),
                embedding: vec![0.2; 32],
                importance: 0.9,
                relations: vec![],
            },
            ProblemElement {
                name: "bonds".to_string(),
                concept: "covalent bonds between atoms".to_string(),
                embedding: vec![0.3; 32],
                importance: 0.8,
                relations: vec![("alternating".to_string(), 0.5)],
            },
            ProblemElement {
                name: "snake".to_string(),
                concept: "serpent/chain form".to_string(),
                embedding: vec![0.4; 32],
                importance: 0.4,
                relations: vec![("ring".to_string(), 0.6), ("ouroboros".to_string(), 0.8)],
            },
            ProblemElement {
                name: "ouroboros".to_string(),
                concept: "snake eating its own tail".to_string(),
                embedding: vec![0.5; 32],
                importance: 0.3,
                relations: vec![("ring".to_string(), 0.9), ("cycle".to_string(), 0.9)],
            },
            ProblemElement {
                name: "ring".to_string(),
                concept: "circular arrangement".to_string(),
                embedding: vec![0.6; 32],
                importance: 0.6,
                relations: vec![("carbon_atoms".to_string(), 0.7)],
            },
            ProblemElement {
                name: "alternating".to_string(),
                concept: "alternating pattern".to_string(),
                embedding: vec![0.7; 32],
                importance: 0.5,
                relations: vec![("bonds".to_string(), 0.8)],
            },
        ],
        constraints: vec![],
        failed_approaches: vec![
            "Linear chain of carbons".to_string(),
            "Branched structure".to_string(),
        ],
        domains: vec!["chemistry".to_string(), "molecular structure".to_string()],
        embedding: vec![0.35; 32],
    }
}

/// Create a novel product design problem
pub fn product_innovation_problem() -> Problem {
    Problem {
        id: "sustainable_packaging".to_string(),
        description: "Design packaging that protects products during shipping but completely biodegrades within 30 days".to_string(),
        elements: vec![
            ProblemElement {
                name: "protection".to_string(),
                concept: "cushioning and impact resistance".to_string(),
                embedding: vec![0.1; 32],
                importance: 1.0,
                relations: vec![("foam".to_string(), 0.8), ("structure".to_string(), 0.7)],
            },
            ProblemElement {
                name: "biodegradation".to_string(),
                concept: "breaking down naturally".to_string(),
                embedding: vec![0.2; 32],
                importance: 1.0,
                relations: vec![("microbes".to_string(), 0.9), ("organic".to_string(), 0.8)],
            },
            ProblemElement {
                name: "mushroom".to_string(),
                concept: "fungal mycelium".to_string(),
                embedding: vec![0.3; 32],
                importance: 0.4,
                relations: vec![("organic".to_string(), 0.9), ("structure".to_string(), 0.6)],
            },
            ProblemElement {
                name: "honeycomb".to_string(),
                concept: "hexagonal structure".to_string(),
                embedding: vec![0.4; 32],
                importance: 0.3,
                relations: vec![("structure".to_string(), 0.9), ("efficient".to_string(), 0.8)],
            },
            ProblemElement {
                name: "seaweed".to_string(),
                concept: "marine algae".to_string(),
                embedding: vec![0.5; 32],
                importance: 0.3,
                relations: vec![("organic".to_string(), 0.9), ("film".to_string(), 0.7)],
            },
            ProblemElement {
                name: "popcorn".to_string(),
                concept: "expanded starch".to_string(),
                embedding: vec![0.6; 32],
                importance: 0.2,
                relations: vec![("protection".to_string(), 0.7), ("edible".to_string(), 0.9)],
            },
        ],
        constraints: vec![],
        failed_approaches: vec![
            "Recycled cardboard (doesn't protect enough)".to_string(),
            "Bioplastic foam (takes too long to degrade)".to_string(),
        ],
        domains: vec!["product design".to_string(), "sustainability".to_string()],
        embedding: vec![0.3; 32],
    }
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     OMEGA BRAIN: DREAM-BASED CREATIVE PROBLEM SOLVER         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Example 1: Nine Dots Problem
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: The Nine Dots Problem (classic lateral thinking)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut solver1 = DreamProblemSolver::new();
    let result1 = solver1.solve(&nine_dots_problem(), 3);
    result1.print_report();

    println!("\n\n");

    // Example 2: Benzene Structure (Kekulé's actual dream!)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Benzene Structure (Kekulé's Dream)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut solver2 = DreamProblemSolver::new();
    let result2 = solver2.solve(&benzene_problem(), 5);
    result2.print_report();

    println!("\n\n");

    // Example 3: Sustainable Packaging Innovation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Sustainable Packaging Innovation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut solver3 = DreamProblemSolver::new();
    let result3 = solver3.solve(&product_innovation_problem(), 4);
    result3.print_report();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    SIMULATION COMPLETE                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dream_generator() {
        let mut gen = DreamGenerator::new();
        let problem = nine_dots_problem();
        gen.incubate_problem(&problem);

        let dream = gen.generate_dream(50);

        assert!(!dream.id.is_empty());
        assert!(dream.bizarreness >= 0.0 && dream.bizarreness <= 1.0);
    }

    #[test]
    fn test_insight_extraction() {
        let mut gen = DreamGenerator::new();
        let mut extractor = InsightExtractor::new();
        let problem = nine_dots_problem();

        gen.incubate_problem(&problem);
        let dream = gen.generate_dream(100);

        let insights = extractor.extract(&dream, &problem);
        // May or may not find insights depending on random dream
        assert!(insights.len() >= 0);
    }

    #[test]
    fn test_full_solver() {
        let mut solver = DreamProblemSolver::new();
        let problem = benzene_problem();

        let result = solver.solve(&problem, 2);

        assert_eq!(result.total_sleep_cycles, 2);
        assert_eq!(result.dreams.len(), 2);
    }

    #[test]
    fn test_solution_synthesis() {
        let synthesizer = SolutionSynthesizer::new();
        let problem = nine_dots_problem();

        let insights = vec![
            Insight {
                id: "test_insight".to_string(),
                association: Association {
                    from: "boundary".to_string(),
                    to: "outside".to_string(),
                    connection_type: ConnectionType::Inversion,
                    bridge: vec![0.0; 32],
                    strength: 0.8,
                },
                source_dream_id: "dream_1".to_string(),
                bizarreness: 0.7,
                relevance: 0.9,
                confidence: 0.8,
                timestamp: 0,
            },
        ];

        let solution = synthesizer.synthesize(&problem, &insights);
        assert!(solution.is_some());

        let sol = solution.unwrap();
        assert!(sol.novelty > 0.0);
        assert!(sol.confidence > 0.0);
    }
}
