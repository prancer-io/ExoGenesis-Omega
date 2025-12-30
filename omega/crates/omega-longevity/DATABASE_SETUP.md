# Omega-Longevity Database Setup

## Overview

The omega-longevity crate is now connected to a **ruvector-postgres** Docker container for high-performance vector similarity search and genome-scale data storage.

## Container Details

**Container Name:** `omega-longevity-db`
**Image:** `ruvnet/ruvector-postgres:latest`
**Port:** `5434` (host) → `5432` (container)
**Database:** `omega_longevity`
**User:** `omega`
**Password:** `longevity`

## Extensions Installed

1. **RuVector v0.1.0** - SIMD-optimized vector similarity search
   - Cosine distance, L2 distance, L1 distance, inner product
   - Embedding generation functions
   - Graph neural network support (GCN, GraphSAGE)
   - RDF triple stores
   - Multi-agent coordination
   - Self-learning capabilities

## Configuration Files

### 1. Environment Variables (`.env`)

Location: `/home/farchide/repo/ExoGenesis-Omega/omega/crates/omega-longevity/.env`

```env
RUVECTOR_DATABASE_URL=postgres://omega:longevity@localhost:5434/omega_longevity
RUVECTOR_MAX_CONNECTIONS=20
RUVECTOR_TIMEOUT=30
RUVECTOR_EMBEDDING_DIM=384
RUVECTOR_INDEX_TYPE=HNSW
RUVECTOR_HNSW_M=16
RUVECTOR_HNSW_EF_CONSTRUCTION=64
RUVECTOR_HNSW_EF_SEARCH=40
RUVECTOR_SCHEMA=public
RUVECTOR_BATCH_SIZE=1000
RUVECTOR_ENABLE_SIMD=true
RUST_LOG=info
RUVECTOR_LOG_LEVEL=info
SQLX_LOG=warn
```

### 2. Database Schema (`schema.sql`)

Location: `/home/farchide/repo/ExoGenesis-Omega/omega/crates/omega-longevity/schema.sql`

Contains tables for:
- Gene vector database (384-dim embeddings)
- Genome-scale variant database (150M+ SNPs)
- Biomarker dream session results
- Lifespan simulation results
- VUS interpretations
- Causal patterns
- Research papers
- Intervention rankings
- Aging trajectories

## Quick Start

### 1. Start the Container

```bash
docker start omega-longevity-db
```

### 2. Check Container Status

```bash
docker ps | grep omega-longevity-db
# Should show: "Up X seconds (healthy)"
```

### 3. Verify Database Connection

```bash
docker exec omega-longevity-db psql -U omega -d omega_longevity -c "\dx"
# Should show ruvector extension
```

### 4. Run Tests

```bash
cd /home/farchide/repo/ExoGenesis-Omega/omega/crates/omega-longevity
export RUVECTOR_DATABASE_URL="postgres://omega:longevity@localhost:5434/omega_longevity"
cargo test --test database_connection_test --features vector-db -- --nocapture
```

**Expected Output:**
```
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Container Management

### Stop the Container

```bash
docker stop omega-longevity-db
```

### Restart the Container

```bash
docker restart omega-longevity-db
```

### View Logs

```bash
docker logs omega-longevity-db -f
```

### Access PostgreSQL Shell

```bash
docker exec -it omega-longevity-db psql -U omega -d omega_longevity
```

### Backup Database

```bash
docker exec omega-longevity-db pg_dump -U omega omega_longevity > backup_$(date +%Y%m%d).sql
```

### Restore Database

```bash
cat backup_YYYYMMDD.sql | docker exec -i omega-longevity-db psql -U omega -d omega_longevity
```

## RuVector Features

### Vector Similarity Search

```sql
-- Cosine distance between vectors
SELECT ruvector_cosine_distance(vec1, vec2) FROM table_name;

-- L2 (Euclidean) distance
SELECT ruvector_l2_distance(vec1, vec2) FROM table_name;

-- Inner product
SELECT ruvector_inner_product(vec1, vec2) FROM table_name;
```

### Embedding Generation

```sql
-- Generate embedding from text
SELECT ruvector_embed('gene description text');

-- Batch embeddings
SELECT ruvector_embed_batch(ARRAY['text1', 'text2', 'text3']);

-- Check available models
SELECT * FROM ruvector_embedding_models();
```

### Graph Neural Networks

```sql
-- GCN forward pass
SELECT ruvector_gcn_forward(features, src, dst, weights, out_dim);

-- GraphSAGE forward pass
SELECT ruvector_graphsage_forward(features, src, dst, out_dim, sample_size);
```

### Self-Learning

```sql
-- Enable learning on a table
SELECT ruvector_enable_learning('table_name', '{"adaptation_rate": 0.01}'::jsonb);

-- Get learning statistics
SELECT ruvector_learning_stats('table_name');

-- Extract patterns
SELECT ruvector_extract_patterns('table_name', 10);
```

## Performance Tuning

### HNSW Index Parameters

- **m**: Number of bi-directional links (default: 16)
  - Higher = better recall, more memory
  - Range: 4-64

- **ef_construction**: Construction time candidates (default: 64)
  - Higher = better index quality, slower build
  - Range: 16-512

- **ef_search**: Search time candidates (default: 40)
  - Higher = better recall, slower search
  - Range: 10-512

### Connection Pool

- **max_connections**: Maximum pool size (default: 20)
- **timeout**: Connection timeout in seconds (default: 30)

### SIMD Acceleration

RuVector uses SIMD instructions for 4-8x faster distance calculations:
- AVX2 (Intel/AMD)
- AVX-512 (newer Intel)
- NEON (ARM)

## Integration with Omega-Longevity

The vector database powers these modules:

1. **Gene Vector Database** (`gene_vector_db.rs`)
   - 49 aging genes with 384-dim embeddings
   - Ultra-fast similarity search
   - Pathway partner discovery

2. **Genome-Scale Database** (`genome_scale_db.rs`)
   - 150M+ SNP variants
   - Chromosome partitioning
   - Vector compression (50-87.5% reduction)

3. **Biomarker Dreamer** (`biomarker_dreamer.rs`)
   - Novel target discovery storage
   - Dream session results

4. **Research Integrator** (`research_integrator.rs`)
   - Literature synthesis
   - Intervention rankings

## Troubleshooting

### Container Won't Start

```bash
# Check if port 5434 is in use
lsof -i :5434

# Remove and recreate container
docker rm omega-longevity-db
docker run -d --name omega-longevity-db \
  -e POSTGRES_PASSWORD=longevity \
  -e POSTGRES_DB=omega_longevity \
  -e POSTGRES_USER=omega \
  -p 5434:5432 \
  --health-cmd="pg_isready -U omega" \
  --health-interval=10s \
  ruvnet/ruvector-postgres:latest
```

### Connection Refused

```bash
# Verify container is healthy
docker inspect omega-longevity-db | grep -i health

# Restart container
docker restart omega-longevity-db && sleep 10

# Test connection
docker exec omega-longevity-db pg_isready -U omega
```

### Extension Not Found

```bash
# Reinstall extensions
docker exec omega-longevity-db psql -U omega -d omega_longevity -c "
  DROP EXTENSION IF EXISTS ruvector CASCADE;
  CREATE EXTENSION ruvector;
"
```

## Additional Resources

- RuVector Documentation: https://hub.docker.com/r/ruvnet/ruvector-postgres
- PostgreSQL Documentation: https://www.postgresql.org/docs/
- HNSW Algorithm: https://arxiv.org/abs/1603.09320
- Vector Similarity Search: https://www.pinecone.io/learn/vector-similarity/

## Status

✅ **Container Running:** omega-longevity-db (healthy)
✅ **Extension Installed:** RuVector v0.1.0
✅ **Database Created:** omega_longevity
✅ **Tests Passing:** 3/3 (100%)
✅ **Connection Verified:** localhost:5434

**Last Updated:** 2025-12-22
