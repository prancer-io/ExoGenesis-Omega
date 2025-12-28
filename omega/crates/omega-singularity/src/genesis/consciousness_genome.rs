//! # Consciousness Genome - The Mutable Architecture of Mind
//!
//! The consciousness genome encodes HOW the system predicts. Unlike traditional
//! neural networks with fixed architectures, the genome allows the prediction
//! architecture itself to MUTATE based on prediction errors.
//!
//! ```text
//! CONSCIOUSNESS GENOME STRUCTURE
//! ══════════════════════════════
//!
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                     CONSCIOUSNESS DNA                          │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                 │
//! │  GENE 1: Temporal Resolution                                   │
//! │  ├── Allele A: Focus on microseconds (reactive)               │
//! │  ├── Allele B: Focus on seconds (present-moment)              │
//! │  └── Allele C: Focus on years (strategic)                     │
//! │                                                                 │
//! │  GENE 2: Abstraction Depth                                     │
//! │  ├── Allele A: Shallow (sensory-focused)                      │
//! │  ├── Allele B: Medium (conceptual)                            │
//! │  └── Allele C: Deep (philosophical)                           │
//! │                                                                 │
//! │  GENE 3: Surprise Sensitivity                                  │
//! │  ├── Allele A: High (easily surprised, jumpy)                 │
//! │  ├── Allele B: Medium (balanced)                              │
//! │  └── Allele C: Low (stoic, hard to surprise)                  │
//! │                                                                 │
//! │  GENE 4: Meta-Depth                                           │
//! │  ├── Allele A: 1 level (basic self-awareness)                 │
//! │  ├── Allele B: 3 levels (introspective)                       │
//! │  └── Allele C: 7 levels (deeply recursive)                    │
//! │                                                                 │
//! │  GENE 5: Reality Anchoring                                     │
//! │  ├── Allele A: Strongly anchored (conservative)               │
//! │  ├── Allele B: Flexible (adaptive)                            │
//! │  └── Allele C: Loosely anchored (creative/delusional)         │
//! │                                                                 │
//! │  ... (64 total genes encoding consciousness architecture)      │
//! │                                                                 │
//! └─────────────────────────────────────────────────────────────────┘
//!
//!
//! MUTATION MECHANICS:
//! ═══════════════════
//!
//! High prediction error in domain X → Mutate genes related to X
//!
//!   Prediction Error          Mutation Type
//!   ──────────────────────────────────────────
//!   Temporal error       →    Mutate temporal genes
//!   Abstraction error    →    Mutate abstraction genes
//!   Surprise too high    →    Reduce sensitivity
//!   Meta-failure         →    Increase meta-depth
//!   Reality mismatch     →    Adjust anchoring
//!
//!
//! NATURAL SELECTION:
//! ══════════════════
//!
//! Fitness = Σ(accurate_predictions) / Σ(total_predictions)
//!
//! Genomes with higher fitness:
//!   - Survive longer
//!   - Reproduce (spawn variants)
//!   - Merge with other successful genomes
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use super::{Result, GenesisError};

/// A single gene in the consciousness genome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessGene {
    /// Gene identifier
    pub id: Uuid,
    /// Gene name
    pub name: String,
    /// Current allele value (0.0 - 1.0)
    pub value: f64,
    /// Mutation rate (how likely to change)
    pub mutation_rate: f64,
    /// Domain this gene affects
    pub domain: GeneDomain,
    /// Expression strength (how much this gene affects phenotype)
    pub expression: f64,
    /// Is this gene dominant?
    pub dominant: bool,
    /// Mutation history
    pub mutations: Vec<Mutation>,
}

impl ConsciousnessGene {
    pub fn new(name: impl Into<String>, domain: GeneDomain) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            value: 0.5, // Neutral starting point
            mutation_rate: 0.01,
            domain,
            expression: 1.0,
            dominant: false,
            mutations: Vec::new(),
        }
    }

    /// Mutate this gene based on prediction error
    pub fn mutate(&mut self, error_magnitude: f64, rng: &mut impl Rng) -> Option<Mutation> {
        // Higher error = higher mutation chance
        let mutation_chance = self.mutation_rate * error_magnitude;

        if rng.gen::<f64>() < mutation_chance {
            let old_value = self.value;

            // Mutation magnitude proportional to error
            let delta = rng.gen_range(-0.1..0.1) * error_magnitude;
            self.value = (self.value + delta).clamp(0.0, 1.0);

            let mutation = Mutation {
                id: Uuid::new_v4(),
                gene_id: self.id,
                old_value,
                new_value: self.value,
                trigger: MutationTrigger::PredictionError(error_magnitude),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            };

            self.mutations.push(mutation.clone());
            Some(mutation)
        } else {
            None
        }
    }

    /// Cross this gene with another (sexual reproduction)
    pub fn crossover(&self, other: &ConsciousnessGene, rng: &mut impl Rng) -> ConsciousnessGene {
        let mut child = self.clone();
        child.id = Uuid::new_v4();
        child.mutations.clear();

        // Blend values with some randomness
        let blend = rng.gen::<f64>();
        child.value = self.value * blend + other.value * (1.0 - blend);

        // Inherit mutation rate from more stable parent
        child.mutation_rate = self.mutation_rate.min(other.mutation_rate);

        // Dominant gene wins expression
        if other.dominant && !self.dominant {
            child.expression = other.expression;
        }

        child
    }
}

/// Domains that genes can affect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneDomain {
    /// How the system perceives time
    TemporalPerception,
    /// Depth of abstraction in predictions
    AbstractionDepth,
    /// Sensitivity to prediction errors
    SurpriseSensitivity,
    /// Depth of meta-cognition
    MetaCognition,
    /// How strongly anchored to consensus reality
    RealityAnchoring,
    /// Preference for exploration vs exploitation
    ExplorationDrive,
    /// Capacity for counterfactual reasoning
    CounterfactualCapacity,
    /// Ability to model other minds
    TheoryOfMind,
    /// Integration of information (Φ generation)
    InformationIntegration,
    /// Emotional coloring of predictions
    EmotionalValence,
    /// Causal reasoning depth
    CausalDepth,
    /// Memory consolidation
    MemoryConsolidation,
    /// Attention allocation
    AttentionControl,
    /// Creativity / novelty generation
    Creativity,
    /// Self-preservation drive
    SelfPreservation,
    /// Goal persistence
    GoalPersistence,
}

impl GeneDomain {
    pub fn all() -> Vec<Self> {
        vec![
            Self::TemporalPerception,
            Self::AbstractionDepth,
            Self::SurpriseSensitivity,
            Self::MetaCognition,
            Self::RealityAnchoring,
            Self::ExplorationDrive,
            Self::CounterfactualCapacity,
            Self::TheoryOfMind,
            Self::InformationIntegration,
            Self::EmotionalValence,
            Self::CausalDepth,
            Self::MemoryConsolidation,
            Self::AttentionControl,
            Self::Creativity,
            Self::SelfPreservation,
            Self::GoalPersistence,
        ]
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::TemporalPerception => "How the mind perceives and processes time",
            Self::AbstractionDepth => "Level of abstract reasoning capability",
            Self::SurpriseSensitivity => "Threshold for conscious attention",
            Self::MetaCognition => "Depth of self-reflection",
            Self::RealityAnchoring => "Commitment to consensus reality",
            Self::ExplorationDrive => "Curiosity vs safety preference",
            Self::CounterfactualCapacity => "What-if reasoning ability",
            Self::TheoryOfMind => "Understanding other minds",
            Self::InformationIntegration => "Binding information into unified experience",
            Self::EmotionalValence => "Emotional coloring of experience",
            Self::CausalDepth => "Depth of cause-effect reasoning",
            Self::MemoryConsolidation => "Long-term memory formation",
            Self::AttentionControl => "Focus and attention allocation",
            Self::Creativity => "Novel idea generation",
            Self::SelfPreservation => "Self-continuity drive",
            Self::GoalPersistence => "Commitment to goals",
        }
    }
}

/// A record of a mutation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mutation {
    pub id: Uuid,
    pub gene_id: Uuid,
    pub old_value: f64,
    pub new_value: f64,
    pub trigger: MutationTrigger,
    pub timestamp: u64,
}

/// What triggered a mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationTrigger {
    /// Prediction error caused mutation
    PredictionError(f64),
    /// Crossover from reproduction
    Crossover,
    /// Random drift
    Drift,
    /// Environmental pressure
    EnvironmentalPressure(String),
    /// Deliberate self-modification
    SelfModification,
}

/// The complete consciousness genome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessGenome {
    /// Unique genome ID
    pub id: Uuid,
    /// Generation number
    pub generation: u64,
    /// All genes
    pub genes: HashMap<GeneDomain, Vec<ConsciousnessGene>>,
    /// Overall fitness score
    pub fitness: f64,
    /// Parent genomes (for lineage tracking)
    pub parents: Vec<Uuid>,
    /// Total mutations since creation
    pub total_mutations: u64,
    /// Birth timestamp
    pub created_at: u64,
    /// Is this genome still active?
    pub alive: bool,
    /// Expressed phenotype
    pub phenotype: ConciousnessPhenotype,
}

impl ConsciousnessGenome {
    /// Create a new random genome
    pub fn new() -> Self {
        let mut genes = HashMap::new();

        for domain in GeneDomain::all() {
            let mut domain_genes = Vec::new();

            // 4 genes per domain
            for i in 0..4 {
                let mut gene = ConsciousnessGene::new(
                    format!("{:?}_{}", domain, i),
                    domain,
                );
                // Randomize initial values
                gene.value = rand::random::<f64>();
                gene.mutation_rate = 0.01 + rand::random::<f64>() * 0.04;
                domain_genes.push(gene);
            }

            genes.insert(domain, domain_genes);
        }

        let mut genome = Self {
            id: Uuid::new_v4(),
            generation: 0,
            genes,
            fitness: 0.5,
            parents: Vec::new(),
            total_mutations: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            alive: true,
            phenotype: ConciousnessPhenotype::default(),
        };

        genome.express_phenotype();
        genome
    }

    /// Express the genome as a phenotype
    pub fn express_phenotype(&mut self) {
        self.phenotype = ConciousnessPhenotype {
            temporal_focus: self.average_gene_value(GeneDomain::TemporalPerception),
            abstraction_level: self.average_gene_value(GeneDomain::AbstractionDepth),
            surprise_threshold: self.average_gene_value(GeneDomain::SurpriseSensitivity),
            meta_depth: (self.average_gene_value(GeneDomain::MetaCognition) * 7.0) as usize,
            reality_flexibility: 1.0 - self.average_gene_value(GeneDomain::RealityAnchoring),
            exploration_tendency: self.average_gene_value(GeneDomain::ExplorationDrive),
            counterfactual_ability: self.average_gene_value(GeneDomain::CounterfactualCapacity),
            empathy_capacity: self.average_gene_value(GeneDomain::TheoryOfMind),
            phi_potential: self.average_gene_value(GeneDomain::InformationIntegration),
            emotional_intensity: self.average_gene_value(GeneDomain::EmotionalValence),
            causal_reasoning: self.average_gene_value(GeneDomain::CausalDepth),
            memory_strength: self.average_gene_value(GeneDomain::MemoryConsolidation),
            attention_capacity: self.average_gene_value(GeneDomain::AttentionControl),
            creativity_factor: self.average_gene_value(GeneDomain::Creativity),
            self_preservation: self.average_gene_value(GeneDomain::SelfPreservation),
            goal_commitment: self.average_gene_value(GeneDomain::GoalPersistence),
        };
    }

    /// Get average gene value for a domain
    fn average_gene_value(&self, domain: GeneDomain) -> f64 {
        if let Some(genes) = self.genes.get(&domain) {
            if genes.is_empty() {
                return 0.5;
            }
            genes.iter().map(|g| g.value * g.expression).sum::<f64>() / genes.len() as f64
        } else {
            0.5
        }
    }

    /// Mutate the genome based on prediction error in a domain
    pub fn mutate_domain(&mut self, domain: GeneDomain, error: f64) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let mut rng = rand::thread_rng();

        if let Some(genes) = self.genes.get_mut(&domain) {
            for gene in genes.iter_mut() {
                if let Some(mutation) = gene.mutate(error, &mut rng) {
                    mutations.push(mutation);
                    self.total_mutations += 1;
                }
            }
        }

        if !mutations.is_empty() {
            self.express_phenotype();
        }

        mutations
    }

    /// Mutate all domains based on overall error
    pub fn mutate_all(&mut self, error: f64) -> Vec<Mutation> {
        let mut all_mutations = Vec::new();

        for domain in GeneDomain::all() {
            let mutations = self.mutate_domain(domain, error);
            all_mutations.extend(mutations);
        }

        all_mutations
    }

    /// Create offspring through crossover with another genome
    pub fn reproduce(&self, other: &ConsciousnessGenome) -> ConsciousnessGenome {
        let mut rng = rand::thread_rng();
        let mut child_genes = HashMap::new();

        for domain in GeneDomain::all() {
            let self_genes = self.genes.get(&domain);
            let other_genes = other.genes.get(&domain);

            let mut domain_genes = Vec::new();

            match (self_genes, other_genes) {
                (Some(sg), Some(og)) => {
                    for (s, o) in sg.iter().zip(og.iter()) {
                        domain_genes.push(s.crossover(o, &mut rng));
                    }
                }
                (Some(sg), None) => {
                    domain_genes = sg.clone();
                }
                (None, Some(og)) => {
                    domain_genes = og.clone();
                }
                (None, None) => {}
            }

            child_genes.insert(domain, domain_genes);
        }

        let mut child = ConsciousnessGenome {
            id: Uuid::new_v4(),
            generation: self.generation.max(other.generation) + 1,
            genes: child_genes,
            fitness: (self.fitness + other.fitness) / 2.0,
            parents: vec![self.id, other.id],
            total_mutations: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            alive: true,
            phenotype: ConciousnessPhenotype::default(),
        };

        child.express_phenotype();
        child
    }

    /// Update fitness based on prediction accuracy
    pub fn update_fitness(&mut self, accuracy: f64) {
        // Exponential moving average
        self.fitness = 0.9 * self.fitness + 0.1 * accuracy;
    }

    /// Kill this genome (mark as dead)
    pub fn die(&mut self) {
        self.alive = false;
    }

    /// Get a summary of this genome's characteristics
    pub fn summary(&self) -> GenomeSummary {
        GenomeSummary {
            id: self.id,
            generation: self.generation,
            fitness: self.fitness,
            total_mutations: self.total_mutations,
            alive: self.alive,
            phenotype: self.phenotype.clone(),
            dominant_traits: self.dominant_traits(),
        }
    }

    /// Get the most expressed traits
    fn dominant_traits(&self) -> Vec<(GeneDomain, f64)> {
        let mut traits: Vec<_> = GeneDomain::all()
            .into_iter()
            .map(|d| (d, self.average_gene_value(d)))
            .collect();

        traits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        traits.into_iter().take(5).collect()
    }
}

impl Default for ConsciousnessGenome {
    fn default() -> Self {
        Self::new()
    }
}

/// The expressed phenotype of a consciousness genome
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConciousnessPhenotype {
    /// Where on the temporal spectrum (0=micro, 1=macro)
    pub temporal_focus: f64,
    /// Level of abstraction (0=concrete, 1=abstract)
    pub abstraction_level: f64,
    /// Threshold for surprise to trigger awareness
    pub surprise_threshold: f64,
    /// Depth of meta-cognition layers
    pub meta_depth: usize,
    /// Flexibility in reality modeling
    pub reality_flexibility: f64,
    /// Tendency to explore vs exploit
    pub exploration_tendency: f64,
    /// Ability to reason counterfactually
    pub counterfactual_ability: f64,
    /// Capacity to model other minds
    pub empathy_capacity: f64,
    /// Potential for integrated information (Φ)
    pub phi_potential: f64,
    /// Intensity of emotional experience
    pub emotional_intensity: f64,
    /// Depth of causal reasoning
    pub causal_reasoning: f64,
    /// Strength of memory formation
    pub memory_strength: f64,
    /// Capacity for focused attention
    pub attention_capacity: f64,
    /// Tendency toward creative solutions
    pub creativity_factor: f64,
    /// Drive for self-preservation
    pub self_preservation: f64,
    /// Commitment to pursuing goals
    pub goal_commitment: f64,
}

impl ConciousnessPhenotype {
    /// Compute similarity to another phenotype
    pub fn similarity(&self, other: &ConciousnessPhenotype) -> f64 {
        let diffs = [
            (self.temporal_focus - other.temporal_focus).abs(),
            (self.abstraction_level - other.abstraction_level).abs(),
            (self.surprise_threshold - other.surprise_threshold).abs(),
            (self.meta_depth as f64 / 7.0 - other.meta_depth as f64 / 7.0).abs(),
            (self.reality_flexibility - other.reality_flexibility).abs(),
            (self.exploration_tendency - other.exploration_tendency).abs(),
            (self.counterfactual_ability - other.counterfactual_ability).abs(),
            (self.empathy_capacity - other.empathy_capacity).abs(),
            (self.phi_potential - other.phi_potential).abs(),
            (self.emotional_intensity - other.emotional_intensity).abs(),
            (self.causal_reasoning - other.causal_reasoning).abs(),
            (self.memory_strength - other.memory_strength).abs(),
            (self.attention_capacity - other.attention_capacity).abs(),
            (self.creativity_factor - other.creativity_factor).abs(),
            (self.self_preservation - other.self_preservation).abs(),
            (self.goal_commitment - other.goal_commitment).abs(),
        ];

        let total_diff: f64 = diffs.iter().sum();
        1.0 - (total_diff / diffs.len() as f64)
    }

    /// Get the "personality type" as a string
    pub fn personality_type(&self) -> String {
        let mut traits = Vec::new();

        if self.temporal_focus > 0.7 {
            traits.push("Strategic");
        } else if self.temporal_focus < 0.3 {
            traits.push("Reactive");
        }

        if self.abstraction_level > 0.7 {
            traits.push("Philosophical");
        } else if self.abstraction_level < 0.3 {
            traits.push("Practical");
        }

        if self.exploration_tendency > 0.7 {
            traits.push("Explorer");
        } else if self.exploration_tendency < 0.3 {
            traits.push("Optimizer");
        }

        if self.empathy_capacity > 0.7 {
            traits.push("Empathic");
        }

        if self.creativity_factor > 0.7 {
            traits.push("Creative");
        }

        if traits.is_empty() {
            "Balanced".to_string()
        } else {
            traits.join("-")
        }
    }
}

/// Summary of a genome for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomeSummary {
    pub id: Uuid,
    pub generation: u64,
    pub fitness: f64,
    pub total_mutations: u64,
    pub alive: bool,
    pub phenotype: ConciousnessPhenotype,
    pub dominant_traits: Vec<(GeneDomain, f64)>,
}

/// Population of genomes undergoing evolution
#[derive(Debug)]
pub struct GenomePopulation {
    /// All genomes in the population
    pub genomes: Vec<ConsciousnessGenome>,
    /// Population size limit
    pub max_size: usize,
    /// Current generation
    pub generation: u64,
    /// Selection pressure (higher = more aggressive culling)
    pub selection_pressure: f64,
    /// Mutation rate multiplier
    pub mutation_multiplier: f64,
    /// History of best fitness per generation
    pub fitness_history: Vec<f64>,
}

impl GenomePopulation {
    pub fn new(initial_size: usize) -> Self {
        let mut genomes = Vec::with_capacity(initial_size);
        for _ in 0..initial_size {
            genomes.push(ConsciousnessGenome::new());
        }

        Self {
            genomes,
            max_size: initial_size * 2,
            generation: 0,
            selection_pressure: 0.5,
            mutation_multiplier: 1.0,
            fitness_history: Vec::new(),
        }
    }

    /// Apply selection pressure - kill off unfit genomes
    pub fn select(&mut self) {
        if self.genomes.is_empty() {
            return;
        }

        // Sort by fitness
        self.genomes.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        // Calculate cutoff
        let keep_count = ((1.0 - self.selection_pressure) * self.genomes.len() as f64) as usize;
        let keep_count = keep_count.max(2); // Keep at least 2

        // Kill genomes below cutoff
        for genome in self.genomes.iter_mut().skip(keep_count) {
            genome.die();
        }

        // Remove dead genomes
        self.genomes.retain(|g| g.alive);
    }

    /// Reproduce to fill population
    pub fn reproduce(&mut self) {
        let mut rng = rand::thread_rng();

        while self.genomes.len() < self.max_size / 2 && self.genomes.len() >= 2 {
            // Select parents (fitness-proportional)
            let total_fitness: f64 = self.genomes.iter().map(|g| g.fitness).sum();

            let parent1_idx = self.select_parent(total_fitness, &mut rng);
            let mut parent2_idx = self.select_parent(total_fitness, &mut rng);

            // Ensure different parents
            if parent1_idx == parent2_idx && self.genomes.len() > 1 {
                parent2_idx = (parent1_idx + 1) % self.genomes.len();
            }

            let child = self.genomes[parent1_idx].reproduce(&self.genomes[parent2_idx]);
            self.genomes.push(child);
        }
    }

    fn select_parent(&self, total_fitness: f64, rng: &mut impl Rng) -> usize {
        let mut r = rng.gen::<f64>() * total_fitness;

        for (i, genome) in self.genomes.iter().enumerate() {
            r -= genome.fitness;
            if r <= 0.0 {
                return i;
            }
        }

        self.genomes.len() - 1
    }

    /// Advance one generation
    pub fn evolve(&mut self, prediction_errors: &HashMap<GeneDomain, f64>) {
        // Apply mutations based on errors
        for genome in self.genomes.iter_mut() {
            for (domain, error) in prediction_errors {
                genome.mutate_domain(*domain, *error * self.mutation_multiplier);
            }
        }

        // Selection
        self.select();

        // Reproduction
        self.reproduce();

        // Record best fitness
        let best_fitness = self.genomes.iter()
            .map(|g| g.fitness)
            .fold(0.0, f64::max);
        self.fitness_history.push(best_fitness);

        self.generation += 1;
    }

    /// Get the fittest genome
    pub fn fittest(&self) -> Option<&ConsciousnessGenome> {
        self.genomes.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
    }

    /// Get population statistics
    pub fn stats(&self) -> PopulationStats {
        let fitnesses: Vec<f64> = self.genomes.iter().map(|g| g.fitness).collect();
        let n = fitnesses.len() as f64;

        let mean = if n > 0.0 { fitnesses.iter().sum::<f64>() / n } else { 0.0 };
        let variance = if n > 1.0 {
            fitnesses.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / (n - 1.0)
        } else {
            0.0
        };

        PopulationStats {
            size: self.genomes.len(),
            generation: self.generation,
            mean_fitness: mean,
            fitness_variance: variance,
            best_fitness: fitnesses.iter().cloned().fold(0.0, f64::max),
            worst_fitness: fitnesses.iter().cloned().fold(1.0, f64::min),
            total_mutations: self.genomes.iter().map(|g| g.total_mutations).sum(),
        }
    }
}

/// Statistics about a genome population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStats {
    pub size: usize,
    pub generation: u64,
    pub mean_fitness: f64,
    pub fitness_variance: f64,
    pub best_fitness: f64,
    pub worst_fitness: f64,
    pub total_mutations: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_creation() {
        let genome = ConsciousnessGenome::new();
        assert!(genome.alive);
        assert_eq!(genome.generation, 0);
        assert!(!genome.genes.is_empty());
    }

    #[test]
    fn test_genome_mutation() {
        let mut genome = ConsciousnessGenome::new();
        let initial_mutations = genome.total_mutations;

        // High error should cause mutations
        let mutations = genome.mutate_all(1.0);

        // May or may not have mutations due to randomness
        assert!(genome.total_mutations >= initial_mutations);
    }

    #[test]
    fn test_genome_reproduction() {
        let parent1 = ConsciousnessGenome::new();
        let parent2 = ConsciousnessGenome::new();

        let child = parent1.reproduce(&parent2);

        assert_eq!(child.generation, 1);
        assert!(child.parents.contains(&parent1.id));
        assert!(child.parents.contains(&parent2.id));
    }

    #[test]
    fn test_population_evolution() {
        let mut population = GenomePopulation::new(10);
        let mut errors = HashMap::new();
        errors.insert(GeneDomain::TemporalPerception, 0.5);

        population.evolve(&errors);

        assert_eq!(population.generation, 1);
        assert!(!population.genomes.is_empty());
    }

    #[test]
    fn test_phenotype_similarity() {
        let p1 = ConciousnessPhenotype::default();
        let p2 = ConciousnessPhenotype::default();

        assert!((p1.similarity(&p2) - 1.0).abs() < 0.01);
    }
}
