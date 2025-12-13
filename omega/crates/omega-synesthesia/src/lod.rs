//! Level of Detail (LOD) System
//!
//! Manages mesh complexity based on distance for performance optimization.

use crate::geometry::{ProceduralMesh, MeshGenerator};
use crate::mapping::ShapeHint;
use glam::Vec3;
use std::collections::HashMap;

/// LOD level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LodLevel {
    /// Full detail (closest)
    High = 0,
    /// Medium detail
    Medium = 1,
    /// Low detail
    Low = 2,
    /// Billboard/impostor (farthest)
    Billboard = 3,
}

impl LodLevel {
    /// Get subdivision factor for this LOD level
    pub fn subdivision_factor(&self) -> f32 {
        match self {
            Self::High => 1.0,
            Self::Medium => 0.5,
            Self::Low => 0.25,
            Self::Billboard => 0.0, // Uses billboard
        }
    }

    /// Get vertex reduction factor
    pub fn vertex_factor(&self) -> f32 {
        match self {
            Self::High => 1.0,
            Self::Medium => 0.4,
            Self::Low => 0.15,
            Self::Billboard => 0.01,
        }
    }
}

/// LOD configuration
#[derive(Debug, Clone)]
pub struct LodConfig {
    /// Distance thresholds for each LOD level
    pub distances: [f32; 4],
    /// Whether to use screen-space size for LOD selection
    pub use_screen_size: bool,
    /// Screen size thresholds (pixels)
    pub screen_sizes: [f32; 4],
    /// Whether to use smooth transitions
    pub smooth_transitions: bool,
    /// Transition blend distance
    pub blend_distance: f32,
}

impl Default for LodConfig {
    fn default() -> Self {
        Self {
            distances: [0.0, 50.0, 150.0, 400.0],
            use_screen_size: false,
            screen_sizes: [100.0, 50.0, 20.0, 5.0],
            smooth_transitions: true,
            blend_distance: 10.0,
        }
    }
}

/// LOD mesh set for an element
#[derive(Debug, Clone)]
pub struct LodMeshSet {
    /// Meshes for each LOD level
    pub meshes: HashMap<LodLevel, ProceduralMesh>,
    /// Billboard mesh for farthest LOD
    pub billboard: Option<ProceduralMesh>,
    /// Shape hint this was generated from
    pub shape: ShapeHint,
    /// Base scale
    pub scale: f32,
}

impl LodMeshSet {
    /// Get mesh for LOD level
    pub fn get(&self, level: LodLevel) -> Option<&ProceduralMesh> {
        if level == LodLevel::Billboard {
            self.billboard.as_ref().or_else(|| self.meshes.get(&LodLevel::Low))
        } else {
            self.meshes.get(&level).or_else(|| {
                // Fall back to next available lower detail
                match level {
                    LodLevel::High => self.meshes.get(&LodLevel::Medium),
                    LodLevel::Medium => self.meshes.get(&LodLevel::Low),
                    _ => None,
                }
            })
        }
    }

    /// Get vertex count for level
    pub fn vertex_count(&self, level: LodLevel) -> usize {
        self.get(level).map(|m| m.vertex_count()).unwrap_or(0)
    }
}

/// LOD system for managing mesh detail levels
pub struct LodSystem {
    /// Configuration
    pub config: LodConfig,
    /// Mesh generator
    generator: MeshGenerator,
    /// Cached LOD mesh sets
    cache: HashMap<(ShapeHint, u32), LodMeshSet>,
}

impl LodSystem {
    pub fn new(config: LodConfig) -> Self {
        Self {
            config,
            generator: MeshGenerator::new(),
            cache: HashMap::new(),
        }
    }

    /// Get LOD level for distance
    pub fn get_level_for_distance(&self, distance: f32) -> LodLevel {
        if distance < self.config.distances[1] {
            LodLevel::High
        } else if distance < self.config.distances[2] {
            LodLevel::Medium
        } else if distance < self.config.distances[3] {
            LodLevel::Low
        } else {
            LodLevel::Billboard
        }
    }

    /// Get LOD level with blend factor for smooth transitions
    pub fn get_level_with_blend(&self, distance: f32) -> (LodLevel, LodLevel, f32) {
        let level = self.get_level_for_distance(distance);

        if !self.config.smooth_transitions {
            return (level, level, 0.0);
        }

        // Check if we're in a transition zone
        for i in 1..4 {
            let threshold = self.config.distances[i];
            if distance > threshold - self.config.blend_distance &&
               distance < threshold + self.config.blend_distance {
                let blend = (distance - (threshold - self.config.blend_distance)) /
                           (2.0 * self.config.blend_distance);

                let from_level = match i {
                    1 => LodLevel::High,
                    2 => LodLevel::Medium,
                    3 => LodLevel::Low,
                    _ => LodLevel::Billboard,
                };
                let to_level = match i {
                    1 => LodLevel::Medium,
                    2 => LodLevel::Low,
                    _ => LodLevel::Billboard,
                };

                return (from_level, to_level, blend.clamp(0.0, 1.0));
            }
        }

        (level, level, 0.0)
    }

    /// Generate LOD mesh set for a shape
    pub fn generate_lod_set(&mut self, shape: ShapeHint, scale: f32) -> LodMeshSet {
        // Check cache
        let scale_key = (scale * 100.0) as u32;
        let cache_key = (shape, scale_key);

        if let Some(cached) = self.cache.get(&cache_key) {
            return cached.clone();
        }

        let mut meshes = HashMap::new();

        // Generate high detail mesh
        let high_mesh = self.generator.generate_for_hint(shape, scale);
        meshes.insert(LodLevel::High, high_mesh);

        // Generate medium detail mesh
        let medium_mesh = self.generate_reduced_mesh(shape, scale, LodLevel::Medium);
        meshes.insert(LodLevel::Medium, medium_mesh);

        // Generate low detail mesh
        let low_mesh = self.generate_reduced_mesh(shape, scale, LodLevel::Low);
        meshes.insert(LodLevel::Low, low_mesh);

        // Generate billboard
        let billboard = self.generate_billboard(scale);

        let lod_set = LodMeshSet {
            meshes,
            billboard: Some(billboard),
            shape,
            scale,
        };

        // Cache and return
        self.cache.insert(cache_key, lod_set.clone());
        lod_set
    }

    /// Generate reduced detail mesh
    fn generate_reduced_mesh(&self, shape: ShapeHint, scale: f32, level: LodLevel) -> ProceduralMesh {
        let reduction = level.vertex_factor();
        let reduced_scale = scale * reduction.sqrt();

        match shape {
            ShapeHint::Organic => {
                // Sphere with fewer segments
                let segments = match level {
                    LodLevel::High => 16,
                    LodLevel::Medium => 8,
                    LodLevel::Low => 4,
                    LodLevel::Billboard => 2,
                };
                self.generator.generate_sphere(reduced_scale * 0.5, segments)
            }
            ShapeHint::Crystalline => {
                // Simplified icosahedron for low LOD
                if level == LodLevel::Low || level == LodLevel::Billboard {
                    self.generator.generate_cube(reduced_scale)
                } else {
                    self.generator.generate_for_hint(shape, scale)
                }
            }
            ShapeHint::Flowing => {
                // Reduced ribbon segments
                let mesh = self.generator.generate_for_hint(shape, scale);
                self.decimate_mesh(&mesh, reduction)
            }
            ShapeHint::Block => {
                // Cubes are already simple
                self.generator.generate_cube(scale)
            }
            _ => {
                let mesh = self.generator.generate_for_hint(shape, scale);
                self.decimate_mesh(&mesh, reduction)
            }
        }
    }

    /// Simple mesh decimation (keeps every nth vertex)
    fn decimate_mesh(&self, mesh: &ProceduralMesh, factor: f32) -> ProceduralMesh {
        if factor >= 1.0 || mesh.positions.len() < 12 {
            return mesh.clone();
        }

        // Simple vertex decimation - keep every nth vertex
        let keep_ratio = factor.max(0.1);
        let target_vertices = (mesh.positions.len() as f32 * keep_ratio) as usize;
        let step = (mesh.positions.len() / target_vertices).max(1);

        let mut new_positions = Vec::new();
        let mut new_normals = Vec::new();
        let mut new_uvs = Vec::new();
        let mut vertex_map: HashMap<usize, usize> = HashMap::new();

        for (i, (pos, (normal, uv))) in mesh.positions.iter()
            .zip(mesh.normals.iter().zip(mesh.uvs.iter()))
            .enumerate()
            .step_by(step)
        {
            vertex_map.insert(i, new_positions.len());
            new_positions.push(*pos);
            new_normals.push(*normal);
            new_uvs.push(*uv);
        }

        // Remap indices
        let mut new_indices = Vec::new();
        for chunk in mesh.indices.chunks(3) {
            if chunk.len() == 3 {
                // Find closest kept vertex for each index
                let i0 = Self::find_closest_kept(&vertex_map, chunk[0] as usize, step);
                let i1 = Self::find_closest_kept(&vertex_map, chunk[1] as usize, step);
                let i2 = Self::find_closest_kept(&vertex_map, chunk[2] as usize, step);

                // Only add triangle if all vertices are unique
                if i0 != i1 && i1 != i2 && i0 != i2 {
                    new_indices.push(i0 as u32);
                    new_indices.push(i1 as u32);
                    new_indices.push(i2 as u32);
                }
            }
        }

        ProceduralMesh {
            positions: new_positions,
            normals: new_normals,
            uvs: new_uvs,
            indices: new_indices,
            colors: None,
        }
    }

    /// Find closest kept vertex index
    fn find_closest_kept(vertex_map: &HashMap<usize, usize>, idx: usize, step: usize) -> usize {
        // Round to nearest kept vertex
        let rounded = (idx / step) * step;
        *vertex_map.get(&rounded).unwrap_or(&0)
    }

    /// Generate billboard mesh (simple quad)
    fn generate_billboard(&self, scale: f32) -> ProceduralMesh {
        let half = scale * 0.5;

        let positions = vec![
            Vec3::new(-half, -half, 0.0),
            Vec3::new(half, -half, 0.0),
            Vec3::new(half, half, 0.0),
            Vec3::new(-half, half, 0.0),
        ];

        let normals = vec![Vec3::Z; 4];

        let uvs = vec![
            glam::Vec2::new(0.0, 0.0),
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(1.0, 1.0),
            glam::Vec2::new(0.0, 1.0),
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }

    /// Clear LOD cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        let entries = self.cache.len();
        let total_vertices: usize = self.cache.values()
            .flat_map(|set| set.meshes.values())
            .map(|m| m.vertex_count())
            .sum();
        (entries, total_vertices)
    }
}

impl Default for LodSystem {
    fn default() -> Self {
        Self::new(LodConfig::default())
    }
}

/// Instance data for GPU instancing
#[derive(Debug, Clone, Copy)]
pub struct InstanceData {
    /// World transform (4x4 matrix as 16 floats)
    pub transform: [f32; 16],
    /// Color tint
    pub color: [f32; 4],
    /// Custom data (emission, etc)
    pub custom: [f32; 4],
}

/// Instance buffer for batched rendering
#[derive(Debug)]
pub struct InstanceBuffer {
    /// Instance data
    pub instances: Vec<InstanceData>,
    /// Mesh index this buffer is for
    pub mesh_index: usize,
    /// LOD level
    pub lod_level: LodLevel,
}

impl InstanceBuffer {
    pub fn new(mesh_index: usize, lod_level: LodLevel) -> Self {
        Self {
            instances: Vec::new(),
            mesh_index,
            lod_level,
        }
    }

    /// Add instance
    pub fn add(&mut self, transform: glam::Mat4, color: [f32; 4], custom: [f32; 4]) {
        self.instances.push(InstanceData {
            transform: transform.to_cols_array(),
            color,
            custom,
        });
    }

    /// Get instance count
    pub fn count(&self) -> usize {
        self.instances.len()
    }

    /// Convert to byte buffer for GPU upload
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.instances.len() * std::mem::size_of::<InstanceData>());

        for instance in &self.instances {
            for f in &instance.transform {
                bytes.extend(f.to_le_bytes());
            }
            for f in &instance.color {
                bytes.extend(f.to_le_bytes());
            }
            for f in &instance.custom {
                bytes.extend(f.to_le_bytes());
            }
        }

        bytes
    }
}

/// Instance manager for batching similar meshes
pub struct InstanceManager {
    /// Buffers by (shape, scale_bucket, lod_level)
    buffers: HashMap<(ShapeHint, u32, LodLevel), InstanceBuffer>,
    /// Next mesh index
    next_mesh_index: usize,
}

impl InstanceManager {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            next_mesh_index: 0,
        }
    }

    /// Add instance for shape
    pub fn add_instance(
        &mut self,
        shape: ShapeHint,
        scale: f32,
        lod_level: LodLevel,
        transform: glam::Mat4,
        color: [f32; 4],
        custom: [f32; 4],
    ) {
        let scale_bucket = (scale * 10.0) as u32;
        let key = (shape, scale_bucket, lod_level);

        let buffer = self.buffers.entry(key).or_insert_with(|| {
            let mesh_index = self.next_mesh_index;
            self.next_mesh_index += 1;
            InstanceBuffer::new(mesh_index, lod_level)
        });

        buffer.add(transform, color, custom);
    }

    /// Get all buffers
    pub fn get_buffers(&self) -> impl Iterator<Item = &InstanceBuffer> {
        self.buffers.values()
    }

    /// Get buffer count
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }

    /// Get total instance count
    pub fn total_instances(&self) -> usize {
        self.buffers.values().map(|b| b.count()).sum()
    }

    /// Clear all buffers
    pub fn clear(&mut self) {
        self.buffers.clear();
        self.next_mesh_index = 0;
    }
}

impl Default for InstanceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lod_level_selection() {
        let system = LodSystem::default();

        assert_eq!(system.get_level_for_distance(10.0), LodLevel::High);
        assert_eq!(system.get_level_for_distance(100.0), LodLevel::Medium);
        assert_eq!(system.get_level_for_distance(200.0), LodLevel::Low);
        assert_eq!(system.get_level_for_distance(500.0), LodLevel::Billboard);
    }

    #[test]
    fn test_lod_mesh_generation() {
        let mut system = LodSystem::default();
        let lod_set = system.generate_lod_set(ShapeHint::Organic, 1.0);

        assert!(lod_set.get(LodLevel::High).is_some());
        assert!(lod_set.get(LodLevel::Medium).is_some());
        assert!(lod_set.get(LodLevel::Low).is_some());

        // High should have more vertices than low
        let high_verts = lod_set.vertex_count(LodLevel::High);
        let low_verts = lod_set.vertex_count(LodLevel::Low);
        assert!(high_verts > low_verts);
    }

    #[test]
    fn test_instance_buffer() {
        let mut buffer = InstanceBuffer::new(0, LodLevel::High);

        buffer.add(
            glam::Mat4::IDENTITY,
            [1.0, 0.0, 0.0, 1.0],
            [0.0; 4],
        );

        assert_eq!(buffer.count(), 1);

        let bytes = buffer.to_bytes();
        assert_eq!(bytes.len(), std::mem::size_of::<InstanceData>());
    }

    #[test]
    fn test_instance_manager() {
        let mut manager = InstanceManager::new();

        manager.add_instance(
            ShapeHint::Block,
            1.0,
            LodLevel::High,
            glam::Mat4::IDENTITY,
            [1.0, 1.0, 1.0, 1.0],
            [0.0; 4],
        );

        manager.add_instance(
            ShapeHint::Block,
            1.0,
            LodLevel::High,
            glam::Mat4::from_translation(glam::Vec3::X * 10.0),
            [1.0, 1.0, 1.0, 1.0],
            [0.0; 4],
        );

        // Same shape and scale should batch together
        assert_eq!(manager.buffer_count(), 1);
        assert_eq!(manager.total_instances(), 2);
    }
}
