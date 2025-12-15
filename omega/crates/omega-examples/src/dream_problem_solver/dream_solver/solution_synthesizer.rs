//! Solution Synthesizer
//!
//! Synthesizes concrete solutions from dream-derived insights.

/// Synthesizes solutions from insights
#[derive(Default)]
pub struct SolutionSynthesizer;

impl SolutionSynthesizer {
    pub fn new() -> Self {
        Self
    }

    /// Synthesize solution from problem and insights
    pub fn synthesize(
        &self,
        problem: &super::Problem,
        insights: &[super::Insight],
    ) -> Option<super::Solution> {
        if insights.is_empty() {
            return None;
        }

        // Rank insights by combined score
        let mut ranked_insights: Vec<_> = insights.iter()
            .map(|i| {
                let score = i.relevance * 0.4 + i.confidence * 0.3 + i.bizarreness * 0.3;
                (i, score)
            })
            .collect();
        ranked_insights.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Use top insights to build solution
        let top_insights: Vec<_> = ranked_insights.iter()
            .take(3)
            .map(|(i, _)| *i)
            .collect();

        let description = self.generate_solution_description(problem, &top_insights);
        let novel_elements = self.extract_novel_elements(&top_insights);

        let novelty = top_insights.iter()
            .map(|i| i.bizarreness)
            .sum::<f64>() / top_insights.len() as f64;

        let feasibility = 1.0 - novelty * 0.3; // More novel = less immediately feasible

        let confidence = top_insights.iter()
            .map(|i| i.confidence)
            .sum::<f64>() / top_insights.len() as f64;

        Some(super::Solution {
            description,
            insights_used: top_insights.iter().map(|i| i.id.clone()).collect(),
            novel_elements,
            confidence,
            novelty,
            feasibility,
        })
    }

    fn generate_solution_description(
        &self,
        problem: &super::Problem,
        insights: &[&super::Insight]
    ) -> String {
        let mut description = format!(
            "Solution to '{}' using dream-derived insights:\n\n",
            problem.description
        );

        for insight in insights {
            match &insight.association.connection_type {
                super::core_types::ConnectionType::Analogy => {
                    description.push_str(&format!(
                        "- Consider that {} is analogous to {}\n",
                        insight.association.from,
                        insight.association.to
                    ));
                }
                super::core_types::ConnectionType::Synthesis => {
                    description.push_str(&format!(
                        "- Combine {} with {} to create new approach\n",
                        insight.association.from,
                        insight.association.to
                    ));
                }
                super::core_types::ConnectionType::Inversion => {
                    description.push_str(&format!(
                        "- Instead of {}, try the opposite approach\n",
                        insight.association.from
                    ));
                }
                super::core_types::ConnectionType::CommonGround => {
                    description.push_str(&format!(
                        "- {} and {} share hidden connection\n",
                        insight.association.from,
                        insight.association.to
                    ));
                }
                super::core_types::ConnectionType::SpatialProximity => {
                    description.push_str(&format!(
                        "- {} and {} are spatially proximate in dream space (explore their relationship)\n",
                        insight.association.from,
                        insight.association.to
                    ));
                }
                _ => {}
            }
        }

        description
    }

    fn extract_novel_elements(&self, insights: &[&super::Insight]) -> Vec<String> {
        insights.iter()
            .flat_map(|i| vec![
                format!("{}↔{}", i.association.from, i.association.to)
            ])
            .collect()
    }
}
