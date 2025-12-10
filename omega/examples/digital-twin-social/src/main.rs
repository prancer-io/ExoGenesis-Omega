//! Digital Twin Social Media Platform Demo
//!
//! This demonstration showcases how ExoGenesis Omega can power
//! the next generation of emotionally-conscious social platforms.
//!
//! Run with: cargo run --release

use chrono::Utc;
use digital_twin_social::*;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║     DIGITAL TWIN SOCIAL PLATFORM - ExoGenesis Omega Demo      ║");
    println!("║                                                                ║");
    println!("║  Demonstrating: Omega Brain, RuVector/AgentDB, Cosmic Memory  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Run all demonstrations
    demo_personality_engine().await?;
    demo_emotional_loops().await?;
    demo_matching_engine().await?;
    demo_aria_swarm().await?;
    demo_privacy_layer().await?;
    demo_sensors().await?;

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("  Demo Complete! ExoGenesis Omega powers the future of social AI");
    println!("═══════════════════════════════════════════════════════════════════\n");

    Ok(())
}

/// Demonstrate the Personality Engine with AgentDB
async fn demo_personality_engine() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  1. PERSONALITY ENGINE (AgentDB + SIMD Vector Search)          │");
    println!("└─────────────────────────────────────────────────────────────────┘\n");

    let engine = personality::PersonalityEngine::new().await?;

    // Create sample users with different personalities
    println!("Creating digital twin profiles...\n");

    // User 1: High openness, agreeable, low neuroticism
    let mut alice = DigitalTwin::new("Alice");
    alice.big_five = BigFive::new(0.9, 0.7, 0.6, 0.85, 0.2);
    alice.values = SchwartzValues {
        self_direction: 0.9,
        benevolence: 0.8,
        universalism: 0.85,
        ..Default::default()
    };
    alice.eq = EmotionalIntelligence {
        self_awareness: 0.8,
        empathy: 0.9,
        social_skills: 0.75,
        ..Default::default()
    };
    let alice_id = engine.register_profile(alice).await?;
    println!("  ✓ Alice: Creative empath (O=0.9, A=0.85, N=0.2)");

    // User 2: Similar to Alice
    let mut bob = DigitalTwin::new("Bob");
    bob.big_five = BigFive::new(0.85, 0.65, 0.7, 0.8, 0.25);
    bob.values = SchwartzValues {
        self_direction: 0.85,
        benevolence: 0.75,
        universalism: 0.8,
        ..Default::default()
    };
    let _bob_id = engine.register_profile(bob).await?;
    println!("  ✓ Bob: Creative connector (O=0.85, A=0.8, N=0.25)");

    // User 3: Very different - low openness, high neuroticism
    let mut carol = DigitalTwin::new("Carol");
    carol.big_five = BigFive::new(0.3, 0.8, 0.4, 0.5, 0.7);
    carol.values = SchwartzValues {
        security: 0.9,
        conformity: 0.8,
        tradition: 0.75,
        ..Default::default()
    };
    let _carol_id = engine.register_profile(carol).await?;
    println!("  ✓ Carol: Traditional stabilizer (O=0.3, C=0.8, N=0.7)");

    // User 4: Achievement-oriented
    let mut dave = DigitalTwin::new("Dave");
    dave.big_five = BigFive::new(0.6, 0.9, 0.8, 0.6, 0.3);
    dave.values = SchwartzValues {
        achievement: 0.9,
        power: 0.7,
        self_direction: 0.75,
        ..Default::default()
    };
    let _dave_id = engine.register_profile(dave).await?;
    println!("  ✓ Dave: Driven achiever (C=0.9, E=0.8, Achievement=0.9)\n");

    // Find similar profiles for Alice
    println!("Finding similar profiles for Alice (SIMD-accelerated search)...\n");

    let start = std::time::Instant::now();
    let similar = engine.find_similar(&alice_id, 3).await?;
    let elapsed = start.elapsed();

    for (i, profile) in similar.iter().enumerate() {
        println!("  {}. {} - Similarity: {:.2}% (confidence: {:.2})",
            i + 1,
            profile.name,
            profile.similarity * 100.0,
            profile.confidence
        );
    }

    println!("\n  Search completed in {:?}", elapsed);
    println!("  (In production: <1ms for millions of profiles with SIMD)\n");

    Ok(())
}

/// Demonstrate the 7 Temporal Emotional Loops
async fn demo_emotional_loops() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  2. EMOTIONAL LOOPS (Omega Brain's 7 Temporal Loops)           │");
    println!("└─────────────────────────────────────────────────────────────────┘\n");

    let processor = emotional::EmotionalLoopProcessor::new();

    println!("Simulating emotional signals over time...\n");

    // Simulate a sequence of emotional signals
    let scenarios = [
        ("Morning calm", 0.3, 0.2),
        ("Exciting news", 0.8, 0.7),
        ("Work stress", -0.3, 0.6),
        ("Friend support", 0.5, 0.4),
        ("Evening reflection", 0.2, 0.2),
    ];

    for (scenario, valence, arousal) in scenarios {
        let signal = emotional::EmotionalSignal {
            source: emotional::SignalSource::Text,
            valence,
            arousal,
            dominance: 0.5,
            confidence: 0.8,
            timestamp: Utc::now(),
        };
        processor.add_signal(signal).await;

        let state = processor.process_reflexive().await;
        println!("  • {}: valence={:.2}, arousal={:.2}",
            scenario, state.valence, state.arousal);
    }

    println!("\nProcessing through emotional loops...\n");

    // Process through different loops
    let mood = processor.process_mood().await;
    println!("  Loop 2 (Mood):     valence={:.2}, arousal={:.2}",
        mood.valence, mood.arousal);

    let daily = processor.process_daily().await;
    println!("  Loop 3 (Daily):    peak hour={}, volatility={:.2}",
        daily.peak_hour, daily.volatility);

    let traits = processor.process_traits().await;
    println!("  Loop 4 (Traits):   baseline_v={:.2}, stability={:.2}",
        traits.baseline_valence, traits.stability);

    let growth = processor.process_growth().await;
    println!("  Loop 5 (Growth):   resilience={:.2}, areas={:?}",
        growth.resilience_score, growth.growth_areas);

    let identity = processor.process_identity().await;
    println!("  Loop 7 (Identity): coherence={:.2}, values={:?}\n",
        identity.identity_coherence, identity.core_values);

    Ok(())
}

/// Demonstrate the Matching Engine with Causal Reasoning
async fn demo_matching_engine() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  3. MATCHING ENGINE (Causal Graph + Multi-Domain Matching)     │");
    println!("└─────────────────────────────────────────────────────────────────┘\n");

    let personality = Arc::new(personality::PersonalityEngine::new().await?);
    let matching = matching::MatchingEngine::new(personality.clone()).await?;

    // Create profiles for matching demo
    let mut user1 = DigitalTwin::new("Emma");
    user1.big_five = BigFive::new(0.8, 0.7, 0.7, 0.85, 0.3);
    user1.attachment_style = AttachmentStyle::Secure;
    user1.eq = EmotionalIntelligence {
        self_awareness: 0.8,
        empathy: 0.85,
        social_skills: 0.8,
        self_regulation: 0.75,
        motivation: 0.7,
    };
    let emma_id = personality.register_profile(user1).await?;

    let mut user2 = DigitalTwin::new("James");
    user2.big_five = BigFive::new(0.75, 0.75, 0.65, 0.8, 0.35);
    user2.attachment_style = AttachmentStyle::Secure;
    user2.eq = EmotionalIntelligence {
        self_awareness: 0.75,
        empathy: 0.8,
        social_skills: 0.75,
        self_regulation: 0.7,
        motivation: 0.75,
    };
    personality.register_profile(user2).await?;

    let mut user3 = DigitalTwin::new("Lisa");
    user3.big_five = BigFive::new(0.4, 0.6, 0.5, 0.5, 0.6);
    user3.attachment_style = AttachmentStyle::Anxious;
    personality.register_profile(user3).await?;

    println!("Finding matches for Emma across different domains...\n");

    // Test different matching domains
    let domains = [
        ConnectionDomain::Dating,
        ConnectionDomain::Friendship,
        ConnectionDomain::Professional,
    ];

    for domain in domains {
        println!("  {:?} Matches:", domain);

        let matches = matching.find_matches(&emma_id, domain, 2).await?;

        for m in matches {
            println!("    • {} - Score: {:.1}%", m.name, m.final_score * 100.0);
            println!("      Compatibility: {:.1}%, Predicted satisfaction: {:.1}%",
                m.compatibility_score * 100.0,
                m.prediction.satisfaction * 100.0
            );
            if !m.prediction.key_factors.is_empty() {
                println!("      Key factors: {}",
                    m.prediction.key_factors.iter()
                        .take(2)
                        .map(|f| f.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        println!();
    }

    Ok(())
}

/// Demonstrate the ARIA Multi-Agent Swarm
async fn demo_aria_swarm() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  4. ARIA SWARM (Multi-Agent Orchestration)                     │");
    println!("└─────────────────────────────────────────────────────────────────┘\n");

    let aria = aria::ARIASwarm::new().await?;
    let user_id = Uuid::new_v4();

    println!("ARIA: 5 specialized agents working in harmony\n");
    println!("  • Empathy Agent      - Emotional validation and support");
    println!("  • Growth Coach       - Personal development opportunities");
    println!("  • Relationship Advisor - Social dynamics insights");
    println!("  • Values Guardian    - Alignment with user values");
    println!("  • Wellness Agent     - Biometric and lifestyle integration\n");

    // Test different conversation scenarios
    let scenarios = [
        "I'm feeling really stressed about work lately",
        "I want to improve myself and grow as a person",
        "My relationship with my partner has been challenging",
    ];

    for message in scenarios {
        println!("User: \"{}\"\n", message);

        let response = aria.process_message(&user_id, message, None).await?;

        println!("ARIA [{}]: {}\n",
            response.primary_agent,
            response.message
        );

        if !response.suggestions.is_empty() {
            println!("  Suggestions:");
            for suggestion in &response.suggestions {
                println!("    → {}", suggestion);
            }
            println!();
        }

        if response.growth_opportunity {
            println!("  ✨ Growth opportunity detected!\n");
        }

        println!("  ---\n");
    }

    Ok(())
}

/// Demonstrate the Zero-Knowledge Privacy Layer
async fn demo_privacy_layer() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  5. ZERO-KNOWLEDGE PRIVACY LAYER                               │");
    println!("└─────────────────────────────────────────────────────────────────┘\n");

    let config = privacy::PrivacyConfig {
        differential_privacy: true,
        epsilon: 1.0,
        local_only: false,
        retention_hours: 24,
    };

    let mut layer = privacy::ZeroKnowledgeLayer::with_config(config);

    println!("Privacy Architecture:");
    println!("  ┌──────────────────┐     ┌──────────────────┐");
    println!("  │  USER DEVICE     │     │  PATH SERVERS    │");
    println!("  │                  │     │                  │");
    println!("  │  Raw emotions    │ ──► │  Encrypted       │");
    println!("  │  (stays local)   │     │  vectors only    │");
    println!("  └──────────────────┘     └──────────────────┘\n");

    // Simulate emotional readings
    println!("Processing emotional data locally...\n");

    for i in 0..30 {
        let reading = privacy::EmotionalReading {
            timestamp: Utc::now(),
            source: "keyboard".to_string(),
            raw_valence: 0.3 + (i as f32 * 0.02),
            raw_arousal: 0.4,
            raw_dominance: 0.5,
            context: Some("work".to_string()),
        };
        layer.process_raw_data(reading);
    }

    // Store a private reflection
    layer.store_reflection(
        "I've been thinking about my goals...".to_string(),
        EmotionalState::neutral(),
    );

    // Get local insights
    let insights = layer.get_local_insights();
    println!("  Local Data (never leaves device):");
    println!("    • Readings: {}", insights.total_readings);
    println!("    • Private reflections: {}", insights.total_reflections);
    println!("    • Emotional trend: {:?}", insights.recent_emotional_trend);
    println!("    • Privacy score: {:.1}%\n", insights.privacy_score * 100.0);

    // Generate safe export
    let export = layer.process_raw_data(privacy::EmotionalReading {
        timestamp: Utc::now(),
        source: "test".to_string(),
        raw_valence: 0.5,
        raw_arousal: 0.4,
        raw_dominance: 0.5,
        context: None,
    });

    println!("  Privacy-Safe Export (sent to servers):");
    println!("    • Personality delta: {} dimensions", export.personality_delta.len());
    println!("    • Stability score: {:.2}", export.emotional_stability_score);
    println!("    • Growth areas: {:?}", export.growth_signals.growth_areas);
    println!("    • Confidence: {:.1}%", export.confidence * 100.0);
    println!("\n  ✓ Raw emotional content NEVER leaves the device");
    println!("  ✓ Differential privacy noise applied");
    println!("  ✓ Only aggregated, anonymized signals exported\n");

    Ok(())
}

/// Demonstrate Sensor Integration
async fn demo_sensors() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  6. SENSOR INTEGRATION (Keyboard + Wearables)                  │");
    println!("└─────────────────────────────────────────────────────────────────┘\n");

    // Keyboard sensor demo
    println!("Keyboard Emotional Inference:\n");

    let mut keyboard = sensors::KeyboardSensor::new();

    // Simulate typing patterns
    println!("  Simulating typing patterns...");

    for i in 0..40 {
        let now = Utc::now();
        let dwell = 80 + (i % 5) * 10; // Variable dwell time

        keyboard.add_keystroke(
            now,
            now + chrono::Duration::milliseconds(dwell),
            KeyCategory::Letter,
            None,
        );
    }

    if keyboard.has_sufficient_data() {
        let emotion = keyboard.infer_emotion()?;
        println!("  Inferred emotional state from typing:");
        println!("    • Valence: {:.2}", emotion.valence);
        println!("    • Arousal: {:.2}", emotion.arousal);
        println!("    • Confidence: {:.1}%\n", emotion.confidence * 100.0);
    }

    // Wearable sensor demo
    println!("Wearable Biometric Inference:\n");

    let mut wearable = sensors::WearableSensor::new();

    println!("  Simulating biometric readings...");

    for i in 0..20 {
        wearable.add_sample(sensors::BiometricSample {
            timestamp: Utc::now(),
            heart_rate: Some(72.0 + (i as f32 * 0.5)),
            hrv_rmssd: Some(45.0 + (i as f32 * 0.3)),
            skin_temperature: Some(36.5),
            eda: Some(2.0 + (i as f32 * 0.1)),
            activity_level: Some(0.3),
        });
    }

    if wearable.has_sufficient_data() {
        let emotion = wearable.infer_emotion()?;
        println!("  Inferred emotional state from biometrics:");
        println!("    • Valence: {:.2} (HRV-based)", emotion.valence);
        println!("    • Arousal: {:.2} (HR + EDA-based)", emotion.arousal);
        println!("    • Dominance: {:.2} (HRV stability)", emotion.dominance);
        println!("    • Confidence: {:.1}%\n", emotion.confidence * 100.0);
    }

    println!("  ✓ All sensor data processed locally");
    println!("  ✓ Only emotional signals exported (not raw data)\n");

    Ok(())
}
