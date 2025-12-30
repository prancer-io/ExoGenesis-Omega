//! Revelation Shader
//!
//! The core visual effect that implements the "painter's algorithm" -
//! progressive visual emergence as clarity increases.

/// Revelation shader source (WGSL)
/// This shader creates the gradual emergence effect.
pub const REVELATION_SHADER: &str = r#"
// ═══════════════════════════════════════════════════════════════════
// REVELATION SHADER
// Progressive visual emergence based on music understanding
// ═══════════════════════════════════════════════════════════════════

// Clarity levels:
// 0.00 - 0.15: ABSTRACT    - Pure colors and noise
// 0.15 - 0.35: EMERGING    - Forms appearing
// 0.35 - 0.55: FORMING     - Structure visible
// 0.55 - 0.80: CLARIFYING  - Scene revealing
// 0.80 - 1.00: REVEALED    - Full vision

fn revelation_layer(uv: vec2<f32>, clarity: f32, t: f32) -> vec4<f32> {
    let center = uv - 0.5;
    let dist = length(center);
    let angle = atan2(center.y, center.x);

    // ─────────────────────────────────────────────────────────────
    // ABSTRACT LAYER (clarity < 0.15)
    // ─────────────────────────────────────────────────────────────

    // Pure noise and color flow
    let noise_scale = 4.0 - clarity * 3.0;
    let noise_speed = 0.3 + clarity * 0.2;
    let n = fbm(uv * noise_scale + t * noise_speed, 5);

    // Color from noise
    let abstract_color = vec3<f32>(
        0.5 + 0.5 * sin(n * 6.28 + t),
        0.5 + 0.5 * sin(n * 6.28 + t + 2.094),
        0.5 + 0.5 * sin(n * 6.28 + t + 4.188)
    );

    // ─────────────────────────────────────────────────────────────
    // EMERGING LAYER (clarity 0.15 - 0.35)
    // ─────────────────────────────────────────────────────────────

    // Shapes start to coalesce
    let emerging_factor = smoothstep(0.1, 0.35, clarity);
    let shape = smoothstep(0.4, 0.35, dist + sin(angle * 4.0 + t) * 0.1);
    let emerging_color = mix(abstract_color, abstract_color * 1.5, shape * emerging_factor);

    // ─────────────────────────────────────────────────────────────
    // FORMING LAYER (clarity 0.35 - 0.55)
    // ─────────────────────────────────────────────────────────────

    // Definite structures appear
    let forming_factor = smoothstep(0.3, 0.55, clarity);

    // Geometric forms
    let ring = smoothstep(0.02, 0.0, abs(dist - 0.35));
    let rays = abs(sin(angle * 8.0 + t)) * smoothstep(0.4, 0.1, dist);

    let forming_color = emerging_color;
    // forming_color += ring * vec3(1.0) * forming_factor;
    // forming_color += rays * vec3(0.8) * forming_factor;

    // ─────────────────────────────────────────────────────────────
    // CLARIFYING LAYER (clarity 0.55 - 0.80)
    // ─────────────────────────────────────────────────────────────

    let clarifying_factor = smoothstep(0.5, 0.8, clarity);

    // Central focal point
    let focal = smoothstep(0.15, 0.0, dist) * clarifying_factor;

    // Detailed patterns
    let pattern = sin(uv.x * 20.0 + t) * sin(uv.y * 20.0 + t * 0.7);
    let pattern_mask = smoothstep(0.8, 1.0, pattern) * (1.0 - dist) * clarifying_factor;

    let clarifying_color = mix(forming_color, vec3<f32>(1.0), focal * 0.5);

    // ─────────────────────────────────────────────────────────────
    // REVEALED LAYER (clarity 0.80 - 1.00)
    // ─────────────────────────────────────────────────────────────

    let revealed_factor = smoothstep(0.75, 1.0, clarity);

    // Full scene with detail
    let detail = fbm(uv * 10.0 + t * 0.1, 6);
    let revealed_color = clarifying_color * (0.9 + detail * 0.2 * revealed_factor);

    // Cinematic vignette
    let vignette = 1.0 - dist * 0.5 * revealed_factor;

    return vec4<f32>(revealed_color * vignette, 1.0);
}

// Noise functions (included for shader compilation)
fn hash_r(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn noise_r(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash_r(i), hash_r(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash_r(i + vec2<f32>(0.0, 1.0)), hash_r(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

fn fbm_r(p: vec2<f32>, octaves: i32) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var p_var = p;
    for (var i = 0; i < octaves; i = i + 1) {
        value = value + amplitude * noise_r(p_var);
        p_var = p_var * 2.0;
        amplitude = amplitude * 0.5;
    }
    return value;
}
"#;
