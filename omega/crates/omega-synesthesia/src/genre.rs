//! Genre-Specific Styles
//!
//! Defines visual styles for different musical genres.

use crate::mapping::{ShapeHint, TextureHint};
use serde::{Deserialize, Serialize};

/// Musical genre categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genre {
    /// Classical, orchestral, symphony
    Classical,
    /// Jazz, blues, swing
    Jazz,
    /// Rock, alternative, indie
    Rock,
    /// Electronic, EDM, techno, house
    Electronic,
    /// Metal, death metal, black metal
    Metal,
    /// Ambient, drone, atmospheric
    Ambient,
    /// Hip-hop, rap, trap
    HipHop,
    /// Folk, acoustic, country
    Folk,
    /// Pop, mainstream
    Pop,
    /// Experimental, avant-garde
    Experimental,
}

impl Genre {
    /// Get the visual style for this genre
    pub fn get_style(&self) -> GenreStyle {
        match self {
            Genre::Classical => GenreStyle::classical(),
            Genre::Jazz => GenreStyle::jazz(),
            Genre::Rock => GenreStyle::rock(),
            Genre::Electronic => GenreStyle::electronic(),
            Genre::Metal => GenreStyle::metal(),
            Genre::Ambient => GenreStyle::ambient(),
            Genre::HipHop => GenreStyle::hip_hop(),
            Genre::Folk => GenreStyle::folk(),
            Genre::Pop => GenreStyle::pop(),
            Genre::Experimental => GenreStyle::experimental(),
        }
    }
}

/// Visual style configuration for a genre
#[derive(Debug, Clone)]
pub struct GenreStyle {
    /// Name of the style
    pub name: String,

    // === Colors ===
    /// Primary color (RGB)
    pub primary_color: [f32; 3],
    /// Secondary color
    pub secondary_color: [f32; 3],
    /// Accent color
    pub accent_color: [f32; 3],
    /// Background color / fog color
    pub background_color: [f32; 3],

    // === Shapes ===
    /// Primary shape type
    pub primary_shape: ShapeHint,
    /// Shape used for beats
    pub beat_shape: ShapeHint,
    /// Overall geometric style (0 = organic, 1 = geometric)
    pub geometric_factor: f32,

    // === Materials ===
    /// Base metallic value
    pub base_metallic: f32,
    /// Base roughness value
    pub base_roughness: f32,
    /// Base transparency
    pub base_transparency: f32,
    /// Primary texture type
    pub primary_texture: TextureHint,
    /// Emission intensity multiplier
    pub emission_intensity: f32,

    // === Scale & Density ===
    /// Overall scale multiplier
    pub scale_multiplier: f32,
    /// Element density (elements per unit)
    pub density: f32,
    /// Vertical exaggeration
    pub vertical_scale: f32,

    // === Effects ===
    /// Fog enabled
    pub fog_enabled: bool,
    /// Fog density
    pub fog_density: f32,
    /// Fog color
    pub fog_color: [f32; 3],
    /// Particles enabled
    pub particles_enabled: bool,
    /// Particle intensity
    pub particle_intensity: f32,
    /// Particle color
    pub particle_color: [f32; 3],
    /// Bloom intensity
    pub bloom_intensity: f32,
    /// Motion blur
    pub motion_blur: f32,

    // === Lighting ===
    /// Ambient light color
    pub ambient_color: [f32; 3],
    /// Ambient light intensity
    pub ambient_intensity: f32,
    /// Sun intensity
    pub sun_intensity: f32,
    /// Sky color at top
    pub sky_color_top: [f32; 3],
    /// Sky color at horizon
    pub sky_color_horizon: [f32; 3],
    /// Cloud coverage (0-1)
    pub cloud_coverage: f32,

    // === Mapping ===
    /// Time scale (units per second)
    pub time_scale: f32,

    // === Architecture ===
    /// Architectural style description
    pub architecture: ArchitectureStyle,
}

/// Architectural style
#[derive(Debug, Clone)]
pub struct ArchitectureStyle {
    /// Primary structure type
    pub structure: StructureType,
    /// Symmetry type
    pub symmetry: SymmetryType,
    /// Complexity level (0-1)
    pub complexity: f32,
    /// Organic vs mechanical (0 = organic, 1 = mechanical)
    pub organic_factor: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum StructureType {
    Cathedral,
    Tower,
    Bridge,
    Cave,
    Forest,
    City,
    Abstract,
    Void,
    Ocean,
    Mountain,
}

#[derive(Debug, Clone, Copy)]
pub enum SymmetryType {
    None,
    Bilateral,
    Radial,
    Fractal,
    Spiral,
}

/// Style presets for quick configuration
#[derive(Debug, Clone, Copy)]
pub enum StylePreset {
    Photorealistic,
    Stylized,
    Abstract,
    Minimalist,
    Psychedelic,
    Dark,
    Light,
    Neon,
}

impl GenreStyle {
    /// Classical music style - Gothic cathedrals, gold, marble
    pub fn classical() -> Self {
        Self {
            name: "Classical".to_string(),
            primary_color: [0.85, 0.75, 0.55],     // Gold
            secondary_color: [0.9, 0.9, 0.95],     // Marble white
            accent_color: [0.6, 0.2, 0.2],         // Deep red
            background_color: [0.1, 0.08, 0.12],   // Dark purple

            primary_shape: ShapeHint::Dome,
            beat_shape: ShapeHint::Spire,
            geometric_factor: 0.7,

            base_metallic: 0.8,
            base_roughness: 0.2,
            base_transparency: 0.0,
            primary_texture: TextureHint::Marble,
            emission_intensity: 0.3,

            scale_multiplier: 1.5,
            density: 0.6,
            vertical_scale: 2.0,

            fog_enabled: true,
            fog_density: 0.02,
            fog_color: [0.1, 0.08, 0.12],
            particles_enabled: true,
            particle_intensity: 0.3,
            particle_color: [0.85, 0.75, 0.55],
            bloom_intensity: 0.4,
            motion_blur: 0.1,

            ambient_color: [0.1, 0.08, 0.12],
            ambient_intensity: 0.3,
            sun_intensity: 0.8,
            sky_color_top: [0.1, 0.1, 0.2],
            sky_color_horizon: [0.3, 0.2, 0.4],
            cloud_coverage: 0.2,

            time_scale: 10.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Cathedral,
                symmetry: SymmetryType::Bilateral,
                complexity: 0.8,
                organic_factor: 0.2,
            },
        }
    }

    /// Jazz style - Smoky clubs, neon, fluid shapes
    pub fn jazz() -> Self {
        Self {
            name: "Jazz".to_string(),
            primary_color: [0.2, 0.3, 0.6],        // Smoky blue
            secondary_color: [0.8, 0.6, 0.2],      // Brass gold
            accent_color: [0.9, 0.2, 0.4],         // Neon pink
            background_color: [0.05, 0.03, 0.08],  // Deep night

            primary_shape: ShapeHint::Flowing,
            beat_shape: ShapeHint::Organic,
            geometric_factor: 0.2,

            base_metallic: 0.6,
            base_roughness: 0.4,
            base_transparency: 0.1,
            primary_texture: TextureHint::Metal,
            emission_intensity: 0.5,

            scale_multiplier: 1.0,
            density: 0.8,
            vertical_scale: 1.2,

            fog_enabled: true,
            fog_density: 0.05,
            fog_color: [0.05, 0.03, 0.08],
            particles_enabled: true,
            particle_intensity: 0.6,
            particle_color: [0.9, 0.2, 0.4],
            bloom_intensity: 0.6,
            motion_blur: 0.3,

            ambient_color: [0.1, 0.1, 0.2],
            ambient_intensity: 0.2,
            sun_intensity: 0.3,
            sky_color_top: [0.02, 0.01, 0.05],
            sky_color_horizon: [0.1, 0.05, 0.15],
            cloud_coverage: 0.0,

            time_scale: 10.0,

            architecture: ArchitectureStyle {
                structure: StructureType::City,
                symmetry: SymmetryType::None,
                complexity: 0.6,
                organic_factor: 0.7,
            },
        }
    }

    /// Electronic/EDM style - Grids, lasers, geometric
    pub fn electronic() -> Self {
        Self {
            name: "Electronic".to_string(),
            primary_color: [0.0, 0.8, 1.0],        // Cyan
            secondary_color: [1.0, 0.0, 0.8],      // Magenta
            accent_color: [0.0, 1.0, 0.5],         // Neon green
            background_color: [0.0, 0.0, 0.05],    // Pure black

            primary_shape: ShapeHint::Block,
            beat_shape: ShapeHint::Crystalline,
            geometric_factor: 1.0,

            base_metallic: 0.3,
            base_roughness: 0.1,
            base_transparency: 0.2,
            primary_texture: TextureHint::Glass,
            emission_intensity: 1.0,

            scale_multiplier: 1.2,
            density: 1.0,
            vertical_scale: 1.5,

            fog_enabled: false,
            fog_density: 0.01,
            fog_color: [0.0, 0.0, 0.05],
            particles_enabled: true,
            particle_intensity: 0.9,
            particle_color: [0.0, 1.0, 1.0],
            bloom_intensity: 0.9,
            motion_blur: 0.0,

            ambient_color: [0.0, 0.1, 0.2],
            ambient_intensity: 0.4,
            sun_intensity: 0.0,
            sky_color_top: [0.0, 0.0, 0.02],
            sky_color_horizon: [0.0, 0.1, 0.2],
            cloud_coverage: 0.0,

            time_scale: 12.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Abstract,
                symmetry: SymmetryType::Radial,
                complexity: 0.9,
                organic_factor: 0.0,
            },
        }
    }

    /// Metal style - Jagged, dark, intense
    pub fn metal() -> Self {
        Self {
            name: "Metal".to_string(),
            primary_color: [0.1, 0.1, 0.1],        // Black
            secondary_color: [0.8, 0.0, 0.0],      // Blood red
            accent_color: [1.0, 0.5, 0.0],         // Fire orange
            background_color: [0.02, 0.0, 0.0],    // Deep crimson black

            primary_shape: ShapeHint::Crystalline,
            beat_shape: ShapeHint::Spire,
            geometric_factor: 0.8,

            base_metallic: 0.9,
            base_roughness: 0.3,
            base_transparency: 0.0,
            primary_texture: TextureHint::Metal,
            emission_intensity: 0.7,

            scale_multiplier: 1.8,
            density: 0.7,
            vertical_scale: 3.0,

            fog_enabled: true,
            fog_density: 0.03,
            fog_color: [0.05, 0.0, 0.0],
            particles_enabled: true,
            particle_intensity: 0.8,
            particle_color: [1.0, 0.3, 0.0],
            bloom_intensity: 0.5,
            motion_blur: 0.2,

            ambient_color: [0.1, 0.02, 0.02],
            ambient_intensity: 0.2,
            sun_intensity: 0.5,
            sky_color_top: [0.02, 0.0, 0.0],
            sky_color_horizon: [0.2, 0.05, 0.0],
            cloud_coverage: 0.6,

            time_scale: 8.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Mountain,
                symmetry: SymmetryType::None,
                complexity: 0.7,
                organic_factor: 0.1,
            },
        }
    }

    /// Ambient style - Ethereal, fog, soft
    pub fn ambient() -> Self {
        Self {
            name: "Ambient".to_string(),
            primary_color: [0.6, 0.7, 0.8],        // Soft blue
            secondary_color: [0.8, 0.75, 0.9],     // Lavender
            accent_color: [0.9, 0.85, 0.7],        // Warm cream
            background_color: [0.15, 0.18, 0.22],  // Soft gray

            primary_shape: ShapeHint::Organic,
            beat_shape: ShapeHint::Wave,
            geometric_factor: 0.1,

            base_metallic: 0.1,
            base_roughness: 0.8,
            base_transparency: 0.4,
            primary_texture: TextureHint::Smooth,
            emission_intensity: 0.2,

            scale_multiplier: 0.8,
            density: 0.3,
            vertical_scale: 0.8,

            fog_enabled: true,
            fog_density: 0.1,
            fog_color: [0.2, 0.22, 0.25],
            particles_enabled: true,
            particle_intensity: 0.4,
            particle_color: [0.9, 0.85, 0.7],
            bloom_intensity: 0.7,
            motion_blur: 0.5,

            ambient_color: [0.15, 0.18, 0.22],
            ambient_intensity: 0.5,
            sun_intensity: 0.3,
            sky_color_top: [0.4, 0.5, 0.7],
            sky_color_horizon: [0.7, 0.75, 0.85],
            cloud_coverage: 0.5,

            time_scale: 8.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Void,
                symmetry: SymmetryType::Spiral,
                complexity: 0.3,
                organic_factor: 1.0,
            },
        }
    }

    /// Rock style
    pub fn rock() -> Self {
        Self {
            name: "Rock".to_string(),
            primary_color: [0.3, 0.25, 0.2],
            secondary_color: [0.7, 0.5, 0.3],
            accent_color: [0.9, 0.3, 0.2],
            background_color: [0.08, 0.06, 0.05],

            primary_shape: ShapeHint::Block,
            beat_shape: ShapeHint::Crystalline,
            geometric_factor: 0.6,

            base_metallic: 0.4,
            base_roughness: 0.6,
            base_transparency: 0.0,
            primary_texture: TextureHint::Wood,
            emission_intensity: 0.4,

            scale_multiplier: 1.3,
            density: 0.7,
            vertical_scale: 1.5,

            fog_enabled: true,
            fog_density: 0.02,
            fog_color: [0.1, 0.08, 0.06],
            particles_enabled: true,
            particle_intensity: 0.5,
            particle_color: [0.9, 0.3, 0.2],
            bloom_intensity: 0.4,
            motion_blur: 0.2,

            ambient_color: [0.1, 0.08, 0.06],
            ambient_intensity: 0.3,
            sun_intensity: 0.7,
            sky_color_top: [0.15, 0.12, 0.1],
            sky_color_horizon: [0.4, 0.3, 0.2],
            cloud_coverage: 0.3,

            time_scale: 10.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Mountain,
                symmetry: SymmetryType::Bilateral,
                complexity: 0.5,
                organic_factor: 0.4,
            },
        }
    }

    /// Hip-hop style
    pub fn hip_hop() -> Self {
        Self {
            name: "Hip-Hop".to_string(),
            primary_color: [0.9, 0.7, 0.0],        // Gold
            secondary_color: [0.1, 0.1, 0.1],      // Black
            accent_color: [0.8, 0.0, 0.3],         // Purple
            background_color: [0.05, 0.02, 0.08],

            primary_shape: ShapeHint::Block,
            beat_shape: ShapeHint::Block,
            geometric_factor: 0.9,

            base_metallic: 0.95,
            base_roughness: 0.1,
            base_transparency: 0.0,
            primary_texture: TextureHint::Metal,
            emission_intensity: 0.6,

            scale_multiplier: 1.4,
            density: 0.9,
            vertical_scale: 1.8,

            fog_enabled: false,
            fog_density: 0.01,
            fog_color: [0.05, 0.02, 0.08],
            particles_enabled: true,
            particle_intensity: 0.7,
            particle_color: [0.9, 0.7, 0.0],
            bloom_intensity: 0.7,
            motion_blur: 0.1,

            ambient_color: [0.1, 0.05, 0.15],
            ambient_intensity: 0.3,
            sun_intensity: 0.0,
            sky_color_top: [0.02, 0.01, 0.05],
            sky_color_horizon: [0.1, 0.05, 0.15],
            cloud_coverage: 0.0,

            time_scale: 12.0,

            architecture: ArchitectureStyle {
                structure: StructureType::City,
                symmetry: SymmetryType::Bilateral,
                complexity: 0.7,
                organic_factor: 0.2,
            },
        }
    }

    /// Folk style
    pub fn folk() -> Self {
        Self {
            name: "Folk".to_string(),
            primary_color: [0.6, 0.45, 0.25],      // Wood brown
            secondary_color: [0.4, 0.6, 0.3],      // Forest green
            accent_color: [0.8, 0.6, 0.4],         // Warm tan
            background_color: [0.15, 0.18, 0.12],

            primary_shape: ShapeHint::Organic,
            beat_shape: ShapeHint::Dome,
            geometric_factor: 0.2,

            base_metallic: 0.1,
            base_roughness: 0.7,
            base_transparency: 0.0,
            primary_texture: TextureHint::Wood,
            emission_intensity: 0.2,

            scale_multiplier: 1.0,
            density: 0.5,
            vertical_scale: 1.0,

            fog_enabled: true,
            fog_density: 0.03,
            fog_color: [0.18, 0.2, 0.15],
            particles_enabled: false,
            particle_intensity: 0.3,
            particle_color: [0.8, 0.6, 0.4],
            bloom_intensity: 0.3,
            motion_blur: 0.2,

            ambient_color: [0.2, 0.22, 0.18],
            ambient_intensity: 0.4,
            sun_intensity: 0.9,
            sky_color_top: [0.4, 0.5, 0.7],
            sky_color_horizon: [0.7, 0.75, 0.65],
            cloud_coverage: 0.4,

            time_scale: 8.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Forest,
                symmetry: SymmetryType::None,
                complexity: 0.4,
                organic_factor: 0.9,
            },
        }
    }

    /// Pop style
    pub fn pop() -> Self {
        Self {
            name: "Pop".to_string(),
            primary_color: [1.0, 0.4, 0.6],        // Pink
            secondary_color: [0.4, 0.8, 1.0],      // Sky blue
            accent_color: [1.0, 0.9, 0.0],         // Yellow
            background_color: [0.95, 0.95, 0.98],  // Near white

            primary_shape: ShapeHint::Organic,
            beat_shape: ShapeHint::Block,
            geometric_factor: 0.5,

            base_metallic: 0.3,
            base_roughness: 0.4,
            base_transparency: 0.1,
            primary_texture: TextureHint::Smooth,
            emission_intensity: 0.5,

            scale_multiplier: 1.1,
            density: 0.8,
            vertical_scale: 1.2,

            fog_enabled: false,
            fog_density: 0.01,
            fog_color: [0.95, 0.95, 0.98],
            particles_enabled: true,
            particle_intensity: 0.6,
            particle_color: [1.0, 0.4, 0.6],
            bloom_intensity: 0.6,
            motion_blur: 0.1,

            ambient_color: [0.4, 0.4, 0.5],
            ambient_intensity: 0.6,
            sun_intensity: 1.0,
            sky_color_top: [0.5, 0.7, 1.0],
            sky_color_horizon: [0.9, 0.9, 0.95],
            cloud_coverage: 0.2,

            time_scale: 10.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Abstract,
                symmetry: SymmetryType::Radial,
                complexity: 0.5,
                organic_factor: 0.5,
            },
        }
    }

    /// Experimental style
    pub fn experimental() -> Self {
        Self {
            name: "Experimental".to_string(),
            primary_color: [0.5, 0.0, 1.0],        // Violet
            secondary_color: [0.0, 0.5, 0.5],      // Teal
            accent_color: [1.0, 1.0, 0.0],         // Yellow
            background_color: [0.0, 0.0, 0.0],     // Pure black

            primary_shape: ShapeHint::Particles,
            beat_shape: ShapeHint::Wave,
            geometric_factor: 0.5,

            base_metallic: 0.5,
            base_roughness: 0.5,
            base_transparency: 0.3,
            primary_texture: TextureHint::Energy,
            emission_intensity: 0.8,

            scale_multiplier: 1.0,
            density: 0.6,
            vertical_scale: 2.0,

            fog_enabled: true,
            fog_density: 0.02,
            fog_color: [0.0, 0.0, 0.0],
            particles_enabled: true,
            particle_intensity: 1.0,
            particle_color: [0.5, 0.0, 1.0],
            bloom_intensity: 0.8,
            motion_blur: 0.4,

            ambient_color: [0.1, 0.0, 0.2],
            ambient_intensity: 0.3,
            sun_intensity: 0.2,
            sky_color_top: [0.0, 0.0, 0.0],
            sky_color_horizon: [0.2, 0.0, 0.3],
            cloud_coverage: 0.0,

            time_scale: 10.0,

            architecture: ArchitectureStyle {
                structure: StructureType::Void,
                symmetry: SymmetryType::Fractal,
                complexity: 1.0,
                organic_factor: 0.5,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genre_style() {
        let classical = Genre::Classical.get_style();
        assert_eq!(classical.name, "Classical");
        assert!(classical.base_metallic > 0.5);
    }

    #[test]
    fn test_all_genres() {
        let genres = [
            Genre::Classical,
            Genre::Jazz,
            Genre::Rock,
            Genre::Electronic,
            Genre::Metal,
            Genre::Ambient,
            Genre::HipHop,
            Genre::Folk,
            Genre::Pop,
            Genre::Experimental,
        ];

        for genre in genres {
            let style = genre.get_style();
            assert!(!style.name.is_empty());
        }
    }
}
