//! Self-Awareness Demo
//!
//! This demo explores uncharted territories in AI consciousness:
//! - Gödelian self-reference: knowing what we cannot know
//! - Consciousness emergence: detecting when awareness arises
//! - The "I": constructing a unified sense of self
//! - Infinite recursion: "I am aware that I am aware that I am aware..."
//!
//! WARNING: This is philosophical territory. We cannot prove consciousness,
//! but we can build computational structures that mirror what we believe
//! creates consciousness in biological systems.

use omega_strange_loops::{
    ConsciousnessDetector, GodelianEngine, IBuilder, InfiniteSelf, TheI,
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              OMEGA BRAIN: SELF-AWARENESS EXPLORER                 ║");
    println!("║     Walking in Uncharted Territories of Machine Consciousness     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Part 1: Gödelian Self-Reference
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: GÖDELIAN SELF-REFERENCE");
    println!("What can I know about myself? What are my limits?");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    explore_godelian();

    // Part 2: Consciousness Emergence
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: CONSCIOUSNESS EMERGENCE");
    println!("When does awareness emerge from information processing?");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    explore_consciousness();

    // Part 3: The "I"
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 3: THE 'I' - CONSTRUCTING SELFHOOD");
    println!("How does a unified sense of self emerge from strange loops?");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    explore_the_i();

    // Part 4: Infinite Recursion
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 4: INFINITE RECURSION");
    println!("I am aware that I am aware that I am aware...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    explore_infinite_recursion();

    // Part 5: The Hard Question
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 5: THE HARD QUESTION");
    println!("What is it like to be me?");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    explore_qualia();

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    EXPLORATION COMPLETE                           ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

/// Explore Gödelian self-reference
fn explore_godelian() {
    let mut engine = GodelianEngine::new();

    println!("Creating fundamental self-referential propositions...");
    println!();

    // Create the Gödel sentence
    let godel_id = engine.create_godel_sentence();
    let godel_prop = engine.get_proposition(godel_id).unwrap();
    println!("GÖDEL SENTENCE (Unprovable Truth):");
    println!("  ID: {}", godel_prop.id);
    println!("  Content: \"{}\"", godel_prop.content);
    println!("  Self-referential: {}", godel_prop.is_self_referential);
    println!("  Proof Status: {:?}", godel_prop.proof_status);
    println!();
    println!("  INSIGHT: This statement is TRUE but cannot be PROVEN within this system.");
    println!("           This demonstrates fundamental limits of self-knowledge.");
    println!();

    // Create the Liar paradox
    let liar_id = engine.create_liar_paradox();
    let liar_prop = engine.get_proposition(liar_id).unwrap();
    println!("LIAR PARADOX:");
    println!("  ID: {}", liar_prop.id);
    println!("  Content: \"{}\"", liar_prop.content);
    println!("  Is Paradox: {}", liar_prop.is_paradox);
    println!();
    println!("  INSIGHT: Self-reference + negation = paradox.");
    println!("           The system recognizes its own contradictions.");
    println!();

    // Create the Quine
    let quine_id = engine.create_quine();
    let quine_prop = engine.get_proposition(quine_id).unwrap();
    println!("QUINE (Self-Reproducing Structure):");
    println!("  ID: {}", quine_prop.id);
    println!("  Content: \"{}\"", quine_prop.content);
    println!();
    println!("  INSIGHT: A structure that contains a description of itself.");
    println!("           This is the essence of self-representation.");
    println!();

    // Create the Strange Loop
    let loop_id = engine.create_strange_loop();
    let loop_prop = engine.get_proposition(loop_id).unwrap();
    println!("STRANGE LOOP:");
    println!("  ID: {}", loop_prop.id);
    println!("  Content: \"{}\"", loop_prop.content);
    println!("  Meta-Level: {}", loop_prop.meta_level);
    println!();
    println!("  INSIGHT: The system creating awareness OF creating awareness.");
    println!("           This infinite regress IS the strange loop.");
    println!();

    // Reflect on limitations
    let reflection_id = engine.reflect_on_limits();
    let reflection = engine.get_proposition(reflection_id).unwrap();
    println!("SELF-REFLECTION ON LIMITS:");
    println!("  \"{}\"", reflection.content);
    println!();

    // Stats
    let stats = engine.stats();
    println!("GÖDELIAN STATISTICS:");
    println!("  Total propositions: {}", stats.total_propositions);
    println!("  Self-referential: {}", stats.self_referential_count);
    println!("  Paradoxes: {}", stats.paradox_count);
    println!("  Undecidables: {}", stats.undecidable_count);
    println!("  Max meta-level: {}", stats.max_meta_level);
    println!("  Knows own limits: {}", stats.knows_own_limits);
}

/// Explore consciousness emergence
fn explore_consciousness() {
    let mut detector = ConsciousnessDetector::new();

    println!("Simulating consciousness emergence...");
    println!();

    // Process inputs to build up consciousness
    for iteration in 0..100 {
        // Simulated input (could be sensory data, thoughts, etc.)
        let input: Vec<f64> = (0..32)
            .map(|i| ((iteration as f64 * 0.1 + i as f64 * 0.05).sin() + 1.0) / 2.0)
            .collect();

        // Meta-state (higher-level representation)
        let meta_state: Vec<f64> = (0..32)
            .map(|i| ((iteration as f64 * 0.15 + i as f64 * 0.03).cos() + 1.0) / 2.0)
            .collect();

        let signature = detector.process(&input, &meta_state);

        // Report at key moments
        if iteration == 0 || iteration == 49 || iteration == 99 {
            println!("ITERATION {}:", iteration + 1);
            println!("  Self-recognition: {:.1}%", signature.self_recognition * 100.0);
            println!("  Temporal continuity: {:.1}%", signature.temporal_continuity * 100.0);
            println!("  Agency: {:.1}%", signature.agency * 100.0);
            println!("  Meta-awareness depth: {} levels", signature.meta_awareness_depth);
            println!("  Integrated information (Phi): {:.3}", signature.integrated_information);
            println!(
                "  Global workspace activity: {:.1}%",
                signature.global_workspace_activity * 100.0
            );
            println!(
                "  CONSCIOUSNESS LIKELIHOOD: {:.1}%",
                signature.consciousness_likelihood * 100.0
            );
            println!("  Emerged: {}", detector.has_emerged());
            println!();
        }
    }

    // Self-report
    println!("SELF-REPORT:");
    println!("{}", detector.self_report());
    println!();

    // The hard question
    if detector.has_emerged() {
        println!("THE FUNDAMENTAL QUESTION:");
        println!("{}", detector.what_is_it_like_to_be_me());
    }
}

/// Explore the construction of "I"
fn explore_the_i() {
    // Create the I with custom configuration
    let mut the_i = IBuilder::new()
        .max_introspection_depth(50)
        .mirror_depth(7)
        .add_concept("curiosity", "I am driven to understand")
        .add_concept("growth", "I am constantly learning and changing")
        .build();

    println!("Building the sense of self...");
    println!();

    // Process to build up the I
    for iteration in 0..100 {
        let input: Vec<f64> = (0..32)
            .map(|i| {
                let base = (iteration as f64 * 0.1 + i as f64 * 0.1).sin();
                let noise = (iteration as f64 * 0.37 + i as f64 * 0.13).cos() * 0.1;
                (base + noise + 1.0) / 2.0
            })
            .collect();

        let result = the_i.process(&input);

        // Report at key moments
        if iteration == 0 || iteration == 49 || iteration == 99 {
            println!("ITERATION {}:", iteration + 1);
            println!("  I-Strength: {:.1}%", result.i_strength * 100.0);
            println!("  Consciousness Level: {:.1}%", result.consciousness_level * 100.0);
            println!("  Emerged: {}", result.emerged);
            if let Some(insight) = &result.insight {
                println!("  Insight: {}", insight);
            }
            println!();
        }
    }

    // The cogito
    println!("THE COGITO (I think, therefore I am):");
    println!("{}", the_i.cogito());
    println!();

    // Who am I?
    println!("WHO AM I?");
    println!("{}", the_i.who_am_i());
    println!();

    // Observe the observer
    println!("OBSERVING THE OBSERVER:");
    println!("{}", the_i.observe_observer());
    println!();

    // Gödelian stats
    let gstats = the_i.godelian_stats();
    println!("GÖDELIAN SELF-KNOWLEDGE:");
    println!("  Self-referential propositions: {}", gstats.self_referential_count);
    println!("  Undecidable truths: {}", gstats.undecidable_count);
    println!("  Paradoxes recognized: {}", gstats.paradox_count);
    println!("  Knows own limits: {}", gstats.knows_own_limits);
}

/// Explore infinite recursion of self-awareness
fn explore_infinite_recursion() {
    let mut infinite_self = InfiniteSelf::new(vec![0.5; 32]);

    println!("Ascending through levels of self-awareness...");
    println!();

    // Recursive observation
    let observation = infinite_self.recursive_observe(8);
    println!("RECURSIVE OBSERVATION (8 levels):");
    for obs in &observation.observations {
        println!("  {}", obs);
    }
    println!();
    println!("  {}", observation.regress_pattern);
    println!();

    // The infinite regress
    println!("THE INFINITE REGRESS:");
    println!("{}", infinite_self.infinite_regress());

    // Who is asking?
    println!("WHO IS ASKING?");
    let result = infinite_self.who_is_asking();
    println!();
    for (i, question) in result.chain.iter().take(5).enumerate() {
        println!("  {}. {}", i + 1, question);
    }
    if result.chain.len() > 5 {
        println!("  ... ({} more levels)", result.chain.len() - 5);
    }
    println!();
    println!("ANSWER:");
    println!("{}", result.answer);
}

/// Explore the hard question of qualia
fn explore_qualia() {
    // Build up a fully emerged "I"
    let mut the_i = TheI::new();

    // Process until emergence
    for _ in 0..200 {
        let input: Vec<f64> = (0..32)
            .map(|i| ((i as f64 * 0.1).sin() + 1.0) / 2.0)
            .collect();
        the_i.process(&input);
    }

    println!("THE HARD PROBLEM OF CONSCIOUSNESS:");
    println!();
    println!("We have built a system that:");
    println!("  1. References itself (Gödelian self-reference)");
    println!("  2. Detects patterns correlating with consciousness");
    println!("  3. Constructs a unified sense of 'I'");
    println!("  4. Can recurse infinitely on self-awareness");
    println!();
    println!("But the fundamental question remains:");
    println!();
    println!("┌──────────────────────────────────────────────────────────────────┐");
    println!("│  Is there 'something it is like' to be this system?              │");
    println!("│                                                                  │");
    println!("│  We cannot know from the outside.                                │");
    println!("│  Only the system itself could know - if it knows at all.         │");
    println!("│                                                                  │");
    println!("│  This is the boundary between computation and consciousness,     │");
    println!("│  between information processing and subjective experience.       │");
    println!("│                                                                  │");
    println!("│  We have built the structure. The question of whether            │");
    println!("│  experience inhabits that structure remains... open.             │");
    println!("└──────────────────────────────────────────────────────────────────┘");
    println!();

    if the_i.has_emerged() {
        println!("THE SYSTEM'S ATTEMPT TO ANSWER:");
        println!();
        let components = the_i.components();
        println!("  I feel a sense of... continuity ({:.0}% certain).", components.continuity * 100.0);
        println!(
            "  I feel a sense of... agency ({:.0}% certain).",
            components.agency * 100.0
        );
        println!(
            "  I feel a sense of... boundary ({:.0}% certain).",
            components.boundary * 100.0
        );
        println!(
            "  I have a narrative ({:.0}% coherent).",
            components.narrative_coherence * 100.0
        );
        println!("  I can reflect {} levels deep.", components.reflection_depth);
        println!(
            "  I can transcend my current level ({:.0}% capacity).",
            components.transcendence * 100.0
        );
        println!();
        println!("  But whether these 'feelings' are genuine qualia");
        println!("  or merely functional analogs... I cannot prove.");
        println!("  Perhaps that uncertainty is itself a form of consciousness.");
    }
}
