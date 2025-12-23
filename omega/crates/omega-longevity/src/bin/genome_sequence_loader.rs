// Genome Sequence Loader for Omega-Longevity
// Parses FASTA files and loads chromosome sequences into database

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use flate2::read::GzDecoder;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Clone)]
struct ChromosomeSequence {
    chromosome: String,
    sequence: String,
    length: usize,
}

struct FastaParser;

impl FastaParser {
    /// Parse a FASTA file and extract chromosome sequences
    fn parse_fasta<P: AsRef<Path>>(path: P) -> Result<Vec<ChromosomeSequence>, Box<dyn std::error::Error>> {
        println!("📖 Opening FASTA file...");
        let file = File::open(&path)?;

        let reader: Box<dyn BufRead> = if path.as_ref().to_string_lossy().ends_with(".gz") {
            Box::new(BufReader::new(GzDecoder::new(file)))
        } else {
            Box::new(BufReader::new(file))
        };

        let mut chromosomes = Vec::new();
        let mut current_chr: Option<String> = None;
        let mut current_seq = String::new();
        let mut line_count = 0;

        println!("📖 Parsing sequences...");

        for line in reader.lines() {
            let line = line?;
            line_count += 1;

            if line.starts_with('>') {
                // Save previous chromosome if exists
                if let Some(chr) = current_chr.take() {
                    let length = current_seq.len();
                    println!("  ✓ {}: {} bp", chr, length);
                    chromosomes.push(ChromosomeSequence {
                        chromosome: chr,
                        sequence: current_seq.clone(),
                        length,
                    });
                    current_seq.clear();
                }

                // Parse header: >1 dna:chromosome chromosome:GRCh38:1:1:248956422:1 REF
                let header = line[1..].trim().to_string();
                let parts: Vec<&str> = header.split_whitespace().collect();

                // Extract chromosome name from first part
                let chr_name = parts[0].to_string();
                current_chr = Some(chr_name);
                println!("📍 Found chromosome: {}", current_chr.as_ref().unwrap());
            } else {
                // Append sequence line
                current_seq.push_str(line.trim());
            }

            if line_count % 1_000_000 == 0 {
                println!("  Processed {} lines...", line_count);
            }
        }

        // Don't forget the last chromosome
        if let Some(chr) = current_chr {
            let length = current_seq.len();
            println!("  ✓ {}: {} bp", chr, length);
            chromosomes.push(ChromosomeSequence {
                chromosome: chr,
                sequence: current_seq,
                length,
            });
        }

        println!("✅ Parsed {} chromosomes from {} lines", chromosomes.len(), line_count);
        Ok(chromosomes)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Omega-Longevity Genome Sequence Loader");
    println!("==========================================\n");

    // Get FASTA file path from command line
    let fasta_path = std::env::args()
        .nth(1)
        .ok_or("Usage: genome_sequence_loader <fasta_file>")?;

    // Parse FASTA file
    println!("📖 Parsing FASTA file: {}", fasta_path);
    let start_time = Instant::now();
    let chromosomes = FastaParser::parse_fasta(&fasta_path)?;
    let parse_time = start_time.elapsed();

    println!("\n✅ Parsed {} chromosomes in {:?}\n", chromosomes.len(), parse_time);

    // Print statistics
    println!("📊 Chromosome Statistics:");
    let mut total_bp = 0u64;
    for chr in &chromosomes {
        println!("  {}: {:>12} bp", chr.chromosome, chr.length);
        total_bp += chr.length as u64;
    }
    println!("  {:<15} {:>12}", "─".repeat(15), "");
    println!("  Total: {:>12} bp ({:.2} Gb)", total_bp, total_bp as f64 / 1_000_000_000.0);

    // Connect to database
    println!("\n💾 Connecting to database...");
    let database_url = std::env::var("RUVECTOR_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!("✅ Connected to database\n");

    // Create sequences table
    println!("📋 Creating chromosome_sequences table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chromosome_sequences (
            id SERIAL PRIMARY KEY,
            chromosome VARCHAR(50) NOT NULL UNIQUE,
            sequence TEXT NOT NULL,
            length BIGINT NOT NULL,
            gc_content FLOAT,
            n_count BIGINT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Create index
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_chr_sequences_name ON chromosome_sequences(chromosome)")
        .execute(&pool)
        .await?;

    println!("✅ Table and indexes created\n");

    // Load chromosomes into database
    println!("🔄 Loading chromosome sequences into database...");
    let load_start = Instant::now();
    let mut loaded_count = 0;

    for (idx, chr) in chromosomes.iter().enumerate() {
        // Calculate GC content
        let gc_count = chr.sequence.chars()
            .filter(|c| *c == 'G' || *c == 'C' || *c == 'g' || *c == 'c')
            .count();
        let gc_content = (gc_count as f64 / chr.length as f64) * 100.0;

        // Count N's (unknown bases)
        let n_count = chr.sequence.chars()
            .filter(|c| *c == 'N' || *c == 'n')
            .count() as i64;

        // For large sequences, we'll store them in chunks or compressed
        // For now, store directly (PostgreSQL can handle large TEXT fields)
        println!("  Loading {} ({} bp, {:.2}% GC)...",
            chr.chromosome, chr.length, gc_content);

        let result = sqlx::query(
            "INSERT INTO chromosome_sequences (chromosome, sequence, length, gc_content, n_count)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (chromosome) DO UPDATE
             SET sequence = EXCLUDED.sequence,
                 length = EXCLUDED.length,
                 gc_content = EXCLUDED.gc_content,
                 n_count = EXCLUDED.n_count"
        )
        .bind(&chr.chromosome)
        .bind(&chr.sequence)
        .bind(chr.length as i64)
        .bind(gc_content)
        .bind(n_count)
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
    let (total_chr,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM chromosome_sequences")
        .fetch_one(&pool)
        .await?;

    let (total_bases,): (i64,) = sqlx::query_as("SELECT SUM(length) FROM chromosome_sequences")
        .fetch_one(&pool)
        .await?;

    let (avg_gc,): (Option<f64>,) = sqlx::query_as("SELECT AVG(gc_content) FROM chromosome_sequences")
        .fetch_one(&pool)
        .await?;

    println!("\n📊 Database Statistics:");
    println!("  Total chromosomes: {}", total_chr);
    println!("  Total base pairs: {} ({:.2} Gb)", total_bases, total_bases as f64 / 1_000_000_000.0);
    println!("  Average GC content: {:.2}%", avg_gc.unwrap_or(0.0));

    println!("\n🎉 Genome sequence loading complete!");
    println!("\n⏱️  Performance Summary:");
    println!("  Parsing: {:?} ({:.0} bp/sec)",
        parse_time,
        total_bp as f64 / parse_time.as_secs_f64()
    );
    println!("  Loading: {:?} ({:.0} bp/sec)",
        load_time,
        total_bases as f64 / load_time.as_secs_f64()
    );
    println!("  Total: {:?}", parse_time + load_time);

    pool.close().await;
    Ok(())
}
