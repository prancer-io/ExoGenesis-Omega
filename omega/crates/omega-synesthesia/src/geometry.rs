//! Procedural Geometry Generation
//!
//! Creates meshes and shapes for world elements.

use crate::mapping::ShapeHint;
use glam::{Vec3, Vec2};

/// A procedurally generated mesh
#[derive(Debug, Clone)]
pub struct ProceduralMesh {
    /// Vertex positions
    pub positions: Vec<Vec3>,
    /// Vertex normals
    pub normals: Vec<Vec3>,
    /// Texture coordinates
    pub uvs: Vec<Vec2>,
    /// Triangle indices
    pub indices: Vec<u32>,
    /// Vertex colors (optional)
    pub colors: Option<Vec<[f32; 4]>>,
}

impl ProceduralMesh {
    /// Create empty mesh
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            colors: None,
        }
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    /// Get triangle count
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Calculate bounding box
    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        if self.positions.is_empty() {
            return (Vec3::ZERO, Vec3::ZERO);
        }

        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        for pos in &self.positions {
            min = min.min(*pos);
            max = max.max(*pos);
        }

        (min, max)
    }

    /// Merge another mesh into this one
    pub fn merge(&mut self, other: &ProceduralMesh) {
        let offset = self.positions.len() as u32;

        self.positions.extend(&other.positions);
        self.normals.extend(&other.normals);
        self.uvs.extend(&other.uvs);

        for idx in &other.indices {
            self.indices.push(idx + offset);
        }

        if let (Some(ref mut colors), Some(ref other_colors)) = (&mut self.colors, &other.colors) {
            colors.extend(other_colors);
        }
    }
}

impl Default for ProceduralMesh {
    fn default() -> Self {
        Self::new()
    }
}

/// Primitive shape types
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Cube,
    Sphere { subdivisions: u32 },
    Cylinder { segments: u32 },
    Cone { segments: u32 },
    Torus { major_segments: u32, minor_segments: u32 },
    Icosahedron,
    Pyramid,
}

/// Mesh generator for creating procedural geometry
pub struct MeshGenerator {
    /// Default subdivision level
    pub default_subdivisions: u32,
}

impl MeshGenerator {
    /// Create a new mesh generator
    pub fn new() -> Self {
        Self {
            default_subdivisions: 16,
        }
    }

    /// Generate mesh for a shape hint
    pub fn generate_for_hint(&self, hint: ShapeHint, scale: f32) -> ProceduralMesh {
        match hint {
            ShapeHint::Organic => self.generate_organic(scale),
            ShapeHint::Crystalline => self.generate_crystalline(scale),
            ShapeHint::Flowing => self.generate_flowing(scale),
            ShapeHint::Particles => self.generate_particle_points(scale),
            ShapeHint::Block => self.generate_cube(scale),
            ShapeHint::Spire => self.generate_spire(scale),
            ShapeHint::Dome => self.generate_dome(scale),
            ShapeHint::Wave => self.generate_wave(scale),
        }
    }

    /// Generate a cube
    pub fn generate_cube(&self, size: f32) -> ProceduralMesh {
        let half = size * 0.5;

        let positions = vec![
            // Front face
            Vec3::new(-half, -half, half),
            Vec3::new(half, -half, half),
            Vec3::new(half, half, half),
            Vec3::new(-half, half, half),
            // Back face
            Vec3::new(half, -half, -half),
            Vec3::new(-half, -half, -half),
            Vec3::new(-half, half, -half),
            Vec3::new(half, half, -half),
            // Top face
            Vec3::new(-half, half, half),
            Vec3::new(half, half, half),
            Vec3::new(half, half, -half),
            Vec3::new(-half, half, -half),
            // Bottom face
            Vec3::new(-half, -half, -half),
            Vec3::new(half, -half, -half),
            Vec3::new(half, -half, half),
            Vec3::new(-half, -half, half),
            // Right face
            Vec3::new(half, -half, half),
            Vec3::new(half, -half, -half),
            Vec3::new(half, half, -half),
            Vec3::new(half, half, half),
            // Left face
            Vec3::new(-half, -half, -half),
            Vec3::new(-half, -half, half),
            Vec3::new(-half, half, half),
            Vec3::new(-half, half, -half),
        ];

        let normals = vec![
            // Front
            Vec3::Z, Vec3::Z, Vec3::Z, Vec3::Z,
            // Back
            Vec3::NEG_Z, Vec3::NEG_Z, Vec3::NEG_Z, Vec3::NEG_Z,
            // Top
            Vec3::Y, Vec3::Y, Vec3::Y, Vec3::Y,
            // Bottom
            Vec3::NEG_Y, Vec3::NEG_Y, Vec3::NEG_Y, Vec3::NEG_Y,
            // Right
            Vec3::X, Vec3::X, Vec3::X, Vec3::X,
            // Left
            Vec3::NEG_X, Vec3::NEG_X, Vec3::NEG_X, Vec3::NEG_X,
        ];

        let uvs = vec![
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0,       // Front
            4, 5, 6, 6, 7, 4,       // Back
            8, 9, 10, 10, 11, 8,    // Top
            12, 13, 14, 14, 15, 12, // Bottom
            16, 17, 18, 18, 19, 16, // Right
            20, 21, 22, 22, 23, 20, // Left
        ];

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }

    /// Generate a sphere
    pub fn generate_sphere(&self, radius: f32, segments: u32) -> ProceduralMesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let stacks = segments;
        let slices = segments * 2;

        // Generate vertices
        for i in 0..=stacks {
            let v = i as f32 / stacks as f32;
            let phi = v * std::f32::consts::PI;

            for j in 0..=slices {
                let u = j as f32 / slices as f32;
                let theta = u * std::f32::consts::PI * 2.0;

                let x = phi.sin() * theta.cos();
                let y = phi.cos();
                let z = phi.sin() * theta.sin();

                let normal = Vec3::new(x, y, z);
                positions.push(normal * radius);
                normals.push(normal);
                uvs.push(Vec2::new(u, v));
            }
        }

        // Generate indices
        for i in 0..stacks {
            for j in 0..slices {
                let a = i * (slices + 1) + j;
                let b = a + slices + 1;

                indices.push(a);
                indices.push(b);
                indices.push(a + 1);

                indices.push(b);
                indices.push(b + 1);
                indices.push(a + 1);
            }
        }

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }

    /// Generate organic (smooth) shape
    fn generate_organic(&self, scale: f32) -> ProceduralMesh {
        // Create a smooth sphere with some noise displacement
        let mut mesh = self.generate_sphere(scale * 0.5, 16);

        // Add organic variation
        for (pos, normal) in mesh.positions.iter_mut().zip(mesh.normals.iter()) {
            let noise = (pos.x * 3.0).sin() * (pos.y * 3.0).cos() * (pos.z * 3.0).sin() * 0.1;
            *pos += *normal * noise * scale;
        }

        // Recalculate normals would go here in production
        mesh
    }

    /// Generate crystalline (angular) shape
    fn generate_crystalline(&self, scale: f32) -> ProceduralMesh {
        // Icosahedron for crystalline look
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let a = scale * 0.5;
        let b = a / phi;

        let positions = vec![
            Vec3::new(0.0, b, -a),
            Vec3::new(b, a, 0.0),
            Vec3::new(-b, a, 0.0),
            Vec3::new(0.0, b, a),
            Vec3::new(0.0, -b, a),
            Vec3::new(-a, 0.0, b),
            Vec3::new(0.0, -b, -a),
            Vec3::new(a, 0.0, -b),
            Vec3::new(a, 0.0, b),
            Vec3::new(-a, 0.0, -b),
            Vec3::new(b, -a, 0.0),
            Vec3::new(-b, -a, 0.0),
        ];

        let indices = vec![
            2, 1, 0,   1, 2, 3,   5, 4, 3,   4, 8, 3,
            7, 6, 0,   6, 9, 0,   11, 10, 4,  10, 11, 6,
            9, 5, 2,   5, 9, 11,  8, 7, 1,   7, 8, 10,
            2, 5, 3,   8, 1, 3,   9, 2, 0,   1, 7, 0,
            11, 9, 6,  7, 10, 6,  5, 11, 4,  10, 8, 4,
        ];

        // Calculate flat normals
        let mut normals = vec![Vec3::ZERO; positions.len()];
        for i in (0..indices.len()).step_by(3) {
            let i0 = indices[i] as usize;
            let i1 = indices[i + 1] as usize;
            let i2 = indices[i + 2] as usize;

            let v0 = positions[i0];
            let v1 = positions[i1];
            let v2 = positions[i2];

            let normal = (v1 - v0).cross(v2 - v0).normalize();
            normals[i0] += normal;
            normals[i1] += normal;
            normals[i2] += normal;
        }

        for normal in &mut normals {
            *normal = normal.normalize();
        }

        let uvs = positions.iter().map(|p| {
            Vec2::new(
                (p.x / scale + 0.5).clamp(0.0, 1.0),
                (p.y / scale + 0.5).clamp(0.0, 1.0),
            )
        }).collect();

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }

    /// Generate flowing (ribbon-like) shape
    fn generate_flowing(&self, scale: f32) -> ProceduralMesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let segments = 32;
        let width = scale * 0.3;

        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = t * std::f32::consts::PI * 2.0;

            // Spiral path
            let x = angle.cos() * scale * 0.5;
            let y = (t - 0.5) * scale;
            let z = angle.sin() * scale * 0.5;

            let center = Vec3::new(x, y, z);

            // Create ribbon width
            let tangent = Vec3::new(-angle.sin(), 1.0 / segments as f32, angle.cos()).normalize();
            let right = tangent.cross(Vec3::Y).normalize() * width;

            positions.push(center - right);
            positions.push(center + right);

            normals.push(Vec3::Y);
            normals.push(Vec3::Y);

            uvs.push(Vec2::new(0.0, t));
            uvs.push(Vec2::new(1.0, t));

            if i < segments {
                let base = (i * 2) as u32;
                indices.extend_from_slice(&[
                    base, base + 2, base + 1,
                    base + 1, base + 2, base + 3,
                ]);
            }
        }

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }

    /// Generate particle points (for particle systems)
    fn generate_particle_points(&self, scale: f32) -> ProceduralMesh {
        // Generate point cloud represented as tiny quads
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let num_particles = 100;
        let point_size = scale * 0.02;

        for i in 0..num_particles {
            // Random position within sphere
            let theta = (i as f32 / num_particles as f32) * std::f32::consts::PI * 2.0 * 5.0;
            let phi = (i as f32 / num_particles as f32) * std::f32::consts::PI;
            let r = (i as f32 / num_particles as f32) * scale * 0.5;

            let center = Vec3::new(
                r * phi.sin() * theta.cos(),
                r * phi.cos(),
                r * phi.sin() * theta.sin(),
            );

            // Create billboard quad
            let base = (i * 4) as u32;
            positions.extend_from_slice(&[
                center + Vec3::new(-point_size, -point_size, 0.0),
                center + Vec3::new(point_size, -point_size, 0.0),
                center + Vec3::new(point_size, point_size, 0.0),
                center + Vec3::new(-point_size, point_size, 0.0),
            ]);

            normals.extend_from_slice(&[Vec3::Z; 4]);
            uvs.extend_from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ]);

            indices.extend_from_slice(&[
                base, base + 1, base + 2,
                base, base + 2, base + 3,
            ]);
        }

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }

    /// Generate spire/tower shape
    fn generate_spire(&self, scale: f32) -> ProceduralMesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let segments = 8;
        let height = scale * 2.0;
        let base_radius = scale * 0.3;

        // Generate cone-like spire
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = t * std::f32::consts::PI * 2.0;

            // Taper radius with height
            let layers = 4;
            for j in 0..=layers {
                let h = j as f32 / layers as f32;
                let radius = base_radius * (1.0 - h * 0.9);
                let y = h * height;

                positions.push(Vec3::new(
                    angle.cos() * radius,
                    y,
                    angle.sin() * radius,
                ));

                normals.push(Vec3::new(angle.cos(), 0.3, angle.sin()).normalize());
                uvs.push(Vec2::new(t, h));
            }
        }

        // Generate indices
        let layers = 4;
        for i in 0..segments {
            for j in 0..layers {
                let a = i * (layers + 1) + j;
                let b = (i + 1) * (layers + 1) + j;

                indices.extend_from_slice(&[
                    a as u32, b as u32, (a + 1) as u32,
                    (a + 1) as u32, b as u32, (b + 1) as u32,
                ]);
            }
        }

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }

    /// Generate dome shape
    fn generate_dome(&self, scale: f32) -> ProceduralMesh {
        // Half sphere
        let mesh = self.generate_sphere(scale * 0.5, 12);

        // Keep only top half
        let filtered_positions: Vec<_> = mesh.positions.iter()
            .enumerate()
            .filter(|(_, p)| p.y >= -0.01)
            .map(|(i, p)| (i, *p))
            .collect();

        // Rebuild mesh with only top half
        let mut new_positions = Vec::new();
        let mut new_normals = Vec::new();
        let mut new_uvs = Vec::new();
        let mut index_map = std::collections::HashMap::new();

        for (old_idx, pos) in filtered_positions {
            index_map.insert(old_idx, new_positions.len());
            new_positions.push(pos);
            new_normals.push(mesh.normals[old_idx]);
            new_uvs.push(mesh.uvs[old_idx]);
        }

        let mut new_indices = Vec::new();
        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            if let (Some(&ni0), Some(&ni1), Some(&ni2)) = (
                index_map.get(&i0),
                index_map.get(&i1),
                index_map.get(&i2),
            ) {
                new_indices.push(ni0 as u32);
                new_indices.push(ni1 as u32);
                new_indices.push(ni2 as u32);
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

    /// Generate wave shape
    fn generate_wave(&self, scale: f32) -> ProceduralMesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let segments_x = 16;
        let segments_z = 16;

        for i in 0..=segments_x {
            let x = (i as f32 / segments_x as f32 - 0.5) * scale;
            let u = i as f32 / segments_x as f32;

            for j in 0..=segments_z {
                let z = (j as f32 / segments_z as f32 - 0.5) * scale;
                let v = j as f32 / segments_z as f32;

                // Wave function
                let y = ((x * 3.0).sin() + (z * 3.0).sin()) * scale * 0.1;

                positions.push(Vec3::new(x, y, z));

                // Approximate normal from wave gradient
                let dx = (x * 3.0).cos() * 3.0 * scale * 0.1;
                let dz = (z * 3.0).cos() * 3.0 * scale * 0.1;
                normals.push(Vec3::new(-dx, 1.0, -dz).normalize());

                uvs.push(Vec2::new(u, v));
            }
        }

        for i in 0..segments_x {
            for j in 0..segments_z {
                let a = i * (segments_z + 1) + j;
                let b = (i + 1) * (segments_z + 1) + j;

                indices.extend_from_slice(&[
                    a as u32, b as u32, (a + 1) as u32,
                    (a + 1) as u32, b as u32, (b + 1) as u32,
                ]);
            }
        }

        ProceduralMesh {
            positions,
            normals,
            uvs,
            indices,
            colors: None,
        }
    }
}

impl Default for MeshGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_generation() {
        let gen = MeshGenerator::new();
        let cube = gen.generate_cube(1.0);
        assert_eq!(cube.vertex_count(), 24);
        assert_eq!(cube.triangle_count(), 12);
    }

    #[test]
    fn test_sphere_generation() {
        let gen = MeshGenerator::new();
        let sphere = gen.generate_sphere(1.0, 8);
        assert!(sphere.vertex_count() > 0);
        assert!(sphere.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_merge() {
        let gen = MeshGenerator::new();
        let mut mesh1 = gen.generate_cube(1.0);
        let mesh2 = gen.generate_cube(1.0);

        let original_verts = mesh1.vertex_count();
        mesh1.merge(&mesh2);

        assert_eq!(mesh1.vertex_count(), original_verts * 2);
    }
}
