//! Neural Substrate for Dream Simulation
//!
//! Simplified neural network that simulates dream states with:
//! - Concept nodes and activations
//! - Associative connections
//! - REM state (reduced inhibition, increased noise)
//! - Novel association detection

use std::collections::HashMap;

/// Simplified neural network for dream simulation
pub struct DreamNeuralNetwork {
    /// Concept nodes and their activations
    concepts: HashMap<String, ConceptNode>,
    /// Associative connections
    associations: HashMap<(String, String), f64>,
    /// Current activation state
    activations: HashMap<String, f64>,
    /// Noise level (higher during REM)
    noise_level: f64,
    /// Prefrontal inhibition (lower during REM)
    prefrontal_inhibition: f64,
}

#[allow(dead_code)]
struct ConceptNode {
    embedding: Vec<f64>,
    base_activation: f64,
    decay_rate: f64,
}

impl Default for DreamNeuralNetwork {
    fn default() -> Self {
        Self {
            concepts: HashMap::new(),
            associations: HashMap::new(),
            activations: HashMap::new(),
            noise_level: 0.1,
            prefrontal_inhibition: 1.0,
        }
    }
}

impl DreamNeuralNetwork {
    pub fn new() -> Self {
        Self::default()
    }

    /// Encode a concept into the network
    pub fn encode(&mut self, name: &str, embedding: Vec<f64>, importance: f64) {
        self.concepts.insert(name.to_string(), ConceptNode {
            embedding: embedding.clone(),
            base_activation: importance,
            decay_rate: 0.1,
        });
        self.activations.insert(name.to_string(), importance);
    }

    /// Create association between concepts
    pub fn associate(&mut self, from: &str, to: &str, strength: f64) {
        self.associations.insert((from.to_string(), to.to_string()), strength);
        self.associations.insert((to.to_string(), from.to_string()), strength * 0.8);
    }

    /// Enter REM state (reduced inhibition, increased noise)
    pub fn enter_rem(&mut self) {
        self.prefrontal_inhibition = 0.2; // Prefrontal cortex offline
        self.noise_level = 0.5; // More random activation
    }

    /// Exit REM state
    pub fn exit_rem(&mut self) {
        self.prefrontal_inhibition = 1.0;
        self.noise_level = 0.1;
    }

    /// Simulate one step of neural dynamics
    pub fn step(&mut self, dt: f64) -> Vec<(String, String, f64)> {
        let mut new_activations = HashMap::new();
        let mut novel_associations = Vec::new();

        // Spread activation through network
        for (concept, activation) in &self.activations {
            // Decay
            let decayed = activation * (1.0 - self.concepts[concept].decay_rate * dt);

            // Add noise (REM has more noise)
            let noise = (rand_float() - 0.5) * self.noise_level;

            // Collect input from associated concepts
            let mut input = 0.0;
            for ((from, to), strength) in &self.associations {
                if to == concept {
                    input += self.activations.get(from).unwrap_or(&0.0) * strength;
                }
            }

            // Apply prefrontal inhibition (limits unusual associations when awake)
            let gated_input = input * (1.0 - self.prefrontal_inhibition * 0.5);

            new_activations.insert(
                concept.clone(),
                (decayed + gated_input + noise).clamp(0.0, 1.0),
            );
        }

        // Detect novel co-activations (potential insights)
        let active_concepts: Vec<_> = new_activations.iter()
            .filter(|(_, &a)| a > 0.5)
            .map(|(c, _)| c.clone())
            .collect();

        for i in 0..active_concepts.len() {
            for j in i+1..active_concepts.len() {
                let c1 = &active_concepts[i];
                let c2 = &active_concepts[j];

                // Check if this is a novel association
                let key = (c1.clone(), c2.clone());
                if !self.associations.contains_key(&key) {
                    // Novel co-activation!
                    let strength = new_activations[c1] * new_activations[c2];
                    if strength > 0.3 {
                        novel_associations.push((c1.clone(), c2.clone(), strength));
                    }
                }
            }
        }

        self.activations = new_activations;
        novel_associations
    }

    /// Get most active concepts
    pub fn most_active(&self, n: usize) -> Vec<(String, f64)> {
        let mut sorted: Vec<_> = self.activations.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sorted.into_iter().take(n).collect()
    }
}

// Helper function for random floats
fn rand_float() -> f64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};

    let rs = RandomState::new();
    let mut hasher = rs.build_hasher();
    std::time::SystemTime::now().hash(&mut hasher);
    (hasher.finish() % 10000) as f64 / 10000.0
}
