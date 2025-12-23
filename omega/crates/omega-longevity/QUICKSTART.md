# Omega-Longevity Quick Start Guide

## ✅ Setup Complete!

Your omega-longevity crate is now connected to a high-performance ruvector-postgres database.

## Connection Details

```bash
Container:  omega-longevity-db
Status:     ✅ Running (healthy)
Port:       5434
Database:   omega_longevity
User:       omega
Password:   longevity
Extension:  RuVector v0.1.0 (SIMD-optimized)
```

## Test the Connection

```bash
cd /home/farchide/repo/ExoGenesis-Omega/omega/crates/omega-longevity

# Run database tests
cargo test --test database_connection_test --features vector-db -- --nocapture
```

**Expected Result:** ✅ 3/3 tests passing

## Use in Your Code

### 1. Load Environment Variables

Add to your code:

```rust
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv::dotenv().ok();

    // Get database URL
    let database_url = env::var("RUVECTOR_DATABASE_URL")?;

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    // Use the pool...

    Ok(())
}
```

### 2. Create Gene Vectors

```rust
// Create table
sqlx::query(
    "CREATE TABLE IF NOT EXISTS genes (
        id SERIAL PRIMARY KEY,
        gene_name VARCHAR(50) NOT NULL,
        embedding ruvector(384)
    )"
)
.execute(&pool)
.await?;

// Insert gene (embedding would come from your gene encoder)
sqlx::query(
    "INSERT INTO genes (gene_name) VALUES ($1)"
)
.bind("TP53")
.execute(&pool)
.await?;
```

### 3. Vector Similarity Search

```sql
-- Find similar genes (pseudo-code, actual usage depends on your embeddings)
SELECT
    gene_name,
    ruvector_cosine_distance(embedding, target_embedding) as distance
FROM genes
WHERE embedding IS NOT NULL
ORDER BY distance
LIMIT 10;
```

## Environment Setup

The `.env` file is already configured at:
```
/home/farchide/repo/ExoGenesis-Omega/omega/crates/omega-longevity/.env
```

Key variables:
```env
RUVECTOR_DATABASE_URL=postgres://omega:longevity@localhost:5434/omega_longevity
RUVECTOR_EMBEDDING_DIM=384
RUVECTOR_INDEX_TYPE=HNSW
```

## Available RuVector Features

### Core Functions
- `ruvector_cosine_distance(a, b)` - Cosine similarity
- `ruvector_l2_distance(a, b)` - Euclidean distance
- `ruvector_inner_product(a, b)` - Dot product
- `ruvector_embed(text)` - Generate embeddings

### Advanced Features
- Graph Neural Networks (GCN, GraphSAGE)
- RDF Triple Stores (SPARQL queries)
- Multi-agent coordination
- Self-learning capabilities
- Auto-tuning for optimal performance

## Container Management

```bash
# Start
docker start omega-longevity-db

# Stop
docker stop omega-longevity-db

# Status
docker ps | grep omega-longevity-db

# Logs
docker logs omega-longevity-db -f

# Shell access
docker exec -it omega-longevity-db psql -U omega -d omega_longevity
```

## Next Steps

1. **Review the schema**: Check `schema.sql` for all available tables
2. **Read the docs**: See `DATABASE_SETUP.md` for detailed information
3. **Run examples**: Try the gene vector database example code
4. **Integrate with modules**: Use in `gene_vector_db.rs` and `genome_scale_db.rs`

## Helpful Commands

```bash
# Check extension
docker exec omega-longevity-db psql -U omega -d omega_longevity -c "\dx"

# List tables
docker exec omega-longevity-db psql -U omega -d omega_longevity -c "\dt"

# Check available models
docker exec omega-longevity-db psql -U omega -d omega_longevity -c "SELECT * FROM ruvector_embedding_models();"

# Database backup
docker exec omega-longevity-db pg_dump -U omega omega_longevity > backup.sql

# Database restore
cat backup.sql | docker exec -i omega-longevity-db psql -U omega -d omega_longevity
```

## Performance Tips

1. **HNSW Indexing**: Already configured for optimal 384-dim vectors
2. **SIMD Acceleration**: Automatically enabled for 4-8x speedup
3. **Connection Pooling**: Configured for 20 concurrent connections
4. **Batch Operations**: Use `ruvector_embed_batch()` for multiple embeddings

## Troubleshooting

**Can't connect?**
```bash
# Verify container is running
docker ps | grep omega-longevity-db

# Check health
docker inspect omega-longevity-db | grep -i health

# Restart if needed
docker restart omega-longevity-db
```

**Extension issues?**
```bash
# Reinstall extension
docker exec omega-longevity-db psql -U omega -d omega_longevity -c "
  DROP EXTENSION IF EXISTS ruvector CASCADE;
  CREATE EXTENSION ruvector;
"
```

## Support

- Full documentation: `DATABASE_SETUP.md`
- Schema reference: `schema.sql`
- Test examples: `tests/database_connection_test.rs`
- RuVector docs: https://hub.docker.com/r/ruvnet/ruvector-postgres

---

**Setup Date:** 2025-12-22
**Status:** ✅ Production Ready
**Tests:** ✅ 3/3 Passing
