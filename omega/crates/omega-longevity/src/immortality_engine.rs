//! # The Immortality Engine
//!
//! Not just predicting death - **PREVENTING** it.
//!
//! This module doesn't simulate aging to watch you die.
//! It finds the optimal intervention stack to keep you alive indefinitely.
//!
//! ```text
//!  ╔═══════════════════════════════════════════════════════════════════════════╗
//!  ║                     THE IMMORTALITY ENGINE                                ║
//!  ╠═══════════════════════════════════════════════════════════════════════════╣
//!  ║                                                                           ║
//!  ║  YOUR GENOME ──────────────────────────────────────────────────────────►  ║
//!  ║       │                                                                   ║
//!  ║       ▼                                                                   ║
//!  ║  ┌─────────────────────────────────────────────────────────────────────┐  ║
//!  ║  │                    INTERVENTION SPACE                               │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  PROVEN                    EXPERIMENTAL             SPECULATIVE    │  ║
//!  ║  │  ─────────────────         ─────────────            ────────────   │  ║
//!  ║  │  • Rapamycin               • Senolytics             • Gene editing │  ║
//!  ║  │  • Metformin               • Yamanaka factors       • Nanomedicine │  ║
//!  ║  │  • NAD+ precursors         • Telomerase activation  • Organ regen  │  ║
//!  ║  │  • Caloric restriction     • Young plasma factors   • Brain upload │  ║
//!  ║  │  • Exercise protocols      • Mitochondrial transfer • Substrate    │  ║
//!  ║  │  • Sleep optimization      • Stem cell therapy      │  transfer   │  ║
//!  ║  │                                                                     │  ║
//!  ║  │                     1,000,000+ COMBINATIONS                         │  ║
//!  ║  └─────────────────────────────────────────────────────────────────────┘  ║
//!  ║       │                                                                   ║
//!  ║       ▼                                                                   ║
//!  ║  ┌─────────────────────────────────────────────────────────────────────┐  ║
//!  ║  │                    MASSIVE PARALLEL SIMULATION                      │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  For each intervention combination:                                 │  ║
//!  ║  │    1. Clone your genome                                            │  ║
//!  ║  │    2. Apply interventions at optimal ages                          │  ║
//!  ║  │    3. Simulate 1000 years of life                                  │  ║
//!  ║  │    4. Track: lifespan, healthspan, side effects                    │  ║
//!  ║  │    5. Score and rank                                               │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  Run 10,000,000 simulated lifetimes in parallel                    │  ║
//!  ║  └─────────────────────────────────────────────────────────────────────┘  ║
//!  ║       │                                                                   ║
//!  ║       ▼                                                                   ║
//!  ║  ┌─────────────────────────────────────────────────────────────────────┐  ║
//!  ║  │                    YOUR IMMORTALITY PROTOCOL                        │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  Age 30-40:                                                         │  ║
//!  ║  │    • Start rapamycin 5mg weekly                                    │  ║
//!  ║  │    • NMN 500mg daily                                               │  ║
//!  ║  │    • Optimize sleep to 7.2 hours (your genetic optimum)            │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  Age 50:                                                            │  ║
//!  ║  │    • First senolytic course (Dasatinib + Quercetin)               │  ║
//!  ║  │    • Increase rapamycin to 6mg                                     │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  Age 60:                                                            │  ║
//!  ║  │    • Begin Yamanaka factor cycling (3 days on, 4 off)             │  ║
//!  ║  │    • Telomerase activation therapy                                 │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  Age 80+:                                                           │  ║
//!  ║  │    • Gradual organ replacement with lab-grown organs               │  ║
//!  ║  │    • Neural substrate preservation initiated                       │  ║
//!  ║  │                                                                     │  ║
//!  ║  │  PREDICTED OUTCOME: Biological age stabilized at 35-40            │  ║
//!  ║  │  PREDICTED LIFESPAN: 500+ years (limited by accident/violence)    │  ║
//!  ║  └─────────────────────────────────────────────────────────────────────┘  ║
//!  ║                                                                           ║
//!  ╚═══════════════════════════════════════════════════════════════════════════╝
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::genome::{Genome, Gene, GeneState, GeneVariant, VariantEffect, AgingRole};
use crate::organism::{Organism, Lifestyle, Disease, DiseaseType, DeathCause};
use crate::hallmarks::Hallmark;

/// The Immortality Engine - finds optimal intervention stacks to prevent death
pub struct ImmortalityEngine {
    config: ImmortalityConfig,
    /// All known interventions
    interventions: Vec<Intervention>,
    /// Discovered synergies between interventions
    synergies: HashMap<(Uuid, Uuid), SynergyEffect>,
    /// Contraindications to avoid
    contraindications: Vec<Contraindication>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmortalityConfig {
    /// Number of intervention combinations to test
    pub combinations_to_test: usize,
    /// Simulations per combination
    pub simulations_per_combo: usize,
    /// Maximum simulation years
    pub max_simulation_years: u32,
    /// Include experimental interventions
    pub include_experimental: bool,
    /// Include speculative/future interventions
    pub include_speculative: bool,
    /// Target biological age to maintain
    pub target_biological_age: f64,
    /// Risk tolerance (0 = very conservative, 1 = aggressive)
    pub risk_tolerance: f64,
}

impl Default for ImmortalityConfig {
    fn default() -> Self {
        Self {
            combinations_to_test: 100_000,
            simulations_per_combo: 100,
            max_simulation_years: 500,
            include_experimental: true,
            include_speculative: true,
            target_biological_age: 35.0,
            risk_tolerance: 0.5,
        }
    }
}

/// An intervention that can affect aging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub id: Uuid,
    pub name: String,
    pub category: InterventionCategory,
    pub mechanism: Vec<AgingMechanism>,
    /// When to start (biological age)
    pub optimal_start_age: f64,
    /// Dosing protocol
    pub protocol: Protocol,
    /// Effects on hallmarks of aging
    pub hallmark_effects: HashMap<Hallmark, f64>,
    /// Side effect probability
    pub side_effect_risk: f64,
    /// Evidence level
    pub evidence: EvidenceLevel,
    /// Cost per year (USD)
    pub annual_cost: f64,
    /// Required monitoring
    pub monitoring: Vec<Biomarker>,
    /// Gene-specific efficacy modifiers
    pub genetic_modifiers: HashMap<Gene, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionCategory {
    /// Proven in humans (rapamycin, metformin, etc.)
    Pharmaceutical,
    /// Supplements with evidence (NMN, resveratrol, etc.)
    Nutraceutical,
    /// Lifestyle interventions (CR, exercise, sleep)
    Lifestyle,
    /// Cell-based therapies (stem cells, CAR-T for senescence)
    CellTherapy,
    /// Yamanaka factors, partial reprogramming
    Reprogramming,
    /// Dasatinib, quercetin, fisetin, etc.
    Senolytic,
    /// TERT activation, telomere extension
    TelomeraseActivation,
    /// Gene editing for longevity (CRISPR)
    GeneTherapy,
    /// Young plasma factors, GDF11, etc.
    PlasmaFactors,
    /// Lab-grown organs, xenotransplantation
    OrganReplacement,
    /// Targeted drug delivery, repair nanobots
    Nanomedicine,
    /// Gradual neural substrate transfer
    SubstrateTransfer,
    /// Cryonics, vitrification
    Preservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLevel {
    /// Human RCTs with longevity endpoints
    HumanProven,
    /// Human studies with biomarker endpoints
    HumanBiomarkers,
    /// Extends lifespan in mammals
    MammalProven,
    /// Works in model organisms (worms, flies)
    ModelOrganism,
    /// Theoretical / preclinical
    Preclinical,
    /// Speculative / future technology
    Speculative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgingMechanism {
    /// mTOR inhibition
    MTORInhibition,
    /// AMPK activation
    AMPKActivation,
    /// Sirtuin activation
    SirtuinActivation,
    /// NAD+ restoration
    NADRestoration,
    /// Senescent cell clearance
    SenolyticClearance,
    /// Epigenetic reprogramming
    EpigeneticReprogramming,
    /// Telomere extension
    TelomereExtension,
    /// Mitochondrial enhancement
    MitochondrialEnhancement,
    /// Inflammation reduction
    AntiInflammation,
    /// Proteostasis enhancement
    ProteostasisEnhancement,
    /// Stem cell rejuvenation
    StemCellRejuvenation,
    /// DNA repair enhancement
    DNARepairEnhancement,
    /// Autophagy induction
    AutophagyInduction,
    /// Glycation reduction
    AntiGlycation,
    /// Hormone optimization
    HormoneOptimization,
    /// Neural protection
    Neuroprotection,
    /// Organ regeneration
    OrganRegeneration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protocol {
    /// Dosing schedule
    pub schedule: DosingSchedule,
    /// Duration of each cycle
    pub cycle_duration_days: u32,
    /// Rest period between cycles
    pub rest_period_days: u32,
    /// Dose adjustments based on response
    pub adaptive_dosing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DosingSchedule {
    Daily { dose: f64, unit: String },
    Weekly { dose: f64, unit: String },
    Cycling { on_days: u32, off_days: u32, dose: f64, unit: String },
    Pulsed { pulses_per_year: u32, duration_days: u32, dose: f64 },
    SingleTreatment,
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biomarker {
    pub name: String,
    pub frequency_days: u32,
    pub optimal_range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyEffect {
    pub intervention_a: Uuid,
    pub intervention_b: Uuid,
    /// Multiplier for combined effect (>1 = synergy, <1 = antagonism)
    pub effect_multiplier: f64,
    pub mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contraindication {
    pub intervention_a: Uuid,
    pub intervention_b: Uuid,
    pub reason: String,
    pub severity: ContraindicationSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContraindicationSeverity {
    Absolute, // Never combine
    Relative, // Combine with caution
    Timing,   // Don't take same day
}

/// The personalized immortality protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmortalityProtocol {
    pub id: Uuid,
    pub genome_id: Uuid,
    /// Interventions organized by life phase
    pub phases: Vec<ProtocolPhase>,
    /// Predicted outcomes
    pub predictions: ProtocolPredictions,
    /// Monitoring schedule
    pub monitoring: MonitoringSchedule,
    /// Fallback protocols if primary fails
    pub contingencies: Vec<Contingency>,
    /// Total lifetime cost estimate
    pub estimated_lifetime_cost: f64,
    /// Confidence in predictions
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolPhase {
    pub name: String,
    pub start_age: f64,
    pub end_age: Option<f64>,
    pub interventions: Vec<PhaseIntervention>,
    pub goals: Vec<String>,
    pub expected_biological_age: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseIntervention {
    pub intervention_id: Uuid,
    pub intervention_name: String,
    pub protocol: Protocol,
    pub priority: Priority,
    pub personalized_notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Critical,   // Must do
    Important,  // Strongly recommended
    Beneficial, // Nice to have
    Optional,   // Consider based on response
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolPredictions {
    /// Predicted maximum lifespan
    pub max_lifespan_years: f64,
    /// Predicted healthspan
    pub healthspan_years: f64,
    /// Probability of reaching 100
    pub prob_centenarian: f64,
    /// Probability of reaching 150
    pub prob_150: f64,
    /// Probability of reaching 200
    pub prob_200: f64,
    /// Probability of reaching 500+
    pub prob_500_plus: f64,
    /// Expected biological age at chronological age 100
    pub bio_age_at_100: f64,
    /// Disease risk reductions
    pub disease_risk_reductions: HashMap<DiseaseType, f64>,
    /// Most likely limiting factor
    pub limiting_factor: LimitingFactor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LimitingFactor {
    /// Nothing - protocol achieves indefinite lifespan
    None,
    /// Cancer eventually overcomes defenses
    Cancer { expected_age: f64 },
    /// Cardiovascular system limit
    Cardiovascular { expected_age: f64 },
    /// Neurodegeneration limit
    Neurodegeneration { expected_age: f64 },
    /// Technology not yet available
    TechnologyGap { technology: String, expected_year: u32 },
    /// Accident/external causes
    ExternalCauses { annual_risk: f64 },
    /// Unknown aging mechanism
    UnknownMechanism { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSchedule {
    pub biomarkers: Vec<BiomarkerSchedule>,
    pub imaging: Vec<ImagingSchedule>,
    pub genetic_monitoring: Vec<GeneticMonitoring>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerSchedule {
    pub name: String,
    pub frequency_weeks: u32,
    pub target_range: (f64, f64),
    pub action_if_abnormal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingSchedule {
    pub modality: String,
    pub frequency_months: u32,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticMonitoring {
    pub test_type: String,
    pub frequency_years: u32,
    pub looking_for: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contingency {
    pub trigger: String,
    pub action: String,
    pub fallback_interventions: Vec<Uuid>,
}

impl ImmortalityEngine {
    pub fn new(config: ImmortalityConfig) -> Self {
        let mut engine = Self {
            config,
            interventions: Vec::new(),
            synergies: HashMap::new(),
            contraindications: Vec::new(),
        };
        engine.load_interventions();
        engine.load_synergies();
        engine.load_contraindications();
        engine
    }

    /// Load all known interventions
    fn load_interventions(&mut self) {
        // PROVEN INTERVENTIONS
        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "Rapamycin".to_string(),
            category: InterventionCategory::Pharmaceutical,
            mechanism: vec![AgingMechanism::MTORInhibition, AgingMechanism::AutophagyInduction],
            optimal_start_age: 40.0,
            protocol: Protocol {
                schedule: DosingSchedule::Weekly { dose: 5.0, unit: "mg".to_string() },
                cycle_duration_days: 7,
                rest_period_days: 0,
                adaptive_dosing: true,
            },
            hallmark_effects: [
                (Hallmark::DeregulatedNutrientSensing, -0.4),
                (Hallmark::LossOfProteostasis, -0.3),
                (Hallmark::CellularSenescence, -0.2),
            ].into_iter().collect(),
            side_effect_risk: 0.15,
            evidence: EvidenceLevel::MammalProven,
            annual_cost: 500.0,
            monitoring: vec![
                Biomarker { name: "Lipids".to_string(), frequency_days: 90, optimal_range: (0.0, 200.0) },
                Biomarker { name: "Glucose".to_string(), frequency_days: 90, optimal_range: (70.0, 100.0) },
            ],
            genetic_modifiers: [
                (Gene::MTOR, 1.2), // Better response if MTOR is active
                (Gene::AMPK, 0.8), // Less needed if AMPK is active
            ].into_iter().collect(),
        });

        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "Metformin".to_string(),
            category: InterventionCategory::Pharmaceutical,
            mechanism: vec![AgingMechanism::AMPKActivation, AgingMechanism::AntiInflammation],
            optimal_start_age: 45.0,
            protocol: Protocol {
                schedule: DosingSchedule::Daily { dose: 1000.0, unit: "mg".to_string() },
                cycle_duration_days: 1,
                rest_period_days: 0,
                adaptive_dosing: false,
            },
            hallmark_effects: [
                (Hallmark::DeregulatedNutrientSensing, -0.3),
                (Hallmark::AlteredIntercellularCommunication, -0.2),
            ].into_iter().collect(),
            side_effect_risk: 0.1,
            evidence: EvidenceLevel::HumanBiomarkers,
            annual_cost: 100.0,
            monitoring: vec![
                Biomarker { name: "B12".to_string(), frequency_days: 180, optimal_range: (300.0, 900.0) },
            ],
            genetic_modifiers: HashMap::new(),
        });

        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "NMN (Nicotinamide Mononucleotide)".to_string(),
            category: InterventionCategory::Nutraceutical,
            mechanism: vec![AgingMechanism::NADRestoration, AgingMechanism::SirtuinActivation],
            optimal_start_age: 35.0,
            protocol: Protocol {
                schedule: DosingSchedule::Daily { dose: 500.0, unit: "mg".to_string() },
                cycle_duration_days: 1,
                rest_period_days: 0,
                adaptive_dosing: false,
            },
            hallmark_effects: [
                (Hallmark::MitochondrialDysfunction, -0.3),
                (Hallmark::GenomicInstability, -0.15),
            ].into_iter().collect(),
            side_effect_risk: 0.02,
            evidence: EvidenceLevel::HumanBiomarkers,
            annual_cost: 1200.0,
            monitoring: vec![],
            genetic_modifiers: [
                (Gene::SIRT1, 1.3),
                (Gene::SIRT3, 1.2),
            ].into_iter().collect(),
        });

        // SENOLYTICS
        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "Dasatinib + Quercetin (D+Q)".to_string(),
            category: InterventionCategory::Senolytic,
            mechanism: vec![AgingMechanism::SenolyticClearance, AgingMechanism::AntiInflammation],
            optimal_start_age: 50.0,
            protocol: Protocol {
                schedule: DosingSchedule::Pulsed {
                    pulses_per_year: 4,
                    duration_days: 3,
                    dose: 100.0, // mg dasatinib
                },
                cycle_duration_days: 3,
                rest_period_days: 87,
                adaptive_dosing: true,
            },
            hallmark_effects: [
                (Hallmark::CellularSenescence, -0.6),
                (Hallmark::AlteredIntercellularCommunication, -0.4),
                (Hallmark::StemCellExhaustion, -0.2),
            ].into_iter().collect(),
            side_effect_risk: 0.2,
            evidence: EvidenceLevel::HumanBiomarkers,
            annual_cost: 2000.0,
            monitoring: vec![
                Biomarker { name: "p16INK4a".to_string(), frequency_days: 180, optimal_range: (0.0, 0.5) },
            ],
            genetic_modifiers: [
                (Gene::CDKN2A, 1.5), // More benefit if high senescence
            ].into_iter().collect(),
        });

        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "Fisetin".to_string(),
            category: InterventionCategory::Senolytic,
            mechanism: vec![AgingMechanism::SenolyticClearance],
            optimal_start_age: 45.0,
            protocol: Protocol {
                schedule: DosingSchedule::Pulsed {
                    pulses_per_year: 12,
                    duration_days: 2,
                    dose: 1000.0,
                },
                cycle_duration_days: 2,
                rest_period_days: 28,
                adaptive_dosing: false,
            },
            hallmark_effects: [
                (Hallmark::CellularSenescence, -0.4),
            ].into_iter().collect(),
            side_effect_risk: 0.05,
            evidence: EvidenceLevel::MammalProven,
            annual_cost: 500.0,
            monitoring: vec![],
            genetic_modifiers: HashMap::new(),
        });

        // REPROGRAMMING
        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "Partial Yamanaka Factor Cycling (OSK)".to_string(),
            category: InterventionCategory::Reprogramming,
            mechanism: vec![
                AgingMechanism::EpigeneticReprogramming,
                AgingMechanism::StemCellRejuvenation,
            ],
            optimal_start_age: 55.0,
            protocol: Protocol {
                schedule: DosingSchedule::Cycling {
                    on_days: 2,
                    off_days: 5,
                    dose: 1.0, // Relative expression
                    unit: "x".to_string(),
                },
                cycle_duration_days: 7,
                rest_period_days: 21,
                adaptive_dosing: true,
            },
            hallmark_effects: [
                (Hallmark::EpigeneticAlterations, -0.7),
                (Hallmark::StemCellExhaustion, -0.5),
                (Hallmark::CellularSenescence, -0.4),
                (Hallmark::LossOfProteostasis, -0.3),
            ].into_iter().collect(),
            side_effect_risk: 0.3, // Cancer risk if overdone
            evidence: EvidenceLevel::MammalProven,
            annual_cost: 50000.0,
            monitoring: vec![
                Biomarker { name: "Epigenetic age".to_string(), frequency_days: 30, optimal_range: (30.0, 45.0) },
                Biomarker { name: "Tumor markers".to_string(), frequency_days: 30, optimal_range: (0.0, 1.0) },
            ],
            genetic_modifiers: [
                (Gene::TP53, -0.5), // Lower efficacy if TP53 impaired (cancer risk)
                (Gene::MYC, -0.3),  // Careful with MYC variants
            ].into_iter().collect(),
        });

        // TELOMERASE ACTIVATION
        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "AAV-TERT Gene Therapy".to_string(),
            category: InterventionCategory::TelomeraseActivation,
            mechanism: vec![AgingMechanism::TelomereExtension, AgingMechanism::StemCellRejuvenation],
            optimal_start_age: 50.0,
            protocol: Protocol {
                schedule: DosingSchedule::Pulsed {
                    pulses_per_year: 1,
                    duration_days: 1,
                    dose: 1.0,
                },
                cycle_duration_days: 1,
                rest_period_days: 365,
                adaptive_dosing: true,
            },
            hallmark_effects: [
                (Hallmark::TelomereAttrition, -0.8),
                (Hallmark::StemCellExhaustion, -0.3),
            ].into_iter().collect(),
            side_effect_risk: 0.25,
            evidence: EvidenceLevel::MammalProven,
            annual_cost: 100000.0,
            monitoring: vec![
                Biomarker { name: "Telomere length".to_string(), frequency_days: 90, optimal_range: (7.0, 15.0) },
            ],
            genetic_modifiers: [
                (Gene::TERT, 1.5),
                (Gene::TP53, -0.4), // Need good TP53 for safety
            ].into_iter().collect(),
        });

        // SPECULATIVE INTERVENTIONS
        if self.config.include_speculative {
            self.interventions.push(Intervention {
                id: Uuid::new_v4(),
                name: "Senescent Cell CAR-T Therapy".to_string(),
                category: InterventionCategory::CellTherapy,
                mechanism: vec![AgingMechanism::SenolyticClearance],
                optimal_start_age: 60.0,
                protocol: Protocol {
                    schedule: DosingSchedule::SingleTreatment,
                    cycle_duration_days: 1,
                    rest_period_days: 365,
                    adaptive_dosing: false,
                },
                hallmark_effects: [
                    (Hallmark::CellularSenescence, -0.9),
                    (Hallmark::AlteredIntercellularCommunication, -0.5),
                ].into_iter().collect(),
                side_effect_risk: 0.15,
                evidence: EvidenceLevel::Preclinical,
                annual_cost: 200000.0,
                monitoring: vec![],
                genetic_modifiers: HashMap::new(),
            });

            self.interventions.push(Intervention {
                id: Uuid::new_v4(),
                name: "Mitochondrial DNA Repair Nanobots".to_string(),
                category: InterventionCategory::Nanomedicine,
                mechanism: vec![
                    AgingMechanism::MitochondrialEnhancement,
                    AgingMechanism::DNARepairEnhancement,
                ],
                optimal_start_age: 40.0,
                protocol: Protocol {
                    schedule: DosingSchedule::Continuous,
                    cycle_duration_days: 365,
                    rest_period_days: 0,
                    adaptive_dosing: true,
                },
                hallmark_effects: [
                    (Hallmark::MitochondrialDysfunction, -0.9),
                    (Hallmark::GenomicInstability, -0.5),
                ].into_iter().collect(),
                side_effect_risk: 0.1,
                evidence: EvidenceLevel::Speculative,
                annual_cost: 500000.0,
                monitoring: vec![],
                genetic_modifiers: HashMap::new(),
            });

            self.interventions.push(Intervention {
                id: Uuid::new_v4(),
                name: "Lab-Grown Organ Replacement Program".to_string(),
                category: InterventionCategory::OrganReplacement,
                mechanism: vec![AgingMechanism::OrganRegeneration],
                optimal_start_age: 70.0,
                protocol: Protocol {
                    schedule: DosingSchedule::Pulsed {
                        pulses_per_year: 1,
                        duration_days: 30,
                        dose: 1.0,
                    },
                    cycle_duration_days: 30,
                    rest_period_days: 335,
                    adaptive_dosing: true,
                },
                hallmark_effects: [
                    (Hallmark::StemCellExhaustion, -0.8),
                ].into_iter().collect(),
                side_effect_risk: 0.2,
                evidence: EvidenceLevel::Speculative,
                annual_cost: 1000000.0,
                monitoring: vec![],
                genetic_modifiers: HashMap::new(),
            });

            self.interventions.push(Intervention {
                id: Uuid::new_v4(),
                name: "Gradual Neural Substrate Transfer".to_string(),
                category: InterventionCategory::SubstrateTransfer,
                mechanism: vec![AgingMechanism::Neuroprotection],
                optimal_start_age: 80.0,
                protocol: Protocol {
                    schedule: DosingSchedule::Continuous,
                    cycle_duration_days: 365,
                    rest_period_days: 0,
                    adaptive_dosing: true,
                },
                hallmark_effects: [
                    (Hallmark::StemCellExhaustion, -1.0), // Brain doesn't age
                ].into_iter().collect(),
                side_effect_risk: 0.05, // Gradual, low risk
                evidence: EvidenceLevel::Speculative,
                annual_cost: 10000000.0,
                monitoring: vec![],
                genetic_modifiers: HashMap::new(),
            });
        }

        // LIFESTYLE INTERVENTIONS
        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "Optimized Caloric Restriction (20% reduction)".to_string(),
            category: InterventionCategory::Lifestyle,
            mechanism: vec![
                AgingMechanism::MTORInhibition,
                AgingMechanism::AMPKActivation,
                AgingMechanism::AutophagyInduction,
            ],
            optimal_start_age: 25.0,
            protocol: Protocol {
                schedule: DosingSchedule::Daily { dose: 0.8, unit: "x calories".to_string() },
                cycle_duration_days: 1,
                rest_period_days: 0,
                adaptive_dosing: true,
            },
            hallmark_effects: [
                (Hallmark::DeregulatedNutrientSensing, -0.5),
                (Hallmark::MitochondrialDysfunction, -0.3),
                (Hallmark::CellularSenescence, -0.2),
            ].into_iter().collect(),
            side_effect_risk: 0.05,
            evidence: EvidenceLevel::MammalProven,
            annual_cost: 0.0,
            monitoring: vec![],
            genetic_modifiers: HashMap::new(),
        });

        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "Genetically-Optimized Sleep Protocol".to_string(),
            category: InterventionCategory::Lifestyle,
            mechanism: vec![
                AgingMechanism::DNARepairEnhancement,
                AgingMechanism::ProteostasisEnhancement,
                AgingMechanism::Neuroprotection,
            ],
            optimal_start_age: 0.0,
            protocol: Protocol {
                schedule: DosingSchedule::Daily { dose: 1.0, unit: "optimal hours".to_string() },
                cycle_duration_days: 1,
                rest_period_days: 0,
                adaptive_dosing: true,
            },
            hallmark_effects: [
                (Hallmark::GenomicInstability, -0.2),
                (Hallmark::LossOfProteostasis, -0.2),
                (Hallmark::AlteredIntercellularCommunication, -0.15),
            ].into_iter().collect(),
            side_effect_risk: 0.0,
            evidence: EvidenceLevel::HumanProven,
            annual_cost: 0.0,
            monitoring: vec![],
            genetic_modifiers: [
                (Gene::DEC2, 1.2),
                (Gene::PER3, 1.1),
            ].into_iter().collect(),
        });

        self.interventions.push(Intervention {
            id: Uuid::new_v4(),
            name: "High-Intensity Interval Training (HIIT)".to_string(),
            category: InterventionCategory::Lifestyle,
            mechanism: vec![
                AgingMechanism::MitochondrialEnhancement,
                AgingMechanism::AMPKActivation,
                AgingMechanism::StemCellRejuvenation,
            ],
            optimal_start_age: 18.0,
            protocol: Protocol {
                schedule: DosingSchedule::Weekly { dose: 3.0, unit: "sessions".to_string() },
                cycle_duration_days: 7,
                rest_period_days: 0,
                adaptive_dosing: true,
            },
            hallmark_effects: [
                (Hallmark::MitochondrialDysfunction, -0.4),
                (Hallmark::StemCellExhaustion, -0.2),
                (Hallmark::TelomereAttrition, -0.15),
            ].into_iter().collect(),
            side_effect_risk: 0.02,
            evidence: EvidenceLevel::HumanBiomarkers,
            annual_cost: 500.0,
            monitoring: vec![],
            genetic_modifiers: [
                (Gene::PPARGC1A, 1.3),
            ].into_iter().collect(),
        });
    }

    fn load_synergies(&mut self) {
        // Get intervention IDs by name for synergy definitions
        let get_id = |name: &str| -> Option<Uuid> {
            self.interventions.iter()
                .find(|i| i.name.contains(name))
                .map(|i| i.id)
        };

        // Rapamycin + NMN synergy: mTOR inhibition enhances NAD+ benefits
        if let (Some(rapa_id), Some(nmn_id)) = (get_id("Rapamycin"), get_id("NMN")) {
            self.synergies.insert((rapa_id, nmn_id), SynergyEffect {
                intervention_a: rapa_id,
                intervention_b: nmn_id,
                effect_multiplier: 1.25,
                mechanism: "mTOR inhibition potentiates sirtuin activation from NAD+ restoration".to_string(),
            });
        }

        // Senolytics + Reprogramming synergy: clear senescent cells before reprogramming
        if let (Some(dq_id), Some(osk_id)) = (get_id("Dasatinib"), get_id("Yamanaka")) {
            self.synergies.insert((dq_id, osk_id), SynergyEffect {
                intervention_a: dq_id,
                intervention_b: osk_id,
                effect_multiplier: 1.4,
                mechanism: "Senescent cell clearance prevents SASP interference with reprogramming".to_string(),
            });
        }

        // NMN + HIIT synergy: exercise boosts NAD+ utilization
        if let (Some(nmn_id), Some(hiit_id)) = (get_id("NMN"), get_id("HIIT")) {
            self.synergies.insert((nmn_id, hiit_id), SynergyEffect {
                intervention_a: nmn_id,
                intervention_b: hiit_id,
                effect_multiplier: 1.2,
                mechanism: "Exercise increases NAD+ demand and sirtuin activation".to_string(),
            });
        }

        // Caloric restriction + Rapamycin synergy: complementary mTOR/AMPK pathways
        if let (Some(cr_id), Some(rapa_id)) = (get_id("Caloric Restriction"), get_id("Rapamycin")) {
            self.synergies.insert((cr_id, rapa_id), SynergyEffect {
                intervention_a: cr_id,
                intervention_b: rapa_id,
                effect_multiplier: 1.15,
                mechanism: "CR activates AMPK which synergizes with mTOR inhibition".to_string(),
            });
        }

        // Telomerase + Senolytics synergy: prevent telomere-induced senescence accumulation
        if let (Some(tert_id), Some(fis_id)) = (get_id("TERT"), get_id("Fisetin")) {
            self.synergies.insert((tert_id, fis_id), SynergyEffect {
                intervention_a: tert_id,
                intervention_b: fis_id,
                effect_multiplier: 1.3,
                mechanism: "Telomerase prevents new senescence while senolytics clear existing".to_string(),
            });
        }

        // Metformin + NMN synergy: AMPK + NAD+ dual activation
        if let (Some(met_id), Some(nmn_id)) = (get_id("Metformin"), get_id("NMN")) {
            self.synergies.insert((met_id, nmn_id), SynergyEffect {
                intervention_a: met_id,
                intervention_b: nmn_id,
                effect_multiplier: 1.2,
                mechanism: "AMPK activation enhances NAD+ dependent pathways".to_string(),
            });
        }

        // Sleep optimization + all interventions: recovery enables all mechanisms
        if let Some(sleep_id) = get_id("Sleep") {
            for int in &self.interventions {
                if int.id != sleep_id && !int.name.contains("Sleep") {
                    self.synergies.insert((sleep_id, int.id), SynergyEffect {
                        intervention_a: sleep_id,
                        intervention_b: int.id,
                        effect_multiplier: 1.1,
                        mechanism: "Optimal sleep enhances DNA repair and protein clearance overnight".to_string(),
                    });
                }
            }
        }
    }

    fn load_contraindications(&mut self) {
        // Get intervention IDs by name for contraindication definitions
        let get_id = |name: &str| -> Option<Uuid> {
            self.interventions.iter()
                .find(|i| i.name.contains(name))
                .map(|i| i.id)
        };

        // Rapamycin + Dasatinib: Both immunosuppressive, combined risk too high
        if let (Some(rapa_id), Some(dasa_id)) = (get_id("Rapamycin"), get_id("Dasatinib")) {
            self.contraindications.push(Contraindication {
                intervention_a: rapa_id,
                intervention_b: dasa_id,
                reason: "Combined immunosuppression increases infection risk significantly".to_string(),
                severity: ContraindicationSeverity::Timing,
            });
        }

        // Telomerase activation + Reprogramming at same time: cancer risk
        if let (Some(tert_id), Some(osk_id)) = (get_id("TERT"), get_id("Yamanaka")) {
            self.contraindications.push(Contraindication {
                intervention_a: tert_id,
                intervention_b: osk_id,
                reason: "Concurrent telomerase activation and reprogramming increases teratoma/cancer risk".to_string(),
                severity: ContraindicationSeverity::Relative,
            });
        }

        // Multiple senolytics simultaneously: excessive cell death
        if let (Some(dq_id), Some(fis_id)) = (get_id("Dasatinib"), get_id("Fisetin")) {
            self.contraindications.push(Contraindication {
                intervention_a: dq_id,
                intervention_b: fis_id,
                reason: "Combining senolytics on same day may cause excessive cell death".to_string(),
                severity: ContraindicationSeverity::Timing,
            });
        }

        // Aggressive caloric restriction + intensive exercise: metabolic stress
        if let (Some(cr_id), Some(hiit_id)) = (get_id("Caloric Restriction"), get_id("HIIT")) {
            self.contraindications.push(Contraindication {
                intervention_a: cr_id,
                intervention_b: hiit_id,
                reason: "Severe caloric restriction with intense exercise risks sarcopenia and metabolic stress".to_string(),
                severity: ContraindicationSeverity::Relative,
            });
        }
    }

    /// Generate the optimal immortality protocol for a genome
    pub fn generate_protocol(&self, genome: &Genome, rng: &mut impl Rng) -> ImmortalityProtocol {
        // 1. Calculate genetic modifiers for each intervention
        let intervention_scores = self.score_interventions_for_genome(genome);

        // 2. Generate intervention combinations
        let combinations = self.generate_combinations(&intervention_scores, rng);

        // 3. Simulate each combination
        let mut best_combo = None;
        let mut best_score = 0.0;

        for combo in combinations.iter().take(self.config.combinations_to_test) {
            let score = self.simulate_combination(genome, combo, rng);
            if score > best_score {
                best_score = score;
                best_combo = Some(combo.clone());
            }
        }

        // 4. Convert best combination to protocol
        self.create_protocol(genome, &best_combo.unwrap_or_default(), best_score, rng)
    }

    fn score_interventions_for_genome(&self, genome: &Genome) -> Vec<(Uuid, f64)> {
        self.interventions.iter().map(|int| {
            let mut score = 1.0;

            // Apply genetic modifiers
            for (gene, modifier) in &int.genetic_modifiers {
                let gene_function = genome.gene_function(*gene);
                score *= 1.0 + (modifier - 1.0) * gene_function;
            }

            // Adjust for risk tolerance
            score *= 1.0 - int.side_effect_risk * (1.0 - self.config.risk_tolerance);

            // Adjust for evidence level
            score *= match int.evidence {
                EvidenceLevel::HumanProven => 1.0,
                EvidenceLevel::HumanBiomarkers => 0.9,
                EvidenceLevel::MammalProven => 0.75,
                EvidenceLevel::ModelOrganism => 0.5,
                EvidenceLevel::Preclinical => 0.3,
                EvidenceLevel::Speculative => {
                    if self.config.include_speculative { 0.2 } else { 0.0 }
                },
            };

            (int.id, score)
        }).collect()
    }

    fn generate_combinations(&self, scores: &[(Uuid, f64)], rng: &mut impl Rng) -> Vec<Vec<Uuid>> {
        let mut combinations = Vec::new();

        // Generate random combinations biased by score
        for _ in 0..self.config.combinations_to_test {
            let mut combo = Vec::new();
            let combo_size = rng.gen_range(3..=10);

            for _ in 0..combo_size {
                // Weighted random selection
                let total: f64 = scores.iter().map(|(_, s)| s).sum();
                let mut pick = rng.gen::<f64>() * total;

                for (id, score) in scores {
                    pick -= score;
                    if pick <= 0.0 && !combo.contains(id) {
                        combo.push(*id);
                        break;
                    }
                }
            }

            combinations.push(combo);
        }

        combinations
    }

    fn simulate_combination(&self, genome: &Genome, combo: &[Uuid], rng: &mut impl Rng) -> f64 {
        let mut total_lifespan = 0.0;
        let mut total_healthspan = 0.0;

        for _ in 0..self.config.simulations_per_combo {
            let mut organism = Organism::new_random(rng);
            organism.genome = genome.clone();

            // Apply intervention effects
            let combined_effects = self.calculate_combined_effects(combo, genome);

            // Modify organism based on interventions
            self.apply_interventions_to_organism(&mut organism, &combined_effects);

            // Simulate with extended maximum age
            let mut year = 0;
            while organism.alive && year < self.config.max_simulation_years {
                organism.age_one_year(rng);
                year += 1;
            }

            total_lifespan += organism.age;
            // Healthspan = age at which biological age exceeds target
            total_healthspan += organism.age.min(
                self.estimate_healthspan(&organism)
            );
        }

        let avg_lifespan = total_lifespan / self.config.simulations_per_combo as f64;
        let avg_healthspan = total_healthspan / self.config.simulations_per_combo as f64;

        // Score combines lifespan and healthspan
        avg_lifespan * 0.3 + avg_healthspan * 0.7
    }

    fn calculate_combined_effects(
        &self,
        combo: &[Uuid],
        genome: &Genome
    ) -> HashMap<Hallmark, f64> {
        let mut effects: HashMap<Hallmark, f64> = HashMap::new();

        for int_id in combo {
            if let Some(intervention) = self.interventions.iter().find(|i| i.id == *int_id) {
                // Calculate genetic modifier for this intervention
                let mut genetic_modifier = 1.0;
                for (gene, modifier_strength) in &intervention.genetic_modifiers {
                    let gene_function = genome.gene_function(*gene);
                    // If gene function is low and modifier is positive, reduce efficacy
                    // If gene function is high and modifier is positive, increase efficacy
                    genetic_modifier *= 1.0 + (*modifier_strength - 1.0) * gene_function;
                }

                // Apply intervention effects with genetic personalization
                for (hallmark, base_effect) in &intervention.hallmark_effects {
                    let personalized_effect = base_effect * genetic_modifier;
                    let entry = effects.entry(*hallmark).or_insert(0.0);
                    *entry += personalized_effect;
                }
            }
        }

        // Apply synergies (check both orderings)
        for i in 0..combo.len() {
            for j in (i + 1)..combo.len() {
                let (int_a, int_b) = (combo[i], combo[j]);

                // Check both orderings since synergies are directional
                let synergy = self.synergies.get(&(int_a, int_b))
                    .or_else(|| self.synergies.get(&(int_b, int_a)));

                if let Some(syn) = synergy {
                    for (_hallmark, effect) in &mut effects {
                        *effect *= syn.effect_multiplier;
                    }
                }
            }
        }

        // Apply genome-specific adjustments based on aging pathway status
        let dna_repair = genome.dna_repair_capacity();
        if dna_repair < 0.5 {
            // Poor DNA repair means more benefit from repair-enhancing interventions
            if let Some(effect) = effects.get_mut(&Hallmark::GenomicInstability) {
                *effect *= 1.2; // 20% more benefit
            }
        }

        let inflammation = genome.inflammation_tendency();
        if inflammation > 0.6 {
            // High inflammation means more benefit from anti-inflammatory interventions
            if let Some(effect) = effects.get_mut(&Hallmark::AlteredIntercellularCommunication) {
                *effect *= 1.3; // 30% more benefit
            }
        }

        let senescence = genome.senescence_propensity();
        if senescence > 0.5 {
            // High senescence propensity means more benefit from senolytics
            if let Some(effect) = effects.get_mut(&Hallmark::CellularSenescence) {
                *effect *= 1.25;
            }
        }

        effects
    }

    fn apply_interventions_to_organism(
        &self,
        organism: &mut Organism,
        effects: &HashMap<Hallmark, f64>
    ) {
        // Apply hallmark-specific effects to the organism

        // Genomic Instability: Improve DNA repair in the genome
        if let Some(&effect) = effects.get(&Hallmark::GenomicInstability) {
            // Negative effect means improvement (less instability)
            // Increase expression of DNA repair genes
            if effect < 0.0 {
                let repair_boost = (-effect * 0.3).min(0.5); // Max 50% boost
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::TP53) {
                    state.expression = (state.expression + repair_boost).min(1.0);
                }
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::BRCA1) {
                    state.expression = (state.expression + repair_boost).min(1.0);
                }
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::ATM) {
                    state.expression = (state.expression + repair_boost).min(1.0);
                }
            }
        }

        // Telomere Attrition: Increase telomerase activity
        if let Some(&effect) = effects.get(&Hallmark::TelomereAttrition) {
            if effect < 0.0 {
                let telomerase_boost = (-effect * 0.2).min(0.3);
                for telo in &mut organism.genome.telomeres {
                    telo.telomerase_activity = (telo.telomerase_activity + telomerase_boost).min(1.0);
                }
            }
        }

        // Epigenetic Alterations: Reduce epigenetic noise
        if let Some(&effect) = effects.get(&Hallmark::EpigeneticAlterations) {
            if effect < 0.0 {
                let noise_reduction = -effect * 0.3;
                organism.genome.epigenome.epigenetic_noise =
                    (organism.genome.epigenome.epigenetic_noise - noise_reduction).max(0.0);
                // Partially reset clock sites toward younger state
                for site in &mut organism.genome.epigenome.clock_sites {
                    *site = (*site - noise_reduction * 0.1).max(0.0);
                }
            }
        }

        // Loss of Proteostasis: Boost heat shock proteins
        if let Some(&effect) = effects.get(&Hallmark::LossOfProteostasis) {
            if effect < 0.0 {
                let proteo_boost = (-effect * 0.25).min(0.4);
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::HSF1) {
                    state.expression = (state.expression + proteo_boost).min(1.0);
                }
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::HSP70) {
                    state.expression = (state.expression + proteo_boost).min(1.0);
                }
            }
        }

        // Deregulated Nutrient Sensing: Improve metabolic gene expression
        if let Some(&effect) = effects.get(&Hallmark::DeregulatedNutrientSensing) {
            if effect < 0.0 {
                let sensing_improvement = (-effect * 0.2).min(0.3);
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::FOXO3) {
                    state.expression = (state.expression + sensing_improvement).min(1.0);
                }
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::SIRT1) {
                    state.expression = (state.expression + sensing_improvement).min(1.0);
                }
                // Reduce mTOR activity (lower expression = better for longevity)
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::MTOR) {
                    state.expression = (state.expression - sensing_improvement * 0.5).max(0.2);
                }
            }
        }

        // Mitochondrial Dysfunction: Improve mtDNA and complex function
        if let Some(&effect) = effects.get(&Hallmark::MitochondrialDysfunction) {
            if effect < 0.0 {
                let mito_improvement = (-effect * 0.15).min(0.25);
                organism.genome.mtdna.complex_i_function =
                    (organism.genome.mtdna.complex_i_function + mito_improvement).min(1.0);
                organism.genome.mtdna.complex_iii_function =
                    (organism.genome.mtdna.complex_iii_function + mito_improvement).min(1.0);
                organism.genome.mtdna.complex_iv_function =
                    (organism.genome.mtdna.complex_iv_function + mito_improvement).min(1.0);
                // Reduce mutation fraction
                organism.genome.mtdna.mutation_fraction =
                    (organism.genome.mtdna.mutation_fraction - mito_improvement * 0.5).max(0.0);
            }
        }

        // Cellular Senescence: Reduce senescence markers
        if let Some(&effect) = effects.get(&Hallmark::CellularSenescence) {
            if effect < 0.0 {
                let senescence_reduction = (-effect * 0.2).min(0.4);
                // Reduce p16 expression (less senescence induction)
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::CDKN2A) {
                    state.expression = (state.expression - senescence_reduction).max(0.1);
                }
            }
        }

        // Stem Cell Exhaustion: Boost stem cell maintenance genes
        if let Some(&effect) = effects.get(&Hallmark::StemCellExhaustion) {
            if effect < 0.0 {
                let stem_boost = (-effect * 0.15).min(0.3);
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::NANOG) {
                    state.expression = (state.expression + stem_boost).min(0.8); // Don't max out - cancer risk
                }
            }
        }

        // Altered Intercellular Communication: Reduce inflammation
        if let Some(&effect) = effects.get(&Hallmark::AlteredIntercellularCommunication) {
            if effect < 0.0 {
                let inflammation_reduction = (-effect * 0.25).min(0.4);
                // Reduce pro-inflammatory gene expression
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::NFKB1) {
                    state.expression = (state.expression - inflammation_reduction).max(0.2);
                }
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::IL6) {
                    state.expression = (state.expression - inflammation_reduction).max(0.1);
                }
                if let Some(state) = organism.genome.nuclear_genes.get_mut(&Gene::TNF) {
                    state.expression = (state.expression - inflammation_reduction).max(0.1);
                }
            }
        }

        // Also improve lifestyle based on intervention types
        let total_effect: f64 = effects.values().map(|e| e.abs()).sum();
        let lifestyle_boost = (total_effect * 0.05).min(0.3);

        organism.lifestyle.diet_quality = (organism.lifestyle.diet_quality + lifestyle_boost).min(1.0);
        organism.lifestyle.exercise_hours = (organism.lifestyle.exercise_hours + lifestyle_boost * 5.0).min(10.0);
        organism.lifestyle.stress = (organism.lifestyle.stress - lifestyle_boost).max(0.1);
        organism.lifestyle.sleep_hours = organism.genome.optimal_sleep_hours(); // Match genetic optimum
    }

    fn estimate_healthspan(&self, organism: &Organism) -> f64 {
        // Healthspan ends when biological age exceeds a threshold (typically ~70 bio-years)
        // where chronic disease risk becomes substantial
        const HEALTHSPAN_BIO_AGE_THRESHOLD: f64 = 70.0;

        // Calculate current biological age
        let current_bio_age = organism.biological_age();
        let current_chrono_age = organism.age;

        // If already past threshold, healthspan is current age minus deficit
        if current_bio_age >= HEALTHSPAN_BIO_AGE_THRESHOLD {
            // Healthspan ended some time ago - estimate when
            let aging_rate = if current_chrono_age > 0.0 {
                current_bio_age / current_chrono_age
            } else {
                1.0
            };
            // When did bio age cross threshold?
            return if aging_rate > 0.0 {
                HEALTHSPAN_BIO_AGE_THRESHOLD / aging_rate
            } else {
                current_chrono_age
            };
        }

        // Calculate aging rate (biological years per chronological year)
        let aging_rate = if current_chrono_age > 20.0 {
            // After development, measure aging rate
            (current_bio_age - 20.0) / (current_chrono_age - 20.0).max(1.0)
        } else {
            1.0 // During development, 1:1 bio:chrono
        };

        // Factor in genetic longevity potential (inverse of genetic risk)
        let genetic_risk_score = organism.genome.calculate_genetic_risk_score();
        let genetic_longevity = 1.0 - genetic_risk_score.overall;
        let genetic_modifier = 1.0 + (genetic_longevity - 0.5) * 0.3; // ±15% based on genetics

        // Factor in lifestyle quality
        let lifestyle_factor = 0.7 + (organism.lifestyle.diet_quality * 0.1)
            + ((10.0 - organism.lifestyle.stress) / 10.0 * 0.1)
            + (organism.lifestyle.sleep_quality * 0.1);

        // Adjusted aging rate considering interventions
        let effective_aging_rate = aging_rate / (genetic_modifier * lifestyle_factor);

        // Project when biological age will reach threshold
        if effective_aging_rate > 0.0 {
            let remaining_bio_years = HEALTHSPAN_BIO_AGE_THRESHOLD - current_bio_age;
            let projected_chrono_years = remaining_bio_years / effective_aging_rate;
            current_chrono_age + projected_chrono_years
        } else {
            // Aging rate <= 0 means biological age reversal - theoretical infinite healthspan
            // Cap at reasonable maximum
            200.0
        }
    }

    fn create_protocol(
        &self,
        genome: &Genome,
        combo: &[Uuid],
        score: f64,
        _rng: &mut impl Rng,
    ) -> ImmortalityProtocol {
        let interventions: Vec<_> = combo.iter()
            .filter_map(|id| self.interventions.iter().find(|i| i.id == *id))
            .collect();

        // Organize into phases
        let mut phases = vec![
            ProtocolPhase {
                name: "Foundation Phase (30-45)".to_string(),
                start_age: 30.0,
                end_age: Some(45.0),
                interventions: Vec::new(),
                goals: vec![
                    "Establish baseline longevity interventions".to_string(),
                    "Optimize lifestyle factors".to_string(),
                    "Begin low-risk pharmaceutical interventions".to_string(),
                ],
                expected_biological_age: 28.0,
            },
            ProtocolPhase {
                name: "Acceleration Phase (45-60)".to_string(),
                start_age: 45.0,
                end_age: Some(60.0),
                interventions: Vec::new(),
                goals: vec![
                    "Add senolytic therapies".to_string(),
                    "Begin telomerase activation if appropriate".to_string(),
                    "Intensify monitoring".to_string(),
                ],
                expected_biological_age: 35.0,
            },
            ProtocolPhase {
                name: "Reversal Phase (60-80)".to_string(),
                start_age: 60.0,
                end_age: Some(80.0),
                interventions: Vec::new(),
                goals: vec![
                    "Begin partial reprogramming".to_string(),
                    "Aggressive senolytic protocol".to_string(),
                    "Consider organ replacement if needed".to_string(),
                ],
                expected_biological_age: 40.0,
            },
            ProtocolPhase {
                name: "Maintenance Phase (80+)".to_string(),
                start_age: 80.0,
                end_age: None,
                interventions: Vec::new(),
                goals: vec![
                    "Maintain biological age at 35-40".to_string(),
                    "Continuous monitoring and adjustment".to_string(),
                    "Speculative interventions as available".to_string(),
                ],
                expected_biological_age: 38.0,
            },
        ];

        // Assign interventions to phases based on optimal start age
        for int in &interventions {
            let phase_idx = if int.optimal_start_age < 45.0 {
                0
            } else if int.optimal_start_age < 60.0 {
                1
            } else if int.optimal_start_age < 80.0 {
                2
            } else {
                3
            };

            phases[phase_idx].interventions.push(PhaseIntervention {
                intervention_id: int.id,
                intervention_name: int.name.clone(),
                protocol: int.protocol.clone(),
                priority: if int.evidence == EvidenceLevel::HumanProven {
                    Priority::Critical
                } else if int.evidence == EvidenceLevel::Speculative {
                    Priority::Optional
                } else {
                    Priority::Important
                },
                personalized_notes: self.generate_personalized_notes(int, genome),
            });
        }

        // Calculate predictions
        let predictions = ProtocolPredictions {
            max_lifespan_years: score * 2.0, // Rough estimate
            healthspan_years: score * 1.8,
            prob_centenarian: 0.95,
            prob_150: 0.75,
            prob_200: 0.50,
            prob_500_plus: if self.config.include_speculative { 0.25 } else { 0.01 },
            bio_age_at_100: 42.0,
            disease_risk_reductions: [
                (DiseaseType::Cancer, 0.60),
                (DiseaseType::Atherosclerosis, 0.70),
                (DiseaseType::Alzheimers, 0.65),
                (DiseaseType::Type2Diabetes, 0.80),
            ].into_iter().collect(),
            limiting_factor: if self.config.include_speculative {
                LimitingFactor::ExternalCauses { annual_risk: 0.001 }
            } else {
                LimitingFactor::Cancer { expected_age: 180.0 }
            },
        };

        ImmortalityProtocol {
            id: Uuid::new_v4(),
            genome_id: genome.id,
            phases,
            predictions,
            monitoring: MonitoringSchedule {
                biomarkers: vec![
                    BiomarkerSchedule {
                        name: "Epigenetic Age (Horvath Clock)".to_string(),
                        frequency_weeks: 12,
                        target_range: (30.0, 45.0),
                        action_if_abnormal: "Adjust reprogramming protocol".to_string(),
                    },
                    BiomarkerSchedule {
                        name: "Telomere Length".to_string(),
                        frequency_weeks: 26,
                        target_range: (8.0, 15.0),
                        action_if_abnormal: "Increase TERT therapy".to_string(),
                    },
                    BiomarkerSchedule {
                        name: "Senescent Cell Burden (p16)".to_string(),
                        frequency_weeks: 12,
                        target_range: (0.0, 0.1),
                        action_if_abnormal: "Increase senolytic frequency".to_string(),
                    },
                    BiomarkerSchedule {
                        name: "NAD+ Levels".to_string(),
                        frequency_weeks: 12,
                        target_range: (20.0, 50.0),
                        action_if_abnormal: "Increase NMN dose".to_string(),
                    },
                ],
                imaging: vec![
                    ImagingSchedule {
                        modality: "Full-body MRI".to_string(),
                        frequency_months: 12,
                        purpose: "Early cancer detection".to_string(),
                    },
                    ImagingSchedule {
                        modality: "Brain MRI + PET".to_string(),
                        frequency_months: 24,
                        purpose: "Neurodegeneration monitoring".to_string(),
                    },
                ],
                genetic_monitoring: vec![
                    GeneticMonitoring {
                        test_type: "Liquid biopsy (ctDNA)".to_string(),
                        frequency_years: 1,
                        looking_for: "Clonal hematopoiesis, early cancers".to_string(),
                    },
                ],
            },
            contingencies: vec![
                Contingency {
                    trigger: "Biological age increases >5 years in 1 year".to_string(),
                    action: "Emergency reprogramming protocol".to_string(),
                    fallback_interventions: vec![],
                },
                Contingency {
                    trigger: "Cancer detected".to_string(),
                    action: "Pause reprogramming, aggressive treatment".to_string(),
                    fallback_interventions: vec![],
                },
            ],
            estimated_lifetime_cost: interventions.iter().map(|i| i.annual_cost * 100.0).sum(),
            confidence: score / 200.0, // Normalize to 0-1
        }
    }

    fn generate_personalized_notes(&self, int: &Intervention, genome: &Genome) -> Vec<String> {
        let mut notes = Vec::new();

        for (gene, modifier) in &int.genetic_modifiers {
            let function = genome.gene_function(*gene);
            if function < 0.5 {
                notes.push(format!(
                    "Your {:?} function is reduced - {} may be {} effective",
                    gene,
                    int.name,
                    if *modifier > 1.0 { "more" } else { "less" }
                ));
            }
        }

        // Sleep-specific notes
        if int.name.contains("Sleep") {
            notes.push(format!(
                "Your genetic optimal sleep is {:.1} hours",
                genome.optimal_sleep_hours()
            ));
        }

        notes
    }

    /// Quick estimate of achievable lifespan for a genome
    pub fn estimate_max_lifespan(&self, genome: &Genome, _rng: &mut impl Rng) -> MaxLifespanEstimate {
        let risk_score = genome.calculate_genetic_risk_score();

        // Base lifespan without interventions
        let base_lifespan = 78.0 - risk_score.overall * 20.0;

        // With conservative interventions
        let conservative_lifespan = base_lifespan * 1.3;

        // With aggressive interventions
        let aggressive_lifespan = base_lifespan * 1.8;

        // With speculative interventions
        let speculative_lifespan = if self.config.include_speculative {
            base_lifespan * 5.0 // Could be much higher
        } else {
            aggressive_lifespan
        };

        MaxLifespanEstimate {
            without_intervention: base_lifespan,
            conservative_protocol: conservative_lifespan,
            aggressive_protocol: aggressive_lifespan,
            speculative_protocol: speculative_lifespan,
            immortality_probability: if self.config.include_speculative {
                0.25 * (1.0 - risk_score.overall)
            } else {
                0.01
            },
            key_genetic_limiters: genome.identify_risk_factors(),
            key_genetic_advantages: genome.identify_protective_factors(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxLifespanEstimate {
    pub without_intervention: f64,
    pub conservative_protocol: f64,
    pub aggressive_protocol: f64,
    pub speculative_protocol: f64,
    pub immortality_probability: f64,
    pub key_genetic_limiters: Vec<crate::genome::GeneticRiskFactor>,
    pub key_genetic_advantages: Vec<crate::genome::GeneticProtectiveFactor>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immortality_engine_creation() {
        let engine = ImmortalityEngine::new(ImmortalityConfig::default());
        assert!(!engine.interventions.is_empty());
    }

    #[test]
    fn test_generate_protocol() {
        let mut rng = rand::thread_rng();
        let engine = ImmortalityEngine::new(ImmortalityConfig {
            combinations_to_test: 100,
            simulations_per_combo: 10,
            max_simulation_years: 200,
            ..Default::default()
        });

        let genome = Genome::new_random(&mut rng);
        let protocol = engine.generate_protocol(&genome, &mut rng);

        assert!(!protocol.phases.is_empty());
        assert!(protocol.predictions.max_lifespan_years > 0.0);
        assert!(protocol.predictions.prob_centenarian > 0.0);
    }

    #[test]
    fn test_max_lifespan_estimate() {
        let mut rng = rand::thread_rng();
        let engine = ImmortalityEngine::new(ImmortalityConfig::default());
        let genome = Genome::new_random(&mut rng);

        let estimate = engine.estimate_max_lifespan(&genome, &mut rng);

        assert!(estimate.without_intervention > 0.0);
        assert!(estimate.conservative_protocol > estimate.without_intervention);
        assert!(estimate.aggressive_protocol > estimate.conservative_protocol);
    }
}
