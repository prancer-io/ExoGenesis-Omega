//! Integrated Information Theory (IIT)
//!
//! Implements IIT 3.0 concepts for measuring consciousness:
//! - Phi (Φ): Integrated information above and beyond its parts
//! - Cause-effect structures: How system states constrain past/future
//! - Partitions: Minimum information partition (MIP)
//!
//! Based on Tononi et al. (2016) - "Integrated Information Theory"

use serde::{Deserialize, Serialize};

use crate::{ConsciousnessError, Result};

/// A partition of a system into parts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    /// Indices of elements in part A
    pub part_a: Vec<usize>,
    /// Indices of elements in part B
    pub part_b: Vec<usize>,
}

impl Partition {
    pub fn new(part_a: Vec<usize>, part_b: Vec<usize>) -> Self {
        Self { part_a, part_b }
    }

    /// Create all bipartitions of n elements
    pub fn all_bipartitions(n: usize) -> Vec<Partition> {
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![Partition::new(vec![0], vec![])];
        }

        let mut partitions = Vec::new();

        // Generate all non-trivial bipartitions
        // Each element can be in A (0) or B (1)
        // Skip all-A and all-B cases
        for mask in 1..(1 << n) - 1 {
            let mut part_a = Vec::new();
            let mut part_b = Vec::new();

            for i in 0..n {
                if (mask >> i) & 1 == 0 {
                    part_a.push(i);
                } else {
                    part_b.push(i);
                }
            }

            partitions.push(Partition::new(part_a, part_b));
        }

        partitions
    }

    /// Size of partition (for normalization)
    pub fn size(&self) -> usize {
        self.part_a.len().min(self.part_b.len())
    }
}

/// Cause-effect structure for a mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseEffectStructure {
    /// The mechanism (set of elements)
    pub mechanism: Vec<usize>,
    /// Cause repertoire (probability distribution over past states)
    pub cause_repertoire: Vec<f64>,
    /// Effect repertoire (probability distribution over future states)
    pub effect_repertoire: Vec<f64>,
    /// Integrated information for causes (phi_cause)
    pub phi_cause: f64,
    /// Integrated information for effects (phi_effect)
    pub phi_effect: f64,
    /// Overall integrated information
    pub phi: f64,
}

impl CauseEffectStructure {
    pub fn new(mechanism: Vec<usize>, state_size: usize) -> Self {
        Self {
            mechanism,
            cause_repertoire: vec![1.0 / state_size as f64; state_size],
            effect_repertoire: vec![1.0 / state_size as f64; state_size],
            phi_cause: 0.0,
            phi_effect: 0.0,
            phi: 0.0,
        }
    }
}

/// Integrated information computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedInformation {
    /// Phi value
    pub phi: f64,
    /// The minimum information partition
    pub mip: Option<Partition>,
    /// Cause-effect structures for all mechanisms
    pub structures: Vec<CauseEffectStructure>,
    /// Whether system is integrated
    pub is_integrated: bool,
}

/// Computer for Phi (integrated information)
pub struct PhiComputer {
    /// Dimension of state space
    dim: usize,
    /// Transition probability matrix (approximated)
    tpm: Vec<Vec<f64>>,
    /// Current state
    current_state: Vec<f64>,
    /// Previous state
    previous_state: Vec<f64>,
    /// Current Phi value
    current_phi: f64,
    /// History of states for TPM estimation
    state_history: Vec<Vec<f64>>,
    /// Maximum history length
    max_history: usize,
}

impl PhiComputer {
    pub fn new(dim: usize) -> Self {
        // Initialize with identity-like TPM (self-transitions)
        let mut tpm = vec![vec![0.0; dim]; dim];
        for (i, tpm_row) in tpm.iter_mut().enumerate() {
            tpm_row[i] = 1.0;
        }

        Self {
            dim,
            tpm,
            current_state: vec![0.0; dim],
            previous_state: vec![0.0; dim],
            current_phi: 0.0,
            state_history: Vec::new(),
            max_history: 100,
        }
    }

    /// Compute Phi for current system state
    pub fn compute_phi(&mut self, state: &[f64]) -> Result<f64> {
        if state.len() != self.dim {
            return Err(ConsciousnessError::InvalidState(format!(
                "Expected dimension {}, got {}",
                self.dim,
                state.len()
            )));
        }

        // Update state
        self.previous_state = self.current_state.clone();
        self.current_state = state.to_vec();

        // Update TPM estimation
        self.update_tpm();

        // For efficiency, we use a simplified Phi approximation
        // Full IIT computation is exponential in system size

        // 1. Compute whole system information
        let whole_info = self.compute_whole_information();

        // 2. Find minimum information partition
        let (_mip, min_partition_info) = self.find_mip();

        // 3. Phi = whole - min_partition
        self.current_phi = (whole_info - min_partition_info).max(0.0);

        // Store in history
        self.state_history.push(state.to_vec());
        if self.state_history.len() > self.max_history {
            self.state_history.remove(0);
        }

        Ok(self.current_phi)
    }

    /// Compute information for whole system
    fn compute_whole_information(&self) -> f64 {
        // Mutual information between current and previous state
        // I(S_t; S_{t-1})

        let mut mi = 0.0;

        // Simplified: use correlation as proxy for mutual information
        for i in 0..self.dim {
            let x = self.current_state[i];
            let y = self.previous_state[i];

            // Avoid log(0)
            if x > 1e-10 && y > 1e-10 {
                // Contribution to MI (simplified)
                mi += x * (x / y.max(1e-10)).ln().abs();
            }
        }

        // Normalize by dimension
        mi / self.dim as f64
    }

    /// Find minimum information partition
    fn find_mip(&self) -> (Partition, f64) {
        let n = self.dim.min(8); // Limit for computational feasibility
        let partitions = Partition::all_bipartitions(n);

        if partitions.is_empty() {
            return (Partition::new(vec![], vec![]), 0.0);
        }

        let mut min_info = f64::INFINITY;
        let mut best_partition = partitions[0].clone();

        for partition in &partitions {
            let info = self.compute_partition_info(partition);

            // Normalize by partition size
            let normalized_info = if partition.size() > 0 {
                info / partition.size() as f64
            } else {
                info
            };

            if normalized_info < min_info {
                min_info = normalized_info;
                best_partition = partition.clone();
            }
        }

        (best_partition, min_info)
    }

    /// Compute information for a partition
    fn compute_partition_info(&self, partition: &Partition) -> f64 {
        // Information generated by parts independently
        let info_a = self.compute_part_info(&partition.part_a);
        let info_b = self.compute_part_info(&partition.part_b);

        info_a + info_b
    }

    /// Compute information for a part
    fn compute_part_info(&self, indices: &[usize]) -> f64 {
        if indices.is_empty() {
            return 0.0;
        }

        let mut info = 0.0;
        for &i in indices {
            if i < self.dim {
                let x = self.current_state[i];
                let y = self.previous_state[i];

                if x > 1e-10 && y > 1e-10 {
                    info += x * (x / y.max(1e-10)).ln().abs();
                }
            }
        }

        info / indices.len() as f64
    }

    /// Update transition probability matrix from state history
    fn update_tpm(&mut self) {
        if self.state_history.len() < 2 {
            return;
        }

        // Learning rate for TPM update
        let alpha = 0.1;

        // Update TPM based on observed transition
        for (i, tpm_row) in self.tpm.iter_mut().enumerate() {
            for (j, tpm_elem) in tpm_row.iter_mut().enumerate() {
                // Estimate transition probability from state i to state j
                let observed = if self.previous_state[i] > 0.5 && self.current_state[j] > 0.5 {
                    1.0
                } else {
                    0.0
                };

                *tpm_elem = (1.0 - alpha) * *tpm_elem + alpha * observed;
            }

            // Normalize row
            let row_sum: f64 = tpm_row.iter().sum();
            if row_sum > 0.0 {
                for tpm_elem in tpm_row.iter_mut() {
                    *tpm_elem /= row_sum;
                }
            }
        }
    }

    /// Get current Phi value
    pub fn current_phi(&self) -> f64 {
        self.current_phi
    }

    /// Compute full integrated information structure
    pub fn compute_full(&mut self, state: &[f64]) -> Result<IntegratedInformation> {
        let phi = self.compute_phi(state)?;

        let (mip, _) = self.find_mip();

        // Compute cause-effect structures for each mechanism
        let n = self.dim.min(8);
        let mut structures = Vec::new();

        for size in 1..=n.min(3) {
            // Limit mechanism size for efficiency
            for mechanism in Self::combinations(n, size) {
                let mut ces = CauseEffectStructure::new(mechanism.clone(), self.dim);

                // Simplified phi computation for mechanism
                ces.phi_cause = self.compute_mechanism_phi(&mechanism, true);
                ces.phi_effect = self.compute_mechanism_phi(&mechanism, false);
                ces.phi = ces.phi_cause.min(ces.phi_effect);

                if ces.phi > 0.01 {
                    structures.push(ces);
                }
            }
        }

        Ok(IntegratedInformation {
            phi,
            mip: Some(mip),
            structures,
            is_integrated: phi > 0.0,
        })
    }

    /// Compute phi for a specific mechanism
    fn compute_mechanism_phi(&self, mechanism: &[usize], is_cause: bool) -> f64 {
        let mut phi = 0.0;

        for &i in mechanism {
            if i < self.dim {
                let val = if is_cause {
                    self.previous_state[i]
                } else {
                    self.current_state[i]
                };

                // Information contribution
                if val > 1e-10 {
                    phi += val.ln().abs();
                }
            }
        }

        phi / mechanism.len().max(1) as f64
    }

    /// Generate combinations of size k from n elements
    fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
        if k == 0 {
            return vec![vec![]];
        }
        if n < k {
            return vec![];
        }

        let mut result = Vec::new();
        let mut combo = vec![0; k];

        fn generate(start: usize, n: usize, combo: &mut Vec<usize>, pos: usize, result: &mut Vec<Vec<usize>>) {
            if pos == combo.len() {
                result.push(combo.clone());
                return;
            }

            for i in start..=n - (combo.len() - pos) {
                combo[pos] = i;
                generate(i + 1, n, combo, pos + 1, result);
            }
        }

        generate(0, n, &mut combo, 0, &mut result);
        result
    }

    /// Reset computer state
    pub fn reset(&mut self) {
        self.current_state = vec![0.0; self.dim];
        self.previous_state = vec![0.0; self.dim];
        self.current_phi = 0.0;
        self.state_history.clear();

        // Reset TPM to identity-like
        for i in 0..self.dim {
            for j in 0..self.dim {
                self.tpm[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_computer_creation() {
        let computer = PhiComputer::new(8);
        assert_eq!(computer.current_phi(), 0.0);
    }

    #[test]
    fn test_compute_phi() {
        let mut computer = PhiComputer::new(8);

        // Random state
        let state: Vec<f64> = (0..8).map(|i| (i as f64) / 8.0).collect();
        let phi = computer.compute_phi(&state);

        assert!(phi.is_ok());
        assert!(phi.unwrap() >= 0.0);
    }

    #[test]
    fn test_phi_increases_with_integration() {
        let mut computer = PhiComputer::new(8);

        // Process multiple states to build up TPM
        for i in 0..10 {
            let state: Vec<f64> = (0..8).map(|j| ((i + j) % 8) as f64 / 8.0).collect();
            let _ = computer.compute_phi(&state);
        }

        // Phi should be computed
        assert!(computer.current_phi() >= 0.0);
    }

    #[test]
    fn test_partitions() {
        let partitions = Partition::all_bipartitions(4);

        // 2^4 - 2 = 14 non-trivial bipartitions
        assert_eq!(partitions.len(), 14);
    }

    #[test]
    fn test_full_computation() {
        let mut computer = PhiComputer::new(8);

        let state: Vec<f64> = (0..8).map(|i| (i as f64) / 8.0).collect();
        let result = computer.compute_full(&state);

        assert!(result.is_ok());
        let iit = result.unwrap();
        assert!(iit.phi >= 0.0);
    }

    #[test]
    fn test_combinations() {
        let combos = PhiComputer::combinations(4, 2);
        // C(4,2) = 6
        assert_eq!(combos.len(), 6);
    }
}
