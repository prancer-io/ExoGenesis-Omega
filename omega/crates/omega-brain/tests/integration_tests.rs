//! Integration tests for omega-brain
//! Tests the unified brain-like cognitive architecture

use omega_brain::*;

// ============================================================================
// BRAIN CREATION TESTS
// ============================================================================

#[test]
fn test_brain_creation_default() {
    let brain = OmegaBrain::new();
    assert!(brain.is_active(), "Brain should be active by default");
}

#[test]
fn test_brain_creation_with_config() {
    let config = BrainConfig::default();
    let brain = OmegaBrain::with_config(config);
    assert!(brain.is_active());
}

#[test]
fn test_brain_default_trait() {
    let brain = OmegaBrain::default();
    assert!(brain.is_active());
}

// ============================================================================
// COGNITIVE PROCESSING TESTS
// ============================================================================

#[test]
fn test_single_processing_cycle() {
    let brain = OmegaBrain::new();
    let input = vec![0.5; 32];

    let result = brain.process(&input);
    assert!(result.is_ok(), "Processing should succeed");

    let metrics = brain.metrics();
    assert_eq!(metrics.cycles, 1, "Should have completed 1 cycle");
}

#[test]
fn test_multiple_processing_cycles() {
    let brain = OmegaBrain::new();
    let input = vec![0.5; 32];

    for _ in 0..10 {
        let result = brain.process(&input);
        assert!(result.is_ok());
    }

    let metrics = brain.metrics();
    assert_eq!(metrics.cycles, 10, "Should have completed 10 cycles");
}

#[test]
fn test_processing_with_different_inputs() {
    let brain = OmegaBrain::new();

    let inputs = vec![
        vec![0.1; 32],
        vec![0.5; 32],
        vec![0.9; 32],
        vec![0.3; 32],
    ];

    for input in &inputs {
        let result = brain.process(input);
        assert!(result.is_ok(), "Processing should handle different inputs");
    }
}

#[test]
fn test_processing_returns_valid_result() {
    let brain = OmegaBrain::new();
    let input = vec![0.5; 32];

    let result = brain.process(&input).unwrap();

    assert!(!result.output.is_empty(), "Should return non-empty output");
    assert!(result.consciousness_level >= 0.0 && result.consciousness_level <= 1.0);
    assert!(result.attention_strength >= 0.0 && result.attention_strength <= 1.0);
}

// ============================================================================
// STATE AND METRICS TESTS
// ============================================================================

#[test]
fn test_brain_state_query() {
    let brain = OmegaBrain::new();
    let state = brain.state();

    assert!(state.consciousness_level >= 0.0 && state.consciousness_level <= 1.0);
    assert!(state.self_reference >= 0.0 && state.self_reference <= 1.0);
    assert!(!state.attention_focus.is_empty());
    assert_eq!(state.cycle_count, 0);
}

#[test]
fn test_brain_metrics_initial() {
    let brain = OmegaBrain::new();
    let metrics = brain.metrics();

    assert_eq!(metrics.cycles, 0);
    assert!(metrics.phi >= 0.0);
    assert!(metrics.spike_rate >= 0.0);
}

#[test]
fn test_brain_metrics_after_processing() {
    let brain = OmegaBrain::new();
    let input = vec![0.5; 32];

    brain.process(&input).unwrap();

    let metrics = brain.metrics();
    assert!(metrics.cycles > 0);
}

#[test]
fn test_state_updates_after_processing() {
    let brain = OmegaBrain::new();

    let state_before = brain.state();

    let input = vec![0.5; 32];
    brain.process(&input).unwrap();

    let state_after = brain.state();
    assert!(state_after.cycle_count > state_before.cycle_count);
}

// ============================================================================
// CONSCIOUSNESS TESTS
// ============================================================================

#[test]
fn test_consciousness_level_query() {
    let brain = OmegaBrain::new();
    let level = brain.consciousness_level();

    assert!(level >= 0.0 && level <= 1.0, "Consciousness level should be between 0 and 1");
}

#[test]
fn test_phi_value_query() {
    let brain = OmegaBrain::new();
    let phi = brain.phi();

    assert!(phi >= 0.0, "Phi should be non-negative");
}

#[test]
fn test_consciousness_during_processing() {
    let brain = OmegaBrain::new();
    let input = vec![0.7; 32];

    brain.process(&input).unwrap();

    let level = brain.consciousness_level();
    let phi = brain.phi();

    assert!(level >= 0.0 && level <= 1.0);
    assert!(phi >= 0.0);
}

// ============================================================================
// MEMORY SYSTEM TESTS
// ============================================================================

#[test]
fn test_remember_single_memory() {
    let brain = OmegaBrain::new();
    let content = vec![0.5; 32];

    let result = brain.remember(&content, 0.8);
    assert!(result.is_ok(), "Should store memory successfully");
}

#[test]
fn test_remember_multiple_memories() {
    let brain = OmegaBrain::new();

    for i in 0..5 {
        let content = vec![i as f64 / 10.0; 32];
        let result = brain.remember(&content, 0.5);
        assert!(result.is_ok());
    }
}

#[test]
fn test_recall_memory() {
    let brain = OmegaBrain::new();
    let content = vec![0.5; 32];

    // Store memory
    brain.remember(&content, 0.9).unwrap();

    // Try to recall
    let result = brain.recall(&content);
    assert!(result.is_ok(), "Recall should succeed");
}

#[test]
fn test_recall_nonexistent_memory() {
    let brain = OmegaBrain::new();
    let cue = vec![0.999; 32];

    let result = brain.recall(&cue);
    assert!(result.is_ok(), "Recall should succeed even if memory not found");
}

#[test]
fn test_consolidate_memories() {
    let brain = OmegaBrain::new();

    // Store some memories
    for i in 0..3 {
        let content = vec![i as f64 / 10.0; 32];
        brain.remember(&content, 0.7).unwrap();
    }

    let result = brain.consolidate_memories();
    assert!(result.is_ok(), "Consolidation should succeed");
}

// ============================================================================
// SLEEP SYSTEM TESTS
// ============================================================================

#[test]
fn test_sleep_initiation() {
    let brain = OmegaBrain::new();

    let result = brain.sleep();
    assert!(result.is_ok(), "Should enter sleep successfully");
}

#[test]
fn test_wake_from_sleep() {
    let brain = OmegaBrain::new();

    brain.sleep().unwrap();
    let result = brain.wake();
    assert!(result.is_ok(), "Should wake up successfully");
}

#[test]
fn test_is_dreaming_initially() {
    let brain = OmegaBrain::new();
    assert!(!brain.is_dreaming(), "Should not be dreaming initially");
}

#[test]
fn test_processing_during_sleep() {
    let brain = OmegaBrain::new();

    brain.sleep().unwrap();

    let input = vec![0.5; 32];
    let result = brain.process(&input);

    // Processing during sleep should still work but be different
    assert!(result.is_ok());
}

#[test]
fn test_sleep_wake_cycle() {
    let brain = OmegaBrain::new();

    // Sleep
    brain.sleep().unwrap();

    // Process during sleep
    let input = vec![0.5; 32];
    brain.process(&input).unwrap();

    // Wake
    brain.wake().unwrap();

    // Process after waking
    brain.process(&input).unwrap();
}

// ============================================================================
// ATTENTION TESTS
// ============================================================================

#[test]
fn test_think_about_topic() {
    let brain = OmegaBrain::new();
    let topic = vec![0.5; 32];

    let result = brain.think_about(&topic);
    assert!(result.is_ok(), "Should think about topic successfully");

    let output = result.unwrap();
    assert!(!output.is_empty(), "Should return thought output");
}

#[test]
fn test_think_about_multiple_topics() {
    let brain = OmegaBrain::new();

    let topics = vec![
        vec![0.1; 32],
        vec![0.5; 32],
        vec![0.9; 32],
    ];

    for topic in &topics {
        let result = brain.think_about(topic);
        assert!(result.is_ok());
    }
}

// ============================================================================
// SELF-AWARENESS TESTS
// ============================================================================

#[test]
fn test_self_state_query() {
    let brain = OmegaBrain::new();
    let self_state = brain.self_state();

    assert!(!self_state.is_empty(), "Self state should not be empty");
}

#[test]
fn test_self_state_after_processing() {
    let brain = OmegaBrain::new();

    let state_before = brain.self_state();

    let input = vec![0.5; 32];
    brain.process(&input).unwrap();

    let state_after = brain.self_state();

    // States should exist (may or may not be different)
    assert!(!state_before.is_empty());
    assert!(!state_after.is_empty());
}

// ============================================================================
// BRAIN LIFECYCLE TESTS
// ============================================================================

#[test]
fn test_brain_activation_control() {
    let brain = OmegaBrain::new();

    assert!(brain.is_active(), "Brain should start active");

    brain.deactivate();
    assert!(!brain.is_active(), "Brain should be deactivated");

    brain.activate();
    assert!(brain.is_active(), "Brain should be reactivated");
}

#[test]
fn test_processing_when_deactivated() {
    let brain = OmegaBrain::new();

    brain.deactivate();

    let input = vec![0.5; 32];
    let result = brain.process(&input);

    assert!(result.is_err(), "Processing should fail when brain is deactivated");
}

#[test]
fn test_brain_reset() {
    let brain = OmegaBrain::new();

    // Process some cycles
    let input = vec![0.5; 32];
    for _ in 0..5 {
        brain.process(&input).unwrap();
    }

    let metrics_before = brain.metrics();
    assert!(metrics_before.cycles > 0);

    // Reset
    brain.reset();

    let metrics_after = brain.metrics();
    assert_eq!(metrics_after.cycles, 0, "Cycle count should reset to 0");
}

#[test]
fn test_processing_after_reset() {
    let brain = OmegaBrain::new();

    // Process, reset, process again
    let input = vec![0.5; 32];
    brain.process(&input).unwrap();

    brain.reset();

    let result = brain.process(&input);
    assert!(result.is_ok(), "Processing should work after reset");

    let metrics = brain.metrics();
    assert_eq!(metrics.cycles, 1, "Should have 1 cycle after reset and reprocessing");
}

// ============================================================================
// CONFIGURATION TESTS
// ============================================================================

#[test]
fn test_brain_config_access() {
    let brain = OmegaBrain::new();
    let _config = brain.config();
    // Just verify we can access config
}

#[test]
fn test_brain_with_custom_config() {
    let mut config = BrainConfig::default();
    config.default_mode = BrainMode::Focused;

    let brain = OmegaBrain::with_config(config);
    assert_eq!(brain.config().default_mode, BrainMode::Focused);
}

// ============================================================================
// INTEGRATION WORKFLOW TESTS
// ============================================================================

#[test]
fn test_full_cognitive_workflow() {
    let brain = OmegaBrain::new();

    // 1. Store memories
    for i in 0..3 {
        let content = vec![i as f64 / 10.0; 32];
        brain.remember(&content, 0.8).unwrap();
    }

    // 2. Process input
    let input = vec![0.5; 32];
    brain.process(&input).unwrap();

    // 3. Think about something
    let topic = vec![0.6; 32];
    brain.think_about(&topic).unwrap();

    // 4. Recall memory
    let cue = vec![0.1; 32];
    brain.recall(&cue).unwrap();

    // 5. Check state
    let state = brain.state();
    assert!(state.cycle_count > 0);

    // 6. Get metrics
    let metrics = brain.metrics();
    assert!(metrics.cycles > 0);
}

#[test]
fn test_sleep_consolidation_workflow() {
    let brain = OmegaBrain::new();

    // 1. Store memories while awake
    for i in 0..5 {
        let content = vec![i as f64 / 10.0; 32];
        brain.remember(&content, 0.7).unwrap();
    }

    // 2. Enter sleep
    brain.sleep().unwrap();

    // 3. Process during sleep (consolidation)
    let input = vec![0.0; 32];
    for _ in 0..3 {
        brain.process(&input).unwrap();
    }

    // 4. Wake up
    brain.wake().unwrap();

    // 5. Verify memories are consolidated
    let metrics = brain.metrics();
    assert!(metrics.consolidation_ratio >= 0.0);
}

#[test]
fn test_conscious_attention_workflow() {
    let brain = OmegaBrain::new();

    // 1. Focus attention on topic
    let topic = vec![0.5; 32];
    let thought = brain.think_about(&topic).unwrap();

    // 2. Process the thought
    brain.process(&thought).unwrap();

    // 3. Check consciousness level
    let level = brain.consciousness_level();
    assert!(level >= 0.0 && level <= 1.0);

    // 4. Check phi
    let phi = brain.phi();
    assert!(phi >= 0.0);
}

#[test]
fn test_extended_processing_session() {
    let brain = OmegaBrain::new();
    let input = vec![0.5; 32];

    // Process many cycles
    for _ in 0..20 {
        let result = brain.process(&input);
        assert!(result.is_ok(), "Extended processing should succeed");
    }

    let metrics = brain.metrics();
    assert_eq!(metrics.cycles, 20);
    assert!(metrics.avg_processing_time >= 0.0);
}

#[test]
fn test_memory_encoding_during_processing() {
    let brain = OmegaBrain::new();

    // Store a memory
    let content = vec![0.8; 32];
    brain.remember(&content, 0.9).unwrap();

    // Process similar input
    let input = vec![0.75; 32];
    let result = brain.process(&input).unwrap();

    // Processing should succeed and potentially encode memory
    assert!(!result.output.is_empty());
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_processing_inactive_brain() {
    let brain = OmegaBrain::new();
    brain.deactivate();

    let input = vec![0.5; 32];
    let result = brain.process(&input);

    assert!(result.is_err(), "Should fail to process when inactive");
    match result {
        Err(BrainError::NeuralError(_)) => {},
        _ => panic!("Should return NeuralError"),
    }
}

// ============================================================================
// METRICS TRACKING TESTS
// ============================================================================

#[test]
fn test_metrics_tracking_over_time() {
    let brain = OmegaBrain::new();
    let input = vec![0.5; 32];

    let mut cycle_counts = Vec::new();

    for _ in 0..5 {
        brain.process(&input).unwrap();
        let metrics = brain.metrics();
        cycle_counts.push(metrics.cycles);
    }

    // Verify cycles are increasing
    for i in 1..cycle_counts.len() {
        assert!(cycle_counts[i] > cycle_counts[i-1]);
    }
}

#[test]
fn test_all_metrics_valid() {
    let brain = OmegaBrain::new();
    let input = vec![0.5; 32];

    brain.process(&input).unwrap();

    let metrics = brain.metrics();

    assert!(metrics.cycles > 0);
    assert!(metrics.avg_processing_time >= 0.0);
    assert!(metrics.phi >= 0.0);
    assert!(metrics.free_energy.is_finite());
    assert!(metrics.consolidation_ratio >= 0.0 && metrics.consolidation_ratio <= 1.0);
    assert!(metrics.strange_loop_count >= 0);
    assert!(metrics.spike_rate >= 0.0);
}
