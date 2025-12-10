//! Place Cells
//!
//! Spatial representation in hippocampus:
//! - Location-specific firing
//! - Place fields
//! - Cognitive maps
//! - Path integration

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A place cell that fires for specific locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceCell {
    /// Unique identifier
    pub id: usize,
    /// Center of place field (x, y)
    pub center: (f64, f64),
    /// Radius of place field
    pub radius: f64,
    /// Peak firing rate
    pub peak_rate: f64,
    /// Current activation
    pub activation: f64,
    /// Whether currently in field
    pub in_field: bool,
}

impl PlaceCell {
    pub fn new(id: usize, center: (f64, f64), radius: f64) -> Self {
        Self {
            id,
            center,
            radius,
            peak_rate: 1.0,
            activation: 0.0,
            in_field: false,
        }
    }

    /// Compute activation for position (x, y)
    pub fn compute(&mut self, x: f64, y: f64) -> f64 {
        // Gaussian place field
        let dx = x - self.center.0;
        let dy = y - self.center.1;
        let dist_sq = dx * dx + dy * dy;
        let sigma_sq = self.radius * self.radius;

        self.activation = self.peak_rate * (-dist_sq / (2.0 * sigma_sq)).exp();
        self.in_field = self.activation > 0.1;

        self.activation
    }

    /// Check if position is within place field
    pub fn is_in_field(&self, x: f64, y: f64) -> bool {
        let dx = x - self.center.0;
        let dy = y - self.center.1;
        (dx * dx + dy * dy).sqrt() < self.radius * 2.0
    }

    /// Update center (place field remapping)
    pub fn remap(&mut self, new_center: (f64, f64)) {
        self.center = new_center;
    }
}

/// A place field representing a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceField {
    /// Center position
    pub center: (f64, f64),
    /// Field radius
    pub radius: f64,
    /// Associated place cell IDs
    pub cell_ids: Vec<usize>,
    /// Average activation when in field
    pub avg_activation: f64,
    /// Visit count
    pub visits: u32,
}

impl PlaceField {
    pub fn new(center: (f64, f64), radius: f64) -> Self {
        Self {
            center,
            radius,
            cell_ids: Vec::new(),
            avg_activation: 0.0,
            visits: 0,
        }
    }

    /// Update field statistics
    pub fn update(&mut self, activation: f64) {
        self.visits += 1;
        self.avg_activation = (self.avg_activation * (self.visits - 1) as f64 + activation)
            / self.visits as f64;
    }
}

/// Spatial map using place cells
pub struct SpatialMap {
    /// Environment width
    width: f64,
    /// Environment height
    height: f64,
    /// Place cells
    cells: Vec<PlaceCell>,
    /// Current position
    current_pos: (f64, f64),
    /// Path integration estimate
    path_integration: (f64, f64),
    /// Movement history
    trajectory: Vec<(f64, f64)>,
    /// Max trajectory length
    max_trajectory: usize,
    /// Head direction
    head_direction: f64,
}

impl SpatialMap {
    /// Create new spatial map
    pub fn new(width: f64, height: f64, num_cells: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Distribute place cells across environment
        let cells: Vec<PlaceCell> = (0..num_cells)
            .map(|id| {
                let x = rng.gen_range(0.0..width);
                let y = rng.gen_range(0.0..height);
                let radius = rng.gen_range(width / 10.0..width / 5.0);
                PlaceCell::new(id, (x, y), radius)
            })
            .collect();

        Self {
            width,
            height,
            cells,
            current_pos: (width / 2.0, height / 2.0),
            path_integration: (width / 2.0, height / 2.0),
            trajectory: Vec::with_capacity(1000),
            max_trajectory: 1000,
            head_direction: 0.0,
        }
    }

    /// Update position and compute place cell activity
    pub fn update_position(&mut self, x: f64, y: f64) {
        // Store trajectory
        self.trajectory.push(self.current_pos);
        if self.trajectory.len() > self.max_trajectory {
            self.trajectory.remove(0);
        }

        // Update head direction
        let dx = x - self.current_pos.0;
        let dy = y - self.current_pos.1;
        if dx.abs() > 0.01 || dy.abs() > 0.01 {
            self.head_direction = dy.atan2(dx);
        }

        // Update current position
        self.current_pos = (x.max(0.0).min(self.width), y.max(0.0).min(self.height));

        // Update path integration (with drift)
        self.path_integration.0 += dx * 0.98; // Slight drift
        self.path_integration.1 += dy * 0.98;

        // Compute all place cell activations
        for cell in &mut self.cells {
            cell.compute(self.current_pos.0, self.current_pos.1);
        }
    }

    /// Move by velocity and update
    pub fn move_by(&mut self, vx: f64, vy: f64, dt: f64) {
        let new_x = self.current_pos.0 + vx * dt;
        let new_y = self.current_pos.1 + vy * dt;
        self.update_position(new_x, new_y);
    }

    /// Get current place cell activity pattern
    pub fn get_activity(&self) -> Vec<f64> {
        self.cells.iter().map(|c| c.activation).collect()
    }

    /// Get active place cells (above threshold)
    pub fn get_active_cells(&self, threshold: f64) -> Vec<usize> {
        self.cells
            .iter()
            .filter(|c| c.activation > threshold)
            .map(|c| c.id)
            .collect()
    }

    /// Decode position from place cell activity
    pub fn decode_position(&self) -> (f64, f64) {
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut weight_sum = 0.0;

        for cell in &self.cells {
            if cell.activation > 0.01 {
                x_sum += cell.activation * cell.center.0;
                y_sum += cell.activation * cell.center.1;
                weight_sum += cell.activation;
            }
        }

        if weight_sum > 0.0 {
            (x_sum / weight_sum, y_sum / weight_sum)
        } else {
            self.current_pos
        }
    }

    /// Get path integration position estimate
    pub fn get_path_integration(&self) -> (f64, f64) {
        self.path_integration
    }

    /// Reset path integration (e.g., at landmark)
    pub fn reset_path_integration(&mut self) {
        self.path_integration = self.current_pos;
    }

    /// Get current position
    pub fn current_position(&self) -> (f64, f64) {
        self.current_pos
    }

    /// Get head direction
    pub fn head_direction(&self) -> f64 {
        self.head_direction
    }

    /// Get trajectory
    pub fn trajectory(&self) -> &[(f64, f64)] {
        &self.trajectory
    }

    /// Compute occupancy map
    pub fn compute_occupancy(&self, resolution: usize) -> Vec<Vec<u32>> {
        let mut map = vec![vec![0u32; resolution]; resolution];

        for &(x, y) in &self.trajectory {
            let ix = ((x / self.width) * resolution as f64).min(resolution as f64 - 1.0) as usize;
            let iy = ((y / self.height) * resolution as f64).min(resolution as f64 - 1.0) as usize;
            map[iy][ix] += 1;
        }

        map
    }

    /// Compute place field map (firing rate by location)
    pub fn compute_rate_map(&self, cell_id: usize, resolution: usize) -> Vec<Vec<f64>> {
        let mut map = vec![vec![0.0; resolution]; resolution];

        if let Some(cell) = self.cells.get(cell_id) {
            let step_x = self.width / resolution as f64;
            let step_y = self.height / resolution as f64;

            for iy in 0..resolution {
                for ix in 0..resolution {
                    let x = ix as f64 * step_x + step_x / 2.0;
                    let y = iy as f64 * step_y + step_y / 2.0;

                    let dx = x - cell.center.0;
                    let dy = y - cell.center.1;
                    let dist_sq = dx * dx + dy * dy;
                    let sigma_sq = cell.radius * cell.radius;

                    map[iy][ix] = cell.peak_rate * (-dist_sq / (2.0 * sigma_sq)).exp();
                }
            }
        }

        map
    }

    /// Get environment dimensions
    pub fn dimensions(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    /// Get number of place cells
    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    /// Clear trajectory
    pub fn clear_trajectory(&mut self) {
        self.trajectory.clear();
    }
}

impl Default for SpatialMap {
    fn default() -> Self {
        Self::new(100.0, 100.0, 100)
    }
}

/// Head direction cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadDirectionCell {
    /// Preferred direction (radians)
    pub preferred_direction: f64,
    /// Tuning width
    pub tuning_width: f64,
    /// Current activation
    pub activation: f64,
}

impl HeadDirectionCell {
    pub fn new(preferred_direction: f64) -> Self {
        Self {
            preferred_direction,
            tuning_width: PI / 4.0,
            activation: 0.0,
        }
    }

    /// Compute activation for head direction
    pub fn compute(&mut self, direction: f64) -> f64 {
        let diff = (direction - self.preferred_direction).sin().powi(2)
            + (direction - self.preferred_direction).cos().powi(2)
            - 1.0;
        let angular_diff = (diff + 1.0).acos();

        self.activation = (-angular_diff.powi(2) / (2.0 * self.tuning_width.powi(2))).exp();
        self.activation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_cell() {
        let mut pc = PlaceCell::new(0, (50.0, 50.0), 10.0);

        // At center
        let a1 = pc.compute(50.0, 50.0);
        assert!((a1 - 1.0).abs() < 0.01);

        // Far away
        let a2 = pc.compute(100.0, 100.0);
        assert!(a2 < 0.1);
    }

    #[test]
    fn test_spatial_map() {
        let mut map = SpatialMap::new(100.0, 100.0, 50);

        map.update_position(25.0, 25.0);
        let activity = map.get_activity();

        assert_eq!(activity.len(), 50);
    }

    #[test]
    fn test_position_decoding() {
        let mut map = SpatialMap::new(100.0, 100.0, 100);

        map.update_position(30.0, 40.0);
        let decoded = map.decode_position();

        // Decoded should be close to actual
        let error = ((decoded.0 - 30.0).powi(2) + (decoded.1 - 40.0).powi(2)).sqrt();
        assert!(error < 30.0); // Within reasonable range
    }

    #[test]
    fn test_trajectory() {
        let mut map = SpatialMap::new(100.0, 100.0, 20);

        for i in 0..10 {
            map.update_position(i as f64 * 5.0, i as f64 * 5.0);
        }

        assert!(map.trajectory().len() >= 9);
    }

    #[test]
    fn test_head_direction_cell() {
        let mut hd = HeadDirectionCell::new(0.0);

        // Facing preferred direction
        let a1 = hd.compute(0.0);
        assert!(a1 > 0.9);

        // Facing opposite
        let a2 = hd.compute(PI);
        assert!(a2 < 0.5);
    }

    #[test]
    fn test_rate_map() {
        let map = SpatialMap::new(100.0, 100.0, 10);
        let rate_map = map.compute_rate_map(0, 10);

        assert_eq!(rate_map.len(), 10);
        assert_eq!(rate_map[0].len(), 10);
    }
}
