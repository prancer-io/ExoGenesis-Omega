//! Procedural Texture Generation
//!
//! Creates textures based on musical features and genre styles.

use crate::features::EmotionalValence;

/// Procedural texture generator
pub struct TextureGenerator {
    /// Texture resolution
    pub resolution: u32,
    /// Seed for random variation
    pub seed: u64,
}

impl TextureGenerator {
    pub fn new(resolution: u32) -> Self {
        Self {
            resolution,
            seed: 42,
        }
    }

    /// Generate noise texture
    pub fn generate_noise(&self, octaves: u32, persistence: f32) -> TextureData {
        let size = (self.resolution * self.resolution) as usize;
        let mut data = vec![0u8; size * 4];

        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let nx = x as f32 / self.resolution as f32;
                let ny = y as f32 / self.resolution as f32;

                let value = self.fbm(nx * 4.0, ny * 4.0, octaves, persistence);
                let byte = ((value * 0.5 + 0.5) * 255.0) as u8;

                let idx = ((y * self.resolution + x) * 4) as usize;
                data[idx] = byte;
                data[idx + 1] = byte;
                data[idx + 2] = byte;
                data[idx + 3] = 255;
            }
        }

        TextureData {
            width: self.resolution,
            height: self.resolution,
            data,
            format: TextureFormat::Rgba8,
        }
    }

    /// Generate marble texture
    pub fn generate_marble(&self, color1: [f32; 3], color2: [f32; 3], turbulence: f32) -> TextureData {
        let size = (self.resolution * self.resolution) as usize;
        let mut data = vec![0u8; size * 4];

        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let nx = x as f32 / self.resolution as f32;
                let ny = y as f32 / self.resolution as f32;

                let noise = self.fbm(nx * 4.0, ny * 4.0, 4, 0.5) * turbulence;
                let value = ((nx + ny + noise) * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;

                let idx = ((y * self.resolution + x) * 4) as usize;
                data[idx] = ((color1[0] * (1.0 - value) + color2[0] * value) * 255.0) as u8;
                data[idx + 1] = ((color1[1] * (1.0 - value) + color2[1] * value) * 255.0) as u8;
                data[idx + 2] = ((color1[2] * (1.0 - value) + color2[2] * value) * 255.0) as u8;
                data[idx + 3] = 255;
            }
        }

        TextureData {
            width: self.resolution,
            height: self.resolution,
            data,
            format: TextureFormat::Rgba8,
        }
    }

    /// Generate wood grain texture
    pub fn generate_wood(&self, color1: [f32; 3], color2: [f32; 3], ring_frequency: f32) -> TextureData {
        let size = (self.resolution * self.resolution) as usize;
        let mut data = vec![0u8; size * 4];

        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let nx = (x as f32 / self.resolution as f32 - 0.5) * 2.0;
                let ny = (y as f32 / self.resolution as f32 - 0.5) * 2.0;

                let dist = (nx * nx + ny * ny).sqrt();
                let noise = self.fbm(nx * 8.0, ny * 8.0, 2, 0.5) * 0.1;

                let rings = ((dist + noise) * ring_frequency).sin() * 0.5 + 0.5;
                let value = rings * rings;

                let idx = ((y * self.resolution + x) * 4) as usize;
                data[idx] = ((color1[0] * (1.0 - value) + color2[0] * value) * 255.0) as u8;
                data[idx + 1] = ((color1[1] * (1.0 - value) + color2[1] * value) * 255.0) as u8;
                data[idx + 2] = ((color1[2] * (1.0 - value) + color2[2] * value) * 255.0) as u8;
                data[idx + 3] = 255;
            }
        }

        TextureData {
            width: self.resolution,
            height: self.resolution,
            data,
            format: TextureFormat::Rgba8,
        }
    }

    /// Generate energy/plasma texture
    pub fn generate_energy(&self, color: [f32; 3], speed: f32) -> TextureData {
        let size = (self.resolution * self.resolution) as usize;
        let mut data = vec![0u8; size * 4];

        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let nx = x as f32 / self.resolution as f32;
                let ny = y as f32 / self.resolution as f32;

                // Multiple overlapping sine waves
                let v1 = (nx * 10.0).sin();
                let v2 = (ny * 10.0).sin();
                let v3 = ((nx + ny) * 10.0 * speed).sin();
                let v4 = ((nx * nx + ny * ny).sqrt() * 12.0).sin();

                let value = (v1 + v2 + v3 + v4) / 4.0 * 0.5 + 0.5;
                let intensity = value * value;

                let idx = ((y * self.resolution + x) * 4) as usize;
                data[idx] = (color[0] * intensity * 255.0) as u8;
                data[idx + 1] = (color[1] * intensity * 255.0) as u8;
                data[idx + 2] = (color[2] * intensity * 255.0) as u8;
                data[idx + 3] = (intensity * 255.0) as u8;
            }
        }

        TextureData {
            width: self.resolution,
            height: self.resolution,
            data,
            format: TextureFormat::Rgba8,
        }
    }

    /// Generate voronoi/cell texture
    pub fn generate_voronoi(&self, cell_count: u32, color1: [f32; 3], color2: [f32; 3]) -> TextureData {
        let size = (self.resolution * self.resolution) as usize;
        let mut data = vec![0u8; size * 4];

        // Generate cell centers using seeded random
        let mut rng = SimpleRng::new(self.seed);
        let cells: Vec<(f32, f32)> = (0..cell_count)
            .map(|_| (rng.next_float(), rng.next_float()))
            .collect();

        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let nx = x as f32 / self.resolution as f32;
                let ny = y as f32 / self.resolution as f32;

                // Find distance to nearest and second nearest cell
                let mut min_dist = f32::MAX;
                let mut second_dist = f32::MAX;

                for &(cx, cy) in &cells {
                    let dist = ((nx - cx) * (nx - cx) + (ny - cy) * (ny - cy)).sqrt();
                    if dist < min_dist {
                        second_dist = min_dist;
                        min_dist = dist;
                    } else if dist < second_dist {
                        second_dist = dist;
                    }
                }

                // F2 - F1 for cell boundaries
                let edge = (second_dist - min_dist).clamp(0.0, 1.0);
                let value = edge;

                let idx = ((y * self.resolution + x) * 4) as usize;
                data[idx] = ((color1[0] * (1.0 - value) + color2[0] * value) * 255.0) as u8;
                data[idx + 1] = ((color1[1] * (1.0 - value) + color2[1] * value) * 255.0) as u8;
                data[idx + 2] = ((color1[2] * (1.0 - value) + color2[2] * value) * 255.0) as u8;
                data[idx + 3] = 255;
            }
        }

        TextureData {
            width: self.resolution,
            height: self.resolution,
            data,
            format: TextureFormat::Rgba8,
        }
    }

    /// Generate emotion-based texture
    pub fn generate_emotion_texture(&self, emotion: EmotionalValence) -> TextureData {
        let color = emotion.color();
        let secondary = self.shift_color(color, 0.2);

        match emotion {
            EmotionalValence::Joy => self.generate_energy(color, 1.5),
            EmotionalValence::Sadness => self.generate_marble(color, secondary, 0.5),
            EmotionalValence::Anger => self.generate_voronoi(20, color, [0.0, 0.0, 0.0]),
            EmotionalValence::Peace => self.generate_marble(color, secondary, 0.2),
            EmotionalValence::Fear => self.generate_noise(6, 0.7),
            EmotionalValence::Surprise => self.generate_energy(color, 2.0),
            EmotionalValence::Neutral => self.generate_noise(4, 0.5),
        }
    }

    /// Generate normal map from height map
    pub fn generate_normal_map(&self, height_map: &TextureData, strength: f32) -> TextureData {
        let mut data = vec![0u8; height_map.data.len()];

        for y in 1..height_map.height - 1 {
            for x in 1..height_map.width - 1 {
                let idx = |px: u32, py: u32| ((py * height_map.width + px) * 4) as usize;

                let h_left = height_map.data[idx(x - 1, y)] as f32 / 255.0;
                let h_right = height_map.data[idx(x + 1, y)] as f32 / 255.0;
                let h_up = height_map.data[idx(x, y - 1)] as f32 / 255.0;
                let h_down = height_map.data[idx(x, y + 1)] as f32 / 255.0;

                let dx = (h_left - h_right) * strength;
                let dy = (h_up - h_down) * strength;

                // Convert to tangent space normal
                let normal = glam::Vec3::new(-dx, -dy, 1.0).normalize();

                let out_idx = idx(x, y);
                data[out_idx] = ((normal.x * 0.5 + 0.5) * 255.0) as u8;
                data[out_idx + 1] = ((normal.y * 0.5 + 0.5) * 255.0) as u8;
                data[out_idx + 2] = ((normal.z * 0.5 + 0.5) * 255.0) as u8;
                data[out_idx + 3] = 255;
            }
        }

        TextureData {
            width: height_map.width,
            height: height_map.height,
            data,
            format: TextureFormat::Rgba8,
        }
    }

    /// Fractal Brownian motion
    fn fbm(&self, x: f32, y: f32, octaves: u32, persistence: f32) -> f32 {
        let mut total = 0.0f32;
        let mut amplitude = 1.0f32;
        let mut frequency = 1.0f32;
        let mut max_value = 0.0f32;

        for _ in 0..octaves {
            total += self.noise(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= 2.0;
        }

        total / max_value
    }

    /// Simple value noise
    fn noise(&self, x: f32, y: f32) -> f32 {
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;
        let xf = x.fract();
        let yf = y.fract();

        // Smooth interpolation
        let u = xf * xf * (3.0 - 2.0 * xf);
        let v = yf * yf * (3.0 - 2.0 * yf);

        // Hash corners
        let n00 = self.hash(xi, yi);
        let n10 = self.hash(xi + 1, yi);
        let n01 = self.hash(xi, yi + 1);
        let n11 = self.hash(xi + 1, yi + 1);

        // Bilinear interpolation
        let nx0 = n00 * (1.0 - u) + n10 * u;
        let nx1 = n01 * (1.0 - u) + n11 * u;

        nx0 * (1.0 - v) + nx1 * v
    }

    /// Hash function for noise (using wrapping arithmetic to prevent overflow)
    fn hash(&self, x: i32, y: i32) -> f32 {
        let n = x.wrapping_add(y.wrapping_mul(57)).wrapping_add(self.seed as i32);
        let n = (n << 13) ^ n;
        let t = n.wrapping_mul(
            n.wrapping_mul(n)
                .wrapping_mul(15731)
                .wrapping_add(789221),
        )
        .wrapping_add(1376312589)
            & 0x7fffffff;
        1.0 - (t as f32 / 1073741824.0)
    }

    /// Shift color hue
    fn shift_color(&self, color: [f32; 3], amount: f32) -> [f32; 3] {
        [
            (color[0] + amount).fract(),
            color[1] * 0.8,
            color[2] * 0.9,
        ]
    }
}

impl Default for TextureGenerator {
    fn default() -> Self {
        Self::new(512)
    }
}

/// Texture data container
#[derive(Debug, Clone)]
pub struct TextureData {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Raw pixel data
    pub data: Vec<u8>,
    /// Pixel format
    pub format: TextureFormat,
}

impl TextureData {
    /// Convert to PNG bytes
    pub fn to_png(&self) -> Result<Vec<u8>, String> {
        use std::io::Cursor;

        let mut buffer = Cursor::new(Vec::new());

        let color_type = match self.format {
            TextureFormat::Rgba8 => image::ColorType::Rgba8,
            TextureFormat::Rgb8 => image::ColorType::Rgb8,
        };

        image::write_buffer_with_format(
            &mut buffer,
            &self.data,
            self.width,
            self.height,
            color_type,
            image::ImageFormat::Png,
        ).map_err(|e| e.to_string())?;

        Ok(buffer.into_inner())
    }

    /// Get pixel at position
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let idx = ((y * self.width + x) * 4) as usize;
        [
            self.data.get(idx).copied().unwrap_or(0),
            self.data.get(idx + 1).copied().unwrap_or(0),
            self.data.get(idx + 2).copied().unwrap_or(0),
            self.data.get(idx + 3).copied().unwrap_or(255),
        ]
    }

    /// Set pixel at position
    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 3 < self.data.len() {
            self.data[idx] = color[0];
            self.data[idx + 1] = color[1];
            self.data[idx + 2] = color[2];
            self.data[idx + 3] = color[3];
        }
    }
}

/// Texture format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Rgba8,
    Rgb8,
}

/// Simple random number generator
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_float(&mut self) -> f32 {
        (self.next() >> 32) as f32 / u32::MAX as f32
    }
}

/// Texture atlas for batching
#[derive(Debug)]
pub struct TextureAtlas {
    /// Atlas texture
    pub texture: TextureData,
    /// UV regions for each texture
    pub regions: Vec<TextureRegion>,
    /// Padding between textures
    pub padding: u32,
}

impl TextureAtlas {
    /// Create new atlas
    pub fn new(size: u32, padding: u32) -> Self {
        Self {
            texture: TextureData {
                width: size,
                height: size,
                data: vec![0u8; (size * size * 4) as usize],
                format: TextureFormat::Rgba8,
            },
            regions: Vec::new(),
            padding,
        }
    }

    /// Add texture to atlas (simple left-to-right, top-to-bottom packing)
    pub fn add(&mut self, texture: &TextureData) -> Option<usize> {
        // Find next available position
        let (x, y) = self.find_position(texture.width, texture.height)?;

        // Copy texture data
        for ty in 0..texture.height {
            for tx in 0..texture.width {
                let src_pixel = texture.get_pixel(tx, ty);
                self.texture.set_pixel(x + tx, y + ty, src_pixel);
            }
        }

        // Add region
        let region = TextureRegion {
            x,
            y,
            width: texture.width,
            height: texture.height,
            u_min: x as f32 / self.texture.width as f32,
            v_min: y as f32 / self.texture.height as f32,
            u_max: (x + texture.width) as f32 / self.texture.width as f32,
            v_max: (y + texture.height) as f32 / self.texture.height as f32,
        };

        let idx = self.regions.len();
        self.regions.push(region);
        Some(idx)
    }

    /// Find position for new texture
    fn find_position(&self, width: u32, height: u32) -> Option<(u32, u32)> {
        let atlas_size = self.texture.width;

        // Simple row-based packing
        let mut x = self.padding;
        let mut y = self.padding;
        let mut row_height = 0u32;

        for region in &self.regions {
            if x + width + self.padding <= atlas_size {
                x = region.x + region.width + self.padding;
                row_height = row_height.max(region.height);
            } else {
                // Move to next row
                x = self.padding;
                y += row_height + self.padding;
                row_height = 0;
            }
        }

        // Check if it fits
        if x + width <= atlas_size && y + height <= atlas_size {
            Some((x, y))
        } else {
            None
        }
    }

    /// Get UV coordinates for region
    pub fn get_uvs(&self, index: usize) -> Option<[f32; 4]> {
        self.regions.get(index).map(|r| [r.u_min, r.v_min, r.u_max, r.v_max])
    }
}

/// Region in texture atlas
#[derive(Debug, Clone)]
pub struct TextureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_generation() {
        let gen = TextureGenerator::new(64);
        let texture = gen.generate_noise(4, 0.5);

        assert_eq!(texture.width, 64);
        assert_eq!(texture.height, 64);
        assert_eq!(texture.data.len(), 64 * 64 * 4);
    }

    #[test]
    fn test_marble_generation() {
        let gen = TextureGenerator::new(64);
        let texture = gen.generate_marble([1.0, 1.0, 1.0], [0.5, 0.5, 0.5], 0.5);

        assert_eq!(texture.data.len(), 64 * 64 * 4);
    }

    #[test]
    fn test_voronoi_generation() {
        let gen = TextureGenerator::new(64);
        let texture = gen.generate_voronoi(10, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);

        assert_eq!(texture.data.len(), 64 * 64 * 4);
    }

    #[test]
    fn test_emotion_texture() {
        let gen = TextureGenerator::new(64);

        for emotion in [
            EmotionalValence::Joy,
            EmotionalValence::Sadness,
            EmotionalValence::Anger,
        ] {
            let texture = gen.generate_emotion_texture(emotion);
            assert_eq!(texture.data.len(), 64 * 64 * 4);
        }
    }

    #[test]
    fn test_texture_atlas() {
        let mut atlas = TextureAtlas::new(256, 2);
        let gen = TextureGenerator::new(32);

        let tex1 = gen.generate_noise(2, 0.5);
        let tex2 = gen.generate_marble([1.0, 1.0, 1.0], [0.5, 0.5, 0.5], 0.5);

        let idx1 = atlas.add(&tex1);
        let idx2 = atlas.add(&tex2);

        assert!(idx1.is_some());
        assert!(idx2.is_some());
    }
}
