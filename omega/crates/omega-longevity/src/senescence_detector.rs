//! Senescence Pattern Detector - Synthetic Intuition for Aging Patterns
//!
//! Uses subconscious pattern recognition to detect subtle aging signatures
//! in multi-omics data that conscious analysis might miss.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────┐
//!  │               SENESCENCE PATTERN DETECTOR                           │
//!  ├─────────────────────────────────────────────────────────────────────┤
//!  │                                                                     │
//!  │   MULTI-OMICS DATA        INTUITION ENGINE       PATTERN OUTPUT    │
//!  │   ┌──────────────┐        ┌───────────────┐      ┌──────────────┐ │
//!  │   │ Transcriptome│        │  Synchrony    │      │ Senescence   │ │
//!  │   │ Proteome     │───────►│  Detection    │─────►│ Signatures   │ │
//!  │   │ Metabolome   │        │               │      │              │ │
//!  │   │ Epigenome    │        │  "Gut Feeling"│      │ Novel        │ │
//!  │   │ Microbiome   │        │  Patterns     │      │ Biomarkers   │ │
//!  │   └──────────────┘        └───────────────┘      └──────────────┘ │
//!  │                                                                     │
//!  │   Detects: Early senescence, Tissue-specific aging, SASP patterns  │
//!  │                                                                     │
//!  └─────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
use rand::Rng;

use crate::hallmarks::Hallmark;
use crate::{Result, LongevityError};

/// Configuration for senescence detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorConfig {
    /// Minimum synchrony for pattern detection
    pub min_synchrony: f64,
    /// Confidence threshold for reporting
    pub confidence_threshold: f64,
    /// Enable multi-tissue analysis
    pub multi_tissue: bool,
    /// Number of processing cycles
    pub processing_cycles: usize,
    /// Enable predictive mode (future senescence)
    pub predictive_mode: bool,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            min_synchrony: 0.5,
            confidence_threshold: 0.6,
            multi_tissue: true,
            processing_cycles: 100,
            predictive_mode: true,
        }
    }
}

/// Types of omics data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OmicsType {
    Transcriptome,
    Proteome,
    Metabolome,
    Epigenome,
    Microbiome,
    Lipidome,
    Glycome,
}

/// A multi-omics sample for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmicsSample {
    pub id: Uuid,
    pub sample_id: String,
    /// Chronological age of subject
    pub chronological_age: f64,
    /// Tissue type
    pub tissue: TissueType,
    /// Omics data layers
    pub layers: HashMap<OmicsType, OmicsLayer>,
    /// Clinical metadata
    pub metadata: SampleMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmicsLayer {
    pub omics_type: OmicsType,
    /// Feature names
    pub features: Vec<String>,
    /// Feature values (normalized)
    pub values: Vec<f64>,
    /// Quality scores
    pub quality: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TissueType {
    Blood,
    Skin,
    Liver,
    Brain,
    Muscle,
    Adipose,
    Heart,
    Kidney,
    Lung,
    Intestine,
    BoneMarrow,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleMetadata {
    pub sex: Option<String>,
    pub health_status: Option<String>,
    pub medications: Vec<String>,
    pub lifestyle_factors: Vec<String>,
}

/// A detected senescence pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenescencePattern {
    pub id: Uuid,
    /// Pattern name/identifier
    pub name: String,
    /// Type of senescence
    pub senescence_type: SenescenceType,
    /// Confidence in detection
    pub confidence: f64,
    /// Synchrony score
    pub synchrony: f64,
    /// Associated omics features
    pub features: Vec<PatternFeature>,
    /// Affected hallmarks
    pub hallmarks: Vec<Hallmark>,
    /// Tissue specificity
    pub tissue_specific: Option<TissueType>,
    /// Predicted progression rate
    pub progression_rate: Option<f64>,
    /// Suggested interventions
    pub suggested_interventions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SenescenceType {
    /// Replicative senescence (telomere-driven)
    Replicative,
    /// Oncogene-induced senescence
    OncogeneInduced,
    /// Stress-induced premature senescence
    StressInduced,
    /// Therapy-induced senescence
    TherapyInduced,
    /// Mitochondria-associated senescence
    MitochondrialAssociated,
    /// Paracrine senescence (SASP-driven)
    Paracrine,
    /// Unknown/novel type
    Novel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternFeature {
    pub name: String,
    pub omics_type: OmicsType,
    pub weight: f64,
    pub direction: FeatureDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureDirection {
    Increased,
    Decreased,
    Altered, // Non-directional change
}

/// A biological age prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalAgePrediction {
    pub id: Uuid,
    /// Predicted biological age
    pub biological_age: f64,
    /// Chronological age
    pub chronological_age: f64,
    /// Age acceleration (bio - chrono)
    pub age_acceleration: f64,
    /// Confidence interval
    pub confidence_interval: (f64, f64),
    /// Contributing factors
    pub top_contributors: Vec<(String, f64)>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// A senescence trajectory prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenescenceTrajectory {
    pub id: Uuid,
    /// Current senescent burden (%)
    pub current_burden: f64,
    /// Predicted future burdens (years, %)
    pub future_predictions: Vec<(f64, f64)>,
    /// Critical threshold age (when burden becomes pathological)
    pub critical_age: Option<f64>,
    /// Intervention impact scenarios
    pub intervention_scenarios: Vec<InterventionScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionScenario {
    pub intervention: String,
    pub predicted_reduction: f64,
    pub new_trajectory: Vec<(f64, f64)>,
}

/// Known senescence markers
pub struct SenescenceMarkers {
    /// Core senescence markers
    pub core_markers: Vec<String>,
    /// SASP factors
    pub sasp_factors: Vec<String>,
    /// Tissue-specific markers
    pub tissue_markers: HashMap<TissueType, Vec<String>>,
}

impl Default for SenescenceMarkers {
    fn default() -> Self {
        let mut tissue_markers = HashMap::new();
        tissue_markers.insert(TissueType::Skin, vec![
            "MMP1".to_string(), "MMP3".to_string(), "ELN".to_string(),
        ]);
        tissue_markers.insert(TissueType::Brain, vec![
            "GFAP".to_string(), "AQP4".to_string(), "SLC1A2".to_string(),
        ]);
        tissue_markers.insert(TissueType::Liver, vec![
            "CDKN2A".to_string(), "GDF15".to_string(), "IGFBP7".to_string(),
        ]);

        Self {
            core_markers: vec![
                "CDKN2A".to_string(),  // p16
                "CDKN1A".to_string(),  // p21
                "TP53".to_string(),
                "GLB1".to_string(),    // SA-β-gal
                "LMNB1".to_string(),   // Lamin B1 (decreased)
                "H2AFX".to_string(),   // γH2AX
            ],
            sasp_factors: vec![
                "IL6".to_string(),
                "IL8".to_string(),
                "CXCL1".to_string(),
                "CCL2".to_string(),
                "MMP3".to_string(),
                "PAI1".to_string(),
                "GDF15".to_string(),
                "IGFBP7".to_string(),
            ],
            tissue_markers,
        }
    }
}

/// The Senescence Pattern Detector
pub struct SenescenceDetector {
    config: DetectorConfig,
    /// Known senescence markers
    markers: SenescenceMarkers,
    /// Detected patterns
    patterns: Vec<SenescencePattern>,
    /// Biological age predictions
    predictions: Vec<BiologicalAgePrediction>,
    /// Historical patterns (for learning)
    historical_patterns: Vec<SenescencePattern>,
    /// RNG
    rng: rand::rngs::ThreadRng,
}

impl SenescenceDetector {
    pub fn new(config: DetectorConfig) -> Self {
        Self {
            config,
            markers: SenescenceMarkers::default(),
            patterns: Vec::new(),
            predictions: Vec::new(),
            historical_patterns: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Analyze a multi-omics sample for senescence patterns
    pub fn analyze_sample(&mut self, sample: &OmicsSample) -> Result<Vec<SenescencePattern>> {
        let mut detected_patterns = Vec::new();

        // 1. Check core senescence markers
        if let Some(pattern) = self.detect_core_senescence(sample) {
            detected_patterns.push(pattern);
        }

        // 2. Check SASP signature
        if let Some(pattern) = self.detect_sasp(sample) {
            detected_patterns.push(pattern);
        }

        // 3. Tissue-specific patterns
        if self.config.multi_tissue {
            if let Some(pattern) = self.detect_tissue_specific(sample) {
                detected_patterns.push(pattern);
            }
        }

        // 4. Intuitive pattern detection (subconscious)
        let intuitive_patterns = self.intuitive_scan(sample);
        detected_patterns.extend(intuitive_patterns);

        // Store patterns
        self.patterns.extend(detected_patterns.clone());

        Ok(detected_patterns)
    }

    fn detect_core_senescence(&self, sample: &OmicsSample) -> Option<SenescencePattern> {
        let mut score = 0.0;
        let mut found_features = Vec::new();

        // Check transcriptome for core markers
        if let Some(layer) = sample.layers.get(&OmicsType::Transcriptome) {
            for (i, feature) in layer.features.iter().enumerate() {
                if self.markers.core_markers.contains(feature) {
                    let value = layer.values.get(i).unwrap_or(&0.0);

                    // Determine expected direction
                    let (expected_high, is_high) = if feature == "LMNB1" {
                        (false, *value < 0.5) // Lamin B1 decreases
                    } else {
                        (true, *value > 0.5) // Most markers increase
                    };

                    if is_high == expected_high {
                        score += 0.2;
                        found_features.push(PatternFeature {
                            name: feature.clone(),
                            omics_type: OmicsType::Transcriptome,
                            weight: *value,
                            direction: if expected_high {
                                FeatureDirection::Increased
                            } else {
                                FeatureDirection::Decreased
                            },
                        });
                    }
                }
            }
        }

        if score >= self.config.confidence_threshold && !found_features.is_empty() {
            Some(SenescencePattern {
                id: Uuid::new_v4(),
                name: "Core Senescence Signature".to_string(),
                senescence_type: SenescenceType::Replicative,
                confidence: score.min(1.0),
                synchrony: self.calculate_synchrony(&found_features),
                features: found_features,
                hallmarks: vec![Hallmark::CellularSenescence],
                tissue_specific: Some(sample.tissue),
                progression_rate: Some(0.02), // 2% per year
                suggested_interventions: vec![
                    "Senolytic therapy (D+Q)".to_string(),
                    "Fisetin supplementation".to_string(),
                ],
            })
        } else {
            None
        }
    }

    fn detect_sasp(&self, sample: &OmicsSample) -> Option<SenescencePattern> {
        let mut score = 0.0;
        let mut found_features = Vec::new();

        // Check proteome for SASP factors
        if let Some(layer) = sample.layers.get(&OmicsType::Proteome) {
            for (i, feature) in layer.features.iter().enumerate() {
                if self.markers.sasp_factors.contains(feature) {
                    let value = layer.values.get(i).unwrap_or(&0.0);
                    if *value > 0.5 {
                        score += 0.15;
                        found_features.push(PatternFeature {
                            name: feature.clone(),
                            omics_type: OmicsType::Proteome,
                            weight: *value,
                            direction: FeatureDirection::Increased,
                        });
                    }
                }
            }
        }

        if score >= self.config.confidence_threshold && !found_features.is_empty() {
            Some(SenescencePattern {
                id: Uuid::new_v4(),
                name: "SASP Signature".to_string(),
                senescence_type: SenescenceType::Paracrine,
                confidence: score.min(1.0),
                synchrony: self.calculate_synchrony(&found_features),
                features: found_features,
                hallmarks: vec![Hallmark::CellularSenescence, Hallmark::ChronicInflammation],
                tissue_specific: Some(sample.tissue),
                progression_rate: Some(0.03),
                suggested_interventions: vec![
                    "Anti-inflammatory intervention".to_string(),
                    "SASP inhibitors (Rapamycin)".to_string(),
                ],
            })
        } else {
            None
        }
    }

    fn detect_tissue_specific(&self, sample: &OmicsSample) -> Option<SenescencePattern> {
        let tissue_markers = self.markers.tissue_markers.get(&sample.tissue)?;
        let mut score = 0.0;
        let mut found_features = Vec::new();

        for layer in sample.layers.values() {
            for (i, feature) in layer.features.iter().enumerate() {
                if tissue_markers.contains(feature) {
                    let value = layer.values.get(i).unwrap_or(&0.0);
                    score += 0.25;
                    found_features.push(PatternFeature {
                        name: feature.clone(),
                        omics_type: layer.omics_type,
                        weight: *value,
                        direction: if *value > 0.5 {
                            FeatureDirection::Increased
                        } else {
                            FeatureDirection::Decreased
                        },
                    });
                }
            }
        }

        if score >= 0.5 && !found_features.is_empty() {
            Some(SenescencePattern {
                id: Uuid::new_v4(),
                name: format!("{:?}-Specific Senescence", sample.tissue),
                senescence_type: SenescenceType::StressInduced,
                confidence: score.min(1.0),
                synchrony: self.calculate_synchrony(&found_features),
                features: found_features,
                hallmarks: vec![Hallmark::CellularSenescence, Hallmark::StemCellExhaustion],
                tissue_specific: Some(sample.tissue),
                progression_rate: Some(0.025),
                suggested_interventions: vec![
                    format!("Tissue-targeted senolytic for {:?}", sample.tissue),
                ],
            })
        } else {
            None
        }
    }

    /// Intuitive pattern detection - find patterns conscious analysis might miss
    fn intuitive_scan(&mut self, sample: &OmicsSample) -> Vec<SenescencePattern> {
        let mut patterns = Vec::new();

        // Simulate subconscious processing
        for _ in 0..self.config.processing_cycles {
            // Cross-omics synchrony detection
            let synchrony = self.detect_cross_omics_synchrony(sample);

            if synchrony > self.config.min_synchrony {
                // "Gut feeling" about a pattern
                let confidence = self.rng.gen_range(0.5..0.9);

                if confidence >= self.config.confidence_threshold {
                    patterns.push(SenescencePattern {
                        id: Uuid::new_v4(),
                        name: "Intuitive Cross-Omics Pattern".to_string(),
                        senescence_type: SenescenceType::Novel,
                        confidence,
                        synchrony,
                        features: vec![],
                        hallmarks: vec![Hallmark::CellularSenescence],
                        tissue_specific: Some(sample.tissue),
                        progression_rate: None,
                        suggested_interventions: vec![
                            "Requires validation studies".to_string(),
                        ],
                    });
                    break; // One intuitive pattern per scan
                }
            }
        }

        patterns
    }

    fn detect_cross_omics_synchrony(&mut self, sample: &OmicsSample) -> f64 {
        // Check if patterns across omics layers are correlated
        let mut correlations = Vec::new();

        let layer_values: Vec<&Vec<f64>> = sample.layers.values()
            .map(|l| &l.values)
            .collect();

        if layer_values.len() < 2 {
            return 0.0;
        }

        // Simple correlation between first two layers
        for i in 0..layer_values.len() {
            for j in (i + 1)..layer_values.len() {
                let corr = self.simple_correlation(layer_values[i], layer_values[j]);
                correlations.push(corr.abs());
            }
        }

        if correlations.is_empty() {
            0.0
        } else {
            correlations.iter().sum::<f64>() / correlations.len() as f64
        }
    }

    fn simple_correlation(&self, a: &[f64], b: &[f64]) -> f64 {
        let n = a.len().min(b.len());
        if n == 0 {
            return 0.0;
        }

        let mean_a: f64 = a.iter().take(n).sum::<f64>() / n as f64;
        let mean_b: f64 = b.iter().take(n).sum::<f64>() / n as f64;

        let mut cov = 0.0;
        let mut var_a = 0.0;
        let mut var_b = 0.0;

        for i in 0..n {
            let da = a[i] - mean_a;
            let db = b[i] - mean_b;
            cov += da * db;
            var_a += da * da;
            var_b += db * db;
        }

        if var_a == 0.0 || var_b == 0.0 {
            return 0.0;
        }

        cov / (var_a.sqrt() * var_b.sqrt())
    }

    fn calculate_synchrony(&self, features: &[PatternFeature]) -> f64 {
        if features.is_empty() {
            return 0.0;
        }
        // Average weight as synchrony proxy
        features.iter().map(|f| f.weight).sum::<f64>() / features.len() as f64
    }

    /// Predict biological age from multi-omics data
    pub fn predict_biological_age(&mut self, sample: &OmicsSample) -> Result<BiologicalAgePrediction> {
        let chrono_age = sample.chronological_age;

        // Simplified biological age calculation
        // In reality, would use trained epigenetic clocks, etc.
        let mut age_adjustments = 0.0;
        let mut contributors = Vec::new();

        // Senescence burden adjustment
        let patterns = self.analyze_sample(sample)?;
        for pattern in &patterns {
            let adjustment = pattern.confidence * 5.0; // Up to 5 years per pattern
            age_adjustments += adjustment;
            contributors.push((pattern.name.clone(), adjustment));
        }

        // Inflammation adjustment
        if let Some(proteome) = sample.layers.get(&OmicsType::Proteome) {
            let inflammation_score: f64 = proteome.values.iter()
                .take(10)
                .sum::<f64>() / 10.0;
            let adjustment = (inflammation_score - 0.5) * 10.0;
            age_adjustments += adjustment;
            contributors.push(("Inflammatory burden".to_string(), adjustment));
        }

        let biological_age = chrono_age + age_adjustments;
        let age_acceleration = age_adjustments;

        let prediction = BiologicalAgePrediction {
            id: Uuid::new_v4(),
            biological_age,
            chronological_age: chrono_age,
            age_acceleration,
            confidence_interval: (biological_age - 3.0, biological_age + 3.0),
            top_contributors: contributors,
            recommendations: self.generate_recommendations(age_acceleration),
        };

        self.predictions.push(prediction.clone());
        Ok(prediction)
    }

    fn generate_recommendations(&self, acceleration: f64) -> Vec<String> {
        let mut recs = Vec::new();

        if acceleration > 5.0 {
            recs.push("Consider senolytic intervention".to_string());
            recs.push("Evaluate chronic inflammation sources".to_string());
        } else if acceleration > 2.0 {
            recs.push("Lifestyle optimization recommended".to_string());
            recs.push("Consider NAD+ supplementation".to_string());
        } else if acceleration < -2.0 {
            recs.push("Excellent biological age - maintain current regimen".to_string());
        } else {
            recs.push("Age-appropriate profile - standard preventive measures".to_string());
        }

        recs
    }

    /// Predict senescence trajectory
    pub fn predict_trajectory(
        &mut self,
        sample: &OmicsSample,
        years_ahead: f64,
    ) -> Result<SenescenceTrajectory> {
        if !self.config.predictive_mode {
            return Err(LongevityError::PredictionFailed("Predictive mode disabled".to_string()));
        }

        let current_patterns = self.analyze_sample(sample)?;
        let current_burden = current_patterns.iter()
            .map(|p| p.confidence)
            .sum::<f64>()
            .min(1.0);

        let mut future_predictions = Vec::new();
        let progression_rate = 0.02; // 2% per year base

        for year in 1..=(years_ahead as usize) {
            let future_burden = (current_burden + progression_rate * year as f64).min(1.0);
            future_predictions.push((year as f64, future_burden));
        }

        // Find critical age
        let critical_threshold = 0.7;
        let critical_age = future_predictions.iter()
            .find(|(_, burden)| *burden >= critical_threshold)
            .map(|(year, _)| sample.chronological_age + year);

        // Intervention scenarios
        let scenarios = vec![
            InterventionScenario {
                intervention: "Senolytic therapy".to_string(),
                predicted_reduction: 0.3,
                new_trajectory: future_predictions.iter()
                    .map(|(y, b)| (*y, (b * 0.7).max(0.0)))
                    .collect(),
            },
            InterventionScenario {
                intervention: "Rapamycin".to_string(),
                predicted_reduction: 0.2,
                new_trajectory: future_predictions.iter()
                    .map(|(y, b)| (*y, (b * 0.8).max(0.0)))
                    .collect(),
            },
        ];

        Ok(SenescenceTrajectory {
            id: Uuid::new_v4(),
            current_burden,
            future_predictions,
            critical_age,
            intervention_scenarios: scenarios,
        })
    }

    /// Get all detected patterns
    pub fn patterns(&self) -> &[SenescencePattern] {
        &self.patterns
    }

    /// Get high-confidence patterns
    pub fn high_confidence_patterns(&self, threshold: f64) -> Vec<&SenescencePattern> {
        self.patterns.iter()
            .filter(|p| p.confidence >= threshold)
            .collect()
    }

    /// Learn from historical patterns (improve detection)
    pub fn learn_from_outcome(&mut self, pattern_id: Uuid, was_accurate: bool) {
        if was_accurate {
            if let Some(pattern) = self.patterns.iter().find(|p| p.id == pattern_id) {
                self.historical_patterns.push(pattern.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sample() -> OmicsSample {
        let mut layers = HashMap::new();

        layers.insert(OmicsType::Transcriptome, OmicsLayer {
            omics_type: OmicsType::Transcriptome,
            features: vec![
                "CDKN2A".to_string(),
                "CDKN1A".to_string(),
                "TP53".to_string(),
                "OTHER".to_string(),
            ],
            values: vec![0.8, 0.7, 0.6, 0.3],
            quality: 0.95,
        });

        layers.insert(OmicsType::Proteome, OmicsLayer {
            omics_type: OmicsType::Proteome,
            features: vec![
                "IL6".to_string(),
                "IL8".to_string(),
                "GDF15".to_string(),
            ],
            values: vec![0.7, 0.6, 0.8],
            quality: 0.90,
        });

        OmicsSample {
            id: Uuid::new_v4(),
            sample_id: "TEST001".to_string(),
            chronological_age: 65.0,
            tissue: TissueType::Blood,
            layers,
            metadata: SampleMetadata {
                sex: Some("M".to_string()),
                health_status: Some("Healthy".to_string()),
                medications: vec![],
                lifestyle_factors: vec!["Exercise".to_string()],
            },
        }
    }

    #[test]
    fn test_detector_creation() {
        let config = DetectorConfig::default();
        let detector = SenescenceDetector::new(config);
        assert!(detector.patterns.is_empty());
    }

    #[test]
    fn test_analyze_sample() {
        let config = DetectorConfig::default();
        let mut detector = SenescenceDetector::new(config);
        let sample = create_test_sample();

        let patterns = detector.analyze_sample(&sample).unwrap();
        // Should detect at least core senescence
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_biological_age_prediction() {
        let config = DetectorConfig::default();
        let mut detector = SenescenceDetector::new(config);
        let sample = create_test_sample();

        let prediction = detector.predict_biological_age(&sample).unwrap();
        assert!(prediction.biological_age > 0.0);
    }
}
