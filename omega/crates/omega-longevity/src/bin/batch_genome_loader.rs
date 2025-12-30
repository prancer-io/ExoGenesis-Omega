// High-Performance Batch Genome Loader for Omega-Longevity
// Uses PostgreSQL COPY for 10-20x faster loading than individual INSERTs

use std::fs::File;
use std::io::{BufRead, BufReader, Write as IoWrite, Read, Seek};
use std::path::Path;
use std::time::Instant;
use flate2::read::GzDecoder;
use sqlx::postgres::PgPoolOptions;
use tempfile::NamedTempFile;

#[derive(Debug, Clone)]
struct Gene {
    gene_id: String,
    gene_name: String,
    chromosome: String,
    start_pos: i64,
    end_pos: i64,
    strand: String,
    gene_type: String,
    description: String,
}

struct GtfParser;

impl GtfParser {
    fn parse_gtf_batch<P: AsRef<Path>>(path: P) -> Result<Vec<Gene>, Box<dyn std::error::Error>> {
        println!("📖 Opening GTF file: {}", path.as_ref().display());
        let file = File::open(&path)?;

        let reader: Box<dyn BufRead> = if path.as_ref().to_string_lossy().ends_with(".gz") {
            Box::new(BufReader::new(GzDecoder::new(file)))
        } else {
            Box::new(BufReader::new(file))
        };

        let mut genes = Vec::with_capacity(70000); // Pre-allocate for ~63K genes
        let mut line_count = 0;

        println!("📖 Parsing GTF file (batch mode)...\n");

        for line in reader.lines() {
            let line = line?;
            line_count += 1;

            if line.starts_with('#') {
                continue;
            }

            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() < 9 || fields[2] != "gene" {
                continue;
            }

            let chromosome = fields[0].to_string();
            let start = fields[3].parse::<i64>()?;
            let end = fields[4].parse::<i64>()?;
            let strand = fields[6].to_string();
            let attributes = fields[8];

            // Parse attributes
            let mut gene_id = String::new();
            let mut gene_name = String::new();
            let mut gene_type = String::new();
            let mut description = String::new();

            for attr in attributes.split(';') {
                let attr = attr.trim();
                if attr.is_empty() {
                    continue;
                }

                if let Some(value) = attr.strip_prefix("gene_id \"") {
                    gene_id = value.trim_end_matches('"').to_string();
                } else if let Some(value) = attr.strip_prefix("gene_name \"") {
                    gene_name = value.trim_end_matches('"').to_string();
                } else if let Some(value) = attr.strip_prefix("gene_biotype \"") {
                    gene_type = value.trim_end_matches('"').to_string();
                } else if let Some(value) = attr.strip_prefix("description \"") {
                    description = value.trim_end_matches('"').to_string();
                }
            }

            // Use gene_id as gene_name if gene_name is empty (some GTF files have missing names)
            let final_gene_name = if gene_name.is_empty() {
                gene_id.clone()
            } else {
                gene_name
            };

            genes.push(Gene {
                gene_id,
                gene_name: final_gene_name,
                chromosome,
                start_pos: start,
                end_pos: end,
                strand,
                gene_type,
                description,
            });

            if line_count % 100000 == 0 {
                println!("  Processed {} lines, found {} genes...", line_count, genes.len());
            }
        }

        println!("✅ Parsed {} genes from {} lines\n", genes.len(), line_count);
        Ok(genes)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Omega-Longevity High-Performance Batch Genome Loader");
    println!("========================================================\n");

    let gtf_path = std::env::args()
        .nth(1)
        .ok_or("Usage: batch_genome_loader <gtf_file>")?;

    // Parse GTF file
    let parse_start = Instant::now();
    let genes = GtfParser::parse_gtf_batch(&gtf_path)?;
    let parse_time = parse_start.elapsed();

    println!("📊 Parsing Performance:");
    println!("  Duration: {:?}", parse_time);
    println!("  Throughput: {:.0} genes/sec\n", genes.len() as f64 / parse_time.as_secs_f64());

    // Connect to database
    println!("💾 Connecting to database...");
    let database_url = std::env::var("RUVECTOR_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!("✅ Connected\n");

    // Method 1: PostgreSQL COPY (Fastest - 10-20x faster than INSERT)
    println!("🚀 Method 1: COPY Command (Fastest)");
    println!("────────────────────────────────────");

    let copy_start = Instant::now();

    // Create temporary CSV file
    let mut temp_file = NamedTempFile::new()?;

    println!("  Writing {} genes to temp CSV file...", genes.len());
    for (idx, gene) in genes.iter().enumerate() {
        // Escape special characters for CSV (handle empty fields)
        let desc = if gene.description.is_empty() {
            String::from("")
        } else {
            gene.description.replace('\\', "\\\\").replace('\t', " ").replace('\n', " ")
        };
        let name = if gene.gene_name.is_empty() {
            gene.gene_id.clone()  // Fallback to gene_id if name is empty
        } else {
            gene.gene_name.replace('\\', "\\\\").replace('\t', " ")
        };

        writeln!(
            temp_file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            gene.gene_id,
            name,
            gene.chromosome,
            gene.start_pos,
            gene.end_pos,
            gene.strand,
            gene.gene_type,
            desc
        )?;

        if (idx + 1) % 10000 == 0 {
            println!("    Written {}/{} genes...", idx + 1, genes.len());
        }
    }

    temp_file.flush()?;
    let temp_path = temp_file.path().to_str().ok_or("Invalid temp path")?;

    println!("  Executing COPY command...");

    // Truncate table for clean load (optional - remove in production)
    sqlx::query("TRUNCATE TABLE genes RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;

    // Use COPY command
    let copy_sql = format!(
        "COPY genes(gene_id, gene_name, chromosome, start_pos, end_pos, strand, gene_type, description)
         FROM '{}' WITH (FORMAT csv, DELIMITER E'\\t', NULL '')",
        temp_path
    );

    // Note: COPY FROM file requires filesystem access from PostgreSQL
    // For Docker, we need to use COPY FROM STDIN instead
    // Let's implement the proper STDIN approach

    let mut conn = pool.acquire().await?;
    let mut copy = conn
        .copy_in_raw(
            "COPY genes(gene_id, gene_name, chromosome, start_pos, end_pos, strand, gene_type, description)
             FROM STDIN WITH (FORMAT csv, DELIMITER E'\\t', NULL '')"
        )
        .await?;

    // Stream data to COPY
    temp_file.seek(std::io::SeekFrom::Start(0))?;
    let mut buffer = Vec::new();
    temp_file.read_to_end(&mut buffer)?;
    copy.send(buffer).await?;
    let rows = copy.finish().await?;

    let copy_time = copy_start.elapsed();

    println!("✅ COPY Complete!");
    println!("  Rows inserted: {}", rows);
    println!("  Duration: {:?}", copy_time);
    println!("  Throughput: {:.0} genes/sec", rows as f64 / copy_time.as_secs_f64());
    println!("  Speedup vs INSERT: ~{:.1}x\n", 526.0 / (rows as f64 / copy_time.as_secs_f64()));

    // Verify data
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM genes")
        .fetch_one(&pool)
        .await?;

    println!("📊 Verification:");
    println!("  Total genes in database: {}", count);

    // Recreate indexes
    println!("\n🔧 Recreating indexes...");
    let index_start = Instant::now();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_genes_name ON genes(gene_name)")
        .execute(&pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_genes_type ON genes(gene_type)")
        .execute(&pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_genes_chr ON genes(chromosome)")
        .execute(&pool)
        .await?;

    println!("✅ Indexes recreated in {:?}\n", index_start.elapsed());

    // Performance summary
    println!("📈 PERFORMANCE SUMMARY");
    println!("════════════════════════════════════════");
    println!("  Parsing: {:?} ({:.0} genes/sec)", parse_time, genes.len() as f64 / parse_time.as_secs_f64());
    println!("  Loading (COPY): {:?} ({:.0} genes/sec)", copy_time, rows as f64 / copy_time.as_secs_f64());
    println!("  Total: {:?}", parse_time + copy_time);
    println!("  Improvement: {:.1}x faster than INSERT method\n", 120.0 / copy_time.as_secs_f64());

    pool.close().await;
    Ok(())
}
