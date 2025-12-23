// Database Connection Test for Omega-Longevity
// Tests connection to ruvector-postgres container

use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::test]
async fn test_database_connection() {
    // Load DATABASE_URL from environment or use default
    let database_url = env::var("RUVECTOR_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

    println!("Connecting to: {}", database_url.replace("longevity", "***"));

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Test basic query
    let row: (String,) = sqlx::query_as("SELECT 'Database connection successful!' as message")
        .fetch_one(&pool)
        .await
        .expect("Failed to execute test query");

    println!("{}", row.0);
    assert_eq!(row.0, "Database connection successful!");

    // Test ruvector extension
    let row: (String, String) = sqlx::query_as(
        "SELECT extname, extversion FROM pg_extension WHERE extname = 'ruvector'"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to query ruvector extension");

    println!("RuVector extension: {} v{}", row.0, row.1);
    assert_eq!(row.0, "ruvector");

    pool.close().await;
}

#[tokio::test]
async fn test_ruvector_functions() {
    let database_url = env::var("RUVECTOR_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Create a test table with ruvector embeddings
    sqlx::query(
        "CREATE TEMP TABLE test_vectors (
            id SERIAL PRIMARY KEY,
            vec ruvector(3)
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create temp table");

    // Test ruvector embedding generation (if model is available)
    // For now, just test that the extension is loaded properly
    let (ext_version,): (String,) = sqlx::query_as(
        "SELECT extversion FROM pg_extension WHERE extname = 'ruvector'"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to query ruvector version");

    println!("RuVector extension version: {}", ext_version);
    assert!(!ext_version.is_empty());

    pool.close().await;
}

#[tokio::test]
async fn test_create_gene_table() {
    let database_url = env::var("RUVECTOR_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://omega:longevity@localhost:5434/omega_longevity".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Create test table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS test_genes (
            id SERIAL PRIMARY KEY,
            gene_name VARCHAR(50) NOT NULL,
            embedding ruvector(384)
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create test table");

    println!("Test gene table created successfully");

    // Clean up
    sqlx::query("DROP TABLE IF EXISTS test_genes")
        .execute(&pool)
        .await
        .expect("Failed to drop test table");

    println!("Test table cleaned up");

    pool.close().await;
}
