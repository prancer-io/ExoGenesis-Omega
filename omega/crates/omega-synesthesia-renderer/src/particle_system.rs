/*!
 * GPU Compute Shader Particle System
 *
 * Handles MILLIONS of particles using GPU compute shaders for simulation.
 * Updates positions, velocities, and colors entirely on GPU for maximum performance.
 */

use wgpu::util::DeviceExt;
use glam::{Vec3, Vec4};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    color: [f32; 4],
    lifetime: f32,
    age: f32,
    size: f32,
    _padding: f32,
}

pub struct ParticleSystem {
    particle_count: u32,
    particle_buffer: wgpu::Buffer,
    particle_bind_group: wgpu::BindGroup,
    compute_pipeline: wgpu::ComputePipeline,
    render_pipeline: wgpu::RenderPipeline,
    config: ParticleConfig,
}

pub struct ParticleConfig {
    pub position: Vec3,
    pub particle_count: u32,
    pub size: f32,
    pub color: Vec4,
    pub velocity_range: Vec3,
    pub lifetime: f32,
    pub gravity: Vec3,
    pub rainbow_mode: bool,
    pub float_mode: bool,
}

impl ParticleSystem {
    pub fn new(
        device: &wgpu::Device,
        config: ParticleConfig,
    ) -> Self {
        let particle_count = config.particle_count;

        // Initialize particles on CPU
        let mut particles = Vec::with_capacity(particle_count as usize);
        for i in 0..particle_count {
            let angle = (i as f32 / particle_count as f32) * std::f32::consts::PI * 2.0;
            let radius = (i as f32 / particle_count as f32) * 5.0;

            let vel = if config.float_mode {
                Vec3::new(
                    (rand::random::<f32>() - 0.5) * config.velocity_range.x,
                    (rand::random::<f32>() - 0.5) * config.velocity_range.y,
                    (rand::random::<f32>() - 0.5) * config.velocity_range.z,
                )
            } else {
                Vec3::new(
                    angle.cos() * config.velocity_range.x,
                    config.velocity_range.y,
                    angle.sin() * config.velocity_range.z,
                )
            };

            particles.push(Particle {
                position: [
                    config.position.x + angle.cos() * radius,
                    config.position.y,
                    config.position.z + angle.sin() * radius,
                ],
                velocity: vel.to_array(),
                color: config.color.to_array(),
                lifetime: config.lifetime,
                age: rand::random::<f32>() * config.lifetime,
                size: config.size,
                _padding: 0.0,
            });
        }

        // Create GPU buffer
        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particle Buffer"),
            contents: bytemuck::cast_slice(&particles),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
        });

        // Compute shader for particle simulation
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Particle Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/particle_compute.wgsl").into()),
        });

        // Compute pipeline
        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Particle Compute Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Particle Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
        });

        // Render shader for particle rendering
        let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Particle Render Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/particle_render.wgsl").into()),
        });

        // Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Particle Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let particle_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_buffer.as_entire_binding(),
                },
            ],
        });

        // Render pipeline layout
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Particle Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Particle Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            particle_count,
            particle_buffer,
            particle_bind_group,
            compute_pipeline,
            render_pipeline,
            config,
        }
    }

    pub fn update(&mut self, encoder: &mut wgpu::CommandEncoder, _dt: f32) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Particle Compute Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.particle_bind_group, &[]);

        // Dispatch compute shader (64 particles per workgroup)
        let workgroup_count = (self.particle_count + 63) / 64;
        compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.particle_bind_group, &[]);
        render_pass.draw(0..self.particle_count, 0..1);
    }
}

fn rand_range(min: f32, max: f32) -> f32 {
    min + rand::random::<f32>() * (max - min)
}
