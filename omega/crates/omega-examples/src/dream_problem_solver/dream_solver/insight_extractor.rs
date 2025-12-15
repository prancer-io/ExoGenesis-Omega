//! Insight Extractor with Spatial Discovery
//!
//! Extracts actionable insights from dreams, including spatial proximity insights
//! from 3D dream world representations.

use super::dream_generator::DreamWorld3D;

const PROXIMITY_THRESHOLD: f64 = 15.0;

/// Extracts actionable insights from dreams
#[derive(Default)]
pub struct InsightExtractor {
    insight_id_counter: u64,
}

impl InsightExtractor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extract insights from a dream (standard version)
    pub fn extract(
        &mut self,
        dream: &super::Dream,
        problem: &super::Problem
    ) -> Vec<super::Insight> {
        self.extract_with_spatial(dream, None, problem)
    }

    /// Extract insights from dream AND its 3D representation
    pub fn extract_with_spatial(
        &mut self,
        dream: &super::Dream,
        dream_world: Option<&DreamWorld3D>,
        problem: &super::Problem
    ) -> Vec<super::Insight> {
        let mut insights = Vec::new();

        // Analyze novel combinations from neural dynamics
        for (a, b) in &dream.novel_combinations {
            // Check if both relate to problem
            let a_relevant = problem.elements.iter().any(|e| e.name == *a);
            let b_relevant = problem.elements.iter().any(|e| e.name == *b);

            if a_relevant || b_relevant {
                self.insight_id_counter += 1;

                let connection_type = self.infer_connection_type(a, b, dream);
                let relevance = self.compute_relevance(a, b, problem);

                insights.push(super::Insight {
                    id: format!("insight_{}", self.insight_id_counter),
                    association: super::Association {
                        from: a.clone(),
                        to: b.clone(),
                        connection_type,
                        bridge: vec![0.0; 32], // Would compute proper embedding
                        strength: dream.bizarreness,
                    },
                    source_dream_id: dream.id.clone(),
                    bizarreness: dream.bizarreness,
                    relevance,
                    confidence: relevance * (1.0 - dream.bizarreness * 0.5),
                    timestamp: current_timestamp(),
                });
            }
        }

        // NEW: Spatial proximity insights from 3D dream world
        if let Some(world) = dream_world {
            for (concept_a, loc_a) in &world.concept_locations {
                for (concept_b, loc_b) in &world.concept_locations {
                    if concept_a >= concept_b {
                        continue; // Avoid duplicates
                    }

                    let distance = loc_a.position.distance_to(&loc_b.position);

                    if distance < PROXIMITY_THRESHOLD {
                        // Close proximity in dream space = potential spatial insight
                        let spatial_insight = self.create_proximity_insight(
                            concept_a,
                            concept_b,
                            distance,
                            loc_a.activation,
                            loc_b.activation,
                            &dream.id,
                        );

                        insights.push(spatial_insight);
                    }
                }
            }
        }

        // Look for inversions (if failed approaches appeared transformed)
        for element in &dream.elements {
            if element.original_concept.starts_with("failed_")
                && element.transformation_type == super::TransformationType::Displaced
            {
                // Failed approach appeared in disguise - potential inversion insight
                self.insight_id_counter += 1;
                insights.push(super::Insight {
                    id: format!("insight_{}", self.insight_id_counter),
                    association: super::Association {
                        from: element.original_concept.clone(),
                        to: "inverted_approach".to_string(),
                        connection_type: super::core_types::ConnectionType::Inversion,
                        bridge: vec![0.0; 32],
                        strength: element.activation,
                    },
                    source_dream_id: dream.id.clone(),
                    bizarreness: dream.bizarreness,
                    relevance: 0.7,
                    confidence: 0.5,
                    timestamp: current_timestamp(),
                });
            }
        }

        insights
    }

    /// NEW: Create insight from spatial proximity in dream world
    fn create_proximity_insight(
        &mut self,
        concept_a: &str,
        concept_b: &str,
        distance: f64,
        activation_a: f64,
        activation_b: f64,
        dream_id: &str,
    ) -> super::Insight {
        self.insight_id_counter += 1;

        // Closer concepts have higher strength
        let strength = (1.0 - (distance / PROXIMITY_THRESHOLD)).max(0.0);

        // Combined activation indicates relevance
        let relevance = ((activation_a + activation_b) / 2.0).min(1.0);

        super::Insight {
            id: format!("spatial_insight_{}", self.insight_id_counter),
            association: super::Association {
                from: concept_a.to_string(),
                to: concept_b.to_string(),
                connection_type: super::core_types::ConnectionType::SpatialProximity,
                bridge: vec![distance; 32], // Encode distance in bridge
                strength,
            },
            source_dream_id: dream_id.to_string(),
            bizarreness: 0.3, // Spatial insights are less bizarre
            relevance,
            confidence: strength * relevance,
            timestamp: current_timestamp(),
        }
    }

    fn infer_connection_type(&self, a: &str, b: &str, dream: &super::Dream) -> super::core_types::ConnectionType {
        // Heuristics for connection type
        if dream.bizarreness > 0.7 {
            super::core_types::ConnectionType::Synthesis // Highly bizarre = novel synthesis
        } else if a.contains("failed") || b.contains("failed") {
            super::core_types::ConnectionType::Inversion
        } else if dream.bizarreness > 0.4 {
            super::core_types::ConnectionType::Analogy
        } else {
            super::core_types::ConnectionType::CommonGround
        }
    }

    fn compute_relevance(&self, a: &str, b: &str, problem: &super::Problem) -> f64 {
        let mut score = 0.0;

        for element in &problem.elements {
            if element.name == *a || element.name == *b {
                score += element.importance;
            }
        }

        (score / 2.0).min(1.0)
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
