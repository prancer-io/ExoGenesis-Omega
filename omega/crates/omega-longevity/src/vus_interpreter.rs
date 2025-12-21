//! VUS Interpreter - Classify Variants of Unknown Significance
//!
//! When genetic testing returns a VUS (Variant of Unknown Significance),
//! this module simulates the variant's functional effect across thousands
//! of virtual lifespans to predict pathogenicity and longevity impact.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────────┐
//!  │                      VUS INTERPRETATION                                 │
//!  ├─────────────────────────────────────────────────────────────────────────┤
//!  │                                                                         │
//!  │  PATIENT VUS                                                            │
//!  │  ┌────────────────────────────────────────────────────────────────┐    │
//!  │  │  Gene: BRCA1                                                    │    │
//!  │  │  Variant: c.1234A>G (p.Lys412Arg)                              │    │
//!  │  │  Classification: VUS (Unknown significance)                     │    │
//!  │  │  Question: Is this pathogenic for longevity/cancer?            │    │
//!  │  └────────────────────────────────────────────────────────────────┘    │
//!  │                              │                                          │
//!  │                              ▼                                          │
//!  │  ┌────────────────────────────────────────────────────────────────┐    │
//!  │  │                 FUNCTIONAL MODELING                             │    │
//!  │  │                                                                 │    │
//!  │  │  Model variant effect:                                         │    │
//!  │  │  • Protein structure impact → function prediction              │    │
//!  │  │  • Conservation analysis → evolutionary constraint             │    │
//!  │  │  • Domain analysis → critical region?                          │    │
//!  │  │  → Predicted: 30% reduction in DNA repair capacity             │    │
//!  │  └────────────────────────────────────────────────────────────────┘    │
//!  │                              │                                          │
//!  │                              ▼                                          │
//!  │  ┌────────────────────────────────────────────────────────────────┐    │
//!  │  │              POPULATION SIMULATION (10,000 lives)               │    │
//!  │  │                                                                 │    │
//!  │  │  WITH VARIANT              WITHOUT VARIANT                     │    │
//!  │  │  ┌──────────────┐         ┌──────────────┐                    │    │
//!  │  │  │ 5,000 lives  │         │ 5,000 lives  │                    │    │
//!  │  │  │ Mean: 71.3 y │   vs    │ Mean: 78.2 y │                    │    │
//!  │  │  │ Cancer: 34%  │         │ Cancer: 12%  │                    │    │
//!  │  │  └──────────────┘         └──────────────┘                    │    │
//!  │  └────────────────────────────────────────────────────────────────┘    │
//!  │                              │                                          │
//!  │                              ▼                                          │
//!  │  ┌────────────────────────────────────────────────────────────────┐    │
//!  │  │                    INTERPRETATION                               │    │
//!  │  │                                                                 │    │
//!  │  │  Predicted Classification: LIKELY PATHOGENIC                   │    │
//!  │  │  Confidence: 87%                                                │    │
//!  │  │                                                                 │    │
//!  │  │  Evidence:                                                      │    │
//!  │  │  • -6.9 years lifespan effect (p < 0.001)                      │    │
//!  │  │  • 2.8x cancer risk increase                                   │    │
//!  │  │  • DNA repair pathway significantly impaired                   │    │
//!  │  │  • Effect consistent across genetic backgrounds                │    │
//!  │  │                                                                 │    │
//!  │  │  Clinical Recommendation:                                       │    │
//!  │  │  Consider enhanced cancer screening protocol                    │    │
//!  │  └────────────────────────────────────────────────────────────────┘    │
//!  │                                                                         │
//!  └─────────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use crate::genome::{Genome, Gene, GeneState, GeneVariant, VariantEffect, AgingRole};
use crate::organism::{Organism, Lifestyle, DiseaseType, DeathCause};
use crate::causal_discovery::{PopulationSimulator, PopulationConfig};
use crate::{Result, LongevityError};

/// A variant of unknown significance to interpret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantQuery {
    pub id: Uuid,
    /// Gene affected
    pub gene: Gene,
    /// HGVS notation (e.g., "c.1234A>G")
    pub hgvs_cdna: String,
    /// Protein change (e.g., "p.Lys412Arg")
    pub hgvs_protein: Option<String>,
    /// dbSNP ID if known
    pub rsid: Option<String>,
    /// Amino acid position
    pub aa_position: Option<u32>,
    /// Conservation score (0-1, higher = more conserved)
    pub conservation_score: Option<f64>,
    /// Is this in a known functional domain?
    pub in_functional_domain: bool,
    /// Predicted structural impact (from tools like AlphaFold)
    pub structural_impact: Option<StructuralImpact>,
    /// Population allele frequency (if known)
    pub allele_frequency: Option<f64>,
    /// Any existing evidence
    pub existing_evidence: Vec<ExistingEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralImpact {
    None,
    Minor,
    Moderate,
    Severe,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistingEvidence {
    pub source: String,
    pub classification: Option<ACMGClassification>,
    pub evidence_type: EvidenceType,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    FunctionalStudy,
    CaseReport,
    PopulationData,
    ComputationalPrediction,
    Segregation,
    DeNovo,
}

/// ACMG classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ACMGClassification {
    Pathogenic,
    LikelyPathogenic,
    VUS,
    LikelyBenign,
    Benign,
}

/// Result of VUS interpretation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VUSInterpretation {
    pub id: Uuid,
    pub query: VariantQuery,
    /// Predicted classification
    pub predicted_classification: ACMGClassification,
    /// Confidence in prediction (0-1)
    pub confidence: f64,
    /// Predicted functional effect
    pub functional_effect: PredictedFunctionalEffect,
    /// Simulation results
    pub simulation_results: VUSSimulationResults,
    /// Clinical implications
    pub clinical_implications: Vec<ClinicalImplication>,
    /// Evidence summary
    pub evidence_summary: Vec<EvidenceItem>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedFunctionalEffect {
    /// Effect on gene function (0 = complete loss, 1 = normal, >1 = gain)
    pub function_level: f64,
    /// Effect type
    pub effect_type: VariantEffect,
    /// Which cellular process is affected
    pub affected_process: AgingRole,
    /// Mechanism description
    pub mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VUSSimulationResults {
    /// Number of simulated lives
    pub total_simulations: usize,
    /// Mean lifespan with variant
    pub lifespan_with_variant: f64,
    /// Mean lifespan without variant
    pub lifespan_without_variant: f64,
    /// Lifespan difference
    pub lifespan_delta: f64,
    /// Statistical significance (p-value)
    pub p_value: f64,
    /// Disease risk changes
    pub disease_risks: HashMap<DiseaseType, DiseaseRiskChange>,
    /// Effect consistency across backgrounds
    pub effect_consistency: f64,
    /// Age of effect onset
    pub effect_onset_age: Option<f64>,
    /// Penetrance (% of carriers affected)
    pub penetrance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRiskChange {
    pub baseline_risk: f64,
    pub variant_risk: f64,
    pub relative_risk: f64,
    pub absolute_risk_increase: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalImplication {
    pub implication_type: ClinicalImplicationType,
    pub severity: Severity,
    pub description: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClinicalImplicationType {
    CancerRisk,
    CardiovascularRisk,
    NeurodegenerativeRisk,
    MetabolicRisk,
    LifespanImpact,
    DrugResponse,
    FamilyScreening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub category: String,
    pub strength: f64,
    pub description: String,
    pub supporting: bool, // Supports pathogenicity?
}

/// Configuration for VUS interpretation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VUSConfig {
    /// Number of simulations per condition
    pub simulations_per_arm: usize,
    /// Include lifestyle variation
    pub lifestyle_variation: bool,
    /// Minimum effect size to call significant
    pub min_effect_years: f64,
    /// Confidence threshold for classification
    pub confidence_threshold: f64,
}

impl Default for VUSConfig {
    fn default() -> Self {
        Self {
            simulations_per_arm: 1000,
            lifestyle_variation: true,
            min_effect_years: 1.0,
            confidence_threshold: 0.8,
        }
    }
}

/// The VUS Interpreter
pub struct VUSInterpreter {
    config: VUSConfig,
}

impl VUSInterpreter {
    pub fn new(config: VUSConfig) -> Self {
        Self { config }
    }

    /// Interpret a VUS
    pub fn interpret(&self, query: VariantQuery, rng: &mut impl Rng) -> VUSInterpretation {
        // 1. Predict functional effect from variant features
        let functional_effect = self.predict_functional_effect(&query);

        // 2. Run simulation comparing with/without variant
        let simulation_results = self.simulate_variant_effect(&query, &functional_effect, rng);

        // 3. Determine classification
        let (predicted_classification, confidence) =
            self.determine_classification(&query, &functional_effect, &simulation_results);

        // 4. Generate clinical implications
        let clinical_implications = self.generate_clinical_implications(
            &query, &functional_effect, &simulation_results
        );

        // 5. Compile evidence
        let evidence_summary = self.compile_evidence(
            &query, &functional_effect, &simulation_results
        );

        // 6. Generate recommendations
        let recommendations = self.generate_recommendations(
            &predicted_classification, &clinical_implications
        );

        VUSInterpretation {
            id: Uuid::new_v4(),
            query,
            predicted_classification,
            confidence,
            functional_effect,
            simulation_results,
            clinical_implications,
            evidence_summary,
            recommendations,
        }
    }

    fn predict_functional_effect(&self, query: &VariantQuery) -> PredictedFunctionalEffect {
        let mut function_level = 1.0;
        let mut effect_type = VariantEffect::Neutral;

        // Use conservation to predict impact
        if let Some(conservation) = query.conservation_score {
            if conservation > 0.9 {
                // Highly conserved position - likely damaging
                function_level -= 0.3;
            } else if conservation > 0.7 {
                function_level -= 0.15;
            }
        }

        // Structural impact
        match query.structural_impact {
            Some(StructuralImpact::Severe) => {
                function_level -= 0.5;
                effect_type = VariantEffect::LossOfFunction;
            }
            Some(StructuralImpact::Moderate) => {
                function_level -= 0.25;
                effect_type = VariantEffect::ReducedFunction;
            }
            Some(StructuralImpact::Minor) => {
                function_level -= 0.1;
            }
            _ => {}
        }

        // Functional domain
        if query.in_functional_domain {
            function_level -= 0.15;
        }

        // Population frequency (common = likely benign)
        if let Some(af) = query.allele_frequency {
            if af > 0.01 { // >1% frequency
                function_level += 0.2; // Likely benign
                effect_type = VariantEffect::Neutral;
            }
        }

        function_level = function_level.clamp(0.0, 1.5);

        if function_level < 0.3 {
            effect_type = VariantEffect::LossOfFunction;
        } else if function_level < 0.7 {
            effect_type = VariantEffect::ReducedFunction;
        } else if function_level > 1.1 {
            effect_type = VariantEffect::GainOfFunction;
        }

        let affected_process = query.gene.aging_role();

        let mechanism = match affected_process {
            AgingRole::DNARepair => format!(
                "{:?} variant affects DNA repair capacity → genomic instability",
                query.gene
            ),
            AgingRole::TelomereMaintenance => format!(
                "{:?} variant affects telomere maintenance → accelerated senescence",
                query.gene
            ),
            AgingRole::Senescence => format!(
                "{:?} variant alters senescence regulation → altered cell fate",
                query.gene
            ),
            AgingRole::Mitochondrial => format!(
                "{:?} variant affects mitochondrial function → energy/ROS imbalance",
                query.gene
            ),
            AgingRole::NutrientSensing => format!(
                "{:?} variant alters nutrient sensing → metabolic dysregulation",
                query.gene
            ),
            AgingRole::Inflammation => format!(
                "{:?} variant affects inflammatory signaling",
                query.gene
            ),
            _ => format!("{:?} variant affects {:?}", query.gene, affected_process),
        };

        PredictedFunctionalEffect {
            function_level,
            effect_type,
            affected_process,
            mechanism,
        }
    }

    fn simulate_variant_effect(
        &self,
        query: &VariantQuery,
        functional_effect: &PredictedFunctionalEffect,
        rng: &mut impl Rng,
    ) -> VUSSimulationResults {
        let mut with_variant_lifespans = Vec::new();
        let mut without_variant_lifespans = Vec::new();
        let mut with_variant_diseases: HashMap<DiseaseType, usize> = HashMap::new();
        let mut without_variant_diseases: HashMap<DiseaseType, usize> = HashMap::new();

        // Run simulations with variant
        for _ in 0..self.config.simulations_per_arm {
            let mut organism = Organism::new_random(rng);

            // Apply the variant effect
            if let Some(gene_state) = organism.genome.nuclear_genes.get_mut(&query.gene) {
                gene_state.expression *= functional_effect.function_level;
                gene_state.variants.push(GeneVariant {
                    rsid: query.rsid.clone().unwrap_or_else(|| "query_vus".to_string()),
                    effect: functional_effect.effect_type,
                    allele_frequency: query.allele_frequency.unwrap_or(0.001),
                    longevity_effect: functional_effect.function_level - 1.0,
                });
            }

            if self.config.lifestyle_variation {
                organism.lifestyle = random_lifestyle(rng);
            }

            organism.simulate_life(rng);
            with_variant_lifespans.push(organism.age);

            for disease in &organism.diseases {
                *with_variant_diseases.entry(disease.disease_type).or_insert(0) += 1;
            }
        }

        // Run simulations without variant (control)
        for _ in 0..self.config.simulations_per_arm {
            let mut organism = Organism::new_random(rng);

            if self.config.lifestyle_variation {
                organism.lifestyle = random_lifestyle(rng);
            }

            organism.simulate_life(rng);
            without_variant_lifespans.push(organism.age);

            for disease in &organism.diseases {
                *without_variant_diseases.entry(disease.disease_type).or_insert(0) += 1;
            }
        }

        let n = self.config.simulations_per_arm as f64;

        let mean_with = with_variant_lifespans.iter().sum::<f64>() / n;
        let mean_without = without_variant_lifespans.iter().sum::<f64>() / n;
        let delta = mean_with - mean_without;

        // Calculate p-value (simplified t-test)
        let var_with = with_variant_lifespans.iter()
            .map(|x| (x - mean_with).powi(2))
            .sum::<f64>() / n;
        let var_without = without_variant_lifespans.iter()
            .map(|x| (x - mean_without).powi(2))
            .sum::<f64>() / n;
        let se = ((var_with / n) + (var_without / n)).sqrt();
        let t_stat = if se > 0.0 { delta.abs() / se } else { 0.0 };
        // Approximate p-value (simplified)
        let p_value = if t_stat > 3.0 { 0.001 } else if t_stat > 2.0 { 0.05 } else { 0.5 };

        // Disease risks
        let mut disease_risks = HashMap::new();
        for disease in [DiseaseType::Cancer, DiseaseType::Atherosclerosis,
                       DiseaseType::Type2Diabetes, DiseaseType::Alzheimers] {
            let baseline = *without_variant_diseases.get(&disease).unwrap_or(&0) as f64 / n;
            let variant = *with_variant_diseases.get(&disease).unwrap_or(&0) as f64 / n;

            disease_risks.insert(disease, DiseaseRiskChange {
                baseline_risk: baseline,
                variant_risk: variant,
                relative_risk: if baseline > 0.0 { variant / baseline } else { 1.0 },
                absolute_risk_increase: variant - baseline,
            });
        }

        // Effect consistency (how consistent is the effect across simulations?)
        let effect_direction_consistent = with_variant_lifespans.iter()
            .zip(without_variant_lifespans.iter())
            .filter(|(w, wo)| (*w < **wo) == (delta < 0.0))
            .count() as f64 / n;

        // Penetrance (% showing significant effect)
        let penetrance = if delta < 0.0 {
            with_variant_lifespans.iter()
                .filter(|&&l| l < mean_without - 5.0) // >5 year reduction
                .count() as f64 / n
        } else {
            0.0
        };

        VUSSimulationResults {
            total_simulations: self.config.simulations_per_arm * 2,
            lifespan_with_variant: mean_with,
            lifespan_without_variant: mean_without,
            lifespan_delta: delta,
            p_value,
            disease_risks,
            effect_consistency: effect_direction_consistent,
            effect_onset_age: Some(40.0), // Placeholder
            penetrance,
        }
    }

    fn determine_classification(
        &self,
        query: &VariantQuery,
        functional_effect: &PredictedFunctionalEffect,
        simulation: &VUSSimulationResults,
    ) -> (ACMGClassification, f64) {
        let mut pathogenic_score = 0.0;
        let mut benign_score = 0.0;

        // Simulation evidence
        if simulation.lifespan_delta < -5.0 && simulation.p_value < 0.05 {
            pathogenic_score += 0.4;
        } else if simulation.lifespan_delta < -2.0 && simulation.p_value < 0.1 {
            pathogenic_score += 0.2;
        } else if simulation.lifespan_delta.abs() < 1.0 {
            benign_score += 0.3;
        }

        // Functional effect
        if functional_effect.function_level < 0.3 {
            pathogenic_score += 0.3;
        } else if functional_effect.function_level < 0.6 {
            pathogenic_score += 0.15;
        } else if functional_effect.function_level > 0.9 {
            benign_score += 0.2;
        }

        // Conservation
        if let Some(cons) = query.conservation_score {
            if cons > 0.95 {
                pathogenic_score += 0.15;
            } else if cons < 0.5 {
                benign_score += 0.1;
            }
        }

        // Population frequency
        if let Some(af) = query.allele_frequency {
            if af > 0.01 {
                benign_score += 0.3; // Common variant
            } else if af > 0.001 {
                benign_score += 0.1;
            }
        }

        // Disease risk increase
        for (_, risk) in &simulation.disease_risks {
            if risk.relative_risk > 2.0 {
                pathogenic_score += 0.1;
            }
        }

        // Effect consistency
        if simulation.effect_consistency > 0.8 {
            pathogenic_score += 0.1;
        }

        // Penetrance
        if simulation.penetrance > 0.5 {
            pathogenic_score += 0.2;
        }

        let total = pathogenic_score + benign_score;
        let confidence = if total > 0.0 {
            (pathogenic_score - benign_score).abs() / total
        } else {
            0.0
        };

        let classification = if pathogenic_score > benign_score + 0.3 {
            if confidence > self.config.confidence_threshold {
                ACMGClassification::Pathogenic
            } else {
                ACMGClassification::LikelyPathogenic
            }
        } else if benign_score > pathogenic_score + 0.3 {
            if confidence > self.config.confidence_threshold {
                ACMGClassification::Benign
            } else {
                ACMGClassification::LikelyBenign
            }
        } else {
            ACMGClassification::VUS
        };

        (classification, confidence)
    }

    fn generate_clinical_implications(
        &self,
        query: &VariantQuery,
        functional_effect: &PredictedFunctionalEffect,
        simulation: &VUSSimulationResults,
    ) -> Vec<ClinicalImplication> {
        let mut implications = Vec::new();

        // Lifespan impact
        if simulation.lifespan_delta.abs() > 2.0 {
            implications.push(ClinicalImplication {
                implication_type: ClinicalImplicationType::LifespanImpact,
                severity: if simulation.lifespan_delta < -10.0 {
                    Severity::Critical
                } else if simulation.lifespan_delta < -5.0 {
                    Severity::High
                } else {
                    Severity::Moderate
                },
                description: format!(
                    "Predicted lifespan effect: {:+.1} years",
                    simulation.lifespan_delta
                ),
                actionable: true,
            });
        }

        // Cancer risk
        if let Some(risk) = simulation.disease_risks.get(&DiseaseType::Cancer) {
            if risk.relative_risk > 1.5 {
                implications.push(ClinicalImplication {
                    implication_type: ClinicalImplicationType::CancerRisk,
                    severity: if risk.relative_risk > 3.0 { Severity::High } else { Severity::Moderate },
                    description: format!(
                        "{:.1}x increased cancer risk ({:.1}% → {:.1}%)",
                        risk.relative_risk,
                        risk.baseline_risk * 100.0,
                        risk.variant_risk * 100.0
                    ),
                    actionable: true,
                });
            }
        }

        // Cardiovascular risk
        if let Some(risk) = simulation.disease_risks.get(&DiseaseType::Atherosclerosis) {
            if risk.relative_risk > 1.5 {
                implications.push(ClinicalImplication {
                    implication_type: ClinicalImplicationType::CardiovascularRisk,
                    severity: if risk.relative_risk > 2.5 { Severity::High } else { Severity::Moderate },
                    description: format!(
                        "{:.1}x increased cardiovascular risk",
                        risk.relative_risk
                    ),
                    actionable: true,
                });
            }
        }

        // Family screening recommendation
        if simulation.penetrance > 0.3 && simulation.lifespan_delta < -5.0 {
            implications.push(ClinicalImplication {
                implication_type: ClinicalImplicationType::FamilyScreening,
                severity: Severity::Moderate,
                description: "Consider cascade testing in first-degree relatives".to_string(),
                actionable: true,
            });
        }

        implications
    }

    fn compile_evidence(
        &self,
        query: &VariantQuery,
        functional_effect: &PredictedFunctionalEffect,
        simulation: &VUSSimulationResults,
    ) -> Vec<EvidenceItem> {
        let mut evidence = Vec::new();

        // Simulation evidence
        evidence.push(EvidenceItem {
            category: "In-silico simulation".to_string(),
            strength: if simulation.p_value < 0.01 { 0.9 } else if simulation.p_value < 0.05 { 0.7 } else { 0.3 },
            description: format!(
                "{} simulated lives: {:+.1}y lifespan (p={:.3})",
                simulation.total_simulations,
                simulation.lifespan_delta,
                simulation.p_value
            ),
            supporting: simulation.lifespan_delta < -self.config.min_effect_years,
        });

        // Functional prediction
        evidence.push(EvidenceItem {
            category: "Functional effect prediction".to_string(),
            strength: 0.6,
            description: format!(
                "{:?} (function at {:.0}% of normal)",
                functional_effect.effect_type,
                functional_effect.function_level * 100.0
            ),
            supporting: functional_effect.function_level < 0.7,
        });

        // Conservation
        if let Some(cons) = query.conservation_score {
            evidence.push(EvidenceItem {
                category: "Evolutionary conservation".to_string(),
                strength: cons,
                description: format!("Conservation score: {:.2}", cons),
                supporting: cons > 0.8,
            });
        }

        // Structural impact
        if let Some(impact) = query.structural_impact {
            evidence.push(EvidenceItem {
                category: "Structural impact".to_string(),
                strength: match impact {
                    StructuralImpact::Severe => 0.9,
                    StructuralImpact::Moderate => 0.6,
                    StructuralImpact::Minor => 0.3,
                    _ => 0.0,
                },
                description: format!("{:?} structural impact", impact),
                supporting: !matches!(impact, StructuralImpact::None | StructuralImpact::Minor),
            });
        }

        // Effect consistency
        evidence.push(EvidenceItem {
            category: "Effect consistency".to_string(),
            strength: simulation.effect_consistency,
            description: format!(
                "{:.0}% consistent effect direction across genetic backgrounds",
                simulation.effect_consistency * 100.0
            ),
            supporting: simulation.effect_consistency > 0.7,
        });

        evidence
    }

    fn generate_recommendations(
        &self,
        classification: &ACMGClassification,
        implications: &[ClinicalImplication],
    ) -> Vec<String> {
        let mut recs = Vec::new();

        match classification {
            ACMGClassification::Pathogenic | ACMGClassification::LikelyPathogenic => {
                recs.push("Consider this variant in clinical decision-making".to_string());

                for imp in implications {
                    if imp.actionable {
                        match imp.implication_type {
                            ClinicalImplicationType::CancerRisk => {
                                recs.push("Enhanced cancer screening may be warranted".to_string());
                            }
                            ClinicalImplicationType::CardiovascularRisk => {
                                recs.push("Cardiovascular risk assessment recommended".to_string());
                            }
                            ClinicalImplicationType::FamilyScreening => {
                                recs.push("Cascade genetic testing for at-risk relatives".to_string());
                            }
                            _ => {}
                        }
                    }
                }
            }
            ACMGClassification::VUS => {
                recs.push("Variant remains of uncertain significance".to_string());
                recs.push("Clinical decisions should not be based solely on this variant".to_string());
                recs.push("Consider periodic re-evaluation as evidence accumulates".to_string());
            }
            ACMGClassification::LikelyBenign | ACMGClassification::Benign => {
                recs.push("Variant unlikely to be clinically significant".to_string());
                recs.push("Standard care recommendations apply".to_string());
            }
        }

        recs
    }

    /// Batch interpret multiple VUS
    pub fn batch_interpret(
        &self,
        queries: Vec<VariantQuery>,
        rng: &mut impl Rng,
    ) -> Vec<VUSInterpretation> {
        queries.into_iter()
            .map(|q| self.interpret(q, rng))
            .collect()
    }
}

fn random_lifestyle(rng: &mut impl Rng) -> Lifestyle {
    Lifestyle {
        caloric_intake: rng.gen_range(0.8..1.4),
        diet_quality: rng.gen_range(0.2..0.95),
        exercise_hours: rng.gen_range(0.0..10.0),
        sleep_quality: rng.gen_range(0.3..0.95),
        sleep_hours: rng.gen_range(4.0..10.0), // Varies around population mean
        stress: rng.gen_range(0.1..0.8),
        smoking: if rng.gen::<f64>() < 0.2 { rng.gen_range(0..30) } else { 0 },
        alcohol: rng.gen_range(0..30),
        sun_exposure: rng.gen_range(0.1..0.8),
        pollution: rng.gen_range(0.0..0.6),
        social: rng.gen_range(0.2..0.9),
    }
}

/// Quick VUS query builder
pub fn query_vus(gene: Gene, hgvs: &str) -> VariantQuery {
    VariantQuery {
        id: Uuid::new_v4(),
        gene,
        hgvs_cdna: hgvs.to_string(),
        hgvs_protein: None,
        rsid: None,
        aa_position: None,
        conservation_score: None,
        in_functional_domain: false,
        structural_impact: None,
        allele_frequency: None,
        existing_evidence: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vus_interpretation() {
        let mut rng = rand::thread_rng();

        let query = VariantQuery {
            id: Uuid::new_v4(),
            gene: Gene::BRCA1,
            hgvs_cdna: "c.1234A>G".to_string(),
            hgvs_protein: Some("p.Lys412Arg".to_string()),
            rsid: None,
            aa_position: Some(412),
            conservation_score: Some(0.95), // Highly conserved
            in_functional_domain: true,
            structural_impact: Some(StructuralImpact::Moderate),
            allele_frequency: Some(0.0001), // Rare
            existing_evidence: Vec::new(),
        };

        let config = VUSConfig {
            simulations_per_arm: 100, // Small for testing
            ..Default::default()
        };

        let interpreter = VUSInterpreter::new(config);
        let result = interpreter.interpret(query, &mut rng);

        // Should produce some classification
        assert!(!result.recommendations.is_empty());
        assert!(result.confidence >= 0.0);
    }

    #[test]
    fn test_benign_variant() {
        let mut rng = rand::thread_rng();

        // Common variant, low conservation = likely benign
        let query = VariantQuery {
            id: Uuid::new_v4(),
            gene: Gene::SIRT1,
            hgvs_cdna: "c.100A>G".to_string(),
            hgvs_protein: None,
            rsid: Some("rs12345678".to_string()),
            aa_position: None,
            conservation_score: Some(0.3), // Not conserved
            in_functional_domain: false,
            structural_impact: Some(StructuralImpact::None),
            allele_frequency: Some(0.05), // Common (5%)
            existing_evidence: Vec::new(),
        };

        let config = VUSConfig {
            simulations_per_arm: 100,
            ..Default::default()
        };

        let interpreter = VUSInterpreter::new(config);
        let result = interpreter.interpret(query, &mut rng);

        // Should lean benign due to high frequency + low conservation
        assert!(matches!(result.predicted_classification,
            ACMGClassification::LikelyBenign | ACMGClassification::Benign | ACMGClassification::VUS));
    }

    #[test]
    fn test_functional_effect_prediction() {
        let interpreter = VUSInterpreter::new(VUSConfig::default());

        let query = VariantQuery {
            id: Uuid::new_v4(),
            gene: Gene::TP53,
            hgvs_cdna: "c.742C>T".to_string(),
            hgvs_protein: Some("p.Arg248Trp".to_string()),
            rsid: None,
            aa_position: Some(248),
            conservation_score: Some(0.99),
            in_functional_domain: true,
            structural_impact: Some(StructuralImpact::Severe),
            allele_frequency: None,
            existing_evidence: Vec::new(),
        };

        let effect = interpreter.predict_functional_effect(&query);

        // Should predict significant loss of function
        assert!(effect.function_level < 0.5);
        assert!(matches!(effect.effect_type, VariantEffect::LossOfFunction | VariantEffect::ReducedFunction));
    }
}
