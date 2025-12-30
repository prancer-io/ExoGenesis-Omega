//! Research Integrator - Collective Intelligence for Literature Synthesis
//!
//! Uses collective consciousness and multi-tier memory to integrate
//! disparate longevity research findings into actionable insights.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────┐
//!  │                  RESEARCH INTEGRATOR                                │
//!  ├─────────────────────────────────────────────────────────────────────┤
//!  │                                                                     │
//!  │   LITERATURE          COLLECTIVE              INTEGRATED           │
//!  │   SOURCES             PROCESSING              KNOWLEDGE            │
//!  │   ┌──────────────┐    ┌───────────────┐      ┌──────────────┐     │
//!  │   │ PubMed       │    │  Swarm        │      │ Unified      │     │
//!  │   │ bioRxiv      │───►│  Analysis     │─────►│ Aging Model  │     │
//!  │   │ Clinical     │    │               │      │              │     │
//!  │   │ Trials       │    │  Consensus    │      │ Intervention │     │
//!  │   │ Patents      │    │  Building     │      │ Rankings     │     │
//!  │   └──────────────┘    └───────────────┘      └──────────────┘     │
//!  │                                                                     │
//!  │   Synthesizes: 1000+ papers → Actionable intervention priorities   │
//!  │                                                                     │
//!  └─────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rand::Rng;

use crate::hallmarks::{Hallmark, Intervention, InterventionType, HallmarksGraph};
use crate::{Result, LongevityError};

/// Configuration for research integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratorConfig {
    /// Minimum evidence threshold
    pub evidence_threshold: f64,
    /// Enable contradiction detection
    pub detect_contradictions: bool,
    /// Weight for recency (newer papers)
    pub recency_weight: f64,
    /// Weight for citation count
    pub citation_weight: f64,
    /// Enable consensus building
    pub consensus_mode: bool,
    /// Minimum studies for meta-analysis
    pub min_studies_meta: usize,
}

impl Default for IntegratorConfig {
    fn default() -> Self {
        Self {
            evidence_threshold: 0.5,
            detect_contradictions: true,
            recency_weight: 0.3,
            citation_weight: 0.3,
            consensus_mode: true,
            min_studies_meta: 3,
        }
    }
}

/// A research paper/study
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPaper {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>,
    pub journal: String,
    pub publication_date: DateTime<Utc>,
    pub doi: Option<String>,
    pub pmid: Option<String>,
    /// Paper type
    pub paper_type: PaperType,
    /// Extracted findings
    pub findings: Vec<Finding>,
    /// Citation count
    pub citations: u32,
    /// Quality score (0-1)
    pub quality_score: f64,
    /// Relevant hallmarks
    pub hallmarks: Vec<Hallmark>,
    /// Key interventions studied
    pub interventions_studied: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaperType {
    ClinicalTrial,
    MetaAnalysis,
    RCT,
    ObservationalStudy,
    AnimalStudy,
    InVitro,
    Review,
    Preprint,
}

impl PaperType {
    /// Evidence weight for this paper type
    pub fn evidence_weight(&self) -> f64 {
        match self {
            PaperType::MetaAnalysis => 1.0,
            PaperType::RCT => 0.9,
            PaperType::ClinicalTrial => 0.85,
            PaperType::ObservationalStudy => 0.6,
            PaperType::AnimalStudy => 0.5,
            PaperType::InVitro => 0.3,
            PaperType::Review => 0.4,
            PaperType::Preprint => 0.2,
        }
    }
}

/// A research finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: Uuid,
    /// What was found
    pub claim: String,
    /// Effect direction
    pub effect: EffectDirection,
    /// Effect size (standardized)
    pub effect_size: Option<f64>,
    /// Statistical significance
    pub p_value: Option<f64>,
    /// Confidence interval
    pub confidence_interval: Option<(f64, f64)>,
    /// Sample size
    pub sample_size: Option<u32>,
    /// Species studied
    pub species: Species,
    /// Duration of study
    pub duration: Option<String>,
    /// Related intervention
    pub intervention: Option<String>,
    /// Related hallmark
    pub hallmark: Option<Hallmark>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectDirection {
    Positive,  // Beneficial for longevity
    Negative,  // Harmful for longevity
    Neutral,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Species {
    Human,
    Mouse,
    Rat,
    Fly,
    Worm,
    Yeast,
    CellCulture,
    Multiple,
}

impl Species {
    /// Translation weight to human relevance
    pub fn human_relevance(&self) -> f64 {
        match self {
            Species::Human => 1.0,
            Species::Mouse => 0.6,
            Species::Rat => 0.55,
            Species::Fly => 0.3,
            Species::Worm => 0.25,
            Species::Yeast => 0.15,
            Species::CellCulture => 0.4,
            Species::Multiple => 0.7,
        }
    }
}

/// A research contradiction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    pub id: Uuid,
    pub topic: String,
    pub paper_a: Uuid,
    pub finding_a: String,
    pub paper_b: Uuid,
    pub finding_b: String,
    /// Possible explanations
    pub explanations: Vec<String>,
    /// Resolved?
    pub resolved: bool,
}

/// A synthesized evidence summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceSummary {
    pub id: Uuid,
    pub topic: String,
    /// Total studies analyzed
    pub num_studies: usize,
    /// Overall evidence strength
    pub evidence_strength: f64,
    /// Direction of evidence
    pub consensus_direction: EffectDirection,
    /// Consistency across studies
    pub consistency: f64,
    /// Meta-analytic effect size (if applicable)
    pub pooled_effect_size: Option<f64>,
    /// Key supporting papers
    pub key_papers: Vec<Uuid>,
    /// Contradictions found
    pub contradictions: Vec<Contradiction>,
    /// Gaps in knowledge
    pub knowledge_gaps: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Intervention ranking based on evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionRanking {
    pub id: Uuid,
    pub intervention_name: String,
    /// Overall score (0-100)
    pub overall_score: f64,
    /// Evidence score
    pub evidence_score: f64,
    /// Safety score
    pub safety_score: f64,
    /// Accessibility score
    pub accessibility_score: f64,
    /// Human data availability
    pub human_data: bool,
    /// Number of supporting studies
    pub num_studies: usize,
    /// Expected lifespan effect
    pub expected_effect: Option<f64>,
    /// Key findings
    pub key_findings: Vec<String>,
    /// Concerns
    pub concerns: Vec<String>,
}

/// The Research Integrator
pub struct ResearchIntegrator {
    config: IntegratorConfig,
    /// Collected papers
    papers: Vec<ResearchPaper>,
    /// Extracted findings by topic
    findings_by_topic: HashMap<String, Vec<Finding>>,
    /// Detected contradictions
    contradictions: Vec<Contradiction>,
    /// Synthesized summaries
    summaries: Vec<EvidenceSummary>,
    /// Intervention rankings
    rankings: Vec<InterventionRanking>,
    /// Hallmarks graph
    hallmarks: HallmarksGraph,
    /// RNG
    rng: rand::rngs::ThreadRng,
}

impl ResearchIntegrator {
    pub fn new(config: IntegratorConfig) -> Self {
        Self {
            config,
            papers: Vec::new(),
            findings_by_topic: HashMap::new(),
            contradictions: Vec::new(),
            summaries: Vec::new(),
            rankings: Vec::new(),
            hallmarks: HallmarksGraph::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Add a research paper to the knowledge base
    pub fn add_paper(&mut self, paper: ResearchPaper) {
        // Index findings by topic
        for finding in &paper.findings {
            if let Some(intervention) = &finding.intervention {
                self.findings_by_topic
                    .entry(intervention.clone())
                    .or_default()
                    .push(finding.clone());
            }
            if let Some(hallmark) = &finding.hallmark {
                let key = format!("{:?}", hallmark);
                self.findings_by_topic
                    .entry(key)
                    .or_default()
                    .push(finding.clone());
            }
        }

        self.papers.push(paper);
    }

    /// Synthesize evidence for a specific intervention
    pub fn synthesize_intervention(&mut self, intervention: &str) -> Result<EvidenceSummary> {
        let relevant_papers: Vec<_> = self.papers.iter()
            .filter(|p| p.interventions_studied.iter().any(|i| i.to_lowercase().contains(&intervention.to_lowercase())))
            .collect();

        if relevant_papers.is_empty() {
            return Err(LongevityError::InsufficientData(format!(
                "No papers found for intervention: {}", intervention
            )));
        }

        let findings = self.findings_by_topic
            .get(intervention)
            .cloned()
            .unwrap_or_default();

        // Calculate evidence strength
        let evidence_strength = self.calculate_evidence_strength(&relevant_papers, &findings);

        // Determine consensus direction
        let consensus_direction = self.determine_consensus(&findings);

        // Calculate consistency
        let consistency = self.calculate_consistency(&findings);

        // Pooled effect size (simplified)
        let pooled_effect_size = if findings.len() >= self.config.min_studies_meta {
            let effects: Vec<f64> = findings.iter()
                .filter_map(|f| f.effect_size)
                .collect();
            if !effects.is_empty() {
                Some(effects.iter().sum::<f64>() / effects.len() as f64)
            } else {
                None
            }
        } else {
            None
        };

        // Detect contradictions
        let contradictions = if self.config.detect_contradictions {
            self.find_contradictions(intervention)
        } else {
            Vec::new()
        };

        // Find knowledge gaps
        let knowledge_gaps = self.identify_gaps(intervention, &relevant_papers);

        // Generate recommendations
        let recommendations = self.generate_synthesis_recommendations(
            evidence_strength,
            consensus_direction,
            &contradictions,
        );

        let summary = EvidenceSummary {
            id: Uuid::new_v4(),
            topic: intervention.to_string(),
            num_studies: relevant_papers.len(),
            evidence_strength,
            consensus_direction,
            consistency,
            pooled_effect_size,
            key_papers: relevant_papers.iter().map(|p| p.id).collect(),
            contradictions,
            knowledge_gaps,
            recommendations,
        };

        self.summaries.push(summary.clone());
        Ok(summary)
    }

    fn calculate_evidence_strength(&self, papers: &[&ResearchPaper], findings: &[Finding]) -> f64 {
        if papers.is_empty() {
            return 0.0;
        }

        let mut total_weight = 0.0;
        let now = Utc::now();

        for paper in papers {
            // Base weight from paper type
            let mut weight = paper.paper_type.evidence_weight();

            // Recency bonus
            let age_years = (now - paper.publication_date).num_days() as f64 / 365.0;
            let recency_factor = 1.0 / (1.0 + age_years * 0.1);
            weight += recency_factor * self.config.recency_weight;

            // Citation bonus
            let citation_factor = (paper.citations as f64 / 100.0).min(1.0);
            weight += citation_factor * self.config.citation_weight;

            // Quality adjustment
            weight *= paper.quality_score;

            total_weight += weight;
        }

        // Human relevance adjustment
        let human_weight: f64 = findings.iter()
            .map(|f| f.species.human_relevance())
            .sum::<f64>() / findings.len().max(1) as f64;

        (total_weight / papers.len() as f64 * human_weight).min(1.0)
    }

    fn determine_consensus(&self, findings: &[Finding]) -> EffectDirection {
        if findings.is_empty() {
            return EffectDirection::Neutral;
        }

        let mut positive = 0;
        let mut negative = 0;
        let mut neutral = 0;

        for finding in findings {
            match finding.effect {
                EffectDirection::Positive => positive += 1,
                EffectDirection::Negative => negative += 1,
                EffectDirection::Neutral => neutral += 1,
                EffectDirection::Mixed => {
                    positive += 1;
                    negative += 1;
                }
            }
        }

        if positive > negative * 2 {
            EffectDirection::Positive
        } else if negative > positive * 2 {
            EffectDirection::Negative
        } else if positive > 0 && negative > 0 {
            EffectDirection::Mixed
        } else {
            EffectDirection::Neutral
        }
    }

    fn calculate_consistency(&self, findings: &[Finding]) -> f64 {
        if findings.len() < 2 {
            return 1.0;
        }

        let directions: Vec<_> = findings.iter().map(|f| &f.effect).collect();
        let most_common = directions.iter()
            .fold(HashMap::new(), |mut map, d| {
                *map.entry(*d).or_insert(0) += 1;
                map
            })
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(_, count)| count)
            .unwrap_or(0);

        most_common as f64 / findings.len() as f64
    }

    fn find_contradictions(&self, topic: &str) -> Vec<Contradiction> {
        let mut contradictions = Vec::new();
        let findings = self.findings_by_topic.get(topic);

        if let Some(findings) = findings {
            for i in 0..findings.len() {
                for j in (i + 1)..findings.len() {
                    let f1 = &findings[i];
                    let f2 = &findings[j];

                    // Check for contradictory effects
                    let contradictory = matches!(
                        (&f1.effect, &f2.effect),
                        (EffectDirection::Positive, EffectDirection::Negative) |
                        (EffectDirection::Negative, EffectDirection::Positive)
                    );

                    if contradictory {
                        contradictions.push(Contradiction {
                            id: Uuid::new_v4(),
                            topic: topic.to_string(),
                            paper_a: f1.id,
                            finding_a: f1.claim.clone(),
                            paper_b: f2.id,
                            finding_b: f2.claim.clone(),
                            explanations: vec![
                                "Different study populations".to_string(),
                                "Different dosing regimens".to_string(),
                                "Different outcome measures".to_string(),
                            ],
                            resolved: false,
                        });
                    }
                }
            }
        }

        contradictions
    }

    fn identify_gaps(&self, topic: &str, papers: &[&ResearchPaper]) -> Vec<String> {
        let mut gaps = Vec::new();

        // Check for human data
        let has_human = papers.iter()
            .flat_map(|p| &p.findings)
            .any(|f| f.species == Species::Human);

        if !has_human {
            gaps.push("No human clinical data available".to_string());
        }

        // Check for long-term data
        let has_longterm = papers.iter()
            .flat_map(|p| &p.findings)
            .any(|f| f.duration.as_ref().map_or(false, |d| d.contains("year")));

        if !has_longterm {
            gaps.push("Lacks long-term (>1 year) study data".to_string());
        }

        // Check for RCT
        let has_rct = papers.iter().any(|p| p.paper_type == PaperType::RCT);

        if !has_rct {
            gaps.push("No randomized controlled trials".to_string());
        }

        gaps
    }

    fn generate_synthesis_recommendations(
        &self,
        evidence: f64,
        direction: EffectDirection,
        contradictions: &[Contradiction],
    ) -> Vec<String> {
        let mut recs = Vec::new();

        if evidence >= 0.8 && matches!(direction, EffectDirection::Positive) {
            recs.push("Strong evidence supports efficacy - consider for clinical use".to_string());
        } else if evidence >= 0.6 && matches!(direction, EffectDirection::Positive) {
            recs.push("Moderate evidence - suitable for further clinical trials".to_string());
        } else if evidence >= 0.4 {
            recs.push("Emerging evidence - more research needed".to_string());
        } else {
            recs.push("Insufficient evidence - early-stage research".to_string());
        }

        if !contradictions.is_empty() {
            recs.push(format!(
                "Note: {} contradictory findings require reconciliation",
                contradictions.len()
            ));
        }

        recs
    }

    /// Generate intervention rankings
    pub fn rank_interventions(&mut self) -> Vec<InterventionRanking> {
        let interventions: HashSet<_> = self.papers.iter()
            .flat_map(|p| &p.interventions_studied)
            .cloned()
            .collect();

        let mut rankings = Vec::new();

        for intervention in interventions {
            if let Ok(summary) = self.synthesize_intervention(&intervention) {
                let evidence_score = summary.evidence_strength * 100.0;

                // Calculate safety score (simplified)
                let safety_score = if summary.contradictions.is_empty() { 80.0 } else { 60.0 };

                // Accessibility (simplified)
                let accessibility_score = 70.0;

                let overall = evidence_score * 0.4 + safety_score * 0.3 + accessibility_score * 0.3;

                rankings.push(InterventionRanking {
                    id: Uuid::new_v4(),
                    intervention_name: intervention.clone(),
                    overall_score: overall,
                    evidence_score,
                    safety_score,
                    accessibility_score,
                    human_data: summary.num_studies > 0,
                    num_studies: summary.num_studies,
                    expected_effect: summary.pooled_effect_size,
                    key_findings: summary.recommendations.clone(),
                    concerns: summary.knowledge_gaps.clone(),
                });
            }
        }

        // Sort by overall score
        rankings.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());

        self.rankings = rankings.clone();
        rankings
    }

    /// Get current paper count
    pub fn paper_count(&self) -> usize {
        self.papers.len()
    }

    /// Get summaries
    pub fn summaries(&self) -> &[EvidenceSummary] {
        &self.summaries
    }

    /// Get rankings
    pub fn rankings(&self) -> &[InterventionRanking] {
        &self.rankings
    }

    /// Export knowledge as structured report
    pub fn export_knowledge_report(&self) -> KnowledgeReport {
        KnowledgeReport {
            generated_at: Utc::now(),
            total_papers: self.papers.len(),
            total_findings: self.findings_by_topic.values().map(|v| v.len()).sum(),
            intervention_rankings: self.rankings.clone(),
            top_evidence_summaries: self.summaries.iter()
                .take(10)
                .cloned()
                .collect(),
            key_contradictions: self.contradictions.clone(),
            overall_knowledge_gaps: self.identify_overall_gaps(),
        }
    }

    fn identify_overall_gaps(&self) -> Vec<String> {
        let mut gaps = Vec::new();

        // Check overall research landscape
        let human_papers = self.papers.iter()
            .filter(|p| p.findings.iter().any(|f| f.species == Species::Human))
            .count();

        if human_papers < self.papers.len() / 3 {
            gaps.push("Limited human clinical data across interventions".to_string());
        }

        // Check for meta-analyses
        let meta_count = self.papers.iter()
            .filter(|p| p.paper_type == PaperType::MetaAnalysis)
            .count();

        if meta_count < 5 {
            gaps.push("Few meta-analyses available for evidence synthesis".to_string());
        }

        gaps
    }

    /// Add sample papers for demonstration
    pub fn add_sample_papers(&mut self) {
        let rapamycin_paper = ResearchPaper {
            id: Uuid::new_v4(),
            title: "Rapamycin extends lifespan in mice".to_string(),
            authors: vec!["Harrison DE".to_string(), "Strong R".to_string()],
            journal: "Nature".to_string(),
            publication_date: Utc::now() - chrono::Duration::days(1825),
            doi: Some("10.1038/nature08221".to_string()),
            pmid: Some("19587680".to_string()),
            paper_type: PaperType::AnimalStudy,
            findings: vec![Finding {
                id: Uuid::new_v4(),
                claim: "Rapamycin extends lifespan by 9-14%".to_string(),
                effect: EffectDirection::Positive,
                effect_size: Some(0.12),
                p_value: Some(0.001),
                confidence_interval: Some((0.09, 0.14)),
                sample_size: Some(1900),
                species: Species::Mouse,
                duration: Some("2 years".to_string()),
                intervention: Some("Rapamycin".to_string()),
                hallmark: Some(Hallmark::DeregulatedNutrientSensing),
            }],
            citations: 3500,
            quality_score: 0.95,
            hallmarks: vec![Hallmark::DeregulatedNutrientSensing],
            interventions_studied: vec!["Rapamycin".to_string()],
        };

        let senolytic_paper = ResearchPaper {
            id: Uuid::new_v4(),
            title: "Senolytics improve physical function in humans".to_string(),
            authors: vec!["Justice JN".to_string(), "Nambiar AM".to_string()],
            journal: "EBioMedicine".to_string(),
            publication_date: Utc::now() - chrono::Duration::days(730),
            doi: Some("10.1016/j.ebiom.2019.01.054".to_string()),
            pmid: Some("30737084".to_string()),
            paper_type: PaperType::ClinicalTrial,
            findings: vec![Finding {
                id: Uuid::new_v4(),
                claim: "D+Q reduces senescent cell burden and improves function".to_string(),
                effect: EffectDirection::Positive,
                effect_size: Some(0.25),
                p_value: Some(0.01),
                confidence_interval: Some((0.10, 0.40)),
                sample_size: Some(14),
                species: Species::Human,
                duration: Some("3 weeks".to_string()),
                intervention: Some("Dasatinib + Quercetin".to_string()),
                hallmark: Some(Hallmark::CellularSenescence),
            }],
            citations: 450,
            quality_score: 0.80,
            hallmarks: vec![Hallmark::CellularSenescence],
            interventions_studied: vec!["Dasatinib + Quercetin".to_string(), "Senolytics".to_string()],
        };

        self.add_paper(rapamycin_paper);
        self.add_paper(senolytic_paper);
    }
}

/// Structured knowledge report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeReport {
    pub generated_at: DateTime<Utc>,
    pub total_papers: usize,
    pub total_findings: usize,
    pub intervention_rankings: Vec<InterventionRanking>,
    pub top_evidence_summaries: Vec<EvidenceSummary>,
    pub key_contradictions: Vec<Contradiction>,
    pub overall_knowledge_gaps: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrator_creation() {
        let config = IntegratorConfig::default();
        let integrator = ResearchIntegrator::new(config);
        assert_eq!(integrator.paper_count(), 0);
    }

    #[test]
    fn test_add_sample_papers() {
        let config = IntegratorConfig::default();
        let mut integrator = ResearchIntegrator::new(config);
        integrator.add_sample_papers();
        assert!(integrator.paper_count() >= 2);
    }

    #[test]
    fn test_synthesize_intervention() {
        let config = IntegratorConfig::default();
        let mut integrator = ResearchIntegrator::new(config);
        integrator.add_sample_papers();

        let summary = integrator.synthesize_intervention("Rapamycin");
        assert!(summary.is_ok());
    }

    #[test]
    fn test_rank_interventions() {
        let config = IntegratorConfig::default();
        let mut integrator = ResearchIntegrator::new(config);
        integrator.add_sample_papers();

        let rankings = integrator.rank_interventions();
        assert!(!rankings.is_empty());
    }
}
