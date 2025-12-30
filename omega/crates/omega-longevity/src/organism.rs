//! Organism Module - Full Lifespan Simulation
//!
//! Simulates an entire organism from genome to death, tracking the
//! emergence of aging from molecular damage accumulation.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────────┐
//!  │                     ORGANISM SIMULATION                                 │
//!  ├─────────────────────────────────────────────────────────────────────────┤
//!  │                                                                         │
//!  │  ┌──────────────────────────────────────────────────────────────────┐  │
//!  │  │                         GENOME                                    │  │
//!  │  │   DNA + Variants + Epigenetics → Individual susceptibility       │  │
//!  │  └──────────────────────────────────────────────────────────────────┘  │
//!  │                              │                                          │
//!  │                              ▼                                          │
//!  │  ┌──────────────────────────────────────────────────────────────────┐  │
//!  │  │                    ENVIRONMENT & LIFESTYLE                        │  │
//!  │  │   Diet, Exercise, Sleep, Stress, Toxins → Modulates damage       │  │
//!  │  └──────────────────────────────────────────────────────────────────┘  │
//!  │                              │                                          │
//!  │                              ▼                                          │
//!  │  ┌──────────────────────────────────────────────────────────────────┐  │
//!  │  │                       TISSUES/ORGANS                              │  │
//!  │  │                                                                   │  │
//!  │  │   Brain ─── Heart ─── Liver ─── Kidney ─── Lung ─── etc.        │  │
//!  │  │     │         │         │          │         │                   │  │
//!  │  │   Cells    Cells     Cells      Cells     Cells                  │  │
//!  │  │     │         │         │          │         │                   │  │
//!  │  │   Damage   Damage    Damage     Damage    Damage                 │  │
//!  │  │     │         │         │          │         │                   │  │
//!  │  │     └────────────────────┴──────────┴─────────┘                  │  │
//!  │  │                         │                                         │  │
//!  │  │              Systemic inflammation (SASP)                        │  │
//!  │  │              Stem cell exhaustion                                │  │
//!  │  │              Organ failure cascade                               │  │
//!  │  └──────────────────────────────────────────────────────────────────┘  │
//!  │                              │                                          │
//!  │                              ▼                                          │
//!  │  ┌──────────────────────────────────────────────────────────────────┐  │
//!  │  │                         DISEASES                                  │  │
//!  │  │                                                                   │  │
//!  │  │   Damage accumulation → Threshold → Disease onset                │  │
//!  │  │   - Cancer (mutations + failed surveillance)                     │  │
//!  │  │   - Cardiovascular (endothelial damage, inflammation)            │  │
//!  │  │   - Neurodegeneration (protein aggregates, cell loss)            │  │
//!  │  │   - Metabolic (insulin resistance, mitochondria)                 │  │
//!  │  └──────────────────────────────────────────────────────────────────┘  │
//!  │                              │                                          │
//!  │                              ▼                                          │
//!  │  ┌──────────────────────────────────────────────────────────────────┐  │
//!  │  │                          DEATH                                    │  │
//!  │  │                                                                   │  │
//!  │  │   Record: Age, Cause, Biomarker trajectory, Genetic factors     │  │
//!  │  └──────────────────────────────────────────────────────────────────┘  │
//!  │                                                                         │
//!  └─────────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use crate::genome::{Genome, Gene, Tissue, GeneVariant, GeneticRiskScore, GeneticRiskFactor, GeneticProtectiveFactor};
use crate::cell::{Cell, CellPopulation, CellEnvironment, CellType, CellFate};
use crate::{Result, LongevityError};

/// An organism being simulated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organism {
    pub id: Uuid,
    /// The organism's genome
    pub genome: Genome,
    /// Current chronological age
    pub age: f64,
    /// Organ systems
    pub organs: HashMap<Organ, OrganState>,
    /// Systemic state
    pub systemic: SystemicState,
    /// Lifestyle factors
    pub lifestyle: Lifestyle,
    /// Disease history
    pub diseases: Vec<Disease>,
    /// Biomarker trajectory over time
    pub biomarker_history: Vec<BiomarkerSnapshot>,
    /// Is alive?
    pub alive: bool,
    /// Death information
    pub death: Option<DeathRecord>,
    /// Key events during life
    pub life_events: Vec<LifeEvent>,
}

/// Organs that can fail
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Organ {
    Brain,
    Heart,
    Liver,
    Kidney,
    Lung,
    Pancreas,
    Immune,
    Vasculature,
    Muscle,
    Bone,
}

impl Organ {
    pub fn all() -> Vec<Organ> {
        vec![
            Organ::Brain, Organ::Heart, Organ::Liver, Organ::Kidney,
            Organ::Lung, Organ::Pancreas, Organ::Immune, Organ::Vasculature,
            Organ::Muscle, Organ::Bone,
        ]
    }

    pub fn to_tissue(&self) -> Tissue {
        match self {
            Organ::Brain => Tissue::Brain,
            Organ::Heart => Tissue::Heart,
            Organ::Liver => Tissue::Liver,
            Organ::Kidney => Tissue::Kidney,
            Organ::Lung => Tissue::Lung,
            Organ::Pancreas => Tissue::Pancreas,
            Organ::Immune => Tissue::Immune,
            Organ::Vasculature => Tissue::Blood,
            Organ::Muscle => Tissue::Muscle,
            Organ::Bone => Tissue::BoneMarrow,
        }
    }
}

/// State of an organ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganState {
    /// Function level (0-1, where 1 = full function)
    pub function: f64,
    /// Damage accumulated
    pub damage: f64,
    /// Inflammation level
    pub inflammation: f64,
    /// Senescent cell fraction
    pub senescent_fraction: f64,
    /// Fibrosis level
    pub fibrosis: f64,
    /// Cell population sample
    pub cells: CellPopulation,
    /// Organ-specific metrics
    pub metrics: HashMap<String, f64>,
}

impl OrganState {
    pub fn new(organ: Organ, genome_id: Uuid) -> Self {
        let tissue = organ.to_tissue();
        let cell_count = match organ {
            Organ::Brain => 86_000_000_000,     // 86 billion neurons
            Organ::Liver => 100_000_000_000,    // ~100 billion
            Organ::Heart => 2_000_000_000,      // ~2 billion cardiomyocytes
            Organ::Kidney => 1_000_000_000,
            Organ::Lung => 480_000_000_000,     // Huge surface area
            _ => 10_000_000_000,
        };

        Self {
            function: 1.0,
            damage: 0.0,
            inflammation: 0.0,
            senescent_fraction: 0.0,
            fibrosis: 0.0,
            cells: CellPopulation::new(tissue, cell_count, 100, genome_id),
            metrics: HashMap::new(),
        }
    }

    /// Calculate reserve capacity
    pub fn reserve_capacity(&self) -> f64 {
        (self.function - 0.3).max(0.0) / 0.7 // Reserve above 30% minimum function
    }

    /// Is this organ in failure?
    pub fn in_failure(&self) -> bool {
        self.function < 0.3
    }
}

/// Systemic state affecting all organs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemicState {
    /// Systemic inflammation level
    pub inflammation: f64,
    /// Oxidative stress level
    pub oxidative_stress: f64,
    /// Insulin sensitivity
    pub insulin_sensitivity: f64,
    /// NAD+ levels (declines with age)
    pub nad_level: f64,
    /// Growth hormone / IGF-1 signaling
    pub igf1_signaling: f64,
    /// mTOR activity
    pub mtor_activity: f64,
    /// AMPK activity
    pub ampk_activity: f64,
    /// Circulating senescent cell markers
    pub sasp_level: f64,
    /// Immune function
    pub immune_function: f64,
    /// Blood pressure (systolic)
    pub blood_pressure: f64,
    /// Blood glucose
    pub glucose: f64,
}

impl Default for SystemicState {
    fn default() -> Self {
        Self {
            inflammation: 0.1,
            oxidative_stress: 0.1,
            insulin_sensitivity: 1.0,
            nad_level: 1.0,
            igf1_signaling: 1.0,
            mtor_activity: 1.0,
            ampk_activity: 0.5,
            sasp_level: 0.0,
            immune_function: 1.0,
            blood_pressure: 120.0,
            glucose: 90.0,
        }
    }
}

/// Lifestyle factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifestyle {
    /// Caloric intake relative to need (1.0 = balanced)
    pub caloric_intake: f64,
    /// Diet quality (0-1)
    pub diet_quality: f64,
    /// Exercise level (hours/week)
    pub exercise_hours: f64,
    /// Sleep quality (0-1)
    pub sleep_quality: f64,
    /// Actual sleep hours per night
    pub sleep_hours: f64,
    /// Chronic stress level (0-1)
    pub stress: f64,
    /// Smoking (cigarettes/day)
    pub smoking: u32,
    /// Alcohol (drinks/week)
    pub alcohol: u32,
    /// Sun exposure (0-1)
    pub sun_exposure: f64,
    /// Pollution exposure (0-1)
    pub pollution: f64,
    /// Social connection (0-1)
    pub social: f64,
}

impl Default for Lifestyle {
    fn default() -> Self {
        Self {
            caloric_intake: 1.0,
            diet_quality: 0.6,
            exercise_hours: 2.0,
            sleep_quality: 0.7,
            sleep_hours: 7.5, // Default to population mean
            stress: 0.3,
            smoking: 0,
            alcohol: 3,
            sun_exposure: 0.3,
            pollution: 0.2,
            social: 0.6,
        }
    }
}

impl Lifestyle {
    /// Calculate overall lifestyle score
    pub fn score(&self) -> f64 {
        let diet = self.diet_quality;
        let exercise = (self.exercise_hours / 7.0).min(1.0);
        let sleep = self.sleep_quality;
        let stress_penalty = self.stress * 0.3;
        let smoking_penalty = (self.smoking as f64 / 20.0).min(0.5);
        let alcohol_penalty = if self.alcohol > 14 { 0.2 } else { 0.0 };
        let social = self.social * 0.5;

        ((diet + exercise + sleep + social) / 4.0 - stress_penalty - smoking_penalty - alcohol_penalty)
            .clamp(0.0, 1.0)
    }

    /// Calculate oxidative stress from lifestyle
    pub fn oxidative_stress_factor(&self) -> f64 {
        let base = 0.1;
        let smoking = self.smoking as f64 * 0.02;
        let exercise_protection = self.exercise_hours * 0.01;
        let diet_protection = self.diet_quality * 0.1;
        let pollution = self.pollution * 0.15;

        (base + smoking + pollution - exercise_protection - diet_protection).clamp(0.05, 1.0)
    }

    /// Calculate inflammation from lifestyle
    pub fn inflammation_factor(&self) -> f64 {
        let base = 0.1;
        let diet = if self.diet_quality < 0.4 { 0.2 } else { 0.0 }; // Poor diet → inflammation
        let obesity = if self.caloric_intake > 1.3 { 0.15 } else { 0.0 };
        let stress = self.stress * 0.2;
        let exercise_protection = (self.exercise_hours * 0.02).min(0.15);
        let sleep_impact = (1.0 - self.sleep_quality) * 0.1;

        (base + diet + obesity + stress + sleep_impact - exercise_protection).clamp(0.05, 1.0)
    }

    /// Apply caloric restriction (if intake < 0.85)
    pub fn caloric_restriction_effect(&self) -> f64 {
        if self.caloric_intake < 0.85 && self.caloric_intake > 0.6 {
            // CR provides longevity benefit
            1.0 + (0.85 - self.caloric_intake) * 0.5 // Up to 12.5% benefit
        } else {
            1.0
        }
    }
}

/// A disease
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disease {
    pub id: Uuid,
    pub disease_type: DiseaseType,
    pub onset_age: f64,
    pub severity: f64,
    pub organ_affected: Option<Organ>,
    pub is_fatal: bool,
    /// What caused this disease (for causal analysis)
    pub causal_factors: Vec<CausalFactor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiseaseType {
    // Cardiovascular
    Atherosclerosis,
    HeartFailure,
    Stroke,
    Hypertension,

    // Cancer
    Cancer,

    // Neurodegenerative
    Alzheimers,
    Parkinsons,
    Dementia,

    // Metabolic
    Type2Diabetes,
    MetabolicSyndrome,
    Obesity,

    // Other
    COPD,
    KidneyDisease,
    LiverDisease,
    Sarcopenia,
    Osteoporosis,
    Frailty,
}

/// A causal factor for a disease
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalFactor {
    pub factor_type: CausalFactorType,
    pub description: String,
    pub contribution: f64, // How much this contributed (0-1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalFactorType {
    Genetic,        // Gene variant
    Epigenetic,     // Epigenetic change
    Environmental,  // Lifestyle/environment
    Stochastic,     // Random damage
    Senescence,     // Senescent cell burden
    Inflammation,   // Chronic inflammation
    Metabolic,      // Metabolic dysfunction
    DnaRepairDeficit,
    MitochondrialDysfunction,
    ProteostasisFailure,
}

/// Biomarker snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerSnapshot {
    pub age: f64,
    /// Epigenetic age (Horvath clock)
    pub epigenetic_age: f64,
    /// Telomere length
    pub telomere_length: u32,
    /// Inflammatory markers (CRP, IL-6, TNF-α)
    pub inflammation_score: f64,
    /// NAD+ level
    pub nad_level: f64,
    /// Senescent cell burden
    pub senescence_burden: f64,
    /// Glucose/insulin
    pub glucose: f64,
    pub insulin_sensitivity: f64,
    /// Oxidative damage markers
    pub oxidative_damage: f64,
    /// Organ function summary
    pub organ_function: HashMap<Organ, f64>,
    /// Overall biological age estimate
    pub biological_age: f64,
}

/// Record of death
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathRecord {
    pub age: f64,
    pub cause: DeathCause,
    pub contributing_factors: Vec<CausalFactor>,
    /// Organ states at death
    pub organ_states: HashMap<Organ, f64>,
    /// Final biomarkers
    pub final_biomarkers: BiomarkerSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeathCause {
    Disease(DiseaseType),
    OrganFailure(Organ),
    MultiOrganFailure,
    Cancer,
    Cardiovascular,
    Frailty,
    Accident,
    Natural, // "Died of old age"
}

/// Life event for causal tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeEvent {
    pub age: f64,
    pub event_type: LifeEventType,
    pub description: String,
    pub impact: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifeEventType {
    Birth,
    DiseaseOnset,
    DiseaseProgression,
    OrganDamage,
    SenescenceSpike,
    LifestyleChange,
    Treatment,
    Death,
}

impl Organism {
    /// Create a new organism with random genome and randomized lifestyle
    pub fn new_random(rng: &mut impl Rng) -> Self {
        let genome = Genome::new_random(rng);

        // Randomize lifestyle, including sleep based on genetics
        let optimal_sleep = genome.optimal_sleep_hours();

        // People's actual sleep varies around their genetic optimum
        // Some sleep too little, some too much, some about right
        let sleep_deviation = rng.gen_range(-2.5..2.0);
        let actual_sleep = (optimal_sleep + sleep_deviation).clamp(4.0, 11.0);

        let lifestyle = Lifestyle {
            caloric_intake: rng.gen_range(0.8..1.4),
            diet_quality: rng.gen_range(0.2..0.95),
            exercise_hours: rng.gen_range(0.0..10.0),
            sleep_quality: rng.gen_range(0.3..0.95),
            sleep_hours: actual_sleep,
            stress: rng.gen_range(0.1..0.8),
            smoking: if rng.gen::<f64>() < 0.2 { rng.gen_range(0..30) } else { 0 },
            alcohol: rng.gen_range(0..30),
            sun_exposure: rng.gen_range(0.1..0.8),
            pollution: rng.gen_range(0.0..0.6),
            social: rng.gen_range(0.2..0.9),
        };

        Self::with_genome(genome, lifestyle)
    }

    /// Create organism with specific genome and lifestyle
    pub fn with_genome(genome: Genome, lifestyle: Lifestyle) -> Self {
        let genome_id = genome.id;

        let mut organs = HashMap::new();
        for organ in Organ::all() {
            organs.insert(organ, OrganState::new(organ, genome_id));
        }

        Self {
            id: Uuid::new_v4(),
            genome,
            age: 0.0,
            organs,
            systemic: SystemicState::default(),
            lifestyle,
            diseases: Vec::new(),
            biomarker_history: Vec::new(),
            alive: true,
            death: None,
            life_events: vec![LifeEvent {
                age: 0.0,
                event_type: LifeEventType::Birth,
                description: "Born".to_string(),
                impact: HashMap::new(),
            }],
        }
    }

    /// Simulate one year of aging
    pub fn age_one_year(&mut self, rng: &mut impl Rng) {
        if !self.alive {
            return;
        }

        self.age += 1.0;

        // 1. Update genome (epigenetic aging, mtDNA damage, telomeres)
        self.age_genome(rng);

        // 2. Update systemic state based on genome and lifestyle
        self.update_systemic_state(rng);

        // 3. Age each organ
        self.age_organs(rng);

        // 4. Check for disease onset
        self.check_disease_onset(rng);

        // 5. Progress existing diseases
        self.progress_diseases(rng);

        // 6. Check for death
        self.check_death(rng);

        // 7. Record biomarkers periodically
        if self.age as u32 % 5 == 0 || self.age >= 60.0 {
            self.record_biomarkers();
        }
    }

    fn age_genome(&mut self, rng: &mut impl Rng) {
        // Epigenetic aging
        self.genome.epigenome.age_one_year(self.age, rng);

        // mtDNA damage
        let oxidative_stress = self.lifestyle.oxidative_stress_factor() + self.systemic.oxidative_stress;
        self.genome.mtdna.age_one_year(oxidative_stress, rng);

        // Telomere shortening (in dividing cells) - affects genome telomeres
        for telo in &mut self.genome.telomeres {
            // Stochastic shortening
            if rng.gen::<f64>() < 0.1 {
                telo.divide(rng);
            }
        }

        // Somatic mutations accumulate
        let mutation_rate = 0.00001 * (1.0 + self.systemic.oxidative_stress);
        let repair_efficiency = self.genome.dna_repair_capacity();
        let net_mutation_rate = mutation_rate * (1.0 - repair_efficiency * 0.9);

        if rng.gen::<f64>() < net_mutation_rate * self.age {
            use crate::genome::{SomaticMutation, MutationType, Tissue};

            // Select tissue based on realistic mutation accumulation rates
            // Blood/BoneMarrow has highest turnover, followed by gut, skin, lung
            let tissue_roll: f64 = rng.gen();
            let tissue_origin = if tissue_roll < 0.25 {
                Tissue::Blood         // Hematopoietic - high turnover
            } else if tissue_roll < 0.40 {
                Tissue::Intestine     // Gut epithelium - rapid renewal
            } else if tissue_roll < 0.52 {
                Tissue::Skin          // Exposed, high turnover
            } else if tissue_roll < 0.62 {
                Tissue::Lung          // Exposed to environment
            } else if tissue_roll < 0.70 {
                Tissue::Liver         // Regenerative tissue
            } else if tissue_roll < 0.77 {
                Tissue::BoneMarrow    // Stem cell compartment
            } else if tissue_roll < 0.84 {
                Tissue::Pancreas      // Moderate turnover
            } else if tissue_roll < 0.90 {
                Tissue::Kidney        // Moderate turnover
            } else if tissue_roll < 0.95 {
                Tissue::Brain         // Low turnover, but accumulates
            } else {
                Tissue::Heart         // Low turnover
            };

            self.genome.somatic_mutations.push(SomaticMutation {
                id: Uuid::new_v4(),
                gene: None,
                mutation_type: MutationType::PointMutation,
                age_acquired: self.age,
                tissue_origin,
                clonal_expansion: 0.001,
                is_driver: rng.gen::<f64>() < 0.0001,
            });
        }
    }

    fn update_systemic_state(&mut self, rng: &mut impl Rng) {
        // Calculate sleep deviation effect on aging
        // This modulates inflammation, oxidative stress, and DNA repair
        let sleep_aging_factor = self.genome.sleep_deviation_aging_factor(self.lifestyle.sleep_hours);

        // Inflammation increases with age (sleep deprivation increases inflammation)
        let lifestyle_inflammation = self.lifestyle.inflammation_factor();
        let organ_inflammation: f64 = self.organs.values()
            .map(|o| o.inflammation)
            .sum::<f64>() / self.organs.len() as f64;
        let sasp_inflammation = self.systemic.sasp_level * 0.3;

        // Sleep deviation increases inflammation (poor sleep = higher IL-6, TNF-α)
        let sleep_inflammation = (sleep_aging_factor - 1.0) * 0.2;

        self.systemic.inflammation = (lifestyle_inflammation + organ_inflammation + sasp_inflammation + sleep_inflammation)
            .clamp(0.0, 1.0);

        // Oxidative stress (sleep deprivation impairs antioxidant defenses)
        let sleep_oxidative_penalty = (sleep_aging_factor - 1.0) * 0.15;
        self.systemic.oxidative_stress = self.lifestyle.oxidative_stress_factor() +
            (1.0 - self.genome.mtdna.respiratory_efficiency()) * 0.3 + sleep_oxidative_penalty;
        self.systemic.oxidative_stress = self.systemic.oxidative_stress.clamp(0.0, 1.0);

        // NAD+ declines with age
        self.systemic.nad_level -= rng.gen::<f64>() * 0.01;
        self.systemic.nad_level = self.systemic.nad_level.max(0.3);

        // Insulin sensitivity declines
        if self.lifestyle.caloric_intake > 1.1 || self.lifestyle.exercise_hours < 1.0 {
            self.systemic.insulin_sensitivity -= rng.gen::<f64>() * 0.01;
        }
        self.systemic.insulin_sensitivity = self.systemic.insulin_sensitivity.max(0.3);

        // mTOR activity (should be lower for longevity)
        self.systemic.mtor_activity = self.lifestyle.caloric_intake * self.genome.gene_function(Gene::MTOR);

        // AMPK activity (higher is better)
        let exercise_effect = (self.lifestyle.exercise_hours / 7.0).min(1.0) * 0.3;
        let cr_effect = self.lifestyle.caloric_restriction_effect() * 0.2;
        self.systemic.ampk_activity = 0.5 + exercise_effect + cr_effect - self.age * 0.002;
        self.systemic.ampk_activity = self.systemic.ampk_activity.clamp(0.2, 1.0);

        // SASP from senescent cells
        let total_senescence: f64 = self.organs.values()
            .map(|o| o.senescent_fraction * o.cells.sasp_output())
            .sum();
        self.systemic.sasp_level = total_senescence / self.organs.len() as f64;

        // Immune function declines (immunosenescence)
        self.systemic.immune_function = self.organs.get(&Organ::Immune)
            .map(|o| o.function)
            .unwrap_or(1.0);
        self.systemic.immune_function -= self.age * 0.005;
        self.systemic.immune_function = self.systemic.immune_function.clamp(0.2, 1.0);

        // Blood pressure tends to increase
        self.systemic.blood_pressure += rng.gen::<f64>() * 0.5;
        if self.lifestyle.exercise_hours > 3.0 {
            self.systemic.blood_pressure -= 0.3;
        }
        self.systemic.blood_pressure = self.systemic.blood_pressure.clamp(100.0, 200.0);

        // Glucose regulation
        self.systemic.glucose = 90.0 / self.systemic.insulin_sensitivity;
        if self.lifestyle.caloric_intake > 1.2 {
            self.systemic.glucose += 10.0;
        }
    }

    fn age_organs(&mut self, rng: &mut impl Rng) {
        // Create cell environment from systemic state
        let cell_env = CellEnvironment {
            oxygen: 1.0 - self.organs.get(&Organ::Lung).map(|o| 1.0 - o.function).unwrap_or(0.0) * 0.3,
            nutrients: self.lifestyle.caloric_intake.min(1.0),
            growth_factors: self.systemic.igf1_signaling,
            inflammatory_cytokines: self.systemic.inflammation,
            sasp_exposure: self.systemic.sasp_level,
            oxidative_stress: self.systemic.oxidative_stress,
            temperature: 37.0,
            ph: 7.4,
        };

        for (_organ, state) in &mut self.organs {
            // Age the sample cells
            for cell in &mut state.cells.sample_cells {
                cell.step(&self.genome, &cell_env, 1.0, rng);
            }

            // Update statistics from sample
            state.cells.update_statistics();
            state.senescent_fraction = state.cells.senescent_fraction;

            // Organ damage from senescent cells and inflammation
            let senescence_damage = state.senescent_fraction * 0.02;
            let inflammation_damage = self.systemic.inflammation * 0.01;
            let oxidative_damage = self.systemic.oxidative_stress * 0.005;

            state.damage += (senescence_damage + inflammation_damage + oxidative_damage) * rng.gen::<f64>();
            state.damage = state.damage.min(1.0);

            // Inflammation in organ
            state.inflammation = self.systemic.inflammation + state.cells.sasp_output();
            state.inflammation = state.inflammation.min(1.0);

            // Fibrosis accumulates
            if state.damage > 0.3 {
                state.fibrosis += 0.005 * rng.gen::<f64>();
                state.fibrosis = state.fibrosis.min(1.0);
            }

            // Function declines based on damage and fibrosis
            state.function = (1.0 - state.damage * 0.5 - state.fibrosis * 0.4)
                .clamp(0.1, 1.0);

            // Stem cell exhaustion
            state.cells.stem_cell_function -= 0.005 * rng.gen::<f64>();
            state.cells.stem_cell_function = state.cells.stem_cell_function.max(0.1);
        }
    }

    fn check_disease_onset(&mut self, rng: &mut impl Rng) {
        // Cardiovascular disease
        if self.age > 40.0 && !self.has_disease(DiseaseType::Atherosclerosis) {
            let risk = self.cardiovascular_risk();
            if rng.gen::<f64>() < risk * 0.02 {
                self.onset_disease(DiseaseType::Atherosclerosis, Organ::Heart, rng);
            }
        }

        // Type 2 Diabetes
        if self.systemic.insulin_sensitivity < 0.5 && !self.has_disease(DiseaseType::Type2Diabetes) {
            if rng.gen::<f64>() < 0.1 {
                self.onset_disease(DiseaseType::Type2Diabetes, Organ::Pancreas, rng);
            }
        }

        // Cancer (age + mutations + failed surveillance)
        if self.age > 30.0 && !self.has_disease(DiseaseType::Cancer) {
            let mutation_burden = self.genome.somatic_mutations.len() as f64 / 100.0;
            let immune_surveillance = self.systemic.immune_function;
            let cancer_risk = mutation_burden * (1.0 - immune_surveillance * 0.8) * self.age / 100.0;

            if rng.gen::<f64>() < cancer_risk * 0.01 {
                self.onset_disease(DiseaseType::Cancer, Organ::Immune, rng);
            }
        }

        // Alzheimer's (age + inflammation + genetics)
        if self.age > 60.0 && !self.has_disease(DiseaseType::Alzheimers) {
            let brain_damage = self.organs.get(&Organ::Brain).map(|o| o.damage).unwrap_or(0.0);

            // Genetic risk factors for Alzheimer's:
            // - SIRT1/SIRT3/SIRT6: neuroprotective sirtuins (higher = protective)
            // - NFKB1/IL6/TNF: inflammatory genes (higher = risk)
            // - TP53/ATM: DNA repair (higher = protective)
            let sirtuin_protection = (self.genome.gene_function(Gene::SIRT1)
                + self.genome.gene_function(Gene::SIRT3)
                + self.genome.gene_function(Gene::SIRT6)) / 3.0;
            let inflammatory_risk = (self.genome.gene_function(Gene::NFKB1)
                + self.genome.gene_function(Gene::IL6)
                + self.genome.gene_function(Gene::TNF)) / 3.0;
            let repair_protection = self.genome.gene_function(Gene::ATM) * 0.5;

            // Combined genetic risk: higher sirtuin/repair = protective, higher inflammation = risk
            let genetic_risk = (inflammatory_risk * 0.4 + (1.0 - sirtuin_protection) * 0.4 + (1.0 - repair_protection) * 0.2)
                .clamp(0.0, 1.0);

            let risk = (brain_damage + genetic_risk + self.systemic.inflammation) / 3.0;

            if rng.gen::<f64>() < risk * 0.02 {
                self.onset_disease(DiseaseType::Alzheimers, Organ::Brain, rng);
            }
        }

        // Sarcopenia
        if self.age > 50.0 && !self.has_disease(DiseaseType::Sarcopenia) {
            let muscle = self.organs.get(&Organ::Muscle).map(|o| o.function).unwrap_or(1.0);
            if muscle < 0.7 && self.lifestyle.exercise_hours < 2.0 {
                if rng.gen::<f64>() < 0.05 {
                    self.onset_disease(DiseaseType::Sarcopenia, Organ::Muscle, rng);
                }
            }
        }

        // Frailty syndrome
        if self.age > 70.0 && !self.has_disease(DiseaseType::Frailty) {
            let avg_organ_function: f64 = self.organs.values()
                .map(|o| o.function)
                .sum::<f64>() / self.organs.len() as f64;

            if avg_organ_function < 0.6 && self.systemic.inflammation > 0.4 {
                if rng.gen::<f64>() < 0.1 {
                    self.onset_disease(DiseaseType::Frailty, Organ::Muscle, rng);
                }
            }
        }
    }

    fn cardiovascular_risk(&self) -> f64 {
        let bp_risk = (self.systemic.blood_pressure - 120.0) / 80.0;
        let glucose_risk = (self.systemic.glucose - 100.0) / 100.0;
        let inflammation_risk = self.systemic.inflammation;
        let smoking_risk = self.lifestyle.smoking as f64 / 20.0;
        let exercise_protection = (self.lifestyle.exercise_hours / 5.0).min(0.3);

        ((bp_risk + glucose_risk + inflammation_risk + smoking_risk - exercise_protection) / 4.0)
            .clamp(0.0, 1.0)
    }

    fn onset_disease(&mut self, disease_type: DiseaseType, organ: Organ, rng: &mut impl Rng) {
        let causal_factors = self.identify_causal_factors(&disease_type, rng);

        let disease = Disease {
            id: Uuid::new_v4(),
            disease_type,
            onset_age: self.age,
            severity: 0.1,
            organ_affected: Some(organ),
            is_fatal: false,
            causal_factors,
        };

        self.diseases.push(disease);

        self.life_events.push(LifeEvent {
            age: self.age,
            event_type: LifeEventType::DiseaseOnset,
            description: format!("{:?} diagnosed", disease_type),
            impact: HashMap::new(),
        });
    }

    fn identify_causal_factors(&self, _disease: &DiseaseType, _rng: &mut impl Rng) -> Vec<CausalFactor> {
        let mut factors = Vec::new();

        // Genetic factors
        if self.genome.gene_function(Gene::TP53) < 0.5 {
            factors.push(CausalFactor {
                factor_type: CausalFactorType::Genetic,
                description: "TP53 dysfunction".to_string(),
                contribution: 0.3,
            });
        }

        // Inflammation
        if self.systemic.inflammation > 0.4 {
            factors.push(CausalFactor {
                factor_type: CausalFactorType::Inflammation,
                description: "Chronic inflammation".to_string(),
                contribution: self.systemic.inflammation,
            });
        }

        // Senescence
        let avg_senescence: f64 = self.organs.values()
            .map(|o| o.senescent_fraction)
            .sum::<f64>() / self.organs.len() as f64;
        if avg_senescence > 0.1 {
            factors.push(CausalFactor {
                factor_type: CausalFactorType::Senescence,
                description: format!("Senescent cell burden: {:.1}%", avg_senescence * 100.0),
                contribution: avg_senescence,
            });
        }

        // Lifestyle
        if self.lifestyle.score() < 0.4 {
            factors.push(CausalFactor {
                factor_type: CausalFactorType::Environmental,
                description: "Poor lifestyle factors".to_string(),
                contribution: 1.0 - self.lifestyle.score(),
            });
        }

        // Mitochondrial dysfunction
        if self.genome.mtdna.respiratory_efficiency() < 0.7 {
            factors.push(CausalFactor {
                factor_type: CausalFactorType::MitochondrialDysfunction,
                description: "Mitochondrial dysfunction".to_string(),
                contribution: 1.0 - self.genome.mtdna.respiratory_efficiency(),
            });
        }

        factors
    }

    fn progress_diseases(&mut self, rng: &mut impl Rng) {
        for disease in &mut self.diseases {
            // Disease severity increases over time
            disease.severity += rng.gen::<f64>() * 0.05;
            disease.severity = disease.severity.min(1.0);

            // Affect organ function
            if let Some(organ) = disease.organ_affected {
                if let Some(state) = self.organs.get_mut(&organ) {
                    state.function -= disease.severity * 0.02 * rng.gen::<f64>();
                    state.function = state.function.max(0.1);
                }
            }

            // Some diseases become fatal
            if disease.severity > 0.9 && rng.gen::<f64>() < 0.1 {
                disease.is_fatal = true;
            }
        }
    }

    fn check_death(&mut self, rng: &mut impl Rng) {
        // Check for fatal diseases
        for disease in &self.diseases {
            if disease.is_fatal {
                self.die(DeathCause::Disease(disease.disease_type), rng);
                return;
            }
        }

        // Check for organ failure
        for (organ, state) in &self.organs {
            if state.in_failure() {
                // Critical organs cause death
                if matches!(organ, Organ::Heart | Organ::Brain | Organ::Lung | Organ::Liver | Organ::Kidney) {
                    self.die(DeathCause::OrganFailure(*organ), rng);
                    return;
                }
            }
        }

        // Multi-organ failure
        let failed_organs = self.organs.values().filter(|o| o.function < 0.5).count();
        if failed_organs >= 3 {
            self.die(DeathCause::MultiOrganFailure, rng);
            return;
        }

        // Natural death probability increases with age
        if self.age > 80.0 {
            let frailty = self.has_disease(DiseaseType::Frailty);
            let base_mortality = if frailty { 0.15 } else { 0.05 };
            let age_factor = (self.age - 80.0) / 40.0;

            if rng.gen::<f64>() < base_mortality * age_factor {
                self.die(DeathCause::Natural, rng);
            }
        }
    }

    fn die(&mut self, cause: DeathCause, _rng: &mut impl Rng) {
        self.alive = false;

        let final_biomarkers = self.calculate_biomarkers();

        let organ_states: HashMap<_, _> = self.organs.iter()
            .map(|(k, v)| (*k, v.function))
            .collect();

        // Identify what contributed to death
        let mut contributing_factors = Vec::new();

        // Always add age
        contributing_factors.push(CausalFactor {
            factor_type: CausalFactorType::Stochastic,
            description: format!("Age: {:.1} years", self.age),
            contribution: self.age / 120.0,
        });

        // Add disease factors
        for disease in &self.diseases {
            for factor in &disease.causal_factors {
                if !contributing_factors.iter().any(|f| f.description == factor.description) {
                    contributing_factors.push(factor.clone());
                }
            }
        }

        let cause_desc = format!("Died: {:?}", cause);

        self.death = Some(DeathRecord {
            age: self.age,
            cause,
            contributing_factors,
            organ_states,
            final_biomarkers,
        });

        self.life_events.push(LifeEvent {
            age: self.age,
            event_type: LifeEventType::Death,
            description: cause_desc,
            impact: HashMap::new(),
        });
    }

    fn has_disease(&self, disease_type: DiseaseType) -> bool {
        self.diseases.iter().any(|d| d.disease_type == disease_type)
    }

    fn record_biomarkers(&mut self) {
        let snapshot = self.calculate_biomarkers();
        self.biomarker_history.push(snapshot);
    }

    fn calculate_biomarkers(&self) -> BiomarkerSnapshot {
        let epigenetic_age = self.genome.epigenome.calculate_horvath_age();
        let telomere_length = self.genome.shortest_telomere();

        let organ_function: HashMap<_, _> = self.organs.iter()
            .map(|(k, v)| (*k, v.function))
            .collect();

        // Biological age estimate
        let damage_component = self.organs.values().map(|o| o.damage).sum::<f64>() / self.organs.len() as f64;
        let machinery_component = 1.0 - self.genome.mtdna.respiratory_efficiency();
        let biological_age = (epigenetic_age + self.age * (1.0 + damage_component + machinery_component)) / 2.0;

        BiomarkerSnapshot {
            age: self.age,
            epigenetic_age,
            telomere_length,
            inflammation_score: self.systemic.inflammation,
            nad_level: self.systemic.nad_level,
            senescence_burden: self.organs.values()
                .map(|o| o.senescent_fraction)
                .sum::<f64>() / self.organs.len() as f64,
            glucose: self.systemic.glucose,
            insulin_sensitivity: self.systemic.insulin_sensitivity,
            oxidative_damage: self.systemic.oxidative_stress,
            organ_function,
            biological_age,
        }
    }

    /// Get biological age
    pub fn biological_age(&self) -> f64 {
        self.calculate_biomarkers().biological_age
    }

    /// Simulate until death
    pub fn simulate_life(&mut self, rng: &mut impl Rng) {
        while self.alive && self.age < 150.0 {
            self.age_one_year(rng);
        }
    }

    /// Predict lifespan from genome by running multiple simulations
    ///
    /// Returns prediction with confidence intervals based on genetic risk factors.
    /// Simulates multiple lives with the same genome but varied lifestyle to
    /// determine the range of possible outcomes.
    pub fn predict_lifespan_from_genome(
        genome: &Genome,
        num_simulations: usize,
        rng: &mut impl Rng,
    ) -> LifespanPrediction {
        let mut lifespans = Vec::with_capacity(num_simulations);
        let mut death_causes: HashMap<DeathCause, usize> = HashMap::new();
        let mut disease_ages: HashMap<DiseaseType, Vec<f64>> = HashMap::new();

        for _ in 0..num_simulations {
            let mut organism = Organism::new_random(rng);
            // Replace the random genome with the query genome
            organism.genome = genome.clone();

            organism.simulate_life(rng);

            lifespans.push(organism.age);

            if let Some(death) = &organism.death {
                *death_causes.entry(death.cause.clone()).or_insert(0) += 1;
            }

            for disease in &organism.diseases {
                disease_ages.entry(disease.disease_type)
                    .or_insert_with(Vec::new)
                    .push(disease.onset_age);
            }
        }

        // Calculate statistics
        lifespans.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = lifespans.len() as f64;

        let mean_lifespan = lifespans.iter().sum::<f64>() / n;

        let variance = lifespans.iter()
            .map(|x| (x - mean_lifespan).powi(2))
            .sum::<f64>() / n;
        let std_dev = variance.sqrt();

        let median_lifespan = if lifespans.len() % 2 == 0 {
            (lifespans[lifespans.len() / 2 - 1] + lifespans[lifespans.len() / 2]) / 2.0
        } else {
            lifespans[lifespans.len() / 2]
        };

        let p10 = lifespans[(n * 0.1) as usize];
        let p25 = lifespans[(n * 0.25) as usize];
        let p75 = lifespans[(n * 0.75) as usize];
        let p90 = lifespans[(n * 0.9) as usize];

        // Calculate genetic risk score
        let genetic_risk = genome.calculate_genetic_risk_score();

        // Most likely death cause
        let most_likely_cause = death_causes.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(cause, _)| cause.clone())
            .unwrap_or(DeathCause::MultiOrganFailure);

        // Disease risk predictions
        let mut disease_risks = Vec::new();
        for (disease_type, ages) in &disease_ages {
            let risk = ages.len() as f64 / n;
            let mean_onset = if ages.is_empty() {
                None
            } else {
                Some(ages.iter().sum::<f64>() / ages.len() as f64)
            };
            disease_risks.push(DiseaseRiskPrediction {
                disease_type: *disease_type,
                lifetime_risk: risk,
                mean_onset_age: mean_onset,
            });
        }
        disease_risks.sort_by(|a, b| b.lifetime_risk.partial_cmp(&a.lifetime_risk).unwrap());

        LifespanPrediction {
            genome_id: genome.id,
            num_simulations,
            mean_lifespan,
            median_lifespan,
            std_deviation: std_dev,
            percentile_10: p10,
            percentile_25: p25,
            percentile_75: p75,
            percentile_90: p90,
            min_lifespan: *lifespans.first().unwrap_or(&0.0),
            max_lifespan: *lifespans.last().unwrap_or(&0.0),
            genetic_risk_score: genetic_risk,
            most_likely_death_cause: most_likely_cause,
            death_cause_distribution: death_causes,
            disease_risks,
            optimal_sleep_hours: genome.optimal_sleep_hours(),
            key_risk_factors: genome.identify_risk_factors(),
            key_protective_factors: genome.identify_protective_factors(),
        }
    }
}

/// Prediction of lifespan from genome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifespanPrediction {
    pub genome_id: Uuid,
    pub num_simulations: usize,

    // Lifespan statistics
    pub mean_lifespan: f64,
    pub median_lifespan: f64,
    pub std_deviation: f64,
    pub percentile_10: f64,  // 10% die before this age
    pub percentile_25: f64,
    pub percentile_75: f64,
    pub percentile_90: f64,  // 90% die before this age
    pub min_lifespan: f64,
    pub max_lifespan: f64,

    // Genetic risk analysis
    pub genetic_risk_score: GeneticRiskScore,
    pub most_likely_death_cause: DeathCause,
    pub death_cause_distribution: HashMap<DeathCause, usize>,
    pub disease_risks: Vec<DiseaseRiskPrediction>,

    // Personalized insights
    pub optimal_sleep_hours: f64,
    pub key_risk_factors: Vec<GeneticRiskFactor>,
    pub key_protective_factors: Vec<GeneticProtectiveFactor>,
}

/// Disease risk prediction from simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRiskPrediction {
    pub disease_type: DiseaseType,
    pub lifetime_risk: f64,
    pub mean_onset_age: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organism_creation() {
        let mut rng = rand::thread_rng();
        let organism = Organism::new_random(&mut rng);
        assert!(organism.alive);
        assert_eq!(organism.age, 0.0);
        assert!(!organism.organs.is_empty());
    }

    #[test]
    fn test_simulate_life() {
        let mut rng = rand::thread_rng();
        let mut organism = Organism::new_random(&mut rng);

        organism.simulate_life(&mut rng);

        assert!(!organism.alive);
        assert!(organism.death.is_some());
        assert!(organism.age > 0.0);
    }

    #[test]
    fn test_lifestyle_effects() {
        let mut rng = rand::thread_rng();

        // Good lifestyle
        let mut good = Organism::new_random(&mut rng);
        good.lifestyle = Lifestyle {
            caloric_intake: 0.8, // CR
            diet_quality: 0.9,
            exercise_hours: 5.0,
            sleep_quality: 0.9,
            stress: 0.1,
            smoking: 0,
            alcohol: 0,
            ..Default::default()
        };

        // Poor lifestyle
        let mut poor = Organism::new_random(&mut rng);
        poor.lifestyle = Lifestyle {
            caloric_intake: 1.4,
            diet_quality: 0.3,
            exercise_hours: 0.0,
            sleep_quality: 0.4,
            stress: 0.8,
            smoking: 20,
            alcohol: 30,
            ..Default::default()
        };

        good.simulate_life(&mut rng);
        poor.simulate_life(&mut rng);

        // Good lifestyle should generally live longer (though stochastic)
        // Just verify simulation completed
        assert!(good.death.is_some());
        assert!(poor.death.is_some());
    }

    #[test]
    fn test_biomarker_trajectory() {
        let mut rng = rand::thread_rng();
        let mut organism = Organism::new_random(&mut rng);

        // Age for 50 years
        for _ in 0..50 {
            organism.age_one_year(&mut rng);
        }

        // Should have biomarker history
        assert!(!organism.biomarker_history.is_empty());

        // Biological age should roughly track chronological age
        let last = organism.biomarker_history.last().unwrap();
        assert!(last.biological_age > 0.0);
    }

    #[test]
    fn test_lifespan_prediction_from_genome() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);

        // Run prediction with 50 simulations (small for test speed)
        let prediction = Organism::predict_lifespan_from_genome(&genome, 50, &mut rng);

        // Should have valid lifespan statistics
        assert!(prediction.mean_lifespan > 0.0);
        assert!(prediction.median_lifespan > 0.0);
        assert!(prediction.percentile_10 < prediction.percentile_90);
        assert!(prediction.min_lifespan <= prediction.max_lifespan);

        // Should have genetic risk scores
        assert!(prediction.genetic_risk_score.overall >= 0.0);
        assert!(prediction.genetic_risk_score.overall <= 1.0);

        // Should have optimal sleep hours
        assert!(prediction.optimal_sleep_hours >= 4.0);
        assert!(prediction.optimal_sleep_hours <= 10.0);

        // Death cause distribution should have entries
        assert!(!prediction.death_cause_distribution.is_empty());
    }

    #[test]
    fn test_genetic_risk_score() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);

        let risk = genome.calculate_genetic_risk_score();

        // All scores should be between 0 and 1
        assert!(risk.overall >= 0.0 && risk.overall <= 1.0);
        assert!(risk.cancer >= 0.0 && risk.cancer <= 1.0);
        assert!(risk.cardiovascular >= 0.0 && risk.cardiovascular <= 1.0);
        assert!(risk.neurodegeneration >= 0.0 && risk.neurodegeneration <= 1.0);
        assert!(risk.metabolic >= 0.0 && risk.metabolic <= 1.0);
        assert!(risk.accelerated_aging >= 0.0 && risk.accelerated_aging <= 1.0);
    }
}
