//! Biomarker Dreamer - Novel Intervention Discovery Through Dream States
//!
//! Leverages the Omega Brain's Dream Solver to discover novel connections
//! between aging biomarkers, pathways, and potential interventions that
//! conscious analysis would miss.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────┐
//!  │                  BIOMARKER DREAMER                                  │
//!  ├─────────────────────────────────────────────────────────────────────┤
//!  │                                                                     │
//!  │  KNOWLEDGE BASE          REM EXPLORATION         NOVEL INSIGHTS    │
//!  │  ┌──────────────┐        ┌───────────────┐      ┌──────────────┐  │
//!  │  │ Biomarkers   │        │   Forbidden   │      │ New Drug     │  │
//!  │  │ Pathways     │───────►│   Connections │─────►│ Targets      │  │
//!  │  │ Interventions│        │   Explored    │      │ Discovered   │  │
//!  │  │ Literature   │        │               │      │              │  │
//!  │  └──────────────┘        └───────────────┘      └──────────────┘  │
//!  │                                                                     │
//!  │  Example Discovery:                                                 │
//!  │  "Gut microbiome metabolite → Epigenetic clock → Novel senolytic"  │
//!  │                                                                     │
//!  └─────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use uuid::Uuid;
use rand::Rng;

use crate::hallmarks::{Hallmark, HallmarksGraph, Intervention, InterventionType, NovelHallmarkConnection};
use crate::{Result, LongevityError};

/// Configuration for the biomarker dreamer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamerConfig {
    /// Duration of each dream cycle
    pub cycle_duration: Duration,
    /// Number of REM cycles per session
    pub rem_cycles: usize,
    /// Minimum confidence to report an insight
    pub insight_threshold: f64,
    /// Enable cross-domain exploration
    pub cross_domain: bool,
    /// Maximum forbidden connections to explore
    pub max_forbidden: usize,
    /// Enable combination therapy discovery
    pub combination_mode: bool,
}

impl Default for DreamerConfig {
    fn default() -> Self {
        Self {
            cycle_duration: Duration::from_secs(60),
            rem_cycles: 5,
            insight_threshold: 0.6,
            cross_domain: true,
            max_forbidden: 100,
            combination_mode: true,
        }
    }
}

/// A biomarker of aging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgingBiomarker {
    pub id: Uuid,
    pub name: String,
    pub category: BiomarkerCategory,
    /// Associated hallmarks
    pub hallmarks: Vec<Hallmark>,
    /// Molecular pathway
    pub pathway: String,
    /// Direction with age (increases or decreases)
    pub age_direction: AgeDirection,
    /// Modifiability (can interventions change it?)
    pub modifiable: bool,
    /// Known correlations with other biomarkers
    pub correlations: Vec<(Uuid, f64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomarkerCategory {
    Epigenetic,      // DNA methylation clocks, etc.
    Proteomic,       // Protein levels
    Metabolomic,     // Metabolites
    Inflammatory,    // Cytokines, CRP
    Functional,      // Grip strength, VO2max
    Imaging,         // Brain volume, etc.
    Cellular,        // Telomere length, senescent cells
    Composite,       // Biological age calculators
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgeDirection {
    Increases,
    Decreases,
    NonLinear,
}

/// A potential drug target discovered through dreaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelTarget {
    pub id: Uuid,
    pub name: String,
    pub target_type: TargetType,
    /// How it was discovered
    pub discovery_path: Vec<String>,
    /// Confidence in this target
    pub confidence: f64,
    /// Affected hallmarks
    pub affected_hallmarks: Vec<Hallmark>,
    /// Predicted mechanism
    pub mechanism_hypothesis: String,
    /// Was this a "forbidden" connection?
    pub from_forbidden: bool,
    /// Suggested validation experiments
    pub validation_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetType {
    Protein,
    Enzyme,
    Receptor,
    Transcription,
    Epigenetic,
    Metabolic,
    Microbiome,
    Unknown,
}

/// A combination therapy insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinationInsight {
    pub id: Uuid,
    /// Interventions to combine
    pub interventions: Vec<String>,
    /// Synergy hypothesis
    pub synergy_rationale: String,
    /// Predicted synergy score
    pub synergy_score: f64,
    /// Hallmarks addressed
    pub hallmarks_targeted: Vec<Hallmark>,
    /// Potential concerns
    pub concerns: Vec<String>,
}

/// A dream session result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamSession {
    pub id: Uuid,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub duration: Duration,
    pub problem_focus: String,
    pub novel_targets: Vec<NovelTarget>,
    pub combination_insights: Vec<CombinationInsight>,
    pub hallmark_connections: Vec<NovelHallmarkConnection>,
    pub total_insights: usize,
}

/// The Biomarker Dreamer system
pub struct BiomarkerDreamer {
    config: DreamerConfig,
    /// Knowledge base: biomarkers
    biomarkers: Vec<AgingBiomarker>,
    /// Knowledge base: pathways
    pathways: HashMap<String, Vec<String>>,
    /// Forbidden connections (unexplored but possible)
    forbidden: HashSet<(String, String)>,
    /// Discovered targets
    discoveries: Vec<NovelTarget>,
    /// Combination insights
    combinations: Vec<CombinationInsight>,
    /// Session history
    sessions: Vec<DreamSession>,
    /// Hallmarks graph
    hallmarks: HallmarksGraph,
    /// RNG
    rng: rand::rngs::ThreadRng,
}

impl BiomarkerDreamer {
    pub fn new(config: DreamerConfig) -> Self {
        let mut dreamer = Self {
            config,
            biomarkers: Vec::new(),
            pathways: HashMap::new(),
            forbidden: HashSet::new(),
            discoveries: Vec::new(),
            combinations: Vec::new(),
            sessions: Vec::new(),
            hallmarks: HallmarksGraph::new(),
            rng: rand::thread_rng(),
        };

        dreamer.initialize_knowledge_base();
        dreamer.generate_forbidden_connections();

        dreamer
    }

    /// Initialize with known biomarkers and pathways
    fn initialize_knowledge_base(&mut self) {
        // Add key aging biomarkers
        self.add_biomarker("GrimAge", BiomarkerCategory::Epigenetic, vec![Hallmark::EpigeneticAlterations], AgeDirection::Increases);
        self.add_biomarker("PhenoAge", BiomarkerCategory::Composite, vec![Hallmark::ChronicInflammation], AgeDirection::Increases);
        self.add_biomarker("TelomereLength", BiomarkerCategory::Cellular, vec![Hallmark::TelomereAttrition], AgeDirection::Decreases);
        self.add_biomarker("p16INK4a", BiomarkerCategory::Cellular, vec![Hallmark::CellularSenescence], AgeDirection::Increases);
        self.add_biomarker("NAD+", BiomarkerCategory::Metabolomic, vec![Hallmark::MitochondrialDysfunction], AgeDirection::Decreases);
        self.add_biomarker("IL-6", BiomarkerCategory::Inflammatory, vec![Hallmark::ChronicInflammation], AgeDirection::Increases);
        self.add_biomarker("GDF15", BiomarkerCategory::Proteomic, vec![Hallmark::CellularSenescence, Hallmark::MitochondrialDysfunction], AgeDirection::Increases);
        self.add_biomarker("Cystatin C", BiomarkerCategory::Proteomic, vec![Hallmark::AlteredIntercellularCommunication], AgeDirection::Increases);
        self.add_biomarker("Albumin", BiomarkerCategory::Proteomic, vec![Hallmark::LossOfProteostasis], AgeDirection::Decreases);
        self.add_biomarker("SASP Index", BiomarkerCategory::Composite, vec![Hallmark::CellularSenescence], AgeDirection::Increases);

        // Add key pathways
        self.pathways.insert("mTOR".to_string(), vec![
            "Raptor".to_string(), "Rictor".to_string(), "S6K1".to_string(),
            "4E-BP1".to_string(), "Autophagy".to_string(),
        ]);
        self.pathways.insert("AMPK".to_string(), vec![
            "LKB1".to_string(), "ACC".to_string(), "PGC-1α".to_string(),
            "FOXO".to_string(), "Mitochondria".to_string(),
        ]);
        self.pathways.insert("Sirtuins".to_string(), vec![
            "SIRT1".to_string(), "SIRT3".to_string(), "SIRT6".to_string(),
            "NAD+".to_string(), "Deacetylation".to_string(),
        ]);
        self.pathways.insert("Senescence".to_string(), vec![
            "p53".to_string(), "p21".to_string(), "p16".to_string(),
            "Rb".to_string(), "SASP".to_string(), "BCL-2".to_string(),
        ]);
        self.pathways.insert("Inflammation".to_string(), vec![
            "NF-κB".to_string(), "NLRP3".to_string(), "IL-6".to_string(),
            "TNF-α".to_string(), "Inflammasome".to_string(),
        ]);
    }

    fn add_biomarker(&mut self, name: &str, category: BiomarkerCategory, hallmarks: Vec<Hallmark>, direction: AgeDirection) {
        self.biomarkers.push(AgingBiomarker {
            id: Uuid::new_v4(),
            name: name.to_string(),
            category,
            hallmarks,
            pathway: String::new(),
            age_direction: direction,
            modifiable: true,
            correlations: Vec::new(),
        });
    }

    /// Generate "forbidden" connections - paths not yet explored
    fn generate_forbidden_connections(&mut self) {
        // Cross-domain connections (e.g., gut → brain, metabolism → epigenetics)
        let domains = vec![
            ("Microbiome", vec!["Gut bacteria", "Metabolites", "SCFA", "LPS"]),
            ("Epigenetic", vec!["DNAm", "Histones", "lncRNA", "Chromatin"]),
            ("Metabolic", vec!["NAD+", "α-KG", "Acetyl-CoA", "ATP"]),
            ("Immune", vec!["T cells", "Macrophages", "Cytokines", "Inflammasome"]),
            ("Neural", vec!["Neurons", "Glia", "Synapses", "Neurogenesis"]),
            ("Stem", vec!["HSC", "MSC", "ISC", "NSC"]),
        ];

        for (d1, items1) in &domains {
            for (d2, items2) in &domains {
                if d1 != d2 {
                    for item1 in items1 {
                        for item2 in items2 {
                            if self.forbidden.len() < self.config.max_forbidden {
                                self.forbidden.insert((item1.to_string(), item2.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }

    /// Run a dream session focused on a specific problem
    pub fn dream_session(&mut self, problem: &str) -> Result<DreamSession> {
        let session_id = Uuid::new_v4();
        let started_at = chrono::Utc::now();
        let mut novel_targets = Vec::new();
        let mut combination_insights = Vec::new();
        let mut hallmark_connections = Vec::new();

        // Phase 1: N1/N2 - Memory consolidation
        // Associate biomarkers with problem space
        let relevant_biomarkers = self.find_relevant_biomarkers(problem);

        // Phase 2: N3 - Deep processing
        // Strengthen pathway connections
        let pathway_insights = self.deep_pathway_analysis(problem, &relevant_biomarkers);

        // Phase 3: REM - Creative exploration
        for cycle in 0..self.config.rem_cycles {
            // Explore forbidden connections
            if let Some(target) = self.explore_forbidden_connection(problem) {
                novel_targets.push(target);
            }

            // Try combination insights
            if self.config.combination_mode {
                if let Some(combo) = self.explore_combinations(problem) {
                    combination_insights.push(combo);
                }
            }

            // Cross-hallmark connections
            if let Some(connection) = self.explore_hallmark_bridge(problem) {
                hallmark_connections.push(connection);
            }
        }

        let total_insights = novel_targets.len() + combination_insights.len() + hallmark_connections.len();

        // Store discoveries
        self.discoveries.extend(novel_targets.clone());
        self.combinations.extend(combination_insights.clone());

        // Add connections to hallmarks graph
        for conn in &hallmark_connections {
            self.hallmarks.add_novel_connection(conn.clone());
        }

        let session = DreamSession {
            id: session_id,
            started_at,
            duration: self.config.cycle_duration * self.config.rem_cycles as u32,
            problem_focus: problem.to_string(),
            novel_targets,
            combination_insights,
            hallmark_connections,
            total_insights,
        };

        self.sessions.push(session.clone());
        Ok(session)
    }

    fn find_relevant_biomarkers(&self, problem: &str) -> Vec<&AgingBiomarker> {
        let problem_lower = problem.to_lowercase();

        self.biomarkers.iter()
            .filter(|b| {
                problem_lower.contains(&b.name.to_lowercase()) ||
                b.hallmarks.iter().any(|h| format!("{:?}", h).to_lowercase().contains(&problem_lower))
            })
            .collect()
    }

    fn deep_pathway_analysis(&self, _problem: &str, biomarkers: &[&AgingBiomarker]) -> Vec<String> {
        let mut insights = Vec::new();

        for biomarker in biomarkers {
            for (pathway_name, components) in &self.pathways {
                // Check if biomarker might connect to this pathway
                if self.could_connect(biomarker, pathway_name) {
                    insights.push(format!(
                        "{} may modulate {} through {}",
                        biomarker.name, pathway_name, components.join("/")
                    ));
                }
            }
        }

        insights
    }

    fn could_connect(&self, biomarker: &AgingBiomarker, pathway: &str) -> bool {
        // Simple heuristic - in reality would use embedding similarity
        match biomarker.category {
            BiomarkerCategory::Metabolomic => pathway == "AMPK" || pathway == "Sirtuins",
            BiomarkerCategory::Inflammatory => pathway == "Inflammation" || pathway == "Senescence",
            BiomarkerCategory::Epigenetic => pathway == "Sirtuins" || pathway == "mTOR",
            _ => self.rng.clone().gen::<f64>() > 0.7,
        }
    }

    /// Explore a forbidden connection in dream state
    fn explore_forbidden_connection(&mut self, problem: &str) -> Option<NovelTarget> {
        if self.forbidden.is_empty() {
            return None;
        }

        // Random selection in dream state
        let forbidden_vec: Vec<_> = self.forbidden.iter().cloned().collect();
        let idx = self.rng.gen_range(0..forbidden_vec.len());
        let (from, to) = &forbidden_vec[idx];

        // Generate hypothesis
        let confidence = self.rng.gen_range(0.4..0.9);

        if confidence < self.config.insight_threshold {
            return None;
        }

        // Generate bridging mechanism
        let mechanism = self.generate_mechanism_hypothesis(from, to);
        let affected_hallmarks = self.infer_hallmarks(from, to);

        Some(NovelTarget {
            id: Uuid::new_v4(),
            name: format!("{}-{} axis", from, to),
            target_type: TargetType::Unknown,
            discovery_path: vec![
                format!("Started from: {}", from),
                format!("Dream bridge to: {}", to),
                format!("Context: {}", problem),
            ],
            confidence,
            affected_hallmarks,
            mechanism_hypothesis: mechanism,
            from_forbidden: true,
            validation_suggestions: vec![
                format!("Test {} modulation effect on {}", from, to),
                "Conduct pathway enrichment analysis".to_string(),
                "Validate in senescent cell model".to_string(),
            ],
        })
    }

    fn generate_mechanism_hypothesis(&self, from: &str, to: &str) -> String {
        let mechanisms = vec![
            "may activate transcription of",
            "could inhibit degradation of",
            "might epigenetically regulate",
            "could modulate post-translational modification of",
            "may affect membrane localization of",
            "could alter metabolic flux toward",
        ];

        let mut rng = rand::thread_rng();
        let mech = &mechanisms[rng.gen_range(0..mechanisms.len())];
        format!("{} {} {}", from, mech, to)
    }

    fn infer_hallmarks(&self, from: &str, to: &str) -> Vec<Hallmark> {
        let mut hallmarks = Vec::new();

        // Simple keyword matching
        let combined = format!("{} {}", from, to).to_lowercase();

        if combined.contains("dna") || combined.contains("genom") {
            hallmarks.push(Hallmark::GenomicInstability);
        }
        if combined.contains("mitochond") || combined.contains("atp") || combined.contains("nad") {
            hallmarks.push(Hallmark::MitochondrialDysfunction);
        }
        if combined.contains("senesc") || combined.contains("p16") || combined.contains("sasp") {
            hallmarks.push(Hallmark::CellularSenescence);
        }
        if combined.contains("inflam") || combined.contains("cytokine") || combined.contains("nfkb") {
            hallmarks.push(Hallmark::ChronicInflammation);
        }
        if combined.contains("epigenet") || combined.contains("methyl") || combined.contains("histone") {
            hallmarks.push(Hallmark::EpigeneticAlterations);
        }
        if combined.contains("mtor") || combined.contains("insulin") || combined.contains("ampk") {
            hallmarks.push(Hallmark::DeregulatedNutrientSensing);
        }

        if hallmarks.is_empty() {
            // Default to most common
            hallmarks.push(Hallmark::CellularSenescence);
        }

        hallmarks
    }

    /// Explore combination therapies
    fn explore_combinations(&mut self, _problem: &str) -> Option<CombinationInsight> {
        let interventions: Vec<_> = self.hallmarks.top_interventions(10)
            .iter()
            .map(|i| i.name.clone())
            .collect();

        if interventions.len() < 2 {
            return None;
        }

        // Pick random combination
        let idx1 = self.rng.gen_range(0..interventions.len());
        let mut idx2 = self.rng.gen_range(0..interventions.len());
        while idx2 == idx1 {
            idx2 = self.rng.gen_range(0..interventions.len());
        }

        let int1 = &interventions[idx1];
        let int2 = &interventions[idx2];

        let synergy = self.rng.gen_range(0.5..1.0);

        if synergy < 0.7 {
            return None;
        }

        Some(CombinationInsight {
            id: Uuid::new_v4(),
            interventions: vec![int1.clone(), int2.clone()],
            synergy_rationale: format!(
                "{} and {} may synergize by targeting complementary pathways",
                int1, int2
            ),
            synergy_score: synergy,
            hallmarks_targeted: vec![Hallmark::CellularSenescence, Hallmark::MitochondrialDysfunction],
            concerns: vec!["Drug-drug interactions need evaluation".to_string()],
        })
    }

    /// Explore novel hallmark bridges
    fn explore_hallmark_bridge(&mut self, problem: &str) -> Option<NovelHallmarkConnection> {
        let hallmarks = Hallmark::all();
        let h1 = hallmarks[self.rng.gen_range(0..hallmarks.len())];
        let h2 = hallmarks[self.rng.gen_range(0..hallmarks.len())];

        if h1 == h2 {
            return None;
        }

        let confidence = self.rng.gen_range(0.4..0.85);

        if confidence < self.config.insight_threshold {
            return None;
        }

        Some(NovelHallmarkConnection {
            id: Uuid::new_v4(),
            from: h1,
            to: h2,
            hypothesis: format!(
                "{:?} may influence {:?} through an undiscovered pathway (context: {})",
                h1, h2, problem
            ),
            confidence,
            discovered_via: "BiomarkerDreamer REM exploration".to_string(),
            potential_intervention: Some(format!("Target {:?}/{:?} axis", h1, h2)),
            requires_validation: true,
        })
    }

    /// Get all novel targets discovered
    pub fn discoveries(&self) -> &[NovelTarget] {
        &self.discoveries
    }

    /// Get high-confidence discoveries only
    pub fn high_confidence_discoveries(&self, threshold: f64) -> Vec<&NovelTarget> {
        self.discoveries.iter()
            .filter(|d| d.confidence >= threshold)
            .collect()
    }

    /// Get combination therapy insights
    pub fn combination_insights(&self) -> &[CombinationInsight] {
        &self.combinations
    }

    /// Get session history
    pub fn sessions(&self) -> &[DreamSession] {
        &self.sessions
    }

    /// Export discoveries for validation pipeline
    pub fn export_for_validation(&self) -> Vec<ValidationCandidate> {
        self.discoveries.iter()
            .filter(|d| d.from_forbidden && d.confidence >= 0.7)
            .map(|d| ValidationCandidate {
                id: d.id,
                name: d.name.clone(),
                hypothesis: d.mechanism_hypothesis.clone(),
                priority: (d.confidence * 100.0) as u32,
                suggested_experiments: d.validation_suggestions.clone(),
            })
            .collect()
    }
}

/// A candidate for experimental validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCandidate {
    pub id: Uuid,
    pub name: String,
    pub hypothesis: String,
    pub priority: u32,
    pub suggested_experiments: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dreamer_creation() {
        let config = DreamerConfig::default();
        let dreamer = BiomarkerDreamer::new(config);
        assert!(!dreamer.biomarkers.is_empty());
        assert!(!dreamer.forbidden.is_empty());
    }

    #[test]
    fn test_dream_session() {
        let config = DreamerConfig {
            rem_cycles: 3,
            insight_threshold: 0.3, // Lower for testing
            ..Default::default()
        };
        let mut dreamer = BiomarkerDreamer::new(config);

        let session = dreamer.dream_session("cellular senescence").unwrap();
        assert!(!session.problem_focus.is_empty());
    }

    #[test]
    fn test_export_validation() {
        let config = DreamerConfig {
            rem_cycles: 5,
            insight_threshold: 0.3,
            ..Default::default()
        };
        let mut dreamer = BiomarkerDreamer::new(config);

        dreamer.dream_session("mitochondrial dysfunction").unwrap();
        let candidates = dreamer.export_for_validation();
        // May or may not have candidates (probabilistic)
    }
}
