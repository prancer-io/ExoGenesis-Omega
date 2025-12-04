//! Proximal Policy Optimization for architecture refinement

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PPOError {
    #[error("Optimization failed: {0}")]
    OptimizationFailed(String),

    #[error("Invalid trajectory: {0}")]
    InvalidTrajectory(String),
}

/// PPO configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPOConfig {
    /// Clipping parameter (epsilon)
    pub clip_epsilon: f64,

    /// Value function coefficient
    pub value_coef: f64,

    /// Entropy coefficient for exploration
    pub entropy_coef: f64,

    /// Learning rate
    pub learning_rate: f64,

    /// Batch size
    pub batch_size: usize,

    /// Number of epochs per update
    pub epochs: usize,

    /// Discount factor
    pub gamma: f64,

    /// GAE lambda
    pub gae_lambda: f64,
}

impl Default for PPOConfig {
    fn default() -> Self {
        Self {
            clip_epsilon: 0.2,
            value_coef: 0.5,
            entropy_coef: 0.01,
            learning_rate: 3e-4,
            batch_size: 64,
            epochs: 4,
            gamma: 0.99,
            gae_lambda: 0.95,
        }
    }
}

/// A trajectory of experiences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub states: Vec<Vec<f32>>,
    pub actions: Vec<Vec<f32>>,
    pub rewards: Vec<f64>,
    pub values: Vec<f64>,
    pub log_probs: Vec<f64>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            actions: Vec::new(),
            rewards: Vec::new(),
            values: Vec::new(),
            log_probs: Vec::new(),
        }
    }

    pub fn add_step(
        &mut self,
        state: Vec<f32>,
        action: Vec<f32>,
        reward: f64,
        value: f64,
        log_prob: f64,
    ) {
        self.states.push(state);
        self.actions.push(action);
        self.rewards.push(reward);
        self.values.push(value);
        self.log_probs.push(log_prob);
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

/// Policy network for PPO
pub struct PolicyNetwork {
    // Simplified - would be a neural network in production
    pub input_dim: usize,
    pub output_dim: usize,
}

impl PolicyNetwork {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        Self { input_dim, output_dim }
    }

    pub fn forward(&self, _state: &[f32]) -> Vec<f32> {
        vec![0.5; self.output_dim]
    }

    pub fn log_prob(&self, _state: &[f32], _action: &[f32]) -> f64 {
        0.0
    }

    pub fn entropy(&self, _state: &[f32]) -> f64 {
        1.0
    }
}

/// Value network for PPO
pub struct ValueNetwork {
    // Simplified - would be a neural network in production
    pub input_dim: usize,
}

impl ValueNetwork {
    pub fn new(input_dim: usize) -> Self {
        Self { input_dim }
    }

    pub fn forward(&self, _state: &[f32]) -> f64 {
        0.5
    }
}

/// Result of PPO optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub iterations: usize,
    pub final_loss: f64,
    pub policy_loss: f64,
    pub value_loss: f64,
    pub entropy: f64,
}

/// PPO optimizer for architecture hyperparameters
pub struct PPOOptimizer {
    config: PPOConfig,
    policy_network: PolicyNetwork,
    value_network: ValueNetwork,
}

impl PPOOptimizer {
    pub fn new(config: PPOConfig) -> Self {
        let policy_network = PolicyNetwork::new(4096, 128);
        let value_network = ValueNetwork::new(4096);

        Self {
            config,
            policy_network,
            value_network,
        }
    }

    /// Optimize architecture hyperparameters using PPO
    pub async fn optimize(
        &mut self,
        trajectories: &[Trajectory],
    ) -> Result<OptimizationResult, PPOError> {
        if trajectories.is_empty() {
            return Err(PPOError::InvalidTrajectory("Empty trajectories".to_string()));
        }

        let mut total_policy_loss = 0.0;
        let mut total_value_loss = 0.0;
        let mut total_entropy = 0.0;

        for epoch in 0..self.config.epochs {
            for trajectory in trajectories {
                // Compute advantages using GAE
                let advantages = self.compute_advantages(trajectory);

                // Compute returns
                let returns = self.compute_returns(trajectory);

                // Update policy and value networks
                for i in 0..trajectory.len() {
                    let state = &trajectory.states[i];
                    let action = &trajectory.actions[i];
                    let old_log_prob = trajectory.log_probs[i];
                    let advantage = advantages[i];
                    let return_val = returns[i];

                    // Compute new log probability
                    let new_log_prob = self.policy_network.log_prob(state, action);

                    // Compute probability ratio
                    let ratio = ((new_log_prob - old_log_prob).exp()).min(10.0).max(0.1);

                    // Clipped surrogate objective
                    let clipped_ratio = ratio.clamp(
                        1.0 - self.config.clip_epsilon,
                        1.0 + self.config.clip_epsilon,
                    );

                    let policy_loss = -(ratio * advantage).min(clipped_ratio * advantage);

                    // Value loss
                    let value_pred = self.value_network.forward(state);
                    let value_loss = (value_pred - return_val).powi(2);

                    // Entropy bonus
                    let entropy = self.policy_network.entropy(state);

                    total_policy_loss += policy_loss;
                    total_value_loss += value_loss;
                    total_entropy += entropy;
                }
            }

            tracing::debug!(
                "PPO epoch {}/{}: policy_loss={:.4}, value_loss={:.4}",
                epoch + 1,
                self.config.epochs,
                total_policy_loss,
                total_value_loss
            );
        }

        let num_steps = trajectories.iter().map(|t| t.len()).sum::<usize>() as f64;

        Ok(OptimizationResult {
            iterations: self.config.epochs,
            final_loss: (total_policy_loss + total_value_loss) / num_steps,
            policy_loss: total_policy_loss / num_steps,
            value_loss: total_value_loss / num_steps,
            entropy: total_entropy / num_steps,
        })
    }

    /// Compute advantages using Generalized Advantage Estimation (GAE)
    pub fn compute_advantages(&self, trajectory: &Trajectory) -> Vec<f64> {
        let mut advantages = vec![0.0; trajectory.len()];
        let mut gae = 0.0;

        for t in (0..trajectory.len()).rev() {
            let reward = trajectory.rewards[t];
            let value = trajectory.values[t];
            let next_value = if t + 1 < trajectory.len() {
                trajectory.values[t + 1]
            } else {
                0.0
            };

            let delta = reward + self.config.gamma * next_value - value;
            gae = delta + self.config.gamma * self.config.gae_lambda * gae;
            advantages[t] = gae;
        }

        // Normalize advantages
        let mean = advantages.iter().sum::<f64>() / advantages.len() as f64;
        let std = (advantages.iter()
            .map(|a| (a - mean).powi(2))
            .sum::<f64>() / advantages.len() as f64)
            .sqrt()
            .max(1e-8);

        advantages.iter().map(|a| (a - mean) / std).collect()
    }

    /// Compute returns (discounted sum of rewards)
    fn compute_returns(&self, trajectory: &Trajectory) -> Vec<f64> {
        let mut returns = vec![0.0; trajectory.len()];
        let mut return_val = 0.0;

        for t in (0..trajectory.len()).rev() {
            return_val = trajectory.rewards[t] + self.config.gamma * return_val;
            returns[t] = return_val;
        }

        returns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ppo_optimize() {
        let config = PPOConfig::default();
        let mut optimizer = PPOOptimizer::new(config);

        let mut trajectory = Trajectory::new();
        trajectory.add_step(
            vec![0.0; 4096],
            vec![0.5; 128],
            1.0,
            0.5,
            0.0,
        );

        let result = optimizer.optimize(&[trajectory]).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_compute_advantages() {
        let config = PPOConfig::default();
        let optimizer = PPOOptimizer::new(config);

        let mut trajectory = Trajectory::new();
        for _ in 0..10 {
            trajectory.add_step(
                vec![0.0; 4096],
                vec![0.5; 128],
                1.0,
                0.5,
                0.0,
            );
        }

        let advantages = optimizer.compute_advantages(&trajectory);
        assert_eq!(advantages.len(), 10);
    }
}
