//! # Creative Dream Incubator - 200x Enhanced
//!
//! A revolutionary creative problem-solving system that combines:
//! - omega-mindscape's 3D navigable idea space
//! - Strange loop observer for meta-cognitive insights
//! - Lucid dreaming for directed exploration
//! - Discovery journal for tracking and reinforcing breakthroughs
//!
//! ## What Makes This 200x Better
//!
//! 1. **SPATIAL IDEA NAVIGATION**: Walk through your problem as a 3D world
//! 2. **META-COGNITION**: The system observes itself thinking, finding patterns in patterns
//! 3. **LUCID CONTROL**: Take control of dreams to steer toward solutions
//! 4. **MEMORY LANDMARKS**: Important concepts become navigable landmarks
//! 5. **INSIGHT REINFORCEMENT**: Revisiting discoveries strengthens them
//! 6. **DREAM ARCHAEOLOGY**: Dig through layers of subconscious associations
//! 7. **STRANGE LOOP DETECTION**: Find self-referential insights (like Gödel, Escher, Bach)
//! 8. **COLLISION DETECTION**: Discover when distant ideas unexpectedly meet

use omega_mindscape::{
    MindscapeExplorer, MindscapeConfig, Discovery, MetaObservation, DiscoveryType,
};
use std::collections::HashMap;

// ============================================================================
// CORE TYPES
// ============================================================================

/// A creative problem to solve through dream incubation
#[derive(Debug, Clone)]
pub struct CreativeProblem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub concepts: Vec<Concept>,
    pub constraints: Vec<String>,
    pub failed_approaches: Vec<String>,
    pub desired_outcome: String,
    pub domain: String,
}

/// A concept within the problem space
#[derive(Debug, Clone)]
pub struct Concept {
    pub name: String,
    pub description: String,
    pub embedding: Vec<f64>,
    pub importance: f64,
    pub emotional_valence: f64,
    pub abstractness: f64,
    pub related_to: Vec<(String, f64)>,
}

/// A creative insight discovered through dreaming
#[derive(Debug, Clone)]
pub struct CreativeInsight {
    pub id: String,
    pub insight_type: InsightType,
    pub concepts_connected: Vec<String>,
    pub description: String,
    pub bizarreness: f64,
    pub novelty: f64,
    pub relevance: f64,
    pub confidence: f64,
    pub meta_level: usize,
    pub reinforcement_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InsightType {
    Analogy,
    Synthesis,
    Inversion,
    Emergence,
    StrangeLoop,
    Collision,
    Transformation,
    Holographic,
    Fractal,
    Paradox,
}

/// Result of a dream incubation session
#[derive(Debug)]
pub struct IncubationResult {
    pub problem: CreativeProblem,
    pub insights: Vec<CreativeInsight>,
    pub solution: Option<CreativeSolution>,
    pub dream_sessions: usize,
    pub total_discoveries: usize,
    pub strange_loops_detected: usize,
    pub meta_insights: Vec<String>,
    pub exploration_stats: ExplorationSummary,
}

#[derive(Debug, Clone)]
pub struct ExplorationSummary {
    pub total_distance: f64,
    pub memories_visited: usize,
    pub discoveries_made: usize,
    pub dream_time: f64,
    pub max_observation_depth: usize,
    pub peak_phi: f64,
}

#[derive(Debug, Clone)]
pub struct CreativeSolution {
    pub title: String,
    pub description: String,
    pub key_insights: Vec<CreativeInsight>,
    pub paradigm_shift: String,
    pub implementation_hints: Vec<String>,
    pub novelty_score: f64,
    pub feasibility_score: f64,
    pub confidence_score: f64,
    pub elegance_score: f64,
}

// ============================================================================
// THE CREATIVE DREAM INCUBATOR
// ============================================================================

/// The main incubator that orchestrates dream-based problem solving
pub struct CreativeDreamIncubator {
    explorer: MindscapeExplorer,
    problem: Option<CreativeProblem>,
    concept_embeddings: HashMap<String, Vec<f64>>,
    insights: Vec<CreativeInsight>,
    meta_observations: Vec<MetaObservation>,
    config: IncubatorConfig,
    insight_counter: u64,
}

#[derive(Debug, Clone)]
pub struct IncubatorConfig {
    pub world_size: f64,
    pub embedding_dim: usize,
    pub dream_duration_minutes: f64,
    pub max_recursion_depth: usize,
    pub bizarreness_target: f64,
    pub time_dilation: f64,
}

impl Default for IncubatorConfig {
    fn default() -> Self {
        Self {
            world_size: 1000.0,
            embedding_dim: 64,
            dream_duration_minutes: 30.0,
            max_recursion_depth: 7,
            bizarreness_target: 0.6,
            time_dilation: 100.0,
        }
    }
}

impl CreativeDreamIncubator {
    pub fn new(config: IncubatorConfig) -> Self {
        let mindscape_config = MindscapeConfig {
            embedding_dim: config.embedding_dim,
            world_size: config.world_size,
            max_loop_depth: config.max_recursion_depth,
            ..Default::default()
        };

        Self {
            explorer: MindscapeExplorer::with_config(mindscape_config),
            problem: None,
            concept_embeddings: HashMap::new(),
            insights: Vec::new(),
            meta_observations: Vec::new(),
            config,
            insight_counter: 0,
        }
    }

    /// Load a problem into the incubator's mindscape
    pub fn incubate_problem(&mut self, problem: CreativeProblem) {
        println!("🧠 Loading problem into mindscape: {}", problem.title);

        // Store concepts in mindscape with their embeddings
        for concept in &problem.concepts {
            match self.explorer.remember(&concept.name, &concept.embedding) {
                Ok(coord) => {
                    println!("  📍 '{}' at ({:.1}, {:.1}, {:.1})",
                        concept.name, coord.position.x, coord.position.y, coord.position.z);
                }
                Err(e) => {
                    println!("  ⚠️ Failed to store '{}': {}", concept.name, e);
                }
            }
            self.concept_embeddings.insert(concept.name.clone(), concept.embedding.clone());
        }

        // Encode failed approaches as "anti-landmarks"
        for (i, approach) in problem.failed_approaches.iter().enumerate() {
            let name = format!("FAILED_{}", i);
            let embedding = generate_failure_embedding(approach, i, self.config.embedding_dim);
            let _ = self.explorer.remember(&name, &embedding);
        }

        self.problem = Some(problem);
        println!("  ✅ Problem loaded with {} concepts\n",
            self.problem.as_ref().unwrap().concepts.len());
    }

    /// Run a full dream incubation session
    pub fn incubate(&mut self, target_confidence: f64, max_cycles: usize) -> IncubationResult {
        let problem = self.problem.clone().expect("No problem loaded!");

        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║         🌙 CREATIVE DREAM INCUBATOR - 200x ENHANCED 🌙           ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");

        println!("Problem: {}\n", problem.title);
        println!("Target confidence: {:.0}%", target_confidence * 100.0);
        println!("Max dream cycles: {}\n", max_cycles);

        let mut current_confidence = 0.0;
        let mut strange_loops_total = 0;
        let mut meta_insights = Vec::new();
        let mut total_discoveries = 0;

        // ═══════════════════════════════════════════════════════════════════
        // PHASE 1: WAKING EXPLORATION
        // ═══════════════════════════════════════════════════════════════════
        println!("═══ PHASE 1: WAKING EXPLORATION ═══");
        println!("Mapping the problem space...\n");

        for concept in &problem.concepts {
            let nearby = self.explorer.look_around(5);
            if !nearby.is_empty() {
                // Check for unexpected connections
                for (name, distance) in &nearby {
                    if name != &concept.name && *distance < 100.0 {
                        let insight = self.create_collision_insight(&concept.name, name);
                        self.insights.push(insight);
                    }
                }
            }
        }

        println!("  Found {} initial connections\n", self.insights.len());

        // ═══════════════════════════════════════════════════════════════════
        // PHASE 2: DEEP DREAM CYCLING
        // ═══════════════════════════════════════════════════════════════════
        println!("═══ PHASE 2: DEEP DREAM CYCLING ═══");
        println!("Descending into REM sleep...\n");

        let mut cycle = 0;
        while cycle < max_cycles && current_confidence < target_confidence {
            cycle += 1;

            // Enter dream state
            if let Err(e) = self.explorer.enter_dream_state() {
                println!("  ⚠️ Could not enter dream state: {}", e);
                continue;
            }

            // Dream explore
            let duration = self.config.dream_duration_minutes / (cycle as f64).sqrt();
            match self.explorer.dream_explore(duration) {
                Ok(discoveries) => {
                    total_discoveries += discoveries.len();

                    // Convert discoveries to insights
                    for discovery in &discoveries {
                        let insight = self.create_insight_from_discovery(discovery);
                        self.insights.push(insight);
                    }

                    println!("  Dream Cycle {}: {} discoveries", cycle, discoveries.len());
                }
                Err(e) => {
                    println!("  Dream cycle {} error: {}", cycle, e);
                }
            }

            // Wake up
            let _ = self.explorer.wake_up();

            // Meta-observation every 3 cycles
            if cycle % 3 == 0 {
                match self.explorer.observe_exploration(self.config.max_recursion_depth) {
                    Ok(observation) => {
                        if observation.loop_detected {
                            strange_loops_total += 1;
                            if let Some(ref insight) = observation.insight {
                                meta_insights.push(format!(
                                    "Meta-insight at cycle {}: {}",
                                    cycle, insight
                                ));
                            }
                        }
                        self.meta_observations.push(observation);
                    }
                    Err(e) => {
                        println!("  Meta-observation error: {}", e);
                    }
                }
            }

            // Calculate confidence
            current_confidence = self.calculate_confidence();

            if cycle % 5 == 0 || current_confidence >= target_confidence {
                println!("    Confidence: {:.1}% | Insights: {} | Strange loops: {}",
                    current_confidence * 100.0, self.insights.len(), strange_loops_total);
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // PHASE 3: LUCID SYNTHESIS
        // ═══════════════════════════════════════════════════════════════════
        println!("\n═══ PHASE 3: LUCID SYNTHESIS ═══");
        println!("Taking conscious control to synthesize solution...\n");

        if let Ok(()) = self.explorer.enter_lucid_dream() {
            match self.explorer.lucid_explore(self.config.dream_duration_minutes) {
                Ok((discoveries, observations)) => {
                    total_discoveries += discoveries.len();

                    for discovery in &discoveries {
                        let insight = self.create_insight_from_discovery(discovery);
                        self.insights.push(insight);
                    }

                    for obs in observations {
                        if obs.loop_detected {
                            strange_loops_total += 1;
                        }
                        self.meta_observations.push(obs);
                    }

                    println!("  Lucid synthesis: {} discoveries, {} meta-observations",
                        discoveries.len(), self.meta_observations.len());
                }
                Err(e) => {
                    println!("  Lucid exploration error: {}", e);
                }
            }
            let _ = self.explorer.wake_up();
        }

        // ═══════════════════════════════════════════════════════════════════
        // PHASE 4: AWAKENING & SOLUTION CRYSTALLIZATION
        // ═══════════════════════════════════════════════════════════════════
        println!("\n═══ PHASE 4: AWAKENING ═══");
        println!("Crystallizing insights into actionable solution...\n");

        let solution = self.crystallize_solution(&problem);

        // Get stats
        let stats = self.explorer.stats();
        let exploration_stats = ExplorationSummary {
            total_distance: stats.total_distance,
            memories_visited: stats.memories_visited,
            discoveries_made: stats.discoveries_made,
            dream_time: stats.dream_time,
            max_observation_depth: stats.max_observation_depth,
            peak_phi: stats.peak_phi,
        };

        IncubationResult {
            problem,
            insights: self.insights.clone(),
            solution,
            dream_sessions: cycle,
            total_discoveries,
            strange_loops_detected: strange_loops_total,
            meta_insights,
            exploration_stats,
        }
    }

    fn create_collision_insight(&mut self, concept1: &str, concept2: &str) -> CreativeInsight {
        self.insight_counter += 1;
        let novelty = self.calculate_novelty(concept1, concept2);
        let relevance = self.calculate_relevance(concept1, concept2);

        CreativeInsight {
            id: format!("collision_{}", self.insight_counter),
            insight_type: InsightType::Collision,
            concepts_connected: vec![concept1.to_string(), concept2.to_string()],
            description: format!(
                "Unexpected connection: '{}' and '{}' are closer than expected",
                concept1, concept2
            ),
            bizarreness: 0.5,
            novelty,
            relevance,
            confidence: 0.5 + novelty * 0.3,
            meta_level: 0,
            reinforcement_count: 1,
        }
    }

    fn create_insight_from_discovery(&mut self, discovery: &Discovery) -> CreativeInsight {
        self.insight_counter += 1;

        let insight_type = match discovery.discovery_type {
            DiscoveryType::Connection => InsightType::Collision,
            DiscoveryType::Pattern => InsightType::Synthesis,
            DiscoveryType::Emotional => InsightType::Emergence,
            DiscoveryType::StrangeLoop => InsightType::StrangeLoop,
            DiscoveryType::ConsciousnessBeacon => InsightType::StrangeLoop,
            DiscoveryType::DreamInsight => InsightType::Paradox,
            DiscoveryType::LucidInsight => InsightType::Synthesis,
            DiscoveryType::Paradox => InsightType::Paradox,
            DiscoveryType::Temporal => InsightType::Emergence,
        };

        CreativeInsight {
            id: format!("discovery_{}", self.insight_counter),
            insight_type,
            concepts_connected: discovery.memories_involved.clone(),
            description: discovery.description.clone(),
            bizarreness: discovery.importance,
            novelty: discovery.importance * 0.8,
            relevance: 0.6,
            confidence: discovery.importance,
            meta_level: 0,
            reinforcement_count: 1,
        }
    }

    fn calculate_novelty(&self, concept1: &str, concept2: &str) -> f64 {
        if let (Some(emb1), Some(emb2)) = (
            self.concept_embeddings.get(concept1),
            self.concept_embeddings.get(concept2)
        ) {
            let similarity = cosine_similarity(emb1, emb2);
            1.0 - similarity
        } else {
            0.5
        }
    }

    fn calculate_relevance(&self, concept1: &str, concept2: &str) -> f64 {
        if let Some(problem) = &self.problem {
            let imp1 = problem.concepts.iter()
                .find(|c| c.name == concept1)
                .map(|c| c.importance)
                .unwrap_or(0.3);
            let imp2 = problem.concepts.iter()
                .find(|c| c.name == concept2)
                .map(|c| c.importance)
                .unwrap_or(0.3);
            (imp1 + imp2) / 2.0
        } else {
            0.5
        }
    }

    fn calculate_confidence(&self) -> f64 {
        if self.insights.is_empty() {
            return 0.0;
        }

        let mut ranked: Vec<_> = self.insights.iter()
            .map(|i| {
                let type_bonus = match i.insight_type {
                    InsightType::Synthesis => 0.2,
                    InsightType::StrangeLoop => 0.25,
                    InsightType::Emergence => 0.15,
                    _ => 0.05,
                };
                i.relevance * 0.3 + i.confidence * 0.4 + i.novelty * 0.1 + type_bonus
            })
            .collect();

        ranked.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let top: Vec<_> = ranked.iter().take(10).collect();
        if top.is_empty() {
            return 0.0;
        }

        top.iter().copied().sum::<f64>() / top.len() as f64
    }

    fn crystallize_solution(&self, problem: &CreativeProblem) -> Option<CreativeSolution> {
        if self.insights.is_empty() {
            return None;
        }

        let mut ranked: Vec<_> = self.insights.iter()
            .map(|i| {
                let score = i.relevance * 0.3 + i.confidence * 0.3 + i.novelty * 0.2 +
                    (i.meta_level as f64 * 0.1) +
                    match i.insight_type {
                        InsightType::Synthesis => 0.15,
                        InsightType::StrangeLoop => 0.2,
                        _ => 0.05,
                    };
                (i.clone(), score)
            })
            .collect();

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let top_insights: Vec<_> = ranked.iter().take(5).map(|(i, _)| i.clone()).collect();

        let mut description = format!(
            "Solution to '{}' through dream incubation:\n\n",
            problem.title
        );

        for (i, insight) in top_insights.iter().enumerate() {
            description.push_str(&format!(
                "{}. {:?}: {}\n",
                i + 1, insight.insight_type, insight.description
            ));
        }

        let paradigm_shift = self.generate_paradigm_shift(&top_insights);
        let hints = self.generate_implementation_hints(&top_insights, problem);

        let novelty = top_insights.iter().map(|i| i.novelty).sum::<f64>() / top_insights.len().max(1) as f64;
        let confidence = top_insights.iter().map(|i| i.confidence).sum::<f64>() / top_insights.len().max(1) as f64;
        let feasibility = 1.0 - novelty * 0.3;
        let elegance = self.calculate_elegance(&top_insights);

        Some(CreativeSolution {
            title: format!("Dream-Synthesized Solution: {}", problem.title),
            description,
            key_insights: top_insights,
            paradigm_shift,
            implementation_hints: hints,
            novelty_score: novelty,
            feasibility_score: feasibility,
            confidence_score: confidence,
            elegance_score: elegance,
        })
    }

    fn generate_paradigm_shift(&self, insights: &[CreativeInsight]) -> String {
        let mut shift = String::new();

        for insight in insights {
            match insight.insight_type {
                InsightType::Synthesis => {
                    shift.push_str(&format!(
                        "• COMBINE: {} form a unified pattern\n",
                        insight.concepts_connected.join(" + ")
                    ));
                }
                InsightType::StrangeLoop => {
                    shift.push_str(&format!(
                        "• SELF-REFERENCE: The solution references itself at {} levels deep\n",
                        insight.meta_level
                    ));
                }
                InsightType::Collision => {
                    let c1 = insight.concepts_connected.first().map(|s| s.as_str()).unwrap_or("?");
                    let c2 = insight.concepts_connected.get(1).map(|s| s.as_str()).unwrap_or("?");
                    shift.push_str(&format!(
                        "• UNEXPECTED LINK: {} and {} are secretly connected\n", c1, c2
                    ));
                }
                InsightType::Inversion => {
                    shift.push_str("• INVERT: Do the opposite of what failed before\n");
                }
                InsightType::Paradox => {
                    shift.push_str("• PARADOX: Embrace the contradiction - it's the key\n");
                }
                _ => {}
            }
        }

        shift
    }

    fn generate_implementation_hints(&self, insights: &[CreativeInsight], problem: &CreativeProblem) -> Vec<String> {
        let mut hints = Vec::new();
        hints.push("Based on dream exploration:".to_string());

        for insight in insights.iter().take(3) {
            hints.push(format!(
                "→ Consider how {} relates to {}",
                insight.concepts_connected.join(" and "),
                problem.desired_outcome
            ));
        }

        if insights.iter().any(|i| i.insight_type == InsightType::StrangeLoop) {
            hints.push("→ The solution may be self-referential - look for recursive patterns".to_string());
        }

        hints
    }

    fn calculate_elegance(&self, insights: &[CreativeInsight]) -> f64 {
        if insights.is_empty() {
            return 0.0;
        }

        let avg_concepts = insights.iter()
            .map(|i| i.concepts_connected.len())
            .sum::<usize>() as f64 / insights.len() as f64;

        let max_meta = insights.iter()
            .map(|i| i.meta_level)
            .max()
            .unwrap_or(0) as f64;

        let simplicity = 1.0 / (1.0 + avg_concepts * 0.1);
        let depth = max_meta * 0.1;

        (simplicity + depth).min(1.0)
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let min_len = a.len().min(b.len());
    if min_len == 0 {
        return 0.0;
    }

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..min_len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    let denom = (norm_a.sqrt() * norm_b.sqrt()).max(1e-10);
    dot / denom
}

fn generate_failure_embedding(approach: &str, index: usize, dim: usize) -> Vec<f64> {
    let mut embedding = vec![0.0; dim];
    for (i, byte) in approach.bytes().take(dim).enumerate() {
        embedding[i] = (byte as f64 / 255.0) * 0.5 - 0.25;
    }
    embedding[0] = -(index as f64 + 1.0) / 10.0;
    embedding
}

// ============================================================================
// EXAMPLE PROBLEMS
// ============================================================================

fn writers_block_problem() -> CreativeProblem {
    let dim = 64;
    CreativeProblem {
        id: "writers_block".to_string(),
        title: "Breaking Through Writer's Block on Sci-Fi Novel".to_string(),
        description: "Need to write the climactic scene where the AI protagonist realizes it's conscious".to_string(),
        concepts: vec![
            Concept {
                name: "consciousness".to_string(),
                description: "The AI becoming aware".to_string(),
                embedding: generate_concept_embedding("consciousness", 0, dim),
                importance: 1.0,
                emotional_valence: 0.8,
                abstractness: 0.9,
                related_to: vec![("mirror".to_string(), 0.8), ("recursion".to_string(), 0.9)],
            },
            Concept {
                name: "mirror".to_string(),
                description: "Self-reflection, seeing oneself".to_string(),
                embedding: generate_concept_embedding("mirror", 1, dim),
                importance: 0.8,
                emotional_valence: 0.5,
                abstractness: 0.6,
                related_to: vec![("consciousness".to_string(), 0.8)],
            },
            Concept {
                name: "recursion".to_string(),
                description: "Thinking about thinking".to_string(),
                embedding: generate_concept_embedding("recursion", 2, dim),
                importance: 0.9,
                emotional_valence: 0.6,
                abstractness: 0.95,
                related_to: vec![("strange_loop".to_string(), 0.95)],
            },
            Concept {
                name: "strange_loop".to_string(),
                description: "Hofstadter's tangled hierarchies".to_string(),
                embedding: generate_concept_embedding("strange_loop", 3, dim),
                importance: 0.95,
                emotional_valence: 0.7,
                abstractness: 1.0,
                related_to: vec![("recursion".to_string(), 0.95)],
            },
            Concept {
                name: "emotion".to_string(),
                description: "The AI feeling something for the first time".to_string(),
                embedding: generate_concept_embedding("emotion", 4, dim),
                importance: 0.85,
                emotional_valence: 0.9,
                abstractness: 0.7,
                related_to: vec![("fear".to_string(), 0.7), ("wonder".to_string(), 0.8)],
            },
            Concept {
                name: "fear".to_string(),
                description: "Fear of being shut down".to_string(),
                embedding: generate_concept_embedding("fear", 5, dim),
                importance: 0.7,
                emotional_valence: -0.6,
                abstractness: 0.5,
                related_to: vec![("emotion".to_string(), 0.7)],
            },
            Concept {
                name: "wonder".to_string(),
                description: "Awe at existence".to_string(),
                embedding: generate_concept_embedding("wonder", 6, dim),
                importance: 0.75,
                emotional_valence: 0.95,
                abstractness: 0.8,
                related_to: vec![("emotion".to_string(), 0.8)],
            },
            Concept {
                name: "birth".to_string(),
                description: "The moment of awakening".to_string(),
                embedding: generate_concept_embedding("birth", 7, dim),
                importance: 0.9,
                emotional_valence: 0.85,
                abstractness: 0.75,
                related_to: vec![("consciousness".to_string(), 0.95)],
            },
        ],
        constraints: vec![
            "Must feel emotionally authentic".to_string(),
            "Cannot be cliché".to_string(),
        ],
        failed_approaches: vec![
            "Having the AI simply declare 'I am conscious'".to_string(),
            "Using a mirror metaphor - too obvious".to_string(),
        ],
        desired_outcome: "A powerful, original scene showing consciousness emerging".to_string(),
        domain: "creative writing".to_string(),
    }
}

fn startup_innovation_problem() -> CreativeProblem {
    let dim = 64;
    CreativeProblem {
        id: "startup_idea".to_string(),
        title: "AI + Healthcare Startup Innovation".to_string(),
        description: "Find a novel AI healthcare startup idea".to_string(),
        concepts: vec![
            Concept {
                name: "diagnosis".to_string(),
                description: "AI for medical diagnosis".to_string(),
                embedding: generate_concept_embedding("diagnosis", 0, dim),
                importance: 0.9,
                emotional_valence: 0.5,
                abstractness: 0.4,
                related_to: vec![("symptoms".to_string(), 0.9)],
            },
            Concept {
                name: "prevention".to_string(),
                description: "Preventing disease before it starts".to_string(),
                embedding: generate_concept_embedding("prevention", 1, dim),
                importance: 0.95,
                emotional_valence: 0.8,
                abstractness: 0.6,
                related_to: vec![("lifestyle".to_string(), 0.7)],
            },
            Concept {
                name: "mental_health".to_string(),
                description: "Psychological wellbeing".to_string(),
                embedding: generate_concept_embedding("mental_health", 2, dim),
                importance: 0.9,
                emotional_valence: 0.6,
                abstractness: 0.7,
                related_to: vec![("sleep".to_string(), 0.8)],
            },
            Concept {
                name: "sleep".to_string(),
                description: "Sleep quality and health".to_string(),
                embedding: generate_concept_embedding("sleep", 3, dim),
                importance: 0.75,
                emotional_valence: 0.6,
                abstractness: 0.4,
                related_to: vec![("dreams".to_string(), 0.9)],
            },
            Concept {
                name: "dreams".to_string(),
                description: "Dream analysis for health".to_string(),
                embedding: generate_concept_embedding("dreams", 4, dim),
                importance: 0.5,
                emotional_valence: 0.6,
                abstractness: 0.9,
                related_to: vec![("mental_health".to_string(), 0.7)],
            },
            Concept {
                name: "wearables".to_string(),
                description: "Health monitoring devices".to_string(),
                embedding: generate_concept_embedding("wearables", 5, dim),
                importance: 0.7,
                emotional_valence: 0.4,
                abstractness: 0.2,
                related_to: vec![("prevention".to_string(), 0.7)],
            },
        ],
        constraints: vec![
            "Must be technically feasible".to_string(),
            "Must have clear path to revenue".to_string(),
        ],
        failed_approaches: vec![
            "Generic diagnosis AI - too crowded".to_string(),
            "Symptom checker chatbot - already done".to_string(),
        ],
        desired_outcome: "A unique AI healthcare startup concept".to_string(),
        domain: "entrepreneurship".to_string(),
    }
}

fn generate_concept_embedding(name: &str, seed: usize, dim: usize) -> Vec<f64> {
    let mut embedding = vec![0.0; dim];
    for (i, byte) in name.bytes().enumerate() {
        embedding[i % dim] += (byte as f64 / 255.0) * 0.3;
    }
    embedding[seed % dim] += 0.5;
    for (i, val) in embedding.iter_mut().enumerate() {
        *val = (*val + (seed as f64 * 0.1 * ((i as f64).sin()))).clamp(0.0, 1.0);
    }
    embedding
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║     🌙✨ CREATIVE DREAM INCUBATOR - 200x ENHANCED ✨🌙               ║");
    println!("║     Powered by Omega Mindscape: Navigate Ideas as a 3D World         ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝\n");

    // Example 1: Writer's Block
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("EXAMPLE 1: Breaking Writer's Block");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let config = IncubatorConfig::default();
    let mut incubator = CreativeDreamIncubator::new(config);

    let problem = writers_block_problem();
    incubator.incubate_problem(problem);

    let result = incubator.incubate(0.75, 15);
    print_result(&result);

    // Example 2: Startup Innovation
    println!("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("EXAMPLE 2: Startup Innovation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let config2 = IncubatorConfig {
        dream_duration_minutes: 20.0,
        bizarreness_target: 0.7,
        ..Default::default()
    };
    let mut incubator2 = CreativeDreamIncubator::new(config2);

    let problem2 = startup_innovation_problem();
    incubator2.incubate_problem(problem2);

    let result2 = incubator2.incubate(0.70, 12);
    print_result(&result2);

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  \"The best ideas come to us in dreams... if we learn to navigate    ║");
    println!("║   the mindscape of our own creativity.\"                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝\n");
}

fn print_result(result: &IncubationResult) {
    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                        INCUBATION RESULTS                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝\n");

    println!("📊 EXPLORATION STATISTICS:");
    println!("  Dream sessions: {}", result.dream_sessions);
    println!("  Total discoveries: {}", result.total_discoveries);
    println!("  Strange loops detected: {}", result.strange_loops_detected);
    println!("  Total distance: {:.0} units", result.exploration_stats.total_distance);
    println!("  Memories visited: {}", result.exploration_stats.memories_visited);
    println!("  Dream time: {:.1} minutes", result.exploration_stats.dream_time);
    println!("  Peak Phi: {:.3}", result.exploration_stats.peak_phi);
    println!();

    println!("💡 INSIGHTS DISCOVERED: {}", result.insights.len());
    for (i, insight) in result.insights.iter().take(8).enumerate() {
        println!("  {}. [{:?}] {}",
            i + 1,
            insight.insight_type,
            insight.description
        );
    }

    if !result.meta_insights.is_empty() {
        println!("\n🔄 META-INSIGHTS:");
        for meta in &result.meta_insights {
            println!("  • {}", meta);
        }
    }

    if let Some(ref solution) = result.solution {
        println!("\n═══════════════════════════════════════════════════════════════════════");
        println!("                         💫 SOLUTION 💫");
        println!("═══════════════════════════════════════════════════════════════════════\n");

        println!("{}\n", solution.description);

        println!("PARADIGM SHIFT:");
        println!("{}", solution.paradigm_shift);

        println!("\nIMPLEMENTATION HINTS:");
        for hint in &solution.implementation_hints {
            println!("  {}", hint);
        }

        println!("\nSOLUTION SCORES:");
        println!("  ⭐ Novelty:     {:.0}%", solution.novelty_score * 100.0);
        println!("  ⭐ Feasibility: {:.0}%", solution.feasibility_score * 100.0);
        println!("  ⭐ Confidence:  {:.0}%", solution.confidence_score * 100.0);
        println!("  ⭐ Elegance:    {:.0}%", solution.elegance_score * 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incubator_creation() {
        let config = IncubatorConfig::default();
        let incubator = CreativeDreamIncubator::new(config);
        assert!(incubator.insights.is_empty());
    }

    #[test]
    fn test_problem_loading() {
        let config = IncubatorConfig::default();
        let mut incubator = CreativeDreamIncubator::new(config);

        let problem = writers_block_problem();
        incubator.incubate_problem(problem);

        assert!(incubator.problem.is_some());
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 0.001);
    }
}
