//! World Generation Module
//!
//! Generates navigable 3D worlds from spatial musical data.

use crate::genre::GenreStyle;
use crate::mapping::{SpatialMoment, ShapeHint, MaterialHint};
use crate::{Result, SynesthesiaError};
use glam::{Vec3, Mat4};

/// A complete synesthesia world
#[derive(Debug, Clone)]
pub struct SynesthesiaWorld {
    /// Style configuration
    pub style: GenreStyle,
    /// World chunks (spatial partitions)
    pub chunks: Vec<WorldChunk>,
    /// Global lighting settings
    pub lighting: WorldLighting,
    /// Fog/atmosphere settings
    pub atmosphere: AtmosphereSettings,
    /// World bounds
    pub bounds: WorldBounds,
}

impl SynesthesiaWorld {
    /// Create a new empty world with given style
    pub fn new(style: GenreStyle) -> Self {
        Self {
            style,
            chunks: Vec::new(),
            lighting: WorldLighting::default(),
            atmosphere: AtmosphereSettings::default(),
            bounds: WorldBounds::default(),
        }
    }

    /// Generate world from spatial data
    pub fn generate_from_spatial_data(
        &mut self,
        spatial_data: &[SpatialMoment],
        style: &GenreStyle,
    ) -> Result<()> {
        if spatial_data.is_empty() {
            return Err(SynesthesiaError::WorldGenError(
                "No spatial data to generate world from".to_string()
            ));
        }

        // Calculate world bounds
        self.calculate_bounds(spatial_data);

        // Create chunks based on time segments
        let chunk_duration = 10.0; // 10 seconds per chunk
        let chunk_size = self.style.time_scale * chunk_duration;

        let mut current_chunk = WorldChunk::new(0, Vec3::ZERO);
        let mut chunk_start_x = 0.0;
        let mut chunk_index = 0;

        for moment in spatial_data {
            let pos_x = moment.position.x;

            // Start new chunk if we've moved past current chunk boundary
            if pos_x >= chunk_start_x + chunk_size {
                if !current_chunk.elements.is_empty() {
                    current_chunk.finalize();
                    self.chunks.push(current_chunk);
                }

                chunk_index += 1;
                chunk_start_x = (pos_x / chunk_size).floor() * chunk_size;
                current_chunk = WorldChunk::new(
                    chunk_index,
                    Vec3::new(chunk_start_x, 0.0, 0.0),
                );
            }

            // Create world element from spatial moment
            let element = self.create_element(moment, style);
            current_chunk.elements.push(element);
        }

        // Add final chunk
        if !current_chunk.elements.is_empty() {
            current_chunk.finalize();
            self.chunks.push(current_chunk);
        }

        // Configure lighting based on style
        self.configure_lighting(style);

        // Configure atmosphere
        self.configure_atmosphere(style);

        Ok(())
    }

    /// Calculate world bounds from spatial data
    fn calculate_bounds(&mut self, spatial_data: &[SpatialMoment]) {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        for moment in spatial_data {
            let pos = moment.position.to_vec3();
            min = min.min(pos - Vec3::splat(moment.scale));
            max = max.max(pos + Vec3::splat(moment.scale));
        }

        self.bounds = WorldBounds { min, max };
    }

    /// Create a world element from a spatial moment
    fn create_element(&self, moment: &SpatialMoment, style: &GenreStyle) -> WorldElement {
        let position = moment.position.to_vec3();

        // Create transform matrix
        let translation = Mat4::from_translation(position);
        let rotation = Mat4::from_euler(
            glam::EulerRot::XYZ,
            moment.rotation.x,
            moment.rotation.y,
            moment.rotation.z,
        );
        let scale = Mat4::from_scale(Vec3::splat(moment.scale));
        let transform = translation * rotation * scale;

        // Determine element type based on features
        let element_type = self.determine_element_type(moment, style);

        WorldElement {
            id: format!("elem_{:.3}", moment.features.timestamp),
            element_type,
            transform,
            position,
            scale: moment.scale,
            rotation: moment.rotation,
            color: moment.color,
            emission: moment.emission,
            shape: moment.shape,
            material: moment.material_hint.clone(),
            timestamp: moment.features.timestamp,
            is_beat: moment.features.is_beat,
            loudness: moment.features.loudness,
        }
    }

    /// Determine element type based on musical features
    fn determine_element_type(&self, moment: &SpatialMoment, _style: &GenreStyle) -> ElementType {
        let features = &moment.features;

        // Strong beats become landmarks
        if features.is_beat && features.onset_strength > 0.8 {
            return ElementType::Landmark;
        }

        // High loudness creates structural elements
        if features.loudness > 0.7 {
            return ElementType::Structure;
        }

        // High brightness creates decorative elements
        if features.brightness > 0.6 {
            return ElementType::Decoration;
        }

        // Low energy becomes ambient
        if features.arousal < 0.3 {
            return ElementType::Ambient;
        }

        // Default to standard geometry
        ElementType::Geometry
    }

    /// Configure world lighting based on genre style
    fn configure_lighting(&mut self, style: &GenreStyle) {
        self.lighting = WorldLighting {
            ambient_color: style.ambient_color,
            ambient_intensity: style.ambient_intensity,
            sun_direction: Vec3::new(-0.5, -1.0, -0.3).normalize(),
            sun_color: [1.0, 0.95, 0.9],
            sun_intensity: style.sun_intensity,
            fog_enabled: style.fog_enabled,
            fog_color: style.fog_color,
            fog_density: style.fog_density,
        };
    }

    /// Configure atmosphere based on genre style
    fn configure_atmosphere(&mut self, style: &GenreStyle) {
        self.atmosphere = AtmosphereSettings {
            sky_color_top: style.sky_color_top,
            sky_color_horizon: style.sky_color_horizon,
            cloud_coverage: style.cloud_coverage,
            particle_density: if style.particles_enabled { 0.5 } else { 0.0 },
            particle_color: style.particle_color,
        };
    }

    /// Get total element count
    pub fn total_elements(&self) -> usize {
        self.chunks.iter().map(|c| c.elements.len()).sum()
    }

    /// Estimate total vertex count
    pub fn estimated_vertices(&self) -> usize {
        self.chunks.iter()
            .flat_map(|c| &c.elements)
            .map(|e| match e.shape {
                ShapeHint::Organic => 512,
                ShapeHint::Crystalline => 128,
                ShapeHint::Flowing => 256,
                ShapeHint::Particles => 1000,
                ShapeHint::Block => 24,
                ShapeHint::Spire => 64,
                ShapeHint::Dome => 256,
                ShapeHint::Wave => 128,
            })
            .sum()
    }

    /// Get elements near a position
    pub fn get_elements_near(&self, position: Vec3, radius: f32) -> Vec<&WorldElement> {
        let radius_sq = radius * radius;
        self.chunks.iter()
            .flat_map(|c| &c.elements)
            .filter(|e| (e.position - position).length_squared() < radius_sq)
            .collect()
    }

    /// Get chunk containing position
    pub fn get_chunk_at(&self, position: Vec3) -> Option<&WorldChunk> {
        self.chunks.iter().find(|c| c.contains(position))
    }
}

/// A spatial partition of the world
#[derive(Debug, Clone)]
pub struct WorldChunk {
    /// Chunk index
    pub index: usize,
    /// Chunk origin position
    pub origin: Vec3,
    /// Elements in this chunk
    pub elements: Vec<WorldElement>,
    /// Chunk bounding box min
    pub bounds_min: Vec3,
    /// Chunk bounding box max
    pub bounds_max: Vec3,
    /// Is chunk finalized
    pub finalized: bool,
}

impl WorldChunk {
    /// Create a new chunk
    pub fn new(index: usize, origin: Vec3) -> Self {
        Self {
            index,
            origin,
            elements: Vec::new(),
            bounds_min: origin,
            bounds_max: origin,
            finalized: false,
        }
    }

    /// Finalize chunk by calculating bounds
    pub fn finalize(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        for element in &self.elements {
            let half_size = Vec3::splat(element.scale);
            min = min.min(element.position - half_size);
            max = max.max(element.position + half_size);
        }

        self.bounds_min = min;
        self.bounds_max = max;
        self.finalized = true;
    }

    /// Check if position is within chunk bounds
    pub fn contains(&self, position: Vec3) -> bool {
        position.x >= self.bounds_min.x && position.x <= self.bounds_max.x &&
        position.y >= self.bounds_min.y && position.y <= self.bounds_max.y &&
        position.z >= self.bounds_min.z && position.z <= self.bounds_max.z
    }
}

/// A single element in the world
#[derive(Debug, Clone)]
pub struct WorldElement {
    /// Unique identifier
    pub id: String,
    /// Element type
    pub element_type: ElementType,
    /// Transform matrix
    pub transform: Mat4,
    /// World position
    pub position: Vec3,
    /// Scale factor
    pub scale: f32,
    /// Rotation (Euler angles)
    pub rotation: Vec3,
    /// Base color
    pub color: [f32; 3],
    /// Emission intensity
    pub emission: f32,
    /// Shape hint for mesh generation
    pub shape: ShapeHint,
    /// Material properties
    pub material: MaterialHint,
    /// Original timestamp in audio
    pub timestamp: f64,
    /// Was this a beat moment
    pub is_beat: bool,
    /// Loudness at this moment
    pub loudness: f32,
}

/// Types of world elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementType {
    /// Major structural landmark (beat drops, crescendos)
    Landmark,
    /// Supporting structure
    Structure,
    /// Ambient background element
    Ambient,
    /// Decorative detail
    Decoration,
    /// Standard geometry
    Geometry,
    /// Particle system
    Particles,
    /// Light source
    Light,
}

/// World lighting configuration
#[derive(Debug, Clone)]
pub struct WorldLighting {
    pub ambient_color: [f32; 3],
    pub ambient_intensity: f32,
    pub sun_direction: Vec3,
    pub sun_color: [f32; 3],
    pub sun_intensity: f32,
    pub fog_enabled: bool,
    pub fog_color: [f32; 3],
    pub fog_density: f32,
}

impl Default for WorldLighting {
    fn default() -> Self {
        Self {
            ambient_color: [0.1, 0.1, 0.15],
            ambient_intensity: 0.3,
            sun_direction: Vec3::new(-0.5, -1.0, -0.3).normalize(),
            sun_color: [1.0, 0.95, 0.9],
            sun_intensity: 1.0,
            fog_enabled: true,
            fog_color: [0.7, 0.8, 0.9],
            fog_density: 0.01,
        }
    }
}

/// Atmosphere settings
#[derive(Debug, Clone)]
pub struct AtmosphereSettings {
    pub sky_color_top: [f32; 3],
    pub sky_color_horizon: [f32; 3],
    pub cloud_coverage: f32,
    pub particle_density: f32,
    pub particle_color: [f32; 3],
}

impl Default for AtmosphereSettings {
    fn default() -> Self {
        Self {
            sky_color_top: [0.2, 0.3, 0.6],
            sky_color_horizon: [0.6, 0.7, 0.9],
            cloud_coverage: 0.3,
            particle_density: 0.1,
            particle_color: [1.0, 1.0, 1.0],
        }
    }
}

/// World bounds
#[derive(Debug, Clone, Default)]
pub struct WorldBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl WorldBounds {
    /// Get world dimensions
    pub fn dimensions(&self) -> Vec3 {
        self.max - self.min
    }

    /// Get world center
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genre::Genre;

    #[test]
    fn test_world_creation() {
        let style = Genre::Classical.get_style();
        let world = SynesthesiaWorld::new(style);
        assert!(world.chunks.is_empty());
    }

    #[test]
    fn test_chunk_creation() {
        let chunk = WorldChunk::new(0, Vec3::ZERO);
        assert_eq!(chunk.index, 0);
        assert!(chunk.elements.is_empty());
    }
}
