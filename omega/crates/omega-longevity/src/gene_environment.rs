//! # Gene-Environment Interaction Analysis
//!
//! Models how environmental exposures (smoking, alcohol, diet) interact with
//! genetic variants to affect disease risk. Key use case: **VUS classification
//! via environmental stress testing**.
//!
//! ```text
//!  ╔══════════════════════════════════════════════════════════════════════════╗
//!  ║           GENE-ENVIRONMENT INTERACTION ANALYSIS                          ║
//!  ╠══════════════════════════════════════════════════════════════════════════╣
//!  ║                                                                          ║
//!  ║   THE KEY INSIGHT:                                                       ║
//!  ║   Some genetic variants only reveal their effect under environmental    ║
//!  ║   "stress" - like a DNA repair variant that's fine normally but         ║
//!  ║   becomes pathogenic when exposed to carcinogens.                        ║
//!  ║                                                                          ║
//!  ║   ┌─────────────────────────────────────────────────────────────────┐   ║
//!  ║   │                    4-WAY SCENARIO MATRIX                         │   ║
//!  ║   │                                                                  │   ║
//!  ║   │                     Gene Normal      Gene Variant               │   ║
//!  ║   │                    ┌────────────┬────────────────┐              │   ║
//!  ║   │   No Exposure      │  BASELINE  │  GENETIC RISK  │              │   ║
//!  ║   │                    │   (12%)    │     (80%)      │              │   ║
//!  ║   │                    ├────────────┼────────────────┤              │   ║
//!  ║   │   With Exposure    │ LIFESTYLE  │  SYNERGISTIC   │              │   ║
//!  ║   │   (smoking)        │   RISK     │    RISK        │              │   ║
//!  ║   │                    │   (15%)    │    (92%!)      │              │   ║
//!  ║   │                    └────────────┴────────────────┘              │   ║
//!  ║   │                                                                  │   ║
//!  ║   │   If interaction effect > additive → SYNERGY                    │   ║
//!  ║   │   This helps classify VUS as pathogenic!                        │   ║
//!  ║   └─────────────────────────────────────────────────────────────────┘   ║
//!  ║                                                                          ║
//!  ║   ENVIRONMENTAL STRESSORS:                                              ║
//!  ║   • Smoking (tobacco carcinogens → DNA damage)                          ║
//!  ║   • Alcohol (acetaldehyde → DNA crosslinks)                             ║
//!  ║   • UV radiation (pyrimidine dimers)                                    ║
//!  ║   • Processed meat (nitrosamines)                                       ║
//!  ║   • Chronic inflammation                                                 ║
//!  ║                                                                          ║
//!  ║   GENE CATEGORIES:                                                       ║
//!  ║   • DNA Repair: BRCA1/2, ATM, CHEK2, PALB2 (repair damage)             ║
//!  ║   • Detoxification: CYP1A1, GSTM1, NAT2 (process toxins)               ║
//!  ║   • Tumor Suppressors: TP53, RB1, APC (prevent cancer)                  ║
//!  ║   • Cell Cycle: CDKN2A, CDK4 (control division)                         ║
//!  ║                                                                          ║
//!  ╚══════════════════════════════════════════════════════════════════════════╝
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use crate::genome::{Genome, Gene, GeneState, GeneVariant, VariantEffect};
use crate::organism::{Organism, Lifestyle, DiseaseType};
use crate::vus_interpreter::{VariantQuery, VUSInterpretation, ACMGClassification};

/// Gene-Environment Interaction Analyzer
pub struct GeneEnvironmentAnalyzer {
    config: GeneEnvironmentConfig,
    /// Known gene-exposure interactions
    interactions: Vec<GeneExposureInteraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneEnvironmentConfig {
    /// Number of simulations per scenario
    pub simulations_per_scenario: usize,
    /// Maximum age to simulate
    pub max_age: f64,
    /// Significance threshold for interaction detection
    pub significance_threshold: f64,
}

impl Default for GeneEnvironmentConfig {
    fn default() -> Self {
        Self {
            simulations_per_scenario: 1000,
            max_age: 85.0,
            significance_threshold: 0.05,
        }
    }
}

/// An environmental exposure that can interact with genes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalExposure {
    pub name: String,
    pub category: ExposureCategory,
    /// Dose per week (standardized units)
    pub dose_per_week: f64,
    /// DNA damage rate multiplier
    pub dna_damage_rate: f64,
    /// Specific damage types caused
    pub damage_types: Vec<DamageType>,
    /// Organs primarily affected
    pub target_organs: Vec<TargetOrgan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExposureCategory {
    Smoking,
    Alcohol,
    UVRadiation,
    Diet,
    Pollution,
    Infection,
    Medication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    /// Oxidative damage (8-oxoG)
    Oxidative,
    /// Bulky DNA adducts (PAHs from smoke)
    BulkyAdducts,
    /// DNA crosslinks (alcohol, chemotherapy)
    Crosslinks,
    /// Double-strand breaks
    DoubleStrandBreaks,
    /// Alkylation damage
    Alkylation,
    /// Pyrimidine dimers (UV)
    PyrimidineDimers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetOrgan {
    Lung,
    Breast,
    Colon,
    Liver,
    Pancreas,
    Esophagus,
    Bladder,
    Skin,
    Blood,
    Prostate,
    Ovary,
}

/// Known gene-exposure interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneExposureInteraction {
    pub gene: Gene,
    pub exposure_category: ExposureCategory,
    /// How the gene's function modifies exposure effect
    pub interaction_type: InteractionType,
    /// Multiplier when gene is impaired AND exposed
    /// > 1.0 means synergistic (worse than additive)
    /// = 1.0 means additive
    /// < 1.0 means antagonistic (protective)
    pub synergy_factor: f64,
    /// Which cancers this interaction affects
    pub affected_cancers: Vec<DiseaseType>,
    /// Scientific evidence
    pub evidence: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    /// Gene repairs damage caused by exposure
    RepairDeficiency,
    /// Gene detoxifies the exposure
    DetoxificationDeficiency,
    /// Gene suppresses tumors from this exposure
    TumorSuppressionLoss,
    /// Gene controls cell cycle under stress
    CellCycleDefect,
    /// Gene affects immune surveillance
    ImmuneDeficiency,
}

/// Result of 4-way scenario analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioAnalysis {
    pub id: Uuid,
    pub gene: Gene,
    pub variant_description: String,
    pub exposure: EnvironmentalExposure,

    /// The 4 scenarios
    pub scenarios: FourWayScenarios,

    /// Interaction statistics
    pub interaction: InteractionStatistics,

    /// VUS classification recommendation
    pub vus_recommendation: Option<VUSRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourWayScenarios {
    /// Gene normal, no exposure (baseline)
    pub baseline: ScenarioResult,
    /// Gene normal, with exposure (lifestyle risk only)
    pub exposure_only: ScenarioResult,
    /// Gene variant, no exposure (genetic risk only)
    pub variant_only: ScenarioResult,
    /// Gene variant + exposure (combined/interaction)
    pub variant_plus_exposure: ScenarioResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub name: String,
    pub cancer_risk: f64,
    pub mean_onset_age: Option<f64>,
    pub simulations: usize,
    /// 95% confidence interval
    pub confidence_interval: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionStatistics {
    /// Expected risk if additive (baseline + exposure_effect + variant_effect)
    pub expected_additive_risk: f64,
    /// Observed risk with both factors
    pub observed_combined_risk: f64,
    /// Interaction effect (observed - expected)
    pub interaction_effect: f64,
    /// Relative excess risk due to interaction (RERI)
    pub reri: f64,
    /// Synergy index (SI)
    pub synergy_index: f64,
    /// P-value for interaction
    pub p_value: f64,
    /// Is the interaction statistically significant?
    pub significant: bool,
    /// Direction of interaction
    pub interaction_direction: InteractionDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionDirection {
    /// Worse than additive - gene variant makes exposure MORE dangerous
    Synergistic,
    /// Exactly additive - no interaction
    Additive,
    /// Better than additive - gene variant makes exposure LESS dangerous
    Antagonistic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VUSRecommendation {
    /// Recommended ACMG classification
    pub classification: ACMGClassification,
    /// Confidence in recommendation
    pub confidence: f64,
    /// Evidence supporting classification
    pub evidence: Vec<String>,
    /// Key finding
    pub key_finding: String,
}

impl GeneEnvironmentAnalyzer {
    pub fn new(config: GeneEnvironmentConfig) -> Self {
        let mut analyzer = Self {
            config,
            interactions: Vec::new(),
        };
        analyzer.load_known_interactions();
        analyzer
    }

    fn load_known_interactions(&mut self) {
        // BRCA1 - DNA double-strand break repair
        self.interactions.push(GeneExposureInteraction {
            gene: Gene::BRCA1,
            exposure_category: ExposureCategory::Smoking,
            interaction_type: InteractionType::RepairDeficiency,
            synergy_factor: 1.4, // 40% worse than additive
            affected_cancers: vec![DiseaseType::Cancer],
            evidence: "BRCA1 repairs DSBs; smoking causes DSBs; impaired BRCA1 + smoking = synergistic".to_string(),
        });

        self.interactions.push(GeneExposureInteraction {
            gene: Gene::BRCA1,
            exposure_category: ExposureCategory::Alcohol,
            interaction_type: InteractionType::RepairDeficiency,
            synergy_factor: 1.3,
            affected_cancers: vec![DiseaseType::Cancer],
            evidence: "Alcohol metabolite acetaldehyde causes DNA crosslinks requiring BRCA1-mediated repair".to_string(),
        });

        // BRCA2 - Also DNA repair
        self.interactions.push(GeneExposureInteraction {
            gene: Gene::BRCA2,
            exposure_category: ExposureCategory::Smoking,
            interaction_type: InteractionType::RepairDeficiency,
            synergy_factor: 1.35,
            affected_cancers: vec![DiseaseType::Cancer],
            evidence: "BRCA2 in homologous recombination; smoking-induced damage accumulates without repair".to_string(),
        });

        // ATM - DNA damage response
        self.interactions.push(GeneExposureInteraction {
            gene: Gene::ATM,
            exposure_category: ExposureCategory::Smoking,
            interaction_type: InteractionType::RepairDeficiency,
            synergy_factor: 1.5,
            affected_cancers: vec![DiseaseType::Cancer],
            evidence: "ATM coordinates DNA damage response; impaired ATM fails to arrest cell cycle after smoking damage".to_string(),
        });

        // TP53 - Guardian of the genome
        self.interactions.push(GeneExposureInteraction {
            gene: Gene::TP53,
            exposure_category: ExposureCategory::Smoking,
            interaction_type: InteractionType::TumorSuppressionLoss,
            synergy_factor: 2.0, // Very synergistic - p53 is critical
            affected_cancers: vec![DiseaseType::Cancer],
            evidence: "TP53 triggers apoptosis of damaged cells; loss + carcinogen = damaged cells survive and proliferate".to_string(),
        });

        self.interactions.push(GeneExposureInteraction {
            gene: Gene::TP53,
            exposure_category: ExposureCategory::UVRadiation,
            interaction_type: InteractionType::TumorSuppressionLoss,
            synergy_factor: 2.5,
            affected_cancers: vec![DiseaseType::Cancer],
            evidence: "UV causes pyrimidine dimers; TP53 normally eliminates damaged keratinocytes".to_string(),
        });

        // RB1 - Cell cycle checkpoint/tumor suppressor
        self.interactions.push(GeneExposureInteraction {
            gene: Gene::RB1,
            exposure_category: ExposureCategory::Smoking,
            interaction_type: InteractionType::CellCycleDefect,
            synergy_factor: 1.25,
            affected_cancers: vec![DiseaseType::Cancer],
            evidence: "RB1 controls cell cycle G1/S checkpoint; loss allows damaged cells to divide".to_string(),
        });
    }

    /// Create common environmental exposures
    pub fn smoking_exposure(cigars_per_week: f64) -> EnvironmentalExposure {
        EnvironmentalExposure {
            name: format!("{} cigars/week", cigars_per_week),
            category: ExposureCategory::Smoking,
            dose_per_week: cigars_per_week,
            // 1 cigar ≈ 1-8 cigarettes worth of tobacco
            // DNA damage scales with dose
            dna_damage_rate: 1.0 + (cigars_per_week * 0.05), // 5% more damage per cigar/week
            damage_types: vec![
                DamageType::BulkyAdducts,      // PAHs
                DamageType::Oxidative,          // ROS
                DamageType::Alkylation,         // Nitrosamines
            ],
            target_organs: vec![
                TargetOrgan::Lung,
                TargetOrgan::Esophagus,
                TargetOrgan::Bladder,
                TargetOrgan::Pancreas,
            ],
        }
    }

    pub fn alcohol_exposure(drinks_per_week: f64) -> EnvironmentalExposure {
        EnvironmentalExposure {
            name: format!("{} drinks/week", drinks_per_week),
            category: ExposureCategory::Alcohol,
            dose_per_week: drinks_per_week,
            // Moderate drinking (7-14/week) increases cancer risk ~10-20%
            dna_damage_rate: 1.0 + (drinks_per_week * 0.01), // 1% more damage per drink/week
            damage_types: vec![
                DamageType::Crosslinks,         // Acetaldehyde
                DamageType::Oxidative,          // ROS from metabolism
            ],
            target_organs: vec![
                TargetOrgan::Liver,
                TargetOrgan::Breast,
                TargetOrgan::Colon,
                TargetOrgan::Esophagus,
            ],
        }
    }

    /// Run full 4-way scenario analysis
    pub fn analyze_gene_environment(
        &self,
        gene: Gene,
        variant: &GeneVariant,
        exposure: EnvironmentalExposure,
        rng: &mut impl Rng,
    ) -> ScenarioAnalysis {
        // Scenario 1: Baseline (normal gene, no exposure)
        let baseline = self.simulate_scenario(
            gene,
            None, // No variant
            None, // No exposure
            "Baseline (no variant, no exposure)",
            rng,
        );

        // Scenario 2: Exposure only (normal gene, with exposure)
        let exposure_only = self.simulate_scenario(
            gene,
            None,
            Some(&exposure),
            &format!("Exposure only ({})", exposure.name),
            rng,
        );

        // Scenario 3: Variant only (variant, no exposure)
        let variant_only = self.simulate_scenario(
            gene,
            Some(variant),
            None,
            &format!("Variant only ({:?})", variant.effect),
            rng,
        );

        // Scenario 4: Both variant and exposure
        let variant_plus_exposure = self.simulate_scenario(
            gene,
            Some(variant),
            Some(&exposure),
            &format!("Variant + Exposure ({} + {})",
                     format!("{:?}", variant.effect), exposure.name),
            rng,
        );

        let scenarios = FourWayScenarios {
            baseline,
            exposure_only,
            variant_only,
            variant_plus_exposure,
        };

        // Calculate interaction statistics
        let interaction = self.calculate_interaction(&scenarios);

        // Generate VUS recommendation if applicable
        let vus_recommendation = self.generate_vus_recommendation(
            gene, variant, &exposure, &scenarios, &interaction
        );

        ScenarioAnalysis {
            id: Uuid::new_v4(),
            gene,
            variant_description: format!("{:?}", variant.effect),
            exposure,
            scenarios,
            interaction,
            vus_recommendation,
        }
    }

    fn simulate_scenario(
        &self,
        gene: Gene,
        variant: Option<&GeneVariant>,
        exposure: Option<&EnvironmentalExposure>,
        name: &str,
        rng: &mut impl Rng,
    ) -> ScenarioResult {
        let mut cancer_count = 0;
        let mut onset_ages = Vec::new();

        for _ in 0..self.config.simulations_per_scenario {
            let mut organism = Organism::new_random(rng);

            // Apply variant if present
            if let Some(v) = variant {
                if let Some(gene_state) = organism.genome.nuclear_genes.get_mut(&gene) {
                    gene_state.variants.push(v.clone());
                    // Reduce function based on variant effect
                    gene_state.expression *= match v.effect {
                        VariantEffect::LossOfFunction => 0.0,
                        VariantEffect::ReducedFunction => 0.3,
                        VariantEffect::Neutral => 1.0,
                        VariantEffect::EnhancedFunction => 1.2,
                        VariantEffect::GainOfFunction => 1.5,
                    };
                }
            }

            // Apply exposure if present
            if let Some(exp) = exposure {
                match exp.category {
                    ExposureCategory::Smoking => {
                        organism.lifestyle.smoking = (exp.dose_per_week * 5.0) as u32; // Convert to cigarette equivalents
                    }
                    ExposureCategory::Alcohol => {
                        organism.lifestyle.alcohol = exp.dose_per_week as u32;
                    }
                    _ => {}
                }
            }

            // Simulate life
            organism.simulate_life(rng);

            // Check for cancer
            let got_cancer = organism.diseases.iter()
                .any(|d| d.disease_type == DiseaseType::Cancer);

            if got_cancer {
                cancer_count += 1;
                if let Some(cancer) = organism.diseases.iter()
                    .find(|d| d.disease_type == DiseaseType::Cancer)
                {
                    onset_ages.push(cancer.onset_age);
                }
            }
        }

        let n = self.config.simulations_per_scenario as f64;
        let risk = cancer_count as f64 / n;

        // Calculate 95% CI using Wilson score interval
        let z = 1.96;
        let denominator = 1.0 + z * z / n;
        let center = (risk + z * z / (2.0 * n)) / denominator;
        let margin = z * (risk * (1.0 - risk) / n + z * z / (4.0 * n * n)).sqrt() / denominator;

        ScenarioResult {
            name: name.to_string(),
            cancer_risk: risk,
            mean_onset_age: if onset_ages.is_empty() {
                None
            } else {
                Some(onset_ages.iter().sum::<f64>() / onset_ages.len() as f64)
            },
            simulations: self.config.simulations_per_scenario,
            confidence_interval: ((center - margin).max(0.0), (center + margin).min(1.0)),
        }
    }

    fn calculate_interaction(&self, scenarios: &FourWayScenarios) -> InteractionStatistics {
        let r00 = scenarios.baseline.cancer_risk;           // Neither
        let r10 = scenarios.exposure_only.cancer_risk;      // Exposure only
        let r01 = scenarios.variant_only.cancer_risk;       // Variant only
        let r11 = scenarios.variant_plus_exposure.cancer_risk; // Both

        // Expected under additivity: R00 + (R10 - R00) + (R01 - R00) = R10 + R01 - R00
        let expected_additive = r10 + r01 - r00;

        // Interaction effect
        let interaction_effect = r11 - expected_additive;

        // RERI (Relative Excess Risk due to Interaction)
        // RERI = RR11 - RR10 - RR01 + 1
        let rr10 = if r00 > 0.0 { r10 / r00 } else { 1.0 };
        let rr01 = if r00 > 0.0 { r01 / r00 } else { 1.0 };
        let rr11 = if r00 > 0.0 { r11 / r00 } else { 1.0 };
        let reri = rr11 - rr10 - rr01 + 1.0;

        // Synergy Index: SI = (RR11 - 1) / ((RR10 - 1) + (RR01 - 1))
        let numerator = rr11 - 1.0;
        let denominator = (rr10 - 1.0) + (rr01 - 1.0);
        let synergy_index = if denominator.abs() > 0.001 {
            numerator / denominator
        } else {
            1.0
        };

        // Simple significance test (would use proper statistical test in production)
        let n = self.config.simulations_per_scenario as f64;
        let se = (r11 * (1.0 - r11) / n).sqrt();
        let z = if se > 0.0 { interaction_effect.abs() / se } else { 0.0 };
        let p_value = 2.0 * (1.0 - normal_cdf(z));

        let significant = p_value < self.config.significance_threshold;

        let direction = if interaction_effect > 0.02 && significant {
            InteractionDirection::Synergistic
        } else if interaction_effect < -0.02 && significant {
            InteractionDirection::Antagonistic
        } else {
            InteractionDirection::Additive
        };

        InteractionStatistics {
            expected_additive_risk: expected_additive,
            observed_combined_risk: r11,
            interaction_effect,
            reri,
            synergy_index,
            p_value,
            significant,
            interaction_direction: direction,
        }
    }

    fn generate_vus_recommendation(
        &self,
        gene: Gene,
        variant: &GeneVariant,
        exposure: &EnvironmentalExposure,
        scenarios: &FourWayScenarios,
        interaction: &InteractionStatistics,
    ) -> Option<VUSRecommendation> {
        // Only generate recommendation for potential VUS (reduced function or neutral variants)
        // In a real system, we'd have a separate VUS flag
        if variant.effect == VariantEffect::LossOfFunction || variant.effect == VariantEffect::GainOfFunction {
            return None; // Already classified
        }

        let mut evidence = Vec::new();
        let mut pathogenic_points: f64 = 0.0;
        let mut benign_points: f64 = 0.0;

        // Evidence 1: Does variant alone increase risk?
        let variant_effect = scenarios.variant_only.cancer_risk - scenarios.baseline.cancer_risk;
        if variant_effect > 0.10 {
            evidence.push(format!(
                "Variant alone increases cancer risk by {:.1}% (significant genetic effect)",
                variant_effect * 100.0
            ));
            pathogenic_points += 0.3;
        } else if variant_effect < 0.02 {
            evidence.push("Variant alone shows minimal effect on cancer risk".to_string());
            benign_points += 0.2;
        }

        // Evidence 2: Is there synergistic interaction with environmental exposure?
        if interaction.interaction_direction == InteractionDirection::Synergistic {
            evidence.push(format!(
                "SYNERGISTIC INTERACTION: Variant + {} increases risk {:.1}% MORE than expected additively",
                exposure.name,
                interaction.interaction_effect * 100.0
            ));
            pathogenic_points += 0.4;

            // This is the KEY finding for VUS classification!
            if interaction.reri > 0.5 {
                evidence.push(format!(
                    "Strong synergy (RERI={:.2}) suggests variant impairs {:?}-specific pathway",
                    interaction.reri,
                    exposure.category
                ));
                pathogenic_points += 0.2;
            }
        } else if interaction.interaction_direction == InteractionDirection::Additive {
            evidence.push("No significant gene-environment interaction detected".to_string());
        }

        // Evidence 3: Functional consistency with gene role
        let known_interaction = self.interactions.iter()
            .find(|i| i.gene == gene && i.exposure_category == exposure.category);

        if let Some(ki) = known_interaction {
            if interaction.synergy_index > 1.0 {
                evidence.push(format!(
                    "Interaction pattern consistent with known {} function",
                    format!("{:?}", ki.interaction_type)
                ));
                pathogenic_points += 0.15;
            }
        }

        // Evidence 4: Earlier onset with exposure
        if let (Some(onset_variant), Some(onset_combo)) = (
            scenarios.variant_only.mean_onset_age,
            scenarios.variant_plus_exposure.mean_onset_age
        ) {
            let onset_acceleration = onset_variant - onset_combo;
            if onset_acceleration > 5.0 {
                evidence.push(format!(
                    "Exposure accelerates cancer onset by {:.1} years in variant carriers",
                    onset_acceleration
                ));
                pathogenic_points += 0.15;
            }
        }

        // Determine classification
        let total = pathogenic_points + benign_points;
        let pathogenic_fraction = if total > 0.0 { pathogenic_points / total } else { 0.5 };

        let (classification, key_finding) = if pathogenic_points > 0.5 && pathogenic_fraction > 0.7 {
            if interaction.interaction_direction == InteractionDirection::Synergistic {
                (ACMGClassification::LikelyPathogenic,
                 format!("VUS shows SYNERGISTIC interaction with {} - suggests functional impairment",
                         exposure.name))
            } else {
                (ACMGClassification::LikelyPathogenic,
                 "VUS independently increases cancer risk significantly".to_string())
            }
        } else if benign_points > 0.4 && pathogenic_fraction < 0.3 {
            (ACMGClassification::LikelyBenign,
             "VUS shows minimal effect even under environmental stress".to_string())
        } else {
            (ACMGClassification::VUS,
             "Insufficient evidence for classification; recommend additional functional studies".to_string())
        };

        Some(VUSRecommendation {
            classification,
            confidence: (pathogenic_points - benign_points).abs().min(1.0),
            evidence,
            key_finding,
        })
    }

    /// Quick comparison: smoking vs drinking for a specific gene variant
    pub fn compare_exposures(
        &self,
        gene: Gene,
        variant: &GeneVariant,
        rng: &mut impl Rng,
    ) -> ExposureComparison {
        let smoking = Self::smoking_exposure(1.0); // 1 cigar/week
        let alcohol = Self::alcohol_exposure(2.0); // 2 drinks/week

        let smoking_analysis = self.analyze_gene_environment(gene, variant, smoking.clone(), rng);
        let alcohol_analysis = self.analyze_gene_environment(gene, variant, alcohol.clone(), rng);

        ExposureComparison {
            gene,
            variant_description: format!("{:?}", variant.effect),
            smoking_analysis,
            alcohol_analysis,
            recommendation: self.generate_comparison_recommendation(gene),
        }
    }

    fn generate_comparison_recommendation(&self, gene: Gene) -> String {
        let smoking_interaction = self.interactions.iter()
            .find(|i| i.gene == gene && i.exposure_category == ExposureCategory::Smoking);
        let alcohol_interaction = self.interactions.iter()
            .find(|i| i.gene == gene && i.exposure_category == ExposureCategory::Alcohol);

        match (smoking_interaction, alcohol_interaction) {
            (Some(s), Some(a)) if s.synergy_factor > a.synergy_factor => {
                format!("Smoking is MORE dangerous than alcohol for {:?} variant carriers (synergy {:.1}x vs {:.1}x)",
                        gene, s.synergy_factor, a.synergy_factor)
            }
            (Some(s), Some(a)) if a.synergy_factor > s.synergy_factor => {
                format!("Alcohol is MORE dangerous than smoking for {:?} variant carriers (synergy {:.1}x vs {:.1}x)",
                        gene, a.synergy_factor, s.synergy_factor)
            }
            (Some(_), Some(_)) => {
                format!("Both smoking and alcohol are dangerous for {:?} variant carriers", gene)
            }
            (Some(s), None) => {
                format!("Smoking has known synergy with {:?} ({:.1}x), alcohol interaction unknown",
                        gene, s.synergy_factor)
            }
            (None, Some(a)) => {
                format!("Alcohol has known synergy with {:?} ({:.1}x), smoking interaction unknown",
                        gene, a.synergy_factor)
            }
            (None, None) => {
                format!("No known gene-environment interactions for {:?}", gene)
            }
        }
    }
}

/// Comparison of multiple exposures for same variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureComparison {
    pub gene: Gene,
    pub variant_description: String,
    pub smoking_analysis: ScenarioAnalysis,
    pub alcohol_analysis: ScenarioAnalysis,
    pub recommendation: String,
}

/// Helper: Standard normal CDF approximation
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Error function approximation
fn erf(x: f64) -> f64 {
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = GeneEnvironmentAnalyzer::new(GeneEnvironmentConfig::default());
        assert!(!analyzer.interactions.is_empty());
    }

    #[test]
    fn test_smoking_exposure() {
        let exposure = GeneEnvironmentAnalyzer::smoking_exposure(1.0);
        assert_eq!(exposure.category, ExposureCategory::Smoking);
        assert!(exposure.dna_damage_rate > 1.0);
    }

    #[test]
    fn test_alcohol_exposure() {
        let exposure = GeneEnvironmentAnalyzer::alcohol_exposure(2.0);
        assert_eq!(exposure.category, ExposureCategory::Alcohol);
        assert!(exposure.dna_damage_rate > 1.0);
    }

    #[test]
    fn test_scenario_analysis() {
        let mut rng = rand::thread_rng();
        let analyzer = GeneEnvironmentAnalyzer::new(GeneEnvironmentConfig {
            simulations_per_scenario: 100, // Fewer for test speed
            ..Default::default()
        });

        let variant = GeneVariant {
            position: 12345,
            reference: "G".to_string(),
            alternate: "A".to_string(),
            zygosity: crate::genome::Zygosity::Heterozygous,
            effect: VariantEffect::ReducedFunction,
            clinical_significance: None,
        };

        let smoking = GeneEnvironmentAnalyzer::smoking_exposure(1.0);
        let analysis = analyzer.analyze_gene_environment(Gene::BRCA1, &variant, smoking, &mut rng);

        assert!(analysis.scenarios.baseline.cancer_risk >= 0.0);
        assert!(analysis.scenarios.baseline.cancer_risk <= 1.0);
    }
}
