// Particle Render Shader
// Renders particles as points with size attenuation

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    color: vec4<f32>,
    lifetime: f32,
    age: f32,
    size: f32,
    padding: f32,
}

struct CameraUniforms {
    view_proj: mat4x4<f32>,
    view_pos: vec3<f32>,
    padding: f32,
}

@group(0) @binding(0) var<storage, read> particles: array<Particle>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) size: f32,
}

// Note: Camera bind group must be set before calling render
// We'll use push constants or instance data for camera in a future version
// For now, using identity matrix and manual camera handling

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let particle = particles[vertex_index];

    var output: VertexOutput;
    // Note: View-projection will be handled by updating particle positions
    // Or we can add camera uniform later
    output.position = vec4<f32>(particle.position, 1.0);
    output.color = particle.color;
    output.size = particle.size;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple particle rendering with alpha
    return input.color;
}
