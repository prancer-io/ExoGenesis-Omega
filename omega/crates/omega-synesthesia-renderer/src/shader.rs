//! WGSL shaders for PBR rendering

/// PBR vertex and fragment shader (WGSL)
pub const PBR_SHADER: &str = r#"
// Vertex shader

struct CameraUniforms {
    view_proj: mat4x4<f32>,
    view_pos: vec3<f32>,
    _padding: f32,
};

struct MaterialUniforms {
    base_color: vec4<f32>,
    metallic: f32,
    roughness: f32,
    normal_strength: f32,
    ao_strength: f32,
    emission: vec4<f32>,  // RGB + strength
    _padding: vec2<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@group(1) @binding(0)
var<uniform> material: MaterialUniforms;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position to clip space
    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);

    // Pass world position for lighting
    out.world_position = vertex.position;

    // Pass normal (assume no model transform for now)
    out.normal = normalize(vertex.normal);

    // Pass texture coordinates and color
    out.uv = vertex.uv;
    out.color = vertex.color;

    return out;
}

// Fragment shader

const PI: f32 = 3.14159265359;

// PBR functions

fn distribution_ggx(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(N, H), 0.0);
    let NdotH2 = NdotH * NdotH;

    let num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;

    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;

    return num / denom;
}

fn geometry_smith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32 {
    let NdotV = max(dot(N, V), 0.0);
    let NdotL = max(dot(N, L), 0.0);
    let ggx2 = geometry_schlick_ggx(NdotV, roughness);
    let ggx1 = geometry_schlick_ggx(NdotL, roughness);

    return ggx1 * ggx2;
}

fn fresnel_schlick(cosTheta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Material properties
    let albedo = material.base_color.rgb * in.color.rgb;
    let metallic = material.metallic;
    let roughness = max(material.roughness, 0.04);  // Prevent division by zero

    // Lighting setup (simple directional light for now)
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    let ambient = vec3<f32>(0.03, 0.03, 0.03);

    // View direction
    let N = normalize(in.normal);
    let V = normalize(camera.view_pos - in.world_position);
    let L = light_dir;
    let H = normalize(V + L);

    // Calculate reflectance at normal incidence
    var F0 = vec3<f32>(0.04, 0.04, 0.04);
    F0 = mix(F0, albedo, metallic);

    // Cook-Torrance BRDF
    let NDF = distribution_ggx(N, H, roughness);
    let G = geometry_smith(N, V, L, roughness);
    let F = fresnel_schlick(max(dot(H, V), 0.0), F0);

    let kS = F;
    var kD = vec3<f32>(1.0, 1.0, 1.0) - kS;
    kD = kD * (1.0 - metallic);

    let numerator = NDF * G * F;
    let denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
    let specular = numerator / denominator;

    // Radiance
    let NdotL = max(dot(N, L), 0.0);
    let radiance = light_color;

    let Lo = (kD * albedo / PI + specular) * radiance * NdotL;

    // Ambient lighting (simple)
    let ambient_contribution = ambient * albedo * material.ao_strength;

    // Emission
    let emission = material.emission.rgb * material.emission.w;

    // Final color
    var color = ambient_contribution + Lo + emission;

    // HDR tonemapping (simple Reinhard)
    color = color / (color + vec3<f32>(1.0, 1.0, 1.0));

    // Gamma correction
    color = pow(color, vec3<f32>(1.0 / 2.2, 1.0 / 2.2, 1.0 / 2.2));

    return vec4<f32>(color, material.base_color.a * in.color.a);
}
"#;

/// Simple unlit shader for debugging
pub const UNLIT_SHADER: &str = r#"
struct CameraUniforms {
    view_proj: mat4x4<f32>,
    view_pos: vec3<f32>,
    _padding: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbr_shader_not_empty() {
        assert!(!PBR_SHADER.is_empty());
        assert!(PBR_SHADER.contains("vs_main"));
        assert!(PBR_SHADER.contains("fs_main"));
    }

    #[test]
    fn test_unlit_shader_not_empty() {
        assert!(!UNLIT_SHADER.is_empty());
        assert!(UNLIT_SHADER.contains("vs_main"));
        assert!(UNLIT_SHADER.contains("fs_main"));
    }
}
