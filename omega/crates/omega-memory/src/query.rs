//! Memory query types and execution

use crate::MemoryTier;
use serde::{Deserialize, Serialize};

/// Memory query structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Text-based search
    pub text: Option<String>,

    /// Vector embedding for similarity search
    pub embedding: Option<Vec<f32>>,

    /// Minimum importance threshold
    pub min_importance: Option<f64>,

    /// Maximum number of results
    pub limit: Option<usize>,

    /// Specific tiers to search (if None, searches all)
    pub tiers: Option<Vec<MemoryTier>>,

    /// Metadata filters
    pub metadata: Option<serde_json::Value>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            text: None,
            embedding: None,
            min_importance: None,
            limit: None,
            tiers: None,
            metadata: None,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }

    pub fn with_min_importance(mut self, importance: f64) -> Self {
        self.min_importance = Some(importance);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_tiers(mut self, tiers: Vec<MemoryTier>) -> Self {
        self.tiers = Some(tiers);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

/// Query builder for fluent API
pub struct QueryBuilder {
    query: Query,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            query: Query::new(),
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.query.text = Some(text.into());
        self
    }

    pub fn embedding(mut self, embedding: Vec<f32>) -> Self {
        self.query.embedding = Some(embedding);
        self
    }

    pub fn min_importance(mut self, importance: f64) -> Self {
        self.query.min_importance = Some(importance);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.query.limit = Some(limit);
        self
    }

    pub fn tier(mut self, tier: MemoryTier) -> Self {
        self.query.tiers = Some(vec![tier]);
        self
    }

    pub fn tiers(mut self, tiers: Vec<MemoryTier>) -> Self {
        self.query.tiers = Some(tiers);
        self
    }

    pub fn instant(self) -> Self {
        self.tier(MemoryTier::Instant)
    }

    pub fn session(self) -> Self {
        self.tier(MemoryTier::Session)
    }

    pub fn episodic(self) -> Self {
        self.tier(MemoryTier::Episodic)
    }

    pub fn semantic(self) -> Self {
        self.tier(MemoryTier::Semantic)
    }

    pub fn individual(self) -> Self {
        self.tiers(vec![
            MemoryTier::Instant,
            MemoryTier::Session,
            MemoryTier::Episodic,
            MemoryTier::Semantic,
        ])
    }

    pub fn species(self) -> Self {
        self.tiers(vec![
            MemoryTier::Collective,
            MemoryTier::Evolutionary,
            MemoryTier::Architectural,
            MemoryTier::Substrate,
        ])
    }

    pub fn cosmic(self) -> Self {
        self.tiers(vec![
            MemoryTier::Civilizational,
            MemoryTier::Temporal,
            MemoryTier::Physical,
            MemoryTier::Omega,
        ])
    }

    pub fn all_tiers(self) -> Self {
        self.tiers(MemoryTier::all())
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.query.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Query {
        self.query
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .text("test")
            .min_importance(0.5)
            .limit(10)
            .instant()
            .build();

        assert_eq!(query.text, Some("test".to_string()));
        assert_eq!(query.min_importance, Some(0.5));
        assert_eq!(query.limit, Some(10));
        assert_eq!(query.tiers, Some(vec![MemoryTier::Instant]));
    }

    #[test]
    fn test_query_fluent_api() {
        let query = Query::new()
            .with_text("search")
            .with_limit(5)
            .with_min_importance(0.3);

        assert_eq!(query.text, Some("search".to_string()));
        assert_eq!(query.limit, Some(5));
        assert_eq!(query.min_importance, Some(0.3));
    }
}
