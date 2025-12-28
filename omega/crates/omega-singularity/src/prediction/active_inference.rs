//! Active Inference - Actions as Self-Fulfilling Prophecies
//!
//! "We don't just predict the future - we CREATE it."
//!
//! Active inference unifies perception and action as a single process:
//! minimizing the difference between predictions and reality. We can either
//! update our beliefs (perception) or change reality (action) to match
//! our predictions.
//!
//! ```text
//! ACTIVE INFERENCE
//! ════════════════
//!
//!         ┌─────────────────────────────────────────────┐
//!         │              GENERATIVE MODEL               │
//!         │         (What I believe/want/expect)        │
//!         └──────────────────┬──────────────────────────┘
//!                            │
//!                            ▼
//!                   ┌────────────────┐
//!                   │   PREDICTION   │
//!                   └────────┬───────┘
//!                            │
//!              ┌─────────────┴─────────────┐
//!              │                           │
//!              ▼                           ▼
//!    ┌─────────────────┐         ┌─────────────────┐
//!    │   PERCEPTION    │         │     ACTION      │
//!    │ Update beliefs  │         │ Change reality  │
//!    │ to match world  │         │ to match beliefs│
//!    └────────┬────────┘         └────────┬────────┘
//!              │                           │
//!              └─────────────┬─────────────┘
//!                            │
//!                            ▼
//!                ┌────────────────────┐
//!                │ MINIMIZE FREE ENERGY│
//!                │ (Reduce prediction  │
//!                │  error / surprise)  │
//!                └────────────────────┘
//! ```
//!
//! Key insight: Actions are PREDICTIONS about our future sensory states.
//! When we reach for a cup, we're not executing a motor command - we're
//! predicting what we will feel, and letting our body fulfill that prediction.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use super::Result;

/// Expected free energy for an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedFreeEnergy {
    /// Action/policy ID
    pub action_id: Uuid,
    /// Action description
    pub description: String,
    /// Pragmatic value (achieving goals)
    pub pragmatic_value: f64,
    /// Epistemic value (reducing uncertainty)
    pub epistemic_value: f64,
    /// Risk (KL divergence from preferred states)
    pub risk: f64,
    /// Ambiguity (entropy of expected outcomes)
    pub ambiguity: f64,
    /// Total expected free energy (lower is better)
    pub total: f64,
}

impl ExpectedFreeEnergy {
    pub fn new(action_id: Uuid, description: impl Into<String>) -> Self {
        Self {
            action_id,
            description: description.into(),
            pragmatic_value: 0.0,
            epistemic_value: 0.0,
            risk: 0.0,
            ambiguity: 0.0,
            total: 0.0,
        }
    }

    /// Compute total EFE
    pub fn compute(&mut self) {
        // G = risk + ambiguity - epistemic_value
        // Lower G = better action
        self.total = self.risk + self.ambiguity - self.epistemic_value;
    }

    /// Is this action worth taking?
    pub fn is_beneficial(&self) -> bool {
        self.pragmatic_value > 0.0 || self.epistemic_value > self.risk
    }
}

/// A policy (sequence of actions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Policy ID
    pub id: Uuid,
    /// Sequence of action predictions
    pub actions: Vec<ActionPrediction>,
    /// Expected free energy of this policy
    pub efe: f64,
    /// Probability of selecting this policy
    pub probability: f64,
    /// Expected trajectory
    pub trajectory: Vec<Vec<f64>>,
}

/// Selection among policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySelection {
    /// Evaluated policies
    pub policies: Vec<Policy>,
    /// Selected policy
    pub selected: Uuid,
    /// Selection confidence
    pub confidence: f64,
    /// Posterior over policies
    pub posterior: Vec<f64>,
    /// Next action to take
    pub next_action: Option<ActionPrediction>,
}

/// An action as a prediction about future states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPrediction {
    /// Action ID
    pub id: Uuid,
    /// Predicted sensory state after action
    pub predicted_state: Vec<f64>,
    /// Precision of the prediction
    pub precision: f64,
    /// Motor command (proprioceptive prediction)
    pub motor_command: Vec<f64>,
    /// Time horizon
    pub horizon: u64,
}

impl ActionPrediction {
    pub fn new(predicted_state: Vec<f64>, motor_command: Vec<f64>) -> Self {
        Self {
            id: Uuid::new_v4(),
            predicted_state,
            precision: 1.0,
            motor_command,
            horizon: 1,
        }
    }
}

/// A self-fulfilling prophecy - action that creates its prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfFulfillingProphecy {
    /// Prophecy ID
    pub id: Uuid,
    /// The predicted state
    pub prediction: Vec<f64>,
    /// Actions taken to fulfill
    pub actions: Vec<ActionPrediction>,
    /// Actual outcome
    pub outcome: Option<Vec<f64>>,
    /// Was the prophecy fulfilled?
    pub fulfilled: bool,
    /// Fulfillment error
    pub error: f64,
    /// Time taken to fulfill
    pub fulfillment_time: Option<u64>,
}

/// Belief update from prediction error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefUpdate {
    /// Updated belief state
    pub new_belief: Vec<f64>,
    /// Prediction error that triggered update
    pub prediction_error: Vec<f64>,
    /// Update magnitude
    pub magnitude: f64,
    /// Precision-weighted update
    pub precision_weighted: bool,
    /// Hierarchy level of update
    pub level: usize,
}

/// Configuration for active inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveInferenceConfig {
    /// State dimension
    pub state_dim: usize,
    /// Action dimension
    pub action_dim: usize,
    /// Number of policies to consider
    pub num_policies: usize,
    /// Planning horizon
    pub horizon: usize,
    /// Exploration-exploitation balance (temperature)
    pub temperature: f64,
    /// Prior precision
    pub prior_precision: f64,
    /// Goal precision (how much we care about goals)
    pub goal_precision: f64,
}

impl Default for ActiveInferenceConfig {
    fn default() -> Self {
        Self {
            state_dim: 64,
            action_dim: 8,
            num_policies: 10,
            horizon: 5,
            temperature: 1.0,
            prior_precision: 1.0,
            goal_precision: 4.0,
        }
    }
}

/// The Omega Active Inference Engine
pub struct OmegaActiveInference {
    config: ActiveInferenceConfig,
    /// Current belief state
    belief: Vec<f64>,
    /// Prior/preferred state (goals)
    preferred: Vec<f64>,
    /// Action repertoire
    actions: Vec<ActionPrediction>,
    /// Current policies under consideration
    policies: Vec<Policy>,
    /// Selected policy
    selected_policy: Option<Uuid>,
    /// History of prophecies
    prophecies: Vec<SelfFulfillingProphecy>,
    /// Free energy history
    free_energy_history: VecDeque<f64>,
    /// Current free energy
    current_free_energy: f64,
}

impl OmegaActiveInference {
    pub fn new(config: ActiveInferenceConfig) -> Self {
        Self {
            belief: vec![0.0; config.state_dim],
            preferred: vec![0.0; config.state_dim],
            actions: Vec::new(),
            policies: Vec::new(),
            selected_policy: None,
            prophecies: Vec::new(),
            free_energy_history: VecDeque::with_capacity(100),
            current_free_energy: 0.0,
            config,
        }
    }

    /// Set the preferred/goal state
    pub fn set_preferences(&mut self, preferred: Vec<f64>) {
        self.preferred = preferred;
    }

    /// Add an action to the repertoire
    pub fn add_action(&mut self, action: ActionPrediction) {
        self.actions.push(action);
    }

    /// Update belief based on observation
    pub fn update_belief(&mut self, observation: &[f64]) -> BeliefUpdate {
        // Compute prediction error
        let prediction_error: Vec<f64> = observation.iter()
            .zip(self.belief.iter())
            .map(|(o, b)| o - b)
            .collect();

        let magnitude: f64 = prediction_error.iter()
            .map(|e| e * e)
            .sum::<f64>()
            .sqrt();

        // Precision-weighted belief update
        let precision = self.config.prior_precision;
        for (i, &error) in prediction_error.iter().enumerate() {
            if i < self.belief.len() {
                self.belief[i] += precision * error * 0.1;
            }
        }

        // Update free energy
        self.current_free_energy = self.compute_free_energy(observation);
        self.free_energy_history.push_back(self.current_free_energy);
        if self.free_energy_history.len() > 100 {
            self.free_energy_history.pop_front();
        }

        BeliefUpdate {
            new_belief: self.belief.clone(),
            prediction_error,
            magnitude,
            precision_weighted: true,
            level: 0,
        }
    }

    /// Compute variational free energy
    fn compute_free_energy(&self, observation: &[f64]) -> f64 {
        // F = E_q[log q(x)] - E_q[log p(o,x)]
        // Simplified: F ≈ prediction_error + KL(belief || prior)

        // Prediction error
        let prediction_error: f64 = observation.iter()
            .zip(self.belief.iter())
            .map(|(o, b)| (o - b).powi(2))
            .sum::<f64>()
            * self.config.prior_precision;

        // KL from preferred (goals)
        let kl: f64 = self.belief.iter()
            .zip(self.preferred.iter())
            .map(|(b, p)| {
                let b_pos = b.abs().max(1e-10);
                let p_pos = p.abs().max(1e-10);
                b_pos * (b_pos / p_pos).ln()
            })
            .sum::<f64>()
            * self.config.goal_precision * 0.01;

        prediction_error + kl
    }

    /// Compute expected free energy for an action
    fn compute_efe(&self, action: &ActionPrediction) -> ExpectedFreeEnergy {
        let mut efe = ExpectedFreeEnergy::new(action.id, "Action");

        // Pragmatic value: expected utility of outcome
        efe.pragmatic_value = action.predicted_state.iter()
            .zip(self.preferred.iter())
            .map(|(p, pref)| -((p - pref).powi(2)))
            .sum::<f64>();

        // Epistemic value: expected information gain
        // Simplified: actions that move us to uncertain regions are epistemic
        let uncertainty: f64 = action.predicted_state.iter()
            .map(|&p| {
                let p_bounded = p.clamp(0.01, 0.99);
                -p_bounded * p_bounded.ln() - (1.0 - p_bounded) * (1.0 - p_bounded).ln()
            })
            .sum::<f64>();
        efe.epistemic_value = uncertainty * 0.1;

        // Risk: KL divergence from preferred
        efe.risk = action.predicted_state.iter()
            .zip(self.preferred.iter())
            .map(|(p, pref)| (p - pref).powi(2))
            .sum::<f64>()
            * self.config.goal_precision;

        // Ambiguity: uncertainty about outcome
        efe.ambiguity = 1.0 / (action.precision + 0.1);

        efe.compute();
        efe
    }

    /// Generate and evaluate policies
    pub fn plan(&mut self) -> PolicySelection {
        self.policies.clear();

        // Generate policies
        for _ in 0..self.config.num_policies {
            let mut policy_actions = Vec::new();
            let mut trajectory = Vec::new();
            let mut current_state = self.belief.clone();

            // Plan ahead
            for _ in 0..self.config.horizon {
                // Select action for this step
                if let Some(action) = self.actions.choose_random() {
                    policy_actions.push(action.clone());

                    // Predict next state
                    current_state = action.predicted_state.clone();
                    trajectory.push(current_state.clone());
                }
            }

            // Compute EFE for this policy
            let efe: f64 = policy_actions.iter()
                .map(|a| self.compute_efe(a).total)
                .sum();

            let policy = Policy {
                id: Uuid::new_v4(),
                actions: policy_actions,
                efe,
                probability: 0.0, // Will be computed
                trajectory,
            };

            self.policies.push(policy);
        }

        // Compute posterior over policies (softmax)
        let mut posteriors = Vec::with_capacity(self.policies.len());
        let min_efe = self.policies.iter()
            .map(|p| p.efe)
            .fold(f64::INFINITY, f64::min);

        let mut total = 0.0;
        for policy in &self.policies {
            let exp_neg_efe = (-(policy.efe - min_efe) / self.config.temperature).exp();
            posteriors.push(exp_neg_efe);
            total += exp_neg_efe;
        }

        // Normalize
        for (i, post) in posteriors.iter_mut().enumerate() {
            *post /= total.max(1e-10);
            self.policies[i].probability = *post;
        }

        // Select best policy
        let selected_idx = posteriors.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        let selected = self.policies.get(selected_idx).map(|p| p.id).unwrap_or(Uuid::nil());
        self.selected_policy = Some(selected);

        let next_action = self.policies.get(selected_idx)
            .and_then(|p| p.actions.first().cloned());

        let confidence = posteriors.get(selected_idx).copied().unwrap_or(0.0);

        PolicySelection {
            policies: self.policies.clone(),
            selected,
            confidence,
            posterior: posteriors,
            next_action,
        }
    }

    /// Execute an action (create self-fulfilling prophecy)
    pub fn act(&mut self, action: &ActionPrediction) -> SelfFulfillingProphecy {
        let prophecy = SelfFulfillingProphecy {
            id: Uuid::new_v4(),
            prediction: action.predicted_state.clone(),
            actions: vec![action.clone()],
            outcome: None,
            fulfilled: false,
            error: 0.0,
            fulfillment_time: None,
        };

        self.prophecies.push(prophecy.clone());
        prophecy
    }

    /// Check if a prophecy was fulfilled
    pub fn check_prophecy(&mut self, prophecy_id: Uuid, outcome: &[f64]) -> Option<bool> {
        for prophecy in &mut self.prophecies {
            if prophecy.id == prophecy_id {
                prophecy.outcome = Some(outcome.to_vec());

                // Compute fulfillment error
                prophecy.error = prophecy.prediction.iter()
                    .zip(outcome.iter())
                    .map(|(p, o)| (p - o).powi(2))
                    .sum::<f64>()
                    .sqrt();

                prophecy.fulfilled = prophecy.error < 0.3;

                return Some(prophecy.fulfilled);
            }
        }
        None
    }

    /// Get current belief
    pub fn belief(&self) -> &[f64] {
        &self.belief
    }

    /// Get preferred state
    pub fn preferred(&self) -> &[f64] {
        &self.preferred
    }

    /// Get current free energy
    pub fn free_energy(&self) -> f64 {
        self.current_free_energy
    }

    /// Get prophecy fulfillment rate
    pub fn prophecy_fulfillment_rate(&self) -> f64 {
        let completed: Vec<_> = self.prophecies.iter()
            .filter(|p| p.outcome.is_some())
            .collect();

        if completed.is_empty() {
            return 0.5;
        }

        let fulfilled = completed.iter().filter(|p| p.fulfilled).count();
        fulfilled as f64 / completed.len() as f64
    }

    /// Get action repertoire
    pub fn actions(&self) -> &[ActionPrediction] {
        &self.actions
    }

    /// Get selected policy
    pub fn selected_policy(&self) -> Option<&Policy> {
        self.selected_policy.and_then(|id| {
            self.policies.iter().find(|p| p.id == id)
        })
    }

    /// Reset the engine
    pub fn reset(&mut self) {
        self.belief = vec![0.0; self.config.state_dim];
        self.policies.clear();
        self.selected_policy = None;
        self.free_energy_history.clear();
        self.current_free_energy = 0.0;
    }
}

/// Extension trait for random selection
trait ChooseRandom<T> {
    fn choose_random(&self) -> Option<&T>;
}

impl<T> ChooseRandom<T> for Vec<T> {
    fn choose_random(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let idx = (rand::random::<f64>() * self.len() as f64) as usize;
            self.get(idx.min(self.len() - 1))
        }
    }
}

impl Default for OmegaActiveInference {
    fn default() -> Self {
        Self::new(ActiveInferenceConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_inference_creation() {
        let ai = OmegaActiveInference::default();
        assert_eq!(ai.belief().len(), 64);
    }

    #[test]
    fn test_belief_update() {
        let mut ai = OmegaActiveInference::default();

        let observation = vec![0.5; 64];
        let update = ai.update_belief(&observation);

        assert!(update.magnitude > 0.0);
    }

    #[test]
    fn test_set_preferences() {
        let mut ai = OmegaActiveInference::default();

        let goals = vec![1.0; 64];
        ai.set_preferences(goals.clone());

        assert_eq!(ai.preferred(), goals.as_slice());
    }

    #[test]
    fn test_action_prediction() {
        let predicted = vec![0.5; 10];
        let motor = vec![0.3; 5];
        let action = ActionPrediction::new(predicted.clone(), motor);

        assert_eq!(action.predicted_state, predicted);
    }

    #[test]
    fn test_efe_computation() {
        let ai = OmegaActiveInference::default();

        let action = ActionPrediction::new(
            vec![0.5; 64],
            vec![0.3; 8]
        );

        let efe = ai.compute_efe(&action);
        assert!(efe.total.is_finite());
    }

    #[test]
    fn test_planning() {
        let mut ai = OmegaActiveInference::default();

        // Add some actions
        for i in 0..5 {
            let action = ActionPrediction::new(
                vec![(i as f64) / 10.0; 64],
                vec![0.1; 8]
            );
            ai.add_action(action);
        }

        let selection = ai.plan();
        assert!(!selection.policies.is_empty());
    }

    #[test]
    fn test_prophecy() {
        let mut ai = OmegaActiveInference::default();

        let action = ActionPrediction::new(
            vec![0.5; 64],
            vec![0.3; 8]
        );

        let prophecy = ai.act(&action);
        assert!(!prophecy.fulfilled);

        // Fulfill the prophecy
        let outcome = vec![0.5; 64];
        let fulfilled = ai.check_prophecy(prophecy.id, &outcome);

        assert!(fulfilled.is_some());
        assert!(fulfilled.unwrap());
    }
}
