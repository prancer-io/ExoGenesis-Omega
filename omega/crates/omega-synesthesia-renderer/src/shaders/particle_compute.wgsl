// Particle Compute Shader
// Simulates millions of particles entirely on GPU

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    color: vec4<f32>,
    lifetime: f32,
    age: f32,
    size: f32,
    padding: f32,
}

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    if (index >= arrayLength(&particles)) {
        return;
    }

    var particle = particles[index];

    // Fixed time step for stable simulation (16.67ms = 60 FPS)
    let delta_time = 0.016667;
    let gravity = vec3<f32>(0.0, -9.8, 0.0);

    // Update age
    particle.age += delta_time;

    // Reset particle if dead
    if (particle.age > particle.lifetime) {
        particle.age = 0.0;
        particle.position = vec3<f32>(0.0, 0.0, 0.0);

        // Random velocity
        let angle = f32(index) / f32(arrayLength(&particles)) * 6.28318530718;
        particle.velocity = vec3<f32>(
            cos(angle) * 5.0,
            10.0,
            sin(angle) * 5.0
        );
    }

    // Apply physics
    particle.velocity += gravity * delta_time;
    particle.position += particle.velocity * delta_time;

    // Update color based on lifetime (fade out)
    let life_ratio = particle.age / particle.lifetime;
    particle.color.a = 1.0 - life_ratio;

    particles[index] = particle;
}

// HSL to RGB conversion
fn hsl_to_rgb(hsl: vec3<f32>) -> vec3<f32> {
    let c = (1.0 - abs(2.0 * hsl.z - 1.0)) * hsl.y;
    let x = c * (1.0 - abs((hsl.x * 6.0) % 2.0 - 1.0));
    let m = hsl.z - c / 2.0;

    var rgb = vec3<f32>(0.0);
    let h = hsl.x * 6.0;

    if (h < 1.0) {
        rgb = vec3<f32>(c, x, 0.0);
    } else if (h < 2.0) {
        rgb = vec3<f32>(x, c, 0.0);
    } else if (h < 3.0) {
        rgb = vec3<f32>(0.0, c, x);
    } else if (h < 4.0) {
        rgb = vec3<f32>(0.0, x, c);
    } else if (h < 5.0) {
        rgb = vec3<f32>(x, 0.0, c);
    } else {
        rgb = vec3<f32>(c, 0.0, x);
    }

    return rgb + vec3<f32>(m);
}
