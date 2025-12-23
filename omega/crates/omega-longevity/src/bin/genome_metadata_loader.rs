// Genome Metadata Loader for Omega-Longevity
// Parses FASTA files and loads chromosome metadata (NOT full sequences)

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use flate2::read::GzDecoder;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Clone)]
struct ChromosomeMetadata {
    chromosome: String,
    length: i64,
    gc_content: f64,
    n_count: i64,
    a_count: i64,
    t_count: i64,
    g_count: i64,
    c_count: i64,
}

struct FastaParser;

impl FastaParser {
    /// Parse a FASTA file and extract chromosome metadata
    fn parse_fasta_metadata<P: AsRef<Path>>(path: P) -> Result<Vec<ChromosomeMetadata>, Box<dyn std::error::Error>> {
        println!("📖 Opening FASTA file...");
        let file = File::open(&path)?;

        let reader: Box<dyn BufRead> = if path.as_ref().to_string_lossy().ends_with(".gz") {
            Box::new(BufReader::new(GzDecoder::new(file)))
        } else {
            Box::new(BufReader::new(file))
        };

        let mut chromosomes = Vec::new();
        let mut current_chr: Option<String> = None;
        let mut a_count = 0i64;
        let mut t_count = 0i64;
        let mut g_count = 0i64;
        let mut c_count = 0i64;
        let mut n_count = 0i64;
        let mut length = 0i64;
        let mut line_count = 0;

        println!("📖 Parsing sequences (metadata only)...\n");

        for line in reader.lines() {
            let line = line?;
            line_count += 1;

            if line.starts_with('>') {
                // Save previous chromosome if exists
                if let Some(chr) = current_chr.take() {
                    let gc_content = if length > 0 {
                        ((g_count + c_count) as f64 / length as f64) * 100.0
                    } else {
                        0.0
                    };

                    println!("  ✓ {}: {} bp ({:.2}% GC)", chr, length, gc_content);
                    chromosomes.push(ChromosomeMetadata {
                        chromosome: chr,
                        length,
                        gc_content,
                        n_count,
                        a_count,
                        t_count,
                        g_count,
                        c_count,
                    });

                    // Reset counters
                    a_count = 0;
                    t_count = 0;
                    g_count = 0;
                    c_count = 0;
                    n_count = 0;
                    length = 0;
                }

                // Parse header: >1 dna:chromosome chromosome:GRCh38:1:1:248956422:1 REF
                let header = line[1..].trim().to_string();
                let parts: Vec<&str> = header.split_whitespace().collect();
                let chr_name = parts[0].to_string();
                current_chr = Some(chr_name);
                println!("📍 Found chromosome: {}", current_chr.as_ref().unwrap());
            } else {
                // Count bases without storing the sequence
                for ch in line.chars() {
                    match ch.to_ascii_uppercase() {
                        'A' => a_count += 1,
                        'T' => t_count += 1,
                        'G' => g_count += 1,
                        'C' => c_count += 1,
                        'N' => n_count += 1,
                        _ => {} // Ignore other characters
                    }
                    length += 1;
                }
            }

            if line_count % 1_000_000 == 0 {
                println!("  Processed {} lines...", line_count);
            }
        }

        // Don't forget the last chromosome
        if let Some(chr) = current_chr {
            let gc_content = if length > 0 {
                ((g_count + c_count) as f64 / length as f64) * 100.0
            } else {
                0.0
            };

            println!("  ✓ {}: {} bp ({:.2}% GC)", chr, length, gc_content);
            chromosomes.push(ChromosomeMetadata {
                chromosome: chr,
                length,
                gc_content,
                n_count,
                a_count,
                t_count,
                g_count,
                c_count,
            });
        }

        println!("\n✅ Parsed {} chromosomes from {} lines", chromosomes.len(), line_count);
        Ok(chromosomes)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Omega-Longevity Genome Metadata Loader");
    println!("==========================================\n");

    // Get FASTA file path from command line
    let fasta_path = std::env::args()
        .nth(1)
        .ok_or("Usage: genome_metadata_loader <fasta_file>")?;

    // Parse FASTA file
    println!("📖 Parsing FASTA file: {}", fasta_path);
    let start_time = Instant::now();
    let chromosomes = FastaParser::parse_fasta_metadata(&fasta_path)?;
    let parse_time = start_time.elapsed();

    println!("\n✅ Parsed {} chromosomes in {:?}\n", chromosomes.len(), parse_time);

    // Print statistics
    println!("📊 Chromosome Statistics:");
    let mut total_bp = 0i64;
    let mut total_gc = 0i64;
    for chr in &chromosomes {
        println!("  {:<15} {:>12} bp  ({:.2}% GC, {:>10} N's)",
            chr.chromosome, chr.length, chr.gc_content, chr.n_count);
        total_bp += chr.length;
        total_gc += chr.g_count + chr.c_count;
    }
    let avg_gc = if total_bp > 0 {
        (total_gc as f64 / total_bp as f64) * 100.0
    } else {
        0.0
    };
    println!("  {:<15} {:>12} bp  ({:.2}% GC)", "─".repeat(15), total_bp, avg_gc);
    println!("  Total: {} bp ({:.2} Gb)\n", total_bp, total_bp as f64 / 1_000_000_000.0);

    // Connect to database
    println!("💾 Connecting to database...");
    let database_url = std::env::var("RUVECTOR_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!("✅ Connected to database\n");

    // Create chromosome_metadata table
    println!("📋 Creating chromosome_metadata table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chromosome_metadata (
            id SERIAL PRIMARY KEY,
            chromosome VARCHAR(50) NOT NULL UNIQUE,
            length BIGINT NOT NULL,
            gc_content FLOAT NOT NULL,
            n_count BIGINT NOT NULL,
            a_count BIGINT NOT NULL,
            t_count BIGINT NOT NULL,
            g_count BIGINT NOT NULL,
            c_count BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Create index
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_chr_metadata_name ON chromosome_metadata(chromosome)")
        .execute(&pool)
        .await?;

    println!("✅ Table and index created\n");

    // Load chromosome metadata into database
    println!("🔄 Loading chromosome metadata into database...");
    let load_start = Instant::now();
    let mut loaded_count = 0;

    for chr in chromosomes.iter() {
        println!("  Loading {} ({} bp, {:.2}% GC)...",
            chr.chromosome, chr.length, chr.gc_content);

        let result = sqlx::query(
            "INSERT INTO chromosome_metadata
             (chromosome, length, gc_content, n_count, a_count, t_count, g_count, c_count)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (chromosome) DO UPDATE
             SET length = EXCLUDED.length,
                 gc_content = EXCLUDED.gc_content,
                 n_count = EXCLUDED.n_count,
                 a_count = EXCLUDED.a_count,
                 t_count = EXCLUDED.t_count,
                 g_count = EXCLUDED.g_count,
                 c_count = EXCLUDED.c_count"
        )
        .bind(&chr.chromosome)
        .bind(chr.length)
        .bind(chr.gc_content)
        .bind(chr.n_count)
        .bind(chr.a_count)
        .bind(chr.t_count)
        .bind(chr.g_count)
        .bind(chr.c_count)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => {
                loaded_count += 1;
                println!("    ✓ Loaded successfully");
            }
            Err(e) => eprintln!("    ✗ Error loading chromosome {}: {}", chr.chromosome, e),
        }
    }

    let load_time = load_start.elapsed();
    println!("\n✅ Load complete in {:?}", load_time);
    println!("  Loaded: {} chromosomes", loaded_count);

    // Show final statistics
    let (total_chr,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM chromosome_metadata")
        .fetch_one(&pool)
        .await?;

    let (total_bases,): (Option<i64>,) = sqlx::query_as("SELECT SUM(length)::BIGINT FROM chromosome_metadata")
        .fetch_one(&pool)
        .await?;

    let (avg_gc,): (Option<f64>,) = sqlx::query_as("SELECT AVG(gc_content) FROM chromosome_metadata")
        .fetch_one(&pool)
        .await?;

    println!("\n📊 Database Statistics:");
    println!("  Total chromosomes: {}", total_chr);
    let bases = total_bases.unwrap_or(0);
    println!("  Total base pairs: {} ({:.2} Gb)", bases, bases as f64 / 1_000_000_000.0);
    println!("  Average GC content: {:.2}%", avg_gc.unwrap_or(0.0));

    println!("\n🎉 Genome metadata loading complete!");
    println!("\n⏱️  Performance Summary:");
    println!("  Parsing: {:?} ({:.0} bp/sec)",
        parse_time,
        total_bp as f64 / parse_time.as_secs_f64()
    );
    println!("  Loading: {:?}", load_time);
    println!("  Total: {:?}", parse_time + load_time);

    println!("\n💡 Note: Raw sequences NOT stored in database to save space.");
    println!("   Source file: {}", fasta_path);

    pool.close().await;
    Ok(())
}
