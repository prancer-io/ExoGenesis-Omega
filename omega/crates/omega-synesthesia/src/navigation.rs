//! Navigation Path Generation
//!
//! Creates walkable paths through synesthesia worlds.

use crate::features::MusicalFeatures;
use crate::world::{SynesthesiaWorld, WorldElement};
use crate::biome::{Biome, BiomeType};
use glam::Vec3;

/// A navigation path through the world
#[derive(Debug, Clone)]
pub struct NavigationPath {
    /// Path name
    pub name: String,
    /// Path waypoints
    pub waypoints: Vec<PathWaypoint>,
    /// Total path length
    pub total_length: f32,
    /// Estimated walk time (seconds)
    pub estimated_time: f32,
    /// Is this a loop?
    pub is_loop: bool,
}

impl NavigationPath {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            waypoints: Vec::new(),
            total_length: 0.0,
            estimated_time: 0.0,
            is_loop: false,
        }
    }

    /// Add waypoint
    pub fn add_waypoint(&mut self, waypoint: PathWaypoint) {
        if let Some(last) = self.waypoints.last() {
            self.total_length += (waypoint.position - last.position).length();
        }
        self.waypoints.push(waypoint);
    }

    /// Calculate estimated time at given walk speed
    pub fn calculate_time(&mut self, walk_speed: f32) {
        self.estimated_time = self.total_length / walk_speed;
    }

    /// Get position at normalized progress (0-1)
    pub fn get_position_at(&self, progress: f32) -> Option<Vec3> {
        if self.waypoints.is_empty() {
            return None;
        }

        let target_distance = progress.clamp(0.0, 1.0) * self.total_length;
        let mut accumulated = 0.0f32;

        for i in 0..self.waypoints.len() - 1 {
            let w0 = &self.waypoints[i];
            let w1 = &self.waypoints[i + 1];
            let segment_length = (w1.position - w0.position).length();

            if accumulated + segment_length >= target_distance {
                let t = (target_distance - accumulated) / segment_length;
                return Some(w0.position.lerp(w1.position, t));
            }

            accumulated += segment_length;
        }

        Some(self.waypoints.last().unwrap().position)
    }

    /// Get waypoint at normalized progress
    pub fn get_waypoint_at(&self, progress: f32) -> Option<&PathWaypoint> {
        if self.waypoints.is_empty() {
            return None;
        }

        let index = ((progress.clamp(0.0, 1.0) * (self.waypoints.len() - 1) as f32) as usize)
            .min(self.waypoints.len() - 1);
        Some(&self.waypoints[index])
    }
}

/// A single waypoint on a path
#[derive(Debug, Clone)]
pub struct PathWaypoint {
    /// World position
    pub position: Vec3,
    /// Look direction (normalized)
    pub look_direction: Vec3,
    /// Audio timestamp this corresponds to
    pub audio_time: f32,
    /// Waypoint type
    pub waypoint_type: WaypointType,
    /// Points of interest visible from here
    pub points_of_interest: Vec<PointOfInterest>,
    /// Ambient description
    pub ambient_description: String,
}

impl PathWaypoint {
    pub fn new(position: Vec3, audio_time: f32) -> Self {
        Self {
            position,
            look_direction: Vec3::X,
            audio_time,
            waypoint_type: WaypointType::Path,
            points_of_interest: Vec::new(),
            ambient_description: String::new(),
        }
    }

    /// Set look direction toward a target
    pub fn look_at(&mut self, target: Vec3) {
        let dir = target - self.position;
        if dir.length() > 0.001 {
            self.look_direction = dir.normalize();
        }
    }
}

/// Types of path waypoints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaypointType {
    /// Regular path point
    Path,
    /// Start of path
    Start,
    /// End of path
    End,
    /// Vista/lookout point
    Vista,
    /// Beat moment (landmark)
    Beat,
    /// Transition between biomes
    Transition,
    /// Rest/pause point
    RestArea,
}

/// Point of interest visible from a waypoint
#[derive(Debug, Clone)]
pub struct PointOfInterest {
    /// Name/description
    pub name: String,
    /// Position
    pub position: Vec3,
    /// Distance from waypoint
    pub distance: f32,
    /// Direction from waypoint
    pub direction: Vec3,
    /// Importance (0-1)
    pub importance: f32,
}

/// Navigation path generator
pub struct PathGenerator {
    /// Walk height above ground
    pub walk_height: f32,
    /// Path smoothing iterations
    pub smoothing_iterations: usize,
    /// Minimum distance between waypoints
    pub min_waypoint_distance: f32,
    /// Vista detection radius
    pub vista_radius: f32,
    /// Walk speed for time estimation
    pub walk_speed: f32,
}

impl PathGenerator {
    pub fn new() -> Self {
        Self {
            walk_height: 2.0,
            smoothing_iterations: 3,
            min_waypoint_distance: 5.0,
            vista_radius: 50.0,
            walk_speed: 5.0,
        }
    }

    /// Generate main timeline path from features
    pub fn generate_timeline_path(
        &self,
        features: &[MusicalFeatures],
        world: &SynesthesiaWorld,
        time_scale: f32,
    ) -> NavigationPath {
        let mut path = NavigationPath::new("Timeline");

        if features.is_empty() {
            return path;
        }

        let mut last_pos = Vec3::ZERO;

        for (i, feature) in features.iter().enumerate() {
            // Calculate position along time axis
            let x = feature.timestamp as f32 * time_scale;
            let y = self.walk_height;
            let z = 0.0;

            let position = Vec3::new(x, y, z);

            // Check minimum distance
            if i > 0 && (position - last_pos).length() < self.min_waypoint_distance {
                continue;
            }
            last_pos = position;

            let mut waypoint = PathWaypoint::new(position, feature.timestamp as f32);

            // Determine waypoint type
            waypoint.waypoint_type = if i == 0 {
                WaypointType::Start
            } else if i == features.len() - 1 {
                WaypointType::End
            } else if feature.is_beat && feature.onset_strength > 0.7 {
                WaypointType::Beat
            } else if feature.arousal < 0.3 {
                WaypointType::RestArea
            } else {
                WaypointType::Path
            };

            // Find nearby points of interest
            let nearby_elements = world.get_elements_near(position, self.vista_radius);
            for element in nearby_elements.iter().take(5) {
                let poi = PointOfInterest {
                    name: element.id.clone(),
                    position: element.position,
                    distance: (element.position - position).length(),
                    direction: (element.position - position).normalize_or_zero(),
                    importance: element.loudness,
                };
                waypoint.points_of_interest.push(poi);
            }

            // Set look direction toward most important POI
            if let Some(poi) = waypoint.points_of_interest.first() {
                waypoint.look_at(poi.position);
            } else {
                waypoint.look_direction = Vec3::X; // Look forward
            }

            // Generate ambient description
            waypoint.ambient_description = self.generate_description(feature);

            path.add_waypoint(waypoint);
        }

        // Smooth the path
        self.smooth_path(&mut path);

        // Calculate time
        path.calculate_time(self.walk_speed);

        path
    }

    /// Generate exploration path that visits landmarks
    pub fn generate_exploration_path(
        &self,
        world: &SynesthesiaWorld,
        biomes: &[Biome],
    ) -> NavigationPath {
        let mut path = NavigationPath::new("Exploration");

        // Collect landmarks
        let landmarks: Vec<&WorldElement> = world.chunks.iter()
            .flat_map(|c| &c.elements)
            .filter(|e| e.is_beat && e.loudness > 0.6)
            .collect();

        if landmarks.is_empty() {
            return path;
        }

        // Sort by X position (time)
        let mut sorted: Vec<_> = landmarks.into_iter().collect();
        sorted.sort_by(|a, b| a.position.x.partial_cmp(&b.position.x).unwrap());

        for (i, element) in sorted.iter().enumerate() {
            let position = element.position + Vec3::new(0.0, self.walk_height, -5.0);
            let mut waypoint = PathWaypoint::new(position, element.timestamp as f32);

            waypoint.waypoint_type = if i == 0 {
                WaypointType::Start
            } else if i == sorted.len() - 1 {
                WaypointType::End
            } else {
                WaypointType::Vista
            };

            waypoint.look_at(element.position);

            // Check which biome we're in
            if let Some(biome) = biomes.iter().find(|b| {
                (position - b.center).length() < b.radius
            }) {
                waypoint.ambient_description = format!(
                    "{:?} region - {}",
                    biome.biome_type,
                    self.biome_description(&biome.biome_type)
                );
            }

            path.add_waypoint(waypoint);
        }

        self.smooth_path(&mut path);
        path.calculate_time(self.walk_speed);

        path
    }

    /// Generate scenic route through biomes
    pub fn generate_scenic_route(&self, biomes: &[Biome]) -> NavigationPath {
        let mut path = NavigationPath::new("Scenic");

        if biomes.is_empty() {
            return path;
        }

        // Visit each biome center
        for (i, biome) in biomes.iter().enumerate() {
            if biome.biome_type == BiomeType::Transition {
                continue;
            }

            let position = biome.center + Vec3::new(0.0, self.walk_height, 0.0);
            let mut waypoint = PathWaypoint::new(position, i as f32 * 5.0);

            waypoint.waypoint_type = if i == 0 {
                WaypointType::Start
            } else {
                WaypointType::Vista
            };

            waypoint.ambient_description = format!(
                "Entering {} zone - {}",
                format!("{:?}", biome.biome_type).to_lowercase(),
                self.biome_description(&biome.biome_type)
            );

            // Look toward next biome
            if i + 1 < biomes.len() {
                waypoint.look_at(biomes[i + 1].center);
            }

            path.add_waypoint(waypoint);
        }

        path.calculate_time(self.walk_speed);

        path
    }

    /// Smooth path by averaging adjacent waypoints
    fn smooth_path(&self, path: &mut NavigationPath) {
        if path.waypoints.len() < 3 {
            return;
        }

        for _ in 0..self.smoothing_iterations {
            let mut smoothed = Vec::with_capacity(path.waypoints.len());

            // Keep first and last
            smoothed.push(path.waypoints[0].clone());

            for i in 1..path.waypoints.len() - 1 {
                let prev = &path.waypoints[i - 1];
                let curr = &path.waypoints[i];
                let next = &path.waypoints[i + 1];

                let mut smoothed_waypoint = curr.clone();
                smoothed_waypoint.position = (prev.position + curr.position * 2.0 + next.position) / 4.0;

                smoothed.push(smoothed_waypoint);
            }

            smoothed.push(path.waypoints.last().unwrap().clone());
            path.waypoints = smoothed;
        }

        // Recalculate total length
        path.total_length = 0.0;
        for i in 1..path.waypoints.len() {
            path.total_length += (path.waypoints[i].position - path.waypoints[i - 1].position).length();
        }
    }

    /// Generate description for a feature
    fn generate_description(&self, feature: &MusicalFeatures) -> String {
        let emotion = match feature.emotion {
            crate::features::EmotionalValence::Joy => "joyful, uplifting",
            crate::features::EmotionalValence::Sadness => "melancholic, reflective",
            crate::features::EmotionalValence::Anger => "intense, powerful",
            crate::features::EmotionalValence::Peace => "serene, calm",
            crate::features::EmotionalValence::Fear => "tense, suspenseful",
            crate::features::EmotionalValence::Surprise => "unexpected, dynamic",
            crate::features::EmotionalValence::Neutral => "balanced, flowing",
        };

        let energy = if feature.arousal > 0.7 {
            "high energy"
        } else if feature.arousal < 0.3 {
            "gentle"
        } else {
            "moderate"
        };

        format!("{} passage with {} atmosphere", energy, emotion)
    }

    /// Get description for biome type
    fn biome_description(&self, biome_type: &BiomeType) -> &'static str {
        match biome_type {
            BiomeType::Euphoria => "vibrant crystalline structures reaching skyward",
            BiomeType::Melancholy => "flowing forms in muted blue tones",
            BiomeType::Tension => "angular red formations creating dramatic shadows",
            BiomeType::Serenity => "soft organic shapes in calming greens",
            BiomeType::Rhythmic => "pulsing geometric patterns",
            BiomeType::Melodic => "sweeping curves in harmonic colors",
            BiomeType::Chaos => "fragmented structures in shifting hues",
            BiomeType::Minimal => "sparse elements in clean space",
            BiomeType::Transition => "gradual shift between regions",
        }
    }
}

impl Default for PathGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation mesh for walkable surfaces
#[derive(Debug, Clone)]
pub struct NavMesh {
    /// Vertices
    pub vertices: Vec<Vec3>,
    /// Triangle indices
    pub triangles: Vec<[usize; 3]>,
    /// Walkable mask per triangle
    pub walkable: Vec<bool>,
}

impl NavMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            walkable: Vec::new(),
        }
    }

    /// Generate simple ground plane navmesh
    pub fn generate_ground_plane(min: Vec3, max: Vec3, resolution: f32) -> Self {
        let mut mesh = Self::new();

        let x_steps = ((max.x - min.x) / resolution) as usize + 1;
        let z_steps = ((max.z - min.z) / resolution) as usize + 1;

        // Generate vertices
        for i in 0..x_steps {
            for j in 0..z_steps {
                let x = min.x + i as f32 * resolution;
                let z = min.z + j as f32 * resolution;
                mesh.vertices.push(Vec3::new(x, 0.0, z));
            }
        }

        // Generate triangles
        for i in 0..x_steps - 1 {
            for j in 0..z_steps - 1 {
                let v0 = i * z_steps + j;
                let v1 = v0 + 1;
                let v2 = (i + 1) * z_steps + j;
                let v3 = v2 + 1;

                mesh.triangles.push([v0, v1, v2]);
                mesh.walkable.push(true);

                mesh.triangles.push([v1, v3, v2]);
                mesh.walkable.push(true);
            }
        }

        mesh
    }

    /// Find closest point on navmesh
    pub fn closest_point(&self, position: Vec3) -> Option<Vec3> {
        if self.triangles.is_empty() {
            return None;
        }

        let mut closest = self.vertices[0];
        let mut closest_dist = f32::MAX;

        for tri in &self.triangles {
            let v0 = self.vertices[tri[0]];
            let v1 = self.vertices[tri[1]];
            let v2 = self.vertices[tri[2]];

            let center = (v0 + v1 + v2) / 3.0;
            let dist = (center - position).length();

            if dist < closest_dist {
                closest = center;
                closest_dist = dist;
            }
        }

        Some(closest)
    }

    /// Check if position is on navmesh
    pub fn is_on_mesh(&self, position: Vec3, tolerance: f32) -> bool {
        for tri in &self.triangles {
            let v0 = self.vertices[tri[0]];
            let v1 = self.vertices[tri[1]];
            let v2 = self.vertices[tri[2]];

            if self.point_in_triangle(position, v0, v1, v2, tolerance) {
                return true;
            }
        }
        false
    }

    /// Check if point is in triangle (2D projection)
    fn point_in_triangle(&self, p: Vec3, v0: Vec3, v1: Vec3, v2: Vec3, tolerance: f32) -> bool {
        // Check Y tolerance
        let tri_y = (v0.y + v1.y + v2.y) / 3.0;
        if (p.y - tri_y).abs() > tolerance {
            return false;
        }

        // Barycentric coordinates
        let d00 = (v1 - v0).xz().dot((v1 - v0).xz());
        let d01 = (v1 - v0).xz().dot((v2 - v0).xz());
        let d11 = (v2 - v0).xz().dot((v2 - v0).xz());
        let d20 = (p - v0).xz().dot((v1 - v0).xz());
        let d21 = (p - v0).xz().dot((v2 - v0).xz());

        let denom = d00 * d11 - d01 * d01;
        if denom.abs() < 0.0001 {
            return false;
        }

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        u >= 0.0 && v >= 0.0 && w >= 0.0
    }
}

impl Default for NavMesh {
    fn default() -> Self {
        Self::new()
    }
}

trait Vec3Ext {
    fn xz(&self) -> glam::Vec2;
}

impl Vec3Ext for Vec3 {
    fn xz(&self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::{ChordType, EmotionalValence};
    use crate::genre::Genre;

    fn create_test_feature(timestamp: f64) -> MusicalFeatures {
        MusicalFeatures {
            timestamp,
            pitch: 440.0,
            midi_note: 69,
            pitch_class: 9,
            octave: 4,
            onset_strength: 0.5,
            is_beat: timestamp as i64 % 2 == 0,
            tempo: 120.0,
            beat_phase: 0.0,
            loudness: 0.6,
            dynamics_delta: 0.0,
            brightness: 0.5,
            roughness: 0.3,
            warmth: 0.5,
            sharpness: 0.4,
            key: 0,
            chord_type: ChordType::Major,
            tension: 0.3,
            valence: 0.5,
            arousal: 0.6,
            emotion: EmotionalValence::Joy,
            mfcc: [0.0; 13],
            chroma: [0.0; 12],
        }
    }

    #[test]
    fn test_path_generation() {
        let gen = PathGenerator::new();
        let style = Genre::Classical.get_style();
        let world = SynesthesiaWorld::new(style);

        let features: Vec<_> = (0..20)
            .map(|i| create_test_feature(i as f64))
            .collect();

        let path = gen.generate_timeline_path(&features, &world, 10.0);

        assert!(!path.waypoints.is_empty());
        assert!(path.total_length > 0.0);
    }

    #[test]
    fn test_path_position_sampling() {
        let mut path = NavigationPath::new("Test");

        path.add_waypoint(PathWaypoint::new(Vec3::ZERO, 0.0));
        path.add_waypoint(PathWaypoint::new(Vec3::new(10.0, 0.0, 0.0), 1.0));

        let mid = path.get_position_at(0.5).unwrap();
        assert!((mid.x - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_navmesh_generation() {
        let mesh = NavMesh::generate_ground_plane(
            Vec3::new(-10.0, 0.0, -10.0),
            Vec3::new(10.0, 0.0, 10.0),
            5.0,
        );

        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.triangles.is_empty());
    }

    #[test]
    fn test_navmesh_query() {
        let mesh = NavMesh::generate_ground_plane(
            Vec3::new(-10.0, 0.0, -10.0),
            Vec3::new(10.0, 0.0, 10.0),
            5.0,
        );

        assert!(mesh.is_on_mesh(Vec3::new(0.0, 0.0, 0.0), 1.0));
    }
}
