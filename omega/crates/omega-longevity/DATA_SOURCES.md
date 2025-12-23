# Genome Data Sources for Omega-Longevity

## Recommended Data Sources

### 1. Human Reference Genome (GRCh38/hg38)

**Source:** NCBI/Ensembl
**Size:** ~3.2 GB (compressed FASTA)
**URL:** https://ftp.ensembl.org/pub/release-111/fasta/homo_sapiens/dna/

**Quick Download (Chromosome 22 for testing):**
```bash
# Smallest autosome (~51 MB) - good for testing
wget https://ftp.ensembl.org/pub/release-111/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.chromosome.22.fa.gz

# Full genome (all chromosomes)
wget https://ftp.ensembl.org/pub/release-111/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz
```

### 2. Gene Annotations (GTF/GFF3)

**Source:** Ensembl Gene Build
**Size:** ~50 MB (compressed)
**URL:** https://ftp.ensembl.org/pub/release-111/gtf/homo_sapiens/

**Download:**
```bash
wget https://ftp.ensembl.org/pub/release-111/gtf/homo_sapiens/Homo_sapiens.GRCh38.111.gtf.gz
```

Contains:
- 20,000+ protein-coding genes
- Gene coordinates (chromosome, start, end, strand)
- Transcript variants
- Exon/intron boundaries
- Gene names and IDs

### 3. Variant Data (VCF)

**Source:** 1000 Genomes Project Phase 3
**Size:** ~15 GB per chromosome (compressed)
**URL:** https://ftp.1000genomes.ebi.ac.uk/vol1/ftp/release/20130502/

**Sample Download (Chromosome 22):**
```bash
wget https://ftp.1000genomes.ebi.ac.uk/vol1/ftp/release/20130502/ALL.chr22.phase3_shapeit2_mvncall_integrated_v5b.20130502.genotypes.vcf.gz
```

### 4. dbSNP (Variant Database)

**Source:** NCBI dbSNP Build 156
**Size:** ~30 GB (compressed VCF)
**URL:** https://ftp.ncbi.nih.gov/snp/latest_release/VCF/

**Download:**
```bash
# Per chromosome
wget https://ftp.ncbi.nih.gov/snp/latest_release/VCF/GCF_000001405.40.gz
```

### 5. ClinVar (Clinical Variants)

**Source:** NCBI ClinVar
**Size:** ~200 MB (compressed)
**URL:** https://ftp.ncbi.nlm.nih.gov/pub/clinvar/vcf_GRCh38/

**Download:**
```bash
wget https://ftp.ncbi.nlm.nih.gov/pub/clinvar/vcf_GRCh38/clinvar.vcf.gz
```

## Recommended Download Strategy

### For Initial Testing (Fast - ~100 MB total)

1. **Chromosome 22** (smallest autosome)
2. **Full GTF annotations** (all genes)
3. **ClinVar variants** (clinically relevant)

```bash
cd /home/farchide/repo/ExoGenesis-Omega/omega/crates/omega-longevity/data

# Reference genome - Chr 22 only
wget -P genomes/ https://ftp.ensembl.org/pub/release-111/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.chromosome.22.fa.gz

# Gene annotations - all genes
wget -P annotations/ https://ftp.ensembl.org/pub/release-111/gtf/homo_sapiens/Homo_sapiens.GRCh38.111.gtf.gz

# Clinical variants
wget -P variants/ https://ftp.ncbi.nlm.nih.gov/pub/clinvar/vcf_GRCh38/clinvar.vcf.gz
wget -P variants/ https://ftp.ncbi.nlm.nih.gov/pub/clinvar/vcf_GRCh38/clinvar.vcf.gz.tbi
```

### For Production (Full - ~20 GB total)

```bash
# Full reference genome
wget -P genomes/ https://ftp.ensembl.org/pub/release-111/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz

# All annotations
wget -P annotations/ https://ftp.ensembl.org/pub/release-111/gtf/homo_sapiens/Homo_sapiens.GRCh38.111.gtf.gz

# 1000 Genomes variants (per chromosome)
for i in {1..22} X Y; do
  wget -P variants/ https://ftp.1000genomes.ebi.ac.uk/vol1/ftp/release/20130502/ALL.chr${i}.phase3_shapeit2_mvncall_integrated_v5b.20130502.genotypes.vcf.gz
done
```

## Alternative: Pre-curated Longevity Gene Sets

### LongevityMap

**Source:** Human Ageing Genomic Resources
**URL:** https://genomics.senescence.info/longevity/
**Genes:** 2,000+ longevity-associated genes

### GenAge

**Source:** Database of Aging-Related Genes
**URL:** https://genomics.senescence.info/genes/
**Genes:** 300+ aging-related genes in model organisms

### Download:**
```bash
# Download gene lists
wget -P annotations/ https://genomics.senescence.info/longevity/longevity_genes.zip
```

## Expected Data Sizes

| Dataset | Compressed | Uncompressed | Records |
|---------|-----------|--------------|---------|
| Chr 22 FASTA | 12 MB | 51 MB | 51 Mbp |
| Full genome FASTA | 900 MB | 3.2 GB | 3.2 Gbp |
| GTF annotations | 50 MB | 1.5 GB | 3M+ features |
| ClinVar VCF | 50 MB | 200 MB | 2M+ variants |
| 1000G Chr 22 | 500 MB | 2 GB | 1M+ variants |
| Full dbSNP | 30 GB | 100 GB | 150M+ variants |

## Data Processing Tools Needed

```bash
# Install genomics tools
sudo apt-get update
sudo apt-get install -y \
  tabix \        # Index VCF files
  bcftools \     # VCF manipulation
  samtools \     # FASTA indexing
  bedtools \     # Genomic interval operations
  vcftools       # VCF filtering
```

## Parsing Strategy

1. **FASTA (Genome)**: Use `bio` crate in Rust
2. **GTF (Annotations)**: Custom parser or `gff` crate
3. **VCF (Variants)**: Use `rust-htslib` or `noodles` crate

## Estimated Timeline

- **Download (Chr 22 + annotations)**: 5-10 minutes
- **Parsing GTF**: 2-5 minutes (20K genes)
- **Database loading**: 5-10 minutes (with embeddings)
- **Initial tests**: 5 minutes
- **Total**: ~30 minutes for testing setup

## Performance Expectations

With RuVector HNSW indexing:

- **Gene name lookup**: <1ms
- **Gene similarity search**: 1-10ms (top 100 results)
- **Variant annotation**: 5-50ms per variant
- **Batch operations**: 1000+ genes/second

## Next Steps

1. Download chromosome 22 + GTF (fastest option)
2. Parse and load genes into database
3. Generate embeddings for all genes
4. Create HNSW index
5. Run performance benchmarks
6. Optimize based on results
