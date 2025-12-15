//! Material System for Synesthesia Worlds
//!
//! Defines materials, textures, and visual properties for world elements.

use crate::mapping::TextureHint;
use crate::genre::GenreStyle;

/// Material for a world element
#[derive(Debug, Clone)]
pub struct SynMaterial {
    /// Material name/id
    pub name: String,
    /// Base color (RGB)
    pub base_color: [f32; 3],
    /// Alpha/opacity (0-1)
    pub alpha: f32,
    /// Metallic factor (0-1)
    pub metallic: f32,
    /// Roughness (0-1)
    pub roughness: f32,
    /// Emission color (RGB, values > 1 for HDR)
    pub emission: [f32; 3],
    /// Emission intensity multiplier
    pub emission_intensity: f32,
    /// Normal map strength
    pub normal_strength: f32,
    /// Index of refraction (for transparent materials)
    pub ior: f32,
    /// Texture type
    pub texture_type: TextureType,
    /// Texture scale (UV multiplier)
    pub texture_scale: f32,
    /// Double-sided rendering
    pub double_sided: bool,
}

impl SynMaterial {
    /// Create a new material with defaults
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            base_color: [0.8, 0.8, 0.8],
            alpha: 1.0,
            metallic: 0.0,
            roughness: 0.5,
            emission: [0.0, 0.0, 0.0],
            emission_intensity: 0.0,
            normal_strength: 1.0,
            ior: 1.5,
            texture_type: TextureType::Procedural(ProceduralTexture::Solid),
            texture_scale: 1.0,
            double_sided: false,
        }
    }

    /// Create material from hint
    pub fn from_hint(hint: TextureHint, color: [f32; 3]) -> Self {
        let mut mat = Self::new("from_hint");
        mat.base_color = color;

        match hint {
            TextureHint::Smooth => {
                mat.roughness = 0.1;
                mat.metallic = 0.0;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Solid);
            }
            TextureHint::Marble => {
                mat.roughness = 0.3;
                mat.metallic = 0.0;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Marble);
            }
            TextureHint::Wood => {
                mat.roughness = 0.7;
                mat.metallic = 0.0;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Wood);
            }
            TextureHint::Metal => {
                mat.roughness = 0.3;
                mat.metallic = 0.9;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::BrushedMetal);
            }
            TextureHint::Glass => {
                mat.roughness = 0.0;
                mat.metallic = 0.0;
                mat.alpha = 0.3;
                mat.ior = 1.52;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Solid);
            }
            TextureHint::Crystal => {
                mat.roughness = 0.05;
                mat.metallic = 0.2;
                mat.alpha = 0.7;
                mat.ior = 2.4;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Crystal);
            }
            TextureHint::Fabric => {
                mat.roughness = 0.9;
                mat.metallic = 0.0;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Fabric);
            }
            TextureHint::Organic => {
                mat.roughness = 0.6;
                mat.metallic = 0.0;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Organic);
            }
            TextureHint::Energy => {
                mat.roughness = 0.0;
                mat.metallic = 0.0;
                mat.emission = color;
                mat.emission_intensity = 2.0;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Energy);
            }
            TextureHint::Void => {
                mat.roughness = 0.0;
                mat.metallic = 0.0;
                mat.base_color = [0.01, 0.01, 0.02];
                mat.emission = [0.05, 0.0, 0.1];
                mat.emission_intensity = 0.5;
                mat.texture_type = TextureType::Procedural(ProceduralTexture::Noise);
            }
        }

        mat
    }

    /// Set base color
    pub fn with_color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.base_color = [r, g, b];
        self
    }

    /// Set metallic factor
    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic;
        self
    }

    /// Set roughness
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness;
        self
    }

    /// Set emission
    pub fn with_emission(mut self, r: f32, g: f32, b: f32, intensity: f32) -> Self {
        self.emission = [r, g, b];
        self.emission_intensity = intensity;
        self
    }

    /// Set transparency
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Convert to PBR parameters for export
    pub fn to_pbr_params(&self) -> PbrParams {
        PbrParams {
            base_color_factor: [
                self.base_color[0],
                self.base_color[1],
                self.base_color[2],
                self.alpha,
            ],
            metallic_factor: self.metallic,
            roughness_factor: self.roughness,
            emissive_factor: [
                self.emission[0] * self.emission_intensity,
                self.emission[1] * self.emission_intensity,
                self.emission[2] * self.emission_intensity,
            ],
            alpha_mode: if self.alpha < 1.0 { AlphaMode::Blend } else { AlphaMode::Opaque },
            alpha_cutoff: 0.5,
            double_sided: self.double_sided,
        }
    }
}

impl Default for SynMaterial {
    fn default() -> Self {
        Self::new("default")
    }
}

/// Type of texture
#[derive(Debug, Clone)]
pub enum TextureType {
    /// File-based texture
    Image(String),
    /// Procedurally generated texture
    Procedural(ProceduralTexture),
    /// No texture (solid color)
    None,
}

/// Procedural texture types
#[derive(Debug, Clone, Copy)]
pub enum ProceduralTexture {
    /// Solid color
    Solid,
    /// Perlin noise
    Noise,
    /// Marble pattern
    Marble,
    /// Wood grain
    Wood,
    /// Brushed metal
    BrushedMetal,
    /// Crystal facets
    Crystal,
    /// Woven fabric
    Fabric,
    /// Organic cellular
    Organic,
    /// Energy flow
    Energy,
    /// Gradient
    Gradient,
    /// Voronoi cells
    Voronoi,
}

/// PBR material parameters for glTF export
#[derive(Debug, Clone)]
pub struct PbrParams {
    pub base_color_factor: [f32; 4],
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub emissive_factor: [f32; 3],
    pub alpha_mode: AlphaMode,
    pub alpha_cutoff: f32,
    pub double_sided: bool,
}

/// Alpha blending mode
#[derive(Debug, Clone, Copy)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

/// Material palette for a genre
#[derive(Debug, Clone)]
pub struct MaterialPalette {
    /// Palette name
    pub name: String,
    /// Primary material
    pub primary: SynMaterial,
    /// Secondary material
    pub secondary: SynMaterial,
    /// Accent material
    pub accent: SynMaterial,
    /// Emission material (for lights/glow)
    pub emission: SynMaterial,
    /// Background/ambient material
    pub ambient: SynMaterial,
    /// Beat/landmark material
    pub beat: SynMaterial,
}

impl MaterialPalette {
    /// Create palette for a genre style
    pub fn from_style(style: &GenreStyle) -> Self {
        let primary_color = style.primary_color;
        let secondary_color = style.secondary_color;
        let accent_color = style.accent_color;

        Self {
            name: format!("{:?}_palette", style.architecture),
            primary: SynMaterial::new("primary")
                .with_color(primary_color[0], primary_color[1], primary_color[2])
                .with_metallic(style.base_metallic)
                .with_roughness(style.base_roughness),

            secondary: SynMaterial::new("secondary")
                .with_color(secondary_color[0], secondary_color[1], secondary_color[2])
                .with_metallic(style.base_metallic * 0.8)
                .with_roughness(style.base_roughness * 1.2),

            accent: SynMaterial::new("accent")
                .with_color(accent_color[0], accent_color[1], accent_color[2])
                .with_metallic(0.8)
                .with_roughness(0.2),

            emission: SynMaterial::new("emission")
                .with_color(accent_color[0], accent_color[1], accent_color[2])
                .with_emission(accent_color[0], accent_color[1], accent_color[2], style.emission_intensity),

            ambient: SynMaterial::new("ambient")
                .with_color(
                    primary_color[0] * 0.3,
                    primary_color[1] * 0.3,
                    primary_color[2] * 0.3,
                )
                .with_roughness(0.9),

            beat: SynMaterial::new("beat")
                .with_color(accent_color[0], accent_color[1], accent_color[2])
                .with_metallic(0.9)
                .with_roughness(0.1)
                .with_emission(accent_color[0], accent_color[1], accent_color[2], 1.5),
        }
    }

    /// Get material by role
    pub fn get(&self, role: MaterialRole) -> &SynMaterial {
        match role {
            MaterialRole::Primary => &self.primary,
            MaterialRole::Secondary => &self.secondary,
            MaterialRole::Accent => &self.accent,
            MaterialRole::Emission => &self.emission,
            MaterialRole::Ambient => &self.ambient,
            MaterialRole::Beat => &self.beat,
        }
    }
}

/// Role of a material in the palette
#[derive(Debug, Clone, Copy)]
pub enum MaterialRole {
    Primary,
    Secondary,
    Accent,
    Emission,
    Ambient,
    Beat,
}

/// Shader configuration for advanced rendering
#[derive(Debug, Clone)]
pub struct ShaderConfig {
    /// Vertex shader path
    pub vertex_shader: Option<String>,
    /// Fragment shader path
    pub fragment_shader: Option<String>,
    /// Custom uniforms
    pub uniforms: Vec<ShaderUniform>,
}

/// Custom shader uniform
#[derive(Debug, Clone)]
pub struct ShaderUniform {
    pub name: String,
    pub value: UniformValue,
}

/// Uniform value types
#[derive(Debug, Clone)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Int(i32),
    Texture(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let mat = SynMaterial::new("test");
        assert_eq!(mat.name, "test");
        assert_eq!(mat.alpha, 1.0);
    }

    #[test]
    fn test_material_builder() {
        let mat = SynMaterial::new("test")
            .with_color(1.0, 0.0, 0.0)
            .with_metallic(0.8)
            .with_roughness(0.2);

        assert_eq!(mat.base_color, [1.0, 0.0, 0.0]);
        assert_eq!(mat.metallic, 0.8);
        assert_eq!(mat.roughness, 0.2);
    }

    #[test]
    fn test_pbr_conversion() {
        let mat = SynMaterial::new("test")
            .with_color(1.0, 0.5, 0.25)
            .with_alpha(0.8);

        let pbr = mat.to_pbr_params();
        assert_eq!(pbr.base_color_factor[3], 0.8);
    }

    #[test]
    fn test_from_hint() {
        let mat = SynMaterial::from_hint(TextureHint::Metal, [0.8, 0.8, 0.8]);
        assert!(mat.metallic > 0.5);
    }
}
