//! Memory Landmarks
//!
//! Landmarks are significant points in mindscape representing memories,
//! concepts, or clusters of related information.

use crate::coordinates::{MindscapeCoordinate, Position3D};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of landmark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandmarkType {
    /// Single memory
    Memory,
    /// Cluster of related memories
    Cluster,
    /// High-consciousness region (high Phi)
    ConsciousnessBeacon,
    /// Strange loop detection point
    StrangeLoopNexus,
    /// Dream-discovered location
    DreamDiscovery,
    /// Emotional hotspot
    EmotionalCore,
    /// Knowledge crystallization point
    InsightCrystal,
}

impl LandmarkType {
    /// Get the "glow" intensity for visualization
    pub fn glow_intensity(&self) -> f64 {
        match self {
            Self::Memory => 0.3,
            Self::Cluster => 0.5,
            Self::ConsciousnessBeacon => 1.0,
            Self::StrangeLoopNexus => 0.9,
            Self::DreamDiscovery => 0.7,
            Self::EmotionalCore => 0.8,
            Self::InsightCrystal => 0.95,
        }
    }

    /// Can this landmark be discovered through dreams?
    pub fn dream_visible(&self) -> bool {
        matches!(self, Self::DreamDiscovery | Self::EmotionalCore | Self::InsightCrystal)
    }
}

/// A memory landmark in the mindscape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLandmark {
    /// Unique identifier
    pub id: String,
    /// Human-readable label
    pub label: String,
    /// Location in mindscape
    pub coordinate: MindscapeCoordinate,
    /// Original embedding
    pub embedding: Vec<f64>,
    /// Type of landmark
    pub landmark_type: LandmarkType,
    /// Visit count
    pub visits: u32,
    /// Last visited timestamp
    pub last_visited: Option<u64>,
    /// Connections to other landmarks
    pub connections: Vec<String>,
    /// Strength of this memory (0-1)
    pub strength: f64,
    /// Emotional valence (-1 to 1)
    pub emotional_valence: f64,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl MemoryLandmark {
    pub fn new(
        label: String,
        coordinate: MindscapeCoordinate,
        embedding: Vec<f64>,
        landmark_type: LandmarkType,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            label,
            coordinate,
            embedding,
            landmark_type,
            visits: 0,
            last_visited: None,
            connections: Vec::new(),
            strength: 1.0,
            emotional_valence: 0.0,
            metadata: HashMap::new(),
        }
    }

    /// Record a visit to this landmark
    pub fn visit(&mut self) {
        self.visits += 1;
        self.last_visited = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        );
        // Visiting strengthens the memory
        self.strength = (self.strength * 1.1).min(1.0);
    }

    /// Connect to another landmark
    pub fn connect(&mut self, other_id: String) {
        if !self.connections.contains(&other_id) {
            self.connections.push(other_id);
        }
    }

    /// Decay strength over time
    pub fn decay(&mut self, factor: f64) {
        self.strength *= factor;
        self.strength = self.strength.max(0.01);
    }

    /// Is this a strong memory?
    pub fn is_strong(&self) -> bool {
        self.strength > 0.7
    }

    /// Get position
    pub fn position(&self) -> Position3D {
        self.coordinate.position
    }

    /// Similarity to embedding (cosine similarity)
    pub fn similarity_to(&self, other_embedding: &[f64]) -> f64 {
        let min_len = self.embedding.len().min(other_embedding.len());
        if min_len == 0 {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for i in 0..min_len {
            dot += self.embedding[i] * other_embedding[i];
            norm_a += self.embedding[i] * self.embedding[i];
            norm_b += other_embedding[i] * other_embedding[i];
        }

        let denom = (norm_a.sqrt() * norm_b.sqrt()).max(1e-10);
        dot / denom
    }
}

/// A cluster of related landmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandmarkCluster {
    /// Cluster ID
    pub id: String,
    /// Cluster name/theme
    pub name: String,
    /// Center position
    pub center: Position3D,
    /// Radius
    pub radius: f64,
    /// Member landmark IDs
    pub members: Vec<String>,
    /// Cluster type
    pub cluster_type: LandmarkType,
    /// Average embedding (centroid)
    pub centroid_embedding: Vec<f64>,
    /// Coherence score (how related are members)
    pub coherence: f64,
}

impl LandmarkCluster {
    pub fn new(name: String, center: Position3D, cluster_type: LandmarkType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            center,
            radius: 0.0,
            members: Vec::new(),
            cluster_type,
            centroid_embedding: Vec::new(),
            coherence: 1.0,
        }
    }

    /// Add a landmark to the cluster
    pub fn add_member(&mut self, landmark: &MemoryLandmark) {
        self.members.push(landmark.id.clone());

        // Update radius
        let dist = self.center.distance_to(&landmark.position());
        if dist > self.radius {
            self.radius = dist;
        }

        // Update centroid embedding
        if self.centroid_embedding.is_empty() {
            self.centroid_embedding = landmark.embedding.clone();
        } else {
            let n = self.members.len() as f64;
            for (i, val) in self.centroid_embedding.iter_mut().enumerate() {
                if i < landmark.embedding.len() {
                    *val = (*val * (n - 1.0) + landmark.embedding[i]) / n;
                }
            }
        }
    }

    /// Check if a position is within this cluster
    pub fn contains(&self, position: &Position3D) -> bool {
        self.center.distance_to(position) <= self.radius
    }

    /// Member count
    pub fn size(&self) -> usize {
        self.members.len()
    }
}

/// Builds clusters from landmarks
pub struct ClusterBuilder {
    similarity_threshold: f64,
}

impl ClusterBuilder {
    pub fn new(similarity_threshold: f64) -> Self {
        Self { similarity_threshold }
    }

    /// Build clusters from a collection of landmarks
    pub fn build_clusters(&self, landmarks: &[MemoryLandmark]) -> Vec<LandmarkCluster> {
        let mut clusters: Vec<LandmarkCluster> = Vec::new();
        let mut assigned: Vec<bool> = vec![false; landmarks.len()];

        for (i, landmark) in landmarks.iter().enumerate() {
            if assigned[i] {
                continue;
            }

            // Start a new cluster with this landmark
            let mut cluster = LandmarkCluster::new(
                format!("Cluster around {}", landmark.label),
                landmark.position(),
                LandmarkType::Cluster,
            );

            cluster.add_member(landmark);
            assigned[i] = true;

            // Find similar landmarks
            for (j, other) in landmarks.iter().enumerate() {
                if i == j || assigned[j] {
                    continue;
                }

                let similarity = landmark.similarity_to(&other.embedding);
                if similarity >= self.similarity_threshold {
                    cluster.add_member(other);
                    assigned[j] = true;
                }
            }

            // Only keep clusters with multiple members
            if cluster.size() > 1 {
                // Recalculate center based on all members
                cluster.center = self.compute_centroid(landmarks, &cluster.members);
                clusters.push(cluster);
            }
        }

        clusters
    }

    /// Compute centroid position from member landmarks
    fn compute_centroid(&self, landmarks: &[MemoryLandmark], member_ids: &[String]) -> Position3D {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut count = 0;

        for lm in landmarks {
            if member_ids.contains(&lm.id) {
                x += lm.position().x;
                y += lm.position().y;
                z += lm.position().z;
                count += 1;
            }
        }

        if count == 0 {
            return Position3D::origin();
        }

        Position3D::new(x / count as f64, y / count as f64, z / count as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinates::MindscapeCoordinate;

    #[test]
    fn test_landmark_creation() {
        let coord = MindscapeCoordinate::new(Position3D::new(10.0, 20.0, 30.0), &[0.5; 64]);
        let lm = MemoryLandmark::new(
            "test".to_string(),
            coord,
            vec![0.5; 64],
            LandmarkType::Memory,
        );

        assert_eq!(lm.label, "test");
        assert_eq!(lm.visits, 0);
        assert!(lm.strength > 0.0);
    }

    #[test]
    fn test_landmark_visit() {
        let coord = MindscapeCoordinate::new(Position3D::new(0.0, 0.0, 0.0), &[0.5; 64]);
        let mut lm = MemoryLandmark::new(
            "test".to_string(),
            coord,
            vec![0.5; 64],
            LandmarkType::Memory,
        );

        lm.strength = 0.5;
        lm.visit();

        assert_eq!(lm.visits, 1);
        assert!(lm.strength > 0.5);
    }

    #[test]
    fn test_cluster_building() {
        let builder = ClusterBuilder::new(0.9);

        let landmarks: Vec<MemoryLandmark> = (0..5)
            .map(|i| {
                let embedding = vec![0.5 + (i as f64 * 0.01); 64];
                let coord = MindscapeCoordinate::new(
                    Position3D::new(i as f64 * 10.0, 0.0, 0.0),
                    &embedding,
                );
                MemoryLandmark::new(
                    format!("lm_{}", i),
                    coord,
                    embedding,
                    LandmarkType::Memory,
                )
            })
            .collect();

        let clusters = builder.build_clusters(&landmarks);

        // High similarity threshold should create clusters from similar embeddings
        assert!(!clusters.is_empty() || landmarks.len() < 2);
    }

    #[test]
    fn test_similarity() {
        let coord = MindscapeCoordinate::new(Position3D::origin(), &[0.5; 64]);
        let lm = MemoryLandmark::new("test".to_string(), coord, vec![0.5; 64], LandmarkType::Memory);

        let same_sim = lm.similarity_to(&vec![0.5; 64]);
        let diff_sim = lm.similarity_to(&vec![-0.5; 64]);

        assert!(same_sim > diff_sim);
        assert!((same_sim - 1.0).abs() < 0.01);
    }
}
