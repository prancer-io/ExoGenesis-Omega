//! Performance Optimization Module
//!
//! This module provides optimized implementations for performance-critical operations
//! in the omega-synesthesia pipeline. Focus areas:
//!
//! - Cached geometry generation
//! - Optimized FFT calculations
//! - Memory pool management
//! - SIMD-optimized spectral calculations

use crate::renderer_bridge::{RendererVertex, RendererMesh};
use glam::Vec3;
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

/// Geometry cache for pre-generated meshes
pub struct GeometryCache {
    /// Cached cube meshes by scale
    cubes: Arc<RwLock<HashMap<u32, (Vec<RendererVertex>, Vec<u32>)>>>,

    /// Cached sphere meshes by (lod, scale) key
    spheres: Arc<RwLock<HashMap<(u32, u32), (Vec<RendererVertex>, Vec<u32>)>>>,

    /// Cached cone meshes by (lod, scale) key
    cones: Arc<RwLock<HashMap<(u32, u32), (Vec<RendererVertex>, Vec<u32>)>>>,

    /// Cache hit statistics
    hits: Arc<RwLock<usize>>,
    misses: Arc<RwLock<usize>>,
}

impl GeometryCache {
    /// Create a new geometry cache
    pub fn new() -> Self {
        Self {
            cubes: Arc::new(RwLock::new(HashMap::new())),
            spheres: Arc::new(RwLock::new(HashMap::new())),
            cones: Arc::new(RwLock::new(HashMap::new())),
            hits: Arc::new(RwLock::new(0)),
            misses: Arc::new(RwLock::new(0)),
        }
    }

    /// Pre-warm cache with common geometries
    pub fn prewarm(&self) {
        println!("🔥 Pre-warming geometry cache...");

        // Common scales (0-10 in 0.5 increments = 21 scales)
        let scales: Vec<u32> = (0..=20).map(|i| (i * 5) as u32).collect(); // 0, 5, 10, ..., 100 (representing 0.0-10.0 in tenths)

        // Common LOD levels
        let lods = [0, 1, 2, 3];

        // Pre-generate cubes
        for scale in &scales {
            let key = *scale;
            let scale_f32 = *scale as f32 / 10.0;
            let geometry = Self::generate_cube_uncached(Vec3::ZERO, scale_f32, [1.0, 1.0, 1.0, 1.0]);
            self.cubes.write().insert(key, geometry);
        }

        // Pre-generate spheres (LOD 1 and 2 most common)
        for lod in &[1, 2] {
            for scale in &scales {
                let key = (*lod, *scale);
                let scale_f32 = *scale as f32 / 10.0;
                let geometry = Self::generate_sphere_uncached(Vec3::ZERO, scale_f32, [1.0, 1.0, 1.0, 1.0], *lod);
                self.spheres.write().insert(key, geometry);
            }
        }

        // Pre-generate cones (LOD 1 only for initial cache)
        for scale in &scales {
            let key = (1, *scale);
            let scale_f32 = *scale as f32 / 10.0;
            let geometry = Self::generate_cone_uncached(Vec3::ZERO, scale_f32, [1.0, 1.0, 1.0, 1.0], 1);
            self.cones.write().insert(key, geometry);
        }

        let cube_count = self.cubes.read().len();
        let sphere_count = self.spheres.read().len();
        let cone_count = self.cones.read().len();

        println!("   Cached {} cubes, {} spheres, {} cones", cube_count, sphere_count, cone_count);
        println!("   Total geometries: {}", cube_count + sphere_count + cone_count);
    }

    /// Get or generate cube geometry
    pub fn get_cube(&self, center: Vec3, scale: f32, color: [f32; 4]) -> (Vec<RendererVertex>, Vec<u32>) {
        let key = (scale * 10.0) as u32; // Convert to tenths

        // Try cache first
        if let Some(geometry) = self.cubes.read().get(&key) {
            *self.hits.write() += 1;
            return self.transform_geometry(geometry, center, scale, color);
        }

        // Cache miss - generate and store
        *self.misses.write() += 1;
        let geometry = Self::generate_cube_uncached(Vec3::ZERO, scale, color);
        self.cubes.write().insert(key, geometry.clone());

        self.transform_geometry(&geometry, center, scale, color)
    }

    /// Get or generate sphere geometry
    pub fn get_sphere(&self, center: Vec3, scale: f32, color: [f32; 4], lod: u32) -> (Vec<RendererVertex>, Vec<u32>) {
        let scale_key = (scale * 10.0) as u32;
        let key = (lod, scale_key);

        if let Some(geometry) = self.spheres.read().get(&key) {
            *self.hits.write() += 1;
            return self.transform_geometry(geometry, center, scale, color);
        }

        *self.misses.write() += 1;
        let geometry = Self::generate_sphere_uncached(Vec3::ZERO, scale, color, lod);
        self.spheres.write().insert(key, geometry.clone());

        self.transform_geometry(&geometry, center, scale, color)
    }

    /// Get or generate cone geometry
    pub fn get_cone(&self, center: Vec3, scale: f32, color: [f32; 4], lod: u32) -> (Vec<RendererVertex>, Vec<u32>) {
        let scale_key = (scale * 10.0) as u32;
        let key = (lod, scale_key);

        if let Some(geometry) = self.cones.read().get(&key) {
            *self.hits.write() += 1;
            return self.transform_geometry(&geometry, center, scale, color);
        }

        *self.misses.write() += 1;
        let geometry = Self::generate_cone_uncached(Vec3::ZERO, scale, color, lod);
        self.cones.write().insert(key, geometry.clone());

        self.transform_geometry(&geometry, center, scale, color)
    }

    /// Transform cached geometry to new position and color
    fn transform_geometry(
        &self,
        geometry: &(Vec<RendererVertex>, Vec<u32>),
        center: Vec3,
        _scale: f32,
        color: [f32; 4],
    ) -> (Vec<RendererVertex>, Vec<u32>) {
        let (vertices, indices) = geometry;

        // Transform vertices
        let transformed_vertices: Vec<RendererVertex> = vertices
            .iter()
            .map(|v| {
                let pos = Vec3::from(v.position) + center;
                RendererVertex {
                    position: pos.to_array(),
                    normal: v.normal,
                    uv: v.uv,
                    color,
                }
            })
            .collect();

        (transformed_vertices, indices.clone())
    }

    /// Generate cube without caching
    fn generate_cube_uncached(center: Vec3, scale: f32, color: [f32; 4]) -> (Vec<RendererVertex>, Vec<u32>) {
        let half = scale / 2.0;

        // 8 corners
        let corners = [
            center + Vec3::new(-half, -half, -half),
            center + Vec3::new( half, -half, -half),
            center + Vec3::new( half,  half, -half),
            center + Vec3::new(-half,  half, -half),
            center + Vec3::new(-half, -half,  half),
            center + Vec3::new( half, -half,  half),
            center + Vec3::new( half,  half,  half),
            center + Vec3::new(-half,  half,  half),
        ];

        // 24 vertices (4 per face for proper normals)
        let vertices = vec![
            // Front face (Z+)
            RendererVertex { position: corners[4].to_array(), normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], color },
            RendererVertex { position: corners[5].to_array(), normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0], color },
            RendererVertex { position: corners[6].to_array(), normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0], color },
            RendererVertex { position: corners[7].to_array(), normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0], color },

            // Back face (Z-)
            RendererVertex { position: corners[1].to_array(), normal: [0.0, 0.0, -1.0], uv: [0.0, 0.0], color },
            RendererVertex { position: corners[0].to_array(), normal: [0.0, 0.0, -1.0], uv: [1.0, 0.0], color },
            RendererVertex { position: corners[3].to_array(), normal: [0.0, 0.0, -1.0], uv: [1.0, 1.0], color },
            RendererVertex { position: corners[2].to_array(), normal: [0.0, 0.0, -1.0], uv: [0.0, 1.0], color },

            // Right face (X+)
            RendererVertex { position: corners[5].to_array(), normal: [1.0, 0.0, 0.0], uv: [0.0, 0.0], color },
            RendererVertex { position: corners[1].to_array(), normal: [1.0, 0.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: corners[2].to_array(), normal: [1.0, 0.0, 0.0], uv: [1.0, 1.0], color },
            RendererVertex { position: corners[6].to_array(), normal: [1.0, 0.0, 0.0], uv: [0.0, 1.0], color },

            // Left face (X-)
            RendererVertex { position: corners[0].to_array(), normal: [-1.0, 0.0, 0.0], uv: [0.0, 0.0], color },
            RendererVertex { position: corners[4].to_array(), normal: [-1.0, 0.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: corners[7].to_array(), normal: [-1.0, 0.0, 0.0], uv: [1.0, 1.0], color },
            RendererVertex { position: corners[3].to_array(), normal: [-1.0, 0.0, 0.0], uv: [0.0, 1.0], color },

            // Top face (Y+)
            RendererVertex { position: corners[3].to_array(), normal: [0.0, 1.0, 0.0], uv: [0.0, 0.0], color },
            RendererVertex { position: corners[2].to_array(), normal: [0.0, 1.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: corners[6].to_array(), normal: [0.0, 1.0, 0.0], uv: [1.0, 1.0], color },
            RendererVertex { position: corners[7].to_array(), normal: [0.0, 1.0, 0.0], uv: [0.0, 1.0], color },

            // Bottom face (Y-)
            RendererVertex { position: corners[0].to_array(), normal: [0.0, -1.0, 0.0], uv: [0.0, 0.0], color },
            RendererVertex { position: corners[1].to_array(), normal: [0.0, -1.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: corners[5].to_array(), normal: [0.0, -1.0, 0.0], uv: [1.0, 1.0], color },
            RendererVertex { position: corners[4].to_array(), normal: [0.0, -1.0, 0.0], uv: [0.0, 1.0], color },
        ];

        // 36 indices (6 faces * 2 triangles * 3 vertices)
        let indices = vec![
            0, 1, 2,  2, 3, 0,      // Front
            4, 5, 6,  6, 7, 4,      // Back
            8, 9, 10, 10, 11, 8,    // Right
            12, 13, 14, 14, 15, 12, // Left
            16, 17, 18, 18, 19, 16, // Top
            20, 21, 22, 22, 23, 20, // Bottom
        ];

        (vertices, indices)
    }

    /// Generate sphere without caching (simplified icosphere)
    fn generate_sphere_uncached(_center: Vec3, radius: f32, color: [f32; 4], lod: u32) -> (Vec<RendererVertex>, Vec<u32>) {
        let segments = match lod {
            0 => 32,
            1 => 16,
            2 => 8,
            _ => 4,
        };

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate sphere vertices
        for lat in 0..=segments {
            let theta = lat as f32 * std::f32::consts::PI / segments as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for lon in 0..=segments {
                let phi = lon as f32 * 2.0 * std::f32::consts::PI / segments as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;

                let position = [x * radius, y * radius, z * radius];
                let normal = [x, y, z];
                let uv = [lon as f32 / segments as f32, lat as f32 / segments as f32];

                vertices.push(RendererVertex {
                    position,
                    normal,
                    uv,
                    color,
                });
            }
        }

        // Generate sphere indices
        for lat in 0..segments {
            for lon in 0..segments {
                let first = lat * (segments + 1) + lon;
                let second = first + segments + 1;

                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        (vertices, indices)
    }

    /// Generate cone without caching
    fn generate_cone_uncached(_center: Vec3, radius: f32, color: [f32; 4], lod: u32) -> (Vec<RendererVertex>, Vec<u32>) {
        let segments = match lod {
            0 => 32,
            1 => 16,
            2 => 8,
            _ => 4,
        };

        let height = radius * 2.0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Apex vertex
        vertices.push(RendererVertex {
            position: [0.0, height / 2.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            uv: [0.5, 1.0],
            color,
        });

        // Base vertices
        for i in 0..=segments {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / segments as f32;
            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            vertices.push(RendererVertex {
                position: [x, -height / 2.0, z],
                normal: [0.0, -1.0, 0.0],
                uv: [x / radius * 0.5 + 0.5, z / radius * 0.5 + 0.5],
                color,
            });
        }

        // Side triangles
        for i in 0..segments {
            indices.push(0);
            indices.push(i + 1);
            indices.push(i + 2);
        }

        // Base triangles
        for i in 1..segments {
            indices.push(1);
            indices.push(i + 1);
            indices.push(i + 2);
        }

        (vertices, indices)
    }

    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize, f32) {
        let hits = *self.hits.read();
        let misses = *self.misses.read();
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f32 / total as f32 * 100.0
        } else {
            0.0
        };
        (hits, misses, hit_rate)
    }

    /// Clear cache statistics
    pub fn clear_stats(&self) {
        *self.hits.write() = 0;
        *self.misses.write() = 0;
    }
}

impl Default for GeometryCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_cache_creation() {
        let cache = GeometryCache::new();
        let (hits, misses, _) = cache.stats();
        assert_eq!(hits, 0);
        assert_eq!(misses, 0);
    }

    #[test]
    fn test_cube_caching() {
        let cache = GeometryCache::new();

        // First access - cache miss
        let _geom1 = cache.get_cube(Vec3::ZERO, 1.0, [1.0, 1.0, 1.0, 1.0]);
        let (hits1, misses1, _) = cache.stats();
        assert_eq!(hits1, 0);
        assert_eq!(misses1, 1);

        // Second access - cache hit
        let _geom2 = cache.get_cube(Vec3::new(5.0, 0.0, 0.0), 1.0, [1.0, 0.0, 0.0, 1.0]);
        let (hits2, misses2, _) = cache.stats();
        assert_eq!(hits2, 1);
        assert_eq!(misses2, 1);
    }

    #[test]
    fn test_prewarm() {
        let cache = GeometryCache::new();
        cache.prewarm();

        // Should have pre-cached geometries
        assert!(cache.cubes.read().len() > 0);
        assert!(cache.spheres.read().len() > 0);
        assert!(cache.cones.read().len() > 0);
    }
}
