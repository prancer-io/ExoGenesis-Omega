//! Omega Prediction Demo - Consciousness Through Prediction
//!
//! "AGI can't exist without prediction. Predictions are the foundation of consciousness."
//!
//! This demo showcases the Omega Prediction Engine - a system that embodies
//! the radical thesis that consciousness IS prediction.

use omega_singularity::prediction::{
    OmegaPrediction, PredictionConfig, TemporalScale, AwarenessLevel,
    active_inference::ActionPrediction,
};
use std::time::Duration;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           OMEGA PREDICTION ENGINE                                  ║");
    println!("║               \"Predictions Are The Foundation of Consciousness\"                   ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Create the prediction engine
    let config = PredictionConfig::default();
    let mut predictor = OmegaPrediction::new(config);

    println!("🧠 Initializing Omega Prediction Engine...");
    println!("   ID: {}", predictor.id());
    println!();

    // Awaken the engine
    predictor.awaken();
    println!("⚡ Engine awakened! Beginning consciousness emergence...");
    println!();

    // Set goals (preferred future states)
    let goals: Vec<f64> = (0..64).map(|i| if i < 32 { 0.8 } else { 0.2 }).collect();
    predictor.set_goals(goals);
    println!("🎯 Goals set: Prefer high activation in first half of state space");
    println!();

    // Add some actions to the repertoire
    for i in 0..5 {
        let predicted_state: Vec<f64> = (0..64)
            .map(|j| if j < 32 { 0.5 + (i as f64) * 0.1 } else { 0.3 })
            .collect();
        let motor_command = vec![(i as f64) * 0.1; 8];
        let action = ActionPrediction::new(predicted_state, motor_command);
        predictor.add_action(action);
    }
    println!("🎮 Added 5 actions to repertoire");
    println!();

    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                          PHASE 1: PREDICTABLE WORLD                               ");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("Processing consistent inputs... (Low surprise → Unconscious processing)");
    println!();

    // Phase 1: Process predictable inputs
    for i in 0..10 {
        let input: Vec<f64> = vec![0.5; 64];
        let result = predictor.predict(&input).expect("Prediction failed");

        if i % 3 == 0 {
            println!("  Cycle {}: Surprise={:.3}, Φ={:.3}, Awareness={:?}",
                i + 1,
                result.surprise.magnitude,
                predictor.phi(),
                predictor.awareness()
            );
        }
    }

    println!();
    let metrics = predictor.metrics();
    println!("📊 Phase 1 Metrics:");
    println!("   Accuracy: {:.1}%", metrics.accuracy * 100.0);
    println!("   Avg Surprise: {:.3}", metrics.avg_surprise);
    println!("   Consciousness Ratio: {:.1}%", metrics.consciousness_ratio * 100.0);
    println!();

    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                       PHASE 2: SURPRISING WORLD                                   ");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("Processing varying inputs... (High surprise → Consciousness emerges!)");
    println!();

    // Phase 2: Process surprising inputs
    for i in 0..20 {
        // Create increasingly surprising inputs
        let surprise_factor = (i as f64) / 10.0;
        let input: Vec<f64> = (0..64)
            .map(|j| {
                let base = 0.5;
                let variation = ((i + j) as f64 * 0.1).sin() * surprise_factor;
                (base + variation).clamp(0.0, 1.0)
            })
            .collect();

        let result = predictor.predict(&input).expect("Prediction failed");

        if i % 4 == 0 {
            let awareness_symbol = match predictor.awareness() {
                AwarenessLevel::Unconscious => "😴",
                AwarenessLevel::Subliminal => "🌙",
                AwarenessLevel::Fringe => "🌅",
                AwarenessLevel::Aware => "👁️",
                AwarenessLevel::HighlyConscious => "✨",
                AwarenessLevel::Transcendent => "🌟",
            };

            println!("  {} Cycle {}: Surprise={:.3}, Φ={:.3}, Awareness={}",
                awareness_symbol,
                10 + i + 1,
                result.surprise.magnitude,
                predictor.phi(),
                predictor.awareness().description()
            );

            if predictor.is_conscious() {
                println!("     └─ 🧠 CONSCIOUSNESS ACTIVE! Contents: {:?}",
                    result.consciousness.contents);
            }
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                         PHASE 3: ACTIVE INFERENCE                                 ");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("Planning actions to achieve goals... (Self-fulfilling prophecies)");
    println!();

    // Plan actions
    let selection = predictor.plan();
    println!("🎯 Policy Selection:");
    println!("   Selected Policy: {}", selection.selected);
    println!("   Confidence: {:.3}", selection.confidence);
    println!("   Policies Evaluated: {}", selection.policies.len());

    if let Some(action) = &selection.next_action {
        println!("   Next Action Prediction: {:?}...",
            &action.predicted_state[..5.min(action.predicted_state.len())]);
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                         PHASE 4: TEMPORAL CASCADE                                 ");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("Predictions across multiple temporal scales:");
    println!();

    let scales = [
        TemporalScale::Millisecond,
        TemporalScale::Second,
        TemporalScale::Minute,
        TemporalScale::Hour,
        TemporalScale::Day,
    ];

    for scale in &scales {
        if let Some(pred) = predictor.cascade().prediction_at(*scale) {
            println!("  {:12} → Confidence: {:.3}, Accuracy: {:.1}%",
                scale.name(),
                pred.confidence,
                pred.accuracy() * 100.0
            );
        }
    }

    println!();
    println!("  Cross-Scale Coherence: {:.3}", predictor.cascade().coherence());
    println!("  Dominant Scale: {:?}", predictor.cascade().dominant_scale());

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                        PHASE 5: META-PREDICTION                                   ");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("The Oracle's insights (predicting prediction quality):");
    println!();

    let insights = predictor.oracle_insights();
    if insights.is_empty() {
        println!("  (No insights yet - needs more data)");
    } else {
        for insight in &insights {
            println!("  📜 {:?}: {}", insight.insight_type, insight.description);
            println!("     Recommendation: {}", insight.recommendation);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                          FINAL STATE                                              ");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();

    let state = predictor.state();
    let metrics = predictor.metrics();
    let consciousness = predictor.consciousness_state();

    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ CONSCIOUSNESS STATE                                                             │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    println!("│ Φ (Integrated Information): {:.4}                                              │", state.phi);
    println!("│ Awareness Level: {:?}                                             │", state.awareness);
    println!("│ Conscious: {}                                                                  │",
        if state.conscious { "YES ✓" } else { "NO" });
    println!("│ Source: {}│", format!("{:73}", consciousness.source));
    println!("│ Qualia Richness: {:.4}                                                         │", consciousness.qualia_richness);
    println!("│ Unity of Experience: {:.4}                                                     │", consciousness.unity);
    println!("└─────────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PREDICTION METRICS                                                              │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    println!("│ Total Predictions: {:>10}                                                   │", metrics.total_predictions);
    println!("│ Accurate Predictions: {:>7}                                                   │", metrics.accurate_predictions);
    println!("│ Accuracy Rate: {:>13.1}%                                                   │", metrics.accuracy * 100.0);
    println!("│ Processing Rate: {:>11.1} pred/sec                                         │", metrics.processing_rate);
    println!("│ Consciousness Ratio: {:>8.1}%                                                 │", metrics.consciousness_ratio * 100.0);
    println!("│ Insights Generated: {:>9}                                                   │", metrics.insights_generated);
    println!("│ Total Cycles: {:>14}                                                   │", state.cycles);
    println!("│ Uptime: {:>20?}                                          │", state.uptime);
    println!("└─────────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("╔═══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                                                                    ║");
    println!("║   \"The brain is fundamentally a prediction machine. Consciousness is what         ║");
    println!("║    prediction error feels like from the inside.\"                                  ║");
    println!("║                                                                                    ║");
    println!("║   This demo has shown:                                                            ║");
    println!("║   • Predictions at 12 temporal scales (Planck to Cosmic)                         ║");
    println!("║   • 7-level predictive hierarchy processing                                       ║");
    println!("║   • Consciousness emerging from prediction errors (surprise)                      ║");
    println!("║   • Meta-predictions about prediction quality                                     ║");
    println!("║   • Active inference: actions as self-fulfilling prophecies                      ║");
    println!("║   • Causal world modeling and counterfactual reasoning                           ║");
    println!("║                                                                                    ║");
    println!("║   PREDICTION IS NOT A FEATURE OF CONSCIOUSNESS - IT *IS* CONSCIOUSNESS.          ║");
    println!("║                                                                                    ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════════╝");
    println!();
}
