//! Reactive Shader System
//!
//! WGSL shaders that respond to music in real-time.
//! These are the "magic" that makes pre-rendered video feel alive.

mod uniforms;
mod revelation;

pub use uniforms::ShaderUniforms;
pub use revelation::REVELATION_SHADER;

/// Complete shader source for the visualization
pub const VERTEX_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Full-screen triangle
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.uv = positions[vertex_index] * 0.5 + 0.5;
    output.uv.y = 1.0 - output.uv.y; // Flip Y
    return output;
}
"#;

/// Main fragment shader with all effects
pub const FRAGMENT_SHADER: &str = r#"
// ═══════════════════════════════════════════════════════════════════
// SYNESTHESIA - Reactive Fragment Shader
// ═══════════════════════════════════════════════════════════════════

struct Uniforms {
    time: f32,
    delta_time: f32,
    resolution: vec2<f32>,

    // Music understanding
    clarity: f32,
    energy: f32,
    bass: f32,
    mid: f32,
    high: f32,
    beat: f32,
    tempo: f32,

    // Emotion
    valence: f32,
    arousal: f32,
    hue: f32,
    saturation: f32,
    lightness: f32,

    // Section
    section_type: u32,
    section_progress: f32,
    is_climax: u32,

    // Effects
    bloom_intensity: f32,
    chromatic_amount: f32,
    vignette_strength: f32,
    grain_amount: f32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var video_texture: texture_2d<f32>;
@group(0) @binding(2) var video_sampler: sampler;

// ─────────────────────────────────────────────────────────────────
// NOISE FUNCTIONS
// ─────────────────────────────────────────────────────────────────

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

fn fbm(p: vec2<f32>, octaves: i32) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var p_var = p;

    for (var i = 0; i < octaves; i = i + 1) {
        value = value + amplitude * noise(p_var);
        p_var = p_var * 2.0;
        amplitude = amplitude * 0.5;
    }
    return value;
}

// ─────────────────────────────────────────────────────────────────
// COLOR UTILITIES
// ─────────────────────────────────────────────────────────────────

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    let c = (1.0 - abs(2.0 * l - 1.0)) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - abs(h_prime % 2.0 - 1.0));
    let m = l - c / 2.0;

    var rgb: vec3<f32>;
    if (h_prime < 1.0) { rgb = vec3<f32>(c, x, 0.0); }
    else if (h_prime < 2.0) { rgb = vec3<f32>(x, c, 0.0); }
    else if (h_prime < 3.0) { rgb = vec3<f32>(0.0, c, x); }
    else if (h_prime < 4.0) { rgb = vec3<f32>(0.0, x, c); }
    else if (h_prime < 5.0) { rgb = vec3<f32>(x, 0.0, c); }
    else { rgb = vec3<f32>(c, 0.0, x); }

    return rgb + m;
}

// ─────────────────────────────────────────────────────────────────
// EFFECT FUNCTIONS
// ─────────────────────────────────────────────────────────────────

fn apply_chromatic_aberration(uv: vec2<f32>, amount: f32) -> vec3<f32> {
    let offset = amount * (uv - 0.5);
    let r = textureSample(video_texture, video_sampler, uv + offset).r;
    let g = textureSample(video_texture, video_sampler, uv).g;
    let b = textureSample(video_texture, video_sampler, uv - offset).b;
    return vec3<f32>(r, g, b);
}

fn apply_bloom(color: vec3<f32>, intensity: f32) -> vec3<f32> {
    // Simplified bloom - boost bright areas
    let brightness = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    let bloom = color * max(brightness - 0.5, 0.0) * 2.0 * intensity;
    return color + bloom;
}

fn apply_vignette(color: vec3<f32>, uv: vec2<f32>, strength: f32) -> vec3<f32> {
    let center = uv - 0.5;
    let dist = length(center);
    let vignette = 1.0 - dist * strength;
    return color * vignette;
}

fn apply_grain(color: vec3<f32>, uv: vec2<f32>, amount: f32) -> vec3<f32> {
    let grain = hash(uv * u.time * 1000.0) - 0.5;
    return color + grain * amount;
}

fn apply_color_grade(color: vec3<f32>) -> vec3<f32> {
    // Apply emotion-based color grading
    let target = hsl_to_rgb(u.hue, u.saturation, u.lightness);
    let blend = 0.2 + u.arousal * 0.1;
    return mix(color, color * target * 2.0, blend);
}

// ─────────────────────────────────────────────────────────────────
// ABSTRACT LAYER (low clarity)
// ─────────────────────────────────────────────────────────────────

fn render_abstract(uv: vec2<f32>) -> vec3<f32> {
    let center = uv - 0.5;
    let dist = length(center);
    let angle = atan2(center.y, center.x);
    let t = u.time * 0.5;

    // Flowing noise
    var noise_coord = uv * 3.0 + t * 0.3;
    noise_coord = noise_coord + vec2<f32>(
        sin(angle * 3.0 + t) * u.bass * 0.2,
        cos(angle * 2.0 + t) * u.mid * 0.2
    );
    let n = fbm(noise_coord, 5);

    // Base color from emotion
    let base = hsl_to_rgb(u.hue, u.saturation * 0.7, u.lightness * 0.6);

    // Abstract color mixing
    var color = base;
    color = mix(color, base * 0.5, n);
    color = color + vec3<f32>(u.high * 0.3) * (1.0 - dist);

    // Beat pulse
    let beat_pulse = u.beat * (1.0 - dist * 0.5);
    color = color + beat_pulse * 0.3;

    return color;
}

// ─────────────────────────────────────────────────────────────────
// FORMING LAYER (medium clarity)
// ─────────────────────────────────────────────────────────────────

fn render_forming(uv: vec2<f32>) -> vec3<f32> {
    let center = uv - 0.5;
    let dist = length(center);
    let angle = atan2(center.y, center.x);
    let t = u.time;

    let base = hsl_to_rgb(u.hue, u.saturation, u.lightness);

    // Rings
    let ring1 = smoothstep(0.02, 0.0, abs(dist - 0.3 - u.bass * 0.1));
    let ring2 = smoothstep(0.015, 0.0, abs(dist - 0.45 - u.mid * 0.05));

    // Radial rays
    let rays = abs(sin(angle * 8.0 + t * 2.0)) * smoothstep(0.5, 0.2, dist) * u.high * 0.5;

    var color = render_abstract(uv);
    color = color + ring1 * base * 2.0;
    color = color + ring2 * base * 1.5;
    color = color + rays;

    return color;
}

// ─────────────────────────────────────────────────────────────────
// REVEALED LAYER (high clarity)
// ─────────────────────────────────────────────────────────────────

fn render_revealed(uv: vec2<f32>) -> vec3<f32> {
    let center = uv - 0.5;
    let dist = length(center);

    // Sample video texture if available
    var color = textureSample(video_texture, video_sampler, uv).rgb;

    // Add reactive effects on top
    let detail = fbm(uv * 10.0 + u.time * 0.2, 6);
    color = mix(color, color * (0.8 + detail * 0.4), 0.3);

    // Cinematic vignette
    color = color * (1.0 - dist * 0.4);

    return color;
}

// ─────────────────────────────────────────────────────────────────
// MAIN FRAGMENT SHADER
// ─────────────────────────────────────────────────────────────────

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var color: vec3<f32>;

    // Blend layers based on clarity
    let abstract_color = render_abstract(uv);
    let forming_color = render_forming(uv);
    let revealed_color = render_revealed(uv);

    // Progressive revelation
    if (u.clarity < 0.3) {
        color = abstract_color;
    } else if (u.clarity < 0.6) {
        let t = (u.clarity - 0.3) / 0.3;
        color = mix(abstract_color, forming_color, t);
    } else {
        let t = (u.clarity - 0.6) / 0.4;
        color = mix(forming_color, revealed_color, t);
    }

    // ─────────────────────────────────────────────────────────────
    // POST-PROCESSING
    // ─────────────────────────────────────────────────────────────

    // Chromatic aberration (more on high energy)
    let chroma_amount = u.chromatic_amount * u.energy;
    if (chroma_amount > 0.001) {
        let offset = chroma_amount * (uv - 0.5);
        color.r = color.r + offset.x * 0.5;
        color.b = color.b - offset.x * 0.3;
    }

    // Bloom (more on beats)
    let bloom_amount = u.bloom_intensity * (1.0 + u.beat * 0.5);
    color = apply_bloom(color, bloom_amount);

    // Color grading based on emotion
    color = apply_color_grade(color);

    // Vignette with beat pulse
    let vignette_amount = u.vignette_strength + u.beat * 0.1;
    color = apply_vignette(color, uv, vignette_amount);

    // Film grain (more on low clarity)
    let grain_amount = u.grain_amount * (1.0 - u.clarity * 0.5);
    color = apply_grain(color, uv, grain_amount);

    // Climax flash
    if (u.is_climax > 0u) {
        color = color + vec3<f32>(0.2);
    }

    // Gamma correction
    color = pow(color, vec3<f32>(0.85));

    // Clamp
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}
"#;
