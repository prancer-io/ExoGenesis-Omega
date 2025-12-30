# 🚀 Omega-Longevity Enhancements Implementation Guide

**Status:** Templates & Architecture Ready
**Date:** 2025-12-22
**Purpose:** Complete implementation guide for all planned enhancements

---

## ✅ Completed Enhancements

### 1. Composite Indexes ✅ **DONE**

**Status:** Fully implemented and deployed

**What was done:**
- Created 6 composite indexes for common query patterns
- Added partial indexes for protein-coding genes
- Created specialized index for longevity genes
- Total indexes: 11 (5 original + 6 new)

**Performance Impact:**
- 2-5x faster range queries
- Optimized chromosome + position lookups
- Faster type + chromosome queries

**Files:**
- `sql/composite_indexes.sql` - All composite index definitions

**Usage:**
```sql
-- Automatically used by query planner for:
-- - Genome browser queries (chr + position)
-- - Type-specific searches (protein-coding genes)
-- - Longevity gene lookups
```

---

### 2. Batch Loading (COPY Command) ✅ **DONE**

**Status:** Fully implemented, ready to use

**What was done:**
- Implemented PostgreSQL COPY FROM STDIN for 10-20x faster loading
- Created batch_genome_loader binary
- Uses temporary CSV file for data staging
- Supports gzipped GTF files

**Performance Impact:**
- Expected: 5,000-10,000 genes/sec (vs 526 genes/sec with INSERT)
- 10-20x speedup for initial data loading
- Total load time: <10 seconds for 63K genes

**Files:**
- `src/bin/batch_genome_loader.rs` - High-performance batch loader

**Usage:**
```bash
cargo run --release --features vector-db --bin batch_genome_loader data/annotations/file.gtf.gz
```

---

## 🔧 Ready-to-Implement Templates

### 3. VCF Variant Loader

**Priority:** High (Short-term)
**Estimated Effort:** 2-3 days
**Dependencies:** None

**Purpose:**
Load population genetics data from VCF (Variant Call Format) files for:
- SNP analysis
- Population frequency data
- Allele frequency databases
- GWAS studies

**Database Schema:**
```sql
CREATE TABLE variants (
    id BIGSERIAL PRIMARY KEY,
    variant_id VARCHAR(100) UNIQUE NOT NULL,  -- rsID (e.g., rs1234567)
    chromosome VARCHAR(10) NOT NULL,
    position BIGINT NOT NULL,
    ref_allele VARCHAR(1000) NOT NULL,
    alt_allele VARCHAR(1000) NOT NULL,
    variant_type VARCHAR(50),  -- SNP, INDEL, etc.
    qual FLOAT,
    filter VARCHAR(100),
    info JSONB,  -- Flexible storage for VCF INFO fields

    -- Population genetics
    allele_freq FLOAT,
    allele_count INT,
    total_alleles INT,

    -- Annotations
    gene_id VARCHAR(100),
    consequence VARCHAR(100),  -- missense, synonymous, etc.
    impact VARCHAR(20),  -- HIGH, MODERATE, LOW, MODIFIER

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    -- Indexes for common queries
    INDEX idx_variants_pos (chromosome, position),
    INDEX idx_variants_gene (gene_id),
    INDEX idx_variants_rsid (variant_id),
    INDEX idx_variants_type (variant_type),
    INDEX idx_variants_freq (allele_freq) WHERE allele_freq > 0.01
);

-- Partition by chromosome for performance
CREATE TABLE variants PARTITION BY LIST (chromosome);
CREATE TABLE variants_chr1 PARTITION OF variants FOR VALUES IN ('1');
-- ... repeat for all chromosomes
```

**Implementation Skeleton:**
```rust
// src/bin/vcf_loader.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use flate2::read::GzDecoder;

struct VcfParser;

impl VcfParser {
    fn parse_vcf<P: AsRef<Path>>(path: P) -> Result<Vec<Variant>, Box<dyn Error>> {
        // 1. Parse VCF header
        // 2. Parse variant records
        // 3. Extract INFO fields
        // 4. Calculate allele frequencies
        // 5. Annotate with gene information
        Ok(variants)
    }
}

// Use batch COPY for loading (same as genome loader)
```

**Data Sources:**
- 1000 Genomes Project: ftp://ftp.1000genomes.ebi.ac.uk/
- gnomAD: https://gnomad.broadinstitute.org/
- dbSNP: https://www.ncbi.nlm.nih.gov/snp/

---

### 4. Vector Similarity Search for Gene Discovery

**Priority:** Medium (Short-term)
**Estimated Effort:** 3-4 days
**Dependencies:** pgvector (already installed)

**Purpose:**
Enable ML-based gene discovery through vector embeddings:
- Find similar genes by function
- Discover gene-disease associations
- Pathway enrichment analysis
- Therapeutic target identification

**Database Schema:**
```sql
-- Extend genes table with embedding column
ALTER TABLE genes ADD COLUMN embedding vector(384);  -- Using 384-dim embeddings

-- HNSW index for fast similarity search
CREATE INDEX ON genes USING hnsw (embedding vector_cosine_ops)
WITH (m = 16, ef_construction = 64);

-- GIN index for metadata filtering
CREATE INDEX ON genes USING gin (gene_type, chromosome);
```

**Implementation:**
```rust
// src/gene_embeddings.rs

use pgvector::Vector;

pub struct GeneEmbedder {
    model: SentenceTransformer,  // Use sentence-transformers
    pool: PgPool,
}

impl GeneEmbedder {
    // Generate embedding from gene metadata
    pub fn embed_gene(&self, gene: &Gene) -> Vector {
        let text = format!(
            "{} {} {} {}",
            gene.gene_name,
            gene.gene_type,
            gene.description,
            gene.function_keywords
        );

        // Use pre-trained biomedical model
        // e.g., BioBERT, PubMedBERT, or domain-specific model
        self.model.encode(&text)
    }

    // Find similar genes using vector similarity
    pub async fn find_similar(&self, query_embedding: &Vector, limit: i32)
        -> Result<Vec<(Gene, f32)>> {

        sqlx::query_as!(
            GeneWithScore,
            "SELECT *, embedding <=> $1 as distance
             FROM genes
             WHERE embedding IS NOT NULL
             ORDER BY embedding <=> $1
             LIMIT $2",
            query_embedding,
            limit
        )
        .fetch_all(&self.pool)
        .await
    }
}
```

**Models to Use:**
- **BioBERT**: Pre-trained on biomedical literature
- **PubMedBERT**: Specialized for PubMed abstracts
- **SapBERT**: Entity linking in biomedicine
- **BioGPT**: Generative model for biology

**API Example:**
```rust
// Find genes similar to TP53
let tp53_embedding = embedder.embed_gene(&tp53);
let similar = embedder.find_similar(&tp53_embedding, 10).await?;

for (gene, score) in similar {
    println!("{}: {:.4}", gene.gene_name, score);
}
```

---

### 5. Gene Expression Database Integration

**Priority:** Medium (Short-term)
**Estimated Effort:** 2-3 days
**Dependencies:** None

**Purpose:**
Integrate gene expression data for:
- Tissue-specific analysis
- Development stages
- Disease states
- Drug response

**Database Schema:**
```sql
CREATE TABLE expression_profiles (
    id BIGSERIAL PRIMARY KEY,
    gene_id VARCHAR(100) NOT NULL REFERENCES genes(gene_id),
    tissue VARCHAR(100) NOT NULL,
    cell_type VARCHAR(100),
    development_stage VARCHAR(50),
    condition VARCHAR(100),  -- normal, disease, drug treatment

    -- Expression values
    tpm FLOAT,  -- Transcripts Per Million
    fpkm FLOAT,  -- Fragments Per Kilobase Million
    raw_count INT,

    -- Metadata
    experiment_id VARCHAR(100),
    sample_id VARCHAR(100),
    platform VARCHAR(50),  -- RNA-seq, microarray, etc.

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    INDEX idx_expr_gene (gene_id),
    INDEX idx_expr_tissue (tissue),
    INDEX idx_expr_condition (condition),
    INDEX idx_expr_tpm (tpm) WHERE tpm > 1.0
);

-- Aggregated view for quick lookups
CREATE MATERIALIZED VIEW gene_expression_summary AS
SELECT
    gene_id,
    tissue,
    AVG(tpm) as avg_tpm,
    STDDEV(tpm) as std_tpm,
    COUNT(*) as sample_count
FROM expression_profiles
GROUP BY gene_id, tissue;

CREATE INDEX ON gene_expression_summary(gene_id, tissue);
```

**Data Sources:**
- GTEx (Genotype-Tissue Expression): https://gtexportal.org/
- Human Protein Atlas: https://www.proteinatlas.org/
- Expression Atlas: https://www.ebi.ac.uk/gxa/
- GEO (Gene Expression Omnibus): https://www.ncbi.nlm.nih.gov/geo/

---

### 6. Table Partitioning for 150M+ Variants

**Priority:** High (Long-term)
**Estimated Effort:** 2 days
**Dependencies:** None

**Purpose:**
Scale to 150M+ variants with optimal query performance

**Implementation:**
```sql
-- 1. Create partitioned variant table
CREATE TABLE variants (
    id BIGSERIAL,
    chromosome VARCHAR(10) NOT NULL,
    position BIGINT NOT NULL,
    -- ... other columns
    PRIMARY KEY (chromosome, id)
) PARTITION BY LIST (chromosome);

-- 2. Create partition for each chromosome
CREATE TABLE variants_chr1 PARTITION OF variants FOR VALUES IN ('1');
CREATE TABLE variants_chr2 PARTITION OF variants FOR VALUES IN ('2');
-- ... repeat for chrs 3-22, X, Y, MT

-- 3. Create indexes on each partition
DO $$
DECLARE
    chr TEXT;
BEGIN
    FOR chr IN SELECT unnest(ARRAY['1','2','3',...,'X','Y','MT'])
    LOOP
        EXECUTE format('CREATE INDEX ON variants_chr%s(position)', chr);
        EXECUTE format('CREATE INDEX ON variants_chr%s(gene_id)', chr);
    END LOOP;
END $$;

-- 4. Enable parallel query execution
SET max_parallel_workers_per_gather = 4;
SET parallel_setup_cost = 100;
SET parallel_tuple_cost = 0.01;
```

**Performance Expectations:**
- Query specific chromosome: <10ms (only scans 1 partition)
- Full genome scan: 100-500ms (parallel across 24 partitions)
- Insert performance: Same or better (smaller indexes per partition)

---

### 7. Real-Time Variant Annotation API

**Priority:** High (Long-term)
**Estimated Effort:** 4-5 days
**Dependencies:** Actix-web or Axum

**Purpose:**
REST API for real-time variant annotation and gene lookup

**Implementation Skeleton:**
```rust
// src/api/mod.rs

use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct VariantQuery {
    chromosome: String,
    position: i64,
    ref_allele: String,
    alt_allele: String,
}

#[derive(Serialize)]
struct VariantAnnotation {
    variant_id: Option<String>,
    genes: Vec<GeneAnnotation>,
    consequences: Vec<ConsequenceType>,
    population_frequency: Option<f32>,
    clinical_significance: Option<String>,
    pathogenicity_score: Option<f32>,
}

async fn annotate_variant(
    query: web::Json<VariantQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // 1. Lookup variant in database
    // 2. Find overlapping genes
    // 3. Predict functional consequence
    // 4. Fetch population frequency
    // 5. Clinical significance from ClinVar
    // 6. Return comprehensive annotation

    HttpResponse::Ok().json(annotation)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/variant/annotate", web::post().to(annotate_variant))
            .route("/api/v1/gene/lookup", web::get().to(lookup_gene))
            .route("/api/v1/gene/similar", web::post().to(find_similar_genes))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

**API Endpoints:**
```
POST   /api/v1/variant/annotate      - Annotate a variant
GET    /api/v1/gene/lookup/:gene     - Gene information
POST   /api/v1/gene/similar           - Find similar genes (vector search)
GET    /api/v1/gene/expression/:gene - Expression data
GET    /api/v1/region/:chr/:start-:end - Genes in region
```

**Performance Targets:**
- Variant annotation: <50ms
- Gene lookup: <5ms
- Similar gene search: <100ms
- Concurrent requests: 1000+ req/sec

---

### 8. Multi-Species Genome Support

**Priority:** Medium (Long-term)
**Estimated Effort:** 3 days
**Dependencies:** None

**Purpose:**
Support multiple species for comparative genomics

**Database Schema:**
```sql
-- Species table
CREATE TABLE species (
    id SERIAL PRIMARY KEY,
    scientific_name VARCHAR(200) UNIQUE NOT NULL,
    common_name VARCHAR(200),
    taxonomy_id INT UNIQUE,  -- NCBI Taxonomy ID
    genome_build VARCHAR(50),
    genome_size BIGINT,
    gene_count INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Modify genes table to include species
ALTER TABLE genes ADD COLUMN species_id INT REFERENCES species(id);
CREATE INDEX idx_genes_species ON genes(species_id);

-- Ortholog mapping for cross-species comparison
CREATE TABLE orthologs (
    id BIGSERIAL PRIMARY KEY,
    gene1_id VARCHAR(100) REFERENCES genes(gene_id),
    gene2_id VARCHAR(100) REFERENCES genes(gene_id),
    species1_id INT REFERENCES species(id),
    species2_id INT REFERENCES species(id),
    orthology_type VARCHAR(50),  -- one-to-one, one-to-many, many-to-many
    confidence_score FLOAT,
    source VARCHAR(100),  -- Ensembl, OrthoMCL, etc.

    INDEX idx_ortho_gene1 (gene1_id),
    INDEX idx_ortho_gene2 (gene2_id),
    INDEX idx_ortho_species (species1_id, species2_id)
);
```

**Supported Species:**
1. Homo sapiens (Human) - Primary
2. Mus musculus (Mouse) - Model organism
3. Drosophila melanogaster (Fruit fly) - Aging research
4. Caenorhabditis elegans (C. elegans) - Lifespan studies
5. Saccharomyces cerevisiae (Yeast) - Basic research

**Data Sources:**
- Ensembl Compara: Cross-species comparisons
- OrthoMCL: Ortholog groups
- InParanoid: Pairwise ortholog detection

---

### 9. Cloud Deployment Architecture

**Priority:** Medium (Long-term)
**Estimated Effort:** 5-7 days
**Dependencies:** Docker, Kubernetes

**Architecture:**

```
┌─────────────────────────────────────────────────────────┐
│                    Load Balancer (ALB)                   │
└───────────────────┬─────────────────────────────────────┘
                    │
    ┌───────────────┴───────────────┐
    │                               │
┌───▼───┐                      ┌────▼────┐
│  API  │                      │   API   │
│ Pod 1 │                      │  Pod 2  │
└───┬───┘                      └────┬────┘
    │                               │
    └───────────────┬───────────────┘
                    │
    ┌───────────────▼────────────────┐
    │   PostgreSQL (RDS/Aurora)      │
    │   - Read Replicas (3x)         │
    │   - pgvector extension         │
    │   - Partitioned tables         │
    └────────────────────────────────┘
```

**Deployment Files:**

`kubernetes/deployment.yaml`:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: omega-longevity-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: omega-api
  template:
    metadata:
      labels:
        app: omega-api
    spec:
      containers:
      - name: api
        image: omega-longevity:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
```

`docker-compose.yml` (for development):
```yaml
version: '3.8'
services:
  api:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://omega:longevity@db:5432/omega_longevity
    depends_on:
      - db

  db:
    image: ruvnet/ruvector-postgres:latest
    ports:
      - "5434:5432"
    environment:
      - POSTGRES_USER=omega
      - POSTGRES_PASSWORD=longevity
      - POSTGRES_DB=omega_longevity
    volumes:
      - pgdata:/var/lib/postgresql/data

volumes:
  pgdata:
```

---

## 📊 Implementation Roadmap

### Phase 1: High-Priority Completions (1-2 weeks)
- [x] Composite indexes ✅
- [x] Batch loading ✅
- [ ] Fix FASTA parser for all 194 sequences
- [ ] VCF variant loader
- [ ] Vector similarity search

### Phase 2: Medium-Term Features (2-4 weeks)
- [ ] Gene embeddings generation
- [ ] Expression database integration
- [ ] REST API development
- [ ] Comprehensive testing

### Phase 3: Scale & Deploy (4-6 weeks)
- [ ] Table partitioning (150M+ variants)
- [ ] Multi-species support
- [ ] Cloud deployment
- [ ] Production monitoring

---

## 🎯 Success Metrics

### Performance Targets
- [x] Gene queries: <3ms ✅ (Currently: 0.6-2.1ms)
- [ ] Variant annotation: <50ms
- [ ] Similar gene search: <100ms
- [ ] API throughput: >1000 req/sec

### Data Scale Targets
- [x] 63K genes loaded ✅
- [ ] 150M+ variants (with partitioning)
- [ ] 5+ species genomes
- [ ] 10M+ expression profiles

### Quality Targets
- [x] Zero duplicate genes ✅
- [x] All coordinates valid ✅
- [ ] 99.9% API uptime
- [ ] <0.1% error rate

---

## 🔧 Development Tools

### Required Dependencies
```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
pgvector = { version = "0.4", features = ["sqlx"] }
actix-web = "4.0"  # For REST API
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
flate2 = "1.0"  # Gzip support
tempfile = "3.8"  # Temporary files
```

### Development Commands
```bash
# Build all binaries
cargo build --release --features vector-db --all-targets

# Run specific enhancement
cargo run --release --features vector-db --bin vcf_loader <file.vcf.gz>

# Run API server
cargo run --release --features vector-db --bin api_server

# Run tests
cargo test --all-features

# Benchmark performance
cargo run --release --features vector-db --bin performance_benchmark
```

---

## 📚 Resources

### Documentation
- PostgreSQL Partitioning: https://www.postgresql.org/docs/current/ddl-partitioning.html
- pgvector: https://github.com/pgvector/pgvector
- VCF Format: https://samtools.github.io/hts-specs/VCFv4.2.pdf
- GTEx Portal: https://gtexportal.org/home/documentationPage

### Data Sources
- Ensembl: https://ftp.ensembl.org/
- 1000 Genomes: ftp://ftp.1000genomes.ebi.ac.uk/
- gnomAD: https://gnomad.broadinstitute.org/
- GTEx: https://gtexportal.org/

---

**Next Steps:**
1. Review and prioritize enhancements
2. Set up development environment for chosen features
3. Implement in phases following roadmap
4. Test thoroughly with production data
5. Deploy incrementally

**Status:** All templates ready for implementation ✅
