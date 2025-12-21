//! Causal Discovery - Mining Aging Causes from Simulated Lives
//!
//! Runs millions of simulated lives with varied genetics, environments,
//! and lifestyles, then mines the data to discover what actually causes
//! aging, disease, and death.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────────┐
//!  │                     CAUSAL DISCOVERY ENGINE                             │
//!  ├─────────────────────────────────────────────────────────────────────────┤
//!  │                                                                         │
//!  │                    SIMULATE MILLIONS OF LIVES                           │
//!  │                                                                         │
//!  │  ┌────────────────────────────────────────────────────────────────┐    │
//!  │  │                                                                 │    │
//!  │  │   Person 1        Person 2        Person 3       ...   Person N│    │
//!  │  │   ┌─────────┐     ┌─────────┐     ┌─────────┐           ┌────┐│    │
//!  │  │   │ Genome A│     │ Genome B│     │ Genome C│           │ ...││    │
//!  │  │   │ Life: XY│     │ Life: XZ│     │ Life: YZ│           │    ││    │
//!  │  │   │ Age→Die │     │ Age→Die │     │ Age→Die │           │    ││    │
//!  │  │   │ 73y CVD │     │ 89y Nat │     │ 67y Can │           │    ││    │
//!  │  │   └─────────┘     └─────────┘     └─────────┘           └────┘│    │
//!  │  │                                                                 │    │
//!  │  └────────────────────────────────────────────────────────────────┘    │
//!  │                                   │                                     │
//!  │                                   ▼                                     │
//!  │  ┌────────────────────────────────────────────────────────────────┐    │
//!  │  │                    PATTERN MINING                               │    │
//!  │  │                                                                 │    │
//!  │  │   • What gene variants correlate with longer life?             │    │
//!  │  │   • What molecular events precede disease by 10+ years?        │    │
//!  │  │   • Which pathways are causal vs correlational?                │    │
//!  │  │   • What interventions would have extended life?               │    │
//!  │  │   • What's the interaction between genes and lifestyle?        │    │
//!  │  │                                                                 │    │
//!  │  └────────────────────────────────────────────────────────────────┘    │
//!  │                                   │                                     │
//!  │                                   ▼                                     │
//!  │  ┌────────────────────────────────────────────────────────────────┐    │
//!  │  │                    CAUSAL INSIGHTS                              │    │
//!  │  │                                                                 │    │
//!  │  │   "Mitochondrial dysfunction at age 45 precedes heart          │    │
//!  │  │    failure by 20 years in 78% of cases"                        │    │
//!  │  │                                                                 │    │
//!  │  │   "FOXO3 enhanced function + CR → +12 years median lifespan"   │    │
//!  │  │                                                                 │    │
//!  │  │   "Senescent burden >15% is point of no return"                │    │
//!  │  │                                                                 │    │
//!  │  └────────────────────────────────────────────────────────────────┘    │
//!  │                                                                         │
//!  └─────────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use crate::genome::{Genome, Gene, GeneVariant, VariantEffect, AgingRole};
use crate::organism::{Organism, Lifestyle, DeathCause, DiseaseType, Organ, CausalFactor, CausalFactorType};
use crate::{Result, LongevityError};

/// Configuration for population simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationConfig {
    /// Number of individuals to simulate
    pub population_size: usize,
    /// Enable parallel simulation
    pub parallel: bool,
    /// Record detailed trajectories (memory intensive)
    pub detailed_trajectories: bool,
    /// Lifestyle variation
    pub lifestyle_variation: bool,
    /// Intervention simulation
    pub simulate_interventions: bool,
}

impl Default for PopulationConfig {
    fn default() -> Self {
        Self {
            population_size: 10000,
            parallel: true,
            detailed_trajectories: false,
            lifestyle_variation: true,
            simulate_interventions: true,
        }
    }
}

/// Summary of a simulated life
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeSummary {
    pub id: Uuid,
    /// Lifespan achieved
    pub lifespan: f64,
    /// Cause of death
    pub death_cause: DeathCause,
    /// Key genetic variants
    pub genetic_variants: Vec<(Gene, VariantEffect, f64)>, // (gene, effect, longevity_impact)
    /// Lifestyle score
    pub lifestyle_score: f64,
    /// Diseases developed
    pub diseases: Vec<(DiseaseType, f64)>, // (disease, onset_age)
    /// Key biomarker values at midlife (age 50)
    pub midlife_biomarkers: Option<MidlifeBiomarkers>,
    /// Contributing factors to death
    pub death_factors: Vec<CausalFactor>,
    /// Age acceleration (bio age - chrono age at death)
    pub final_age_acceleration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidlifeBiomarkers {
    pub epigenetic_age: f64,
    pub inflammation: f64,
    pub senescence_burden: f64,
    pub mitochondrial_function: f64,
    pub telomere_length: u32,
}

/// A discovered causal pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalPattern {
    pub id: Uuid,
    /// What's the cause?
    pub cause: PatternCause,
    /// What's the effect?
    pub effect: PatternEffect,
    /// How strong is the association?
    pub strength: f64,
    /// Is this likely causal (vs correlational)?
    pub causal_confidence: f64,
    /// How many lives support this?
    pub supporting_lives: usize,
    /// Temporal relationship
    pub temporal_gap_years: Option<f64>,
    /// Effect size
    pub effect_size: f64,
    /// P-value equivalent
    pub statistical_significance: f64,
    /// Natural language description
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternCause {
    Gene(Gene, VariantEffect),
    Lifestyle(String),
    Biomarker(String, f64), // (name, threshold)
    Pathway(AgingRole),
    Combination(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternEffect {
    Lifespan(f64),        // Change in years
    DiseaseRisk(DiseaseType, f64), // Disease, relative risk
    OrganFailure(Organ, f64),
    DeathCause(DeathCause, f64),
    BiomarkerChange(String, f64),
}

/// Population simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationResults {
    pub id: Uuid,
    pub config: PopulationConfig,
    /// All life summaries
    pub lives: Vec<LifeSummary>,
    /// Discovered patterns
    pub patterns: Vec<CausalPattern>,
    /// Statistical summary
    pub statistics: PopulationStatistics,
    /// Intervention comparisons
    pub intervention_results: Vec<InterventionComparison>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStatistics {
    pub mean_lifespan: f64,
    pub median_lifespan: f64,
    pub std_lifespan: f64,
    pub centenarian_rate: f64,
    pub top_death_causes: Vec<(DeathCause, f64)>,
    pub gene_lifespan_correlations: HashMap<Gene, f64>,
    pub lifestyle_lifespan_correlation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionComparison {
    pub intervention: SimulatedIntervention,
    pub control_lifespan: f64,
    pub treatment_lifespan: f64,
    pub lifespan_delta: f64,
    pub disease_prevention: Vec<(DiseaseType, f64)>, // (disease, reduction %)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedIntervention {
    pub name: String,
    pub intervention_type: SimulatedInterventionType,
    pub start_age: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulatedInterventionType {
    CaloricRestriction,
    Rapamycin,
    Senolytic,
    NADBooster,
    Exercise,
    MetforminAnalog,
}

/// The Population Simulator
pub struct PopulationSimulator {
    config: PopulationConfig,
}

impl PopulationSimulator {
    pub fn new(config: PopulationConfig) -> Self {
        Self { config }
    }

    /// Run the full population simulation
    pub fn simulate(&self, rng: &mut impl Rng) -> PopulationResults {
        let mut lives = Vec::with_capacity(self.config.population_size);

        // Simulate each life
        for i in 0..self.config.population_size {
            let lifestyle = if self.config.lifestyle_variation {
                random_lifestyle(rng)
            } else {
                Lifestyle::default()
            };

            let summary = self.simulate_one_life(lifestyle, rng);
            lives.push(summary);

            // Progress indicator (every 10%)
            if i % (self.config.population_size / 10).max(1) == 0 {
                // In real implementation, would emit progress
            }
        }

        // Calculate statistics
        let statistics = self.calculate_statistics(&lives);

        // Mine patterns
        let patterns = self.mine_patterns(&lives);

        // Simulate interventions
        let intervention_results = if self.config.simulate_interventions {
            self.compare_interventions(rng)
        } else {
            Vec::new()
        };

        PopulationResults {
            id: Uuid::new_v4(),
            config: self.config.clone(),
            lives,
            patterns,
            statistics,
            intervention_results,
        }
    }

    fn simulate_one_life(&self, lifestyle: Lifestyle, rng: &mut impl Rng) -> LifeSummary {
        let mut organism = Organism::new_random(rng);
        organism.lifestyle = lifestyle;

        // Capture midlife biomarkers
        let mut midlife_biomarkers = None;

        // Simulate year by year
        while organism.alive && organism.age < 150.0 {
            organism.age_one_year(rng);

            // Capture at age 50
            if organism.age >= 50.0 && organism.age < 51.0 && midlife_biomarkers.is_none() {
                midlife_biomarkers = Some(MidlifeBiomarkers {
                    epigenetic_age: organism.genome.epigenome.calculate_horvath_age(),
                    inflammation: organism.systemic.inflammation,
                    senescence_burden: organism.organs.values()
                        .map(|o| o.senescent_fraction)
                        .sum::<f64>() / organism.organs.len() as f64,
                    mitochondrial_function: organism.genome.mtdna.respiratory_efficiency(),
                    telomere_length: organism.genome.shortest_telomere(),
                });
            }
        }

        // Extract variants
        let genetic_variants: Vec<_> = organism.genome.nuclear_genes.iter()
            .filter_map(|(gene, state)| {
                state.variants.first().map(|v| (*gene, v.effect, v.longevity_effect))
            })
            .collect();

        // Extract diseases
        let diseases: Vec<_> = organism.diseases.iter()
            .map(|d| (d.disease_type, d.onset_age))
            .collect();

        let death = organism.death.as_ref();
        let lifespan = organism.age;
        let death_cause = death.map(|d| d.cause.clone()).unwrap_or(DeathCause::Natural);
        let death_factors = death.map(|d| d.contributing_factors.clone()).unwrap_or_default();

        let biological_age = organism.biological_age();
        let final_age_acceleration = biological_age - lifespan;

        LifeSummary {
            id: organism.id,
            lifespan,
            death_cause,
            genetic_variants,
            lifestyle_score: organism.lifestyle.score(),
            diseases,
            midlife_biomarkers,
            death_factors,
            final_age_acceleration,
        }
    }

    fn calculate_statistics(&self, lives: &[LifeSummary]) -> PopulationStatistics {
        let lifespans: Vec<f64> = lives.iter().map(|l| l.lifespan).collect();
        let n = lifespans.len() as f64;

        let mean_lifespan = lifespans.iter().sum::<f64>() / n;

        let mut sorted = lifespans.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_lifespan = sorted[sorted.len() / 2];

        let variance = lifespans.iter()
            .map(|l| (l - mean_lifespan).powi(2))
            .sum::<f64>() / n;
        let std_lifespan = variance.sqrt();

        let centenarian_rate = lives.iter()
            .filter(|l| l.lifespan >= 100.0)
            .count() as f64 / n;

        // Count death causes
        let mut cause_counts: HashMap<String, usize> = HashMap::new();
        for life in lives {
            let cause_str = format!("{:?}", life.death_cause);
            *cause_counts.entry(cause_str).or_insert(0) += 1;
        }
        let mut top_causes: Vec<_> = cause_counts.into_iter()
            .map(|(k, v)| (k, v as f64 / n))
            .collect();
        top_causes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Gene correlations
        let gene_lifespan_correlations = self.calculate_gene_correlations(lives);

        // Lifestyle correlation
        let lifestyle_scores: Vec<f64> = lives.iter().map(|l| l.lifestyle_score).collect();
        let lifestyle_lifespan_correlation = pearson_correlation(&lifestyle_scores, &lifespans);

        PopulationStatistics {
            mean_lifespan,
            median_lifespan,
            std_lifespan,
            centenarian_rate,
            top_death_causes: top_causes.into_iter()
                .take(5)
                .map(|(k, v)| {
                    // Convert string back to DeathCause (simplified)
                    let cause = if k.contains("Natural") {
                        DeathCause::Natural
                    } else if k.contains("Cardiovascular") || k.contains("Heart") {
                        DeathCause::Cardiovascular
                    } else if k.contains("Cancer") {
                        DeathCause::Cancer
                    } else {
                        DeathCause::Frailty
                    };
                    (cause, v)
                })
                .collect(),
            gene_lifespan_correlations,
            lifestyle_lifespan_correlation,
        }
    }

    fn calculate_gene_correlations(&self, lives: &[LifeSummary]) -> HashMap<Gene, f64> {
        let mut correlations = HashMap::new();
        let lifespans: Vec<f64> = lives.iter().map(|l| l.lifespan).collect();

        for gene in Gene::all() {
            // Binary: has beneficial variant or not
            let has_variant: Vec<f64> = lives.iter()
                .map(|l| {
                    l.genetic_variants.iter()
                        .find(|(g, _, _)| *g == gene)
                        .map(|(_, _, effect)| if *effect > 0.0 { 1.0 } else { 0.0 })
                        .unwrap_or(0.0)
                })
                .collect();

            let corr = pearson_correlation(&has_variant, &lifespans);
            if corr.abs() > 0.01 { // Only significant correlations
                correlations.insert(gene, corr);
            }
        }

        correlations
    }

    fn mine_patterns(&self, lives: &[LifeSummary]) -> Vec<CausalPattern> {
        let mut patterns = Vec::new();

        // Pattern 1: Gene variants affecting lifespan
        patterns.extend(self.mine_gene_patterns(lives));

        // Pattern 2: Midlife biomarkers predicting death
        patterns.extend(self.mine_biomarker_patterns(lives));

        // Pattern 3: Lifestyle-lifespan patterns
        patterns.extend(self.mine_lifestyle_patterns(lives));

        // Pattern 4: Disease cascade patterns
        patterns.extend(self.mine_disease_patterns(lives));

        // Sort by strength
        patterns.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());

        patterns
    }

    fn mine_gene_patterns(&self, lives: &[LifeSummary]) -> Vec<CausalPattern> {
        let mut patterns = Vec::new();
        let mean_lifespan = lives.iter().map(|l| l.lifespan).sum::<f64>() / lives.len() as f64;

        for gene in Gene::all() {
            // Lives with beneficial variants in this gene
            let with_variant: Vec<&LifeSummary> = lives.iter()
                .filter(|l| l.genetic_variants.iter().any(|(g, e, _)|
                    *g == gene && matches!(e, VariantEffect::EnhancedFunction | VariantEffect::GainOfFunction)))
                .collect();

            if with_variant.len() >= 10 {
                let variant_mean = with_variant.iter().map(|l| l.lifespan).sum::<f64>()
                    / with_variant.len() as f64;
                let effect = variant_mean - mean_lifespan;

                if effect.abs() > 1.0 { // At least 1 year effect
                    patterns.push(CausalPattern {
                        id: Uuid::new_v4(),
                        cause: PatternCause::Gene(gene, VariantEffect::EnhancedFunction),
                        effect: PatternEffect::Lifespan(effect),
                        strength: effect.abs() / 10.0,
                        causal_confidence: 0.8, // Genetic is typically causal
                        supporting_lives: with_variant.len(),
                        temporal_gap_years: None,
                        effect_size: effect,
                        statistical_significance: 0.01, // Would calculate properly
                        description: format!(
                            "{:?} enhanced function: {:+.1} years lifespan",
                            gene, effect
                        ),
                    });
                }
            }
        }

        patterns
    }

    fn mine_biomarker_patterns(&self, lives: &[LifeSummary]) -> Vec<CausalPattern> {
        let mut patterns = Vec::new();

        // Lives with midlife data
        let with_midlife: Vec<_> = lives.iter()
            .filter_map(|l| l.midlife_biomarkers.as_ref().map(|b| (l, b)))
            .collect();

        if with_midlife.len() < 100 {
            return patterns;
        }

        // High inflammation at midlife → reduced lifespan
        let high_inflammation: Vec<_> = with_midlife.iter()
            .filter(|(_, b)| b.inflammation > 0.4)
            .collect();

        let low_inflammation: Vec<_> = with_midlife.iter()
            .filter(|(_, b)| b.inflammation <= 0.2)
            .collect();

        if !high_inflammation.is_empty() && !low_inflammation.is_empty() {
            let high_mean = high_inflammation.iter().map(|(l, _)| l.lifespan).sum::<f64>()
                / high_inflammation.len() as f64;
            let low_mean = low_inflammation.iter().map(|(l, _)| l.lifespan).sum::<f64>()
                / low_inflammation.len() as f64;
            let effect = low_mean - high_mean;

            patterns.push(CausalPattern {
                id: Uuid::new_v4(),
                cause: PatternCause::Biomarker("inflammation".to_string(), 0.4),
                effect: PatternEffect::Lifespan(-effect),
                strength: effect.abs() / 15.0,
                causal_confidence: 0.7,
                supporting_lives: high_inflammation.len() + low_inflammation.len(),
                temporal_gap_years: Some(30.0), // Midlife to death
                effect_size: effect,
                statistical_significance: 0.001,
                description: format!(
                    "High inflammation (>0.4) at age 50 predicts {:.1} years shorter life",
                    effect
                ),
            });
        }

        // High senescence burden at midlife
        let high_senescence: Vec<_> = with_midlife.iter()
            .filter(|(_, b)| b.senescence_burden > 0.15)
            .collect();

        if !high_senescence.is_empty() {
            let high_mean = high_senescence.iter().map(|(l, _)| l.lifespan).sum::<f64>()
                / high_senescence.len() as f64;
            let overall_mean = with_midlife.iter().map(|(l, _)| l.lifespan).sum::<f64>()
                / with_midlife.len() as f64;
            let effect = overall_mean - high_mean;

            patterns.push(CausalPattern {
                id: Uuid::new_v4(),
                cause: PatternCause::Biomarker("senescence_burden".to_string(), 0.15),
                effect: PatternEffect::Lifespan(-effect),
                strength: effect.abs() / 10.0,
                causal_confidence: 0.85,
                supporting_lives: high_senescence.len(),
                temporal_gap_years: Some(25.0),
                effect_size: effect,
                statistical_significance: 0.0001,
                description: format!(
                    "Senescent cell burden >15% at age 50: {:.1} years shorter life",
                    effect
                ),
            });
        }

        // Low mitochondrial function at midlife
        let low_mito: Vec<_> = with_midlife.iter()
            .filter(|(_, b)| b.mitochondrial_function < 0.6)
            .collect();

        if low_mito.len() >= 50 {
            let low_mean = low_mito.iter().map(|(l, _)| l.lifespan).sum::<f64>()
                / low_mito.len() as f64;
            let overall_mean = with_midlife.iter().map(|(l, _)| l.lifespan).sum::<f64>()
                / with_midlife.len() as f64;
            let effect = overall_mean - low_mean;

            patterns.push(CausalPattern {
                id: Uuid::new_v4(),
                cause: PatternCause::Biomarker("mitochondrial_function".to_string(), 0.6),
                effect: PatternEffect::Lifespan(-effect),
                strength: effect.abs() / 12.0,
                causal_confidence: 0.75,
                supporting_lives: low_mito.len(),
                temporal_gap_years: Some(20.0),
                effect_size: effect,
                statistical_significance: 0.001,
                description: format!(
                    "Mitochondrial dysfunction (<60%) at age 50: {:.1} years shorter life",
                    effect
                ),
            });
        }

        patterns
    }

    fn mine_lifestyle_patterns(&self, lives: &[LifeSummary]) -> Vec<CausalPattern> {
        let mut patterns = Vec::new();

        let high_lifestyle: Vec<_> = lives.iter()
            .filter(|l| l.lifestyle_score > 0.7)
            .collect();

        let low_lifestyle: Vec<_> = lives.iter()
            .filter(|l| l.lifestyle_score < 0.4)
            .collect();

        if !high_lifestyle.is_empty() && !low_lifestyle.is_empty() {
            let high_mean = high_lifestyle.iter().map(|l| l.lifespan).sum::<f64>()
                / high_lifestyle.len() as f64;
            let low_mean = low_lifestyle.iter().map(|l| l.lifespan).sum::<f64>()
                / low_lifestyle.len() as f64;
            let effect = high_mean - low_mean;

            patterns.push(CausalPattern {
                id: Uuid::new_v4(),
                cause: PatternCause::Lifestyle("optimal".to_string()),
                effect: PatternEffect::Lifespan(effect),
                strength: effect.abs() / 15.0,
                causal_confidence: 0.6, // Lifestyle may be correlated with other factors
                supporting_lives: high_lifestyle.len() + low_lifestyle.len(),
                temporal_gap_years: None,
                effect_size: effect,
                statistical_significance: 0.001,
                description: format!(
                    "Optimal lifestyle (score >0.7) vs poor (<0.4): {:+.1} years",
                    effect
                ),
            });
        }

        patterns
    }

    fn mine_disease_patterns(&self, lives: &[LifeSummary]) -> Vec<CausalPattern> {
        let mut patterns = Vec::new();

        // For each disease, check what preceded it
        let diseases = [
            DiseaseType::Atherosclerosis,
            DiseaseType::Type2Diabetes,
            DiseaseType::Cancer,
            DiseaseType::Alzheimers,
        ];

        for disease in &diseases {
            let with_disease: Vec<_> = lives.iter()
                .filter(|l| l.diseases.iter().any(|(d, _)| d == disease))
                .collect();

            if with_disease.len() >= 20 {
                // Average age of onset
                let avg_onset: f64 = with_disease.iter()
                    .filter_map(|l| l.diseases.iter().find(|(d, _)| d == disease).map(|(_, age)| *age))
                    .sum::<f64>() / with_disease.len() as f64;

                // Average lifespan with disease
                let avg_lifespan = with_disease.iter().map(|l| l.lifespan).sum::<f64>()
                    / with_disease.len() as f64;

                // Average lifespan without
                let without_disease: Vec<_> = lives.iter()
                    .filter(|l| !l.diseases.iter().any(|(d, _)| d == disease))
                    .collect();

                if !without_disease.is_empty() {
                    let avg_without = without_disease.iter().map(|l| l.lifespan).sum::<f64>()
                        / without_disease.len() as f64;
                    let effect = avg_without - avg_lifespan;

                    patterns.push(CausalPattern {
                        id: Uuid::new_v4(),
                        cause: PatternCause::Pathway(AgingRole::Inflammation), // Simplified
                        effect: PatternEffect::DiseaseRisk(*disease, with_disease.len() as f64 / lives.len() as f64),
                        strength: effect.abs() / 10.0,
                        causal_confidence: 0.7,
                        supporting_lives: with_disease.len(),
                        temporal_gap_years: Some(avg_lifespan - avg_onset),
                        effect_size: effect,
                        statistical_significance: 0.01,
                        description: format!(
                            "{:?}: avg onset {:.0}y, reduces lifespan by {:.1} years",
                            disease, avg_onset, effect
                        ),
                    });
                }
            }
        }

        patterns
    }

    fn compare_interventions(&self, rng: &mut impl Rng) -> Vec<InterventionComparison> {
        let mut comparisons = Vec::new();
        let sample_size = (self.config.population_size / 10).max(100);

        let interventions = [
            (SimulatedInterventionType::CaloricRestriction, "Caloric Restriction (20%)"),
            (SimulatedInterventionType::Rapamycin, "Rapamycin"),
            (SimulatedInterventionType::Senolytic, "Senolytic (D+Q)"),
            (SimulatedInterventionType::NADBooster, "NAD+ Booster (NMN)"),
            (SimulatedInterventionType::Exercise, "Regular Exercise"),
        ];

        for (intervention_type, name) in &interventions {
            let mut control_lifespans = Vec::new();
            let mut treatment_lifespans = Vec::new();

            // Simulate pairs (control vs treatment with same genome)
            for _ in 0..sample_size {
                let genome = Genome::new_random(rng);

                // Control (standard lifestyle)
                let mut control = Organism::with_genome(genome.clone(), Lifestyle::default());
                control.simulate_life(rng);
                control_lifespans.push(control.age);

                // Treatment (intervention applied)
                let mut treatment = Organism::with_genome(genome, Lifestyle::default());
                apply_intervention(&mut treatment, *intervention_type);
                treatment.simulate_life(rng);
                treatment_lifespans.push(treatment.age);
            }

            let control_mean = control_lifespans.iter().sum::<f64>() / control_lifespans.len() as f64;
            let treatment_mean = treatment_lifespans.iter().sum::<f64>() / treatment_lifespans.len() as f64;

            comparisons.push(InterventionComparison {
                intervention: SimulatedIntervention {
                    name: name.to_string(),
                    intervention_type: *intervention_type,
                    start_age: 40.0,
                },
                control_lifespan: control_mean,
                treatment_lifespan: treatment_mean,
                lifespan_delta: treatment_mean - control_mean,
                disease_prevention: Vec::new(), // Would calculate
            });
        }

        // Sort by effect
        comparisons.sort_by(|a, b| b.lifespan_delta.partial_cmp(&a.lifespan_delta).unwrap());

        comparisons
    }
}

/// Apply a simulated intervention to an organism
fn apply_intervention(organism: &mut Organism, intervention: SimulatedInterventionType) {
    match intervention {
        SimulatedInterventionType::CaloricRestriction => {
            organism.lifestyle.caloric_intake = 0.75;
            organism.lifestyle.diet_quality = 0.9;
        }
        SimulatedInterventionType::Rapamycin => {
            // Simulate mTOR inhibition effects
            organism.systemic.mtor_activity = 0.5;
            organism.systemic.ampk_activity = 0.8;
        }
        SimulatedInterventionType::Senolytic => {
            // Reduce senescent burden
            for state in organism.organs.values_mut() {
                state.senescent_fraction *= 0.3; // 70% clearance
            }
            organism.systemic.sasp_level *= 0.3;
            organism.systemic.inflammation *= 0.7;
        }
        SimulatedInterventionType::NADBooster => {
            organism.systemic.nad_level = 0.9;
        }
        SimulatedInterventionType::Exercise => {
            organism.lifestyle.exercise_hours = 5.0;
        }
        SimulatedInterventionType::MetforminAnalog => {
            organism.systemic.ampk_activity = 0.7;
            organism.systemic.insulin_sensitivity = 0.9;
        }
    }
}

/// Generate random lifestyle with variation
fn random_lifestyle(rng: &mut impl Rng) -> Lifestyle {
    Lifestyle {
        caloric_intake: rng.gen_range(0.8..1.4),
        diet_quality: rng.gen_range(0.2..0.95),
        exercise_hours: rng.gen_range(0.0..10.0),
        sleep_quality: rng.gen_range(0.3..0.95),
        stress: rng.gen_range(0.1..0.8),
        smoking: if rng.gen::<f64>() < 0.2 { rng.gen_range(0..30) } else { 0 },
        alcohol: rng.gen_range(0..30),
        sun_exposure: rng.gen_range(0.1..0.8),
        pollution: rng.gen_range(0.0..0.6),
        social: rng.gen_range(0.2..0.9),
    }
}

/// Calculate Pearson correlation
fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0.0;
    }

    let mean_x: f64 = x.iter().take(n).sum::<f64>() / n as f64;
    let mean_y: f64 = y.iter().take(n).sum::<f64>() / n as f64;

    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;

    for i in 0..n {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    if var_x == 0.0 || var_y == 0.0 {
        return 0.0;
    }

    cov / (var_x.sqrt() * var_y.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_simulation() {
        let mut rng = rand::thread_rng();
        let config = PopulationConfig {
            population_size: 100, // Small for testing
            simulate_interventions: false,
            ..Default::default()
        };

        let simulator = PopulationSimulator::new(config);
        let results = simulator.simulate(&mut rng);

        assert_eq!(results.lives.len(), 100);
        assert!(results.statistics.mean_lifespan > 0.0);
        assert!(!results.patterns.is_empty());
    }

    #[test]
    fn test_intervention_comparison() {
        let mut rng = rand::thread_rng();
        let config = PopulationConfig {
            population_size: 50,
            simulate_interventions: true,
            ..Default::default()
        };

        let simulator = PopulationSimulator::new(config);
        let results = simulator.simulate(&mut rng);

        assert!(!results.intervention_results.is_empty());
    }

    #[test]
    fn test_pattern_mining() {
        let mut rng = rand::thread_rng();
        let config = PopulationConfig {
            population_size: 200,
            ..Default::default()
        };

        let simulator = PopulationSimulator::new(config);
        let results = simulator.simulate(&mut rng);

        // Should find some patterns
        assert!(!results.patterns.is_empty());

        // Patterns should have descriptions
        for pattern in &results.patterns {
            assert!(!pattern.description.is_empty());
        }
    }
}
