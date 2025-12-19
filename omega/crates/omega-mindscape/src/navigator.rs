//! Mindscape Navigator
//!
//! Navigate through mindscape using place cell-like representations.
//! Allows walking, teleporting, and pathfinding through memory space.

use crate::coordinates::{MindscapeCoordinate, Position3D};
use crate::MindscapeError;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub use crate::coordinates::Position3D as Pos3D;

/// Result of a movement action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementResult {
    /// New position after movement
    pub new_position: Position3D,
    /// Distance traveled
    pub distance: f64,
    /// Nearby landmarks discovered
    pub landmarks_near: Vec<String>,
    /// Place cell activations
    pub place_activations: Vec<f64>,
    /// Head direction (radians)
    pub heading: f64,
}

/// A path through mindscape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationPath {
    /// Waypoints along the path
    pub waypoints: Vec<Position3D>,
    /// Total distance of path
    pub total_distance: f64,
    /// Estimated landmarks along path
    pub landmarks_passed: Vec<String>,
    /// Path complexity (turns, elevation changes)
    pub complexity: f64,
}

impl NavigationPath {
    pub fn new(waypoints: Vec<Position3D>) -> Self {
        let total_distance = Self::compute_distance(&waypoints);
        let complexity = Self::compute_complexity(&waypoints);

        Self {
            waypoints,
            total_distance,
            landmarks_passed: Vec::new(),
            complexity,
        }
    }

    fn compute_distance(waypoints: &[Position3D]) -> f64 {
        if waypoints.len() < 2 {
            return 0.0;
        }

        waypoints.windows(2)
            .map(|w| w[0].distance_to(&w[1]))
            .sum()
    }

    fn compute_complexity(waypoints: &[Position3D]) -> f64 {
        if waypoints.len() < 3 {
            return 0.0;
        }

        // Count direction changes
        let mut total_angle_change = 0.0;

        for i in 1..waypoints.len() - 1 {
            let v1 = waypoints[i - 1].direction_to(&waypoints[i]);
            let v2 = waypoints[i].direction_to(&waypoints[i + 1]);

            // Dot product gives cosine of angle
            let dot = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
            let angle = dot.clamp(-1.0, 1.0).acos();
            total_angle_change += angle;
        }

        total_angle_change
    }
}

/// Virtual place cell for mindscape navigation
#[derive(Debug, Clone)]
struct VirtualPlaceCell {
    _id: usize,  // Reserved for future place cell identification
    center: Position3D,
    radius: f64,
    activation: f64,
}

impl VirtualPlaceCell {
    fn new(id: usize, center: Position3D, radius: f64) -> Self {
        Self {
            _id: id,
            center,
            radius,
            activation: 0.0,
        }
    }

    fn compute_activation(&mut self, position: &Position3D) -> f64 {
        let dist_sq = (position.x - self.center.x).powi(2)
            + (position.y - self.center.y).powi(2)
            + (position.z - self.center.z).powi(2);
        let sigma_sq = self.radius * self.radius;

        self.activation = (-dist_sq / (2.0 * sigma_sq)).exp();
        self.activation
    }
}

/// Navigator for moving through mindscape
pub struct MindscapeNavigator {
    /// World size
    world_size: f64,
    /// Current position
    position: Position3D,
    /// Current heading (radians in XY plane)
    heading: f64,
    /// Movement speed
    speed: f64,
    /// Virtual place cells
    place_cells: Vec<VirtualPlaceCell>,
    /// Movement history (trajectory)
    trajectory: Vec<Position3D>,
    /// Max trajectory length
    max_trajectory: usize,
    /// Known landmark positions
    landmark_positions: Vec<Position3D>,
    /// Path integration estimate (accumulates drift)
    path_integration: Position3D,
}

impl MindscapeNavigator {
    pub fn new(world_size: f64, num_place_cells: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Distribute place cells throughout the world
        let place_cells: Vec<VirtualPlaceCell> = (0..num_place_cells)
            .map(|id| {
                let x = rng.gen_range(0.0..world_size);
                let y = rng.gen_range(0.0..world_size);
                let z = rng.gen_range(0.0..world_size);
                let radius = rng.gen_range(world_size / 20.0..world_size / 10.0);

                VirtualPlaceCell::new(id, Position3D::new(x, y, z), radius)
            })
            .collect();

        let center = Position3D::new(world_size / 2.0, world_size / 2.0, world_size / 2.0);

        Self {
            world_size,
            position: center,
            heading: 0.0,
            speed: 10.0, // Units per step
            place_cells,
            trajectory: Vec::with_capacity(1000),
            max_trajectory: 1000,
            landmark_positions: Vec::new(),
            path_integration: center,
        }
    }

    /// Get current position
    pub fn current_position(&self) -> Position3D {
        self.position
    }

    /// Get current heading
    pub fn heading(&self) -> f64 {
        self.heading
    }

    /// Move in current direction
    pub fn move_forward(&mut self, distance: f64) -> MovementResult {
        let direction = Position3D::new(
            self.heading.cos(),
            self.heading.sin(),
            0.0, // Horizontal movement
        );

        self.move_in_direction(&direction, distance)
    }

    /// Move toward a specific position
    pub fn move_toward(&mut self, target: &Position3D, distance: f64) -> MovementResult {
        let direction = self.position.direction_to(target);
        self.move_in_direction(&direction, distance)
    }

    /// Move in a specific direction
    fn move_in_direction(&mut self, direction: &Position3D, distance: f64) -> MovementResult {
        // Store old position
        self.trajectory.push(self.position);
        if self.trajectory.len() > self.max_trajectory {
            self.trajectory.remove(0);
        }

        // Calculate new position
        let new_pos = self.position.move_by(direction, distance);

        // Clamp to world bounds
        self.position = Position3D::new(
            new_pos.x.clamp(0.0, self.world_size),
            new_pos.y.clamp(0.0, self.world_size),
            new_pos.z.clamp(0.0, self.world_size),
        );

        // Update path integration (with drift)
        self.path_integration.x += direction.x * distance * 0.98;
        self.path_integration.y += direction.y * distance * 0.98;
        self.path_integration.z += direction.z * distance * 0.98;

        // Update heading
        if direction.x.abs() > 0.01 || direction.y.abs() > 0.01 {
            self.heading = direction.y.atan2(direction.x);
        }

        // Update place cell activations
        let place_activations = self.update_place_cells();

        // Check for nearby landmarks
        let landmarks_near = self.find_nearby_landmarks(distance * 2.0);

        MovementResult {
            new_position: self.position,
            distance,
            landmarks_near,
            place_activations,
            heading: self.heading,
        }
    }

    /// Turn by angle (radians)
    pub fn turn(&mut self, angle: f64) {
        self.heading += angle;
        // Normalize to [0, 2π)
        while self.heading < 0.0 {
            self.heading += 2.0 * std::f64::consts::PI;
        }
        while self.heading >= 2.0 * std::f64::consts::PI {
            self.heading -= 2.0 * std::f64::consts::PI;
        }
    }

    /// Teleport to position (instant movement)
    pub fn teleport(&mut self, target: &Position3D) {
        self.trajectory.push(self.position);
        if self.trajectory.len() > self.max_trajectory {
            self.trajectory.remove(0);
        }

        self.position = Position3D::new(
            target.x.clamp(0.0, self.world_size),
            target.y.clamp(0.0, self.world_size),
            target.z.clamp(0.0, self.world_size),
        );

        // Reset path integration at teleport (like a landmark correction)
        self.path_integration = self.position;

        self.update_place_cells();
    }

    /// Navigate to target position, returning the path
    pub fn navigate_to(&mut self, target: &Position3D) -> Result<NavigationPath, MindscapeError> {
        // Simple direct path (could be enhanced with obstacle avoidance)
        let start = self.position;
        let target_clamped = Position3D::new(
            target.x.clamp(0.0, self.world_size),
            target.y.clamp(0.0, self.world_size),
            target.z.clamp(0.0, self.world_size),
        );

        // Generate waypoints
        let distance = start.distance_to(&target_clamped);
        let num_waypoints = (distance / self.speed).ceil() as usize;
        let num_waypoints = num_waypoints.max(2);

        let mut waypoints = Vec::with_capacity(num_waypoints);
        for i in 0..num_waypoints {
            let t = i as f64 / (num_waypoints - 1) as f64;
            waypoints.push(start.lerp(&target_clamped, t));
        }

        // Execute movement along path
        for waypoint in &waypoints[1..] {
            let dist = self.position.distance_to(waypoint);
            self.move_toward(waypoint, dist);
        }

        Ok(NavigationPath::new(waypoints))
    }

    /// Add a landmark position
    pub fn add_landmark(&mut self, coordinate: &MindscapeCoordinate) {
        self.landmark_positions.push(coordinate.position);
    }

    /// Update place cell activations
    fn update_place_cells(&mut self) -> Vec<f64> {
        self.place_cells
            .iter_mut()
            .map(|cell| cell.compute_activation(&self.position))
            .collect()
    }

    /// Find landmarks within distance
    fn find_nearby_landmarks(&self, distance: f64) -> Vec<String> {
        self.landmark_positions
            .iter()
            .enumerate()
            .filter(|(_, lm)| self.position.distance_to(lm) <= distance)
            .map(|(i, _)| format!("landmark_{}", i))
            .collect()
    }

    /// Get place cell pattern (for position decoding)
    pub fn place_cell_pattern(&self) -> Vec<f64> {
        self.place_cells.iter().map(|c| c.activation).collect()
    }

    /// Decode position from place cell pattern
    pub fn decode_position(&self) -> Position3D {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut total_weight = 0.0;

        for cell in &self.place_cells {
            if cell.activation > 0.01 {
                x += cell.center.x * cell.activation;
                y += cell.center.y * cell.activation;
                z += cell.center.z * cell.activation;
                total_weight += cell.activation;
            }
        }

        if total_weight > 0.0 {
            Position3D::new(x / total_weight, y / total_weight, z / total_weight)
        } else {
            self.position
        }
    }

    /// Get trajectory
    pub fn trajectory(&self) -> &[Position3D] {
        &self.trajectory
    }

    /// Get path integration position
    pub fn path_integration_position(&self) -> Position3D {
        self.path_integration
    }

    /// Reset navigator
    pub fn reset(&mut self) {
        let center = Position3D::new(
            self.world_size / 2.0,
            self.world_size / 2.0,
            self.world_size / 2.0,
        );
        self.position = center;
        self.path_integration = center;
        self.heading = 0.0;
        self.trajectory.clear();
        self.landmark_positions.clear();

        // Re-randomize place cells
        let mut rng = rand::thread_rng();
        for cell in &mut self.place_cells {
            cell.center = Position3D::new(
                rng.gen_range(0.0..self.world_size),
                rng.gen_range(0.0..self.world_size),
                rng.gen_range(0.0..self.world_size),
            );
            cell.activation = 0.0;
        }
    }

    /// Get world bounds
    pub fn world_size(&self) -> f64 {
        self.world_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigator_creation() {
        let nav = MindscapeNavigator::new(1000.0, 100);

        let pos = nav.current_position();
        assert!((pos.x - 500.0).abs() < 0.01);
    }

    #[test]
    fn test_movement() {
        let mut nav = MindscapeNavigator::new(1000.0, 50);

        let initial = nav.current_position();
        nav.move_forward(100.0);
        let final_pos = nav.current_position();

        assert!(initial.distance_to(&final_pos) > 0.0);
    }

    #[test]
    fn test_navigation() {
        let mut nav = MindscapeNavigator::new(1000.0, 50);

        let target = Position3D::new(700.0, 700.0, 500.0);
        let path = nav.navigate_to(&target).unwrap();

        // Should end near target
        let final_pos = nav.current_position();
        assert!(final_pos.distance_to(&target) < 1.0);
        assert!(path.total_distance > 0.0);
    }

    #[test]
    fn test_teleport() {
        let mut nav = MindscapeNavigator::new(1000.0, 50);

        let target = Position3D::new(100.0, 100.0, 100.0);
        nav.teleport(&target);

        let pos = nav.current_position();
        assert!((pos.x - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_place_cells() {
        let mut nav = MindscapeNavigator::new(1000.0, 100);

        nav.move_forward(50.0);
        let pattern = nav.place_cell_pattern();

        assert_eq!(pattern.len(), 100);
        // At least some cells should be active
        assert!(pattern.iter().any(|&a| a > 0.01));
    }
}
