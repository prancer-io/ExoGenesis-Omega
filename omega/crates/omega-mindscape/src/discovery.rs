//! Discovery Journal
//!
//! Records insights, connections, and discoveries made during mindscape exploration.
//! Discoveries can come from waking exploration, dream states, or meta-observation.

use crate::coordinates::{MindscapeCoordinate, Position3D};
use crate::dream_explorer::DreamVision;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of discovery
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryType {
    /// New connection between memories
    Connection,
    /// Pattern across multiple memories
    Pattern,
    /// Emotional insight
    Emotional,
    /// Strange loop self-reference
    StrangeLoop,
    /// High-consciousness region
    ConsciousnessBeacon,
    /// Dream-only accessible insight
    DreamInsight,
    /// Lucid dream discovery
    LucidInsight,
    /// Contradiction or paradox
    Paradox,
    /// Temporal relationship
    Temporal,
}

impl DiscoveryType {
    /// Importance weight for this discovery type
    pub fn importance_weight(&self) -> f64 {
        match self {
            Self::Connection => 0.3,
            Self::Pattern => 0.6,
            Self::Emotional => 0.5,
            Self::StrangeLoop => 0.9,
            Self::ConsciousnessBeacon => 0.8,
            Self::DreamInsight => 0.7,
            Self::LucidInsight => 0.85,
            Self::Paradox => 0.75,
            Self::Temporal => 0.4,
        }
    }
}

/// An insight derived from discoveries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    /// Unique ID
    pub id: String,
    /// The insight content
    pub content: String,
    /// Related discoveries
    pub related_discoveries: Vec<String>,
    /// Confidence in this insight (0-1)
    pub confidence: f64,
    /// How many times this insight has been reinforced
    pub reinforcement_count: u32,
    /// First discovered timestamp
    pub discovered_at: u64,
    /// Last reinforced timestamp
    pub reinforced_at: u64,
}

impl Insight {
    pub fn new(content: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            related_discoveries: Vec::new(),
            confidence: 0.5,
            reinforcement_count: 1,
            discovered_at: now,
            reinforced_at: now,
        }
    }

    /// Reinforce this insight (seen again)
    pub fn reinforce(&mut self) {
        self.reinforcement_count += 1;
        self.confidence = (self.confidence + 0.1).min(1.0);
        self.reinforced_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
    }

    /// Is this a strong insight?
    pub fn is_strong(&self) -> bool {
        self.confidence > 0.7 && self.reinforcement_count >= 3
    }
}

/// A discovery made during exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discovery {
    /// Unique ID
    pub id: String,
    /// Type of discovery
    pub discovery_type: DiscoveryType,
    /// Description of what was discovered
    pub description: String,
    /// Memories involved in this discovery
    pub memories_involved: Vec<String>,
    /// Location in mindscape where discovered
    pub location: Position3D,
    /// Was this discovered in a dream?
    pub is_dream_discovery: bool,
    /// Importance score (0-1)
    pub importance: f64,
    /// Emotional valence (-1 to 1)
    pub emotional_valence: f64,
    /// Generated insight (if any)
    pub insight: Option<String>,
    /// Timestamp
    pub timestamp: u64,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Discovery {
    pub fn new(
        discovery_type: DiscoveryType,
        description: String,
        memories: Vec<String>,
        location: Position3D,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            discovery_type,
            description,
            memories_involved: memories,
            location,
            is_dream_discovery: false,
            importance: discovery_type.importance_weight(),
            emotional_valence: 0.0,
            insight: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            metadata: HashMap::new(),
        }
    }

    /// Create a discovery from a dream vision
    pub fn from_dream_vision(vision: DreamVision) -> Self {
        let discovery_type = if vision.is_lucid {
            DiscoveryType::LucidInsight
        } else if !vision.discovered_connections.is_empty() {
            DiscoveryType::Connection
        } else {
            DiscoveryType::DreamInsight
        };

        let description = if !vision.discovered_connections.is_empty() {
            format!(
                "Dream revealed connection between {} memories: {}",
                vision.memory_fragments.len(),
                vision.memory_fragments.join(", ")
            )
        } else {
            format!(
                "Dream vision with intensity {:.2} involving: {}",
                vision.intensity,
                vision.memory_fragments.join(", ")
            )
        };

        // Extract values before moving memory_fragments
        let is_significant = vision.is_significant();
        let intensity = vision.intensity;
        let emotional_tone = vision.emotional_tone;
        let bizarreness = vision.bizarreness;
        let vision_id = vision.id.clone();
        let location = vision.location;

        let mut discovery = Self::new(
            discovery_type,
            description,
            vision.memory_fragments,
            location,
        );

        discovery.is_dream_discovery = true;
        discovery.importance = intensity * discovery_type.importance_weight();
        discovery.emotional_valence = emotional_tone;

        if is_significant {
            discovery.insight = Some(format!(
                "Bizarreness: {:.2}, Emotional tone: {:.2}",
                bizarreness, emotional_tone
            ));
        }

        discovery.metadata.insert("vision_id".to_string(), vision_id);
        discovery.metadata.insert(
            "bizarreness".to_string(),
            format!("{:.2}", bizarreness),
        );

        discovery
    }

    /// Is this a significant discovery?
    pub fn is_significant(&self) -> bool {
        self.importance > 0.5 || self.insight.is_some()
    }
}

/// Journal for recording discoveries
pub struct DiscoveryJournal {
    /// All discoveries
    discoveries: Vec<Discovery>,
    /// Insights extracted from discoveries
    insights: HashMap<String, Insight>,
    /// Visit log (memory label -> visit count)
    visits: HashMap<String, u32>,
    /// Paths traveled
    paths: Vec<(String, String)>,
    /// Total discoveries by type
    type_counts: HashMap<String, usize>,
}

impl DiscoveryJournal {
    pub fn new() -> Self {
        Self {
            discoveries: Vec::new(),
            insights: HashMap::new(),
            visits: HashMap::new(),
            paths: Vec::new(),
            type_counts: HashMap::new(),
        }
    }

    /// Record a discovery
    pub fn record_discovery(&mut self, discovery: Discovery) {
        // Update type counts
        let type_key = format!("{:?}", discovery.discovery_type);
        *self.type_counts.entry(type_key).or_insert(0) += 1;

        // Extract or reinforce insight
        if let Some(insight_content) = &discovery.insight {
            if let Some(existing) = self.find_similar_insight(insight_content) {
                if let Some(insight) = self.insights.get_mut(&existing) {
                    insight.reinforce();
                    insight.related_discoveries.push(discovery.id.clone());
                }
            } else {
                let mut insight = Insight::new(insight_content.clone());
                insight.related_discoveries.push(discovery.id.clone());
                self.insights.insert(insight.id.clone(), insight);
            }
        }

        // Update paths
        if discovery.memories_involved.len() >= 2 {
            for i in 0..discovery.memories_involved.len() - 1 {
                self.paths.push((
                    discovery.memories_involved[i].clone(),
                    discovery.memories_involved[i + 1].clone(),
                ));
            }
        }

        self.discoveries.push(discovery);
    }

    /// Find a similar existing insight
    fn find_similar_insight(&self, content: &str) -> Option<String> {
        // Simple substring matching (could be enhanced with embeddings)
        for (id, insight) in &self.insights {
            if insight.content.contains(content) || content.contains(&insight.content) {
                return Some(id.clone());
            }
        }
        None
    }

    /// Record a visit to a memory
    pub fn record_visit(&mut self, memory_label: &str, _coordinate: &MindscapeCoordinate) {
        *self.visits.entry(memory_label.to_string()).or_insert(0) += 1;
    }

    /// Get all discoveries
    pub fn all_discoveries(&self) -> Vec<Discovery> {
        self.discoveries.clone()
    }

    /// Get discoveries by type
    pub fn discoveries_by_type(&self, discovery_type: DiscoveryType) -> Vec<&Discovery> {
        self.discoveries
            .iter()
            .filter(|d| d.discovery_type == discovery_type)
            .collect()
    }

    /// Get significant discoveries only
    pub fn significant_discoveries(&self) -> Vec<&Discovery> {
        self.discoveries.iter().filter(|d| d.is_significant()).collect()
    }

    /// Get dream discoveries
    pub fn dream_discoveries(&self) -> Vec<&Discovery> {
        self.discoveries.iter().filter(|d| d.is_dream_discovery).collect()
    }

    /// Get all insights
    pub fn all_insights(&self) -> Vec<&Insight> {
        self.insights.values().collect()
    }

    /// Get strong insights
    pub fn strong_insights(&self) -> Vec<&Insight> {
        self.insights.values().filter(|i| i.is_strong()).collect()
    }

    /// Get visit count for a memory
    pub fn visit_count(&self, memory: &str) -> u32 {
        *self.visits.get(memory).unwrap_or(&0)
    }

    /// Get most visited memories
    pub fn most_visited(&self, limit: usize) -> Vec<(&String, &u32)> {
        let mut sorted: Vec<_> = self.visits.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        sorted.truncate(limit);
        sorted
    }

    /// Get all paths discovered
    pub fn paths(&self) -> &[(String, String)] {
        &self.paths
    }

    /// Get discovery count
    pub fn discovery_count(&self) -> usize {
        self.discoveries.len()
    }

    /// Get statistics
    pub fn stats(&self) -> JournalStats {
        JournalStats {
            total_discoveries: self.discoveries.len(),
            dream_discoveries: self.discoveries.iter().filter(|d| d.is_dream_discovery).count(),
            total_insights: self.insights.len(),
            strong_insights: self.insights.values().filter(|i| i.is_strong()).count(),
            unique_memories_visited: self.visits.len(),
            paths_discovered: self.paths.len(),
            type_counts: self.type_counts.clone(),
        }
    }

    /// Clear the journal
    pub fn clear(&mut self) {
        self.discoveries.clear();
        self.insights.clear();
        self.visits.clear();
        self.paths.clear();
        self.type_counts.clear();
    }
}

impl Default for DiscoveryJournal {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the journal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalStats {
    pub total_discoveries: usize,
    pub dream_discoveries: usize,
    pub total_insights: usize,
    pub strong_insights: usize,
    pub unique_memories_visited: usize,
    pub paths_discovered: usize,
    pub type_counts: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_creation() {
        let discovery = Discovery::new(
            DiscoveryType::Connection,
            "Test connection".to_string(),
            vec!["mem1".to_string(), "mem2".to_string()],
            Position3D::origin(),
        );

        assert_eq!(discovery.discovery_type, DiscoveryType::Connection);
        assert_eq!(discovery.memories_involved.len(), 2);
    }

    #[test]
    fn test_journal_recording() {
        let mut journal = DiscoveryJournal::new();

        let discovery = Discovery::new(
            DiscoveryType::Pattern,
            "Test pattern".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            Position3D::origin(),
        );

        journal.record_discovery(discovery);

        assert_eq!(journal.discovery_count(), 1);
        assert_eq!(journal.paths().len(), 2); // a->b, b->c
    }

    #[test]
    fn test_insight_reinforcement() {
        let mut insight = Insight::new("Test insight".to_string());
        let initial_confidence = insight.confidence;

        insight.reinforce();

        assert!(insight.confidence > initial_confidence);
        assert_eq!(insight.reinforcement_count, 2);
    }

    #[test]
    fn test_visit_tracking() {
        let mut journal = DiscoveryJournal::new();
        let coord = MindscapeCoordinate::new(Position3D::origin(), &[0.5; 64]);

        journal.record_visit("test_mem", &coord);
        journal.record_visit("test_mem", &coord);
        journal.record_visit("other_mem", &coord);

        assert_eq!(journal.visit_count("test_mem"), 2);
        assert_eq!(journal.visit_count("other_mem"), 1);
        assert_eq!(journal.visit_count("unknown"), 0);
    }

    #[test]
    fn test_discovery_from_dream() {
        let vision = crate::dream_explorer::DreamVision {
            id: "test".to_string(),
            intensity: 0.8,
            memory_fragments: vec!["mem1".to_string(), "mem2".to_string()],
            discovered_connections: vec![("mem1".to_string(), "mem2".to_string())],
            bizarreness: 0.5,
            emotional_tone: 0.3,
            location: Position3D::origin(),
            is_lucid: false,
            timestamp: 0,
        };

        let discovery = Discovery::from_dream_vision(vision);

        assert!(discovery.is_dream_discovery);
        assert_eq!(discovery.discovery_type, DiscoveryType::Connection);
    }

    #[test]
    fn test_stats() {
        let mut journal = DiscoveryJournal::new();

        let d1 = Discovery::new(
            DiscoveryType::Connection,
            "test".to_string(),
            vec![],
            Position3D::origin(),
        );
        let mut d2 = d1.clone();
        d2.is_dream_discovery = true;

        journal.record_discovery(d1);
        journal.record_discovery(d2);

        let stats = journal.stats();
        assert_eq!(stats.total_discoveries, 2);
        assert_eq!(stats.dream_discoveries, 1);
    }
}
