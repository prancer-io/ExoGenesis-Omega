// Genome Data Loader for Omega-Longevity
// Parses GTF files and loads genes into RuVector database

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use sqlx::postgres::PgPoolOptions;
use flate2::read::GzDecoder;

#[derive(Debug, Clone)]
struct Gene {
    gene_id: String,
    gene_name: String,
    chromosome: String,
    start: i64,
    end: i64,
    strand: String,
    gene_type: String,
    description: Option<String>,
}

struct GtfParser;

impl GtfParser {
    /// Parse a GTF file and extract gene information
    fn parse_gtf<P: AsRef<Path>>(path: P) -> Result<Vec<Gene>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader: Box<dyn BufRead> = if file.metadata()?.len() > 0 {
            let path_str = file.metadata().ok().and_then(|_| Some(""));
            if let Some(p) = path_str {
                if p.ends_with(".gz") {
                    Box::new(BufReader::new(GzDecoder::new(file)))
                } else {
                    Box::new(BufReader::new(file))
                }
            } else {
                Box::new(BufReader::new(file))
            }
        } else {
            Box::new(BufReader::new(file))
        };

        // Try both compressed and uncompressed
        let is_gzipped = std::env::args()
            .nth(1)
            .map(|p| p.ends_with(".gz"))
            .unwrap_or(false);

        let file = File::open(std::env::args().nth(1).unwrap())?;
        let reader: Box<dyn BufRead> = if is_gzipped {
            Box::new(BufReader::new(GzDecoder::new(file)))
        } else {
            Box::new(BufReader::new(file))
        };

        let mut genes = HashMap::new();
        let mut line_count = 0;

        for line in reader.lines() {
            let line = line?;
            line_count += 1;

            // Skip comments and headers
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            // Parse GTF line
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() < 9 {
                continue;
            }

            // Only process gene features
            let feature_type = fields[2];
            if feature_type != "gene" {
                continue;
            }

            let chromosome = fields[0].to_string();
            let start: i64 = fields[3].parse()?;
            let end: i64 = fields[4].parse()?;
            let strand = fields[6].to_string();
            let attributes = fields[8];

            // Parse attributes
            let mut gene_id = String::new();
            let mut gene_name = String::new();
            let mut gene_type = String::new();
            let mut description = None;

            for attr in attributes.split(';') {
                let attr = attr.trim();
                if attr.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = attr.splitn(2, ' ').collect();
                if parts.len() != 2 {
                    continue;
                }

                let key = parts[0];
                let value = parts[1].trim_matches('"');

                match key {
                    "gene_id" => gene_id = value.to_string(),
                    "gene_name" => gene_name = value.to_string(),
                    "gene_type" | "gene_biotype" => gene_type = value.to_string(),
                    "description" => description = Some(value.to_string()),
                    _ => {}
                }
            }

            if !gene_id.is_empty() {
                genes.insert(
                    gene_id.clone(),
                    Gene {
                        gene_id,
                        gene_name,
                        chromosome,
                        start,
                        end,
                        strand,
                        gene_type,
                        description,
                    },
                );
            }

            if line_count % 100_000 == 0 {
                println!("Processed {} lines, found {} genes", line_count, genes.len());
            }
        }

        let mut gene_vec: Vec<Gene> = genes.into_values().collect();
        gene_vec.sort_by(|a, b| {
            a.chromosome
                .cmp(&b.chromosome)
                .then(a.start.cmp(&b.start))
        });

        println!(
            "Parsing complete: {} genes from {} lines",
            gene_vec.len(),
            line_count
        );

        Ok(gene_vec)
    }
}

/// Simple gene encoder that creates feature vectors from gene properties
struct GeneEncoder;

impl GeneEncoder {
    /// Generate a 384-dimensional embedding for a gene
    /// This is a simplified version - in production, use a proper embedding model
    fn encode_gene(gene: &Gene) -> Vec<f32> {
        let mut embedding = vec![0.0f32; 384];

        // Feature 1-10: Chromosome encoding (one-hot-ish)
        let chr_num = Self::chromosome_to_number(&gene.chromosome);
        if chr_num < 10 {
            embedding[chr_num] = 1.0;
        }

        // Feature 11-20: Gene type encoding
        let type_idx = Self::gene_type_to_index(&gene.gene_type);
        if type_idx < 10 {
            embedding[10 + type_idx] = 1.0;
        }

        // Feature 21-40: Gene length (log-scaled, binned)
        let length = (gene.end - gene.start) as f32;
        let log_length = length.log10();
        let length_bin = (log_length * 2.0).min(19.0) as usize;
        embedding[20 + length_bin] = 1.0;

        // Feature 41-50: Strand information
        embedding[40] = if gene.strand == "+" { 1.0 } else { -1.0 };

        // Feature 51-384: Hash-based encoding of gene name (for similarity)
        let name_hash = Self::hash_string(&gene.gene_name);
        for i in 0..334 {
            let bit_idx = (name_hash >> i) % 2;
            embedding[50 + i] = if bit_idx == 1 { 0.1 } else { -0.1 };
        }

        // Normalize the embedding
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in embedding.iter_mut() {
                *val /= norm;
            }
        }

        embedding
    }

    fn chromosome_to_number(chr: &str) -> usize {
        let chr = chr.trim_start_matches("chr");
        match chr.parse::<usize>() {
            Ok(n) if n <= 22 => n.saturating_sub(1),
            _ if chr == "X" => 22,
            _ if chr == "Y" => 23,
            _ if chr == "MT" || chr == "M" => 24,
            _ => 25,
        }
    }

    fn gene_type_to_index(gene_type: &str) -> usize {
        match gene_type {
            "protein_coding" => 0,
            "lncRNA" | "lincRNA" => 1,
            "miRNA" => 2,
            "snRNA" => 3,
            "snoRNA" => 4,
            "rRNA" => 5,
            "tRNA" => 6,
            "pseudogene" => 7,
            "IG_gene" | "TR_gene" => 8,
            _ => 9,
        }
    }

    fn hash_string(s: &str) -> usize {
        s.bytes().fold(0usize, |acc, b| acc.wrapping_mul(31).wrapping_add(b as usize))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Omega-Longevity Genome Loader");
    println!("================================\n");

    // Get GTF file path from command line
    let gtf_path = std::env::args().nth(1).ok_or("Usage: genome_loader <gtf_file>")?;

    // Parse GTF file
    println!("📖 Parsing GTF file: {}", gtf_path);
    let start_time = Instant::now();
    let genes = GtfParser::parse_gtf(&gtf_path)?;
    let parse_time = start_time.elapsed();
    println!("✅ Parsed {} genes in {:?}\n", genes.len(), parse_time);

    // Print statistics
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    let mut chr_counts: HashMap<String, usize> = HashMap::new();

    for gene in &genes {
        *type_counts.entry(gene.gene_type.clone()).or_insert(0) += 1;
        *chr_counts.entry(gene.chromosome.clone()).or_insert(0) += 1;
    }

    println!("📊 Gene Type Distribution:");
    let mut sorted_types: Vec<_> = type_counts.into_iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(&a.1));
    for (gene_type, count) in sorted_types.iter().take(10) {
        println!("  {}: {}", gene_type, count);
    }

    println!("\n📊 Chromosome Distribution:");
    let mut sorted_chrs: Vec<_> = chr_counts.into_iter().collect();
    sorted_chrs.sort_by_key(|(chr, _)| GeneEncoder::chromosome_to_number(chr));
    for (chr, count) in sorted_chrs {
        println!("  {}: {}", chr, count);
    }

    // Connect to database
    println!("\n💾 Connecting to database...");
    let database_url = std::env::var("RUVECTOR_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!("✅ Connected to database\n");

    // Create genes table if it doesn't exist
    println!("📋 Creating genes table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS genes (
            id SERIAL PRIMARY KEY,
            gene_id VARCHAR(50) NOT NULL UNIQUE,
            gene_name VARCHAR(100) NOT NULL,
            chromosome VARCHAR(10) NOT NULL,
            start_pos BIGINT NOT NULL,
            end_pos BIGINT NOT NULL,
            strand VARCHAR(1) NOT NULL,
            gene_type VARCHAR(50) NOT NULL,
            description TEXT,
            embedding ruvector(384),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Create index on gene_name for fast lookups
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_genes_name ON genes(gene_name)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_genes_type ON genes(gene_type)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_genes_chr ON genes(chromosome)")
        .execute(&pool)
        .await?;

    println!("✅ Table and indexes created\n");

    // Load genes into database
    println!("🔄 Loading genes into database...");
    let load_start = Instant::now();
    let mut loaded_count = 0;
    let mut skipped_count = 0;

    for (idx, gene) in genes.iter().enumerate() {
        // Generate embedding
        let embedding = GeneEncoder::encode_gene(gene);

        // Insert into database
        let result = sqlx::query(
            "INSERT INTO genes (gene_id, gene_name, chromosome, start_pos, end_pos, strand, gene_type, description)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (gene_id) DO NOTHING"
        )
        .bind(&gene.gene_id)
        .bind(&gene.gene_name)
        .bind(&gene.chromosome)
        .bind(gene.start)
        .bind(gene.end)
        .bind(&gene.strand)
        .bind(&gene.gene_type)
        .bind(&gene.description)
        .execute(&pool)
        .await;

        match result {
            Ok(r) if r.rows_affected() > 0 => loaded_count += 1,
            Ok(_) => skipped_count += 1,
            Err(e) => eprintln!("Error inserting gene {}: {}", gene.gene_id, e),
        }

        if (idx + 1) % 1000 == 0 {
            println!("  Loaded {} / {} genes...", idx + 1, genes.len());
        }
    }

    let load_time = load_start.elapsed();
    println!("\n✅ Load complete in {:?}", load_time);
    println!("  Loaded: {} genes", loaded_count);
    println!("  Skipped (duplicates): {} genes", skipped_count);

    // Show final statistics
    let (total_genes,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM genes")
        .fetch_one(&pool)
        .await?;

    println!("\n📊 Database Statistics:");
    println!("  Total genes: {}", total_genes);

    let protein_coding: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM genes WHERE gene_type = 'protein_coding'")
        .fetch_one(&pool)
        .await?;
    println!("  Protein-coding: {}", protein_coding.0);

    println!("\n🎉 Genome loading complete!");
    println!("\n⏱️  Performance Summary:");
    println!("  Parsing: {:?} ({:.0} genes/sec)", parse_time, genes.len() as f64 / parse_time.as_secs_f64());
    println!("  Loading: {:?} ({:.0} genes/sec)", load_time, loaded_count as f64 / load_time.as_secs_f64());
    println!("  Total: {:?}", parse_time + load_time);

    pool.close().await;
    Ok(())
}
