//! PostgreSQL + RuVector Integration Demo
//!
//! This example demonstrates the full PATH Social platform running on
//! RuVector-PostgreSQL with SIMD-accelerated vector search.
//!
//! ## Prerequisites
//!
//! 1. Start the database: `docker-compose up -d`
//! 2. Run the demo: `cargo run --example postgres_demo`
//!
//! ## Features Demonstrated
//!
//! - Persistent digital twin storage
//! - SIMD-accelerated personality matching via HNSW indexes
//! - Emotional signal time-series storage
//! - ARIA conversation persistence
//! - Causal graph for relationship predictions

use digital_twin_social::postgres::*;
use digital_twin_social::*;
use uuid::Uuid;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║     PATH SOCIAL NETWORK - RUVECTOR POSTGRESQL DEMO                    ║");
    println!("║     SIMD-Accelerated Vector Search • HNSW Indexing • Persistent       ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");

    // Connect to PostgreSQL
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  CONNECTING TO RUVECTOR-POSTGRES                                        │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let config = PostgresConfig::from_env();
    println!("  Database URL: {}", mask_password(&config.database_url));
    println!("  Embedding Dimension: {}", config.embedding_dimension);
    println!("  Max Connections: {}\n", config.max_connections);

    let backend = match PostgresBackend::connect(config).await {
        Ok(b) => {
            println!("  ✓ Connected to RuVector-PostgreSQL successfully!\n");
            b
        }
        Err(e) => {
            println!("  ✗ Failed to connect: {}", e);
            println!("\n  💡 Make sure the database is running:");
            println!("     docker-compose up -d ruvector-postgres\n");
            return Ok(());
        }
    };

    // Create digital twins
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  CREATING DIGITAL TWINS WITH 4096-DIM EMBEDDINGS                        │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let users = vec![
        ("Sarah", BigFive::new(0.85, 0.75, 0.80, 0.90, 0.20), "Creative Empath", AttachmentStyle::Secure),
        ("Michael", BigFive::new(0.70, 0.90, 0.75, 0.65, 0.30), "Driven Achiever", AttachmentStyle::Secure),
        ("Emma", BigFive::new(0.90, 0.60, 0.85, 0.80, 0.25), "Social Butterfly", AttachmentStyle::Secure),
        ("David", BigFive::new(0.40, 0.85, 0.45, 0.70, 0.40), "Analytical Mind", AttachmentStyle::Avoidant),
        ("Olivia", BigFive::new(0.75, 0.70, 0.65, 0.95, 0.15), "Nurturing Soul", AttachmentStyle::Secure),
        ("James", BigFive::new(0.65, 0.80, 0.90, 0.55, 0.35), "Bold Leader", AttachmentStyle::Secure),
        ("Sophia", BigFive::new(0.95, 0.55, 0.70, 0.85, 0.30), "Free Spirit", AttachmentStyle::Anxious),
        ("William", BigFive::new(0.50, 0.95, 0.40, 0.75, 0.25), "Steady Rock", AttachmentStyle::Secure),
        ("Ava", BigFive::new(0.80, 0.65, 0.75, 0.70, 0.45), "Complex Dreamer", AttachmentStyle::Anxious),
        ("Alexander", BigFive::new(0.60, 0.75, 0.85, 0.60, 0.20), "Charismatic Visionary", AttachmentStyle::Secure),
    ];

    let mut twin_ids: Vec<(Uuid, String, String)> = Vec::new();

    for (name, big_five, archetype, attachment) in &users {
        let mut twin = DigitalTwin::new(*name);
        twin.big_five = big_five.clone();
        twin.attachment_style = *attachment;
        twin.eq = generate_eq_from_big5(big_five);
        twin.values = generate_values_from_archetype(archetype);
        twin.communication_style = generate_comm_style(big_five);

        // Generate 4096-dimensional embedding from personality
        twin.deep_embedding = generate_deep_embedding(&twin);

        match backend.store_twin_with_archetype(&twin, Some(archetype)).await {
            Ok(id) => {
                twin_ids.push((id, name.to_string(), archetype.to_string()));
                println!("  ✓ {} [{}] → ID: {}", name, archetype, id);
            }
            Err(e) => {
                println!("  ✗ Failed to store {}: {}", name, e);
            }
        }
    }

    println!();

    // Demonstrate SIMD-accelerated vector search
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  SIMD-ACCELERATED PERSONALITY MATCHING (HNSW Index)                     │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    if let Some((sarah_id, _, _)) = twin_ids.first() {
        println!("  Finding matches for Sarah (Creative Empath):\n");

        let start = std::time::Instant::now();
        match backend.find_similar_twins(sarah_id, 5).await {
            Ok(matches) => {
                let elapsed = start.elapsed();
                println!("  ⚡ Search completed in {:?} (SIMD-accelerated)\n", elapsed);

                for (i, m) in matches.iter().enumerate() {
                    let bar = create_similarity_bar(m.similarity);
                    println!("  {}. {} {}", i + 1, m.name, bar);
                    println!("     Similarity: {:.1}%", m.similarity * 100.0);
                    if let Some(ref arch) = m.archetype {
                        println!("     Archetype: {}", arch);
                    }
                    println!();
                }
            }
            Err(e) => println!("  ✗ Search failed: {}", e),
        }
    }

    // Store compatibility scores
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  STORING COMPATIBILITY SCORES WITH CAUSAL REASONING                     │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    if twin_ids.len() >= 2 {
        let (sarah_id, _, _) = &twin_ids[0];
        let (olivia_id, _, _) = &twin_ids[4];

        let score = CompatibilityScore {
            compatibility: 0.92,
            satisfaction: 0.88,
            longevity: 0.85,
            growth_potential: 0.90,
            conflict_risk: 0.15,
            factors: vec![
                "High value alignment".to_string(),
                "Complementary communication styles".to_string(),
                "Compatible attachment styles".to_string(),
            ],
            confidence: 0.95,
        };

        match backend.store_compatibility(sarah_id, olivia_id, "friendship", &score).await {
            Ok(_) => {
                println!("  ✓ Stored Sarah ↔ Olivia friendship compatibility");
                println!("     Compatibility: {:.1}%", score.compatibility * 100.0);
                println!("     Satisfaction Prediction: {:.1}%", score.satisfaction * 100.0);
                println!("     Conflict Risk: {:.1}%\n", score.conflict_risk * 100.0);
            }
            Err(e) => println!("  ✗ Failed to store compatibility: {}", e),
        }
    }

    // Store emotional signals
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  STORING EMOTIONAL SIGNALS (Time-Series Data)                           │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    if let Some((sarah_id, _, _)) = twin_ids.first() {
        let signals = vec![
            ("Morning check-in", 0.3, 0.3, "text"),
            ("Positive interaction", 0.7, 0.5, "interaction"),
            ("Stressful news", -0.4, 0.7, "text"),
            ("Support from friend", 0.5, 0.4, "interaction"),
            ("Evening gratitude", 0.6, 0.3, "text"),
        ];

        println!("  Storing emotional journey for Sarah:\n");

        for (event, valence, arousal, source) in signals {
            let signal = EmotionalSignalRecord {
                source: source.to_string(),
                valence: valence as f32,
                arousal: arousal as f32,
                dominance: 0.5,
                confidence: 0.8,
                embedding: vec![], // Would normally use an embedding model
                context: Some(serde_json::json!({"event": event})),
            };

            let mood = if valence > 0.3 { "😊" } else if valence < -0.2 { "😔" } else { "😐" };

            match backend.store_emotional_signal(sarah_id, &signal).await {
                Ok(_) => println!("  {} {} (v={:.1}, a={:.1})", mood, event, valence, arousal),
                Err(e) => println!("  ✗ Failed: {}", e),
            }
        }
        println!();
    }

    // Store ARIA conversation
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  STORING ARIA CONVERSATION                                              │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    if let Some((sarah_id, _, _)) = twin_ids.first() {
        let session_id = Uuid::new_v4();
        let conversation = ConversationRecord {
            user_message: "I've been feeling overwhelmed balancing work and relationships".to_string(),
            aria_response: "It sounds like you're carrying a lot right now. Finding balance between work and relationships is something many of us struggle with. What feels most overwhelming at this moment?".to_string(),
            primary_agent: "EmpathyAgent".to_string(),
            agent_contributions: vec![
                "EmpathyAgent: Acknowledged emotional state".to_string(),
                "GrowthCoachAgent: Identified growth opportunity".to_string(),
            ],
            suggestions: vec![
                "Try time-blocking for relationship maintenance".to_string(),
                "Consider setting boundaries at work".to_string(),
            ],
            growth_opportunity: true,
            emotional_tone: Some(serde_json::json!({
                "valence": 0.3,
                "arousal": 0.5,
                "primary_emotion": "trust"
            })),
            message_embedding: vec![], // Would use embedding model
        };

        match backend.store_conversation(sarah_id, &session_id, &conversation).await {
            Ok(id) => {
                println!("  👤 Sarah: \"{}\"", conversation.user_message);
                println!();
                println!("  🤖 ARIA [{}]: ", conversation.primary_agent);
                for line in wrap_text(&conversation.aria_response, 60) {
                    println!("     {}", line);
                }
                println!();
                println!("  💡 Suggestions:");
                for s in &conversation.suggestions {
                    println!("     • {}", s);
                }
                println!("\n  ✓ Conversation stored with ID: {}\n", id);
            }
            Err(e) => println!("  ✗ Failed to store conversation: {}", e),
        }
    }

    // Store causal edge
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  BUILDING CAUSAL GRAPH                                                  │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let causal_edges = vec![
        CausalEdgeRecord {
            cause: "high_agreeableness_pair".to_string(),
            effect: "lower_conflict_rate".to_string(),
            uplift: 0.45,
            confidence: 0.92,
            sample_size: 1500,
            first_observed: Utc::now(),
            last_observed: Utc::now(),
        },
        CausalEdgeRecord {
            cause: "complementary_communication".to_string(),
            effect: "higher_satisfaction".to_string(),
            uplift: 0.38,
            confidence: 0.88,
            sample_size: 2200,
            first_observed: Utc::now(),
            last_observed: Utc::now(),
        },
        CausalEdgeRecord {
            cause: "secure_attachment_both".to_string(),
            effect: "relationship_longevity".to_string(),
            uplift: 0.52,
            confidence: 0.95,
            sample_size: 3100,
            first_observed: Utc::now(),
            last_observed: Utc::now(),
        },
    ];

    for edge in &causal_edges {
        match backend.upsert_causal_edge(edge).await {
            Ok(_) => {
                println!("  {} → {} (uplift: +{:.0}%, confidence: {:.0}%)",
                    edge.cause, edge.effect,
                    edge.uplift * 100.0, edge.confidence * 100.0);
            }
            Err(e) => println!("  ✗ Failed: {}", e),
        }
    }
    println!();

    // Database statistics
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  DATABASE STATISTICS                                                    │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    match backend.get_stats().await {
        Ok(stats) => {
            println!("  📊 RuVector-PostgreSQL Statistics:\n");
            println!("     Digital Twins:        {}", stats.digital_twins);
            println!("     Emotional Signals:    {}", stats.emotional_signals);
            println!("     Conversations:        {}", stats.conversations);
            println!("     Compatibility Scores: {}", stats.compatibility_scores);
            println!("     Causal Edges:         {}", stats.causal_edges);
        }
        Err(e) => println!("  ✗ Failed to get stats: {}", e),
    }

    println!("\n═══════════════════════════════════════════════════════════════════════════");
    println!("   RUVECTOR-POSTGRESQL DEMO COMPLETE");
    println!("   SIMD Vector Search • HNSW Indexing • Persistent Storage");
    println!("═══════════════════════════════════════════════════════════════════════════\n");

    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn generate_eq_from_big5(big5: &BigFive) -> EmotionalIntelligence {
    EmotionalIntelligence {
        self_awareness: 0.5 + big5.openness * 0.3 - big5.neuroticism * 0.2,
        self_regulation: 0.4 + big5.conscientiousness * 0.4 - big5.neuroticism * 0.2,
        motivation: 0.5 + big5.conscientiousness * 0.3 + big5.extraversion * 0.1,
        empathy: 0.4 + big5.agreeableness * 0.4 + big5.openness * 0.1,
        social_skills: 0.4 + big5.extraversion * 0.3 + big5.agreeableness * 0.2,
    }
}

fn generate_values_from_archetype(archetype: &str) -> SchwartzValues {
    match archetype {
        "Creative Empath" => SchwartzValues {
            universalism: 0.9, benevolence: 0.85, self_direction: 0.8,
            ..Default::default()
        },
        "Driven Achiever" => SchwartzValues {
            achievement: 0.9, power: 0.7, self_direction: 0.75,
            ..Default::default()
        },
        "Social Butterfly" => SchwartzValues {
            benevolence: 0.85, stimulation: 0.8, hedonism: 0.7,
            ..Default::default()
        },
        "Analytical Mind" => SchwartzValues {
            self_direction: 0.9, achievement: 0.7, security: 0.65,
            ..Default::default()
        },
        "Nurturing Soul" => SchwartzValues {
            benevolence: 0.95, universalism: 0.85, security: 0.7,
            ..Default::default()
        },
        "Bold Leader" => SchwartzValues {
            power: 0.8, achievement: 0.85, self_direction: 0.75,
            ..Default::default()
        },
        "Free Spirit" => SchwartzValues {
            self_direction: 0.95, stimulation: 0.85, universalism: 0.7,
            ..Default::default()
        },
        "Steady Rock" => SchwartzValues {
            security: 0.9, conformity: 0.8, tradition: 0.7,
            ..Default::default()
        },
        "Complex Dreamer" => SchwartzValues {
            self_direction: 0.85, universalism: 0.8, stimulation: 0.65,
            ..Default::default()
        },
        _ => SchwartzValues {
            achievement: 0.8, self_direction: 0.75, benevolence: 0.7,
            ..Default::default()
        },
    }
}

fn generate_comm_style(big5: &BigFive) -> CommunicationStyle {
    CommunicationStyle {
        directness: 0.3 + big5.extraversion * 0.4,
        expressiveness: 0.3 + big5.extraversion * 0.3 + big5.openness * 0.2,
        formality: 0.7 - big5.openness * 0.3,
        conflict_approach: 0.3 + big5.extraversion * 0.2 - big5.agreeableness * 0.1,
        listening_speaking: big5.extraversion,
        emotional_logical: 0.5 - big5.openness * 0.2 + big5.conscientiousness * 0.2,
    }
}

/// Generate a 4096-dimensional embedding from personality traits
fn generate_deep_embedding(twin: &DigitalTwin) -> Vec<f32> {
    let mut embedding = vec![0.0f32; 4096];

    // Encode Big Five (first 512 dimensions)
    let big5 = &twin.big_five;
    for i in 0..512 {
        let trait_idx = i % 5;
        let variation = (i as f32 / 512.0) * std::f32::consts::PI;
        let trait_val = match trait_idx {
            0 => big5.openness,
            1 => big5.conscientiousness,
            2 => big5.extraversion,
            3 => big5.agreeableness,
            _ => big5.neuroticism,
        };
        embedding[i] = trait_val * variation.sin() + (1.0 - trait_val) * variation.cos();
    }

    // Encode Schwartz Values (next 1024 dimensions)
    let values = twin.values.to_vector();
    for i in 0..1024 {
        let val_idx = i % values.len();
        let phase = (i as f32 / 1024.0) * 2.0 * std::f32::consts::PI;
        embedding[512 + i] = values[val_idx] * phase.cos();
    }

    // Encode EQ (next 512 dimensions)
    let eq = &twin.eq;
    let eq_vec = vec![eq.self_awareness, eq.self_regulation, eq.motivation, eq.empathy, eq.social_skills];
    for i in 0..512 {
        let eq_idx = i % 5;
        let phase = (i as f32 / 512.0) * std::f32::consts::PI;
        embedding[1536 + i] = eq_vec[eq_idx] * phase.sin();
    }

    // Encode Communication Style (next 512 dimensions)
    let comm = &twin.communication_style;
    let comm_vec = vec![
        comm.directness, comm.expressiveness, comm.formality,
        comm.conflict_approach, comm.listening_speaking, comm.emotional_logical
    ];
    for i in 0..512 {
        let comm_idx = i % 6;
        let phase = (i as f32 / 512.0) * std::f32::consts::PI;
        embedding[2048 + i] = comm_vec[comm_idx] * (phase + 0.5).cos();
    }

    // Attachment style encoding (next 256 dimensions)
    let attachment_val = match twin.attachment_style {
        AttachmentStyle::Secure => 0.9,
        AttachmentStyle::Anxious => 0.5,
        AttachmentStyle::Avoidant => 0.3,
        AttachmentStyle::Disorganized => 0.1,
    };
    for i in 0..256 {
        let phase = (i as f32 / 256.0) * std::f32::consts::PI;
        embedding[2560 + i] = attachment_val * phase.sin();
    }

    // Interaction terms (remaining dimensions)
    for i in 2816..4096 {
        let idx1 = i % 512;
        let idx2 = (i * 7) % 1024;
        embedding[i] = embedding[idx1] * embedding[512 + idx2] * 0.5;
    }

    // Normalize the embedding
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut embedding {
            *x /= norm;
        }
    }

    embedding
}

fn create_similarity_bar(value: f32) -> String {
    let filled = (value * 10.0) as usize;
    let empty = 10 - filled;
    format!("[{}{}] {:.1}%", "█".repeat(filled), "░".repeat(empty), value * 100.0)
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > width {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn mask_password(url: &str) -> String {
    // Simple password masking for display
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let prefix = &url[..colon_pos + 1];
            let suffix = &url[at_pos..];
            return format!("{}****{}", prefix, suffix);
        }
    }
    url.to_string()
}
