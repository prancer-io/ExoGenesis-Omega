//! Renderer Bridge - Convert Synesthesia Worlds to GPU Meshes
//!
//! This module bridges omega-synesthesia's world representation with
//! omega-synesthesia-renderer's GPU rendering system.
//!
//! ## Architecture
//!
//! ```text
//! WorldChunk → MeshConverter → GPU Meshes → SynesthesiaRenderer
//!   (Elements)    (Bridge)      (wgpu)        (60 FPS)
//! ```

use crate::world::{WorldChunk, WorldElement, ElementType};
use crate::optimization::GeometryCache;
use glam::Vec3;
use std::sync::Arc;

/// Vertex format compatible with omega-synesthesia-renderer
///
/// Note: This matches the Vertex struct in omega-synesthesia-renderer.
/// In a real implementation, we'd import from the renderer crate.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RendererVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

/// Mesh data ready for GPU upload
#[derive(Debug, Clone)]
pub struct RendererMesh {
    pub name: String,
    pub vertices: Vec<RendererVertex>,
    pub indices: Vec<u32>,
}

/// PBR material compatible with renderer
#[derive(Debug, Clone, Copy)]
pub struct RendererMaterial {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub emission: [f32; 3],
    pub emission_strength: f32,
}

/// Converts WorldChunks to renderer-compatible meshes
pub struct MeshConverter {
    /// LOD level for mesh generation (0 = highest detail)
    lod_level: u32,

    /// Mesh cache to avoid regenerating identical elements
    cache_enabled: bool,

    /// Geometry cache for optimized performance
    geometry_cache: Option<Arc<GeometryCache>>,
}

impl MeshConverter {
    /// Create a new mesh converter
    pub fn new(lod_level: u32) -> Self {
        Self {
            lod_level,
            cache_enabled: false,
            geometry_cache: None,
        }
    }

    /// Create a new mesh converter with geometry caching enabled (Week 4 optimization!)
    pub fn new_with_cache(lod_level: u32) -> Self {
        let cache = Arc::new(GeometryCache::new());
        cache.prewarm();  // Pre-generate common geometries

        Self {
            lod_level,
            cache_enabled: true,
            geometry_cache: Some(cache),
        }
    }

    /// Get geometry cache statistics
    pub fn cache_stats(&self) -> Option<(usize, usize, f32)> {
        self.geometry_cache.as_ref().map(|cache| cache.stats())
    }

    /// Convert a world chunk to renderer meshes
    pub fn convert_chunk(&self, chunk: &WorldChunk) -> Vec<(RendererMesh, RendererMaterial)> {
        let mut meshes = Vec::new();

        for element in &chunk.elements {
            if let Some((mesh, material)) = self.convert_element(element) {
                meshes.push((mesh, material));
            }
        }

        meshes
    }

    /// Convert a single world element to mesh + material
    fn convert_element(&self, element: &WorldElement) -> Option<(RendererMesh, RendererMaterial)> {
        // Generate geometry based on element type and shape
        let (vertices, indices) = self.generate_geometry(element)?;

        // Create mesh
        let mesh = RendererMesh {
            name: element.id.clone(),
            vertices,
            indices,
        };

        // Create material from element properties
        let material = self.create_material(element);

        Some((mesh, material))
    }

    /// Generate geometry for an element
    fn generate_geometry(&self, element: &WorldElement) -> Option<(Vec<RendererVertex>, Vec<u32>)> {
        use crate::mapping::ShapeHint;

        // Extract transform components
        let position = element.position;
        let scale = element.scale;

        // Convert [f32; 3] to [f32; 4] for color
        let color = [element.color[0], element.color[1], element.color[2], 1.0];

        // Use cached geometry if available (Week 4 optimization!)
        if let Some(cache) = &self.geometry_cache {
            return match element.shape {
                ShapeHint::Block => Some(cache.get_cube(position, scale, color)),
                ShapeHint::Organic => Some(cache.get_sphere(position, scale, color, self.lod_level)),
                ShapeHint::Spire => Some(cache.get_cone(position, scale, color, self.lod_level)),
                ShapeHint::Dome => Some(cache.get_sphere(position, scale * 0.8, color, self.lod_level)),
                ShapeHint::Crystalline => Some(cache.get_cube(position, scale, color)),
                _ => Some(cache.get_cube(position, scale, color)),
            };
        }

        // Fallback to uncached generation
        match element.shape {
            ShapeHint::Block => self.generate_cube(position, scale, color),
            ShapeHint::Organic => self.generate_sphere(position, scale, color, self.lod_level),
            ShapeHint::Spire => self.generate_cone(position, scale, color, self.lod_level),
            ShapeHint::Dome => self.generate_sphere(position, scale * 0.8, color, self.lod_level),
            ShapeHint::Crystalline => self.generate_cube(position, scale, color),
            _ => self.generate_cube(position, scale, color), // Default to cube
        }
    }

    /// Generate cube geometry
    fn generate_cube(&self, center: Vec3, size: f32, color: [f32; 4]) -> Option<(Vec<RendererVertex>, Vec<u32>)> {
        let s = size / 2.0;

        let vertices = vec![
            // Front face (Z+)
            RendererVertex { position: [center.x - s, center.y - s, center.z + s], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], color },
            RendererVertex { position: [center.x + s, center.y - s, center.z + s], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0], color },
            RendererVertex { position: [center.x + s, center.y + s, center.z + s], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0], color },
            RendererVertex { position: [center.x - s, center.y + s, center.z + s], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0], color },

            // Back face (Z-)
            RendererVertex { position: [center.x - s, center.y - s, center.z - s], normal: [0.0, 0.0, -1.0], uv: [1.0, 0.0], color },
            RendererVertex { position: [center.x - s, center.y + s, center.z - s], normal: [0.0, 0.0, -1.0], uv: [1.0, 1.0], color },
            RendererVertex { position: [center.x + s, center.y + s, center.z - s], normal: [0.0, 0.0, -1.0], uv: [0.0, 1.0], color },
            RendererVertex { position: [center.x + s, center.y - s, center.z - s], normal: [0.0, 0.0, -1.0], uv: [0.0, 0.0], color },

            // Top face (Y+)
            RendererVertex { position: [center.x - s, center.y + s, center.z - s], normal: [0.0, 1.0, 0.0], uv: [0.0, 1.0], color },
            RendererVertex { position: [center.x - s, center.y + s, center.z + s], normal: [0.0, 1.0, 0.0], uv: [0.0, 0.0], color },
            RendererVertex { position: [center.x + s, center.y + s, center.z + s], normal: [0.0, 1.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: [center.x + s, center.y + s, center.z - s], normal: [0.0, 1.0, 0.0], uv: [1.0, 1.0], color },

            // Bottom face (Y-)
            RendererVertex { position: [center.x - s, center.y - s, center.z - s], normal: [0.0, -1.0, 0.0], uv: [0.0, 0.0], color },
            RendererVertex { position: [center.x + s, center.y - s, center.z - s], normal: [0.0, -1.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: [center.x + s, center.y - s, center.z + s], normal: [0.0, -1.0, 0.0], uv: [1.0, 1.0], color },
            RendererVertex { position: [center.x - s, center.y - s, center.z + s], normal: [0.0, -1.0, 0.0], uv: [0.0, 1.0], color },

            // Right face (X+)
            RendererVertex { position: [center.x + s, center.y - s, center.z - s], normal: [1.0, 0.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: [center.x + s, center.y + s, center.z - s], normal: [1.0, 0.0, 0.0], uv: [1.0, 1.0], color },
            RendererVertex { position: [center.x + s, center.y + s, center.z + s], normal: [1.0, 0.0, 0.0], uv: [0.0, 1.0], color },
            RendererVertex { position: [center.x + s, center.y - s, center.z + s], normal: [1.0, 0.0, 0.0], uv: [0.0, 0.0], color },

            // Left face (X-)
            RendererVertex { position: [center.x - s, center.y - s, center.z - s], normal: [-1.0, 0.0, 0.0], uv: [0.0, 0.0], color },
            RendererVertex { position: [center.x - s, center.y - s, center.z + s], normal: [-1.0, 0.0, 0.0], uv: [1.0, 0.0], color },
            RendererVertex { position: [center.x - s, center.y + s, center.z + s], normal: [-1.0, 0.0, 0.0], uv: [1.0, 1.0], color },
            RendererVertex { position: [center.x - s, center.y + s, center.z - s], normal: [-1.0, 0.0, 0.0], uv: [0.0, 1.0], color },
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0,       // Front
            4, 5, 6, 6, 7, 4,       // Back
            8, 9, 10, 10, 11, 8,    // Top
            12, 13, 14, 14, 15, 12, // Bottom
            16, 17, 18, 18, 19, 16, // Right
            20, 21, 22, 22, 23, 20, // Left
        ];

        Some((vertices, indices))
    }

    /// Generate sphere geometry (icosphere approximation)
    fn generate_sphere(&self, center: Vec3, radius: f32, color: [f32; 4], _lod: u32) -> Option<(Vec<RendererVertex>, Vec<u32>)> {
        // Simplified icosphere (12 vertices)
        let t = (1.0 + 5.0_f32.sqrt()) / 2.0;

        let positions = vec![
            Vec3::new(-1.0, t, 0.0).normalize() * radius + center,
            Vec3::new(1.0, t, 0.0).normalize() * radius + center,
            Vec3::new(-1.0, -t, 0.0).normalize() * radius + center,
            Vec3::new(1.0, -t, 0.0).normalize() * radius + center,
            Vec3::new(0.0, -1.0, t).normalize() * radius + center,
            Vec3::new(0.0, 1.0, t).normalize() * radius + center,
            Vec3::new(0.0, -1.0, -t).normalize() * radius + center,
            Vec3::new(0.0, 1.0, -t).normalize() * radius + center,
            Vec3::new(t, 0.0, -1.0).normalize() * radius + center,
            Vec3::new(t, 0.0, 1.0).normalize() * radius + center,
            Vec3::new(-t, 0.0, -1.0).normalize() * radius + center,
            Vec3::new(-t, 0.0, 1.0).normalize() * radius + center,
        ];

        let vertices: Vec<RendererVertex> = positions
            .iter()
            .map(|pos| {
                let normal = (*pos - center).normalize();
                RendererVertex {
                    position: pos.to_array(),
                    normal: normal.to_array(),
                    uv: [0.0, 0.0],
                    color,
                }
            })
            .collect();

        let indices = vec![
            0, 11, 5,  0, 5, 1,   0, 1, 7,   0, 7, 10,  0, 10, 11,
            1, 5, 9,   5, 11, 4,  11, 10, 2, 10, 7, 6,  7, 1, 8,
            3, 9, 4,   3, 4, 2,   3, 2, 6,   3, 6, 8,   3, 8, 9,
            4, 9, 5,   2, 4, 11,  6, 2, 10,  8, 6, 7,   9, 8, 1,
        ];

        Some((vertices, indices))
    }

    /// Generate cylinder geometry
    fn generate_cylinder(&self, center: Vec3, radius: f32, color: [f32; 4], lod: u32) -> Option<(Vec<RendererVertex>, Vec<u32>)> {
        let segments = match lod {
            0 => 32,
            1 => 16,
            _ => 8,
        };

        let height = radius * 2.0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices for top and bottom circles
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            // Top circle
            vertices.push(RendererVertex {
                position: [center.x + x, center.y + height / 2.0, center.z + z],
                normal: [x / radius, 0.0, z / radius],
                uv: [i as f32 / segments as f32, 1.0],
                color,
            });

            // Bottom circle
            vertices.push(RendererVertex {
                position: [center.x + x, center.y - height / 2.0, center.z + z],
                normal: [x / radius, 0.0, z / radius],
                uv: [i as f32 / segments as f32, 0.0],
                color,
            });
        }

        // Generate indices for cylinder sides
        for i in 0..segments {
            let base = i * 2;
            indices.extend_from_slice(&[
                base, base + 1, base + 2,
                base + 2, base + 1, base + 3,
            ]);
        }

        Some((vertices, indices))
    }

    /// Generate cone geometry
    fn generate_cone(&self, center: Vec3, radius: f32, color: [f32; 4], lod: u32) -> Option<(Vec<RendererVertex>, Vec<u32>)> {
        let segments = match lod {
            0 => 32,
            1 => 16,
            _ => 8,
        };

        let height = radius * 2.0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Apex vertex
        vertices.push(RendererVertex {
            position: [center.x, center.y + height, center.z],
            normal: [0.0, 1.0, 0.0],
            uv: [0.5, 1.0],
            color,
        });

        // Base circle vertices
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            vertices.push(RendererVertex {
                position: [center.x + x, center.y, center.z + z],
                normal: [x / radius, 0.0, z / radius],
                uv: [i as f32 / segments as f32, 0.0],
                color,
            });
        }

        // Generate indices
        for i in 1..=segments {
            indices.extend_from_slice(&[0, i, i + 1]);
        }

        Some((vertices, indices))
    }

    /// Create PBR material from element properties
    fn create_material(&self, element: &WorldElement) -> RendererMaterial {
        // Determine material properties based on element type
        let (metallic, roughness) = match element.element_type {
            ElementType::Landmark => (0.8, 0.2),  // Shiny landmarks
            ElementType::Structure => (0.5, 0.5), // Semi-metallic
            ElementType::Geometry => (0.0, 0.7),  // Matte
            ElementType::Ambient => (0.0, 0.9),   // Very matte
            _ => (0.2, 0.6),  // Default
        };

        // Use emission for beats and loud notes
        let emission_strength = if element.is_beat {
            element.loudness * 2.0
        } else {
            element.emission
        };

        // Convert color from [f32; 3] to [f32; 4] (add alpha)
        let base_color = [
            element.color[0],
            element.color[1],
            element.color[2],
            1.0,  // Full opacity
        ];

        RendererMaterial {
            base_color,
            metallic,
            roughness,
            emission: [element.color[0], element.color[1], element.color[2]],
            emission_strength,
        }
    }

    /// Set LOD level (0 = highest detail, 3 = lowest)
    pub fn set_lod(&mut self, level: u32) {
        self.lod_level = level.min(3);
    }

    /// Enable or disable mesh caching
    pub fn set_caching(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::ElementType;
    use crate::mapping::{ShapeHint, MaterialHint, TextureHint};

    #[test]
    fn test_cube_generation() {
        use glam::Mat4;
        let converter = MeshConverter::new(0);
        let element = WorldElement {
            id: "test".to_string(),
            element_type: ElementType::Geometry,
            transform: Mat4::IDENTITY,
            position: Vec3::ZERO,
            scale: 2.0,
            rotation: Vec3::ZERO,
            color: [1.0, 1.0, 1.0],  // RGB only
            emission: 0.0,
            shape: ShapeHint::Block,  // Block = cube
            material: MaterialHint {
                color: [1.0, 1.0, 1.0],
                metallic: 0.0,
                roughness: 0.5,
                emission: [0.0, 0.0, 0.0],
                transparency: 0.0,
                texture: TextureHint::Smooth,
            },
            timestamp: 0.0,
            is_beat: false,
            loudness: 0.5,
        };

        let result = converter.convert_element(&element);
        assert!(result.is_some());

        let (mesh, _material) = result.unwrap();
        assert_eq!(mesh.vertices.len(), 24);  // 4 vertices per face * 6 faces
        assert_eq!(mesh.indices.len(), 36);   // 6 indices per face * 6 faces
    }

    #[test]
    fn test_material_creation() {
        use glam::Mat4;
        let converter = MeshConverter::new(0);

        let element = WorldElement {
            id: "beat".to_string(),
            element_type: ElementType::Landmark,
            transform: Mat4::IDENTITY,
            position: Vec3::ZERO,
            scale: 1.0,
            rotation: Vec3::ZERO,
            color: [1.0, 0.5, 0.0],  // RGB only
            emission: 0.5,
            shape: ShapeHint::Organic,
            material: MaterialHint {
                color: [1.0, 0.5, 0.0],
                metallic: 0.2,
                roughness: 0.4,
                emission: [0.5, 0.25, 0.0],
                transparency: 0.0,
                texture: TextureHint::Smooth,
            },
            timestamp: 1.0,
            is_beat: true,
            loudness: 0.8,
        };

        let material = converter.create_material(&element);

        assert_eq!(material.base_color, [1.0, 0.5, 0.0, 1.0]);  // RGBA
        assert_eq!(material.metallic, 0.8);  // Landmarks are shiny
        assert!(material.emission_strength > 0.5);  // Beats emit light
    }

    #[test]
    fn test_lod_levels() {
        let mut converter = MeshConverter::new(0);

        // LOD 0 should use more vertices than LOD 2
        converter.set_lod(0);
        let high_detail = converter.generate_cylinder(Vec3::ZERO, 1.0, [1.0; 4], 0).unwrap();

        converter.set_lod(2);
        let low_detail = converter.generate_cylinder(Vec3::ZERO, 1.0, [1.0; 4], 2).unwrap();

        assert!(high_detail.0.len() > low_detail.0.len());
    }
}
