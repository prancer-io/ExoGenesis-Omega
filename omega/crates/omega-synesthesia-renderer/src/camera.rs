//! Camera and camera controller

use glam::{Vec3, Mat4, Quat};

/// 3D camera with projection
pub struct Camera {
    /// Camera position
    pub position: Vec3,

    /// Camera target (look-at point)
    pub target: Vec3,

    /// Up vector
    pub up: Vec3,

    /// Field of view in degrees
    pub fov: f32,

    /// Aspect ratio (width / height)
    pub aspect: f32,

    /// Near clipping plane
    pub near: f32,

    /// Far clipping plane
    pub far: f32,
}

impl Camera {
    /// Create a new camera
    pub fn new(position: Vec3, target: Vec3, fov: f32, aspect: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y,
            fov,
            aspect,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Get view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect,
            self.near,
            self.far,
        )
    }

    /// Get view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Get forward direction
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    /// Get right direction
    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }

    /// Move camera forward/backward
    pub fn move_forward(&mut self, distance: f32) {
        let forward = self.forward();
        self.position += forward * distance;
        self.target += forward * distance;
    }

    /// Move camera right/left
    pub fn move_right(&mut self, distance: f32) {
        let right = self.right();
        self.position += right * distance;
        self.target += right * distance;
    }

    /// Move camera up/down
    pub fn move_up(&mut self, distance: f32) {
        self.position += self.up * distance;
        self.target += self.up * distance;
    }

    /// Orbit camera around target
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let offset = self.position - self.target;
        let distance = offset.length();

        // Convert to spherical coordinates
        let current_pitch = (offset.y / distance).asin();
        let current_yaw = offset.z.atan2(offset.x);

        // Apply rotation
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5);  // Limit pitch

        // Convert back to Cartesian
        let x = distance * new_pitch.cos() * new_yaw.cos();
        let y = distance * new_pitch.sin();
        let z = distance * new_pitch.cos() * new_yaw.sin();

        self.position = self.target + Vec3::new(x, y, z);
    }

    /// Update aspect ratio (e.g., on window resize)
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::ZERO,
            45.0,
            16.0 / 9.0,
        )
    }
}

/// Camera controller for user input
pub struct CameraController {
    /// Move speed in units/second
    pub move_speed: f32,

    /// Rotation speed in radians/pixel
    pub rotation_speed: f32,

    /// Orbit speed in radians/pixel
    pub orbit_speed: f32,

    /// Current movement velocity
    velocity: Vec3,

    /// Current rotation velocity
    rotation_velocity: (f32, f32),
}

impl CameraController {
    /// Create a new camera controller
    pub fn new(move_speed: f32, rotation_speed: f32) -> Self {
        Self {
            move_speed,
            rotation_speed,
            orbit_speed: 0.005,
            velocity: Vec3::ZERO,
            rotation_velocity: (0.0, 0.0),
        }
    }

    /// Update camera based on input (call every frame)
    pub fn update(&mut self, camera: &mut Camera, delta_time: f32) {
        // Apply movement
        if self.velocity.length() > 0.0 {
            camera.move_forward(self.velocity.z * delta_time);
            camera.move_right(self.velocity.x * delta_time);
            camera.move_up(self.velocity.y * delta_time);

            // Damping
            self.velocity *= 0.9;
        }

        // Apply rotation
        if self.rotation_velocity.0.abs() > 0.001 || self.rotation_velocity.1.abs() > 0.001 {
            camera.orbit(
                self.rotation_velocity.0 * delta_time,
                self.rotation_velocity.1 * delta_time,
            );

            // Damping
            self.rotation_velocity.0 *= 0.9;
            self.rotation_velocity.1 *= 0.9;
        }
    }

    /// Handle keyboard input (WASD, QE for movement)
    pub fn process_keyboard(&mut self, key: VirtualKey, pressed: bool) {
        let amount = if pressed { self.move_speed } else { 0.0 };

        match key {
            VirtualKey::W => self.velocity.z = -amount,
            VirtualKey::S => self.velocity.z = amount,
            VirtualKey::A => self.velocity.x = -amount,
            VirtualKey::D => self.velocity.x = amount,
            VirtualKey::Q => self.velocity.y = -amount,
            VirtualKey::E => self.velocity.y = amount,
            _ => {}
        }
    }

    /// Handle mouse drag for orbit
    pub fn process_mouse_drag(&mut self, delta_x: f32, delta_y: f32) {
        self.rotation_velocity.0 = -delta_x * self.orbit_speed;
        self.rotation_velocity.1 = -delta_y * self.orbit_speed;
    }

    /// Handle scroll for zoom
    pub fn process_scroll(&mut self, camera: &mut Camera, delta: f32) {
        let forward = camera.forward();
        let zoom_speed = 0.1;
        camera.position += forward * delta * zoom_speed;
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new(5.0, 0.005)
    }
}

/// Simplified virtual key codes for camera control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualKey {
    W,
    A,
    S,
    D,
    Q,
    E,
    Up,
    Down,
    Left,
    Right,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::default();
        assert_eq!(camera.fov, 45.0);
        assert_eq!(camera.position, Vec3::new(0.0, 5.0, 10.0));
    }

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::default();
        let initial_pos = camera.position;

        camera.move_forward(1.0);
        assert_ne!(camera.position, initial_pos);
    }

    #[test]
    fn test_view_projection() {
        let camera = Camera::default();
        let vp = camera.view_projection_matrix();
        assert!(!vp.is_nan());
    }

    #[test]
    fn test_camera_controller() {
        let mut controller = CameraController::default();
        let mut camera = Camera::default();

        controller.process_keyboard(VirtualKey::W, true);
        controller.update(&mut camera, 0.016);  // 60 FPS

        // Camera should have moved
        assert_ne!(controller.velocity, Vec3::ZERO);
    }
}
