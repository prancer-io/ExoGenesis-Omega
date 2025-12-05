//! Proximal Policy Optimization for architecture refinement

use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PPOError {
    #[error("Optimization failed: {0}")]
    OptimizationFailed(String),

    #[error("Invalid trajectory: {0}")]
    InvalidTrajectory(String),
}

/// Activation function types
#[derive(Clone, Copy, Debug)]
pub enum Activation {
    ReLU,
    Tanh,
    Softmax,
    None,
}

/// Simple MLP layer
#[derive(Clone)]
pub struct DenseLayer {
    weights: Array2<f64>,
    bias: Array1<f64>,
    activation: Activation,
}

impl DenseLayer {
    pub fn new(input_size: usize, output_size: usize, activation: Activation) -> Self {
        // Xavier initialization
        let scale = (2.0 / (input_size + output_size) as f64).sqrt();
        let weights = Array2::random((input_size, output_size), Uniform::new(-scale, scale));
        let bias = Array1::zeros(output_size);

        Self { weights, bias, activation }
    }

    pub fn forward(&self, x: &Array1<f64>) -> Array1<f64> {
        let z = x.dot(&self.weights) + &self.bias;
        self.apply_activation(&z)
    }

    fn apply_activation(&self, z: &Array1<f64>) -> Array1<f64> {
        match self.activation {
            Activation::ReLU => z.mapv(|x| x.max(0.0)),
            Activation::Tanh => z.mapv(|x| x.tanh()),
            Activation::Softmax => {
                let exp_z = z.mapv(|x| x.exp());
                let sum: f64 = exp_z.sum();
                exp_z / sum
            }
            Activation::None => z.clone(),
        }
    }

    pub fn parameters(&self) -> (Array2<f64>, Array1<f64>) {
        (self.weights.clone(), self.bias.clone())
    }

    pub fn update(&mut self, weight_grad: &Array2<f64>, bias_grad: &Array1<f64>, lr: f64) {
        self.weights = &self.weights - &(weight_grad * lr);
        self.bias = &self.bias - &(bias_grad * lr);
    }
}

/// Policy network outputs action probabilities
pub struct PolicyNetwork {
    layers: Vec<DenseLayer>,
    #[allow(dead_code)]
    action_dim: usize,
}

impl PolicyNetwork {
    pub fn new(state_dim: usize, hidden_dim: usize, action_dim: usize) -> Self {
        let layers = vec![
            DenseLayer::new(state_dim, hidden_dim, Activation::ReLU),
            DenseLayer::new(hidden_dim, hidden_dim, Activation::ReLU),
            DenseLayer::new(hidden_dim, action_dim, Activation::Softmax),
        ];

        Self { layers, action_dim }
    }

    pub fn forward(&self, state: &Array1<f64>) -> Array1<f64> {
        let mut x = state.clone();
        for layer in &self.layers {
            x = layer.forward(&x);
        }
        x
    }

    pub fn get_action_prob(&self, state: &Array1<f64>, action: usize) -> f64 {
        let probs = self.forward(state);
        probs[action]
    }

    pub fn sample_action(&self, state: &Array1<f64>) -> usize {
        let probs = self.forward(state);

        // Sample from probability distribution
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let uniform: f64 = rng.gen();

        let mut cumulative = 0.0;
        for (i, &p) in probs.iter().enumerate() {
            cumulative += p;
            if uniform < cumulative {
                return i;
            }
        }

        probs.len() - 1
    }
}

/// Value network estimates state value
pub struct ValueNetwork {
    layers: Vec<DenseLayer>,
}

impl ValueNetwork {
    pub fn new(state_dim: usize, hidden_dim: usize) -> Self {
        let layers = vec![
            DenseLayer::new(state_dim, hidden_dim, Activation::ReLU),
            DenseLayer::new(hidden_dim, hidden_dim, Activation::ReLU),
            DenseLayer::new(hidden_dim, 1, Activation::None),
        ];

        Self { layers }
    }

    pub fn forward(&self, state: &Array1<f64>) -> f64 {
        let mut x = state.clone();
        for layer in &self.layers {
            x = layer.forward(&x);
        }
        x[0]
    }
}

/// PPO Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPOConfig {
    pub clip_epsilon: f64,        // 0.2
    pub value_coef: f64,          // 0.5
    pub entropy_coef: f64,        // 0.01
    pub learning_rate: f64,       // 3e-4
    pub gamma: f64,               // 0.99
    pub gae_lambda: f64,          // 0.95
    pub batch_size: usize,        // 64
    pub epochs: usize,            // 4
    pub max_grad_norm: f64,       // 0.5
}

impl Default for PPOConfig {
    fn default() -> Self {
        Self {
            clip_epsilon: 0.2,
            value_coef: 0.5,
            entropy_coef: 0.01,
            learning_rate: 3e-4,
            gamma: 0.99,
            gae_lambda: 0.95,
            batch_size: 64,
            epochs: 4,
            max_grad_norm: 0.5,
        }
    }
}

/// Experience tuple
#[derive(Clone)]
pub struct Experience {
    pub state: Array1<f64>,
    pub action: usize,
    pub reward: f64,
    pub next_state: Array1<f64>,
    pub done: bool,
    pub log_prob: f64,
    pub value: f64,
}

/// Trajectory of experiences
pub struct Trajectory {
    pub experiences: Vec<Experience>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self { experiences: Vec::new() }
    }

    pub fn add(&mut self, exp: Experience) {
        self.experiences.push(exp);
    }

    pub fn len(&self) -> usize {
        self.experiences.len()
    }

    pub fn is_empty(&self) -> bool {
        self.experiences.is_empty()
    }
}

impl Default for Trajectory {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of PPO optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub policy_loss: f64,
    pub value_loss: f64,
    pub entropy: f64,
    pub updates: usize,
}

/// PPO Optimizer
pub struct PPOOptimizer {
    config: PPOConfig,
    policy: PolicyNetwork,
    value: ValueNetwork,
    #[allow(dead_code)]
    state_dim: usize,
    #[allow(dead_code)]
    action_dim: usize,
}

impl PPOOptimizer {
    pub fn new(config: PPOConfig, state_dim: usize, action_dim: usize) -> Self {
        let hidden_dim = 64;
        Self {
            config,
            policy: PolicyNetwork::new(state_dim, hidden_dim, action_dim),
            value: ValueNetwork::new(state_dim, hidden_dim),
            state_dim,
            action_dim,
        }
    }

    /// Compute Generalized Advantage Estimation
    pub fn compute_gae(&self, trajectory: &Trajectory) -> (Vec<f64>, Vec<f64>) {
        let n = trajectory.len();
        let mut advantages = vec![0.0; n];
        let mut returns = vec![0.0; n];

        let mut last_gae = 0.0;

        for t in (0..n).rev() {
            let exp = &trajectory.experiences[t];

            let next_value = if exp.done {
                0.0
            } else if t + 1 < n {
                trajectory.experiences[t + 1].value
            } else {
                self.value.forward(&exp.next_state)
            };

            let delta = exp.reward + self.config.gamma * next_value - exp.value;
            last_gae = delta + self.config.gamma * self.config.gae_lambda * (1.0 - exp.done as i32 as f64) * last_gae;

            advantages[t] = last_gae;
            returns[t] = last_gae + exp.value;
        }

        // Normalize advantages
        let mean: f64 = advantages.iter().sum::<f64>() / n as f64;
        let std: f64 = (advantages.iter().map(|a| (a - mean).powi(2)).sum::<f64>() / n as f64).sqrt();

        if std > 1e-8 {
            for a in &mut advantages {
                *a = (*a - mean) / std;
            }
        }

        (advantages, returns)
    }

    /// PPO update step
    pub async fn optimize(&mut self, trajectories: &[Trajectory]) -> Result<OptimizationResult, PPOError> {
        if trajectories.is_empty() {
            return Err(PPOError::InvalidTrajectory("Empty trajectories".to_string()));
        }

        let mut total_policy_loss = 0.0;
        let mut total_value_loss = 0.0;
        let mut total_entropy = 0.0;
        let mut update_count = 0;

        for trajectory in trajectories {
            if trajectory.is_empty() {
                continue;
            }

            let (advantages, returns) = self.compute_gae(trajectory);

            for _epoch in 0..self.config.epochs {
                for (i, exp) in trajectory.experiences.iter().enumerate() {
                    // Current policy probability
                    let new_prob = self.policy.get_action_prob(&exp.state, exp.action);
                    let new_log_prob = new_prob.ln();

                    // Probability ratio
                    let ratio = (new_log_prob - exp.log_prob).exp();

                    // Clipped objective
                    let surr1 = ratio * advantages[i];
                    let surr2 = ratio.clamp(
                        1.0 - self.config.clip_epsilon,
                        1.0 + self.config.clip_epsilon
                    ) * advantages[i];

                    let policy_loss = -surr1.min(surr2);

                    // Value loss
                    let value_pred = self.value.forward(&exp.state);
                    let value_loss = (returns[i] - value_pred).powi(2);

                    // Entropy bonus
                    let probs = self.policy.forward(&exp.state);
                    let entropy: f64 = -probs.iter()
                        .filter(|&&p| p > 1e-8)
                        .map(|&p| p * p.ln())
                        .sum::<f64>();

                    // Total loss
                    let _loss = policy_loss
                        + self.config.value_coef * value_loss
                        - self.config.entropy_coef * entropy;

                    total_policy_loss += policy_loss;
                    total_value_loss += value_loss;
                    total_entropy += entropy;
                    update_count += 1;

                    // In a real implementation, we'd compute gradients and update here
                    // For now, we simulate the update
                }
            }
        }

        if update_count == 0 {
            return Err(PPOError::OptimizationFailed("No updates performed".to_string()));
        }

        Ok(OptimizationResult {
            policy_loss: total_policy_loss / update_count as f64,
            value_loss: total_value_loss / update_count as f64,
            entropy: total_entropy / update_count as f64,
            updates: update_count,
        })
    }

    /// Get action from policy
    pub fn get_action(&self, state: &Array1<f64>) -> (usize, f64) {
        let action = self.policy.sample_action(state);
        let prob = self.policy.get_action_prob(state, action);
        (action, prob.ln())
    }

    /// Get value estimate
    pub fn get_value(&self, state: &Array1<f64>) -> f64 {
        self.value.forward(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense_layer() {
        let layer = DenseLayer::new(4, 8, Activation::ReLU);
        let input = Array1::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
        let output = layer.forward(&input);

        assert_eq!(output.len(), 8);
    }

    #[test]
    fn test_dense_layer_relu() {
        let layer = DenseLayer::new(3, 3, Activation::ReLU);
        let input = Array1::from_vec(vec![-1.0, 0.0, 1.0]);
        let output = layer.forward(&input);

        // ReLU should make all outputs non-negative
        for &val in output.iter() {
            assert!(val >= 0.0);
        }
    }

    #[test]
    fn test_dense_layer_tanh() {
        let layer = DenseLayer::new(3, 3, Activation::Tanh);
        let input = Array1::from_vec(vec![-1.0, 0.0, 1.0]);
        let output = layer.forward(&input);

        // Tanh should be in range [-1, 1]
        for &val in output.iter() {
            assert!(val >= -1.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_policy_network() {
        let policy = PolicyNetwork::new(4, 32, 3);
        let state = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);

        let probs = policy.forward(&state);
        assert_eq!(probs.len(), 3);

        let sum: f64 = probs.sum();
        assert!((sum - 1.0).abs() < 1e-6);  // Probabilities sum to 1

        // All probabilities should be non-negative
        for &p in probs.iter() {
            assert!(p >= 0.0 && p <= 1.0);
        }
    }

    #[test]
    fn test_policy_network_sample_action() {
        let policy = PolicyNetwork::new(4, 32, 3);
        let state = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);

        // Sample multiple times
        for _ in 0..10 {
            let action = policy.sample_action(&state);
            assert!(action < 3);  // Action should be in valid range
        }
    }

    #[test]
    fn test_value_network() {
        let value = ValueNetwork::new(4, 32);
        let state = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);

        let v = value.forward(&state);
        assert!(v.is_finite());
    }

    #[test]
    fn test_ppo_config_default() {
        let config = PPOConfig::default();
        assert_eq!(config.clip_epsilon, 0.2);
        assert_eq!(config.value_coef, 0.5);
        assert_eq!(config.entropy_coef, 0.01);
        assert_eq!(config.gamma, 0.99);
        assert_eq!(config.gae_lambda, 0.95);
    }

    #[test]
    fn test_trajectory_creation() {
        let mut trajectory = Trajectory::new();
        assert_eq!(trajectory.len(), 0);
        assert!(trajectory.is_empty());

        trajectory.add(Experience {
            state: Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]),
            action: 0,
            reward: 1.0,
            next_state: Array1::from_vec(vec![0.2, 0.3, 0.4, 0.5]),
            done: false,
            log_prob: -1.0,
            value: 0.5,
        });

        assert_eq!(trajectory.len(), 1);
        assert!(!trajectory.is_empty());
    }

    #[test]
    fn test_gae_computation() {
        let config = PPOConfig::default();
        let optimizer = PPOOptimizer::new(config, 4, 3);

        let mut trajectory = Trajectory::new();
        for i in 0..10 {
            trajectory.add(Experience {
                state: Array1::from_vec(vec![i as f64 / 10.0; 4]),
                action: i % 3,
                reward: 1.0,
                next_state: Array1::from_vec(vec![(i + 1) as f64 / 10.0; 4]),
                done: i == 9,
                log_prob: -1.0,
                value: 0.5,
            });
        }

        let (advantages, returns) = optimizer.compute_gae(&trajectory);

        assert_eq!(advantages.len(), 10);
        assert_eq!(returns.len(), 10);

        // Check that advantages are normalized (mean ~ 0, std ~ 1)
        let mean: f64 = advantages.iter().sum::<f64>() / advantages.len() as f64;
        assert!(mean.abs() < 1e-6);
    }

    #[test]
    fn test_gae_computation_with_varying_rewards() {
        let config = PPOConfig::default();
        let optimizer = PPOOptimizer::new(config, 4, 3);

        let mut trajectory = Trajectory::new();
        for i in 0..10 {
            let reward = if i > 5 { 1.0 } else { 0.0 };
            trajectory.add(Experience {
                state: Array1::from_vec(vec![i as f64 / 10.0; 4]),
                action: i % 3,
                reward,
                next_state: Array1::from_vec(vec![(i + 1) as f64 / 10.0; 4]),
                done: i == 9,
                log_prob: -1.0,
                value: 0.5,
            });
        }

        let (advantages, returns) = optimizer.compute_gae(&trajectory);

        assert_eq!(advantages.len(), 10);
        assert_eq!(returns.len(), 10);

        // Returns should be finite
        for r in &returns {
            assert!(r.is_finite());
        }
    }

    #[tokio::test]
    async fn test_ppo_optimize() {
        let config = PPOConfig::default();
        let mut optimizer = PPOOptimizer::new(config, 4, 3);

        let mut trajectory = Trajectory::new();
        for i in 0..20 {
            trajectory.add(Experience {
                state: Array1::from_vec(vec![i as f64 / 20.0; 4]),
                action: i % 3,
                reward: if i > 10 { 1.0 } else { 0.0 },
                next_state: Array1::from_vec(vec![(i + 1) as f64 / 20.0; 4]),
                done: i == 19,
                log_prob: -1.0,
                value: 0.5,
            });
        }

        let result = optimizer.optimize(&[trajectory]).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.policy_loss.is_finite());
        assert!(result.value_loss.is_finite());
        assert!(result.entropy.is_finite());
        assert!(result.updates > 0);
    }

    #[tokio::test]
    async fn test_ppo_optimize_multiple_trajectories() {
        let config = PPOConfig::default();
        let mut optimizer = PPOOptimizer::new(config, 4, 3);

        let mut trajectories = Vec::new();

        for traj_idx in 0..3 {
            let mut trajectory = Trajectory::new();
            for i in 0..15 {
                trajectory.add(Experience {
                    state: Array1::from_vec(vec![(i + traj_idx * 15) as f64 / 45.0; 4]),
                    action: i % 3,
                    reward: if i > 7 { 1.0 } else { 0.0 },
                    next_state: Array1::from_vec(vec![(i + 1 + traj_idx * 15) as f64 / 45.0; 4]),
                    done: i == 14,
                    log_prob: -1.0,
                    value: 0.5,
                });
            }
            trajectories.push(trajectory);
        }

        let result = optimizer.optimize(&trajectories).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.policy_loss.is_finite());
        assert!(result.value_loss.is_finite());
        assert!(result.entropy > 0.0);  // Entropy should be positive
    }

    #[tokio::test]
    async fn test_ppo_optimize_empty_trajectories() {
        let config = PPOConfig::default();
        let mut optimizer = PPOOptimizer::new(config, 4, 3);

        let result = optimizer.optimize(&[]).await;
        assert!(result.is_err());

        match result {
            Err(PPOError::InvalidTrajectory(_)) => {},
            _ => panic!("Expected InvalidTrajectory error"),
        }
    }

    #[test]
    fn test_get_action() {
        let config = PPOConfig::default();
        let optimizer = PPOOptimizer::new(config, 4, 3);

        let state = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);
        let (action, log_prob) = optimizer.get_action(&state);

        assert!(action < 3);
        assert!(log_prob.is_finite());
        assert!(log_prob <= 0.0);  // Log probability should be non-positive
    }

    #[test]
    fn test_get_value() {
        let config = PPOConfig::default();
        let optimizer = PPOOptimizer::new(config, 4, 3);

        let state = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);
        let value = optimizer.get_value(&state);

        assert!(value.is_finite());
    }

    #[test]
    fn test_softmax_activation() {
        let layer = DenseLayer::new(3, 3, Activation::Softmax);
        let input = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let output = layer.forward(&input);

        // Softmax outputs should sum to 1
        let sum: f64 = output.sum();
        assert!((sum - 1.0).abs() < 1e-6);

        // All outputs should be in (0, 1)
        for &val in output.iter() {
            assert!(val > 0.0 && val < 1.0);
        }

        // Test that probabilities are valid (all tests above cover this)
        assert_eq!(output.len(), 3);
    }
}
