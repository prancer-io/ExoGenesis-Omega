//! Genome Module - The Foundation of Mechanistic Aging Simulation
//!
//! Models the genome as the blueprint from which all aging processes emerge.
//! Instead of applying known hallmarks top-down, we simulate molecular
//! mechanisms bottom-up and let aging patterns emerge.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────────┐
//!  │                    MECHANISTIC AGING SIMULATION                         │
//!  ├─────────────────────────────────────────────────────────────────────────┤
//!  │                                                                         │
//!  │  GENOME (DNA)          ENVIRONMENT           EMERGENCE                  │
//!  │  ┌──────────────┐      ┌──────────────┐      ┌──────────────┐          │
//!  │  │ 20,000 genes │      │ Diet         │      │ Aging is NOT │          │
//!  │  │ SNP variants │  ×   │ Toxins       │  →   │ programmed - │          │
//!  │  │ Epigenetics  │      │ Stress       │      │ it EMERGES   │          │
//!  │  │ mtDNA        │      │ Exercise     │      │ from damage  │          │
//!  │  └──────────────┘      └──────────────┘      └──────────────┘          │
//!  │         │                     │                     │                   │
//!  │         ▼                     ▼                     ▼                   │
//!  │  ┌─────────────────────────────────────────────────────────────┐       │
//!  │  │              MOLECULAR DYNAMICS OVER TIME                    │       │
//!  │  │                                                              │       │
//!  │  │  DNA Damage → Repair (imperfect) → Mutations accumulate     │       │
//!  │  │  Epigenetic drift → Gene expression changes                  │       │
//!  │  │  Protein misfolding → Aggregates → Proteostasis loss        │       │
//!  │  │  ROS → Mitochondrial damage → Energy decline                 │       │
//!  │  │  Telomere shortening → Senescence → SASP                    │       │
//!  │  │                                                              │       │
//!  │  └─────────────────────────────────────────────────────────────┘       │
//!  │         │                                                               │
//!  │         ▼                                                               │
//!  │  ┌─────────────────────────────────────────────────────────────┐       │
//!  │  │              TISSUE → ORGAN → ORGANISM FAILURE               │       │
//!  │  │                                                              │       │
//!  │  │  Simulate millions of lives → Discover causal patterns      │       │
//!  │  │  What molecular events PRECEDE disease/death?               │       │
//!  │  │  Which genes/variants accelerate or protect?                │       │
//!  │  │                                                              │       │
//!  │  └─────────────────────────────────────────────────────────────┘       │
//!  │                                                                         │
//!  └─────────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use rand::Rng;
use rand::seq::SliceRandom;

/// A simulated genome with longevity-relevant genes and variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub id: Uuid,
    /// Nuclear DNA - key genes affecting aging
    pub nuclear_genes: HashMap<Gene, GeneState>,
    /// Mitochondrial DNA - 37 genes, high mutation rate
    pub mtdna: MitochondrialDNA,
    /// Telomere lengths per chromosome (23 pairs = 46 chromosomes)
    pub telomeres: Vec<TelomereState>,
    /// Epigenetic state (DNA methylation patterns)
    pub epigenome: Epigenome,
    /// Accumulated mutations
    pub somatic_mutations: Vec<SomaticMutation>,
    /// Chromosomal abnormalities
    pub chromosomal_aberrations: Vec<ChromosomalAberration>,
}

/// Key genes that affect aging and longevity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gene {
    // DNA Repair
    TP53,       // Tumor suppressor, guardian of genome
    BRCA1,      // Double-strand break repair
    BRCA2,
    ATM,        // DNA damage response kinase
    WRN,        // Werner syndrome helicase (progeria)
    LMNA,       // Lamin A (Hutchinson-Gilford progeria)
    ERCC1,      // Nucleotide excision repair

    // Telomere Maintenance
    TERT,       // Telomerase reverse transcriptase
    TERC,       // Telomerase RNA component
    POT1,       // Shelterin complex

    // Nutrient Sensing / Metabolism
    MTOR,       // Mechanistic target of rapamycin
    IGF1R,      // Insulin/IGF-1 receptor
    FOXO3,      // Forkhead transcription factor (longevity)
    SIRT1,      // Sirtuin 1
    SIRT3,      // Sirtuin 3 (mitochondrial)
    SIRT6,      // Sirtuin 6 (DNA repair, metabolism)
    AMPK,       // AMP-activated protein kinase

    // Senescence
    CDKN2A,     // p16INK4a - senescence trigger
    CDKN1A,     // p21 - cell cycle arrest
    RB1,        // Retinoblastoma - tumor suppressor

    // Proteostasis
    HSF1,       // Heat shock factor
    HSP70,      // Heat shock protein 70
    HSP90,      // Heat shock protein 90
    SQSTM1,     // p62 - autophagy receptor

    // Mitochondrial Function
    PPARGC1A,   // PGC-1α - mitochondrial biogenesis
    PINK1,      // Mitophagy
    PRKN,       // Parkin (mitophagy)
    NFE2L2,     // Nrf2 - antioxidant response

    // Inflammation
    NFKB1,      // NF-κB
    NLRP3,      // Inflammasome
    IL6,        // Interleukin-6
    TNF,        // Tumor necrosis factor

    // Stem Cells
    NANOG,      // Pluripotency
    OCT4,       // Pluripotency
    KLF4,       // Yamanaka factor
    MYC,        // Yamanaka factor

    // Apoptosis
    BCL2,       // Anti-apoptotic
    BAX,        // Pro-apoptotic
    CASP3,      // Caspase 3

    // Circadian Rhythm / Sleep
    CLOCK,      // Core circadian transcription factor
    BMAL1,      // ARNTL - partner of CLOCK
    PER1,       // Period 1 - negative feedback
    PER2,       // Period 2 - critical for rhythm
    PER3,       // Period 3 - sleep duration/timing
    CRY1,       // Cryptochrome 1 - negative feedback
    CRY2,       // Cryptochrome 2
    DEC2,       // BHLHE41 - "short sleep" gene
    ADRB1,      // Adrenergic receptor - short sleep mutation
    ADA,        // Adenosine deaminase - sleep pressure
}

impl Gene {
    pub fn all() -> Vec<Gene> {
        vec![
            Gene::TP53, Gene::BRCA1, Gene::BRCA2, Gene::ATM, Gene::WRN, Gene::LMNA, Gene::ERCC1,
            Gene::TERT, Gene::TERC, Gene::POT1,
            Gene::MTOR, Gene::IGF1R, Gene::FOXO3, Gene::SIRT1, Gene::SIRT3, Gene::SIRT6, Gene::AMPK,
            Gene::CDKN2A, Gene::CDKN1A, Gene::RB1,
            Gene::HSF1, Gene::HSP70, Gene::HSP90, Gene::SQSTM1,
            Gene::PPARGC1A, Gene::PINK1, Gene::PRKN, Gene::NFE2L2,
            Gene::NFKB1, Gene::NLRP3, Gene::IL6, Gene::TNF,
            Gene::NANOG, Gene::OCT4, Gene::KLF4, Gene::MYC,
            Gene::BCL2, Gene::BAX, Gene::CASP3,
            Gene::CLOCK, Gene::BMAL1, Gene::PER1, Gene::PER2, Gene::PER3,
            Gene::CRY1, Gene::CRY2, Gene::DEC2, Gene::ADRB1, Gene::ADA,
        ]
    }

    /// Gene's role in aging
    pub fn aging_role(&self) -> AgingRole {
        match self {
            Gene::TP53 | Gene::BRCA1 | Gene::BRCA2 | Gene::ATM | Gene::ERCC1 => AgingRole::DNARepair,
            Gene::WRN | Gene::LMNA => AgingRole::Progeria,
            Gene::TERT | Gene::TERC | Gene::POT1 => AgingRole::TelomereMaintenance,
            Gene::MTOR | Gene::IGF1R | Gene::FOXO3 | Gene::AMPK => AgingRole::NutrientSensing,
            Gene::SIRT1 | Gene::SIRT3 | Gene::SIRT6 => AgingRole::Sirtuins,
            Gene::CDKN2A | Gene::CDKN1A | Gene::RB1 => AgingRole::Senescence,
            Gene::HSF1 | Gene::HSP70 | Gene::HSP90 | Gene::SQSTM1 => AgingRole::Proteostasis,
            Gene::PPARGC1A | Gene::PINK1 | Gene::PRKN | Gene::NFE2L2 => AgingRole::Mitochondrial,
            Gene::NFKB1 | Gene::NLRP3 | Gene::IL6 | Gene::TNF => AgingRole::Inflammation,
            Gene::NANOG | Gene::OCT4 | Gene::KLF4 | Gene::MYC => AgingRole::StemCell,
            Gene::BCL2 | Gene::BAX | Gene::CASP3 => AgingRole::Apoptosis,
            Gene::CLOCK | Gene::BMAL1 | Gene::PER1 | Gene::PER2 | Gene::PER3 |
            Gene::CRY1 | Gene::CRY2 | Gene::DEC2 | Gene::ADRB1 | Gene::ADA => AgingRole::CircadianRhythm,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgingRole {
    DNARepair,
    Progeria,
    TelomereMaintenance,
    NutrientSensing,
    Sirtuins,
    Senescence,
    Proteostasis,
    Mitochondrial,
    Inflammation,
    StemCell,
    Apoptosis,
    CircadianRhythm,
}

/// State of a gene (expression, variants, damage)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneState {
    /// Base expression level (0-1, 0.5 = normal)
    pub expression: f64,
    /// Germline variants (SNPs affecting function)
    pub variants: Vec<GeneVariant>,
    /// Somatic mutations accumulated
    pub mutations: Vec<GeneMutation>,
    /// Epigenetic modifications affecting this gene
    pub methylation_level: f64, // 0 = unmethylated, 1 = fully methylated
    /// Copy number (normally 2)
    pub copy_number: u8,
}

impl Default for GeneState {
    fn default() -> Self {
        Self {
            expression: 0.5,
            variants: Vec::new(),
            mutations: Vec::new(),
            methylation_level: 0.3, // Baseline methylation
            copy_number: 2,
        }
    }
}

/// A germline genetic variant (inherited)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneVariant {
    pub rsid: String,          // e.g., "rs429358" (APOE)
    pub effect: VariantEffect,
    pub allele_frequency: f64, // Population frequency
    pub longevity_effect: f64, // -1 to +1 (harmful to protective)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariantEffect {
    LossOfFunction,
    GainOfFunction,
    ReducedFunction,
    EnhancedFunction,
    Neutral,
}

/// A somatic mutation (acquired during life)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneMutation {
    pub mutation_type: MutationType,
    pub age_acquired: f64,
    pub clonal_fraction: f64, // Fraction of cells with this mutation
    pub driver: bool,         // Is this a driver mutation?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationType {
    PointMutation,
    Deletion,
    Insertion,
    Duplication,
    Translocation,
}

/// Somatic mutation anywhere in genome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomaticMutation {
    pub id: Uuid,
    pub gene: Option<Gene>,   // None if intergenic
    pub mutation_type: MutationType,
    pub age_acquired: f64,
    pub tissue_origin: Tissue,
    pub clonal_expansion: f64,
    pub is_driver: bool,
}

/// Chromosomal abnormality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromosomalAberration {
    pub aberration_type: AberrationType,
    pub chromosome: u8,
    pub age_acquired: f64,
    pub cell_fraction: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AberrationType {
    Aneuploidy,
    LossOfHeterozygosity,
    ChromosomalFusion,
    Inversion,
    LargeScaleDeletion,
}

/// Mitochondrial DNA state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialDNA {
    /// Copy number per cell (typically 100-10,000)
    pub copy_number: u32,
    /// Fraction of mtDNA with deletions (common deletion)
    pub deletion_fraction: f64,
    /// Fraction with point mutations
    pub mutation_fraction: f64,
    /// Heteroplasmy level (% mutant mtDNA)
    pub heteroplasmy: f64,
    /// Key gene states
    pub complex_i_function: f64,   // NADH dehydrogenase
    pub complex_iii_function: f64, // Cytochrome bc1
    pub complex_iv_function: f64,  // Cytochrome c oxidase
    pub complex_v_function: f64,   // ATP synthase
}

impl Default for MitochondrialDNA {
    fn default() -> Self {
        Self {
            copy_number: 1000,
            deletion_fraction: 0.0,
            mutation_fraction: 0.0,
            heteroplasmy: 0.0,
            complex_i_function: 1.0,
            complex_iii_function: 1.0,
            complex_iv_function: 1.0,
            complex_v_function: 1.0,
        }
    }
}

impl MitochondrialDNA {
    /// Overall respiratory chain efficiency
    pub fn respiratory_efficiency(&self) -> f64 {
        (self.complex_i_function * self.complex_iii_function *
         self.complex_iv_function * self.complex_v_function).sqrt().sqrt()
    }

    /// Apply time-dependent damage
    pub fn age_one_year(&mut self, oxidative_stress: f64, rng: &mut impl Rng) {
        // mtDNA has no histone protection, ~10x nuclear mutation rate
        let damage_rate = 0.002 * (1.0 + oxidative_stress);

        self.deletion_fraction += rng.gen::<f64>() * damage_rate;
        self.mutation_fraction += rng.gen::<f64>() * damage_rate * 0.5;
        self.heteroplasmy = self.deletion_fraction + self.mutation_fraction;

        // Clonal expansion of damaged mtDNA (replicative advantage)
        if self.heteroplasmy > 0.1 {
            self.heteroplasmy *= 1.0 + rng.gen::<f64>() * 0.05;
        }

        // Function decline when heteroplasmy exceeds threshold (~60%)
        if self.heteroplasmy > 0.6 {
            let decline = (self.heteroplasmy - 0.6) * 0.1;
            self.complex_i_function = (self.complex_i_function - decline).max(0.1);
            self.complex_iii_function = (self.complex_iii_function - decline * 0.8).max(0.1);
            self.complex_iv_function = (self.complex_iv_function - decline * 0.6).max(0.1);
            self.complex_v_function = (self.complex_v_function - decline * 0.4).max(0.1);
        }
    }
}

/// Telomere state for a chromosome
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TelomereState {
    /// Length in base pairs (newborn ~10-15kb, critical <5kb)
    pub length_bp: u32,
    /// Telomerase activity (0-1)
    pub telomerase_activity: f64,
    /// Is this chromosome's telomere critically short?
    pub critically_short: bool,
}

impl Default for TelomereState {
    fn default() -> Self {
        Self {
            length_bp: 12000, // ~12kb at birth
            telomerase_activity: 0.0, // Most somatic cells have none
            critically_short: false,
        }
    }
}

impl TelomereState {
    /// Shorten telomere by one cell division
    pub fn divide(&mut self, rng: &mut impl Rng) {
        // Lose 50-200 bp per division (end replication problem)
        let loss = rng.gen_range(50..200);

        if self.telomerase_activity > 0.0 {
            // Telomerase can add back some length
            let added = (self.telomerase_activity * 100.0) as u32;
            self.length_bp = self.length_bp.saturating_sub(loss).saturating_add(added);
        } else {
            self.length_bp = self.length_bp.saturating_sub(loss);
        }

        // Critical threshold ~5kb
        self.critically_short = self.length_bp < 5000;
    }

    /// Get biological age signal from telomere
    pub fn biological_age_signal(&self) -> f64 {
        // Map ~12kb (young) to ~6kb (old)
        let normalized = (12000.0 - self.length_bp as f64) / 6000.0;
        normalized.clamp(0.0, 1.0)
    }
}

/// Epigenome - DNA methylation and chromatin state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epigenome {
    /// Global methylation level (decreases with age)
    pub global_methylation: f64,
    /// CpG island methylation (increases with age at specific sites)
    pub cpg_island_methylation: HashMap<String, f64>,
    /// Horvath clock sites (353 CpGs)
    pub clock_sites: Vec<f64>,
    /// Epigenetic noise (entropy increases with age)
    pub epigenetic_noise: f64,
    /// Histone modifications
    pub histone_marks: HistoneState,
}

impl Default for Epigenome {
    fn default() -> Self {
        Self {
            global_methylation: 0.7, // Young = high global methylation
            cpg_island_methylation: HashMap::new(),
            clock_sites: vec![0.0; 353], // Horvath clock CpGs
            epigenetic_noise: 0.0,
            histone_marks: HistoneState::default(),
        }
    }
}

impl Epigenome {
    /// Calculate epigenetic (Horvath) age
    pub fn calculate_horvath_age(&self) -> f64 {
        // Simplified: average of clock sites maps to age
        let mean: f64 = self.clock_sites.iter().sum::<f64>() / self.clock_sites.len() as f64;
        mean * 120.0 // Scale to ~120 year range
    }

    /// Apply age-related epigenetic drift
    pub fn age_one_year(&mut self, _chronological_age: f64, rng: &mut impl Rng) {
        // Global hypomethylation
        self.global_methylation -= rng.gen::<f64>() * 0.002;
        self.global_methylation = self.global_methylation.max(0.3);

        // Clock site changes (predictable)
        for site in &mut self.clock_sites {
            *site += 1.0 / 120.0; // Linear increase toward 1.0
            *site += rng.gen::<f64>() * 0.005; // Stochastic component
            *site = site.clamp(0.0, 1.0);
        }

        // Epigenetic noise accumulation
        self.epigenetic_noise += rng.gen::<f64>() * 0.01;

        // Histone changes
        self.histone_marks.age_one_year(rng);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoneState {
    /// H3K4me3 (active promoters) - declines with age
    pub h3k4me3: f64,
    /// H3K27me3 (repressive) - redistributes with age
    pub h3k27me3: f64,
    /// H3K9me3 (heterochromatin) - loss with age
    pub h3k9me3: f64,
    /// H4K16ac (chromatin structure) - declines with age
    pub h4k16ac: f64,
}

impl Default for HistoneState {
    fn default() -> Self {
        Self {
            h3k4me3: 1.0,
            h3k27me3: 1.0,
            h3k9me3: 1.0,
            h4k16ac: 1.0,
        }
    }
}

impl HistoneState {
    pub fn age_one_year(&mut self, rng: &mut impl Rng) {
        self.h3k4me3 -= rng.gen::<f64>() * 0.003;
        self.h3k9me3 -= rng.gen::<f64>() * 0.004; // Heterochromatin loss
        self.h4k16ac -= rng.gen::<f64>() * 0.002;
        // H3K27me3 redistributes rather than simply declining
        self.h3k27me3 += rng.gen::<f64>() * 0.001 - 0.0005;

        // Clamp all values
        self.h3k4me3 = self.h3k4me3.clamp(0.3, 1.0);
        self.h3k27me3 = self.h3k27me3.clamp(0.3, 1.0);
        self.h3k9me3 = self.h3k9me3.clamp(0.2, 1.0);
        self.h4k16ac = self.h4k16ac.clamp(0.3, 1.0);
    }
}

/// Tissue types for tracking tissue-specific effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tissue {
    Blood,
    Brain,
    Heart,
    Liver,
    Kidney,
    Lung,
    Muscle,
    Skin,
    Adipose,
    Intestine,
    BoneMarrow,
    Pancreas,
    Immune,
}

impl Genome {
    /// Create a new random genome with inherited variants
    pub fn new_random(rng: &mut impl Rng) -> Self {
        let mut nuclear_genes = HashMap::new();

        for gene in Gene::all() {
            let mut state = GeneState::default();

            // Random inherited variants based on population frequencies
            if rng.gen::<f64>() < 0.3 { // 30% chance of carrying a variant
                state.variants.push(generate_random_variant(gene, rng));
            }

            nuclear_genes.insert(gene, state);
        }

        // Initialize telomeres with some variation (46 chromosomes)
        let mut telomeres: Vec<TelomereState> = (0..46)
            .map(|_| TelomereState {
                length_bp: rng.gen_range(10000..14000),
                ..TelomereState::default()
            })
            .collect();

        // Check for TERT/TERC variants that affect telomerase
        if let Some(tert) = nuclear_genes.get(&Gene::TERT) {
            if tert.variants.iter().any(|v| matches!(v.effect, VariantEffect::EnhancedFunction)) {
                for telo in &mut telomeres {
                    telo.telomerase_activity = 0.1; // Some telomerase activity
                }
            }
        }

        Self {
            id: Uuid::new_v4(),
            nuclear_genes,
            mtdna: MitochondrialDNA::default(),
            telomeres,
            epigenome: Epigenome::default(),
            somatic_mutations: Vec::new(),
            chromosomal_aberrations: Vec::new(),
        }
    }

    /// Get the effective function of a gene (considering variants, mutations, expression)
    pub fn gene_function(&self, gene: Gene) -> f64 {
        if let Some(state) = self.nuclear_genes.get(&gene) {
            let mut function = state.expression;

            // Apply variant effects
            for variant in &state.variants {
                match variant.effect {
                    VariantEffect::LossOfFunction => function *= 0.0,
                    VariantEffect::ReducedFunction => function *= 0.5 + variant.longevity_effect * 0.2,
                    VariantEffect::GainOfFunction => function *= 1.5,
                    VariantEffect::EnhancedFunction => function *= 1.2,
                    VariantEffect::Neutral => {}
                }
            }

            // Apply mutation effects
            for mutation in &state.mutations {
                if mutation.driver {
                    function *= 0.5; // Driver mutations typically impair function
                }
            }

            // Methylation silencing
            if state.methylation_level > 0.8 {
                function *= 0.3;
            }

            // Copy number effects
            function *= state.copy_number as f64 / 2.0;

            function.clamp(0.0, 2.0)
        } else {
            0.5 // Default
        }
    }

    /// Calculate DNA repair capacity
    pub fn dna_repair_capacity(&self) -> f64 {
        let repair_genes = [Gene::TP53, Gene::BRCA1, Gene::BRCA2, Gene::ATM, Gene::ERCC1];
        let total: f64 = repair_genes.iter()
            .map(|g| self.gene_function(*g))
            .sum();
        total / repair_genes.len() as f64
    }

    /// Calculate senescence propensity
    pub fn senescence_propensity(&self) -> f64 {
        // High p16/p21 expression + low telomerase = high senescence
        let p16 = self.gene_function(Gene::CDKN2A);
        let p21 = self.gene_function(Gene::CDKN1A);
        let tert = self.gene_function(Gene::TERT);

        (p16 + p21) / 2.0 * (1.0 - tert * 0.5)
    }

    /// Calculate proteostasis capacity
    pub fn proteostasis_capacity(&self) -> f64 {
        let proteo_genes = [Gene::HSF1, Gene::HSP70, Gene::HSP90, Gene::SQSTM1];
        let total: f64 = proteo_genes.iter()
            .map(|g| self.gene_function(*g))
            .sum();
        total / proteo_genes.len() as f64
    }

    /// Calculate inflammation tendency
    pub fn inflammation_tendency(&self) -> f64 {
        let pro_inflam = self.gene_function(Gene::NFKB1)
                       + self.gene_function(Gene::NLRP3)
                       + self.gene_function(Gene::IL6)
                       + self.gene_function(Gene::TNF);
        pro_inflam / 4.0
    }

    /// Get shortest telomere
    pub fn shortest_telomere(&self) -> u32 {
        self.telomeres.iter().map(|t| t.length_bp).min().unwrap_or(0)
    }

    /// Count critically short telomeres
    pub fn critically_short_telomeres(&self) -> usize {
        self.telomeres.iter().filter(|t| t.critically_short).count()
    }

    /// Overall genomic instability score
    pub fn genomic_instability(&self) -> f64 {
        let mutation_burden = self.somatic_mutations.len() as f64 / 1000.0;
        let aberration_burden = self.chromosomal_aberrations.len() as f64 / 10.0;
        let mtdna_damage = self.mtdna.heteroplasmy;
        let repair_deficit = 1.0 - self.dna_repair_capacity();

        (mutation_burden + aberration_burden + mtdna_damage + repair_deficit) / 4.0
    }

    /// Calculate genetically-determined optimal sleep duration (hours)
    ///
    /// Based on circadian rhythm gene variants:
    /// - DEC2/ADRB1 mutations can reduce need to 4-6 hours ("short sleep" phenotype)
    /// - PER3 variants affect sleep duration preference
    /// - ADA variants affect adenosine metabolism and sleep pressure
    /// - CLOCK variants affect circadian period length
    ///
    /// Returns the optimal sleep hours for this genome (typically 6-9 hours)
    pub fn optimal_sleep_hours(&self) -> f64 {
        let mut optimal: f64 = 7.5; // Population mean

        // DEC2 (BHLHE41) - "short sleep" gene
        // Loss-of-function variants allow healthy short sleep
        let dec2 = self.gene_function(Gene::DEC2);
        if dec2 < 0.5 {
            optimal -= 1.5; // Short sleep phenotype
        }

        // ADRB1 - another "short sleep" gene
        let adrb1 = self.gene_function(Gene::ADRB1);
        if adrb1 < 0.5 {
            optimal -= 1.0;
        }

        // PER3 - period gene affecting sleep duration
        // Variable number tandem repeat (VNTR) affects duration
        let per3 = self.gene_function(Gene::PER3);
        if per3 > 0.6 {
            optimal += 0.5; // Long sleep tendency
        } else if per3 < 0.4 {
            optimal -= 0.5;
        }

        // ADA - adenosine deaminase
        // Low function = faster adenosine accumulation = more sleep pressure
        let ada = self.gene_function(Gene::ADA);
        if ada < 0.4 {
            optimal += 0.5; // Higher sleep need
        }

        // CLOCK gene variants affect circadian period
        let clock = self.gene_function(Gene::CLOCK);
        if clock > 0.6 {
            optimal += 0.25; // Slightly longer sleep preference
        }

        optimal.clamp(4.0_f64, 10.0_f64) // Physiological range
    }

    /// Calculate circadian rhythm robustness (0-1)
    ///
    /// Strong circadian rhythms are associated with:
    /// - Better metabolic health
    /// - Lower inflammation
    /// - Improved DNA repair (occurs during sleep)
    /// - Enhanced proteostasis (autophagy during sleep)
    pub fn circadian_robustness(&self) -> f64 {
        let clock_genes = [
            Gene::CLOCK, Gene::BMAL1, Gene::PER1, Gene::PER2, Gene::PER3,
            Gene::CRY1, Gene::CRY2
        ];

        let total: f64 = clock_genes.iter()
            .map(|g| self.gene_function(*g))
            .sum();

        (total / clock_genes.len() as f64).clamp(0.0, 1.0)
    }

    /// Calculate the health impact of actual vs optimal sleep
    ///
    /// Returns a multiplier for aging rate based on sleep deviation:
    /// - 1.0 = optimal (no acceleration)
    /// - >1.0 = accelerated aging from sleep deprivation or excess
    ///
    /// Both insufficient and excessive sleep accelerate aging through:
    /// - Reduced DNA repair (occurs during deep sleep)
    /// - Impaired glymphatic clearance (amyloid removal)
    /// - Increased inflammation
    /// - Metabolic dysfunction
    pub fn sleep_deviation_aging_factor(&self, actual_sleep_hours: f64) -> f64 {
        let optimal = self.optimal_sleep_hours();
        let deviation = (actual_sleep_hours - optimal).abs();

        // Quadratic penalty for deviation from optimal
        // Based on U-shaped mortality curve for sleep duration
        let base_penalty = (deviation / optimal).powi(2);

        // Circadian robustness provides some buffer
        let robustness = self.circadian_robustness();
        let buffered_penalty = base_penalty * (1.5 - robustness * 0.5);

        // Short sleep is worse than long sleep for most outcomes
        let asymmetry = if actual_sleep_hours < optimal { 1.2 } else { 1.0 };

        1.0 + buffered_penalty * asymmetry * 0.3 // Max ~1.3x aging acceleration
    }

    /// Calculate genetic risk score for various conditions
    ///
    /// Returns scores from 0.0 (low risk) to 1.0 (high risk) for:
    /// - Overall mortality risk
    /// - Cancer risk (based on DNA repair genes)
    /// - Cardiovascular risk (inflammation, metabolism)
    /// - Neurodegeneration risk (proteostasis, mitochondria)
    /// - Metabolic risk (nutrient sensing, sirtuins)
    /// - Accelerated aging (telomeres, progeria genes)
    pub fn calculate_genetic_risk_score(&self) -> GeneticRiskScore {
        // Cancer risk: DNA repair deficiency + tumor suppressor variants
        let cancer_risk = {
            let dna_repair = 1.0 - self.dna_repair_capacity();
            let tp53 = 1.0 - self.gene_function(Gene::TP53);
            let brca = (1.0 - self.gene_function(Gene::BRCA1) + 1.0 - self.gene_function(Gene::BRCA2)) / 2.0;
            ((dna_repair + tp53 + brca) / 3.0).clamp(0.0, 1.0)
        };

        // Cardiovascular risk: inflammation + metabolic genes
        let cardiovascular_risk = {
            let inflammation = self.inflammation_tendency();
            let mtor = self.gene_function(Gene::MTOR); // High mTOR = higher risk
            let ampk = 1.0 - self.gene_function(Gene::AMPK); // Low AMPK = higher risk
            ((inflammation + mtor * 0.5 + ampk * 0.5) / 2.0).clamp(0.0, 1.0)
        };

        // Neurodegeneration risk: proteostasis + mitochondrial function
        let neurodegeneration_risk = {
            let proteostasis = 1.0 - self.proteostasis_capacity();
            let mito = 1.0 - self.gene_function(Gene::PPARGC1A);
            let pink1 = 1.0 - self.gene_function(Gene::PINK1);
            ((proteostasis + mito + pink1) / 3.0).clamp(0.0, 1.0)
        };

        // Metabolic risk: nutrient sensing dysregulation
        let metabolic_risk = {
            let igf1 = self.gene_function(Gene::IGF1R); // High = more growth = more risk
            let foxo3 = 1.0 - self.gene_function(Gene::FOXO3); // Low FOXO3 = risk
            let sirt = 1.0 - (self.gene_function(Gene::SIRT1) + self.gene_function(Gene::SIRT3)) / 2.0;
            ((igf1 * 0.5 + foxo3 + sirt) / 2.5).clamp(0.0, 1.0)
        };

        // Accelerated aging risk: telomeres + progeria genes
        let accelerated_aging_risk = {
            let telo = 1.0 - self.gene_function(Gene::TERT);
            let wrn = 1.0 - self.gene_function(Gene::WRN);
            let lmna = 1.0 - self.gene_function(Gene::LMNA);
            let senescence = self.senescence_propensity();
            ((telo + wrn * 0.5 + lmna * 0.5 + senescence) / 3.0).clamp(0.0, 1.0)
        };

        // Overall risk is weighted average
        let overall = (
            cancer_risk * 0.25 +
            cardiovascular_risk * 0.25 +
            neurodegeneration_risk * 0.15 +
            metabolic_risk * 0.15 +
            accelerated_aging_risk * 0.20
        ).clamp(0.0, 1.0);

        GeneticRiskScore {
            overall,
            cancer: cancer_risk,
            cardiovascular: cardiovascular_risk,
            neurodegeneration: neurodegeneration_risk,
            metabolic: metabolic_risk,
            accelerated_aging: accelerated_aging_risk,
        }
    }

    /// Identify key genetic risk factors in this genome
    pub fn identify_risk_factors(&self) -> Vec<GeneticRiskFactor> {
        let mut factors = Vec::new();

        // Check each gene for damaging variants
        for gene in Gene::all() {
            if let Some(state) = self.nuclear_genes.get(&gene) {
                for variant in &state.variants {
                    if variant.longevity_effect < -0.1 {
                        factors.push(GeneticRiskFactor {
                            gene,
                            variant_description: format!("{} {:?}", variant.rsid, variant.effect),
                            impact: match gene.aging_role() {
                                AgingRole::DNARepair => "Reduced DNA repair capacity".to_string(),
                                AgingRole::TelomereMaintenance => "Accelerated telomere shortening".to_string(),
                                AgingRole::Senescence => "Increased cellular senescence".to_string(),
                                AgingRole::Proteostasis => "Impaired protein quality control".to_string(),
                                AgingRole::Mitochondrial => "Mitochondrial dysfunction".to_string(),
                                AgingRole::Inflammation => "Increased inflammatory signaling".to_string(),
                                AgingRole::CircadianRhythm => "Disrupted circadian rhythm".to_string(),
                                AgingRole::Progeria => "Progeroid syndrome risk".to_string(),
                                _ => "Potential negative impact on longevity".to_string(),
                            },
                            risk_increase: -variant.longevity_effect,
                        });
                    }
                }
            }
        }

        // Sort by risk level
        factors.sort_by(|a, b| b.risk_increase.partial_cmp(&a.risk_increase).unwrap());
        factors.truncate(10); // Top 10 risk factors
        factors
    }

    /// Identify key protective genetic factors
    pub fn identify_protective_factors(&self) -> Vec<GeneticProtectiveFactor> {
        let mut factors = Vec::new();

        for gene in Gene::all() {
            if let Some(state) = self.nuclear_genes.get(&gene) {
                for variant in &state.variants {
                    if variant.longevity_effect > 0.1 {
                        factors.push(GeneticProtectiveFactor {
                            gene,
                            variant_description: format!("{} {:?}", variant.rsid, variant.effect),
                            impact: match gene.aging_role() {
                                AgingRole::DNARepair => "Enhanced DNA repair".to_string(),
                                AgingRole::Sirtuins => "Enhanced sirtuin activity".to_string(),
                                AgingRole::NutrientSensing => {
                                    if gene == Gene::FOXO3 {
                                        "FOXO3 longevity variant".to_string()
                                    } else {
                                        "Favorable nutrient sensing".to_string()
                                    }
                                },
                                AgingRole::CircadianRhythm => {
                                    if gene == Gene::DEC2 || gene == Gene::ADRB1 {
                                        "Natural short sleep phenotype".to_string()
                                    } else {
                                        "Robust circadian rhythm".to_string()
                                    }
                                },
                                AgingRole::TelomereMaintenance => "Enhanced telomere maintenance".to_string(),
                                AgingRole::Inflammation => "Reduced inflammatory signaling".to_string(),
                                _ => "Potential positive impact on longevity".to_string(),
                            },
                            protection_level: variant.longevity_effect,
                        });
                    }
                }
            }
        }

        factors.sort_by(|a, b| b.protection_level.partial_cmp(&a.protection_level).unwrap());
        factors.truncate(10);
        factors
    }
}

/// Genetic risk score for various conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRiskScore {
    pub overall: f64,
    pub cancer: f64,
    pub cardiovascular: f64,
    pub neurodegeneration: f64,
    pub metabolic: f64,
    pub accelerated_aging: f64,
}

/// A genetic factor that increases disease/mortality risk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRiskFactor {
    pub gene: Gene,
    pub variant_description: String,
    pub impact: String,
    pub risk_increase: f64,
}

/// A genetic factor that protects against disease/mortality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticProtectiveFactor {
    pub gene: Gene,
    pub variant_description: String,
    pub impact: String,
    pub protection_level: f64,
}

/// Generate a random variant for a gene
fn generate_random_variant(gene: Gene, rng: &mut impl Rng) -> GeneVariant {
    let effects = [
        VariantEffect::LossOfFunction,
        VariantEffect::ReducedFunction,
        VariantEffect::Neutral,
        VariantEffect::EnhancedFunction,
        VariantEffect::GainOfFunction,
    ];

    let effect = effects.choose(rng).unwrap();

    // Longevity effect based on gene role and variant type
    let longevity_effect = match (gene.aging_role(), effect) {
        (AgingRole::DNARepair, VariantEffect::EnhancedFunction) => 0.3,
        (AgingRole::DNARepair, VariantEffect::LossOfFunction) => -0.5,
        (AgingRole::Sirtuins, VariantEffect::EnhancedFunction) => 0.4,
        (AgingRole::NutrientSensing, VariantEffect::ReducedFunction) => {
            // Reduced IGF-1 signaling is often protective
            if gene == Gene::IGF1R || gene == Gene::MTOR { 0.3 } else { -0.2 }
        },
        (AgingRole::Inflammation, VariantEffect::ReducedFunction) => 0.2,
        (AgingRole::TelomereMaintenance, VariantEffect::EnhancedFunction) => 0.2,
        (AgingRole::CircadianRhythm, VariantEffect::ReducedFunction) => {
            // DEC2/ADRB1 loss-of-function = beneficial short sleep phenotype
            if gene == Gene::DEC2 || gene == Gene::ADRB1 { 0.2 } else { -0.1 }
        },
        (AgingRole::CircadianRhythm, VariantEffect::EnhancedFunction) => 0.1,
        _ => rng.gen_range(-0.2..0.2),
    };

    GeneVariant {
        rsid: format!("rs{}", rng.gen::<u32>() % 10000000),
        effect: *effect,
        allele_frequency: rng.gen_range(0.001..0.3),
        longevity_effect,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_creation() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);
        assert!(!genome.nuclear_genes.is_empty());
        assert_eq!(genome.telomeres.len(), 46);
    }

    #[test]
    fn test_dna_repair_capacity() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);
        let capacity = genome.dna_repair_capacity();
        assert!(capacity >= 0.0 && capacity <= 2.0);
    }

    #[test]
    fn test_telomere_shortening() {
        let mut rng = rand::thread_rng();
        let mut telo = TelomereState::default();
        let initial = telo.length_bp;

        for _ in 0..50 { // 50 divisions
            telo.divide(&mut rng);
        }

        assert!(telo.length_bp < initial);
    }

    #[test]
    fn test_mtdna_aging() {
        let mut rng = rand::thread_rng();
        let mut mtdna = MitochondrialDNA::default();

        for _ in 0..80 { // 80 years
            mtdna.age_one_year(0.5, &mut rng);
        }

        assert!(mtdna.heteroplasmy > 0.0);
    }

    #[test]
    fn test_optimal_sleep_hours() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);

        let optimal = genome.optimal_sleep_hours();

        // Optimal sleep should be within physiological range
        assert!(optimal >= 4.0 && optimal <= 10.0);

        // Population mean is around 7.5 hours
        // Most people should be in 6-9 hour range
    }

    #[test]
    fn test_sleep_deviation_aging_factor() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);

        let optimal = genome.optimal_sleep_hours();

        // Sleeping at optimal hours should have no aging penalty
        let optimal_factor = genome.sleep_deviation_aging_factor(optimal);
        assert!((optimal_factor - 1.0).abs() < 0.01);

        // Severe sleep deprivation should accelerate aging
        let deprived_factor = genome.sleep_deviation_aging_factor(4.0);
        assert!(deprived_factor > 1.0);

        // Excessive sleep should also have some penalty
        let oversleep_factor = genome.sleep_deviation_aging_factor(11.0);
        assert!(oversleep_factor > 1.0);
    }

    #[test]
    fn test_circadian_robustness() {
        let mut rng = rand::thread_rng();
        let genome = Genome::new_random(&mut rng);

        let robustness = genome.circadian_robustness();

        // Should be between 0 and 1
        assert!(robustness >= 0.0 && robustness <= 1.0);
    }
}
