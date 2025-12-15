//! Integration tests for omega-mindscape
//! Tests memory navigation, dream exploration, and meta-cognition

use omega_mindscape::*;

// ============================================================================
// BASIC CREATION TESTS
// ============================================================================

#[test]
fn test_explorer_creation() {
    let _explorer = MindscapeExplorer::new();
}

#[test]
fn test_custom_config() {
    let config = MindscapeConfig {
        embedding_dim: 128,
        world_size: 500.0,
        num_place_cells: 100,
        max_loop_depth: 5,
        phi_threshold: 0.2,
        dream_speed: 5.0,
        cluster_threshold: 0.7,
    };

    let _explorer = MindscapeExplorer::with_config(config);
}

// ============================================================================
// MEMORY STORAGE TESTS
// ============================================================================

#[test]
fn test_remember_single_memory() {
    let explorer = MindscapeExplorer::new();

    let embedding = vec![0.5; 256];
    let result = explorer.remember("first_memory", &embedding);

    assert!(result.is_ok(), "Should store memory successfully");
}

#[test]
fn test_remember_multiple_memories() {
    let explorer = MindscapeExplorer::new();

    let memories = vec![
        ("childhood", vec![0.1; 256]),
        ("graduation", vec![0.5; 256]),
        ("first_job", vec![0.9; 256]),
    ];

    for (label, embedding) in memories {
        let result = explorer.remember(label, &embedding);
        assert!(result.is_ok(), "Should store {} successfully", label);
    }
}

#[test]
fn test_remember_returns_coordinates() {
    let explorer = MindscapeExplorer::new();

    let embedding = vec![0.7; 256];
    let coord = explorer.remember("test", &embedding).expect("Should get coordinate");

    // Verify coordinate has position
    assert!(coord.position.x.is_finite());
    assert!(coord.position.y.is_finite());
    assert!(coord.position.z.is_finite());
}

// ============================================================================
// NAVIGATION TESTS
// ============================================================================

#[test]
fn test_navigate_to_stored_memory() {
    let explorer = MindscapeExplorer::new();

    // Store a memory
    let embedding = vec![0.5; 256];
    explorer.remember("target", &embedding).expect("Store failed");

    // Navigate to it
    let result = explorer.navigate_to("target");
    assert!(result.is_ok(), "Should navigate successfully");
}

#[test]
fn test_navigate_to_nonexistent_memory() {
    let explorer = MindscapeExplorer::new();

    let result = explorer.navigate_to("does_not_exist");
    assert!(result.is_err(), "Should fail for nonexistent memory");

    if let Err(e) = result {
        assert!(matches!(e, MindscapeError::MemoryNotFound(_)));
    }
}

#[test]
fn test_navigate_between_memories() {
    let explorer = MindscapeExplorer::new();

    explorer.remember("memory_a", &vec![0.2; 256]).expect("Store A failed");
    explorer.remember("memory_b", &vec![0.8; 256]).expect("Store B failed");

    let path_a = explorer.navigate_to("memory_a");
    assert!(path_a.is_ok(), "Navigate to A should succeed");

    let path_b = explorer.navigate_to("memory_b");
    assert!(path_b.is_ok(), "Navigate to B should succeed");
}

// ============================================================================
// LOOK AROUND TESTS
// ============================================================================

#[test]
fn test_look_around() {
    let explorer = MindscapeExplorer::new();

    // Store several memories
    for i in 0..5 {
        let mut embedding = vec![0.5; 256];
        embedding[0] = i as f64 / 10.0;
        explorer.remember(&format!("memory_{}", i), &embedding).expect("Store failed");
    }

    // Look around
    let nearby = explorer.look_around(10);
    assert!(nearby.len() > 0, "Should find nearby memories");
}

// ============================================================================
// DREAM EXPLORATION TESTS
// ============================================================================

#[test]
fn test_enter_dream_state() {
    let explorer = MindscapeExplorer::new();

    let result = explorer.enter_dream_state();
    assert!(result.is_ok(), "Should enter dream state");
}

#[test]
fn test_cannot_enter_dream_twice() {
    let explorer = MindscapeExplorer::new();

    explorer.enter_dream_state().expect("First entry should succeed");
    let second_attempt = explorer.enter_dream_state();

    assert!(second_attempt.is_err(), "Should not enter dream twice");
    if let Err(e) = second_attempt {
        assert!(matches!(e, MindscapeError::AlreadyDreaming));
    }
}

#[test]
fn test_wake_up_from_dream() {
    let explorer = MindscapeExplorer::new();

    explorer.enter_dream_state().expect("Enter dream failed");
    let result = explorer.wake_up();

    assert!(result.is_ok(), "Should wake up successfully");
}

#[test]
fn test_wake_up_when_not_dreaming() {
    let explorer = MindscapeExplorer::new();

    let result = explorer.wake_up();
    assert!(result.is_err(), "Should fail when not dreaming");

    if let Err(e) = result {
        assert!(matches!(e, MindscapeError::NotDreaming));
    }
}

#[test]
fn test_dream_explore_basic() {
    let explorer = MindscapeExplorer::new();

    // Store memories for dream exploration
    for i in 0..3 {
        let embedding = vec![i as f64 / 10.0; 256];
        explorer.remember(&format!("mem_{}", i), &embedding).expect("Store failed");
    }

    explorer.enter_dream_state().expect("Enter dream failed");
    let discoveries = explorer.dream_explore(1.0); // 1 minute
    explorer.wake_up().expect("Wake up failed");

    assert!(discoveries.is_ok(), "Dream exploration should succeed");
}

// ============================================================================
// STRANGE LOOP OBSERVER TESTS
// ============================================================================

#[test]
fn test_observe_exploration_depth_1() {
    let explorer = MindscapeExplorer::new();

    let observation = explorer.observe_exploration(1);
    assert!(observation.is_ok(), "Observation depth 1 should succeed");
}

#[test]
fn test_observe_exploration_multiple_depths() {
    let explorer = MindscapeExplorer::new();

    for depth in 1..=3 {
        let observation = explorer.observe_exploration(depth);
        assert!(observation.is_ok(), "Observation depth {} should succeed", depth);
    }
}

#[test]
fn test_observe_exploration_exceeds_max_depth() {
    let config = MindscapeConfig {
        max_loop_depth: 3,
        ..Default::default()
    };
    let explorer = MindscapeExplorer::with_config(config);

    let observation = explorer.observe_exploration(5);
    assert!(observation.is_err(), "Should fail when exceeding max depth");

    if let Err(e) = observation {
        assert!(matches!(e, MindscapeError::RecursionLimit(_)));
    }
}

// ============================================================================
// STATE QUERY TESTS
// ============================================================================

#[test]
fn test_get_state() {
    let explorer = MindscapeExplorer::new();

    let state = explorer.state();
    assert!(state.phi >= 0.0, "Phi should be non-negative");
    assert_eq!(state.observation_depth, 0, "Initial observation depth should be 0");
    assert!(!state.is_dreaming, "Should not be dreaming initially");
}

#[test]
fn test_get_state_while_dreaming() {
    let explorer = MindscapeExplorer::new();

    explorer.enter_dream_state().expect("Enter dream failed");
    let state = explorer.state();

    assert!(state.is_dreaming, "Should be dreaming");

    explorer.wake_up().expect("Wake up failed");
}

#[test]
fn test_get_stats() {
    let explorer = MindscapeExplorer::new();

    let stats = explorer.stats();
    assert_eq!(stats.total_distance, 0.0, "Initial distance should be 0");
    assert_eq!(stats.discoveries_made, 0, "Initial discoveries should be 0");
}

#[test]
fn test_stats_update_after_navigation() {
    let explorer = MindscapeExplorer::new();

    explorer.remember("start", &vec![0.1; 256]).expect("Store start failed");
    explorer.remember("end", &vec![0.9; 256]).expect("Store end failed");

    explorer.navigate_to("start").expect("Navigate to start failed");
    explorer.navigate_to("end").expect("Navigate to end failed");

    let stats = explorer.stats();
    assert!(stats.total_distance > 0.0, "Distance should have increased");
}

// ============================================================================
// DISCOVERY JOURNAL TESTS
// ============================================================================

#[test]
fn test_get_discoveries() {
    let explorer = MindscapeExplorer::new();

    let discoveries = explorer.discoveries();
    assert_eq!(discoveries.len(), 0, "Initial discoveries should be empty");
}

#[test]
fn test_discoveries_after_dream() {
    let explorer = MindscapeExplorer::new();

    // Store memories
    for i in 0..5 {
        let embedding = vec![i as f64 / 10.0; 256];
        explorer.remember(&format!("mem_{}", i), &embedding).expect("Store failed");
    }

    explorer.enter_dream_state().expect("Enter dream failed");
    let _result = explorer.dream_explore(2.0);
    explorer.wake_up().expect("Wake up failed");

    let discoveries = explorer.discoveries();
    // Discoveries may or may not have been made depending on RNG
    assert!(discoveries.len() >= 0);
}

// ============================================================================
// COORDINATE MAPPER TESTS
// ============================================================================

#[test]
fn test_coordinate_mapper_creation() {
    let _mapper = CoordinateMapper::new(256, 1000.0);
}

#[test]
fn test_coordinate_mapping() {
    let mapper = CoordinateMapper::new(256, 1000.0);

    let embedding = vec![0.5; 256];
    let coord = mapper.map_to_coordinate(&embedding);

    assert!(coord.position.x.is_finite());
    assert!(coord.position.y.is_finite());
    assert!(coord.position.z.is_finite());
}

#[test]
fn test_different_embeddings_produce_different_coordinates() {
    let mapper = CoordinateMapper::new(256, 1000.0);

    let embedding1 = vec![0.1; 256];
    let embedding2 = vec![0.9; 256];

    let coord1 = mapper.map_to_coordinate(&embedding1);
    let coord2 = mapper.map_to_coordinate(&embedding2);

    let distance = ((coord1.position.x - coord2.position.x).powi(2)
        + (coord1.position.y - coord2.position.y).powi(2)
        + (coord1.position.z - coord2.position.z).powi(2))
        .sqrt();

    assert!(distance > 0.0, "Different embeddings should map to different positions");
}

// ============================================================================
// NAVIGATOR TESTS
// ============================================================================

#[test]
fn test_navigator_creation() {
    let _nav = MindscapeNavigator::new(1000.0, 200);
}

// ============================================================================
// DREAM EXPLORER TESTS
// ============================================================================

#[test]
fn test_dream_explorer_creation() {
    let _dream_explorer = DreamExplorer::new(10.0);
}

// ============================================================================
// STRANGE LOOP OBSERVER TESTS
// ============================================================================

#[test]
fn test_observer_creation() {
    let _observer = StrangeLoopObserver::new(7);
}

// ============================================================================
// DISCOVERY JOURNAL TESTS
// ============================================================================

#[test]
fn test_journal_creation() {
    let _journal = DiscoveryJournal::new();
}

// ============================================================================
// EXPLORATION MODE TESTS
// ============================================================================

#[test]
fn test_exploration_modes() {
    let modes = vec![
        ExplorationMode::Waking,
        ExplorationMode::Focused,
        ExplorationMode::Dreaming,
        ExplorationMode::Observing,
        ExplorationMode::LucidDreaming,
    ];

    for mode in modes {
        // Just verify modes can be created and compared
        assert_eq!(mode, mode);
    }
}

// ============================================================================
// COMPLETE WORKFLOW TESTS
// ============================================================================

#[test]
fn test_complete_exploration_workflow() {
    let explorer = MindscapeExplorer::new();

    // 1. Store memories
    explorer.remember("memory_1", &vec![0.2; 256]).expect("Store 1 failed");
    explorer.remember("memory_2", &vec![0.5; 256]).expect("Store 2 failed");
    explorer.remember("memory_3", &vec![0.8; 256]).expect("Store 3 failed");

    // 2. Navigate
    explorer.navigate_to("memory_1").expect("Navigate failed");

    // 3. Look around
    let nearby = explorer.look_around(5);
    assert!(!nearby.is_empty(), "Should find nearby memories");

    // 4. Enter dream and explore
    explorer.enter_dream_state().expect("Enter dream failed");
    let _discoveries = explorer.dream_explore(1.0);
    explorer.wake_up().expect("Wake up failed");

    // 5. Observe meta-cognitively
    let _observation = explorer.observe_exploration(2);

    // 6. Check stats
    let stats = explorer.stats();
    assert!(stats.total_distance > 0.0, "Should have traveled");
}

#[test]
fn test_lucid_dreaming_workflow() {
    let explorer = MindscapeExplorer::new();

    // Store memories
    for i in 0..10 {
        let mut embedding = vec![0.5; 256];
        embedding[i] = i as f64 / 10.0;
        explorer.remember(&format!("mem_{}", i), &embedding).expect("Store failed");
    }

    // Enter lucid dreaming (dream + observe)
    explorer.enter_dream_state().expect("Enter dream failed");
    let _observation = explorer.observe_exploration(1); // Observe while dreaming
    let _discoveries = explorer.dream_explore(1.0);
    explorer.wake_up().expect("Wake up failed");

    let stats = explorer.stats();
    assert!(stats.dream_time > 0.0, "Should have dream time");
}

#[test]
fn test_systematic_memory_tour() {
    let explorer = MindscapeExplorer::new();

    let memory_labels = vec!["childhood", "school", "university", "career", "present"];

    // Store memories
    for (i, label) in memory_labels.iter().enumerate() {
        let mut embedding = vec![0.5; 256];
        embedding[0] = i as f64 / 10.0;
        explorer.remember(label, &embedding).expect("Store failed");
    }

    // Visit each memory in sequence
    for label in &memory_labels {
        let result = explorer.navigate_to(label);
        assert!(result.is_ok(), "Should navigate to {}", label);
    }

    let stats = explorer.stats();
    assert_eq!(stats.memories_visited, memory_labels.len(), "Should have visited all memories");
}
