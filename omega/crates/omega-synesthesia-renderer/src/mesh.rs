//! Mesh and vertex definitions

use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec2};

/// Vertex format for synesthesia meshes
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    /// Position in 3D space
    pub position: [f32; 3],

    /// Normal vector
    pub normal: [f32; 3],

    /// Texture coordinates
    pub uv: [f32; 2],

    /// Vertex color (RGBA)
    pub color: [f32; 4],
}

impl Vertex {
    /// Create a new vertex
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2, color: [f32; 4]) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            uv: uv.to_array(),
            color,
        }
    }

    /// Get vertex buffer layout for wgpu
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // UV
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// 3D mesh with vertices and indices
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub name: String,
}

impl Mesh {
    /// Create a new mesh
    pub fn new(name: String, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            name,
        }
    }

    /// Create a cube mesh
    pub fn cube(size: f32, color: [f32; 4]) -> Self {
        let s = size / 2.0;

        let vertices = vec![
            // Front face
            Vertex::new(Vec3::new(-s, -s, s), Vec3::Z, Vec2::new(0.0, 0.0), color),
            Vertex::new(Vec3::new(s, -s, s), Vec3::Z, Vec2::new(1.0, 0.0), color),
            Vertex::new(Vec3::new(s, s, s), Vec3::Z, Vec2::new(1.0, 1.0), color),
            Vertex::new(Vec3::new(-s, s, s), Vec3::Z, Vec2::new(0.0, 1.0), color),
            // Back face
            Vertex::new(Vec3::new(-s, -s, -s), Vec3::NEG_Z, Vec2::new(1.0, 0.0), color),
            Vertex::new(Vec3::new(-s, s, -s), Vec3::NEG_Z, Vec2::new(1.0, 1.0), color),
            Vertex::new(Vec3::new(s, s, -s), Vec3::NEG_Z, Vec2::new(0.0, 1.0), color),
            Vertex::new(Vec3::new(s, -s, -s), Vec3::NEG_Z, Vec2::new(0.0, 0.0), color),
            // Top face
            Vertex::new(Vec3::new(-s, s, -s), Vec3::Y, Vec2::new(0.0, 1.0), color),
            Vertex::new(Vec3::new(-s, s, s), Vec3::Y, Vec2::new(0.0, 0.0), color),
            Vertex::new(Vec3::new(s, s, s), Vec3::Y, Vec2::new(1.0, 0.0), color),
            Vertex::new(Vec3::new(s, s, -s), Vec3::Y, Vec2::new(1.0, 1.0), color),
            // Bottom face
            Vertex::new(Vec3::new(-s, -s, -s), Vec3::NEG_Y, Vec2::new(0.0, 0.0), color),
            Vertex::new(Vec3::new(s, -s, -s), Vec3::NEG_Y, Vec2::new(1.0, 0.0), color),
            Vertex::new(Vec3::new(s, -s, s), Vec3::NEG_Y, Vec2::new(1.0, 1.0), color),
            Vertex::new(Vec3::new(-s, -s, s), Vec3::NEG_Y, Vec2::new(0.0, 1.0), color),
            // Right face
            Vertex::new(Vec3::new(s, -s, -s), Vec3::X, Vec2::new(1.0, 0.0), color),
            Vertex::new(Vec3::new(s, s, -s), Vec3::X, Vec2::new(1.0, 1.0), color),
            Vertex::new(Vec3::new(s, s, s), Vec3::X, Vec2::new(0.0, 1.0), color),
            Vertex::new(Vec3::new(s, -s, s), Vec3::X, Vec2::new(0.0, 0.0), color),
            // Left face
            Vertex::new(Vec3::new(-s, -s, -s), Vec3::NEG_X, Vec2::new(0.0, 0.0), color),
            Vertex::new(Vec3::new(-s, -s, s), Vec3::NEG_X, Vec2::new(1.0, 0.0), color),
            Vertex::new(Vec3::new(-s, s, s), Vec3::NEG_X, Vec2::new(1.0, 1.0), color),
            Vertex::new(Vec3::new(-s, s, -s), Vec3::NEG_X, Vec2::new(0.0, 1.0), color),
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0,       // Front
            4, 5, 6, 6, 7, 4,       // Back
            8, 9, 10, 10, 11, 8,    // Top
            12, 13, 14, 14, 15, 12, // Bottom
            16, 17, 18, 18, 19, 16, // Right
            20, 21, 22, 22, 23, 20, // Left
        ];

        Self::new("cube".to_string(), vertices, indices)
    }

    /// Create a sphere mesh (icosphere subdivision)
    pub fn sphere(radius: f32, _subdivisions: u32, color: [f32; 4]) -> Self {
        // Start with icosahedron
        let t = (1.0 + 5.0_f32.sqrt()) / 2.0;

        let mut vertices = vec![
            Vertex::new(Vec3::new(-1.0, t, 0.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(1.0, t, 0.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(-1.0, -t, 0.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(1.0, -t, 0.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(0.0, -1.0, t).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(0.0, 1.0, t).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(0.0, -1.0, -t).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(0.0, 1.0, -t).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(t, 0.0, -1.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(t, 0.0, 1.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(-t, 0.0, -1.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
            Vertex::new(Vec3::new(-t, 0.0, 1.0).normalize() * radius, Vec3::ZERO, Vec2::ZERO, color),
        ];

        let indices = vec![
            0, 11, 5,  0, 5, 1,   0, 1, 7,   0, 7, 10,  0, 10, 11,
            1, 5, 9,   5, 11, 4,  11, 10, 2, 10, 7, 6,  7, 1, 8,
            3, 9, 4,   3, 4, 2,   3, 2, 6,   3, 6, 8,   3, 8, 9,
            4, 9, 5,   2, 4, 11,  6, 2, 10,  8, 6, 7,   9, 8, 1,
        ];

        // Subdivide (simplified - just return icosahedron for now)
        // Full subdivision would recursively split triangles

        // Calculate normals (for sphere, normal = normalized position)
        for vertex in &mut vertices {
            let pos = Vec3::from_array(vertex.position);
            vertex.normal = pos.normalize().to_array();
        }

        Self::new("sphere".to_string(), vertices, indices)
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get triangle count
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_creation() {
        let cube = Mesh::cube(2.0, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(cube.vertex_count(), 24);  // 4 vertices per face * 6 faces
        assert_eq!(cube.triangle_count(), 12);  // 2 triangles per face * 6 faces
    }

    #[test]
    fn test_sphere_creation() {
        let sphere = Mesh::sphere(1.0, 0, [1.0, 0.0, 0.0, 1.0]);
        assert!(sphere.vertex_count() > 0);
        assert!(sphere.triangle_count() > 0);
    }

    #[test]
    fn test_vertex_descriptor() {
        let desc = Vertex::desc();
        assert_eq!(desc.array_stride, std::mem::size_of::<Vertex>() as u64);
        assert_eq!(desc.attributes.len(), 4);  // position, normal, uv, color
    }
}
