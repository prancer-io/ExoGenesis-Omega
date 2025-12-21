//! Hallmarks of Aging - Knowledge Graph
//!
//! Models the 12+ hallmarks of biological aging as an interconnected
//! causal network, enabling discovery of intervention points.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────┐
//!  │              THE HALLMARKS OF AGING NETWORK                        │
//!  ├─────────────────────────────────────────────────────────────────────┤
//!  │                                                                     │
//!  │   PRIMARY (Causes)     ANTAGONISTIC       INTEGRATIVE              │
//!  │   ┌──────────────┐     ┌──────────────┐   ┌──────────────┐        │
//!  │   │ Genomic      │────►│ Senescence   │──►│ Stem Cell    │        │
//!  │   │ Instability  │     │              │   │ Exhaustion   │        │
//!  │   └──────────────┘     └──────────────┘   └──────────────┘        │
//!  │   ┌──────────────┐     ┌──────────────┐   ┌──────────────┐        │
//!  │   │ Telomere     │────►│ Mitochondrial│──►│ Altered      │        │
//!  │   │ Attrition    │     │ Dysfunction  │   │ Communication│        │
//!  │   └──────────────┘     └──────────────┘   └──────────────┘        │
//!  │   ┌──────────────┐     ┌──────────────┐                           │
//!  │   │ Epigenetic   │────►│ Deregulated  │   Each connection is      │
//!  │   │ Alterations  │     │ Sensing      │   a potential intervention│
//!  │   └──────────────┘     └──────────────┘   point!                  │
//!  │   ┌──────────────┐                                                 │
//!  │   │ Proteostasis │     + Inflammation, Dysbiosis, Autophagy       │
//!  │   │ Loss         │                                                 │
//!  │   └──────────────┘                                                 │
//!  │                                                                     │
//!  └─────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{dijkstra, all_simple_paths};
use petgraph::Direction;

/// The 12+ recognized hallmarks of biological aging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hallmark {
    // Primary Hallmarks (Causes of damage)
    GenomicInstability,
    TelomereAttrition,
    EpigeneticAlterations,
    LossOfProteostasis,

    // Antagonistic Hallmarks (Responses to damage)
    DeregulatedNutrientSensing,
    MitochondrialDysfunction,
    CellularSenescence,

    // Integrative Hallmarks (Culprits of phenotype)
    StemCellExhaustion,
    AlteredIntercellularCommunication,

    // Extended Hallmarks (2023+ additions)
    ChronicInflammation,
    Dysbiosis,
    DisabledMacroautophagy,
}

impl Hallmark {
    /// Get all hallmarks
    pub fn all() -> Vec<Hallmark> {
        vec![
            Hallmark::GenomicInstability,
            Hallmark::TelomereAttrition,
            Hallmark::EpigeneticAlterations,
            Hallmark::LossOfProteostasis,
            Hallmark::DeregulatedNutrientSensing,
            Hallmark::MitochondrialDysfunction,
            Hallmark::CellularSenescence,
            Hallmark::StemCellExhaustion,
            Hallmark::AlteredIntercellularCommunication,
            Hallmark::ChronicInflammation,
            Hallmark::Dysbiosis,
            Hallmark::DisabledMacroautophagy,
        ]
    }

    /// Get hallmark category
    pub fn category(&self) -> HallmarkCategory {
        match self {
            Hallmark::GenomicInstability |
            Hallmark::TelomereAttrition |
            Hallmark::EpigeneticAlterations |
            Hallmark::LossOfProteostasis => HallmarkCategory::Primary,

            Hallmark::DeregulatedNutrientSensing |
            Hallmark::MitochondrialDysfunction |
            Hallmark::CellularSenescence => HallmarkCategory::Antagonistic,

            Hallmark::StemCellExhaustion |
            Hallmark::AlteredIntercellularCommunication => HallmarkCategory::Integrative,

            Hallmark::ChronicInflammation |
            Hallmark::Dysbiosis |
            Hallmark::DisabledMacroautophagy => HallmarkCategory::Extended,
        }
    }

    /// Get known molecular pathways associated with this hallmark
    pub fn pathways(&self) -> Vec<&'static str> {
        match self {
            Hallmark::GenomicInstability => vec![
                "DNA damage response (DDR)",
                "Base excision repair (BER)",
                "Nucleotide excision repair (NER)",
                "Homologous recombination",
                "Non-homologous end joining",
                "p53 signaling",
            ],
            Hallmark::TelomereAttrition => vec![
                "Telomerase (TERT/TERC)",
                "Shelterin complex",
                "ALT pathway",
                "Senescence signaling",
            ],
            Hallmark::EpigeneticAlterations => vec![
                "DNA methylation (DNMT)",
                "Histone modifications",
                "Chromatin remodeling",
                "Sirtuins (SIRT1-7)",
                "Polycomb/Trithorax",
            ],
            Hallmark::LossOfProteostasis => vec![
                "Heat shock response (HSF1)",
                "Unfolded protein response (UPR)",
                "Ubiquitin-proteasome system",
                "Autophagy-lysosome",
                "Chaperone network",
            ],
            Hallmark::DeregulatedNutrientSensing => vec![
                "mTOR signaling",
                "AMPK pathway",
                "Insulin/IGF-1 signaling",
                "Sirtuins",
                "FOXO transcription factors",
            ],
            Hallmark::MitochondrialDysfunction => vec![
                "Electron transport chain",
                "Mitophagy (PINK1/Parkin)",
                "Mitochondrial biogenesis (PGC-1α)",
                "ROS signaling",
                "NAD+ metabolism",
            ],
            Hallmark::CellularSenescence => vec![
                "p16INK4a/Rb pathway",
                "p53/p21 pathway",
                "SASP (Senescence-Associated Secretory Phenotype)",
                "Senolytic targets (BCL-2 family)",
            ],
            Hallmark::StemCellExhaustion => vec![
                "Wnt signaling",
                "Notch pathway",
                "Hedgehog signaling",
                "Niche factors",
                "Epigenetic reprogramming",
            ],
            Hallmark::AlteredIntercellularCommunication => vec![
                "NF-κB signaling",
                "Inflammasome (NLRP3)",
                "Cytokine networks",
                "Exosome signaling",
                "Hormonal regulation",
            ],
            Hallmark::ChronicInflammation => vec![
                "Inflammaging",
                "IL-6/TNF-α signaling",
                "Complement system",
                "Tissue-resident macrophages",
            ],
            Hallmark::Dysbiosis => vec![
                "Gut-brain axis",
                "Microbiome metabolites",
                "Intestinal barrier",
                "Immune modulation",
            ],
            Hallmark::DisabledMacroautophagy => vec![
                "LC3/ATG proteins",
                "mTOR inhibition",
                "TFEB activation",
                "Selective autophagy receptors",
            ],
        }
    }

    /// Get known interventions targeting this hallmark
    pub fn known_interventions(&self) -> Vec<Intervention> {
        match self {
            Hallmark::DeregulatedNutrientSensing => vec![
                Intervention::new("Rapamycin", InterventionType::SmallMolecule, 0.85),
                Intervention::new("Metformin", InterventionType::SmallMolecule, 0.70),
                Intervention::new("Caloric Restriction", InterventionType::Lifestyle, 0.90),
                Intervention::new("Intermittent Fasting", InterventionType::Lifestyle, 0.75),
            ],
            Hallmark::CellularSenescence => vec![
                Intervention::new("Dasatinib + Quercetin", InterventionType::Senolytic, 0.80),
                Intervention::new("Fisetin", InterventionType::Senolytic, 0.65),
                Intervention::new("Navitoclax", InterventionType::Senolytic, 0.75),
                Intervention::new("FOXO4-DRI", InterventionType::Peptide, 0.70),
            ],
            Hallmark::MitochondrialDysfunction => vec![
                Intervention::new("NAD+ Precursors (NMN/NR)", InterventionType::Supplement, 0.70),
                Intervention::new("Urolithin A", InterventionType::SmallMolecule, 0.65),
                Intervention::new("MitoQ", InterventionType::SmallMolecule, 0.60),
                Intervention::new("Exercise", InterventionType::Lifestyle, 0.85),
            ],
            Hallmark::EpigeneticAlterations => vec![
                Intervention::new("Yamanaka Factors (partial)", InterventionType::GeneTherapy, 0.90),
                Intervention::new("α-Ketoglutarate", InterventionType::Supplement, 0.55),
                Intervention::new("HDAC Inhibitors", InterventionType::SmallMolecule, 0.50),
            ],
            Hallmark::TelomereAttrition => vec![
                Intervention::new("Telomerase Gene Therapy", InterventionType::GeneTherapy, 0.75),
                Intervention::new("TA-65", InterventionType::SmallMolecule, 0.40),
            ],
            Hallmark::LossOfProteostasis => vec![
                Intervention::new("Spermidine", InterventionType::Supplement, 0.60),
                Intervention::new("HSP90 Modulators", InterventionType::SmallMolecule, 0.50),
            ],
            Hallmark::GenomicInstability => vec![
                Intervention::new("PARP Inhibitors", InterventionType::SmallMolecule, 0.45),
                Intervention::new("NAD+ (DNA repair)", InterventionType::Supplement, 0.55),
            ],
            Hallmark::StemCellExhaustion => vec![
                Intervention::new("Stem Cell Transplant", InterventionType::CellTherapy, 0.65),
                Intervention::new("GDF11", InterventionType::Protein, 0.50),
            ],
            Hallmark::AlteredIntercellularCommunication => vec![
                Intervention::new("Young Blood Factors", InterventionType::Protein, 0.55),
                Intervention::new("Anti-inflammatory Diet", InterventionType::Lifestyle, 0.60),
            ],
            Hallmark::ChronicInflammation => vec![
                Intervention::new("Omega-3 Fatty Acids", InterventionType::Supplement, 0.55),
                Intervention::new("Curcumin", InterventionType::Supplement, 0.50),
            ],
            Hallmark::Dysbiosis => vec![
                Intervention::new("Prebiotics/Probiotics", InterventionType::Supplement, 0.55),
                Intervention::new("Fecal Microbiome Transplant", InterventionType::CellTherapy, 0.50),
            ],
            Hallmark::DisabledMacroautophagy => vec![
                Intervention::new("Spermidine", InterventionType::Supplement, 0.65),
                Intervention::new("Trehalose", InterventionType::SmallMolecule, 0.50),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HallmarkCategory {
    Primary,      // Causes of cellular damage
    Antagonistic, // Initially beneficial responses that become harmful
    Integrative,  // End results affecting tissue homeostasis
    Extended,     // Recently added hallmarks
}

/// An intervention targeting aging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub id: Uuid,
    pub name: String,
    pub intervention_type: InterventionType,
    /// Evidence strength (0-1)
    pub evidence_score: f64,
    /// Targeted hallmarks
    pub targets: Vec<Hallmark>,
    /// Known molecular targets
    pub molecular_targets: Vec<String>,
    /// Clinical trial status
    pub clinical_status: ClinicalStatus,
    /// Estimated effect on lifespan (as fraction, e.g., 0.15 = 15% extension)
    pub lifespan_effect: Option<f64>,
    /// Estimated effect on healthspan
    pub healthspan_effect: Option<f64>,
    /// Safety concerns
    pub safety_notes: Vec<String>,
}

impl Intervention {
    pub fn new(name: &str, intervention_type: InterventionType, evidence: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            intervention_type,
            evidence_score: evidence,
            targets: Vec::new(),
            molecular_targets: Vec::new(),
            clinical_status: ClinicalStatus::Preclinical,
            lifespan_effect: None,
            healthspan_effect: None,
            safety_notes: Vec::new(),
        }
    }

    pub fn with_targets(mut self, targets: Vec<Hallmark>) -> Self {
        self.targets = targets;
        self
    }

    pub fn with_lifespan_effect(mut self, effect: f64) -> Self {
        self.lifespan_effect = Some(effect);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionType {
    SmallMolecule,
    Peptide,
    Protein,
    GeneTherapy,
    CellTherapy,
    Senolytic,
    Supplement,
    Lifestyle,
    Device,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClinicalStatus {
    Preclinical,
    Phase1,
    Phase2,
    Phase3,
    Approved,
    PostMarket,
}

/// A causal relationship between hallmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallmarkEdge {
    /// Source hallmark
    pub from: Hallmark,
    /// Target hallmark
    pub to: Hallmark,
    /// Causal strength (0-1)
    pub strength: f64,
    /// Direction of causation
    pub causation_type: CausationType,
    /// Evidence citations
    pub evidence: Vec<String>,
    /// Potential intervention point
    pub intervention_point: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausationType {
    /// A causes B
    Direct,
    /// A accelerates B
    Amplifying,
    /// A inhibits B
    Inhibiting,
    /// A and B mutually reinforce
    Bidirectional,
    /// Relationship through intermediate
    Indirect,
}

/// The hallmarks knowledge graph
pub struct HallmarksGraph {
    /// Directed graph of hallmark relationships
    graph: DiGraph<Hallmark, HallmarkEdge>,
    /// Map from hallmark to node index
    node_indices: HashMap<Hallmark, NodeIndex>,
    /// Known interventions
    interventions: Vec<Intervention>,
    /// Discovered novel connections (from dream solver)
    novel_connections: Vec<NovelHallmarkConnection>,
}

/// A novel connection discovered through AI exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelHallmarkConnection {
    pub id: Uuid,
    pub from: Hallmark,
    pub to: Hallmark,
    pub hypothesis: String,
    pub confidence: f64,
    pub discovered_via: String,
    pub potential_intervention: Option<String>,
    pub requires_validation: bool,
}

impl HallmarksGraph {
    /// Create a new hallmarks graph with known relationships
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        // Add all hallmarks as nodes
        for hallmark in Hallmark::all() {
            let idx = graph.add_node(hallmark);
            node_indices.insert(hallmark, idx);
        }

        let mut hg = Self {
            graph,
            node_indices,
            interventions: Vec::new(),
            novel_connections: Vec::new(),
        };

        // Add known causal relationships
        hg.add_known_edges();
        hg.add_known_interventions();

        hg
    }

    /// Add established causal relationships
    fn add_known_edges(&mut self) {
        // Primary → Antagonistic
        self.add_edge(Hallmark::GenomicInstability, Hallmark::CellularSenescence, 0.9, CausationType::Direct);
        self.add_edge(Hallmark::TelomereAttrition, Hallmark::CellularSenescence, 0.95, CausationType::Direct);
        self.add_edge(Hallmark::EpigeneticAlterations, Hallmark::MitochondrialDysfunction, 0.7, CausationType::Amplifying);
        self.add_edge(Hallmark::LossOfProteostasis, Hallmark::MitochondrialDysfunction, 0.75, CausationType::Direct);

        // Antagonistic cross-talk
        self.add_edge(Hallmark::MitochondrialDysfunction, Hallmark::GenomicInstability, 0.6, CausationType::Amplifying);
        self.add_edge(Hallmark::CellularSenescence, Hallmark::ChronicInflammation, 0.85, CausationType::Direct);
        self.add_edge(Hallmark::DeregulatedNutrientSensing, Hallmark::DisabledMacroautophagy, 0.8, CausationType::Direct);

        // Antagonistic → Integrative
        self.add_edge(Hallmark::CellularSenescence, Hallmark::StemCellExhaustion, 0.8, CausationType::Direct);
        self.add_edge(Hallmark::MitochondrialDysfunction, Hallmark::StemCellExhaustion, 0.7, CausationType::Amplifying);
        self.add_edge(Hallmark::ChronicInflammation, Hallmark::AlteredIntercellularCommunication, 0.85, CausationType::Direct);

        // Extended hallmark connections
        self.add_edge(Hallmark::Dysbiosis, Hallmark::ChronicInflammation, 0.7, CausationType::Amplifying);
        self.add_edge(Hallmark::DisabledMacroautophagy, Hallmark::LossOfProteostasis, 0.8, CausationType::Bidirectional);

        // Feedback loops (crucial for understanding aging dynamics)
        self.add_edge(Hallmark::ChronicInflammation, Hallmark::GenomicInstability, 0.5, CausationType::Amplifying);
        self.add_edge(Hallmark::AlteredIntercellularCommunication, Hallmark::CellularSenescence, 0.6, CausationType::Amplifying);
    }

    /// Add known interventions
    fn add_known_interventions(&mut self) {
        for hallmark in Hallmark::all() {
            for mut intervention in hallmark.known_interventions() {
                intervention.targets = vec![hallmark];
                self.interventions.push(intervention);
            }
        }
    }

    /// Add an edge between hallmarks
    fn add_edge(&mut self, from: Hallmark, to: Hallmark, strength: f64, causation: CausationType) {
        let from_idx = self.node_indices[&from];
        let to_idx = self.node_indices[&to];

        let edge = HallmarkEdge {
            from,
            to,
            strength,
            causation_type: causation,
            evidence: Vec::new(),
            intervention_point: true,
        };

        self.graph.add_edge(from_idx, to_idx, edge);
    }

    /// Find all paths from one hallmark to another
    pub fn find_paths(&self, from: Hallmark, to: Hallmark, max_length: usize) -> Vec<Vec<Hallmark>> {
        let from_idx = self.node_indices[&from];
        let to_idx = self.node_indices[&to];

        let paths: Vec<Vec<NodeIndex>> = all_simple_paths(&self.graph, from_idx, to_idx, 1, Some(max_length))
            .collect();

        paths.into_iter()
            .map(|path| path.into_iter().map(|idx| self.graph[idx]).collect())
            .collect()
    }

    /// Find the shortest intervention path to reduce a hallmark
    pub fn shortest_intervention_path(&self, target: Hallmark) -> Option<Vec<Hallmark>> {
        let target_idx = self.node_indices[&target];

        // Find paths from primary hallmarks (root causes)
        let primary = vec![
            Hallmark::GenomicInstability,
            Hallmark::TelomereAttrition,
            Hallmark::EpigeneticAlterations,
            Hallmark::LossOfProteostasis,
        ];

        let mut shortest: Option<Vec<Hallmark>> = None;

        for source in primary {
            let source_idx = self.node_indices[&source];
            let distances = dijkstra(&self.graph, source_idx, Some(target_idx), |e| {
                // Weight = inverse of causal strength (stronger = shorter path)
                1.0 / e.weight().strength
            });

            if let Some(&dist) = distances.get(&target_idx) {
                // Reconstruct path (simplified - would need proper backtracking)
                let paths = self.find_paths(source, target, 5);
                for path in paths {
                    if shortest.is_none() || path.len() < shortest.as_ref().unwrap().len() {
                        shortest = Some(path);
                    }
                }
            }
        }

        shortest
    }

    /// Get interventions that target a specific hallmark
    pub fn interventions_for(&self, hallmark: Hallmark) -> Vec<&Intervention> {
        self.interventions.iter()
            .filter(|i| i.targets.contains(&hallmark))
            .collect()
    }

    /// Get all interventions sorted by evidence score
    pub fn top_interventions(&self, n: usize) -> Vec<&Intervention> {
        let mut sorted: Vec<_> = self.interventions.iter().collect();
        sorted.sort_by(|a, b| b.evidence_score.partial_cmp(&a.evidence_score).unwrap());
        sorted.into_iter().take(n).collect()
    }

    /// Find upstream causes of a hallmark
    pub fn upstream_causes(&self, hallmark: Hallmark) -> Vec<(Hallmark, f64)> {
        let idx = self.node_indices[&hallmark];
        self.graph.edges_directed(idx, Direction::Incoming)
            .map(|e| (self.graph[e.source()], e.weight().strength))
            .collect()
    }

    /// Find downstream effects of a hallmark
    pub fn downstream_effects(&self, hallmark: Hallmark) -> Vec<(Hallmark, f64)> {
        let idx = self.node_indices[&hallmark];
        self.graph.edges_directed(idx, Direction::Outgoing)
            .map(|e| (self.graph[e.target()], e.weight().strength))
            .collect()
    }

    /// Calculate centrality score for each hallmark
    /// (How important is this hallmark in the aging network?)
    pub fn hallmark_centrality(&self) -> HashMap<Hallmark, f64> {
        let mut centrality = HashMap::new();

        for hallmark in Hallmark::all() {
            let idx = self.node_indices[&hallmark];
            let incoming = self.graph.edges_directed(idx, Direction::Incoming).count();
            let outgoing = self.graph.edges_directed(idx, Direction::Outgoing).count();

            // Simple degree centrality
            centrality.insert(hallmark, (incoming + outgoing) as f64);
        }

        // Normalize
        let max = centrality.values().cloned().fold(0.0f64, f64::max);
        if max > 0.0 {
            for v in centrality.values_mut() {
                *v /= max;
            }
        }

        centrality
    }

    /// Add a novel connection discovered by the dream solver
    pub fn add_novel_connection(&mut self, connection: NovelHallmarkConnection) {
        if connection.confidence > 0.5 {
            // Add to graph if confidence is reasonable
            self.add_edge(
                connection.from,
                connection.to,
                connection.confidence,
                CausationType::Indirect,
            );
        }
        self.novel_connections.push(connection);
    }

    /// Get all novel connections requiring validation
    pub fn connections_needing_validation(&self) -> Vec<&NovelHallmarkConnection> {
        self.novel_connections.iter()
            .filter(|c| c.requires_validation)
            .collect()
    }

    /// Find the most impactful intervention points
    /// (Hallmarks that, if addressed, would have cascading benefits)
    pub fn find_leverage_points(&self) -> Vec<(Hallmark, f64)> {
        let centrality = self.hallmark_centrality();
        let mut leverage: Vec<_> = centrality.into_iter().collect();

        // Weight by downstream impact
        leverage.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        leverage
    }
}

impl Default for HallmarksGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hallmarks_graph_creation() {
        let graph = HallmarksGraph::new();
        assert!(graph.interventions.len() > 0);
    }

    #[test]
    fn test_find_paths() {
        let graph = HallmarksGraph::new();
        let paths = graph.find_paths(Hallmark::TelomereAttrition, Hallmark::StemCellExhaustion, 3);
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_interventions_for_senescence() {
        let graph = HallmarksGraph::new();
        let interventions = graph.interventions_for(Hallmark::CellularSenescence);
        assert!(!interventions.is_empty());
        assert!(interventions.iter().any(|i| i.name.contains("Quercetin")));
    }

    #[test]
    fn test_leverage_points() {
        let graph = HallmarksGraph::new();
        let leverage = graph.find_leverage_points();
        assert!(!leverage.is_empty());
    }
}
