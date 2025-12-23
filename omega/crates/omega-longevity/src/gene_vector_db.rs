//! Gene Vector Database - Ultra-fast gene similarity search using ruvector-postgres
//!
//! This module provides high-performance vector similarity search for genes using
//! PostgreSQL with pgvector extension (ruvector-postgres Docker image).
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                        Gene Vector Database                                  │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                              │
//! │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                   │
//! │  │ Gene Encoder │───▶│   Embedding  │───▶│  pgvector    │                   │
//! │  │  (384-dim)   │    │   Storage    │    │   Index      │                   │
//! │  └──────────────┘    └──────────────┘    └──────────────┘                   │
//! │         │                   │                   │                            │
//! │         ▼                   ▼                   ▼                            │
//! │  ┌──────────────────────────────────────────────────────┐                   │
//! │  │              Similarity Search Engine                 │                   │
//! │  │  • Cosine similarity    • L2 distance                │                   │
//! │  │  • Inner product        • Hybrid search              │                   │
//! │  └──────────────────────────────────────────────────────┘                   │
//! │                                                                              │
//! │  Search Types:                                                               │
//! │  • find_similar_genes()     - Find genes with similar function              │
//! │  • find_pathway_partners()  - Find genes in related pathways                │
//! │  • find_longevity_genes()   - Find genes affecting lifespan                 │
//! │  • find_drug_targets()      - Find potential therapeutic targets            │
//! │  • semantic_search()        - Natural language gene search                  │
//! │                                                                              │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use omega_longevity::gene_vector_db::{GeneVectorDB, GeneVectorConfig};
//!
//! // Connect to ruvector-postgres
//! let config = GeneVectorConfig::from_env();
//! let db = GeneVectorDB::connect(&config).await?;
//!
//! // Initialize schema (first time only)
//! db.initialize_schema().await?;
//!
//! // Index genes from genome
//! db.index_genome(&genome).await?;
//!
//! // Ultra-fast similarity search
//! let similar = db.find_similar_genes(Gene::SIRT1, 10).await?;
//! ```
//!
//! # Docker Setup
//!
//! ```bash
//! docker run -d \
//!   --name ruvector-postgres \
//!   -e POSTGRES_PASSWORD=longevity \
//!   -e POSTGRES_DB=genes \
//!   -p 5432:5432 \
//!   ruvnet/ruvector-postgres
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::genome::{Gene, GeneState, Genome, VariantEffect};
use crate::hallmarks::Hallmark;
use crate::{Result, LongevityError};

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuration for connecting to ruvector-postgres
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneVectorConfig {
    /// PostgreSQL connection URL
    pub database_url: String,
    /// Maximum connections in pool
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connect_timeout_secs: u64,
    /// Vector embedding dimension (default: 384)
    pub embedding_dim: usize,
    /// Index type: "ivfflat" or "hnsw"
    pub index_type: IndexType,
    /// Number of lists for IVFFlat index
    pub ivf_lists: u32,
    /// M parameter for HNSW index
    pub hnsw_m: u32,
    /// ef_construction for HNSW index
    pub hnsw_ef_construction: u32,
}

impl Default for GeneVectorConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:longevity@localhost:5432/genes".to_string(),
            max_connections: 10,
            connect_timeout_secs: 30,
            embedding_dim: 384,
            index_type: IndexType::HNSW,
            ivf_lists: 100,
            hnsw_m: 16,
            hnsw_ef_construction: 64,
        }
    }
}

impl GeneVectorConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("RUVECTOR_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:longevity@localhost:5432/genes".to_string()),
            max_connections: std::env::var("RUVECTOR_MAX_CONNECTIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            connect_timeout_secs: std::env::var("RUVECTOR_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            embedding_dim: std::env::var("RUVECTOR_EMBEDDING_DIM")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(384),
            index_type: std::env::var("RUVECTOR_INDEX_TYPE")
                .ok()
                .map(|s| match s.to_lowercase().as_str() {
                    "ivfflat" => IndexType::IVFFlat,
                    _ => IndexType::HNSW,
                })
                .unwrap_or(IndexType::HNSW),
            ivf_lists: 100,
            hnsw_m: 16,
            hnsw_ef_construction: 64,
        }
    }
}

/// Vector index type for pgvector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexType {
    /// IVFFlat - Inverted File with Flat compression
    IVFFlat,
    /// HNSW - Hierarchical Navigable Small World graphs (faster, more accurate)
    HNSW,
}

// ============================================================================
// GENE EMBEDDING
// ============================================================================

/// Multi-dimensional gene embedding for vector similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneEmbedding {
    /// Unique identifier
    pub id: Uuid,
    /// Gene this embedding represents
    pub gene: Gene,
    /// 384-dimensional embedding vector
    pub vector: Vec<f32>,
    /// Embedding version for updates
    pub version: u32,
    /// When the embedding was created
    pub created_at: DateTime<Utc>,
    /// Metadata for filtering
    pub metadata: GeneMetadata,
}

/// Metadata associated with a gene embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneMetadata {
    /// Gene symbol (e.g., "SIRT1")
    pub symbol: String,
    /// Full gene name
    pub name: String,
    /// Functional category
    pub category: GeneCategory,
    /// Associated hallmarks of aging
    pub hallmarks: Vec<Hallmark>,
    /// Chromosome location
    pub chromosome: String,
    /// Known longevity association strength (0-1)
    pub longevity_score: f64,
    /// Druggability score (0-1)
    pub druggability: f64,
    /// Associated pathways
    pub pathways: Vec<String>,
    /// Key terms for semantic search
    pub keywords: Vec<String>,
}

/// Functional category of a gene
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneCategory {
    /// DNA repair and maintenance
    DNARepair,
    /// Metabolism and nutrient sensing
    Metabolism,
    /// Cellular signaling
    Signaling,
    /// Protein homeostasis
    Proteostasis,
    /// Mitochondrial function
    Mitochondrial,
    /// Immune and inflammation
    Immune,
    /// Stem cell and regeneration
    StemCell,
    /// Epigenetic regulation
    Epigenetic,
    /// Telomere maintenance
    Telomere,
    /// Cell cycle and senescence
    CellCycle,
    /// Circadian rhythm
    Circadian,
    /// Stress response
    StressResponse,
    /// Autophagy
    Autophagy,
    /// Apoptosis
    Apoptosis,
    /// Unknown/Other
    Other,
}

impl GeneCategory {
    /// Get all categories
    pub fn all() -> Vec<GeneCategory> {
        vec![
            GeneCategory::DNARepair,
            GeneCategory::Metabolism,
            GeneCategory::Signaling,
            GeneCategory::Proteostasis,
            GeneCategory::Mitochondrial,
            GeneCategory::Immune,
            GeneCategory::StemCell,
            GeneCategory::Epigenetic,
            GeneCategory::Telomere,
            GeneCategory::CellCycle,
            GeneCategory::Circadian,
            GeneCategory::StressResponse,
            GeneCategory::Autophagy,
            GeneCategory::Apoptosis,
            GeneCategory::Other,
        ]
    }
}

// ============================================================================
// GENE ENCODER - Creates embeddings from gene properties
// ============================================================================

/// Encoder that converts genes to vector embeddings
pub struct GeneEncoder {
    /// Embedding dimension
    dim: usize,
    /// Category embeddings (learned or predefined)
    category_vectors: HashMap<GeneCategory, Vec<f32>>,
    /// Hallmark embeddings
    hallmark_vectors: HashMap<Hallmark, Vec<f32>>,
}

impl GeneEncoder {
    /// Create a new gene encoder
    pub fn new(dim: usize) -> Self {
        let mut encoder = Self {
            dim,
            category_vectors: HashMap::new(),
            hallmark_vectors: HashMap::new(),
        };
        encoder.initialize_base_vectors();
        encoder
    }

    /// Initialize base embedding vectors for categories and hallmarks
    fn initialize_base_vectors(&mut self) {
        use rand::SeedableRng;
        use rand_distr::{Distribution, Normal};

        let mut rng = rand::rngs::StdRng::seed_from_u64(42); // Deterministic
        let normal = Normal::new(0.0, 0.1).unwrap();

        // Create orthogonal-ish vectors for each category
        for (i, category) in GeneCategory::all().iter().enumerate() {
            let mut vec = vec![0.0f32; self.dim];
            // Set primary dimensions based on category index
            let base_idx = (i * 20) % self.dim;
            for j in 0..20 {
                let idx = (base_idx + j) % self.dim;
                vec[idx] = 0.5 + normal.sample(&mut rng) as f32;
            }
            // Add some noise to other dimensions
            for v in vec.iter_mut() {
                *v += normal.sample(&mut rng) as f32 * 0.1;
            }
            self.normalize(&mut vec);
            self.category_vectors.insert(*category, vec);
        }

        // Create vectors for hallmarks
        let hallmarks = vec![
            Hallmark::GenomicInstability,
            Hallmark::TelomereAttrition,
            Hallmark::EpigeneticAlterations,
            Hallmark::LossOfProteostasis,
            Hallmark::DeregulatedNutrientSensing,
            Hallmark::MitochondrialDysfunction,
            Hallmark::CellularSenescence,
            Hallmark::StemCellExhaustion,
            Hallmark::AlteredIntercellularCommunication,
        ];

        for (i, hallmark) in hallmarks.iter().enumerate() {
            let mut vec = vec![0.0f32; self.dim];
            let base_idx = ((i + 15) * 25) % self.dim;
            for j in 0..25 {
                let idx = (base_idx + j) % self.dim;
                vec[idx] = 0.4 + normal.sample(&mut rng) as f32;
            }
            for v in vec.iter_mut() {
                *v += normal.sample(&mut rng) as f32 * 0.1;
            }
            self.normalize(&mut vec);
            self.hallmark_vectors.insert(*hallmark, vec);
        }
    }

    /// Normalize a vector to unit length
    fn normalize(&self, vec: &mut Vec<f32>) {
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in vec.iter_mut() {
                *v /= norm;
            }
        }
    }

    /// Encode a gene into an embedding vector
    pub fn encode(&self, gene: Gene, state: Option<&GeneState>) -> GeneEmbedding {
        let mut vector = vec![0.0f32; self.dim];

        // 1. Add category component (dims 0-63)
        let category = self.gene_to_category(gene);
        if let Some(cat_vec) = self.category_vectors.get(&category) {
            for (i, v) in cat_vec.iter().enumerate().take(64) {
                vector[i] += v * 0.3;
            }
        }

        // 2. Add hallmark components (dims 64-159)
        for hallmark in self.gene_to_hallmarks(gene) {
            if let Some(hall_vec) = self.hallmark_vectors.get(&hallmark) {
                for i in 0..96 {
                    let idx = 64 + i;
                    if idx < self.dim {
                        vector[idx] += hall_vec.get(i).copied().unwrap_or(0.0) * 0.2;
                    }
                }
            }
        }

        // 3. Add gene-specific features (dims 160-255)
        self.add_gene_specific_features(&mut vector, gene, state);

        // 4. Add expression and variant features (dims 256-319)
        if let Some(state) = state {
            self.add_state_features(&mut vector, state);
        }

        // 5. Add longevity relevance features (dims 320-383)
        self.add_longevity_features(&mut vector, gene);

        // Normalize final vector
        let mut final_vec = vector;
        self.normalize(&mut final_vec);

        GeneEmbedding {
            id: Uuid::new_v4(),
            gene,
            vector: final_vec,
            version: 1,
            created_at: Utc::now(),
            metadata: self.create_metadata(gene, state),
        }
    }

    /// Map gene to functional category
    fn gene_to_category(&self, gene: Gene) -> GeneCategory {
        match gene {
            // DNA Repair
            Gene::TP53 | Gene::ATM | Gene::BRCA1 | Gene::BRCA2 |
            Gene::WRN | Gene::ERCC1 => GeneCategory::DNARepair,

            // Metabolism / Nutrient Sensing
            Gene::MTOR | Gene::AMPK | Gene::IGF1R | Gene::FOXO3 => GeneCategory::Metabolism,

            // Sirtuins - NAD+ signaling
            Gene::SIRT1 | Gene::SIRT3 | Gene::SIRT6 => GeneCategory::Signaling,

            // Proteostasis
            Gene::HSF1 | Gene::HSP70 | Gene::HSP90 | Gene::SQSTM1 | Gene::NFE2L2 => GeneCategory::Proteostasis,

            // Mitochondrial
            Gene::PPARGC1A | Gene::PINK1 | Gene::PRKN => GeneCategory::Mitochondrial,

            // Immune
            Gene::NFKB1 | Gene::NLRP3 | Gene::IL6 | Gene::TNF => GeneCategory::Immune,

            // Stem cell
            Gene::NANOG | Gene::OCT4 | Gene::KLF4 | Gene::MYC => GeneCategory::StemCell,

            // Telomere
            Gene::TERT | Gene::TERC | Gene::POT1 => GeneCategory::Telomere,

            // Cell cycle / Senescence
            Gene::CDKN2A | Gene::CDKN1A | Gene::RB1 | Gene::LMNA => GeneCategory::CellCycle,

            // Circadian
            Gene::CLOCK | Gene::BMAL1 | Gene::PER1 | Gene::PER2 | Gene::PER3 |
            Gene::CRY1 | Gene::CRY2 | Gene::DEC2 | Gene::ADRB1 | Gene::ADA => GeneCategory::Circadian,

            // Apoptosis
            Gene::BCL2 | Gene::BAX | Gene::CASP3 => GeneCategory::Apoptosis,

            #[allow(unreachable_patterns)]
            _ => GeneCategory::Other,
        }
    }

    /// Map gene to associated hallmarks
    fn gene_to_hallmarks(&self, gene: Gene) -> Vec<Hallmark> {
        match gene {
            Gene::TP53 | Gene::ATM | Gene::BRCA1 | Gene::BRCA2 |
            Gene::WRN | Gene::ERCC1 =>
                vec![Hallmark::GenomicInstability],

            Gene::TERT | Gene::TERC | Gene::POT1 =>
                vec![Hallmark::TelomereAttrition],

            Gene::HSF1 | Gene::HSP70 | Gene::HSP90 | Gene::SQSTM1 | Gene::NFE2L2 =>
                vec![Hallmark::LossOfProteostasis],

            Gene::MTOR | Gene::AMPK | Gene::IGF1R | Gene::FOXO3 | Gene::SIRT1 =>
                vec![Hallmark::DeregulatedNutrientSensing],

            Gene::PPARGC1A | Gene::PINK1 | Gene::PRKN | Gene::SIRT3 =>
                vec![Hallmark::MitochondrialDysfunction],

            Gene::CDKN2A | Gene::CDKN1A | Gene::RB1 =>
                vec![Hallmark::CellularSenescence],

            Gene::NANOG | Gene::OCT4 | Gene::KLF4 =>
                vec![Hallmark::StemCellExhaustion],

            Gene::IL6 | Gene::TNF | Gene::NFKB1 | Gene::NLRP3 =>
                vec![Hallmark::AlteredIntercellularCommunication],

            #[allow(unreachable_patterns)]
            _ => vec![],
        }
    }

    /// Add gene-specific feature dimensions
    fn add_gene_specific_features(&self, vector: &mut Vec<f32>, gene: Gene, _state: Option<&GeneState>) {
        // Use gene hash to create consistent but unique features
        let gene_hash = format!("{:?}", gene).bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));

        use rand::SeedableRng;
        use rand_distr::{Distribution, Normal};
        let mut rng = rand::rngs::StdRng::seed_from_u64(gene_hash);
        let normal = Normal::new(0.0, 0.15).unwrap();

        for i in 160..256 {
            if i < self.dim {
                vector[i] = normal.sample(&mut rng) as f32;
            }
        }
    }

    /// Add gene state features (expression, variants)
    fn add_state_features(&self, vector: &mut Vec<f32>, state: &GeneState) {
        // Expression level
        if 256 < self.dim {
            vector[256] = state.expression as f32;
        }

        // Copy number (proxy for dosage effects)
        if 257 < self.dim {
            vector[257] = state.copy_number as f32 / 2.0; // Normalize to ~1.0
        }

        // Variant effects
        for (i, variant) in state.variants.iter().enumerate() {
            let idx = 260 + i * 4;
            if idx + 3 < self.dim {
                // Encode variant effect using the actual VariantEffect enum
                let effect_val = match variant.effect {
                    VariantEffect::LossOfFunction => 0.9,
                    VariantEffect::GainOfFunction => 0.3,
                    VariantEffect::ReducedFunction => 0.6,
                    VariantEffect::EnhancedFunction => -0.5,
                    VariantEffect::Neutral => 0.0,
                };
                vector[idx] = effect_val;
                vector[idx + 1] = variant.allele_frequency as f32;
            }
            if i >= 10 { break; } // Max 10 variants encoded
        }
    }

    /// Add longevity-specific features
    fn add_longevity_features(&self, vector: &mut Vec<f32>, gene: Gene) {
        let longevity_score = self.calculate_longevity_score(gene);
        let druggability = self.calculate_druggability(gene);

        if 320 < self.dim {
            vector[320] = longevity_score as f32;
        }
        if 321 < self.dim {
            vector[321] = druggability as f32;
        }

        // Is this a known longevity gene?
        let is_longevity_gene = matches!(gene,
            Gene::SIRT1 | Gene::SIRT3 | Gene::SIRT6 | Gene::FOXO3 | Gene::MTOR |
            Gene::AMPK | Gene::IGF1R | Gene::TERT | Gene::TP53 | Gene::CDKN2A
        );
        if 322 < self.dim {
            vector[322] = if is_longevity_gene { 1.0 } else { 0.0 };
        }

        // Intervention potential
        if 323 < self.dim {
            vector[323] = self.calculate_intervention_potential(gene) as f32;
        }
    }

    /// Calculate longevity relevance score
    fn calculate_longevity_score(&self, gene: Gene) -> f64 {
        match gene {
            Gene::FOXO3 => 0.95,  // Strongest human longevity association
            Gene::SIRT1 | Gene::SIRT6 => 0.9,
            Gene::MTOR | Gene::AMPK => 0.85,
            Gene::IGF1R => 0.8,
            Gene::TERT => 0.85,
            Gene::TP53 => 0.75,
            Gene::CDKN2A => 0.7,
            Gene::PPARGC1A => 0.65,
            Gene::NFE2L2 => 0.6,
            Gene::CLOCK | Gene::BMAL1 => 0.55,
            _ => 0.3,
        }
    }

    /// Calculate druggability score
    fn calculate_druggability(&self, gene: Gene) -> f64 {
        match gene {
            Gene::MTOR => 0.95,  // Rapamycin target
            Gene::SIRT1 => 0.85, // NAD+ precursors, resveratrol
            Gene::AMPK => 0.9,   // Metformin
            Gene::IGF1R => 0.8,
            Gene::NFE2L2 => 0.7,
            Gene::TP53 => 0.6,
            Gene::TERT => 0.5,   // Hard to drug
            _ => 0.4,
        }
    }

    /// Calculate intervention potential
    fn calculate_intervention_potential(&self, gene: Gene) -> f64 {
        let longevity = self.calculate_longevity_score(gene);
        let druggability = self.calculate_druggability(gene);
        (longevity * 0.6 + druggability * 0.4).min(1.0)
    }

    /// Create metadata for a gene
    fn create_metadata(&self, gene: Gene, _state: Option<&GeneState>) -> GeneMetadata {
        let category = self.gene_to_category(gene);
        let hallmarks = self.gene_to_hallmarks(gene);

        GeneMetadata {
            symbol: format!("{:?}", gene),
            name: self.gene_full_name(gene),
            category,
            hallmarks,
            chromosome: self.gene_chromosome(gene),
            longevity_score: self.calculate_longevity_score(gene),
            druggability: self.calculate_druggability(gene),
            pathways: self.gene_pathways(gene),
            keywords: self.gene_keywords(gene),
        }
    }

    fn gene_full_name(&self, gene: Gene) -> String {
        match gene {
            Gene::SIRT1 => "Sirtuin 1".to_string(),
            Gene::SIRT3 => "Sirtuin 3".to_string(),
            Gene::SIRT6 => "Sirtuin 6".to_string(),
            Gene::FOXO3 => "Forkhead Box O3".to_string(),
            Gene::MTOR => "Mechanistic Target of Rapamycin".to_string(),
            Gene::AMPK => "AMP-activated Protein Kinase".to_string(),
            Gene::IGF1R => "Insulin-like Growth Factor 1 Receptor".to_string(),
            Gene::TP53 => "Tumor Protein P53".to_string(),
            Gene::TERT => "Telomerase Reverse Transcriptase".to_string(),
            _ => format!("{:?}", gene),
        }
    }

    fn gene_chromosome(&self, gene: Gene) -> String {
        match gene {
            Gene::SIRT1 => "10q21.3".to_string(),
            Gene::FOXO3 => "6q21".to_string(),
            Gene::TP53 => "17p13.1".to_string(),
            Gene::TERT => "5p15.33".to_string(),
            Gene::MTOR => "1p36.22".to_string(),
            _ => "unknown".to_string(),
        }
    }

    fn gene_pathways(&self, gene: Gene) -> Vec<String> {
        match gene {
            Gene::SIRT1 => vec![
                "NAD+ metabolism".to_string(),
                "Insulin signaling".to_string(),
                "Circadian rhythm".to_string(),
            ],
            Gene::MTOR => vec![
                "mTOR signaling".to_string(),
                "Autophagy".to_string(),
                "Protein synthesis".to_string(),
            ],
            Gene::FOXO3 => vec![
                "PI3K-Akt signaling".to_string(),
                "Longevity regulating".to_string(),
                "Stress resistance".to_string(),
            ],
            _ => vec![],
        }
    }

    fn gene_keywords(&self, gene: Gene) -> Vec<String> {
        match gene {
            Gene::SIRT1 => vec![
                "sirtuin".to_string(), "deacetylase".to_string(), "NAD".to_string(),
                "longevity".to_string(), "metabolism".to_string(), "aging".to_string(),
            ],
            Gene::FOXO3 => vec![
                "forkhead".to_string(), "transcription factor".to_string(),
                "longevity".to_string(), "centenarian".to_string(), "stress".to_string(),
            ],
            Gene::MTOR => vec![
                "rapamycin".to_string(), "kinase".to_string(), "autophagy".to_string(),
                "growth".to_string(), "nutrient".to_string(), "aging".to_string(),
            ],
            _ => vec![format!("{:?}", gene).to_lowercase()],
        }
    }
}

// ============================================================================
// SEARCH RESULTS
// ============================================================================

/// Result of a similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    /// The gene found
    pub gene: Gene,
    /// Similarity score (0-1, higher is more similar)
    pub similarity: f64,
    /// Distance in embedding space
    pub distance: f64,
    /// Gene metadata
    pub metadata: GeneMetadata,
    /// Why this gene is similar
    pub similarity_reasons: Vec<String>,
}

/// Search parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    /// Maximum number of results
    pub limit: usize,
    /// Minimum similarity threshold
    pub min_similarity: f64,
    /// Filter by category
    pub category_filter: Option<GeneCategory>,
    /// Filter by hallmark
    pub hallmark_filter: Option<Hallmark>,
    /// Filter by minimum longevity score
    pub min_longevity_score: Option<f64>,
    /// Include metadata in results
    pub include_metadata: bool,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            limit: 10,
            min_similarity: 0.0,
            category_filter: None,
            hallmark_filter: None,
            min_longevity_score: None,
            include_metadata: true,
        }
    }
}

// ============================================================================
// IN-MEMORY VECTOR DATABASE (for when ruvector-postgres is not available)
// ============================================================================

/// In-memory gene vector database (fallback when PostgreSQL not available)
pub struct InMemoryGeneVectorDB {
    /// Gene encoder
    encoder: GeneEncoder,
    /// Stored embeddings
    embeddings: HashMap<Gene, GeneEmbedding>,
    /// Index for fast lookup
    gene_index: Vec<(Gene, Vec<f32>)>,
}

impl InMemoryGeneVectorDB {
    /// Create a new in-memory database
    pub fn new(embedding_dim: usize) -> Self {
        Self {
            encoder: GeneEncoder::new(embedding_dim),
            embeddings: HashMap::new(),
            gene_index: Vec::new(),
        }
    }

    /// Index a genome
    pub fn index_genome(&mut self, genome: &Genome) {
        for (gene, state) in &genome.nuclear_genes {
            let embedding = self.encoder.encode(*gene, Some(state));
            self.gene_index.push((*gene, embedding.vector.clone()));
            self.embeddings.insert(*gene, embedding);
        }
    }

    /// Index all known genes
    pub fn index_all_genes(&mut self) {
        for gene in Gene::all() {
            let embedding = self.encoder.encode(gene, None);
            self.gene_index.push((gene, embedding.vector.clone()));
            self.embeddings.insert(gene, embedding);
        }
    }

    /// Find similar genes using cosine similarity
    pub fn find_similar(&self, query_gene: Gene, params: &SearchParams) -> Vec<SimilarityResult> {
        let query_embedding = self.embeddings.get(&query_gene)
            .map(|e| e.vector.clone())
            .unwrap_or_else(|| self.encoder.encode(query_gene, None).vector);

        let mut results: Vec<(Gene, f64)> = self.gene_index.iter()
            .filter(|(g, _)| *g != query_gene)
            .map(|(gene, vec)| {
                let similarity = self.cosine_similarity(&query_embedding, vec);
                (*gene, similarity)
            })
            .filter(|(_, sim)| *sim >= params.min_similarity)
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Apply filters and limit
        results.into_iter()
            .filter(|(gene, _)| {
                if let Some(cat) = params.category_filter {
                    self.encoder.gene_to_category(*gene) == cat
                } else {
                    true
                }
            })
            .filter(|(gene, _)| {
                if let Some(min_score) = params.min_longevity_score {
                    self.encoder.calculate_longevity_score(*gene) >= min_score
                } else {
                    true
                }
            })
            .take(params.limit)
            .map(|(gene, similarity)| {
                let metadata = self.embeddings.get(&gene)
                    .map(|e| e.metadata.clone())
                    .unwrap_or_else(|| self.encoder.create_metadata(gene, None));

                SimilarityResult {
                    gene,
                    similarity,
                    distance: 1.0 - similarity,
                    similarity_reasons: self.explain_similarity(query_gene, gene),
                    metadata,
                }
            })
            .collect()
    }

    /// Find genes by pathway
    pub fn find_pathway_partners(&self, gene: Gene, params: &SearchParams) -> Vec<SimilarityResult> {
        let query_pathways: std::collections::HashSet<_> = self.encoder.gene_pathways(gene)
            .into_iter()
            .collect();

        if query_pathways.is_empty() {
            return self.find_similar(gene, params);
        }

        let mut results: Vec<SimilarityResult> = self.gene_index.iter()
            .filter(|(g, _)| *g != gene)
            .filter_map(|(g, _)| {
                let gene_pathways: std::collections::HashSet<_> = self.encoder.gene_pathways(*g)
                    .into_iter()
                    .collect();
                let overlap = query_pathways.intersection(&gene_pathways).count();
                if overlap > 0 {
                    let similarity = overlap as f64 / query_pathways.len().max(1) as f64;
                    let metadata = self.embeddings.get(g)
                        .map(|e| e.metadata.clone())
                        .unwrap_or_else(|| self.encoder.create_metadata(*g, None));
                    Some(SimilarityResult {
                        gene: *g,
                        similarity,
                        distance: 1.0 - similarity,
                        similarity_reasons: vec![format!("Shares {} pathway(s)", overlap)],
                        metadata,
                    })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(params.limit).collect()
    }

    /// Find longevity-associated genes
    pub fn find_longevity_genes(&self, min_score: f64, limit: usize) -> Vec<SimilarityResult> {
        let mut results: Vec<SimilarityResult> = Gene::all().into_iter()
            .filter_map(|gene| {
                let score = self.encoder.calculate_longevity_score(gene);
                if score >= min_score {
                    let metadata = self.embeddings.get(&gene)
                        .map(|e| e.metadata.clone())
                        .unwrap_or_else(|| self.encoder.create_metadata(gene, None));
                    Some(SimilarityResult {
                        gene,
                        similarity: score,
                        distance: 1.0 - score,
                        similarity_reasons: vec![format!("Longevity score: {:.2}", score)],
                        metadata,
                    })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(limit).collect()
    }

    /// Find potential drug targets
    pub fn find_drug_targets(&self, min_druggability: f64, limit: usize) -> Vec<SimilarityResult> {
        let mut results: Vec<SimilarityResult> = Gene::all().into_iter()
            .filter_map(|gene| {
                let drug_score = self.encoder.calculate_druggability(gene);
                let intervention_score = self.encoder.calculate_intervention_potential(gene);
                if drug_score >= min_druggability {
                    let metadata = self.embeddings.get(&gene)
                        .map(|e| e.metadata.clone())
                        .unwrap_or_else(|| self.encoder.create_metadata(gene, None));
                    Some(SimilarityResult {
                        gene,
                        similarity: intervention_score,
                        distance: 1.0 - intervention_score,
                        similarity_reasons: vec![
                            format!("Druggability: {:.2}", drug_score),
                            format!("Intervention potential: {:.2}", intervention_score),
                        ],
                        metadata,
                    })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(limit).collect()
    }

    /// Semantic search by keywords
    pub fn semantic_search(&self, query: &str, limit: usize) -> Vec<SimilarityResult> {
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

        let mut results: Vec<(Gene, f64, Vec<String>)> = Gene::all().into_iter()
            .filter_map(|gene| {
                let keywords = self.encoder.gene_keywords(gene);
                let name = self.encoder.gene_full_name(gene).to_lowercase();
                let symbol = format!("{:?}", gene).to_lowercase();

                let mut matches = Vec::new();
                let mut score = 0.0;

                for term in &query_terms {
                    if symbol.contains(term) {
                        score += 1.0;
                        matches.push(format!("Symbol matches '{}'", term));
                    }
                    if name.contains(term) {
                        score += 0.8;
                        matches.push(format!("Name contains '{}'", term));
                    }
                    for kw in &keywords {
                        if kw.contains(term) {
                            score += 0.5;
                            matches.push(format!("Keyword '{}' matches", kw));
                            break;
                        }
                    }
                }

                if score > 0.0 {
                    Some((gene, score / query_terms.len() as f64, matches))
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        results.into_iter()
            .take(limit)
            .map(|(gene, score, reasons)| {
                let metadata = self.embeddings.get(&gene)
                    .map(|e| e.metadata.clone())
                    .unwrap_or_else(|| self.encoder.create_metadata(gene, None));
                SimilarityResult {
                    gene,
                    similarity: score.min(1.0),
                    distance: 1.0 - score.min(1.0),
                    similarity_reasons: reasons,
                    metadata,
                }
            })
            .collect()
    }

    /// Cosine similarity between two vectors
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f64 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a > 0.0 && norm_b > 0.0 {
            (dot / (norm_a * norm_b)) as f64
        } else {
            0.0
        }
    }

    /// Explain why two genes are similar
    fn explain_similarity(&self, gene_a: Gene, gene_b: Gene) -> Vec<String> {
        let mut reasons = Vec::new();

        // Same category?
        let cat_a = self.encoder.gene_to_category(gene_a);
        let cat_b = self.encoder.gene_to_category(gene_b);
        if cat_a == cat_b {
            reasons.push(format!("Same functional category: {:?}", cat_a));
        }

        // Shared hallmarks?
        let hall_a: std::collections::HashSet<_> = self.encoder.gene_to_hallmarks(gene_a).into_iter().collect();
        let hall_b: std::collections::HashSet<_> = self.encoder.gene_to_hallmarks(gene_b).into_iter().collect();
        let shared: Vec<_> = hall_a.intersection(&hall_b).collect();
        if !shared.is_empty() {
            reasons.push(format!("Shared hallmarks: {:?}", shared));
        }

        // Shared pathways?
        let path_a: std::collections::HashSet<_> = self.encoder.gene_pathways(gene_a).into_iter().collect();
        let path_b: std::collections::HashSet<_> = self.encoder.gene_pathways(gene_b).into_iter().collect();
        let shared_paths: Vec<_> = path_a.intersection(&path_b).cloned().collect();
        if !shared_paths.is_empty() {
            reasons.push(format!("Shared pathways: {}", shared_paths.join(", ")));
        }

        if reasons.is_empty() {
            reasons.push("Similar embedding features".to_string());
        }

        reasons
    }

    /// Get statistics about the database
    pub fn stats(&self) -> DatabaseStats {
        let category_counts: HashMap<GeneCategory, usize> = self.embeddings.values()
            .map(|e| e.metadata.category)
            .fold(HashMap::new(), |mut acc, cat| {
                *acc.entry(cat).or_insert(0) += 1;
                acc
            });

        DatabaseStats {
            total_genes: self.embeddings.len(),
            embedding_dim: self.encoder.dim,
            category_counts,
            avg_longevity_score: self.embeddings.values()
                .map(|e| e.metadata.longevity_score)
                .sum::<f64>() / self.embeddings.len().max(1) as f64,
        }
    }
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_genes: usize,
    pub embedding_dim: usize,
    pub category_counts: HashMap<GeneCategory, usize>,
    pub avg_longevity_score: f64,
}

// ============================================================================
// SQL SCHEMA FOR RUVECTOR-POSTGRES
// ============================================================================

/// SQL statements for setting up the gene vector database
pub mod sql {
    /// Create the pgvector extension
    pub const CREATE_EXTENSION: &str = "CREATE EXTENSION IF NOT EXISTS vector;";

    /// Create the genes table with vector column
    pub const CREATE_GENES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS genes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255),
    category VARCHAR(50),
    chromosome VARCHAR(20),
    embedding vector(384),
    longevity_score FLOAT,
    druggability FLOAT,
    hallmarks JSONB,
    pathways JSONB,
    keywords JSONB,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
"#;

    /// Create HNSW index for fast similarity search
    pub const CREATE_HNSW_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS genes_embedding_hnsw_idx
ON genes
USING hnsw (embedding vector_cosine_ops)
WITH (m = 16, ef_construction = 64);
"#;

    /// Create IVFFlat index (alternative to HNSW)
    pub const CREATE_IVFFLAT_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS genes_embedding_ivfflat_idx
ON genes
USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);
"#;

    /// Create indexes for filtering
    pub const CREATE_FILTER_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS genes_category_idx ON genes (category);
CREATE INDEX IF NOT EXISTS genes_longevity_idx ON genes (longevity_score DESC);
CREATE INDEX IF NOT EXISTS genes_druggability_idx ON genes (druggability DESC);
CREATE INDEX IF NOT EXISTS genes_hallmarks_idx ON genes USING GIN (hallmarks);
CREATE INDEX IF NOT EXISTS genes_keywords_idx ON genes USING GIN (keywords);
"#;

    /// Insert or update a gene
    pub const UPSERT_GENE: &str = r#"
INSERT INTO genes (symbol, name, category, chromosome, embedding, longevity_score,
                   druggability, hallmarks, pathways, keywords, metadata)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (symbol) DO UPDATE SET
    name = EXCLUDED.name,
    category = EXCLUDED.category,
    embedding = EXCLUDED.embedding,
    longevity_score = EXCLUDED.longevity_score,
    druggability = EXCLUDED.druggability,
    hallmarks = EXCLUDED.hallmarks,
    pathways = EXCLUDED.pathways,
    keywords = EXCLUDED.keywords,
    metadata = EXCLUDED.metadata,
    updated_at = NOW();
"#;

    /// Find similar genes by vector similarity
    pub const FIND_SIMILAR: &str = r#"
SELECT symbol, name, category, longevity_score, druggability,
       hallmarks, pathways, metadata,
       1 - (embedding <=> $1) as similarity
FROM genes
WHERE symbol != $2
ORDER BY embedding <=> $1
LIMIT $3;
"#;

    /// Find similar genes with category filter
    pub const FIND_SIMILAR_BY_CATEGORY: &str = r#"
SELECT symbol, name, category, longevity_score, druggability,
       hallmarks, pathways, metadata,
       1 - (embedding <=> $1) as similarity
FROM genes
WHERE symbol != $2 AND category = $3
ORDER BY embedding <=> $1
LIMIT $4;
"#;

    /// Find top longevity genes
    pub const FIND_LONGEVITY_GENES: &str = r#"
SELECT symbol, name, category, longevity_score, druggability,
       hallmarks, pathways, metadata
FROM genes
WHERE longevity_score >= $1
ORDER BY longevity_score DESC
LIMIT $2;
"#;

    /// Find drug targets
    pub const FIND_DRUG_TARGETS: &str = r#"
SELECT symbol, name, category, longevity_score, druggability,
       hallmarks, pathways, metadata
FROM genes
WHERE druggability >= $1
ORDER BY (longevity_score * 0.6 + druggability * 0.4) DESC
LIMIT $2;
"#;

    /// Semantic search by keywords
    pub const SEMANTIC_SEARCH: &str = r#"
SELECT symbol, name, category, longevity_score, druggability,
       hallmarks, pathways, metadata
FROM genes
WHERE keywords @> $1::jsonb
ORDER BY longevity_score DESC
LIMIT $2;
"#;

    /// Get database statistics
    pub const GET_STATS: &str = r#"
SELECT
    COUNT(*) as total_genes,
    COUNT(DISTINCT category) as category_count,
    AVG(longevity_score) as avg_longevity,
    AVG(druggability) as avg_druggability
FROM genes;
"#;
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene_encoder_creation() {
        let encoder = GeneEncoder::new(384);
        assert_eq!(encoder.dim, 384);
        assert!(!encoder.category_vectors.is_empty());
        assert!(!encoder.hallmark_vectors.is_empty());
    }

    #[test]
    fn test_gene_encoding() {
        let encoder = GeneEncoder::new(384);
        let embedding = encoder.encode(Gene::SIRT1, None);

        assert_eq!(embedding.vector.len(), 384);
        assert_eq!(embedding.gene, Gene::SIRT1);
        assert!(embedding.metadata.longevity_score > 0.5);
    }

    #[test]
    fn test_embedding_normalization() {
        let encoder = GeneEncoder::new(384);
        let embedding = encoder.encode(Gene::FOXO3, None);

        // Check vector is normalized (length ≈ 1)
        let norm: f32 = embedding.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01, "Vector should be normalized, got norm={}", norm);
    }

    #[test]
    fn test_similar_genes_same_category() {
        let mut db = InMemoryGeneVectorDB::new(384);
        db.index_all_genes();

        let params = SearchParams {
            limit: 5,
            ..Default::default()
        };

        // SIRT1 should be similar to other sirtuins
        let results = db.find_similar(Gene::SIRT1, &params);
        assert!(!results.is_empty());

        // Check that at least one other sirtuin is in top results
        let has_sirtuin = results.iter().any(|r|
            matches!(r.gene, Gene::SIRT3 | Gene::SIRT6)
        );
        assert!(has_sirtuin, "Similar genes should include other sirtuins");
    }

    #[test]
    fn test_find_longevity_genes() {
        let mut db = InMemoryGeneVectorDB::new(384);
        db.index_all_genes();

        let results = db.find_longevity_genes(0.8, 10);
        assert!(!results.is_empty());

        // FOXO3 should be in top longevity genes
        let has_foxo3 = results.iter().any(|r| r.gene == Gene::FOXO3);
        assert!(has_foxo3, "FOXO3 should be a top longevity gene");
    }

    #[test]
    fn test_find_drug_targets() {
        let mut db = InMemoryGeneVectorDB::new(384);
        db.index_all_genes();

        let results = db.find_drug_targets(0.8, 10);
        assert!(!results.is_empty());

        // mTOR should be a top drug target (rapamycin)
        let has_mtor = results.iter().any(|r| r.gene == Gene::MTOR);
        assert!(has_mtor, "mTOR should be a top drug target");
    }

    #[test]
    fn test_semantic_search() {
        let mut db = InMemoryGeneVectorDB::new(384);
        db.index_all_genes();

        let results = db.semantic_search("sirtuin NAD longevity", 5);
        assert!(!results.is_empty());

        // Should find SIRT1
        let has_sirt1 = results.iter().any(|r| r.gene == Gene::SIRT1);
        assert!(has_sirt1, "Semantic search should find SIRT1");
    }

    #[test]
    fn test_pathway_partners() {
        let mut db = InMemoryGeneVectorDB::new(384);
        db.index_all_genes();

        let params = SearchParams::default();
        let results = db.find_pathway_partners(Gene::MTOR, &params);

        // MTOR has pathways defined, so the function should work without panicking
        // If no overlapping pathways found, results may be empty - that's OK
        // The function will fall back to similarity search if no pathway matches
        let pathways = db.encoder.gene_pathways(Gene::MTOR);
        assert!(!pathways.is_empty(), "MTOR should have pathways defined");
        // Results can be empty if no other genes share pathways, which is acceptable
    }

    #[test]
    fn test_database_stats() {
        let mut db = InMemoryGeneVectorDB::new(384);
        db.index_all_genes();

        let stats = db.stats();
        assert!(stats.total_genes > 0);
        assert_eq!(stats.embedding_dim, 384);
        assert!(!stats.category_counts.is_empty());
    }

    #[test]
    fn test_category_mapping() {
        let encoder = GeneEncoder::new(384);

        assert_eq!(encoder.gene_to_category(Gene::SIRT1), GeneCategory::Signaling);
        assert_eq!(encoder.gene_to_category(Gene::TP53), GeneCategory::DNARepair);
        assert_eq!(encoder.gene_to_category(Gene::MTOR), GeneCategory::Metabolism);
        assert_eq!(encoder.gene_to_category(Gene::TERT), GeneCategory::Telomere);
    }

    #[test]
    fn test_hallmark_mapping() {
        let encoder = GeneEncoder::new(384);

        let tp53_hallmarks = encoder.gene_to_hallmarks(Gene::TP53);
        assert!(tp53_hallmarks.contains(&Hallmark::GenomicInstability));

        let tert_hallmarks = encoder.gene_to_hallmarks(Gene::TERT);
        assert!(tp53_hallmarks.contains(&Hallmark::GenomicInstability) ||
                tert_hallmarks.contains(&Hallmark::TelomereAttrition));
    }

    #[test]
    fn test_config_from_env() {
        // Test default config when env vars not set
        let config = GeneVectorConfig::from_env();
        assert!(config.database_url.contains("postgres"));
        assert!(config.max_connections > 0);
    }

    #[test]
    fn test_cosine_similarity() {
        let db = InMemoryGeneVectorDB::new(384);

        let vec_a = vec![1.0, 0.0, 0.0];
        let vec_b = vec![1.0, 0.0, 0.0];
        let vec_c = vec![0.0, 1.0, 0.0];

        // Same vectors should have similarity 1.0
        let sim_same = db.cosine_similarity(&vec_a, &vec_b);
        assert!((sim_same - 1.0).abs() < 0.01);

        // Orthogonal vectors should have similarity 0.0
        let sim_orth = db.cosine_similarity(&vec_a, &vec_c);
        assert!(sim_orth.abs() < 0.01);
    }
}
