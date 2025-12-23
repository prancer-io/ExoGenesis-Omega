-- Migration 001: Create Variants Table with Partitioning
-- Purpose: Support 150M+ variants with optimal query performance
-- Date: 2025-12-22

-- 1. Create partitioned variants table
CREATE TABLE IF NOT EXISTS variants (
    id BIGSERIAL,
    variant_id VARCHAR(100),  -- rsID (e.g., rs1234567)
    chromosome VARCHAR(10) NOT NULL,
    position BIGINT NOT NULL,
    ref_allele VARCHAR(1000) NOT NULL,
    alt_allele VARCHAR(1000) NOT NULL,
    variant_type VARCHAR(50),  -- SNP, INDEL, CNV, etc.

    -- Quality metrics
    qual FLOAT,
    filter VARCHAR(100),
    info JSONB,  -- Flexible storage for VCF INFO fields

    -- Population genetics
    allele_freq FLOAT,
    allele_count INT,
    total_alleles INT,

    -- Functional annotation
    gene_id VARCHAR(100),
    consequence VARCHAR(100),  -- missense, synonymous, frameshift, etc.
    impact VARCHAR(20),  -- HIGH, MODERATE, LOW, MODIFIER
    protein_change VARCHAR(200),
    transcript_id VARCHAR(100),

    -- Clinical significance
    clinical_significance VARCHAR(100),
    pathogenicity_score FLOAT,

    -- Metadata
    source VARCHAR(100),  -- 1000G, gnomAD, dbSNP, etc.
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (chromosome, id)
) PARTITION BY LIST (chromosome);

-- 2. Create partitions for each chromosome
CREATE TABLE IF NOT EXISTS variants_chr1 PARTITION OF variants FOR VALUES IN ('1');
CREATE TABLE IF NOT EXISTS variants_chr2 PARTITION OF variants FOR VALUES IN ('2');
CREATE TABLE IF NOT EXISTS variants_chr3 PARTITION OF variants FOR VALUES IN ('3');
CREATE TABLE IF NOT EXISTS variants_chr4 PARTITION OF variants FOR VALUES IN ('4');
CREATE TABLE IF NOT EXISTS variants_chr5 PARTITION OF variants FOR VALUES IN ('5');
CREATE TABLE IF NOT EXISTS variants_chr6 PARTITION OF variants FOR VALUES IN ('6');
CREATE TABLE IF NOT EXISTS variants_chr7 PARTITION OF variants FOR VALUES IN ('7');
CREATE TABLE IF NOT EXISTS variants_chr8 PARTITION OF variants FOR VALUES IN ('8');
CREATE TABLE IF NOT EXISTS variants_chr9 PARTITION OF variants FOR VALUES IN ('9');
CREATE TABLE IF NOT EXISTS variants_chr10 PARTITION OF variants FOR VALUES IN ('10');
CREATE TABLE IF NOT EXISTS variants_chr11 PARTITION OF variants FOR VALUES IN ('11');
CREATE TABLE IF NOT EXISTS variants_chr12 PARTITION OF variants FOR VALUES IN ('12');
CREATE TABLE IF NOT EXISTS variants_chr13 PARTITION OF variants FOR VALUES IN ('13');
CREATE TABLE IF NOT EXISTS variants_chr14 PARTITION OF variants FOR VALUES IN ('14');
CREATE TABLE IF NOT EXISTS variants_chr15 PARTITION OF variants FOR VALUES IN ('15');
CREATE TABLE IF NOT EXISTS variants_chr16 PARTITION OF variants FOR VALUES IN ('16');
CREATE TABLE IF NOT EXISTS variants_chr17 PARTITION OF variants FOR VALUES IN ('17');
CREATE TABLE IF NOT EXISTS variants_chr18 PARTITION OF variants FOR VALUES IN ('18');
CREATE TABLE IF NOT EXISTS variants_chr19 PARTITION OF variants FOR VALUES IN ('19');
CREATE TABLE IF NOT EXISTS variants_chr20 PARTITION OF variants FOR VALUES IN ('20');
CREATE TABLE IF NOT EXISTS variants_chr21 PARTITION OF variants FOR VALUES IN ('21');
CREATE TABLE IF NOT EXISTS variants_chr22 PARTITION OF variants FOR VALUES IN ('22');
CREATE TABLE IF NOT EXISTS variants_chrX PARTITION OF variants FOR VALUES IN ('X');
CREATE TABLE IF NOT EXISTS variants_chrY PARTITION OF variants FOR VALUES IN ('Y');
CREATE TABLE IF NOT EXISTS variants_chrMT PARTITION OF variants FOR VALUES IN ('MT', 'M');

-- 3. Create indexes on each partition
DO $$
DECLARE
    chr TEXT;
    chrs TEXT[] := ARRAY['1','2','3','4','5','6','7','8','9','10','11','12','13','14','15','16','17','18','19','20','21','22','X','Y','MT'];
BEGIN
    FOREACH chr IN ARRAY chrs
    LOOP
        -- Position index (most common query)
        EXECUTE format('CREATE INDEX IF NOT EXISTS idx_variants_chr%s_pos ON variants_chr%s(position)', chr, chr);

        -- rsID index
        EXECUTE format('CREATE INDEX IF NOT EXISTS idx_variants_chr%s_rsid ON variants_chr%s(variant_id) WHERE variant_id IS NOT NULL', chr, chr);

        -- Gene index (for gene-centric queries)
        EXECUTE format('CREATE INDEX IF NOT EXISTS idx_variants_chr%s_gene ON variants_chr%s(gene_id) WHERE gene_id IS NOT NULL', chr, chr);

        -- Allele frequency index (for common variants)
        EXECUTE format('CREATE INDEX IF NOT EXISTS idx_variants_chr%s_freq ON variants_chr%s(allele_freq) WHERE allele_freq > 0.01', chr, chr);

        -- Clinical significance index
        EXECUTE format('CREATE INDEX IF NOT EXISTS idx_variants_chr%s_clinical ON variants_chr%s(clinical_significance) WHERE clinical_significance IS NOT NULL', chr, chr);

        -- JSONB index for INFO field queries
        EXECUTE format('CREATE INDEX IF NOT EXISTS idx_variants_chr%s_info ON variants_chr%s USING GIN(info)', chr, chr);
    END LOOP;
END $$;

-- 4. Create materialized view for variant summary statistics
CREATE MATERIALIZED VIEW IF NOT EXISTS variant_stats AS
SELECT
    chromosome,
    COUNT(*) as total_variants,
    COUNT(DISTINCT gene_id) as genes_with_variants,
    AVG(allele_freq) as avg_allele_freq,
    COUNT(*) FILTER (WHERE variant_type = 'SNP') as snp_count,
    COUNT(*) FILTER (WHERE variant_type = 'INDEL') as indel_count,
    COUNT(*) FILTER (WHERE impact = 'HIGH') as high_impact_count,
    COUNT(*) FILTER (WHERE clinical_significance LIKE '%pathogenic%') as pathogenic_count
FROM variants
GROUP BY chromosome;

CREATE INDEX ON variant_stats(chromosome);

-- 5. Parallel query execution note:
-- Note: Storage parameters cannot be set on partitioned tables.
-- Set parallel_workers on individual partitions if needed, or rely on PostgreSQL's
-- automatic parallelism which works well with partitioned tables.

-- Performance notes:
-- - Each partition can be queried independently (10-100x faster)
-- - Indexes are smaller per partition (faster lookups)
-- - Parallel queries across partitions for full genome scans
-- - Expected performance: <10ms for single chromosome, <500ms for full genome
