//! Cell Module - The Fundamental Unit of Aging
//!
//! Simulates cellular processes that drive aging: damage, repair,
//! senescence, apoptosis, and division. Each cell contains molecular
//! machinery that degrades over time.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────────┐
//!  │                          CELL SIMULATION                                │
//!  ├─────────────────────────────────────────────────────────────────────────┤
//!  │                                                                         │
//!  │  ┌─────────────────────────────────────────────────────────────────┐   │
//!  │  │                        NUCLEUS                                   │   │
//!  │  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │   │
//!  │  │  │ DNA          │    │ Transcription│    │ DNA Repair   │      │   │
//!  │  │  │ (genome ref) │───►│ Machinery    │    │ Machinery    │      │   │
//!  │  │  │              │    │              │    │              │      │   │
//!  │  │  │ Damage       │◄───│ mRNA output  │    │ Error-prone  │      │   │
//!  │  │  │ Accumulates  │    │              │    │ with age     │      │   │
//!  │  │  └──────────────┘    └──────────────┘    └──────────────┘      │   │
//!  │  └─────────────────────────────────────────────────────────────────┘   │
//!  │                                    │                                    │
//!  │                                    ▼                                    │
//!  │  ┌─────────────────────────────────────────────────────────────────┐   │
//!  │  │                        CYTOPLASM                                 │   │
//!  │  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │   │
//!  │  │  │ Ribosomes    │    │ Proteasome   │    │ Lysosomes    │      │   │
//!  │  │  │ (Translation)│    │ (Degradation)│    │ (Autophagy)  │      │   │
//!  │  │  │              │    │              │    │              │      │   │
//!  │  │  │ Error rate ↑ │    │ Efficiency ↓ │    │ Function ↓   │      │   │
//!  │  │  └──────────────┘    └──────────────┘    └──────────────┘      │   │
//!  │  │                                                                  │   │
//!  │  │  ┌──────────────┐    ┌──────────────┐                           │   │
//!  │  │  │ Mitochondria │    │ Protein      │                           │   │
//!  │  │  │ (Energy)     │    │ Aggregates   │                           │   │
//!  │  │  │              │    │              │                           │   │
//!  │  │  │ ATP ↓, ROS ↑ │    │ Accumulate   │                           │   │
//!  │  │  └──────────────┘    └──────────────┘                           │   │
//!  │  └─────────────────────────────────────────────────────────────────┘   │
//!  │                                                                         │
//!  │  CELL FATE DECISIONS:                                                   │
//!  │  • Divide (if telomeres OK, no damage checkpoints)                     │
//!  │  • Senesce (if damage threshold exceeded)                              │
//!  │  • Die (apoptosis if severely damaged, or necrosis)                    │
//!  │  • Transform (if tumor suppressor fails → cancer)                      │
//!  │                                                                         │
//!  └─────────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use crate::genome::{Genome, Gene, Tissue, MutationType, SomaticMutation};

/// A simulated cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: Uuid,
    /// Reference to the genome (shared across cells, but cells can have unique mutations)
    pub genome_id: Uuid,
    /// Cell-specific somatic mutations (beyond genome baseline)
    pub local_mutations: Vec<SomaticMutation>,
    /// Cell type
    pub cell_type: CellType,
    /// Tissue this cell belongs to
    pub tissue: Tissue,
    /// Current cell cycle phase
    pub cycle_phase: CellCyclePhase,
    /// Number of divisions this cell has undergone
    pub division_count: u32,
    /// Replicative capacity remaining (Hayflick limit ~50-70)
    pub replicative_capacity: u32,
    /// Current telomere length (shortens with each division)
    pub telomere_length: u32,
    /// Cellular damage state
    pub damage: CellularDamage,
    /// Molecular machinery state
    pub machinery: MolecularMachinery,
    /// Is this cell senescent?
    pub senescent: bool,
    /// SASP factors being secreted (if senescent)
    pub sasp_output: f64,
    /// Cell age in years
    pub age: f64,
    /// Is this cell alive?
    pub alive: bool,
    /// Death cause if dead
    pub death_cause: Option<CellDeathCause>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellType {
    Epithelial,
    Fibroblast,
    Endothelial,
    Neuron,
    Cardiomyocyte,
    Hepatocyte,
    Adipocyte,
    StemCell,
    ImmuneCell,
    Myocyte,
}

impl CellType {
    /// Division rate (divisions per year for actively dividing cells)
    pub fn division_rate(&self) -> f64 {
        match self {
            CellType::Epithelial => 365.0,    // Daily turnover (gut, skin)
            CellType::Fibroblast => 12.0,     // Monthly
            CellType::Endothelial => 52.0,    // Weekly
            CellType::Neuron => 0.0,          // Post-mitotic
            CellType::Cardiomyocyte => 0.01,  // Rarely divide
            CellType::Hepatocyte => 1.0,      // Yearly, unless damage
            CellType::Adipocyte => 0.1,       // Very slow
            CellType::StemCell => 52.0,       // Weekly (tissue dependent)
            CellType::ImmuneCell => 365.0,    // High turnover
            CellType::Myocyte => 0.001,       // Almost never
        }
    }

    /// Is this a post-mitotic cell type?
    pub fn post_mitotic(&self) -> bool {
        matches!(self, CellType::Neuron | CellType::Cardiomyocyte | CellType::Myocyte)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellCyclePhase {
    G0,     // Quiescent
    G1,     // Growth 1
    S,      // DNA synthesis
    G2,     // Growth 2
    M,      // Mitosis
    Senescent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellDeathCause {
    Apoptosis,      // Programmed death
    Necrosis,       // Damage-induced death
    Autophagy,      // Self-consumption
    Ferroptosis,    // Iron-dependent lipid peroxidation
    Pyroptosis,     // Inflammatory death
    Senolysis,      // Senescent cell cleared by immune
}

/// Cellular damage accumulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularDamage {
    /// DNA damage (double-strand breaks, lesions)
    pub dna_damage: f64,
    /// Oxidative damage (lipid peroxidation, protein oxidation)
    pub oxidative_damage: f64,
    /// Protein aggregates (misfolded proteins)
    pub protein_aggregates: f64,
    /// Lipofuscin accumulation (undegradable waste)
    pub lipofuscin: f64,
    /// Membrane damage
    pub membrane_damage: f64,
    /// ER stress level
    pub er_stress: f64,
}

impl Default for CellularDamage {
    fn default() -> Self {
        Self {
            dna_damage: 0.0,
            oxidative_damage: 0.0,
            protein_aggregates: 0.0,
            lipofuscin: 0.0,
            membrane_damage: 0.0,
            er_stress: 0.0,
        }
    }
}

impl CellularDamage {
    /// Total damage score
    pub fn total(&self) -> f64 {
        (self.dna_damage + self.oxidative_damage + self.protein_aggregates +
         self.lipofuscin + self.membrane_damage + self.er_stress) / 6.0
    }

    /// Should this cell undergo apoptosis?
    pub fn should_apoptose(&self) -> bool {
        self.dna_damage > 0.8 || self.total() > 0.7
    }

    /// Should this cell senesce?
    pub fn should_senesce(&self) -> bool {
        self.dna_damage > 0.4 || self.oxidative_damage > 0.5 || self.total() > 0.5
    }
}

/// Molecular machinery within the cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularMachinery {
    /// DNA repair efficiency (0-1)
    pub dna_repair: f64,
    /// Proteasome activity (protein degradation)
    pub proteasome: f64,
    /// Autophagy capacity
    pub autophagy: f64,
    /// Mitochondrial function
    pub mitochondria: MitochondrialState,
    /// Ribosome translation fidelity
    pub translation_fidelity: f64,
    /// Chaperone activity (protein folding)
    pub chaperones: f64,
    /// Antioxidant capacity
    pub antioxidants: f64,
}

impl Default for MolecularMachinery {
    fn default() -> Self {
        Self {
            dna_repair: 1.0,
            proteasome: 1.0,
            autophagy: 1.0,
            mitochondria: MitochondrialState::default(),
            translation_fidelity: 1.0,
            chaperones: 1.0,
            antioxidants: 1.0,
        }
    }
}

/// Mitochondrial state within a cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialState {
    /// Number of mitochondria
    pub count: u32,
    /// ATP production capacity (0-1)
    pub atp_production: f64,
    /// ROS production (reactive oxygen species) (0-1, higher = more damage)
    pub ros_production: f64,
    /// Membrane potential
    pub membrane_potential: f64,
    /// Mitophagy efficiency (clearing damaged mitochondria)
    pub mitophagy: f64,
    /// Biogenesis rate (creating new mitochondria)
    pub biogenesis: f64,
}

impl Default for MitochondrialState {
    fn default() -> Self {
        Self {
            count: 1000,
            atp_production: 1.0,
            ros_production: 0.1,
            membrane_potential: 1.0,
            mitophagy: 1.0,
            biogenesis: 1.0,
        }
    }
}

impl MitochondrialState {
    /// Net energy available
    pub fn net_energy(&self) -> f64 {
        self.atp_production * (1.0 - self.ros_production * 0.3)
    }

    /// Age mitochondria by one time step
    pub fn age_step(&mut self, rng: &mut impl Rng) {
        // ROS increases
        self.ros_production += rng.gen::<f64>() * 0.001;
        self.ros_production = self.ros_production.min(1.0);

        // ATP production decreases due to mtDNA damage
        self.atp_production -= rng.gen::<f64>() * 0.0005 * self.ros_production;
        self.atp_production = self.atp_production.max(0.2);

        // Membrane potential declines
        self.membrane_potential -= rng.gen::<f64>() * 0.0003;
        self.membrane_potential = self.membrane_potential.max(0.3);

        // Mitophagy and biogenesis decline
        self.mitophagy -= rng.gen::<f64>() * 0.0002;
        self.biogenesis -= rng.gen::<f64>() * 0.0002;
        self.mitophagy = self.mitophagy.max(0.2);
        self.biogenesis = self.biogenesis.max(0.2);
    }
}

/// Environment affecting the cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellEnvironment {
    /// Oxygen level (hypoxia if low)
    pub oxygen: f64,
    /// Nutrient availability
    pub nutrients: f64,
    /// Growth factor signaling
    pub growth_factors: f64,
    /// Inflammatory cytokines in environment
    pub inflammatory_cytokines: f64,
    /// SASP factors from neighboring senescent cells
    pub sasp_exposure: f64,
    /// Oxidative stress level
    pub oxidative_stress: f64,
    /// Temperature (affects protein stability)
    pub temperature: f64,
    /// pH
    pub ph: f64,
}

impl Default for CellEnvironment {
    fn default() -> Self {
        Self {
            oxygen: 1.0,
            nutrients: 1.0,
            growth_factors: 0.5,
            inflammatory_cytokines: 0.1,
            sasp_exposure: 0.0,
            oxidative_stress: 0.1,
            temperature: 37.0,
            ph: 7.4,
        }
    }
}

/// Cell fate decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellFate {
    Continue,       // Keep living
    Divide,         // Undergo mitosis
    Senesce,        // Enter senescence
    Apoptosis,      // Programmed death
    Necrosis,       // Damage-induced death
    Transform,      // Become cancerous
}

impl Cell {
    /// Create a new cell
    pub fn new(
        genome_id: Uuid,
        cell_type: CellType,
        tissue: Tissue,
    ) -> Self {
        let replicative_capacity = if cell_type.post_mitotic() {
            0
        } else {
            50 + rand::thread_rng().gen_range(0..20) // Hayflick limit variation
        };

        Self {
            id: Uuid::new_v4(),
            genome_id,
            local_mutations: Vec::new(),
            cell_type,
            tissue,
            cycle_phase: if cell_type.post_mitotic() { CellCyclePhase::G0 } else { CellCyclePhase::G1 },
            division_count: 0,
            replicative_capacity,
            telomere_length: 12000,
            damage: CellularDamage::default(),
            machinery: MolecularMachinery::default(),
            senescent: false,
            sasp_output: 0.0,
            age: 0.0,
            alive: true,
            death_cause: None,
        }
    }

    /// Simulate one time step (e.g., one day)
    pub fn step(
        &mut self,
        genome: &Genome,
        environment: &CellEnvironment,
        dt_years: f64,
        rng: &mut impl Rng,
    ) -> CellFate {
        if !self.alive {
            return CellFate::Continue;
        }

        self.age += dt_years;

        // 1. Generate damage based on environment and metabolism
        self.generate_damage(genome, environment, dt_years, rng);

        // 2. Attempt repair
        self.attempt_repair(genome, dt_years, rng);

        // 3. Age molecular machinery
        self.age_machinery(genome, dt_years, rng);

        // 4. Make cell fate decision
        let fate = self.decide_fate(genome, environment, rng);

        // 5. Execute fate
        match fate {
            CellFate::Divide => self.divide(genome, rng),
            CellFate::Senesce => self.enter_senescence(),
            CellFate::Apoptosis => {
                self.alive = false;
                self.death_cause = Some(CellDeathCause::Apoptosis);
            }
            CellFate::Necrosis => {
                self.alive = false;
                self.death_cause = Some(CellDeathCause::Necrosis);
            }
            CellFate::Transform => {
                // Cancer transformation - mark but keep alive
                // In full simulation, this would spawn tumor cells
            }
            CellFate::Continue => {}
        }

        fate
    }

    fn generate_damage(
        &mut self,
        genome: &Genome,
        environment: &CellEnvironment,
        dt_years: f64,
        rng: &mut impl Rng,
    ) {
        let days = dt_years * 365.0;

        // DNA damage from endogenous sources (replication errors, ROS)
        let endogenous_dna_damage = 0.00001 * days * self.machinery.mitochondria.ros_production;

        // DNA damage from environment
        let exogenous_dna_damage = 0.00001 * days * environment.oxidative_stress;

        // Total DNA damage
        let repair_efficiency = genome.dna_repair_capacity() * self.machinery.dna_repair;
        let net_dna_damage = (endogenous_dna_damage + exogenous_dna_damage) * (1.0 - repair_efficiency * 0.9);
        self.damage.dna_damage += net_dna_damage * rng.gen::<f64>();
        self.damage.dna_damage = self.damage.dna_damage.min(1.0);

        // Oxidative damage
        let ros_damage = 0.00002 * days * self.machinery.mitochondria.ros_production;
        let antioxidant_protection = self.machinery.antioxidants * genome.gene_function(Gene::NFE2L2);
        self.damage.oxidative_damage += ros_damage * (1.0 - antioxidant_protection * 0.8) * rng.gen::<f64>();
        self.damage.oxidative_damage = self.damage.oxidative_damage.min(1.0);

        // Protein aggregate accumulation
        let misfolding_rate = 0.00001 * days * (1.0 - self.machinery.translation_fidelity);
        let clearance_rate = self.machinery.proteasome * self.machinery.autophagy;
        self.damage.protein_aggregates += (misfolding_rate - clearance_rate * 0.00001 * days) * rng.gen::<f64>();
        self.damage.protein_aggregates = self.damage.protein_aggregates.clamp(0.0, 1.0);

        // Lipofuscin (undegradable, always accumulates)
        self.damage.lipofuscin += 0.000005 * days * rng.gen::<f64>();
        self.damage.lipofuscin = self.damage.lipofuscin.min(1.0);

        // ER stress from misfolded proteins
        self.damage.er_stress = self.damage.protein_aggregates * 0.8;

        // SASP exposure causes damage
        if environment.sasp_exposure > 0.0 {
            self.damage.dna_damage += environment.sasp_exposure * 0.001 * days * rng.gen::<f64>();
            self.damage.oxidative_damage += environment.sasp_exposure * 0.002 * days * rng.gen::<f64>();
        }
    }

    fn attempt_repair(
        &mut self,
        genome: &Genome,
        dt_years: f64,
        rng: &mut impl Rng,
    ) {
        let days = dt_years * 365.0;

        // DNA repair
        let repair_capacity = genome.dna_repair_capacity() * self.machinery.dna_repair;
        self.damage.dna_damage -= repair_capacity * 0.001 * days * rng.gen::<f64>();
        self.damage.dna_damage = self.damage.dna_damage.max(0.0);

        // But repair is error-prone - sometimes introduces mutations
        if rng.gen::<f64>() < self.damage.dna_damage * 0.01 * days {
            self.local_mutations.push(SomaticMutation {
                id: Uuid::new_v4(),
                gene: None, // Random location
                mutation_type: MutationType::PointMutation,
                age_acquired: self.age,
                tissue_origin: self.tissue,
                clonal_expansion: 0.01,
                is_driver: rng.gen::<f64>() < 0.001, // Rare driver mutation
            });
        }

        // Protein clearance
        self.damage.protein_aggregates -= self.machinery.autophagy * 0.0005 * days * rng.gen::<f64>();
        self.damage.protein_aggregates = self.damage.protein_aggregates.max(0.0);

        // Oxidative damage repair
        self.damage.oxidative_damage -= self.machinery.antioxidants * 0.0005 * days * rng.gen::<f64>();
        self.damage.oxidative_damage = self.damage.oxidative_damage.max(0.0);
    }

    fn age_machinery(
        &mut self,
        genome: &Genome,
        dt_years: f64,
        rng: &mut impl Rng,
    ) {
        let years = dt_years;

        // DNA repair declines with age
        self.machinery.dna_repair -= 0.002 * years * rng.gen::<f64>();
        self.machinery.dna_repair = self.machinery.dna_repair.max(0.2);

        // Proteasome declines
        self.machinery.proteasome -= 0.003 * years * rng.gen::<f64>();
        self.machinery.proteasome = self.machinery.proteasome.max(0.2);

        // Autophagy declines
        self.machinery.autophagy -= 0.002 * years * rng.gen::<f64>();
        self.machinery.autophagy = self.machinery.autophagy.max(0.2);

        // Translation fidelity declines
        self.machinery.translation_fidelity -= 0.001 * years * rng.gen::<f64>();
        self.machinery.translation_fidelity = self.machinery.translation_fidelity.max(0.7);

        // Chaperones decline
        self.machinery.chaperones -= 0.002 * years * rng.gen::<f64>();
        self.machinery.chaperones = self.machinery.chaperones.max(0.3);

        // Antioxidants decline
        self.machinery.antioxidants -= 0.002 * years * rng.gen::<f64>();
        self.machinery.antioxidants = self.machinery.antioxidants.max(0.3);

        // Mitochondria age
        self.machinery.mitochondria.age_step(rng);

        // Senescent cells have impaired machinery
        if self.senescent {
            self.sasp_output += 0.01 * years;
            self.sasp_output = self.sasp_output.min(1.0);
        }
    }

    fn decide_fate(
        &mut self,
        genome: &Genome,
        environment: &CellEnvironment,
        rng: &mut impl Rng,
    ) -> CellFate {
        // Check for apoptosis (severe damage)
        if self.damage.should_apoptose() {
            let p53_function = genome.gene_function(Gene::TP53);
            if rng.gen::<f64>() < p53_function {
                return CellFate::Apoptosis;
            } else if rng.gen::<f64>() < 0.001 {
                // Failed apoptosis + high damage = potential transformation
                return CellFate::Transform;
            }
        }

        // Check for necrosis (catastrophic damage)
        if self.damage.total() > 0.9 {
            return CellFate::Necrosis;
        }

        // Check for senescence
        if !self.senescent {
            // Telomere-induced senescence
            if self.telomere_length < 5000 {
                return CellFate::Senesce;
            }

            // Damage-induced senescence
            if self.damage.should_senesce() {
                let senescence_propensity = genome.senescence_propensity();
                if rng.gen::<f64>() < senescence_propensity {
                    return CellFate::Senesce;
                }
            }

            // Replicative exhaustion
            if self.replicative_capacity == 0 && !self.cell_type.post_mitotic() {
                return CellFate::Senesce;
            }
        }

        // Check for division
        if !self.senescent && !self.cell_type.post_mitotic() && self.replicative_capacity > 0 {
            let division_rate = self.cell_type.division_rate();
            let dt = 1.0 / 365.0; // Assume daily steps
            let division_prob = division_rate * dt * environment.growth_factors;

            // Damage checkpoints prevent division
            let checkpoint_pass = self.damage.dna_damage < 0.2 && self.damage.total() < 0.3;

            if checkpoint_pass && rng.gen::<f64>() < division_prob {
                return CellFate::Divide;
            }
        }

        CellFate::Continue
    }

    fn divide(&mut self, genome: &Genome, rng: &mut impl Rng) {
        // Telomere shortening
        let shortening = rng.gen_range(50..200);
        self.telomere_length = self.telomere_length.saturating_sub(shortening);

        // Use replicative capacity
        self.replicative_capacity = self.replicative_capacity.saturating_sub(1);
        self.division_count += 1;

        // Replication can introduce errors
        if rng.gen::<f64>() < 0.0001 * (1.0 - genome.dna_repair_capacity()) {
            self.local_mutations.push(SomaticMutation {
                id: Uuid::new_v4(),
                gene: None,
                mutation_type: MutationType::PointMutation,
                age_acquired: self.age,
                tissue_origin: self.tissue,
                clonal_expansion: 0.01,
                is_driver: false,
            });
        }

        // Reset some damage (distributed to daughter)
        self.damage.protein_aggregates *= 0.5;
        self.damage.oxidative_damage *= 0.7;
    }

    fn enter_senescence(&mut self) {
        self.senescent = true;
        self.cycle_phase = CellCyclePhase::Senescent;
        self.sasp_output = 0.1; // Start secreting SASP

        // Senescent cells are metabolically active
        self.machinery.mitochondria.ros_production *= 1.5;
    }

    /// Get biological age based on damage and machinery state
    pub fn biological_age(&self) -> f64 {
        let damage_age = self.damage.total() * 100.0;
        let machinery_age = (1.0 - (
            self.machinery.dna_repair +
            self.machinery.proteasome +
            self.machinery.autophagy +
            self.machinery.translation_fidelity
        ) / 4.0) * 100.0;
        let telomere_age = (1.0 - self.telomere_length as f64 / 12000.0) * 100.0;

        (damage_age + machinery_age + telomere_age) / 3.0
    }
}

/// A population of cells (for tissue simulation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPopulation {
    pub tissue: Tissue,
    /// Total cell count (not simulating each individually)
    pub total_cells: u64,
    /// Sample of individually simulated cells
    pub sample_cells: Vec<Cell>,
    /// Fraction of cells that are senescent
    pub senescent_fraction: f64,
    /// Average damage level
    pub average_damage: f64,
    /// Stem cell count
    pub stem_cells: u64,
    /// Stem cell function
    pub stem_cell_function: f64,
}

impl CellPopulation {
    pub fn new(tissue: Tissue, total_cells: u64, sample_size: usize, genome_id: Uuid) -> Self {
        let cell_type = match tissue {
            Tissue::Skin | Tissue::Intestine => CellType::Epithelial,
            Tissue::Brain => CellType::Neuron,
            Tissue::Heart => CellType::Cardiomyocyte,
            Tissue::Liver => CellType::Hepatocyte,
            Tissue::Muscle => CellType::Myocyte,
            Tissue::Adipose => CellType::Adipocyte,
            Tissue::BoneMarrow => CellType::StemCell,
            Tissue::Immune => CellType::ImmuneCell,
            _ => CellType::Fibroblast,
        };

        let sample_cells: Vec<Cell> = (0..sample_size)
            .map(|_| Cell::new(genome_id, cell_type, tissue))
            .collect();

        Self {
            tissue,
            total_cells,
            sample_cells,
            senescent_fraction: 0.0,
            average_damage: 0.0,
            stem_cells: total_cells / 1000, // 0.1% are stem cells
            stem_cell_function: 1.0,
        }
    }

    /// Update population statistics from sample
    pub fn update_statistics(&mut self) {
        if self.sample_cells.is_empty() {
            return;
        }

        let alive: Vec<_> = self.sample_cells.iter().filter(|c| c.alive).collect();

        self.senescent_fraction = alive.iter().filter(|c| c.senescent).count() as f64
            / alive.len() as f64;

        self.average_damage = alive.iter().map(|c| c.damage.total()).sum::<f64>()
            / alive.len() as f64;
    }

    /// Get SASP output from senescent cells
    pub fn sasp_output(&self) -> f64 {
        self.sample_cells.iter()
            .filter(|c| c.alive && c.senescent)
            .map(|c| c.sasp_output)
            .sum::<f64>() / self.sample_cells.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new(Uuid::new_v4(), CellType::Fibroblast, Tissue::Skin);
        assert!(cell.alive);
        assert!(!cell.senescent);
        assert!(cell.replicative_capacity > 0);
    }

    #[test]
    fn test_cell_aging() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);
        let mut cell = Cell::new(genome.id, CellType::Fibroblast, Tissue::Skin);
        let env = CellEnvironment::default();

        // Age for 50 years
        for _ in 0..(50 * 365) {
            cell.step(&genome, &env, 1.0 / 365.0, &mut rng);
            if !cell.alive {
                break;
            }
        }

        // Cell should have accumulated damage
        assert!(cell.damage.total() > 0.0 || !cell.alive);
    }

    #[test]
    fn test_cell_senescence() {
        let mut cell = Cell::new(Uuid::new_v4(), CellType::Fibroblast, Tissue::Skin);

        // Force critically short telomeres
        cell.telomere_length = 4000;

        // Should senesce
        cell.enter_senescence();
        assert!(cell.senescent);
        assert!(cell.sasp_output > 0.0);
    }

    #[test]
    fn test_mitochondrial_aging() {
        let mut rng = rand::thread_rng();
        let mut mito = MitochondrialState::default();
        let initial_atp = mito.atp_production;

        for _ in 0..80 {
            mito.age_step(&mut rng);
        }

        assert!(mito.atp_production < initial_atp);
        assert!(mito.ros_production > 0.1);
    }
}
