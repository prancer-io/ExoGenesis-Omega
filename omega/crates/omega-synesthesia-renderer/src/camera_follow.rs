//! Camera Auto-Follow System for Musical Timeline
//!
//! Automatically positions the camera to follow the musical progression through
//! the 3D world, creating a cinematic experience for real-time visualization.

use glam::Vec3;
use std::f32::consts::PI;

/// Camera follow mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowMode {
    /// Fixed orbit around the current musical moment
    Orbit,

    /// Smooth tracking along the timeline
    Tracking,

    /// Cinematic flythrough
    Cinematic,

    /// First-person walk through the music
    FirstPerson,
}

/// Camera auto-follow controller
pub struct CameraFollowController {
    /// Current follow mode
    mode: FollowMode,

    /// Current timeline position (seconds)
    timeline_position: f32,

    /// Camera offset from timeline position
    offset: Vec3,

    /// Orbit angle (radians)
    orbit_angle: f32,

    /// Orbit radius
    orbit_radius: f32,

    /// Orbit height
    orbit_height: f32,

    /// Smoothing factor (0-1)
    smoothing: f32,

    /// Current camera position (smoothed)
    current_position: Vec3,

    /// Current camera target (smoothed)
    current_target: Vec3,

    /// Auto-rotation speed (radians/second)
    auto_rotate_speed: f32,
}

impl CameraFollowController {
    /// Create a new camera follow controller
    pub fn new(mode: FollowMode) -> Self {
        Self {
            mode,
            timeline_position: 0.0,
            offset: Vec3::new(0.0, 5.0, 10.0),
            orbit_angle: 0.0,
            orbit_radius: 15.0,
            orbit_height: 8.0,
            smoothing: 0.1,
            current_position: Vec3::new(0.0, 8.0, 15.0),
            current_target: Vec3::ZERO,
            auto_rotate_speed: 0.2,  // ~11 degrees per second
        }
    }

    /// Update timeline position
    pub fn set_timeline_position(&mut self, position: f32) {
        self.timeline_position = position;
    }

    /// Set follow mode
    pub fn set_mode(&mut self, mode: FollowMode) {
        self.mode = mode;
    }

    /// Update camera position and target based on timeline
    ///
    /// Returns (position, target) for the camera
    pub fn update(&mut self, delta_time: f32) -> (Vec3, Vec3) {
        // Auto-rotate orbit angle
        self.orbit_angle += self.auto_rotate_speed * delta_time;
        if self.orbit_angle > 2.0 * PI {
            self.orbit_angle -= 2.0 * PI;
        }

        // Calculate target position based on mode
        let (target_pos, target_look) = match self.mode {
            FollowMode::Orbit => self.calculate_orbit(),
            FollowMode::Tracking => self.calculate_tracking(),
            FollowMode::Cinematic => self.calculate_cinematic(),
            FollowMode::FirstPerson => self.calculate_first_person(),
        };

        // Smooth camera movement
        self.current_position = self.current_position.lerp(target_pos, self.smoothing);
        self.current_target = self.current_target.lerp(target_look, self.smoothing);

        (self.current_position, self.current_target)
    }

    /// Calculate orbit camera position
    fn calculate_orbit(&self) -> (Vec3, Vec3) {
        // Orbit center is at the current timeline position
        let center = Vec3::new(self.timeline_position, 0.0, 0.0);

        // Calculate orbit position
        let x = center.x + self.orbit_radius * self.orbit_angle.cos();
        let z = self.orbit_radius * self.orbit_angle.sin();
        let y = self.orbit_height;

        let position = Vec3::new(x, y, z);
        let target = center + Vec3::new(0.0, 2.0, 0.0);  // Look slightly above center

        (position, target)
    }

    /// Calculate tracking camera position
    fn calculate_tracking(&self) -> (Vec3, Vec3) {
        // Camera follows behind the timeline position
        let position = Vec3::new(
            self.timeline_position - 10.0,  // 10 units behind
            6.0,                            // Fixed height
            5.0 + (self.timeline_position * 0.1).sin() * 3.0,  // Gentle side-to-side
        );

        let target = Vec3::new(
            self.timeline_position + 5.0,   // Look 5 units ahead
            3.0,
            0.0,
        );

        (position, target)
    }

    /// Calculate cinematic camera position
    fn calculate_cinematic(&self) -> (Vec3, Vec3) {
        // Cinematic sweeping camera with varying height
        let t = self.timeline_position * 0.1;  // Slow sweep

        let position = Vec3::new(
            self.timeline_position - 15.0 + (t * 0.5).sin() * 5.0,
            8.0 + (t * 0.3).sin() * 4.0,  // Varying height
            10.0 + (t * 0.7).cos() * 8.0,  // Wide arc
        );

        let target = Vec3::new(
            self.timeline_position,
            2.0 + (t * 0.2).sin() * 2.0,
            0.0,
        );

        (position, target)
    }

    /// Calculate first-person camera position
    fn calculate_first_person(&self) -> (Vec3, Vec3) {
        // First-person: walk through the music at ground level
        let position = Vec3::new(
            self.timeline_position,
            2.0,  // Eye height
            0.0,
        );

        let target = Vec3::new(
            self.timeline_position + 10.0,  // Look forward
            2.0,
            0.0,
        );

        (position, target)
    }

    /// Set orbit parameters
    pub fn set_orbit_params(&mut self, radius: f32, height: f32) {
        self.orbit_radius = radius;
        self.orbit_height = height;
    }

    /// Set smoothing factor (0-1, higher = smoother but more lag)
    pub fn set_smoothing(&mut self, smoothing: f32) {
        self.smoothing = smoothing.clamp(0.0, 1.0);
    }

    /// Set auto-rotation speed (radians/second)
    pub fn set_auto_rotate_speed(&mut self, speed: f32) {
        self.auto_rotate_speed = speed;
    }

    /// Get current camera position
    pub fn position(&self) -> Vec3 {
        self.current_position
    }

    /// Get current camera target
    pub fn target(&self) -> Vec3 {
        self.current_target
    }

    /// Get current timeline position
    pub fn timeline_position(&self) -> f32 {
        self.timeline_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_follow_creation() {
        let controller = CameraFollowController::new(FollowMode::Orbit);
        assert_eq!(controller.mode, FollowMode::Orbit);
        assert_eq!(controller.timeline_position, 0.0);
    }

    #[test]
    fn test_timeline_update() {
        let mut controller = CameraFollowController::new(FollowMode::Orbit);
        controller.set_timeline_position(10.0);
        assert_eq!(controller.timeline_position(), 10.0);
    }

    #[test]
    fn test_orbit_mode() {
        let mut controller = CameraFollowController::new(FollowMode::Orbit);
        controller.set_timeline_position(5.0);

        let (pos, target) = controller.update(0.016);  // 60 FPS

        // Camera should orbit around timeline position
        assert!(pos.length() > 0.0);
        assert!(target.x >= 4.0 && target.x <= 6.0);  // Target near timeline position
    }

    #[test]
    fn test_mode_switching() {
        let mut controller = CameraFollowController::new(FollowMode::Orbit);

        controller.set_mode(FollowMode::Tracking);
        assert_eq!(controller.mode, FollowMode::Tracking);

        controller.set_mode(FollowMode::Cinematic);
        assert_eq!(controller.mode, FollowMode::Cinematic);
    }
}
