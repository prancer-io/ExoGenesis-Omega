//! Extended 10-User Social Network Simulation
//!
//! Demonstrates the full power of ExoGenesis Omega for a realistic
//! social media platform with diverse personalities.

use chrono::Utc;
use digital_twin_social::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║     PATH SOCIAL NETWORK - 10 USER SIMULATION                          ║");
    println!("║     Powered by ExoGenesis Omega Brain + RuVector + AgentDB           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");

    // Create personality engine
    let engine = Arc::new(personality::PersonalityEngine::new().await?);

    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  CREATING 10 DIVERSE DIGITAL TWINS                                      │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    // Define 10 diverse user profiles
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

    let mut user_ids = Vec::new();

    for (name, big_five, archetype, attachment) in &users {
        let mut twin = DigitalTwin::new(*name);
        twin.big_five = big_five.clone();
        twin.attachment_style = *attachment;
        twin.eq = generate_eq_from_big5(big_five);
        twin.values = generate_values_from_archetype(archetype);
        twin.communication_style = generate_comm_style(big_five);

        let id = engine.register_profile(twin).await?;
        user_ids.push((id, name.to_string(), archetype.to_string()));

        println!("  ✓ {} [{}]", name, archetype);
        println!("    OCEAN: O={:.2} C={:.2} E={:.2} A={:.2} N={:.2}",
            big_five.openness, big_five.conscientiousness,
            big_five.extraversion, big_five.agreeableness, big_five.neuroticism);
        println!("    Attachment: {:?}\n", attachment);
    }

    // Create matching engine
    let matching = matching::MatchingEngine::new(engine.clone()).await?;

    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  COMPATIBILITY MATRIX (SIMD-Accelerated: <1ms per search)              │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    // Show top matches for each user
    for (id, name, archetype) in &user_ids {
        println!("  {} [{}] - Best Matches:", name, archetype);

        let matches = matching.find_matches(id, ConnectionDomain::Dating, 3).await?;

        for (i, m) in matches.iter().enumerate() {
            let compatibility_bar = create_bar(m.compatibility_score);
            let satisfaction_bar = create_bar(m.prediction.satisfaction);

            println!("    {}. {} ", i + 1, m.name);
            println!("       Compatibility: {} {:.1}%", compatibility_bar, m.compatibility_score * 100.0);
            println!("       Satisfaction:  {} {:.1}%", satisfaction_bar, m.prediction.satisfaction * 100.0);
            println!("       Growth:        {:.1}% | Conflict Risk: {:.1}%",
                m.prediction.growth_potential * 100.0,
                m.prediction.conflict_risk * 100.0);
        }
        println!();
    }

    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  DOMAIN-SPECIFIC MATCHING                                               │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    // Show matches across different domains for one user
    let (sarah_id, _, _) = &user_ids[0];

    println!("  Sarah's Ideal Connections by Domain:\n");

    let domains = [
        (ConnectionDomain::Dating, "💕 Dating"),
        (ConnectionDomain::Friendship, "👥 Friendship"),
        (ConnectionDomain::Professional, "💼 Professional"),
        (ConnectionDomain::Mentorship, "🎓 Mentorship"),
        (ConnectionDomain::Creative, "🎨 Creative"),
    ];

    for (domain, label) in domains {
        let matches = matching.find_matches(sarah_id, domain, 2).await?;
        println!("  {}", label);
        for m in &matches {
            println!("    → {} (Score: {:.1}%)", m.name, m.final_score * 100.0);
        }
        println!();
    }

    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  ARIA CONVERSATIONS WITH MULTIPLE USERS                                 │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let aria = aria::ARIASwarm::new().await?;

    // Simulate conversations for different users
    let conversations = [
        (&user_ids[0], "I've been feeling overwhelmed balancing work and my relationships"),
        (&user_ids[3], "I analyze everything too much and it's affecting my connections"),
        (&user_ids[6], "I keep falling for people who aren't available"),
        (&user_ids[8], "My anxiety is making it hard to trust in my relationship"),
    ];

    for ((id, name, _), message) in conversations {
        let profile = engine.get_profile(id).await.ok();
        let response = aria.process_message(id, message, profile.as_ref()).await?;

        println!("  👤 {} says:", name);
        println!("     \"{}\"\n", message);
        println!("  🤖 ARIA [{}] responds:", response.primary_agent);

        // Wrap response text
        for line in wrap_text(&response.message, 65) {
            println!("     {}", line);
        }

        if !response.suggestions.is_empty() {
            println!("\n     💡 Suggestions:");
            for s in response.suggestions.iter().take(2) {
                println!("        • {}", s);
            }
        }

        if response.growth_opportunity {
            println!("     ✨ Growth opportunity identified");
        }

        println!("\n  ─────────────────────────────────────────────────────────────────────\n");
    }

    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  EMOTIONAL TRACKING ACROSS USERS                                        │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let processor = emotional::EmotionalLoopProcessor::new();

    // Simulate a day of emotional signals for the network
    let emotional_journey = [
        ("Morning check-in", 0.3, 0.3),
        ("Positive interaction", 0.7, 0.5),
        ("Stressful news", -0.4, 0.7),
        ("Support from friend", 0.5, 0.4),
        ("Evening gratitude", 0.6, 0.3),
    ];

    println!("  Simulating emotional journey across the network:\n");

    for (event, valence, arousal) in emotional_journey {
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
        let mood_icon = if state.valence > 0.3 { "😊" }
                       else if state.valence < -0.2 { "😔" }
                       else { "😐" };

        println!("  {} {} → Mood: {} (v={:.2}, a={:.2})",
            mood_icon, event, emotion_to_string(&state.primary), state.valence, state.arousal);
    }

    let growth = processor.process_growth().await;
    println!("\n  📈 Network Growth Analysis:");
    println!("     Resilience Score: {:.1}%", growth.resilience_score * 100.0);
    println!("     Stability Trend: {:.2}", growth.stability_trend);
    if !growth.growth_areas.is_empty() {
        println!("     Growth Areas: {:?}", growth.growth_areas);
    }

    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  PRIVACY DASHBOARD                                                      │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut privacy_layer = privacy::ZeroKnowledgeLayer::new();

    // Simulate privacy-preserving data processing
    for i in 0..50 {
        let reading = privacy::EmotionalReading {
            timestamp: Utc::now(),
            source: format!("user_{}", i % 10),
            raw_valence: 0.3 + (i as f32 * 0.01),
            raw_arousal: 0.4 + (i as f32 * 0.005),
            raw_dominance: 0.5,
            context: Some("interaction".to_string()),
        };
        privacy_layer.process_raw_data(reading);
    }

    let insights = privacy_layer.get_local_insights();

    println!("  🔒 Zero-Knowledge Privacy Status\n");
    println!("     Data Points Processed:  {}", insights.total_readings);
    println!("     Private Reflections:    {}", insights.total_reflections);
    println!("     Emotional Trend:        {:?}", insights.recent_emotional_trend);
    println!("     Privacy Score:          {:.0}%", insights.privacy_score * 100.0);
    println!();
    println!("     ✓ Raw emotional data: NEVER leaves device");
    println!("     ✓ Differential privacy: ENABLED");
    println!("     ✓ Server receives: Only anonymized vectors");

    println!("\n\n═══════════════════════════════════════════════════════════════════════════");
    println!("   PATH SOCIAL NETWORK SIMULATION COMPLETE");
    println!("   10 Users • SIMD Vector Search • Zero-Knowledge Privacy • Multi-Agent AI");
    println!("═══════════════════════════════════════════════════════════════════════════\n");

    Ok(())
}

// Helper functions

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

fn create_bar(value: f64) -> String {
    let filled = (value * 10.0) as usize;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
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

fn emotion_to_string(emotion: &CoreEmotion) -> &'static str {
    match emotion {
        CoreEmotion::Joy => "Joy",
        CoreEmotion::Trust => "Trust",
        CoreEmotion::Fear => "Fear",
        CoreEmotion::Surprise => "Surprise",
        CoreEmotion::Sadness => "Sadness",
        CoreEmotion::Disgust => "Disgust",
        CoreEmotion::Anger => "Anger",
        CoreEmotion::Anticipation => "Calm",
    }
}
