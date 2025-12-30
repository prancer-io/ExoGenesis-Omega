-- Omega-Longevity Database Schema
-- PostgreSQL with pgvector and ruvector extensions

-- Ensure extensions are enabled
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS ruvector;

-- Gene Vector Database Tables
-- Stores gene embeddings for similarity search

CREATE TABLE IF NOT EXISTS genes (
    id SERIAL PRIMARY KEY,
    gene_name VARCHAR(50) NOT NULL UNIQUE,
    gene_category VARCHAR(50) NOT NULL,
    description TEXT,
    embedding vector(384) NOT NULL,  -- 384-dimensional embeddings
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create HNSW index for fast similarity search
CREATE INDEX IF NOT EXISTS genes_embedding_hnsw_idx
ON genes USING hnsw (embedding vector_cosine_ops)
WITH (m = 16, ef_construction = 64);

-- Alternative: IVFFlat index (uncomment if preferred)
-- CREATE INDEX IF NOT EXISTS genes_embedding_ivfflat_idx
-- ON genes USING ivfflat (embedding vector_cosine_ops)
-- WITH (lists = 100);

-- Gene metadata table
CREATE TABLE IF NOT EXISTS gene_metadata (
    gene_id INTEGER PRIMARY KEY REFERENCES genes(id) ON DELETE CASCADE,
    chromosome VARCHAR(10),
    start_position BIGINT,
    end_position BIGINT,
    strand CHAR(1),
    biotype VARCHAR(50),
    hallmarks TEXT[],  -- Array of aging hallmarks
    pathways TEXT[],   -- Biological pathways
    interactions TEXT[], -- Gene interactions
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Genome-Scale Variant Database Tables
-- Stores 150M+ SNP variants with vector embeddings

CREATE TABLE IF NOT EXISTS variants (
    id BIGSERIAL PRIMARY KEY,
    rsid VARCHAR(20) UNIQUE,
    chromosome VARCHAR(10) NOT NULL,
    position BIGINT NOT NULL,
    ref_allele VARCHAR(1000),
    alt_allele VARCHAR(1000),
    variant_type VARCHAR(50),  -- SNP, INDEL, CNV, etc.
    embedding vector(384),     -- Variant functional embedding
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Partition variants by chromosome for scalability
CREATE TABLE IF NOT EXISTS variants_chr1 PARTITION OF variants
FOR VALUES IN ('1');

CREATE TABLE IF NOT EXISTS variants_chr2 PARTITION OF variants
FOR VALUES IN ('2');

-- ... (additional chromosome partitions can be created as needed)

CREATE TABLE IF NOT EXISTS variants_chrX PARTITION OF variants
FOR VALUES IN ('X');

CREATE TABLE IF NOT EXISTS variants_chrY PARTITION OF variants
FOR VALUES IN ('Y');

-- Variant annotations
CREATE TABLE IF NOT EXISTS variant_annotations (
    variant_id BIGINT PRIMARY KEY REFERENCES variants(id) ON DELETE CASCADE,
    gene_id INTEGER REFERENCES genes(id),
    consequence VARCHAR(100),  -- missense, synonymous, frameshift, etc.
    impact VARCHAR(20),        -- HIGH, MODERATE, LOW, MODIFIER
    sift_score FLOAT,
    polyphen_score FLOAT,
    cadd_score FLOAT,
    gnomad_af FLOAT,           -- Allele frequency in gnomAD
    clinvar_significance VARCHAR(100),
    longevity_effect FLOAT,    -- Estimated effect on lifespan (-1 to +1)
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Biomarker Dream Session Results
CREATE TABLE IF NOT EXISTS biomarker_dreams (
    id SERIAL PRIMARY KEY,
    session_id UUID NOT NULL,
    dream_cycle INTEGER NOT NULL,
    problem TEXT NOT NULL,
    insight_type VARCHAR(50),
    confidence FLOAT NOT NULL,
    target_name VARCHAR(255),
    mechanism TEXT,
    therapeutic_potential FLOAT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Lifespan Simulation Results
CREATE TABLE IF NOT EXISTS simulation_results (
    id SERIAL PRIMARY KEY,
    simulation_id UUID NOT NULL,
    protocol_name VARCHAR(255) NOT NULL,
    mean_extension FLOAT,
    median_extension FLOAT,
    percentile_5 FLOAT,
    percentile_95 FLOAT,
    prob_centenarian FLOAT,
    convergence_score FLOAT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- VUS Interpretation Results (Clinical Genomics)
CREATE TABLE IF NOT EXISTS vus_interpretations (
    id SERIAL PRIMARY KEY,
    variant_id BIGINT REFERENCES variants(id),
    query_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    classification VARCHAR(50),  -- Pathogenic, Likely Pathogenic, VUS, etc.
    confidence FLOAT,
    lifespan_effect FLOAT,
    disease_risk_increase FLOAT,
    mechanism TEXT,
    recommendations TEXT[],
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Population Simulation Causal Patterns
CREATE TABLE IF NOT EXISTS causal_patterns (
    id SERIAL PRIMARY KEY,
    cause_hallmark VARCHAR(100) NOT NULL,
    effect_hallmark VARCHAR(100) NOT NULL,
    strength FLOAT NOT NULL,
    timing_years FLOAT,
    observed_percent FLOAT,
    mechanism TEXT,
    population_size INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Research Paper Integration
CREATE TABLE IF NOT EXISTS research_papers (
    id SERIAL PRIMARY KEY,
    doi VARCHAR(255) UNIQUE,
    title TEXT NOT NULL,
    authors TEXT[],
    year INTEGER,
    journal VARCHAR(255),
    interventions TEXT[],
    organisms TEXT[],
    evidence_quality VARCHAR(50),
    indexed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Intervention Rankings
CREATE TABLE IF NOT EXISTS intervention_rankings (
    id SERIAL PRIMARY KEY,
    intervention_name VARCHAR(255) NOT NULL UNIQUE,
    rank INTEGER,
    evidence_score FLOAT,
    effect_on_lifespan FLOAT,
    effect_on_healthspan FLOAT,
    num_supporting_papers INTEGER,
    safety_profile TEXT,
    contraindications TEXT[],
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Attractor Landscape Trajectories
CREATE TABLE IF NOT EXISTS aging_trajectories (
    id SERIAL PRIMARY KEY,
    individual_id UUID NOT NULL,
    age FLOAT NOT NULL,
    basin_state VARCHAR(50),  -- Regenerative, Compensated, Decompensated
    biological_age FLOAT,
    epigenetic_entropy FLOAT,
    network_criticality FLOAT,
    hallmark_states JSONB,    -- JSON object with all hallmark values
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_genes_category ON genes(gene_category);
CREATE INDEX IF NOT EXISTS idx_variants_rsid ON variants(rsid);
CREATE INDEX IF NOT EXISTS idx_variants_chr_pos ON variants(chromosome, position);
CREATE INDEX IF NOT EXISTS idx_biomarker_dreams_session ON biomarker_dreams(session_id);
CREATE INDEX IF NOT EXISTS idx_simulation_protocol ON simulation_results(protocol_name);
CREATE INDEX IF NOT EXISTS idx_vus_variant ON vus_interpretations(variant_id);
CREATE INDEX IF NOT EXISTS idx_causal_patterns_cause ON causal_patterns(cause_hallmark);
CREATE INDEX IF NOT EXISTS idx_research_papers_doi ON research_papers(doi);
CREATE INDEX IF NOT EXISTS idx_trajectories_individual ON aging_trajectories(individual_id, age);

-- Views for common queries
CREATE OR REPLACE VIEW gene_summary AS
SELECT
    g.gene_name,
    g.gene_category,
    g.description,
    COUNT(DISTINCT gm.hallmarks) as num_hallmarks,
    COUNT(DISTINCT gm.pathways) as num_pathways
FROM genes g
LEFT JOIN gene_metadata gm ON g.id = gm.gene_id
GROUP BY g.id, g.gene_name, g.gene_category, g.description;

CREATE OR REPLACE VIEW top_interventions AS
SELECT
    intervention_name,
    rank,
    evidence_score,
    effect_on_lifespan,
    num_supporting_papers
FROM intervention_rankings
ORDER BY rank
LIMIT 50;

-- Function to update timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for automatic timestamp updates
CREATE TRIGGER update_genes_updated_at BEFORE UPDATE ON genes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_gene_metadata_updated_at BEFORE UPDATE ON gene_metadata
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_variant_annotations_updated_at BEFORE UPDATE ON variant_annotations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Grant permissions (adjust as needed)
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO omega;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO omega;

-- Done!
SELECT 'Schema initialized successfully!' as status;
