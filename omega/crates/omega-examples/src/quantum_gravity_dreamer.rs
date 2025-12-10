//! # Quantum Gravity Dream Solver
//!
//! Attempting to solve one of physics' greatest unsolved problems:
//! How to unify Quantum Mechanics with General Relativity
//!
//! ## The Problem
//!
//! For nearly 100 years, physicists have struggled to reconcile:
//! - Quantum Mechanics: The physics of the very small (probabilistic, discrete)
//! - General Relativity: The physics of gravity and spacetime (deterministic, continuous)
//!
//! Einstein spent his final 30 years on this. Hawking worked on it. Witten. Penrose.
//! String Theory, Loop Quantum Gravity, Causal Sets - all attempts, none complete.
//!
//! ## What if dreams could find the connection?
//!
//! Like Kekulé dreaming of the benzene ring, perhaps the answer requires
//! thinking that transcends our logical constraints.

use std::collections::{HashMap, HashSet};

// ============================================================================
// CORE TYPES (Simplified from main dream solver)
// ============================================================================

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: String,
    pub description: String,
    pub elements: Vec<ProblemElement>,
    pub failed_approaches: Vec<String>,
    pub domains: Vec<String>,
    pub embedding: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct ProblemElement {
    pub name: String,
    pub concept: String,
    pub embedding: Vec<f64>,
    pub importance: f64,
    pub relations: Vec<(String, f64)>,
}

#[derive(Debug, Clone)]
pub struct Insight {
    pub id: String,
    pub from: String,
    pub to: String,
    pub connection_type: ConnectionType,
    pub bizarreness: f64,
    pub relevance: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionType {
    Analogy,
    Transformation,
    Synthesis,
    Inversion,
    CausalDiscovery,
    CommonGround,
    Duality,        // New: A and B are dual descriptions
    Emergence,      // New: B emerges from A at some scale
    Holography,     // New: A encodes B in lower dimensions
}

#[derive(Debug, Clone)]
pub struct Dream {
    pub id: String,
    pub bizarreness: f64,
    pub elements_present: HashSet<String>,
    pub novel_combinations: Vec<(String, String, f64)>,
    pub narrative: String,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub description: String,
    pub key_insights: Vec<Insight>,
    pub novelty: f64,
    pub confidence: f64,
    pub paradigm_shift: String,
}

// ============================================================================
// DREAM NEURAL NETWORK
// ============================================================================

struct DreamNetwork {
    concepts: HashMap<String, ConceptNode>,
    associations: HashMap<(String, String), f64>,
    activations: HashMap<String, f64>,
    noise_level: f64,
    inhibition: f64,
}

struct ConceptNode {
    importance: f64,
    decay: f64,
}

impl DreamNetwork {
    fn new() -> Self {
        Self {
            concepts: HashMap::new(),
            associations: HashMap::new(),
            activations: HashMap::new(),
            noise_level: 0.1,
            inhibition: 1.0,
        }
    }

    fn encode(&mut self, name: &str, importance: f64) {
        self.concepts.insert(name.to_string(), ConceptNode {
            importance,
            decay: 0.1,
        });
        self.activations.insert(name.to_string(), importance);
    }

    fn associate(&mut self, from: &str, to: &str, strength: f64) {
        self.associations.insert((from.to_string(), to.to_string()), strength);
        self.associations.insert((to.to_string(), from.to_string()), strength * 0.8);
    }

    fn enter_rem(&mut self) {
        self.inhibition = 0.15; // Very low - deep REM
        self.noise_level = 0.6;  // High creativity
    }

    fn exit_rem(&mut self) {
        self.inhibition = 1.0;
        self.noise_level = 0.1;
    }

    fn step(&mut self, dt: f64) -> Vec<(String, String, f64)> {
        let mut new_activations = HashMap::new();
        let mut novel = Vec::new();

        for (concept, activation) in &self.activations {
            let node = &self.concepts[concept];
            let decayed = activation * (1.0 - node.decay * dt);
            let noise = (rand_float() - 0.5) * self.noise_level;

            let mut input = 0.0;
            for ((from, to), strength) in &self.associations {
                if to == concept {
                    input += self.activations.get(from).unwrap_or(&0.0) * strength;
                }
            }

            let gated = input * (1.0 - self.inhibition * 0.5);
            new_activations.insert(concept.clone(), (decayed + gated + noise).clamp(0.0, 1.0));
        }

        // Find novel co-activations
        let active: Vec<_> = new_activations.iter()
            .filter(|(_, &a)| a > 0.5)
            .map(|(c, _)| c.clone())
            .collect();

        for i in 0..active.len() {
            for j in i+1..active.len() {
                let key = (active[i].clone(), active[j].clone());
                if !self.associations.contains_key(&key) {
                    let strength = new_activations[&active[i]] * new_activations[&active[j]];
                    if strength > 0.3 {
                        novel.push((active[i].clone(), active[j].clone(), strength));
                    }
                }
            }
        }

        self.activations = new_activations;
        novel
    }

    fn most_active(&self, n: usize) -> Vec<(String, f64)> {
        let mut sorted: Vec<_> = self.activations.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sorted.into_iter().take(n).collect()
    }
}

// ============================================================================
// QUANTUM GRAVITY PROBLEM DEFINITION
// ============================================================================

fn quantum_gravity_problem() -> Problem {
    Problem {
        id: "quantum_gravity".to_string(),
        description: "Unify Quantum Mechanics with General Relativity into a consistent Theory of Everything".to_string(),
        elements: vec![
            // Quantum Mechanics concepts
            ProblemElement {
                name: "superposition".to_string(),
                concept: "particles exist in multiple states simultaneously until measured".to_string(),
                embedding: vec![0.1; 32],
                importance: 1.0,
                relations: vec![
                    ("probability".to_string(), 0.9),
                    ("measurement".to_string(), 0.8),
                    ("wave_function".to_string(), 0.95),
                ],
            },
            ProblemElement {
                name: "wave_function".to_string(),
                concept: "mathematical description of quantum state".to_string(),
                embedding: vec![0.15; 32],
                importance: 0.95,
                relations: vec![
                    ("probability".to_string(), 0.9),
                    ("hilbert_space".to_string(), 0.85),
                ],
            },
            ProblemElement {
                name: "entanglement".to_string(),
                concept: "non-local correlations between distant particles".to_string(),
                embedding: vec![0.2; 32],
                importance: 0.9,
                relations: vec![
                    ("non_locality".to_string(), 0.95),
                    ("information".to_string(), 0.7),
                ],
            },
            ProblemElement {
                name: "discreteness".to_string(),
                concept: "energy comes in discrete quanta, not continuous".to_string(),
                embedding: vec![0.25; 32],
                importance: 0.85,
                relations: vec![
                    ("planck_scale".to_string(), 0.9),
                    ("quantization".to_string(), 0.95),
                ],
            },
            ProblemElement {
                name: "uncertainty".to_string(),
                concept: "fundamental limits on knowing position and momentum".to_string(),
                embedding: vec![0.3; 32],
                importance: 0.9,
                relations: vec![
                    ("measurement".to_string(), 0.8),
                    ("information".to_string(), 0.7),
                ],
            },

            // General Relativity concepts
            ProblemElement {
                name: "spacetime_curvature".to_string(),
                concept: "mass/energy bends the fabric of spacetime".to_string(),
                embedding: vec![0.35; 32],
                importance: 1.0,
                relations: vec![
                    ("gravity".to_string(), 0.95),
                    ("geometry".to_string(), 0.9),
                    ("continuity".to_string(), 0.85),
                ],
            },
            ProblemElement {
                name: "continuity".to_string(),
                concept: "spacetime is smooth and continuous".to_string(),
                embedding: vec![0.4; 32],
                importance: 0.8,
                relations: vec![
                    ("geometry".to_string(), 0.85),
                    ("discreteness".to_string(), -0.9), // CONFLICT!
                ],
            },
            ProblemElement {
                name: "gravity".to_string(),
                concept: "attractive force from curved spacetime".to_string(),
                embedding: vec![0.45; 32],
                importance: 1.0,
                relations: vec![
                    ("mass".to_string(), 0.95),
                    ("spacetime_curvature".to_string(), 0.95),
                ],
            },
            ProblemElement {
                name: "black_hole".to_string(),
                concept: "extreme curvature where GR and QM must meet".to_string(),
                embedding: vec![0.5; 32],
                importance: 0.95,
                relations: vec![
                    ("singularity".to_string(), 0.9),
                    ("information".to_string(), 0.85),
                    ("hawking_radiation".to_string(), 0.9),
                ],
            },
            ProblemElement {
                name: "singularity".to_string(),
                concept: "point of infinite density where equations break".to_string(),
                embedding: vec![0.55; 32],
                importance: 0.9,
                relations: vec![
                    ("infinity".to_string(), 0.95),
                    ("breakdown".to_string(), 0.9),
                ],
            },

            // Bridge concepts (potential solutions)
            ProblemElement {
                name: "holography".to_string(),
                concept: "3D information encoded on 2D boundary (AdS/CFT)".to_string(),
                embedding: vec![0.6; 32],
                importance: 0.7,
                relations: vec![
                    ("information".to_string(), 0.9),
                    ("boundary".to_string(), 0.85),
                    ("duality".to_string(), 0.9),
                ],
            },
            ProblemElement {
                name: "duality".to_string(),
                concept: "two different descriptions of same physics".to_string(),
                embedding: vec![0.65; 32],
                importance: 0.75,
                relations: vec![
                    ("equivalence".to_string(), 0.9),
                    ("transformation".to_string(), 0.8),
                ],
            },
            ProblemElement {
                name: "emergence".to_string(),
                concept: "spacetime emerges from more fundamental quantum structure".to_string(),
                embedding: vec![0.7; 32],
                importance: 0.8,
                relations: vec![
                    ("entanglement".to_string(), 0.85),
                    ("information".to_string(), 0.9),
                ],
            },
            ProblemElement {
                name: "information".to_string(),
                concept: "fundamental building block more basic than matter".to_string(),
                embedding: vec![0.75; 32],
                importance: 0.85,
                relations: vec![
                    ("entropy".to_string(), 0.9),
                    ("qubits".to_string(), 0.85),
                ],
            },
            ProblemElement {
                name: "planck_scale".to_string(),
                concept: "10^-35 meters where QM and GR both matter".to_string(),
                embedding: vec![0.8; 32],
                importance: 0.9,
                relations: vec![
                    ("discreteness".to_string(), 0.85),
                    ("quantum_foam".to_string(), 0.9),
                ],
            },

            // Exotic concepts for creative connections
            ProblemElement {
                name: "quantum_foam".to_string(),
                concept: "spacetime fluctuating wildly at smallest scales".to_string(),
                embedding: vec![0.82; 32],
                importance: 0.6,
                relations: vec![
                    ("uncertainty".to_string(), 0.8),
                    ("virtual_particles".to_string(), 0.85),
                ],
            },
            ProblemElement {
                name: "spin_network".to_string(),
                concept: "discrete quantum states of geometry (LQG)".to_string(),
                embedding: vec![0.84; 32],
                importance: 0.5,
                relations: vec![
                    ("discreteness".to_string(), 0.9),
                    ("geometry".to_string(), 0.85),
                ],
            },
            ProblemElement {
                name: "strings".to_string(),
                concept: "1D vibrating entities replacing point particles".to_string(),
                embedding: vec![0.86; 32],
                importance: 0.5,
                relations: vec![
                    ("vibration".to_string(), 0.9),
                    ("extra_dimensions".to_string(), 0.85),
                ],
            },
            ProblemElement {
                name: "consciousness".to_string(),
                concept: "observer role in measurement - connected to gravity? (Penrose)".to_string(),
                embedding: vec![0.88; 32],
                importance: 0.4,
                relations: vec![
                    ("measurement".to_string(), 0.7),
                    ("collapse".to_string(), 0.75),
                ],
            },
            ProblemElement {
                name: "time".to_string(),
                concept: "what IS time? Emergent? Fundamental? Illusory?".to_string(),
                embedding: vec![0.9; 32],
                importance: 0.85,
                relations: vec![
                    ("spacetime_curvature".to_string(), 0.9),
                    ("entropy".to_string(), 0.8),
                    ("causality".to_string(), 0.85),
                ],
            },
        ],
        failed_approaches: vec![
            "Naive quantization of gravitational field (non-renormalizable infinities)".to_string(),
            "String theory (no experimental predictions, landscape problem)".to_string(),
            "Loop quantum gravity (difficulty recovering smooth spacetime)".to_string(),
            "Treating spacetime as fundamental rather than emergent".to_string(),
            "Ignoring the role of information and entropy".to_string(),
            "Assuming time is fundamental rather than emergent".to_string(),
        ],
        domains: vec![
            "physics".to_string(),
            "mathematics".to_string(),
            "cosmology".to_string(),
            "information theory".to_string(),
        ],
        embedding: vec![0.5; 32],
    }
}

// ============================================================================
// DREAM SOLVER
// ============================================================================

struct QuantumGravityDreamer {
    network: DreamNetwork,
    problem_elements: Vec<String>,
    all_insights: Vec<Insight>,
    dream_count: u64,
    insight_count: u64,
}

impl QuantumGravityDreamer {
    fn new() -> Self {
        Self {
            network: DreamNetwork::new(),
            problem_elements: Vec::new(),
            all_insights: Vec::new(),
            dream_count: 0,
            insight_count: 0,
        }
    }

    fn incubate(&mut self, problem: &Problem) {
        for element in &problem.elements {
            self.network.encode(&element.name, element.importance);
            self.problem_elements.push(element.name.clone());

            for (related, strength) in &element.relations {
                self.network.associate(&element.name, related, *strength);
            }
        }

        for (i, _approach) in problem.failed_approaches.iter().enumerate() {
            self.network.encode(&format!("failed_{}", i), 0.3);
        }
    }

    fn dream(&mut self, steps: usize) -> Dream {
        self.dream_count += 1;
        self.network.enter_rem();

        let mut all_novel = Vec::new();
        let mut active_elements = HashSet::new();

        for _ in 0..steps {
            let novel = self.network.step(0.1);
            all_novel.extend(novel);

            for (concept, activation) in self.network.most_active(8) {
                if self.problem_elements.contains(&concept) && activation > 0.5 {
                    active_elements.insert(concept);
                }
            }
        }

        self.network.exit_rem();

        let bizarreness = (all_novel.len() as f64 / steps as f64).min(1.0);

        // Generate narrative
        let narrative = self.generate_narrative(&active_elements, &all_novel);

        Dream {
            id: format!("dream_{}", self.dream_count),
            bizarreness,
            elements_present: active_elements,
            novel_combinations: all_novel,
            narrative,
        }
    }

    fn generate_narrative(&self, elements: &HashSet<String>, combos: &[(String, String, f64)]) -> String {
        let mut narrative = String::new();

        // Pick most interesting combinations
        let top_combos: Vec<_> = combos.iter()
            .filter(|(a, b, s)| *s > 0.5 && self.problem_elements.contains(a) && self.problem_elements.contains(b))
            .take(3)
            .collect();

        for (a, b, _) in top_combos {
            narrative.push_str(&format!("In the dream, {} and {} merged into one...\n", a, b));
        }

        if elements.len() > 3 {
            narrative.push_str(&format!(
                "A vision of {} intertwined concepts dancing together.\n",
                elements.len()
            ));
        }

        narrative
    }

    fn extract_insights(&mut self, dream: &Dream, problem: &Problem) -> Vec<Insight> {
        let mut insights = Vec::new();

        for (a, b, strength) in &dream.novel_combinations {
            let a_elem = problem.elements.iter().find(|e| &e.name == a);
            let b_elem = problem.elements.iter().find(|e| &e.name == b);

            if a_elem.is_some() || b_elem.is_some() {
                self.insight_count += 1;

                let connection_type = self.infer_connection(a, b, dream.bizarreness);
                let relevance = self.compute_relevance(a, b, problem);

                if relevance > 0.3 {
                    insights.push(Insight {
                        id: format!("insight_{}", self.insight_count),
                        from: a.clone(),
                        to: b.clone(),
                        connection_type,
                        bizarreness: dream.bizarreness,
                        relevance,
                        confidence: relevance * (1.0 - dream.bizarreness * 0.3),
                    });
                }
            }
        }

        insights
    }

    fn infer_connection(&self, a: &str, b: &str, bizarreness: f64) -> ConnectionType {
        // Special physics-aware inference
        if (a.contains("quantum") || a == "discreteness" || a == "entanglement") &&
           (b.contains("spacetime") || b == "gravity" || b == "continuity") {
            return ConnectionType::Duality;
        }

        if a == "emergence" || b == "emergence" {
            return ConnectionType::Emergence;
        }

        if a == "holography" || b == "holography" || a == "information" || b == "information" {
            return ConnectionType::Holography;
        }

        if a.contains("failed") || b.contains("failed") {
            return ConnectionType::Inversion;
        }

        if bizarreness > 0.7 {
            ConnectionType::Synthesis
        } else if bizarreness > 0.4 {
            ConnectionType::Analogy
        } else {
            ConnectionType::CommonGround
        }
    }

    fn compute_relevance(&self, a: &str, b: &str, problem: &Problem) -> f64 {
        let mut score = 0.0;

        for elem in &problem.elements {
            if elem.name == *a || elem.name == *b {
                score += elem.importance;
            }
        }

        // Bonus for connections between QM and GR concepts
        let qm_concepts = ["superposition", "entanglement", "discreteness", "uncertainty", "wave_function"];
        let gr_concepts = ["spacetime_curvature", "gravity", "continuity", "black_hole", "singularity"];

        let a_is_qm = qm_concepts.contains(&a);
        let b_is_qm = qm_concepts.contains(&b);
        let a_is_gr = gr_concepts.contains(&a);
        let b_is_gr = gr_concepts.contains(&b);

        if (a_is_qm && b_is_gr) || (a_is_gr && b_is_qm) {
            score += 0.5; // Bonus for bridging the divide!
        }

        (score / 2.5).min(1.0)
    }

    fn synthesize_solution(&self, problem: &Problem) -> Option<Solution> {
        if self.all_insights.is_empty() {
            return None;
        }

        // Find most promising insights
        let mut ranked: Vec<_> = self.all_insights.iter()
            .map(|i| {
                let score = i.relevance * 0.3 + i.confidence * 0.3 + i.bizarreness * 0.2 +
                    match i.connection_type {
                        ConnectionType::Duality => 0.3,
                        ConnectionType::Emergence => 0.25,
                        ConnectionType::Holography => 0.25,
                        ConnectionType::Synthesis => 0.2,
                        _ => 0.1,
                    };
                (i, score)
            })
            .collect();

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let top: Vec<_> = ranked.iter().take(5).map(|(i, _)| (*i).clone()).collect();

        // Generate paradigm shift description
        let paradigm_shift = self.describe_paradigm_shift(&top);

        let novelty = top.iter().map(|i| i.bizarreness).sum::<f64>() / top.len() as f64;
        let confidence = top.iter().map(|i| i.confidence).sum::<f64>() / top.len() as f64;

        let description = self.generate_solution_description(&top);

        Some(Solution {
            description,
            key_insights: top,
            novelty,
            confidence,
            paradigm_shift,
        })
    }

    fn describe_paradigm_shift(&self, insights: &[Insight]) -> String {
        let mut shift = String::new();

        for insight in insights {
            match insight.connection_type {
                ConnectionType::Duality => {
                    shift.push_str(&format!(
                        "• {} and {} are DUAL DESCRIPTIONS of the same underlying reality\n",
                        insight.from, insight.to
                    ));
                }
                ConnectionType::Emergence => {
                    shift.push_str(&format!(
                        "• {} EMERGES from {} at larger scales\n",
                        insight.to, insight.from
                    ));
                }
                ConnectionType::Holography => {
                    shift.push_str(&format!(
                        "• {} is HOLOGRAPHICALLY encoded in {}\n",
                        insight.from, insight.to
                    ));
                }
                ConnectionType::Synthesis => {
                    shift.push_str(&format!(
                        "• {} and {} SYNTHESIZE into a new unified concept\n",
                        insight.from, insight.to
                    ));
                }
                ConnectionType::Inversion => {
                    shift.push_str(&format!(
                        "• INVERT the failed approach: {} ↔ {}\n",
                        insight.from, insight.to
                    ));
                }
                _ => {}
            }
        }

        shift
    }

    fn generate_solution_description(&self, insights: &[Insight]) -> String {
        let mut desc = String::from("═══════════════════════════════════════════════════════════\n");
        desc.push_str("           DREAM-SYNTHESIZED THEORY OF QUANTUM GRAVITY\n");
        desc.push_str("═══════════════════════════════════════════════════════════\n\n");

        desc.push_str("The dreams reveal a possible path to unification:\n\n");

        for (i, insight) in insights.iter().enumerate() {
            desc.push_str(&format!("{}. ", i + 1));

            match insight.connection_type {
                ConnectionType::Duality => {
                    desc.push_str(&format!(
                        "DUALITY PRINCIPLE: {} and {} are not in conflict—\n   they are complementary descriptions of the same physics,\n   like wave and particle in quantum mechanics.\n\n",
                        insight.from, insight.to
                    ));
                }
                ConnectionType::Emergence => {
                    desc.push_str(&format!(
                        "EMERGENCE PRINCIPLE: {} is not fundamental—\n   it emerges from {}, like temperature emerges from molecular motion.\n\n",
                        insight.to, insight.from
                    ));
                }
                ConnectionType::Holography => {
                    desc.push_str(&format!(
                        "HOLOGRAPHIC PRINCIPLE: The information in {} is encoded\n   on the boundary via {}, reducing dimensions.\n\n",
                        insight.from, insight.to
                    ));
                }
                ConnectionType::Synthesis => {
                    desc.push_str(&format!(
                        "SYNTHESIS: Combining {} with {} creates\n   a new mathematical framework transcending both.\n\n",
                        insight.from, insight.to
                    ));
                }
                _ => {
                    desc.push_str(&format!(
                        "CONNECTION: {} relates to {} through hidden symmetry.\n\n",
                        insight.from, insight.to
                    ));
                }
            }
        }

        desc
    }

    fn solve(&mut self, problem: &Problem, max_cycles: usize, min_bridge_insights: usize) -> SolverResult {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║     OMEGA BRAIN: QUANTUM GRAVITY DREAM SOLVER                    ║");
        println!("║     Attempting to Unify Quantum Mechanics & General Relativity   ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");

        println!("Problem: {}\n", problem.description);
        println!("This is one of the greatest unsolved problems in physics.");
        println!("Einstein spent 30 years on it. Let's see what dreams reveal...\n");

        // Phase 1: Deep immersion
        println!("═══ PHASE 1: DEEP PROBLEM IMMERSION ═══\n");
        self.incubate(problem);
        println!("Encoded {} concepts into dream network", problem.elements.len());
        println!("Encoded {} failed approaches to potentially invert\n", problem.failed_approaches.len());

        // Phase 2: Dream cycles
        println!("═══ PHASE 2: ENTERING DREAM CYCLES ═══\n");
        println!("Seeking insights that bridge Quantum Mechanics ↔ General Relativity...\n");

        let mut bridge_insights = 0;
        let mut cycle = 0;

        while cycle < max_cycles && bridge_insights < min_bridge_insights {
            cycle += 1;

            let dream = self.dream(150); // Longer dreams for complex problem

            // Extract insights
            let insights = self.extract_insights(&dream, problem);

            // Count bridge insights (QM ↔ GR connections)
            let qm = ["superposition", "entanglement", "discreteness", "uncertainty", "wave_function"];
            let gr = ["spacetime_curvature", "gravity", "continuity", "black_hole", "singularity"];

            let new_bridges: Vec<_> = insights.iter()
                .filter(|i| {
                    (qm.contains(&i.from.as_str()) && gr.contains(&i.to.as_str())) ||
                    (gr.contains(&i.from.as_str()) && qm.contains(&i.to.as_str()))
                })
                .collect();

            bridge_insights += new_bridges.len();

            // Progress output
            if cycle % 5 == 0 || !new_bridges.is_empty() {
                println!("  Sleep Cycle {}/{}:", cycle, max_cycles);
                println!("    Bizarreness: {:.2}", dream.bizarreness);
                println!("    Insights this cycle: {}", insights.len());
                println!("    Bridge insights (QM↔GR): {} (total: {})", new_bridges.len(), bridge_insights);

                if !new_bridges.is_empty() {
                    for insight in &new_bridges {
                        println!("      ★ {:?}: {} ↔ {}", insight.connection_type, insight.from, insight.to);
                    }
                }
                println!();
            }

            self.all_insights.extend(insights);

            // Check for breakthrough
            if bridge_insights >= min_bridge_insights {
                println!("\n  ✨ BREAKTHROUGH! Found {} bridge insights!\n", bridge_insights);
                break;
            }
        }

        // Phase 3: Synthesis
        println!("═══ PHASE 3: SYNTHESIZING SOLUTION ═══\n");

        let solution = self.synthesize_solution(problem);

        SolverResult {
            dreams_generated: cycle,
            total_insights: self.all_insights.len(),
            bridge_insights,
            solution,
        }
    }
}

struct SolverResult {
    dreams_generated: usize,
    total_insights: usize,
    bridge_insights: usize,
    solution: Option<Solution>,
}

impl SolverResult {
    fn print_report(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════════╗");
        println!("║                    DREAM SOLVING REPORT                          ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");

        println!("Statistics:");
        println!("  Dreams generated: {}", self.dreams_generated);
        println!("  Total insights: {}", self.total_insights);
        println!("  Bridge insights (QM↔GR): {}", self.bridge_insights);
        println!();

        if let Some(ref sol) = self.solution {
            println!("{}", sol.description);

            println!("PARADIGM SHIFTS DISCOVERED:\n");
            println!("{}", sol.paradigm_shift);

            println!("Solution Metrics:");
            println!("  Novelty: {:.1}%", sol.novelty * 100.0);
            println!("  Confidence: {:.1}%", sol.confidence * 100.0);
        } else {
            println!("No solution synthesized. More sleep cycles may be needed.");
        }
    }
}

// ============================================================================
// UTILITIES
// ============================================================================

fn rand_float() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    static mut COUNTER: u64 = 0;

    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    unsafe {
        COUNTER += 1;
        COUNTER.hash(&mut hasher);
    }
    (hasher.finish() as f64) / (u64::MAX as f64)
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    let problem = quantum_gravity_problem();
    let mut solver = QuantumGravityDreamer::new();

    // Run with up to 50 sleep cycles, seeking at least 10 bridge insights
    let result = solver.solve(&problem, 50, 10);

    result.print_report();

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║  \"I have no special talents. I am only passionately curious.\"    ║");
    println!("║                                        — Albert Einstein          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_gravity_problem() {
        let problem = quantum_gravity_problem();
        assert!(problem.elements.len() >= 15);
        assert!(problem.failed_approaches.len() >= 4);
    }

    #[test]
    fn test_dreamer() {
        let problem = quantum_gravity_problem();
        let mut solver = QuantumGravityDreamer::new();
        solver.incubate(&problem);

        let dream = solver.dream(50);
        assert!(dream.bizarreness >= 0.0);
    }
}
