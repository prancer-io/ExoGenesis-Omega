//! Coordinate System for Mindscape
//!
//! Maps high-dimensional memory embeddings to 3D navigable coordinates
//! using dimensionality reduction techniques.

use rand::Rng;
use serde::{Deserialize, Serialize};

/// A 3D position in mindscape
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Euclidean distance to another position
    pub fn distance_to(&self, other: &Position3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Linear interpolation toward another position
    pub fn lerp(&self, other: &Position3D, t: f64) -> Position3D {
        Position3D {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    /// Direction vector toward another position (normalized)
    pub fn direction_to(&self, other: &Position3D) -> Position3D {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        let mag = (dx * dx + dy * dy + dz * dz).sqrt();

        if mag < 1e-10 {
            return Position3D::origin();
        }

        Position3D {
            x: dx / mag,
            y: dy / mag,
            z: dz / mag,
        }
    }

    /// Add direction with magnitude
    pub fn move_by(&self, direction: &Position3D, distance: f64) -> Position3D {
        Position3D {
            x: self.x + direction.x * distance,
            y: self.y + direction.y * distance,
            z: self.z + direction.z * distance,
        }
    }
}

impl Default for Position3D {
    fn default() -> Self {
        Self::origin()
    }
}

/// A coordinate in mindscape with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindscapeCoordinate {
    /// 3D position
    pub position: Position3D,
    /// Original embedding (for similarity calculations)
    pub embedding_hash: u64,
    /// Semantic region label
    pub region: Option<String>,
    /// Consciousness intensity at this location
    pub phi_intensity: f64,
    /// Creation timestamp
    pub created_at: u64,
}

impl MindscapeCoordinate {
    pub fn new(position: Position3D, embedding: &[f64]) -> Self {
        Self {
            position,
            embedding_hash: Self::hash_embedding(embedding),
            region: None,
            phi_intensity: 0.0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    fn hash_embedding(embedding: &[f64]) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        for (i, &v) in embedding.iter().enumerate().take(32) {
            let bits = (v * 1000000.0) as i64;
            (i, bits).hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Distance to another coordinate
    pub fn distance_to(&self, other: &MindscapeCoordinate) -> f64 {
        self.position.distance_to(&other.position)
    }
}

/// Dimensional projection method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionalProjection {
    /// Random projection (fast, approximate)
    Random,
    /// PCA-like projection (principal components)
    PCA,
    /// Semantic clustering (group by meaning)
    Semantic,
    /// Temporal ordering (by creation time)
    Temporal,
}

/// Maps embeddings to mindscape coordinates
pub struct CoordinateMapper {
    /// Dimension of input embeddings
    embedding_dim: usize,
    /// Size of the world
    world_size: f64,
    /// Projection method
    projection: DimensionalProjection,
    /// Random projection matrix (3 x embedding_dim)
    projection_matrix: Vec<Vec<f64>>,
    /// Center of the world
    center: Position3D,
}

impl CoordinateMapper {
    pub fn new(embedding_dim: usize, world_size: f64) -> Self {
        let projection_matrix = Self::generate_projection_matrix(embedding_dim);

        Self {
            embedding_dim,
            world_size,
            projection: DimensionalProjection::Random,
            projection_matrix,
            center: Position3D::new(world_size / 2.0, world_size / 2.0, world_size / 2.0),
        }
    }

    /// Generate random projection matrix for embedding_dim -> 3D
    fn generate_projection_matrix(dim: usize) -> Vec<Vec<f64>> {
        let mut rng = rand::thread_rng();
        let scale = 1.0 / (dim as f64).sqrt();

        (0..3)
            .map(|_| {
                (0..dim)
                    .map(|_| rng.gen_range(-1.0..1.0) * scale)
                    .collect()
            })
            .collect()
    }

    /// Map an embedding to a 3D coordinate
    pub fn map_to_coordinate(&self, embedding: &[f64]) -> MindscapeCoordinate {
        let position = self.project(embedding);
        MindscapeCoordinate::new(position, embedding)
    }

    /// Project embedding to 3D using the projection matrix
    fn project(&self, embedding: &[f64]) -> Position3D {
        let dim = embedding.len().min(self.embedding_dim);

        // Compute 3 coordinates using dot product with projection vectors
        let mut coords = [0.0, 0.0, 0.0];

        for (i, proj_vec) in self.projection_matrix.iter().enumerate() {
            let mut sum = 0.0;
            for (j, &e) in embedding.iter().enumerate().take(dim) {
                sum += e * proj_vec.get(j).copied().unwrap_or(0.0);
            }
            coords[i] = sum;
        }

        // Normalize to world bounds with sigmoid-like scaling
        let scale = self.world_size / 2.0;
        let x = self.center.x + Self::sigmoid(coords[0]) * scale;
        let y = self.center.y + Self::sigmoid(coords[1]) * scale;
        let z = self.center.z + Self::sigmoid(coords[2]) * scale;

        Position3D::new(
            x.clamp(0.0, self.world_size),
            y.clamp(0.0, self.world_size),
            z.clamp(0.0, self.world_size),
        )
    }

    /// Sigmoid function for smooth coordinate mapping
    fn sigmoid(x: f64) -> f64 {
        2.0 / (1.0 + (-x).exp()) - 1.0
    }

    /// Compute semantic distance between two coordinates
    /// (Uses original embedding similarity, not just spatial distance)
    pub fn semantic_distance(&self, coord1: &MindscapeCoordinate, coord2: &MindscapeCoordinate) -> f64 {
        // Use hash comparison as proxy (full embeddings would be better)
        if coord1.embedding_hash == coord2.embedding_hash {
            0.0
        } else {
            coord1.position.distance_to(&coord2.position)
        }
    }

    /// Set projection method
    pub fn set_projection(&mut self, projection: DimensionalProjection) {
        self.projection = projection;
    }

    /// Get world size
    pub fn world_size(&self) -> f64 {
        self.world_size
    }

    /// Get center position
    pub fn center(&self) -> Position3D {
        self.center
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_distance() {
        let p1 = Position3D::new(0.0, 0.0, 0.0);
        let p2 = Position3D::new(3.0, 4.0, 0.0);

        assert!((p1.distance_to(&p2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_position_lerp() {
        let p1 = Position3D::new(0.0, 0.0, 0.0);
        let p2 = Position3D::new(10.0, 10.0, 10.0);

        let mid = p1.lerp(&p2, 0.5);
        assert!((mid.x - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_coordinate_mapping() {
        let mapper = CoordinateMapper::new(256, 1000.0);
        let embedding = vec![0.5; 256];

        let coord = mapper.map_to_coordinate(&embedding);

        assert!(coord.position.x >= 0.0 && coord.position.x <= 1000.0);
        assert!(coord.position.y >= 0.0 && coord.position.y <= 1000.0);
        assert!(coord.position.z >= 0.0 && coord.position.z <= 1000.0);
    }

    #[test]
    fn test_different_embeddings_different_coords() {
        let mapper = CoordinateMapper::new(256, 1000.0);

        let emb1 = vec![0.1; 256];
        let emb2 = vec![0.9; 256];

        let coord1 = mapper.map_to_coordinate(&emb1);
        let coord2 = mapper.map_to_coordinate(&emb2);

        // Should be in different locations
        assert!(coord1.distance_to(&coord2) > 1.0);
    }
}
