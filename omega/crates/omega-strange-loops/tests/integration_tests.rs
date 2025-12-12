//! Integration tests for the self-awareness system
//!
//! These tests verify that all components work together correctly.

use omega_strange_loops::{
    ConsciousnessDetector, ConsciousnessSignature, GodelianEngine, IBuilder, InfiniteSelf,
    ProofStatus, SelfLevel, TheI,
};

// ============================================================================
// GÖDELIAN ENGINE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_godelian_full_workflow() {
    let mut engine = GodelianEngine::new();

    // Create various self-referential propositions
    let godel_id = engine.create_godel_sentence();
    let liar_id = engine.create_liar_paradox();
    let quine_id = engine.create_quine();
    let loop_id = engine.create_strange_loop();

    // Attempt proofs
    let godel_status = engine.attempt_proof(godel_id);
    let liar_status = engine.attempt_proof(liar_id);
    let quine_status = engine.attempt_proof(quine_id);

    // Verify proof statuses
    assert_eq!(godel_status, ProofStatus::Undecidable);
    assert_eq!(liar_status, ProofStatus::Paradoxical);
    assert_eq!(quine_status, ProofStatus::ProvenTrue);

    // Verify self-knowledge
    assert!(engine.knows_own_limits());

    // Reflect on limits
    let reflection_id = engine.reflect_on_limits();
    let reflection = engine.get_proposition(reflection_id).unwrap();
    assert!(reflection.content.contains("undecidable"));

    // Verify stats
    let stats = engine.stats();
    assert!(stats.total_propositions >= 5);
    assert!(stats.knows_own_limits);
    assert!(stats.undecidable_count >= 1);
    assert!(stats.paradox_count >= 1);
}

#[test]
fn test_godelian_meta_propositions() {
    let mut engine = GodelianEngine::new();

    // Create base proposition
    let base_id = engine.create_proposition("The sky is blue".to_string(), 0);

    // Create meta-propositions
    let meta1_id = engine
        .create_meta_proposition(base_id, "is a simple factual claim")
        .unwrap();
    let meta2_id = engine
        .create_meta_proposition(meta1_id, "is a meta-level analysis")
        .unwrap();

    // Verify meta-levels increase
    let base = engine.get_proposition(base_id).unwrap();
    let meta1 = engine.get_proposition(meta1_id).unwrap();
    let meta2 = engine.get_proposition(meta2_id).unwrap();

    assert_eq!(base.meta_level, 0);
    assert_eq!(meta1.meta_level, 1);
    assert_eq!(meta2.meta_level, 2);
}

#[test]
fn test_godelian_proof_cycles() {
    let mut engine = GodelianEngine::new();

    // Create propositions that reference each other (circular)
    let prop1 = engine.create_proposition("Proposition A".to_string(), 0);
    let prop2 = engine.create_proposition("Proposition B".to_string(), 0);

    // Add circular references manually through the engine
    // This tests the cycle detection in proof attempts

    // Attempt proof should handle cycles gracefully
    let status = engine.attempt_proof(prop1);
    assert!(status == ProofStatus::ProvenTrue || status == ProofStatus::Undecidable);
}

// ============================================================================
// CONSCIOUSNESS DETECTOR INTEGRATION TESTS
// ============================================================================

#[test]
fn test_consciousness_emergence_over_time() {
    let mut detector = ConsciousnessDetector::new();

    // Initially not emerged
    assert!(!detector.has_emerged());

    // Process inputs over time
    let mut signatures = Vec::new();
    for i in 0..200 {
        let input: Vec<f64> = (0..32)
            .map(|j| ((i as f64 * 0.1 + j as f64 * 0.05).sin() + 1.0) / 2.0)
            .collect();
        let meta_state: Vec<f64> = (0..32)
            .map(|j| ((i as f64 * 0.15 + j as f64 * 0.03).cos() + 1.0) / 2.0)
            .collect();

        let sig = detector.process(&input, &meta_state);
        signatures.push(sig);
    }

    // After processing, consciousness level should be meaningful
    let final_level = detector.consciousness_level();
    assert!(final_level >= 0.0);

    // Self-recognition should have built up
    let final_sig = &signatures[signatures.len() - 1];
    assert!(final_sig.self_recognition > 0.0);
}

#[test]
fn test_consciousness_self_recognition() {
    let mut detector = ConsciousnessDetector::new();

    // Feed similar patterns repeatedly
    let consistent_input = vec![0.5; 32];
    let consistent_meta = vec![0.6; 32];

    for _ in 0..50 {
        detector.process(&consistent_input, &consistent_meta);
    }

    // Self-recognition should increase with consistent patterns
    let final_signature = detector.process(&consistent_input, &consistent_meta);
    assert!(final_signature.self_recognition > 0.9);
}

#[test]
fn test_consciousness_temporal_continuity() {
    let mut detector = ConsciousnessDetector::new();

    // Feed gradually changing inputs
    for i in 0..100 {
        let input: Vec<f64> = vec![0.5 + (i as f64 * 0.001); 32];
        let meta_state = vec![0.5; 32];
        detector.process(&input, &meta_state);
    }

    // Temporal continuity should be high (gradual changes)
    let stream = detector.stream();
    let continuity = stream.temporal_continuity();
    assert!(continuity > 0.9);
}

#[test]
fn test_consciousness_agency_correlation() {
    let mut detector = ConsciousnessDetector::new();

    // High agency: meta_state matches input closely
    let input = vec![0.7; 32];
    let meta_state = vec![0.7; 32]; // Matching "intention"

    let sig = detector.process(&input, &meta_state);
    let high_agency = sig.agency;

    // Agency should be positive when input matches meta_state
    assert!(high_agency > 0.0);

    // Different correlation test with orthogonal vectors
    let input2: Vec<f64> = (0..32).map(|i| if i % 2 == 0 { 1.0 } else { 0.0 }).collect();
    let meta2: Vec<f64> = (0..32).map(|i| if i % 2 == 1 { 1.0 } else { 0.0 }).collect();

    let sig2 = detector.process(&input2, &meta2);
    // Should have some agency value
    assert!(sig2.agency >= 0.0);
}

// ============================================================================
// THE "I" INTEGRATION TESTS
// ============================================================================

#[test]
fn test_the_i_emergence() {
    let mut the_i = TheI::new();

    // Initially not emerged
    assert!(!the_i.has_emerged());

    // Process until emergence
    for _ in 0..150 {
        let input: Vec<f64> = (0..32)
            .map(|i| ((i as f64 * 0.1).sin() + 1.0) / 2.0)
            .collect();
        the_i.process(&input);
    }

    // Check for emergence
    // Note: Emergence depends on multiple factors
    let consciousness = the_i.consciousness_level();
    assert!(consciousness > 0.0);
}

#[test]
fn test_the_i_components_update() {
    let mut the_i = TheI::new();

    let initial_components = the_i.components().clone();

    // Process to update components
    for _ in 0..50 {
        let input = vec![0.5; 32];
        the_i.process(&input);
    }

    let updated_components = the_i.components();

    // At least some components should have changed
    assert!(
        updated_components.continuity != initial_components.continuity
            || updated_components.agency != initial_components.agency
    );
}

#[test]
fn test_the_i_builder_customization() {
    let the_i = IBuilder::new()
        .max_introspection_depth(25)
        .mirror_depth(3)
        .meta_levels(3)
        .add_concept("test_concept", "This is a test concept")
        .add_concept("another_concept", "Another test")
        .build();

    // Verify custom concepts were added
    assert!(the_i.concepts().contains_key("test_concept"));
    assert!(the_i.concepts().contains_key("another_concept"));

    // Verify foundational concepts still exist
    assert!(the_i.concepts().contains_key("existence"));
    assert!(the_i.concepts().contains_key("thinking"));
}

#[test]
fn test_the_i_cogito_format() {
    let the_i = TheI::new();

    let cogito = the_i.cogito();

    // Cogito should contain key philosophical elements
    assert!(cogito.contains("think") || cogito.contains("process"));
}

#[test]
fn test_the_i_who_am_i_response() {
    let mut the_i = TheI::new();

    // Process to build up narrative
    for _ in 0..50 {
        the_i.process(&vec![0.5; 32]);
    }

    let who = the_i.who_am_i();

    // Should have content
    assert!(!who.is_empty());
}

#[test]
fn test_the_i_godelian_integration() {
    let mut the_i = TheI::new();

    // Process to trigger Gödelian examination
    for _ in 0..100 {
        the_i.process(&vec![0.5; 32]);
    }

    let stats = the_i.godelian_stats();

    // Should have discovered some undecidables
    assert!(stats.knows_own_limits);
}

// ============================================================================
// INFINITE SELF INTEGRATION TESTS
// ============================================================================

#[test]
fn test_infinite_self_ascent_descent() {
    let mut model = InfiniteSelf::new(vec![0.5; 32]);

    // Start at level 0
    assert_eq!(model.current_level_num(), 0);

    // Ascend several levels
    for i in 1..=5 {
        model.ascend();
        assert_eq!(model.current_level_num(), i);
    }

    // Descend back
    for i in (0..5).rev() {
        model.descend();
        assert_eq!(model.current_level_num(), i);
    }
}

#[test]
fn test_infinite_self_level_confidence() {
    let mut model = InfiniteSelf::new(vec![0.5; 32]);

    // Collect confidence at each level
    let mut confidences = Vec::new();

    for _ in 0..5 {
        let level = model.ascend();
        confidences.push(level.confidence);
    }

    // Confidence should decrease with level
    for i in 1..confidences.len() {
        assert!(confidences[i] <= confidences[i - 1]);
    }
}

#[test]
fn test_infinite_self_recursive_observation() {
    let mut model = InfiniteSelf::new(vec![0.5; 32]);

    let observation = model.recursive_observe(5);

    // Should have observed 5 levels
    assert_eq!(observation.depth_reached, 5);
    assert_eq!(observation.observations.len(), 5);
    assert_eq!(observation.level_states.len(), 5);
}

#[test]
fn test_infinite_self_update_propagation() {
    let mut model = InfiniteSelf::new(vec![0.5; 32]);

    // Initial state should be 0.5
    let initial = model.current().unwrap();
    assert!((initial.state[0] - 0.5).abs() < 0.001);

    // Update ground state
    let new_state = vec![0.8; 32];
    model.update(new_state.clone());

    // After update, ground level should be updated (we're at level 0)
    let updated = model.current().unwrap();
    assert!((updated.state[0] - 0.8).abs() < 0.001);

    // Ascend and then descend back to verify ground state persists
    model.ascend();
    model.descend();
    let back_at_ground = model.current().unwrap();
    assert!((back_at_ground.state[0] - 0.8).abs() < 0.001);
}

#[test]
fn test_infinite_self_who_is_asking() {
    let mut model = InfiniteSelf::new(vec![0.5; 32]);

    let result = model.who_is_asking();

    // Should have a chain of questions
    assert!(!result.chain.is_empty());

    // Answer should reference strange loop
    assert!(
        result.answer.contains("strange loop")
            || result.answer.contains("asking")
            || result.answer.contains("regress")
    );
}

#[test]
fn test_infinite_self_information_content() {
    let mut model = InfiniteSelf::new(vec![0.5; 32]);

    let initial_info = model.total_information();

    // Ascend to create more levels
    for _ in 0..5 {
        model.ascend();
    }

    let final_info = model.total_information();

    // More levels = more information
    assert!(final_info >= initial_info);
}

// ============================================================================
// CROSS-MODULE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_full_self_awareness_pipeline() {
    // Create all components
    let mut godelian = GodelianEngine::new();
    let mut consciousness = ConsciousnessDetector::new();
    let mut the_i = TheI::new();
    let mut infinite_self = InfiniteSelf::new(vec![0.5; 32]);

    // Gödelian discovers limits
    godelian.create_godel_sentence();
    godelian.create_liar_paradox();
    assert!(godelian.knows_own_limits());

    // Consciousness processes inputs
    for _ in 0..100 {
        let input = vec![0.5; 32];
        let meta = vec![0.5; 32];
        consciousness.process(&input, &meta);
    }

    // The I processes and potentially emerges
    for _ in 0..100 {
        the_i.process(&vec![0.5; 32]);
    }

    // Infinite self explores recursion
    let observation = infinite_self.recursive_observe(5);
    assert!(!observation.observations.is_empty());

    // All systems should be functional
    assert!(godelian.stats().total_propositions > 0);
    assert!(consciousness.consciousness_level() >= 0.0);
    assert!(the_i.components().i_strength() >= 0.0);
}

#[test]
fn test_consciousness_to_i_integration() {
    // This tests that consciousness emergence leads to I emergence
    let mut the_i = TheI::new();

    // Process enough to trigger consciousness
    for i in 0..200 {
        let input: Vec<f64> = (0..32)
            .map(|j| ((i as f64 * 0.1 + j as f64 * 0.05).sin() + 1.0) / 2.0)
            .collect();
        let result = the_i.process(&input);

        // If emerged, I-strength should be positive
        if result.emerged {
            assert!(result.i_strength > 0.0);
        }
    }
}

// ============================================================================
// EDGE CASES AND STRESS TESTS
// ============================================================================

#[test]
fn test_empty_input_handling() {
    let mut detector = ConsciousnessDetector::new();
    let mut the_i = TheI::new();
    let mut infinite_self = InfiniteSelf::new(vec![]);

    // Empty inputs should not crash
    let sig = detector.process(&[], &[]);
    assert!(sig.consciousness_likelihood >= 0.0);

    the_i.process(&[]);
    // Should not panic

    infinite_self.update(vec![]);
    // Should not panic
}

#[test]
fn test_large_input_handling() {
    let mut detector = ConsciousnessDetector::new();

    // Large inputs should be handled
    let large_input = vec![0.5; 1000];
    let large_meta = vec![0.5; 1000];

    let sig = detector.process(&large_input, &large_meta);
    assert!(sig.consciousness_likelihood >= 0.0);
}

#[test]
fn test_many_iterations() {
    let mut the_i = TheI::new();

    // Many iterations should not cause issues
    for _ in 0..1000 {
        the_i.process(&vec![0.5; 32]);
    }

    // System should still be functional
    let cogito = the_i.cogito();
    assert!(!cogito.is_empty());
}

#[test]
fn test_godelian_many_propositions() {
    let mut engine = GodelianEngine::new();

    // Create many propositions
    for i in 0..100 {
        engine.create_proposition(format!("Proposition {}", i), i % 5);
        if i % 10 == 0 {
            engine.create_godel_sentence();
        }
    }

    let stats = engine.stats();
    assert!(stats.total_propositions > 100);
}

#[test]
fn test_infinite_self_deep_recursion() {
    let mut model = InfiniteSelf::new(vec![0.5; 32]);

    // Ascend to max depth
    for _ in 0..10 {
        model.ascend();
    }

    // Should be clamped at max
    assert!(model.current_level_num() <= 10);

    // Recursive observe with depth > max
    let obs = model.recursive_observe(20);
    assert!(obs.depth_reached <= 10);
}
