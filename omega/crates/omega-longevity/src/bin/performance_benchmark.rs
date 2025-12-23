// Performance Benchmarking Suite for Omega-Longevity
// Tests gene search, similarity search, and database operations

use std::time::{Duration, Instant};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

#[derive(Debug)]
struct BenchmarkResult {
    operation: String,
    total_time: Duration,
    iterations: usize,
    avg_time_ms: f64,
    min_time_ms: f64,
    max_time_ms: f64,
    throughput: f64, // operations per second
}

impl BenchmarkResult {
    fn print(&self) {
        println!("\n📊 {} Results:", self.operation);
        println!("  Iterations: {}", self.iterations);
        println!("  Total Time: {:?}", self.total_time);
        println!("  Average: {:.3} ms", self.avg_time_ms);
        println!("  Min: {:.3} ms", self.min_time_ms);
        println!("  Max: {:.3} ms", self.max_time_ms);
        println!("  Throughput: {:.0} ops/sec", self.throughput);
    }
}

struct GeneBenchmark {
    pool: sqlx::PgPool,
}

impl GeneBenchmark {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("RUVECTOR_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Benchmark: Gene name exact lookup
    async fn bench_gene_lookup(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        // Get sample gene names
        let sample_genes: Vec<String> = sqlx::query("SELECT gene_name FROM genes LIMIT 100")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| row.get(0))
            .collect();

        if sample_genes.is_empty() {
            return Err("No genes in database".into());
        }

        let mut times = Vec::with_capacity(iterations);
        let start_total = Instant::now();

        for i in 0..iterations {
            let gene_name = &sample_genes[i % sample_genes.len()];

            let start = Instant::now();
            let _result: Option<(i32,)> = sqlx::query_as("SELECT id FROM genes WHERE gene_name = $1")
                .bind(gene_name)
                .fetch_optional(&self.pool)
                .await?;
            let elapsed = start.elapsed();

            times.push(elapsed);
        }

        let total_time = start_total.elapsed();

        Ok(Self::calculate_stats("Gene Name Lookup", times, total_time, iterations))
    }

    /// Benchmark: Gene type filtering
    async fn bench_gene_type_filter(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        let gene_types = vec!["protein_coding", "lncRNA", "miRNA", "pseudogene"];
        let mut times = Vec::with_capacity(iterations);
        let start_total = Instant::now();

        for i in 0..iterations {
            let gene_type = &gene_types[i % gene_types.len()];

            let start = Instant::now();
            let _result: Vec<(String,)> = sqlx::query_as(
                "SELECT gene_name FROM genes WHERE gene_type = $1 LIMIT 100"
            )
            .bind(gene_type)
            .fetch_all(&self.pool)
            .await?;
            let elapsed = start.elapsed();

            times.push(elapsed);
        }

        let total_time = start_total.elapsed();

        Ok(Self::calculate_stats("Gene Type Filter", times, total_time, iterations))
    }

    /// Benchmark: Chromosome range query
    async fn bench_chromosome_range(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        let mut times = Vec::with_capacity(iterations);
        let start_total = Instant::now();

        for i in 0..iterations {
            // Query different regions
            let start_pos = (i as i64 % 10) * 5_000_000;
            let end_pos = start_pos + 1_000_000;

            let start = Instant::now();
            let _result: Vec<(String,)> = sqlx::query_as(
                "SELECT gene_name FROM genes
                 WHERE chromosome = '22'
                 AND start_pos >= $1
                 AND end_pos <= $2"
            )
            .bind(start_pos)
            .bind(end_pos)
            .fetch_all(&self.pool)
            .await?;
            let elapsed = start.elapsed();

            times.push(elapsed);
        }

        let total_time = start_total.elapsed();

        Ok(Self::calculate_stats("Chromosome Range Query", times, total_time, iterations))
    }

    /// Benchmark: Full table scan with filtering
    async fn bench_full_scan(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        let mut times = Vec::with_capacity(iterations);
        let start_total = Instant::now();

        for _ in 0..iterations {
            let start = Instant::now();
            let _result: Vec<(i64,)> = sqlx::query_as(
                "SELECT COUNT(*) FROM genes WHERE end_pos - start_pos > 100000"
            )
            .fetch_all(&self.pool)
            .await?;
            let elapsed = start.elapsed();

            times.push(elapsed);
        }

        let total_time = start_total.elapsed();

        Ok(Self::calculate_stats("Full Table Scan (>100kb genes)", times, total_time, iterations))
    }

    /// Benchmark: Batch gene retrieval
    async fn bench_batch_retrieval(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        let mut times = Vec::with_capacity(iterations);
        let start_total = Instant::now();

        for _ in 0..iterations {
            let start = Instant::now();
            let _result: Vec<(String, String, String)> = sqlx::query_as(
                "SELECT gene_name, chromosome, gene_type FROM genes LIMIT 1000"
            )
            .fetch_all(&self.pool)
            .await?;
            let elapsed = start.elapsed();

            times.push(elapsed);
        }

        let total_time = start_total.elapsed();

        Ok(Self::calculate_stats("Batch Retrieval (1000 genes)", times, total_time, iterations))
    }

    /// Benchmark: Join with gene metadata (simulated)
    async fn bench_complex_query(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        let mut times = Vec::with_capacity(iterations);
        let start_total = Instant::now();

        for i in 0..iterations {
            let chr_num = (i % 22) + 1;
            let chromosome = chr_num.to_string();

            let start = Instant::now();
            let _result: Vec<(String, String, i64)> = sqlx::query_as(
                "SELECT gene_name, gene_type, (end_pos - start_pos) as length
                 FROM genes
                 WHERE chromosome = $1 AND gene_type = 'protein_coding'
                 ORDER BY start_pos
                 LIMIT 100"
            )
            .bind(&chromosome)
            .fetch_all(&self.pool)
            .await?;
            let elapsed = start.elapsed();

            times.push(elapsed);
        }

        let total_time = start_total.elapsed();

        Ok(Self::calculate_stats("Complex Query (filter + sort + limit)", times, total_time, iterations))
    }

    /// Benchmark: Insert performance
    async fn bench_insert(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        // Drop temp table if it exists, then create it
        let _ = sqlx::query("DROP TABLE IF EXISTS genes_test")
            .execute(&self.pool)
            .await;

        sqlx::query("CREATE TABLE genes_test AS SELECT * FROM genes LIMIT 0")
            .execute(&self.pool)
            .await?;

        let mut times = Vec::with_capacity(iterations);
        let start_total = Instant::now();

        for i in 0..iterations {
            let gene_id = format!("TEST{:06}", i);
            let gene_name = format!("TESTGENE{}", i);

            let start = Instant::now();
            let _result = sqlx::query(
                "INSERT INTO genes_test (gene_id, gene_name, chromosome, start_pos, end_pos, strand, gene_type)
                 VALUES ($1, $2, '22', 1000000, 2000000, '+', 'protein_coding')"
            )
            .bind(&gene_id)
            .bind(&gene_name)
            .execute(&self.pool)
            .await?;
            let elapsed = start.elapsed();

            times.push(elapsed);
        }

        let total_time = start_total.elapsed();

        // Cleanup
        sqlx::query("DROP TABLE genes_test")
            .execute(&self.pool)
            .await?;

        Ok(Self::calculate_stats("Single Gene Insert", times, total_time, iterations))
    }

    /// Benchmark: Concurrent queries
    async fn bench_concurrent_queries(&self, iterations: usize) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        use tokio::task::JoinSet;

        let start_total = Instant::now();

        let mut set = JoinSet::new();

        for _ in 0..iterations {
            let pool = self.pool.clone();
            set.spawn(async move {
                let start = Instant::now();
                let _result: Option<(String,)> = sqlx::query_as(
                    "SELECT gene_name FROM genes ORDER BY RANDOM() LIMIT 1"
                )
                .fetch_optional(&pool)
                .await.ok().flatten();
                start.elapsed()
            });
        }

        let mut times = Vec::new();
        while let Some(result) = set.join_next().await {
            if let Ok(elapsed) = result {
                times.push(elapsed);
            }
        }

        let total_time = start_total.elapsed();

        Ok(Self::calculate_stats("Concurrent Random Queries", times, total_time, iterations))
    }

    fn calculate_stats(operation: &str, mut times: Vec<Duration>, total_time: Duration, iterations: usize) -> BenchmarkResult {
        times.sort();

        let min_time = times.first().unwrap_or(&Duration::ZERO);
        let max_time = times.last().unwrap_or(&Duration::ZERO);
        let sum: Duration = times.iter().sum();
        let avg_time = sum.as_secs_f64() / iterations as f64;

        BenchmarkResult {
            operation: operation.to_string(),
            total_time,
            iterations,
            avg_time_ms: avg_time * 1000.0,
            min_time_ms: min_time.as_secs_f64() * 1000.0,
            max_time_ms: max_time.as_secs_f64() * 1000.0,
            throughput: iterations as f64 / total_time.as_secs_f64(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Omega-Longevity Performance Benchmark Suite");
    println!("===============================================\n");

    let benchmark = GeneBenchmark::new().await?;

    // Get database statistics
    println!("📊 Database Statistics:");
    let (total_genes,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM genes")
        .fetch_one(&benchmark.pool)
        .await?;
    println!("  Total genes: {}", total_genes);

    let (protein_coding,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM genes WHERE gene_type = 'protein_coding'"
    )
    .fetch_one(&benchmark.pool)
    .await?;
    println!("  Protein-coding: {}", protein_coding);

    // Check indexes
    let indexes: Vec<(String,)> = sqlx::query_as(
        "SELECT indexname FROM pg_indexes WHERE tablename = 'genes'"
    )
    .fetch_all(&benchmark.pool)
    .await?;
    println!("  Indexes: {}", indexes.len());
    for (index_name,) in indexes {
        println!("    - {}", index_name);
    }

    println!("\n🏁 Starting Benchmarks...\n");

    let iterations = 100;

    // Run all benchmarks
    let results = vec![
        benchmark.bench_gene_lookup(iterations).await?,
        benchmark.bench_gene_type_filter(iterations).await?,
        benchmark.bench_chromosome_range(iterations).await?,
        benchmark.bench_batch_retrieval(iterations).await?,
        benchmark.bench_complex_query(iterations).await?,
        benchmark.bench_full_scan(10).await?,  // Fewer iterations for expensive operation
        benchmark.bench_insert(100).await?,
        benchmark.bench_concurrent_queries(50).await?,
    ];

    // Print all results
    for result in &results {
        result.print();
    }

    // Summary
    println!("\n{}", "=".repeat(60));
    println!("📈 Performance Summary");
    println!("{}", "=".repeat(60));
    println!("\n{:<40} {:>10} {:>10}", "Operation", "Avg (ms)", "Ops/sec");
    println!("{}", "-".repeat(62));

    for result in &results {
        println!("{:<40} {:>10.3} {:>10.0}",
            result.operation,
            result.avg_time_ms,
            result.throughput
        );
    }

    // Optimization recommendations
    println!("\n💡 Optimization Recommendations:");

    for result in &results {
        if result.avg_time_ms > 50.0 {
            println!("  ⚠️  {}: Slow ({:.1} ms) - Consider indexing or query optimization",
                result.operation, result.avg_time_ms);
        } else if result.avg_time_ms > 10.0 {
            println!("  ⚡ {}: Moderate ({:.1} ms) - Acceptable for current workload",
                result.operation, result.avg_time_ms);
        } else {
            println!("  ✅ {}: Fast ({:.1} ms) - Well optimized",
                result.operation, result.avg_time_ms);
        }
    }

    println!("\n✨ Benchmark complete!");

    Ok(())
}
