//! Counterfactual Engine - What-If Reasoning
//!
//! "What would have happened if...?"
//!
//! Consciousness doesn't just predict the future - it imagines alternatives.
//! The counterfactual engine explores possible worlds that didn't happen,
//! enabling learning from hypothetical scenarios.
//!
//! ```text
//! COUNTERFACTUAL REASONING
//! ════════════════════════
//!
//!                    Actual World
//!                         │
//!    ┌────────────────────┼────────────────────┐
//!    │                    │                    │
//!    ▼                    ▼                    ▼
//! World A              World B              World C
//! (If I had           (If it had           (If they had
//!  turned left)        rained)              said yes)
//!    │                    │                    │
//!    ▼                    ▼                    ▼
//! Outcome A'           Outcome B'           Outcome C'
//!
//!
//! Pearl's Ladder of Causation:
//! ───────────────────────────
//! Level 3: Counterfactuals  │ P(Y_x | X', Y')
//!          "What if?"       │ Imagining alternatives
//!                          │
//! Level 2: Interventions   │ P(Y | do(X))
//!          "What if I do?" │ Experiments
//!                          │
//! Level 1: Association     │ P(Y | X)
//!          "What is?"      │ Observation
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::Result;
use super::causal_world::{CausalWorldModel, Intervention};

/// A counterfactual scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counterfactual {
    /// Unique ID
    pub id: Uuid,
    /// Description (e.g., "What if X had been Y?")
    pub description: String,
    /// The antecedent (the "if" part)
    pub antecedent: Antecedent,
    /// The consequent (what we're asking about)
    pub consequent: Consequent,
    /// Probability of the counterfactual being true
    pub probability: f64,
    /// Confidence in the estimate
    pub confidence: f64,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// The antecedent of a counterfactual (the hypothetical condition)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Antecedent {
    /// Variable that would be different
    pub variable: Uuid,
    /// Original value
    pub original_value: f64,
    /// Counterfactual value
    pub counterfactual_value: f64,
    /// Time of the hypothetical change
    pub time: u64,
}

/// The consequent of a counterfactual (what we're asking about)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequent {
    /// Variable we're asking about
    pub variable: Uuid,
    /// What actually happened
    pub actual_value: f64,
    /// What would have happened
    pub counterfactual_value: Option<f64>,
    /// Time of the outcome
    pub time: u64,
}

/// A what-if scenario with multiple changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfScenario {
    /// Unique ID
    pub id: Uuid,
    /// Description
    pub description: String,
    /// Interventions/changes to make
    pub interventions: Vec<Intervention>,
    /// Variables to observe
    pub observe: Vec<Uuid>,
    /// Results for each observed variable
    pub results: HashMap<Uuid, f64>,
    /// Comparison with actual world
    pub comparison: Comparison,
    /// Plausibility of this scenario
    pub plausibility: f64,
}

/// Comparison between actual and counterfactual worlds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    /// Variables that changed
    pub changed: Vec<(Uuid, f64, f64)>, // (variable, actual, counterfactual)
    /// Variables that stayed the same
    pub unchanged: Vec<Uuid>,
    /// Net effect (positive = counterfactual is better)
    pub net_effect: f64,
    /// Narrative explanation
    pub narrative: String,
}

/// Result of an intervention in a counterfactual world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionResult {
    /// The intervention that was made
    pub intervention: Intervention,
    /// Effects on other variables
    pub effects: HashMap<Uuid, f64>,
    /// Direct effects (immediate consequences)
    pub direct_effects: Vec<(Uuid, f64)>,
    /// Indirect effects (mediated through other variables)
    pub indirect_effects: Vec<(Uuid, Vec<Uuid>, f64)>, // (target, path, effect)
    /// Total effect
    pub total_effect: f64,
}

/// A possible world (state of all variables)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PossibleWorld {
    /// World ID
    pub id: Uuid,
    /// State of all variables
    pub state: HashMap<Uuid, f64>,
    /// Probability of this world
    pub probability: f64,
    /// Distance from actual world
    pub distance: f64,
    /// Accessibility from actual world
    pub accessible: bool,
}

impl PossibleWorld {
    pub fn new(state: HashMap<Uuid, f64>, probability: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            state,
            probability,
            distance: 0.0,
            accessible: true,
        }
    }

    /// Compute distance between two worlds
    pub fn distance_to(&self, other: &PossibleWorld) -> f64 {
        let mut distance = 0.0;
        let mut count = 0;

        for (var, &val) in &self.state {
            if let Some(&other_val) = other.state.get(var) {
                distance += (val - other_val).abs();
                count += 1;
            }
        }

        if count > 0 {
            distance / count as f64
        } else {
            0.0
        }
    }
}

/// Modal reasoning about possibility and necessity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalReasoning {
    /// Is the proposition necessarily true? (true in all accessible worlds)
    pub necessary: bool,
    /// Is the proposition possibly true? (true in some accessible world)
    pub possible: bool,
    /// Probability across accessible worlds
    pub probability: f64,
    /// Worlds where proposition is true
    pub true_worlds: Vec<Uuid>,
    /// Worlds where proposition is false
    pub false_worlds: Vec<Uuid>,
}

/// Configuration for the counterfactual engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualConfig {
    /// Number of possible worlds to consider
    pub num_worlds: usize,
    /// Distance threshold for accessibility
    pub accessibility_threshold: f64,
    /// Minimum plausibility threshold
    pub plausibility_threshold: f64,
    /// Enable structural counterfactuals (Pearl's approach)
    pub structural_counterfactuals: bool,
}

impl Default for CounterfactualConfig {
    fn default() -> Self {
        Self {
            num_worlds: 100,
            accessibility_threshold: 0.5,
            plausibility_threshold: 0.1,
            structural_counterfactuals: true,
        }
    }
}

/// The Counterfactual Engine
pub struct CounterfactualEngine {
    config: CounterfactualConfig,
    /// The actual world
    actual_world: PossibleWorld,
    /// Generated possible worlds
    possible_worlds: Vec<PossibleWorld>,
    /// History of evaluated counterfactuals
    history: Vec<Counterfactual>,
    /// Causal model for structural counterfactuals
    causal_model: CausalWorldModel,
}

impl CounterfactualEngine {
    pub fn new(config: CounterfactualConfig) -> Self {
        Self {
            config,
            actual_world: PossibleWorld::new(HashMap::new(), 1.0),
            possible_worlds: Vec::new(),
            history: Vec::new(),
            causal_model: CausalWorldModel::new(),
        }
    }

    /// Set the actual world state
    pub fn set_actual_world(&mut self, state: HashMap<Uuid, f64>) {
        self.actual_world = PossibleWorld::new(state, 1.0);
    }

    /// Set the causal model
    pub fn set_causal_model(&mut self, model: CausalWorldModel) {
        self.causal_model = model;
    }

    /// Evaluate a counterfactual: "What if A had been different?"
    pub fn evaluate(&mut self, antecedent: Antecedent, consequent_var: Uuid) -> Result<Counterfactual> {
        // Create counterfactual world by modifying the actual world
        let mut cf_state = self.actual_world.state.clone();
        cf_state.insert(antecedent.variable, antecedent.counterfactual_value);

        // Use causal model to propagate effects
        let cf_results = if self.config.structural_counterfactuals {
            self.structural_counterfactual(&antecedent)?
        } else {
            self.simple_counterfactual(&cf_state)
        };

        let cf_value = cf_results.get(&consequent_var).copied();
        let actual_value = self.actual_world.state.get(&consequent_var).copied().unwrap_or(0.0);

        let consequent = Consequent {
            variable: consequent_var,
            actual_value,
            counterfactual_value: cf_value,
            time: antecedent.time + 1,
        };

        // Estimate probability (based on world similarity)
        let probability = self.estimate_probability(&antecedent);

        let counterfactual = Counterfactual {
            id: Uuid::new_v4(),
            description: format!(
                "If {} had been {:.2} instead of {:.2}",
                antecedent.variable,
                antecedent.counterfactual_value,
                antecedent.original_value
            ),
            antecedent,
            consequent,
            probability,
            confidence: 0.7, // Base confidence
            evidence: Vec::new(),
        };

        // Store in history
        self.history.push(counterfactual.clone());

        Ok(counterfactual)
    }

    /// Structural counterfactual using causal model (Pearl's approach)
    fn structural_counterfactual(&mut self, antecedent: &Antecedent) -> Result<HashMap<Uuid, f64>> {
        // Step 1: Abduction - infer noise terms from actual values
        // Step 2: Action - modify the structural equations
        // Step 3: Prediction - compute counterfactual values

        let intervention = Intervention::new(
            antecedent.variable,
            antecedent.counterfactual_value,
            "Counterfactual intervention"
        );

        self.causal_model.intervene(intervention)
    }

    /// Simple counterfactual (linear approximation)
    fn simple_counterfactual(&self, cf_state: &HashMap<Uuid, f64>) -> HashMap<Uuid, f64> {
        // Simple approach: assume proportional effects
        let mut results = cf_state.clone();

        // Propagate changes (simplified)
        for (&var, &val) in cf_state {
            let actual_val = self.actual_world.state.get(&var).copied().unwrap_or(0.0);
            let change = val - actual_val;

            // Apply to connected variables (simplified - just copy for now)
            for (&other_var, other_val) in results.iter_mut() {
                if other_var != var {
                    *other_val += change * 0.5; // Assume 50% transmission
                }
            }
        }

        results
    }

    /// Estimate probability of counterfactual world
    fn estimate_probability(&self, antecedent: &Antecedent) -> f64 {
        // Based on "minimal change" principle
        let change = (antecedent.counterfactual_value - antecedent.original_value).abs();

        // Smaller changes are more probable
        let plausibility = (-change).exp();

        plausibility.clamp(0.01, 0.99)
    }

    /// Create a what-if scenario
    pub fn what_if(&mut self, interventions: Vec<Intervention>, observe: Vec<Uuid>) -> Result<WhatIfScenario> {
        // Apply all interventions
        let mut current_state = self.actual_world.state.clone();

        for intervention in &interventions {
            current_state.insert(intervention.target, intervention.value);
        }

        // Propagate through causal model
        let mut results = HashMap::new();
        for var in &observe {
            if let Some(&val) = current_state.get(var) {
                results.insert(*var, val);
            }
        }

        // Compare with actual world
        let mut changed = Vec::new();
        let mut unchanged = Vec::new();

        for &var in &observe {
            let actual = self.actual_world.state.get(&var).copied().unwrap_or(0.0);
            let counterfactual = results.get(&var).copied().unwrap_or(0.0);

            if (actual - counterfactual).abs() > 0.01 {
                changed.push((var, actual, counterfactual));
            } else {
                unchanged.push(var);
            }
        }

        let net_effect: f64 = changed.iter()
            .map(|(_, actual, cf)| cf - actual)
            .sum();

        let comparison = Comparison {
            changed,
            unchanged,
            net_effect,
            narrative: self.generate_narrative(&interventions, net_effect),
        };

        // Calculate plausibility
        let plausibility = interventions.iter()
            .map(|i| {
                let actual = self.actual_world.state.get(&i.target).copied().unwrap_or(0.0);
                let change = (i.value - actual).abs();
                (-change).exp()
            })
            .product::<f64>();

        let scenario = WhatIfScenario {
            id: Uuid::new_v4(),
            description: format!(
                "What if {} intervention(s) were made?",
                interventions.len()
            ),
            interventions,
            observe,
            results,
            comparison,
            plausibility,
        };

        Ok(scenario)
    }

    /// Generate a narrative explanation
    fn generate_narrative(&self, interventions: &[Intervention], net_effect: f64) -> String {
        let direction = if net_effect > 0.0 { "better" } else if net_effect < 0.0 { "worse" } else { "the same" };

        format!(
            "If {} changes had been made, outcomes would have been {}. Net effect: {:.3}",
            interventions.len(),
            direction,
            net_effect
        )
    }

    /// Generate possible worlds
    pub fn generate_possible_worlds(&mut self, variations: usize) {
        self.possible_worlds.clear();

        // The actual world is always included
        self.possible_worlds.push(self.actual_world.clone());

        // Generate variations
        for _ in 0..variations.min(self.config.num_worlds - 1) {
            let mut state = self.actual_world.state.clone();

            // Randomly perturb some variables
            for (_, val) in state.iter_mut() {
                let perturbation = (rand::random::<f64>() - 0.5) * 0.2;
                *val = (*val + perturbation).clamp(0.0, 1.0);
            }

            let probability = rand::random::<f64>() * 0.5;
            let mut world = PossibleWorld::new(state, probability);

            world.distance = world.distance_to(&self.actual_world);
            world.accessible = world.distance < self.config.accessibility_threshold;

            self.possible_worlds.push(world);
        }
    }

    /// Modal reasoning: is a proposition necessarily/possibly true?
    pub fn modal_reason(&self, proposition: impl Fn(&HashMap<Uuid, f64>) -> bool) -> ModalReasoning {
        let mut true_worlds = Vec::new();
        let mut false_worlds = Vec::new();
        let mut total_prob = 0.0;
        let mut true_prob = 0.0;

        for world in &self.possible_worlds {
            if !world.accessible {
                continue;
            }

            total_prob += world.probability;

            if proposition(&world.state) {
                true_worlds.push(world.id);
                true_prob += world.probability;
            } else {
                false_worlds.push(world.id);
            }
        }

        let probability = if total_prob > 0.0 {
            true_prob / total_prob
        } else {
            0.0
        };

        ModalReasoning {
            necessary: false_worlds.is_empty() && !true_worlds.is_empty(),
            possible: !true_worlds.is_empty(),
            probability,
            true_worlds,
            false_worlds,
        }
    }

    /// Get actual world
    pub fn actual_world(&self) -> &PossibleWorld {
        &self.actual_world
    }

    /// Get possible worlds
    pub fn possible_worlds(&self) -> &[PossibleWorld] {
        &self.possible_worlds
    }

    /// Get accessible worlds
    pub fn accessible_worlds(&self) -> Vec<&PossibleWorld> {
        self.possible_worlds.iter()
            .filter(|w| w.accessible)
            .collect()
    }

    /// Get counterfactual history
    pub fn history(&self) -> &[Counterfactual] {
        &self.history
    }

    /// Get causal model
    pub fn causal_model(&self) -> &CausalWorldModel {
        &self.causal_model
    }

    /// Get mutable causal model
    pub fn causal_model_mut(&mut self) -> &mut CausalWorldModel {
        &mut self.causal_model
    }
}

impl Default for CounterfactualEngine {
    fn default() -> Self {
        Self::new(CounterfactualConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_world() {
        let mut state = HashMap::new();
        state.insert(Uuid::new_v4(), 0.5);
        state.insert(Uuid::new_v4(), 0.7);

        let world = PossibleWorld::new(state, 0.5);
        assert!(world.accessible);
    }

    #[test]
    fn test_world_distance() {
        let var = Uuid::new_v4();

        let mut state1 = HashMap::new();
        state1.insert(var, 0.5);
        let world1 = PossibleWorld::new(state1, 0.5);

        let mut state2 = HashMap::new();
        state2.insert(var, 0.8);
        let world2 = PossibleWorld::new(state2, 0.5);

        let distance = world1.distance_to(&world2);
        assert!((distance - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_counterfactual_engine() {
        let mut engine = CounterfactualEngine::default();

        let var = Uuid::new_v4();
        let mut state = HashMap::new();
        state.insert(var, 0.5);
        engine.set_actual_world(state);

        let antecedent = Antecedent {
            variable: var,
            original_value: 0.5,
            counterfactual_value: 0.8,
            time: 0,
        };

        let result = engine.evaluate(antecedent, var);
        assert!(result.is_ok());
    }

    #[test]
    fn test_what_if_scenario() {
        let mut engine = CounterfactualEngine::default();

        let var = Uuid::new_v4();
        let mut state = HashMap::new();
        state.insert(var, 0.5);
        engine.set_actual_world(state);

        let interventions = vec![
            Intervention::new(var, 0.8, "Test")
        ];

        let result = engine.what_if(interventions, vec![var]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_modal_reasoning() {
        let mut engine = CounterfactualEngine::default();

        let var = Uuid::new_v4();
        let mut state = HashMap::new();
        state.insert(var, 0.5);
        engine.set_actual_world(state);

        engine.generate_possible_worlds(10);

        let reasoning = engine.modal_reason(|s| {
            s.get(&var).copied().unwrap_or(0.0) > 0.3
        });

        // Should be at least possible
        assert!(reasoning.possible);
    }
}
