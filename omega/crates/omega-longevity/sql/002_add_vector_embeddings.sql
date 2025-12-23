-- Migration 002: Add Vector Embeddings for ML-based Gene Discovery
-- Purpose: Enable similarity search for gene discovery using pgvector
-- Date: 2025-12-22
-- Requires: pgvector extension

-- 1. Ensure pgvector extension is enabled
CREATE EXTENSION IF NOT EXISTS vector;

-- 2. Add embedding column to genes table (384 dimensions for bio-embeddings)
ALTER TABLE genes ADD COLUMN IF NOT EXISTS embedding vector(384);

-- 3. Create HNSW index for ultra-fast similarity search
-- HNSW (Hierarchical Navigable Small World) is fastest for high-dim vectors
CREATE INDEX IF NOT EXISTS idx_genes_embedding_hnsw
ON genes USING hnsw (embedding vector_cosine_ops)
WITH (m = 16, ef_construction = 64);

-- Alternative: IVFFlat index (faster build, slightly slower search)
-- CREATE INDEX IF NOT EXISTS idx_genes_embedding_ivfflat
-- ON genes USING ivfflat (embedding vector_cosine_ops)
-- WITH (lists = 100);

-- 4. Create function for batch similarity search
CREATE OR REPLACE FUNCTION find_similar_genes(
    query_embedding vector(384),
    result_limit INT DEFAULT 10,
    min_distance FLOAT DEFAULT NULL,
    gene_type_filter TEXT DEFAULT NULL
)
RETURNS TABLE (
    gene_id TEXT,
    gene_name TEXT,
    gene_type TEXT,
    chromosome TEXT,
    distance FLOAT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        g.gene_id::TEXT,
        g.gene_name::TEXT,
        g.gene_type::TEXT,
        g.chromosome::TEXT,
        (g.embedding <=> query_embedding)::FLOAT as dist
    FROM genes g
    WHERE g.embedding IS NOT NULL
        AND (gene_type_filter IS NULL OR g.gene_type = gene_type_filter)
        AND (min_distance IS NULL OR (g.embedding <=> query_embedding) >= min_distance)
    ORDER BY g.embedding <=> query_embedding
    LIMIT result_limit;
END;
$$ LANGUAGE plpgsql;

-- 5. Create function for gene clustering
CREATE OR REPLACE FUNCTION cluster_genes_by_embedding(
    num_clusters INT DEFAULT 10,
    gene_type_filter TEXT DEFAULT NULL
)
RETURNS TABLE (
    gene_id TEXT,
    gene_name TEXT,
    cluster_id INT,
    distance_to_centroid FLOAT
) AS $$
BEGIN
    -- K-means clustering using embeddings
    -- This is a simplified version - for production, use external ML library
    RETURN QUERY
    WITH centroids AS (
        SELECT
            ntile(num_clusters) OVER (ORDER BY random()) as cluster_id,
            gene_id,
            embedding
        FROM genes
        WHERE embedding IS NOT NULL
            AND (gene_type_filter IS NULL OR gene_type = gene_type_filter)
        LIMIT num_clusters
    )
    SELECT
        g.gene_id::TEXT,
        g.gene_name::TEXT,
        (
            SELECT c.cluster_id
            FROM centroids c
            ORDER BY g.embedding <=> c.embedding
            LIMIT 1
        )::INT as cluster_id,
        (
            SELECT (g.embedding <=> c.embedding)::FLOAT
            FROM centroids c
            ORDER BY g.embedding <=> c.embedding
            LIMIT 1
        ) as distance_to_centroid
    FROM genes g
    WHERE g.embedding IS NOT NULL
        AND (gene_type_filter IS NULL OR g.gene_type = gene_type_filter);
END;
$$ LANGUAGE plpgsql;

-- 6. Create table for storing embedding metadata
CREATE TABLE IF NOT EXISTS embedding_metadata (
    id SERIAL PRIMARY KEY,
    model_name VARCHAR(200) NOT NULL,
    model_version VARCHAR(50),
    embedding_dimension INT NOT NULL,
    training_data TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- 7. Insert default embedding model metadata
INSERT INTO embedding_metadata (model_name, model_version, embedding_dimension, training_data)
VALUES
    ('BioBERT-base', 'v1.1', 384, 'PubMed + PMC'),
    ('PubMedBERT', '2.0', 384, 'PubMed abstracts'),
    ('SapBERT', '1.0', 384, 'UMLS + biomedical entities')
ON CONFLICT DO NOTHING;

-- Usage examples:
--
-- 1. Find genes similar to TP53:
--    SELECT * FROM find_similar_genes(
--        (SELECT embedding FROM genes WHERE gene_name = 'TP53'),
--        10  -- top 10 results
--    );
--
-- 2. Find protein-coding genes similar to query:
--    SELECT * FROM find_similar_genes(
--        query_embedding,
--        20,
--        NULL,
--        'protein_coding'
--    );
--
-- 3. Cluster all protein-coding genes:
--    SELECT * FROM cluster_genes_by_embedding(50, 'protein_coding');

-- Performance notes:
-- - HNSW index provides <100ms search for 100K+ vectors
-- - Cosine distance is standard for text embeddings
-- - Embedding generation should be done offline using ML models
-- - Expected similarity search: 10-50ms for 63K genes
