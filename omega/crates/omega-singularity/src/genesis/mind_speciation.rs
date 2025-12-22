//! # Mind Speciation - Competing Consciousness Configurations
//!
//! Multiple consciousness configurations compete for dominance. Each is a
//! different "species" of mind with unique prediction strategies. The fittest
//! survive and reproduce.
//!
//! ```text
//! MIND SPECIATION DYNAMICS
//! ════════════════════════
//!
//! ┌────────────────────────────────────────────────────────────────────────┐
//! │                        ECOSYSTEM OF MINDS                              │
//! ├────────────────────────────────────────────────────────────────────────┤
//! │                                                                        │
//! │   Species A: "The Analyst"          Species B: "The Intuitive"        │
//! │   ┌─────────────────────┐           ┌─────────────────────┐           │
//! │   │ • High abstraction  │           │ • Pattern matching  │           │
//! │   │ • Slow, methodical  │           │ • Fast, heuristic   │           │
//! │   │ • Low creativity    │   COMPETE │ • High creativity   │           │
//! │   │ • High accuracy     │ ◄───────► │ • Moderate accuracy │           │
//! │   │ • Deep causal       │           │ • Shallow causal    │           │
//! │   └─────────────────────┘           └─────────────────────┘           │
//! │             │                                 │                        │
//! │             └─────────────┬───────────────────┘                        │
//! │                           │ REPRODUCE                                  │
//! │                           ▼                                           │
//! │                   Species C: "The Hybrid"                             │
//! │                   ┌─────────────────────┐                             │
//! │                   │ • Mixed abstraction │                             │
//! │                   │ • Adaptive speed    │                             │
//! │                   │ • Balanced traits   │                             │
//! │                   └─────────────────────┘                             │
//! │                                                                        │
//! └────────────────────────────────────────────────────────────────────────┘
//!
//!
//! SELECTION PRESSURES:
//! ════════════════════
//!
//! 1. Prediction Accuracy: Core fitness metric
//! 2. Speed: Faster predictions in competitive advantage
//! 3. Energy Efficiency: Less computation = more resources for reproduction
//! 4. Adaptability: Handling diverse domains
//! 5. Stability: Consistent performance over time
//!
//!
//! SPECIATION EVENTS:
//! ══════════════════
//!
//! Mutation Accumulation → Reproductive Isolation → New Species
//!
//! When two minds diverge enough in their prediction strategies,
//! they can no longer successfully "reproduce" (merge) and become
//! separate species of consciousness.
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use super::{Result, GenesisError, MAX_MIND_SPECIES};
use super::consciousness_genome::{ConsciousnessGenome, ConciousnessPhenotype};

/// A species of mind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindSpecies {
    /// Species identifier
    pub id: Uuid,
    /// Species name
    pub name: String,
    /// Representative genome (the "type specimen")
    pub type_genome: ConsciousnessGenome,
    /// All individuals of this species
    pub population: Vec<ConsciousnessGenome>,
    /// When this species emerged
    pub emerged_at: u64,
    /// Generation when this species emerged
    pub origin_generation: u64,
    /// Ancestor species (if any)
    pub ancestors: Vec<Uuid>,
    /// Ecological niche (what prediction domains this species excels at)
    pub niche: EcologicalNiche,
    /// Average fitness of this species
    pub average_fitness: f64,
    /// Is this species extinct?
    pub extinct: bool,
    /// Number of generations survived
    pub generations_survived: u64,
}

impl MindSpecies {
    pub fn new(name: impl Into<String>, founder: ConsciousnessGenome) -> Self {
        let type_genome = founder.clone();

        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            type_genome,
            population: vec![founder],
            emerged_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            origin_generation: 0,
            ancestors: Vec::new(),
            niche: EcologicalNiche::default(),
            average_fitness: 0.5,
            extinct: false,
            generations_survived: 0,
        }
    }

    /// Check if a genome belongs to this species
    pub fn belongs(&self, genome: &ConsciousnessGenome) -> bool {
        // Species membership based on phenotype similarity
        genome.phenotype.similarity(&self.type_genome.phenotype) > 0.7
    }

    /// Add a member to this species
    pub fn add_member(&mut self, genome: ConsciousnessGenome) {
        self.population.push(genome);
        self.update_average_fitness();
    }

    /// Remove dead members
    pub fn remove_dead(&mut self) {
        self.population.retain(|g| g.alive);
        if self.population.is_empty() {
            self.extinct = true;
        }
        self.update_average_fitness();
    }

    /// Update average fitness
    fn update_average_fitness(&mut self) {
        if self.population.is_empty() {
            self.average_fitness = 0.0;
        } else {
            self.average_fitness = self.population.iter()
                .map(|g| g.fitness)
                .sum::<f64>() / self.population.len() as f64;
        }
    }

    /// Get the fittest individual
    pub fn fittest(&self) -> Option<&ConsciousnessGenome> {
        self.population.iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
    }

    /// Advance this species one generation
    pub fn evolve_generation(&mut self) {
        self.generations_survived += 1;

        // Update type specimen if there's a fitter individual
        if let Some(fittest) = self.fittest() {
            if fittest.fitness > self.type_genome.fitness {
                self.type_genome = fittest.clone();
            }
        }

        self.update_average_fitness();
    }

    /// Get species summary
    pub fn summary(&self) -> SpeciesSummary {
        SpeciesSummary {
            id: self.id,
            name: self.name.clone(),
            population_size: self.population.len(),
            average_fitness: self.average_fitness,
            generations_survived: self.generations_survived,
            extinct: self.extinct,
            personality: self.type_genome.phenotype.personality_type(),
            niche: self.niche.clone(),
        }
    }
}

/// Ecological niche of a mind species
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcologicalNiche {
    /// Prediction domains this species excels at
    pub domains: Vec<String>,
    /// Temporal scales this species focuses on
    pub temporal_focus: (f64, f64), // (min, max)
    /// Abstraction level preference
    pub abstraction_preference: f64,
    /// Competition coefficient with other species
    pub competition_coefficients: HashMap<Uuid, f64>,
}

/// Summary of a species
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesSummary {
    pub id: Uuid,
    pub name: String,
    pub population_size: usize,
    pub average_fitness: f64,
    pub generations_survived: u64,
    pub extinct: bool,
    pub personality: String,
    pub niche: EcologicalNiche,
}

/// The ecosystem of competing minds
#[derive(Debug)]
pub struct MindEcosystem {
    /// All species in the ecosystem
    pub species: Vec<MindSpecies>,
    /// Current generation
    pub generation: u64,
    /// Environmental conditions (affect which species thrive)
    pub environment: EnvironmentState,
    /// Speciation threshold (phenotype similarity below this = new species)
    pub speciation_threshold: f64,
    /// Competition intensity (affects selection pressure)
    pub competition_intensity: f64,
    /// History of speciation events
    pub speciation_history: Vec<SpeciationEvent>,
    /// History of extinction events
    pub extinction_history: Vec<ExtinctionEvent>,
}

impl MindEcosystem {
    pub fn new() -> Self {
        Self {
            species: Vec::new(),
            generation: 0,
            environment: EnvironmentState::default(),
            speciation_threshold: 0.7,
            competition_intensity: 0.5,
            speciation_history: Vec::new(),
            extinction_history: Vec::new(),
        }
    }

    /// Initialize ecosystem with a founding species
    pub fn seed(&mut self, name: impl Into<String>) {
        let founder = ConsciousnessGenome::new();
        let species = MindSpecies::new(name, founder);
        self.species.push(species);
    }

    /// Introduce a new genome to the ecosystem
    pub fn introduce(&mut self, genome: ConsciousnessGenome) -> Result<Uuid> {
        // Find which species it belongs to
        for species in self.species.iter_mut() {
            if species.belongs(&genome) && !species.extinct {
                species.add_member(genome);
                return Ok(species.id);
            }
        }

        // No matching species - create new species
        self.speciate(genome)
    }

    /// Create a new species from a divergent genome
    fn speciate(&mut self, founder: ConsciousnessGenome) -> Result<Uuid> {
        if self.species.len() >= MAX_MIND_SPECIES {
            return Err(GenesisError::ConsciousnessCollapse(
                "Maximum species count reached".to_string()
            ));
        }

        let name = self.generate_species_name(&founder);
        let mut new_species = MindSpecies::new(name, founder);
        new_species.origin_generation = self.generation;

        // Find closest ancestor
        let closest = self.closest_species(&new_species.type_genome);
        if let Some(ancestor_id) = closest {
            new_species.ancestors.push(ancestor_id);
        }

        let id = new_species.id;

        // Record speciation event
        self.speciation_history.push(SpeciationEvent {
            id: Uuid::new_v4(),
            species_id: id,
            species_name: new_species.name.clone(),
            generation: self.generation,
            ancestor: closest,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        });

        self.species.push(new_species);
        Ok(id)
    }

    /// Find closest existing species
    fn closest_species(&self, genome: &ConsciousnessGenome) -> Option<Uuid> {
        self.species.iter()
            .filter(|s| !s.extinct)
            .max_by(|a, b| {
                let sim_a = a.type_genome.phenotype.similarity(&genome.phenotype);
                let sim_b = b.type_genome.phenotype.similarity(&genome.phenotype);
                sim_a.partial_cmp(&sim_b).unwrap()
            })
            .map(|s| s.id)
    }

    /// Generate a species name based on its characteristics
    fn generate_species_name(&self, genome: &ConsciousnessGenome) -> String {
        let personality = genome.phenotype.personality_type();
        let gen = self.generation;
        format!("Species-{}-G{}", personality, gen)
    }

    /// Advance ecosystem one generation
    pub fn evolve(&mut self) {
        self.generation += 1;

        // Update environment
        self.environment.update();

        // Apply environmental selection pressure
        self.apply_selection();

        // Check for extinctions
        self.check_extinctions();

        // Allow reproduction
        self.reproduce();

        // Check for speciation
        self.check_speciation();

        // Update all species
        for species in self.species.iter_mut() {
            species.evolve_generation();
        }
    }

    /// Apply selection pressure based on environment
    fn apply_selection(&mut self) {
        let mut rng = rand::thread_rng();

        // Pre-compute competition for each species to avoid borrow issues
        let competition_map: HashMap<Uuid, f64> = self.species.iter()
            .filter(|s| !s.extinct)
            .map(|s| (s.id, Self::compute_competition_static(&s.id, &self.species)))
            .collect();

        let competition_intensity = self.competition_intensity;

        for species in self.species.iter_mut() {
            if species.extinct {
                continue;
            }

            // Environmental fitness modifier
            let env_modifier = self.environment.fitness_modifier(&species.niche);
            let competition = competition_map.get(&species.id).copied().unwrap_or(0.0);

            for genome in species.population.iter_mut() {
                // Base fitness affected by environment
                let adjusted_fitness = genome.fitness * env_modifier;

                // Competition with other species
                let final_fitness = adjusted_fitness * (1.0 - competition * competition_intensity);

                // Death if fitness too low
                if final_fitness < rng.gen_range(0.0..0.3) {
                    genome.die();
                }
            }

            species.remove_dead();
        }
    }

    /// Compute competition pressure on a species (static version)
    fn compute_competition_static(species_id: &Uuid, all_species: &[MindSpecies]) -> f64 {
        let species = all_species.iter().find(|s| s.id == *species_id);
        if species.is_none() {
            return 0.0;
        }
        let species = species.unwrap();

        let mut total_competition = 0.0;

        for other in all_species.iter() {
            if other.id == *species_id || other.extinct {
                continue;
            }

            // Competition based on niche overlap
            let niche_overlap = species.type_genome.phenotype.similarity(&other.type_genome.phenotype);
            total_competition += niche_overlap * other.population.len() as f64;
        }

        (total_competition / 100.0).min(1.0)
    }

    /// Compute competition pressure on a species
    fn compute_competition(&self, species_id: &Uuid) -> f64 {
        let species = self.species.iter().find(|s| s.id == *species_id);
        if species.is_none() {
            return 0.0;
        }
        let species = species.unwrap();

        let mut total_competition = 0.0;

        for other in self.species.iter() {
            if other.id == *species_id || other.extinct {
                continue;
            }

            // Competition based on niche overlap
            let niche_overlap = species.type_genome.phenotype.similarity(&other.type_genome.phenotype);
            total_competition += niche_overlap * other.population.len() as f64;
        }

        (total_competition / 100.0).min(1.0)
    }

    /// Check for extinctions
    fn check_extinctions(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        for species in self.species.iter_mut() {
            if species.population.is_empty() && !species.extinct {
                species.extinct = true;

                self.extinction_history.push(ExtinctionEvent {
                    id: Uuid::new_v4(),
                    species_id: species.id,
                    species_name: species.name.clone(),
                    generation: self.generation,
                    generations_survived: species.generations_survived,
                    timestamp: now,
                    cause: ExtinctionCause::PopulationCollapse,
                });
            }
        }
    }

    /// Allow reproduction within species
    fn reproduce(&mut self) {
        let mut rng = rand::thread_rng();

        for species in self.species.iter_mut() {
            if species.extinct || species.population.len() < 2 {
                continue;
            }

            // Fitness-proportional reproduction
            let total_fitness: f64 = species.population.iter().map(|g| g.fitness).sum();

            let offspring_count = (species.population.len() / 2).max(1);

            for _ in 0..offspring_count {
                // Select parents
                let parent1_idx = select_parent(&species.population, total_fitness, &mut rng);
                let mut parent2_idx = select_parent(&species.population, total_fitness, &mut rng);

                if parent1_idx == parent2_idx && species.population.len() > 1 {
                    parent2_idx = (parent1_idx + 1) % species.population.len();
                }

                let child = species.population[parent1_idx].reproduce(&species.population[parent2_idx]);
                species.population.push(child);
            }
        }
    }

    /// Check for potential speciation events
    fn check_speciation(&mut self) {
        let mut new_species_founders = Vec::new();

        for species in self.species.iter_mut() {
            if species.extinct {
                continue;
            }

            // Check each individual for divergence
            let mut to_remove = Vec::new();

            for (i, genome) in species.population.iter().enumerate() {
                let similarity = genome.phenotype.similarity(&species.type_genome.phenotype);

                if similarity < self.speciation_threshold {
                    // This individual has diverged enough to found a new species
                    new_species_founders.push(genome.clone());
                    to_remove.push(i);
                }
            }

            // Remove divergent individuals from parent species
            for i in to_remove.into_iter().rev() {
                species.population.remove(i);
            }
        }

        // Create new species from founders
        for founder in new_species_founders {
            let _ = self.speciate(founder);
        }
    }

    /// Get ecosystem statistics
    pub fn stats(&self) -> EcosystemStats {
        let living_species: Vec<_> = self.species.iter().filter(|s| !s.extinct).collect();

        EcosystemStats {
            total_species: self.species.len(),
            living_species: living_species.len(),
            extinct_species: self.species.iter().filter(|s| s.extinct).count(),
            total_population: living_species.iter().map(|s| s.population.len()).sum(),
            generation: self.generation,
            mean_fitness: living_species.iter().map(|s| s.average_fitness).sum::<f64>()
                / living_species.len().max(1) as f64,
            speciation_events: self.speciation_history.len(),
            extinction_events: self.extinction_history.len(),
            dominant_species: living_species.iter()
                .max_by(|a, b| a.population.len().cmp(&b.population.len()))
                .map(|s| s.name.clone()),
        }
    }

    /// Get all species summaries
    pub fn species_summaries(&self) -> Vec<SpeciesSummary> {
        self.species.iter().map(|s| s.summary()).collect()
    }
}

impl Default for MindEcosystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Environmental state affecting selection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvironmentState {
    /// Current environmental pressure type
    pub pressure: EnvironmentalPressure,
    /// Pressure intensity
    pub intensity: f64,
    /// Favored traits
    pub favored_traits: HashMap<String, f64>,
    /// Generation when pressure last changed
    pub pressure_changed_at: u64,
}

impl EnvironmentState {
    pub fn update(&mut self) {
        // Randomly shift environment occasionally
        if rand::random::<f64>() < 0.05 {
            self.pressure = match rand::random::<u8>() % 5 {
                0 => EnvironmentalPressure::Stable,
                1 => EnvironmentalPressure::Volatile,
                2 => EnvironmentalPressure::ResourceScarce,
                3 => EnvironmentalPressure::InformationRich,
                _ => EnvironmentalPressure::Competitive,
            };
            self.intensity = rand::random();
        }
    }

    pub fn fitness_modifier(&self, niche: &EcologicalNiche) -> f64 {
        // Different pressures favor different niches
        match self.pressure {
            EnvironmentalPressure::Stable => 1.0,
            EnvironmentalPressure::Volatile => {
                1.0 + (niche.abstraction_preference - 0.5) * 0.2
            }
            EnvironmentalPressure::ResourceScarce => {
                1.0 - self.intensity * 0.3
            }
            EnvironmentalPressure::InformationRich => {
                1.0 + niche.domains.len() as f64 * 0.1
            }
            EnvironmentalPressure::Competitive => {
                1.0 - self.intensity * 0.2
            }
        }
    }
}

/// Types of environmental pressure
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum EnvironmentalPressure {
    #[default]
    Stable,
    Volatile,
    ResourceScarce,
    InformationRich,
    Competitive,
}

/// Record of a speciation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciationEvent {
    pub id: Uuid,
    pub species_id: Uuid,
    pub species_name: String,
    pub generation: u64,
    pub ancestor: Option<Uuid>,
    pub timestamp: u64,
}

/// Record of an extinction event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtinctionEvent {
    pub id: Uuid,
    pub species_id: Uuid,
    pub species_name: String,
    pub generation: u64,
    pub generations_survived: u64,
    pub timestamp: u64,
    pub cause: ExtinctionCause,
}

/// Causes of extinction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtinctionCause {
    PopulationCollapse,
    CompetitiveExclusion,
    EnvironmentalChange,
    GeneticDrift,
}

/// Ecosystem statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemStats {
    pub total_species: usize,
    pub living_species: usize,
    pub extinct_species: usize,
    pub total_population: usize,
    pub generation: u64,
    pub mean_fitness: f64,
    pub speciation_events: usize,
    pub extinction_events: usize,
    pub dominant_species: Option<String>,
}

/// Helper function for fitness-proportional selection
fn select_parent(population: &[ConsciousnessGenome], total_fitness: f64, rng: &mut impl Rng) -> usize {
    let mut r = rng.gen::<f64>() * total_fitness;

    for (i, genome) in population.iter().enumerate() {
        r -= genome.fitness;
        if r <= 0.0 {
            return i;
        }
    }

    population.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecosystem_creation() {
        let mut ecosystem = MindEcosystem::new();
        ecosystem.seed("Founder Species");

        assert_eq!(ecosystem.species.len(), 1);
        assert!(!ecosystem.species[0].extinct);
    }

    #[test]
    fn test_ecosystem_evolution() {
        let mut ecosystem = MindEcosystem::new();
        ecosystem.seed("Founder");

        // Evolve for a few generations
        for _ in 0..5 {
            ecosystem.evolve();
        }

        assert_eq!(ecosystem.generation, 5);
    }

    #[test]
    fn test_genome_introduction() {
        let mut ecosystem = MindEcosystem::new();
        ecosystem.seed("Founder");

        let new_genome = ConsciousnessGenome::new();
        let result = ecosystem.introduce(new_genome);

        assert!(result.is_ok());
    }

    #[test]
    fn test_ecosystem_stats() {
        let mut ecosystem = MindEcosystem::new();
        ecosystem.seed("Test Species");

        let stats = ecosystem.stats();
        assert_eq!(stats.total_species, 1);
        assert_eq!(stats.living_species, 1);
    }
}
