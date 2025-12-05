//! HNSW (Hierarchical Navigable Small World) index implementation
//! Provides fast approximate nearest neighbor search for vector embeddings

use instant_distance::{Builder, HnswMap, Search};

/// HNSW configuration parameters
#[derive(Debug, Clone)]
pub struct HnswConfig {
    /// Number of neighbors to consider during construction (100-200 for quality)
    pub ef_construction: usize,
    /// Number of neighbors to consider during search (50-100 for speed/quality tradeoff)
    pub ef_search: usize,
    /// Maximum number of connections per node (16-32 typical)
    pub m: usize,
}

impl Default for HnswConfig {
    fn default() -> Self {
        Self {
            ef_construction: 200,
            ef_search: 100,
            m: 32,
        }
    }
}

/// Point wrapper for HNSW index
#[derive(Clone, Debug)]
pub struct VectorPoint {
    pub id: String,
    pub embedding: Vec<f32>,
    pub metadata: serde_json::Value,
}

impl instant_distance::Point for VectorPoint {
    fn distance(&self, other: &Self) -> f32 {
        // SIMD-optimized cosine distance (10-50x faster than scalar)
        // SimSIMD returns distance directly (0 = identical, 1 = orthogonal, 2 = opposite)
        use simsimd::SpatialSimilarity;

        match f32::cosine(&self.embedding, &other.embedding) {
            Some(distance) => distance as f32,
            None => 1.0, // Fallback for zero-length vectors
        }
    }
}

/// HNSW Index for fast vector search
pub struct HnswIndex {
    config: HnswConfig,
    hnsw: Option<HnswMap<VectorPoint, String>>,
    points: Vec<VectorPoint>,
    ids: Vec<String>,
    needs_rebuild: bool,
}

impl HnswIndex {
    /// Creates a new HNSW index with the given configuration
    pub fn new(config: HnswConfig) -> Self {
        Self {
            config,
            hnsw: None,
            points: Vec::new(),
            ids: Vec::new(),
            needs_rebuild: true,
        }
    }

    /// Inserts a single point into the index
    pub fn insert(&mut self, point: VectorPoint) {
        self.ids.push(point.id.clone());
        self.points.push(point);
        self.needs_rebuild = true;
    }

    /// Inserts multiple points in batch
    pub fn insert_batch(&mut self, points: Vec<VectorPoint>) {
        for point in points {
            self.ids.push(point.id.clone());
            self.points.push(point);
        }
        self.needs_rebuild = true;
    }

    /// Builds or rebuilds the HNSW index
    pub fn build(&mut self) {
        if self.points.is_empty() {
            return;
        }

        let hnsw = Builder::default()
            .ef_construction(self.config.ef_construction)
            .build(self.points.clone(), self.ids.clone());

        self.hnsw = Some(hnsw);
        self.needs_rebuild = false;
    }

    /// Searches for k nearest neighbors to the query vector
    pub fn search(&mut self, query: &[f32], k: usize) -> Vec<SearchResult> {
        if self.needs_rebuild {
            self.build();
        }

        let Some(ref hnsw) = self.hnsw else {
            return Vec::new();
        };

        let query_point = VectorPoint {
            id: String::new(),
            embedding: query.to_vec(),
            metadata: serde_json::Value::Null,
        };

        let mut search = Search::default();
        let results: Vec<_> = hnsw.search(&query_point, &mut search)
            .take(k)
            .map(|item| {
                let idx = item.pid.into_inner() as usize;
                let point = &self.points[idx];
                SearchResult {
                    id: point.id.clone(),
                    distance: item.distance,
                    similarity: 1.0 - item.distance,
                    metadata: point.metadata.clone(),
                }
            })
            .collect();

        results
    }

    /// Removes a point from the index by ID
    pub fn remove(&mut self, id: &str) -> bool {
        if let Some(pos) = self.points.iter().position(|p| p.id == id) {
            self.points.remove(pos);
            self.ids.remove(pos);
            self.needs_rebuild = true;
            true
        } else {
            false
        }
    }

    /// Gets a point by ID
    pub fn get(&self, id: &str) -> Option<&VectorPoint> {
        self.points.iter().find(|p| p.id == id)
    }

    /// Returns the number of points in the index
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns true if the index is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

/// Search result from HNSW query
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub distance: f32,
    pub similarity: f32,
    pub metadata: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hnsw_basic_search() {
        let mut index = HnswIndex::new(HnswConfig::default());

        // Insert some vectors
        for i in 0..100 {
            let embedding: Vec<f32> = (0..128).map(|j| (i * j) as f32 / 1000.0).collect();
            index.insert(VectorPoint {
                id: format!("vec_{}", i),
                embedding,
                metadata: serde_json::json!({"index": i}),
            });
        }

        // Search for vector similar to vec_50
        let query: Vec<f32> = (0..128).map(|j| (50 * j) as f32 / 1000.0).collect();
        let results = index.search(&query, 10);

        // HNSW is approximate - just verify we get results
        assert!(!results.is_empty());
        assert!(results.len() <= 10);
        // Top result should have reasonable similarity
        assert!(results[0].similarity > 0.5);
    }

    #[test]
    fn test_hnsw_empty_index() {
        let mut index = HnswIndex::new(HnswConfig::default());
        let query = vec![0.1, 0.2, 0.3];
        let results = index.search(&query, 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_hnsw_batch_insert() {
        let mut index = HnswIndex::new(HnswConfig::default());

        let points: Vec<VectorPoint> = (0..50).map(|i| {
            let embedding: Vec<f32> = (0..64).map(|j| (i * j) as f32 / 100.0).collect();
            VectorPoint {
                id: format!("batch_{}", i),
                embedding,
                metadata: serde_json::json!({"batch": i}),
            }
        }).collect();

        index.insert_batch(points);
        assert_eq!(index.len(), 50);

        let query: Vec<f32> = (0..64).map(|j| (25 * j) as f32 / 100.0).collect();
        let results = index.search(&query, 3);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_hnsw_remove() {
        let mut index = HnswIndex::new(HnswConfig::default());

        for i in 0..10 {
            let embedding: Vec<f32> = vec![i as f32; 32];
            index.insert(VectorPoint {
                id: format!("vec_{}", i),
                embedding,
                metadata: serde_json::Value::Null,
            });
        }

        assert_eq!(index.len(), 10);
        assert!(index.remove("vec_5"));
        assert_eq!(index.len(), 9);
        assert!(!index.remove("vec_5"));
    }

    #[test]
    fn test_hnsw_get() {
        let mut index = HnswIndex::new(HnswConfig::default());

        let embedding = vec![1.0, 2.0, 3.0];
        index.insert(VectorPoint {
            id: "test_vec".to_string(),
            embedding: embedding.clone(),
            metadata: serde_json::json!({"test": true}),
        });

        let point = index.get("test_vec");
        assert!(point.is_some());
        assert_eq!(point.unwrap().embedding, embedding);

        let missing = index.get("nonexistent");
        assert!(missing.is_none());
    }

    #[test]
    fn test_vector_point_distance() {
        use instant_distance::Point;

        let p1 = VectorPoint {
            id: "1".to_string(),
            embedding: vec![1.0, 0.0, 0.0],
            metadata: serde_json::Value::Null,
        };

        let p2 = VectorPoint {
            id: "2".to_string(),
            embedding: vec![1.0, 0.0, 0.0],
            metadata: serde_json::Value::Null,
        };

        let p3 = VectorPoint {
            id: "3".to_string(),
            embedding: vec![0.0, 1.0, 0.0],
            metadata: serde_json::Value::Null,
        };

        // Same vector should have distance ~0
        assert!(p1.distance(&p2) < 0.001);

        // Orthogonal vectors should have distance = 1
        assert!((p1.distance(&p3) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_hnsw_cosine_similarity() {
        let mut index = HnswIndex::new(HnswConfig::default());

        // Insert many vectors including orthogonal ones
        // HNSW works better with larger datasets
        for i in 0..50 {
            let angle = (i as f32) * std::f32::consts::PI / 25.0;
            index.insert(VectorPoint {
                id: format!("vec_{}", i),
                embedding: vec![angle.cos(), angle.sin(), 0.0],
                metadata: serde_json::Value::Null,
            });
        }

        // Also insert our specific test vectors
        index.insert(VectorPoint {
            id: "x_axis".to_string(),
            embedding: vec![1.0, 0.0, 0.0],
            metadata: serde_json::Value::Null,
        });

        // Search with x-axis
        let results = index.search(&[1.0, 0.0, 0.0], 10);
        assert!(!results.is_empty());

        // HNSW is approximate - just verify we get reasonable results
        // Top results should have decent similarity
        assert!(results[0].similarity > 0.7, "Top result should have >70% similarity");
    }
}
