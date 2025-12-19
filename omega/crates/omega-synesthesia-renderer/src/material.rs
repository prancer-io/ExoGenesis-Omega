//! PBR material system

use bytemuck::{Pod, Zeroable};

/// Material trait for different material types
pub trait Material {
    /// Get material uniforms for shader
    fn get_uniforms(&self) -> MaterialUniforms;
}

/// PBR material with metallic-roughness workflow
#[derive(Debug, Clone, Copy)]
pub struct PbrMaterial {
    /// Base color (albedo)
    pub base_color: [f32; 4],

    /// Metallic factor (0.0 = dielectric, 1.0 = metal)
    pub metallic: f32,

    /// Roughness factor (0.0 = smooth, 1.0 = rough)
    pub roughness: f32,

    /// Emission color (for glowing objects)
    pub emission: [f32; 3],

    /// Emission strength
    pub emission_strength: f32,

    /// Normal map strength
    pub normal_strength: f32,

    /// Ambient occlusion strength
    pub ao_strength: f32,
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self {
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            emission: [0.0, 0.0, 0.0],
            emission_strength: 0.0,
            normal_strength: 1.0,
            ao_strength: 1.0,
        }
    }
}

impl PbrMaterial {
    /// Create a matte material
    pub fn matte(color: [f32; 3]) -> Self {
        Self {
            base_color: [color[0], color[1], color[2], 1.0],
            metallic: 0.0,
            roughness: 0.9,
            ..Default::default()
        }
    }

    /// Create a metallic material
    pub fn metallic(color: [f32; 3], roughness: f32) -> Self {
        Self {
            base_color: [color[0], color[1], color[2], 1.0],
            metallic: 1.0,
            roughness,
            ..Default::default()
        }
    }

    /// Create a glossy material
    pub fn glossy(color: [f32; 3]) -> Self {
        Self {
            base_color: [color[0], color[1], color[2], 1.0],
            metallic: 0.0,
            roughness: 0.2,
            ..Default::default()
        }
    }

    /// Create an emissive material
    pub fn emissive(color: [f32; 3], strength: f32) -> Self {
        Self {
            base_color: [color[0], color[1], color[2], 1.0],
            emission: color,
            emission_strength: strength,
            ..Default::default()
        }
    }
}

impl Material for PbrMaterial {
    fn get_uniforms(&self) -> MaterialUniforms {
        MaterialUniforms {
            base_color: self.base_color,
            metallic: self.metallic,
            roughness: self.roughness,
            emission: [
                self.emission[0],
                self.emission[1],
                self.emission[2],
                self.emission_strength,
            ],
            normal_strength: self.normal_strength,
            ao_strength: self.ao_strength,
            _padding: [0.0; 2],
        }
    }
}

/// Material uniforms for GPU (must match shader layout)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct MaterialUniforms {
    /// Base color (RGBA)
    pub base_color: [f32; 4],

    /// Metallic factor
    pub metallic: f32,

    /// Roughness factor
    pub roughness: f32,

    /// Normal map strength
    pub normal_strength: f32,

    /// AO strength
    pub ao_strength: f32,

    /// Emission color + strength
    pub emission: [f32; 4],

    /// Padding for alignment
    pub _padding: [f32; 2],
}

/// Preset materials for common use cases
pub struct MaterialPresets;

impl MaterialPresets {
    /// Marble material (Classical music)
    pub fn marble() -> PbrMaterial {
        PbrMaterial {
            base_color: [0.95, 0.95, 0.98, 1.0],
            metallic: 0.0,
            roughness: 0.3,
            ..Default::default()
        }
    }

    /// Wood material (Jazz)
    pub fn wood() -> PbrMaterial {
        PbrMaterial {
            base_color: [0.4, 0.25, 0.15, 1.0],
            metallic: 0.0,
            roughness: 0.7,
            ..Default::default()
        }
    }

    /// Volcanic rock (Rock music)
    pub fn volcanic_rock() -> PbrMaterial {
        PbrMaterial {
            base_color: [0.2, 0.15, 0.1, 1.0],
            metallic: 0.0,
            roughness: 0.95,
            emission: [0.8, 0.3, 0.1],
            emission_strength: 0.3,
            ..Default::default()
        }
    }

    /// Neon metal (Electronic)
    pub fn neon_metal() -> PbrMaterial {
        PbrMaterial {
            base_color: [0.1, 0.3, 0.8, 1.0],
            metallic: 0.9,
            roughness: 0.1,
            emission: [0.0, 0.5, 1.0],
            emission_strength: 0.5,
            ..Default::default()
        }
    }

    /// Ethereal fog (Ambient)
    pub fn ethereal() -> PbrMaterial {
        PbrMaterial {
            base_color: [0.8, 0.9, 1.0, 0.3],
            metallic: 0.0,
            roughness: 0.1,
            emission: [0.7, 0.8, 0.9],
            emission_strength: 0.2,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_material() {
        let material = PbrMaterial::default();
        assert_eq!(material.base_color, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(material.metallic, 0.0);
        assert_eq!(material.roughness, 0.5);
    }

    #[test]
    fn test_material_presets() {
        let marble = MaterialPresets::marble();
        assert!(marble.roughness < 0.5);

        let metal = MaterialPresets::neon_metal();
        assert!(metal.metallic > 0.5);

        let emissive = MaterialPresets::volcanic_rock();
        assert!(emissive.emission_strength > 0.0);
    }

    #[test]
    fn test_material_uniforms() {
        let material = PbrMaterial::default();
        let uniforms = material.get_uniforms();
        assert_eq!(uniforms.base_color, material.base_color);
        assert_eq!(uniforms.metallic, material.metallic);
    }

    #[test]
    fn test_material_size() {
        // Ensure proper alignment for GPU
        let size = std::mem::size_of::<MaterialUniforms>();
        assert_eq!(size % 16, 0);  // Must be multiple of 16 bytes
    }
}
