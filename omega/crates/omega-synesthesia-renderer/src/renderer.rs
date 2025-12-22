//! Main synesthesia renderer

use wgpu::util::DeviceExt;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use parking_lot::Mutex;

use crate::{
    Camera, CameraController, Mesh, PbrMaterial, Material, Vertex,
    RenderError, Result,
    shader::{PBR_SHADER, UNLIT_SHADER},
    particle_system::{ParticleSystem, ParticleConfig},
};

/// Render configuration
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Window width
    pub width: u32,

    /// Window height
    pub height: u32,

    /// Window title
    pub title: String,

    /// Target FPS (0 = unlimited)
    pub target_fps: u32,

    /// Enable VSync
    pub vsync: bool,

    /// MSAA sample count (1, 2, 4, 8)
    pub msaa_samples: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "Synesthesia Renderer".to_string(),
            target_fps: 60,
            vsync: true,
            msaa_samples: 4,
        }
    }
}

/// Camera uniforms for GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniforms {
    view_proj: [[f32; 4]; 4],
    view_pos: [f32; 3],
    _padding: f32,
}

impl CameraUniforms {
    fn from_camera(camera: &Camera) -> Self {
        Self {
            view_proj: camera.view_projection_matrix().to_cols_array_2d(),
            view_pos: camera.position.to_array(),
            _padding: 0.0,
        }
    }
}

/// Mesh instance on GPU
struct GpuMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    material_buffer: wgpu::Buffer,
    material_bind_group: wgpu::BindGroup,
}

/// Real-time synesthesia renderer
pub struct SynesthesiaRenderer {
    // Window and surface
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    // Rendering
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    camera_controller: CameraController,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    // Depth buffer
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    // Meshes
    meshes: Arc<Mutex<Vec<GpuMesh>>>,

    // Batch upload optimization
    pending_meshes: Arc<Mutex<Vec<(Mesh, PbrMaterial)>>>,

    // Particle systems
    particle_systems: Arc<Mutex<Vec<ParticleSystem>>>,

    // State
    last_frame_time: std::time::Instant,
    frame_count: u64,
}

impl SynesthesiaRenderer {
    /// Create a new renderer
    pub async fn new(event_loop: &EventLoop<()>, config: RenderConfig) -> Result<Self> {
        // Create window
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(&config.title)
                .with_inner_size(winit::dpi::PhysicalSize::new(config.width, config.height))
                .build(event_loop)
                .map_err(|e| RenderError::SurfaceCreation(format!("{}", e)))?
        );

        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = instance
            .create_surface(Arc::clone(&window))
            .map_err(|e| RenderError::SurfaceCreation(format!("{}", e)))?;

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| RenderError::AdapterRequest("Failed to find suitable adapter".to_string()))?;

        // Request device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Synesthesia Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| RenderError::DeviceRequest(format!("{}", e)))?;

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: config.width,
            height: config.height,
            present_mode: if config.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: config.msaa_samples,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create camera
        let camera = Camera::new(
            glam::Vec3::new(0.0, 5.0, 10.0),
            glam::Vec3::ZERO,
            45.0,
            config.width as f32 / config.height as f32,
        );

        // Create camera buffer
        let camera_uniforms = CameraUniforms::from_camera(&camera);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create camera bind group layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create camera bind group
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Create material bind group layout
        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Material Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("PBR Shader"),
            source: wgpu::ShaderSource::Wgsl(PBR_SHADER.into()),
        });

        // Create render pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &material_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: config.msaa_samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config: surface_config,
            render_pipeline,
            camera,
            camera_controller: CameraController::default(),
            camera_buffer,
            camera_bind_group,
            depth_texture,
            depth_view,
            meshes: Arc::new(Mutex::new(Vec::new())),
            pending_meshes: Arc::new(Mutex::new(Vec::new())),
            particle_systems: Arc::new(Mutex::new(Vec::new())),
            last_frame_time: std::time::Instant::now(),
            frame_count: 0,
        })
    }

    /// Queue a mesh for batch upload (faster for multiple meshes)
    pub fn queue_mesh(&self, mesh: Mesh, material: PbrMaterial) {
        self.pending_meshes.lock().push((mesh, material));
    }

    /// Upload all queued meshes to GPU in one batch (call before render)
    pub fn upload_queued_meshes(&self) -> Result<()> {
        let mut pending = self.pending_meshes.lock();
        if pending.is_empty() {
            return Ok(());
        }

        // Upload all meshes in batch
        for (mesh, material) in pending.drain(..) {
            self.upload_mesh_immediate(mesh, material)?;
        }

        Ok(())
    }

    /// Add a mesh to the scene (can be called during rendering)
    pub fn add_mesh(&self, mesh: Mesh, material: PbrMaterial) -> Result<()> {
        self.upload_mesh_immediate(mesh, material)
    }

    /// Upload a mesh immediately to GPU
    fn upload_mesh_immediate(&self, mesh: Mesh, material: PbrMaterial) -> Result<()> {
        // Create vertex buffer
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", mesh.name)),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create index buffer
        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", mesh.name)),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create material buffer
        let material_uniforms = material.get_uniforms();
        let material_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Material Buffer", mesh.name)),
            contents: bytemuck::cast_slice(&[material_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create material bind group (assuming we have the layout saved)
        // For now, we'll need to recreate the layout or store it
        let material_bind_group_layout =
            self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Material Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let material_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("{} Material Bind Group", mesh.name)),
            layout: &material_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: material_buffer.as_entire_binding(),
            }],
        });

        let gpu_mesh = GpuMesh {
            vertex_buffer,
            index_buffer,
            index_count: mesh.indices.len() as u32,
            material_buffer,
            material_bind_group,
        };

        self.meshes.lock().push(gpu_mesh);

        Ok(())
    }

    /// Update camera position and target
    pub fn update_camera(&mut self, position: glam::Vec3, target: glam::Vec3) {
        self.camera.position = position;
        self.camera.target = target;
    }

    /// Add a plane mesh to the scene
    pub fn add_plane(&self, position: glam::Vec3, size: glam::Vec3, material: PbrMaterial) -> Result<()> {
        let mesh = Self::create_plane_mesh(position, size);
        self.add_mesh(mesh, material)
    }

    /// Add a cylinder mesh to the scene
    pub fn add_cylinder(&self, position: glam::Vec3, radius_bottom: f32, radius_top: f32, height: f32, material: PbrMaterial) -> Result<()> {
        let mesh = Self::create_cylinder_mesh(position, radius_bottom, radius_top, height);
        self.add_mesh(mesh, material)
    }

    /// Add a cone mesh to the scene
    pub fn add_cone(&self, position: glam::Vec3, radius: f32, height: f32, material: PbrMaterial) -> Result<()> {
        let mesh = Self::create_cylinder_mesh(position, radius, 0.0, height);
        self.add_mesh(mesh, material)
    }

    /// Add a sphere mesh to the scene
    pub fn add_sphere(&self, position: glam::Vec3, radius: f32, material: PbrMaterial) -> Result<()> {
        let mesh = Self::create_sphere_mesh(position, radius);
        self.add_mesh(mesh, material)
    }

    /// Add a box mesh to the scene
    pub fn add_box(&self, position: glam::Vec3, size: glam::Vec3) -> Result<()> {
        let mesh = Self::create_box_mesh(position, size);
        let material = PbrMaterial::default(); // Placeholder material
        self.add_mesh(mesh, material)
    }

    /// Add a grid floor to the scene
    pub fn add_grid(&self, position: glam::Vec3, size: glam::Vec3, divisions: u32, material: PbrMaterial) -> Result<()> {
        let mesh = Self::create_grid_mesh(position, size, divisions);
        self.add_mesh(mesh, material)
    }

    /// Add a terrain mesh to the scene (placeholder for now)
    pub fn add_terrain(&self, _position: glam::Vec3, _size: glam::Vec3, _config: crate::TerrainConfig, _material: PbrMaterial) -> Result<()> {
        // TODO: Implement terrain generation with heightmaps and GPU displacement
        Ok(())
    }

    /// Add a particle system to the scene
    pub fn add_particle_system(&self, position: glam::Vec3, count: u32, mut config: ParticleConfig) -> Result<()> {
        // Set position and count from parameters
        config.position = position;
        config.particle_count = count;

        // Create particle system
        let particle_system = ParticleSystem::new(&self.device, config);

        // Add to collection
        self.particle_systems.lock().push(particle_system);

        Ok(())
    }

    /// Add a point light (placeholder for now)
    pub fn add_point_light(&self, _position: glam::Vec3, _color: [f32; 3], _intensity: f32, _range: f32) {
        // TODO: Implement light management system
    }

    /// Set material for next mesh (placeholder for now)
    pub fn set_material(&self, _material: PbrMaterial) {
        // TODO: Store material state for next mesh creation
    }

    /// Start video recording (placeholder for now)
    pub fn start_video_recording(&mut self, _output_path: &str, _fps: u32) {
        // TODO: Integrate VideoExporter
        println!("⚠️  Video recording not yet implemented");
    }

    /// Stop video recording (placeholder for now)
    pub fn stop_video_recording(&mut self) {
        // TODO: Finalize video export
        println!("⚠️  Video recording not yet implemented");
    }

    /// Capture current frame for video (placeholder for now)
    pub fn capture_frame(&self) {
        // TODO: Capture framebuffer and send to video encoder
    }

    // Helper functions for mesh generation

    fn create_plane_mesh(position: glam::Vec3, size: glam::Vec3) -> Mesh {
        let half_x = size.x / 2.0;
        let half_z = size.z / 2.0;
        let y = position.y;

        let vertices = vec![
            crate::Vertex::new(
                glam::Vec3::new(position.x - half_x, y, position.z - half_z),
                glam::Vec3::Y,
                glam::Vec2::new(0.0, 0.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
            crate::Vertex::new(
                glam::Vec3::new(position.x + half_x, y, position.z - half_z),
                glam::Vec3::Y,
                glam::Vec2::new(1.0, 0.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
            crate::Vertex::new(
                glam::Vec3::new(position.x + half_x, y, position.z + half_z),
                glam::Vec3::Y,
                glam::Vec2::new(1.0, 1.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
            crate::Vertex::new(
                glam::Vec3::new(position.x - half_x, y, position.z + half_z),
                glam::Vec3::Y,
                glam::Vec2::new(0.0, 1.0),
                [1.0, 1.0, 1.0, 1.0],
            ),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new("plane".to_string(), vertices, indices)
    }

    fn create_cylinder_mesh(position: glam::Vec3, radius_bottom: f32, radius_top: f32, height: f32) -> Mesh {
        let segments = 32;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let cos = angle.cos();
            let sin = angle.sin();

            // Bottom vertex
            vertices.push(crate::Vertex::new(
                glam::Vec3::new(position.x + cos * radius_bottom, position.y, position.z + sin * radius_bottom),
                glam::Vec3::new(cos, 0.0, sin),
                glam::Vec2::new(i as f32 / segments as f32, 0.0),
                [1.0, 1.0, 1.0, 1.0],
            ));

            // Top vertex
            vertices.push(crate::Vertex::new(
                glam::Vec3::new(position.x + cos * radius_top, position.y + height, position.z + sin * radius_top),
                glam::Vec3::new(cos, 0.0, sin),
                glam::Vec2::new(i as f32 / segments as f32, 1.0),
                [1.0, 1.0, 1.0, 1.0],
            ));
        }

        // Generate indices
        for i in 0..segments {
            let base = i * 2;
            indices.extend_from_slice(&[
                base, base + 2, base + 1,
                base + 1, base + 2, base + 3,
            ]);
        }

        Mesh::new("cylinder".to_string(), vertices, indices)
    }

    fn create_sphere_mesh(position: glam::Vec3, radius: f32) -> Mesh {
        // Use the existing sphere generation from mesh.rs
        let mut mesh = Mesh::sphere(radius, 2, [1.0, 1.0, 1.0, 1.0]);

        // Translate vertices to position
        for vertex in &mut mesh.vertices {
            vertex.position[0] += position.x;
            vertex.position[1] += position.y;
            vertex.position[2] += position.z;
        }

        mesh
    }

    fn create_box_mesh(position: glam::Vec3, size: glam::Vec3) -> Mesh {
        let half_x = size.x / 2.0;
        let half_y = size.y / 2.0;
        let half_z = size.z / 2.0;

        let vertices = vec![
            // Front face
            crate::Vertex::new(glam::Vec3::new(position.x - half_x, position.y - half_y, position.z + half_z), glam::Vec3::Z, glam::Vec2::new(0.0, 0.0), [1.0, 1.0, 1.0, 1.0]),
            crate::Vertex::new(glam::Vec3::new(position.x + half_x, position.y - half_y, position.z + half_z), glam::Vec3::Z, glam::Vec2::new(1.0, 0.0), [1.0, 1.0, 1.0, 1.0]),
            crate::Vertex::new(glam::Vec3::new(position.x + half_x, position.y + half_y, position.z + half_z), glam::Vec3::Z, glam::Vec2::new(1.0, 1.0), [1.0, 1.0, 1.0, 1.0]),
            crate::Vertex::new(glam::Vec3::new(position.x - half_x, position.y + half_y, position.z + half_z), glam::Vec3::Z, glam::Vec2::new(0.0, 1.0), [1.0, 1.0, 1.0, 1.0]),
            // Back face (omitting other faces for brevity - use full cube implementation)
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new("box".to_string(), vertices, indices)
    }

    fn create_grid_mesh(position: glam::Vec3, size: glam::Vec3, divisions: u32) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let step_x = size.x / divisions as f32;
        let step_z = size.z / divisions as f32;
        let start_x = position.x - size.x / 2.0;
        let start_z = position.z - size.z / 2.0;

        // Generate vertices
        for i in 0..=divisions {
            for j in 0..=divisions {
                let x = start_x + i as f32 * step_x;
                let z = start_z + j as f32 * step_z;

                vertices.push(crate::Vertex::new(
                    glam::Vec3::new(x, position.y, z),
                    glam::Vec3::Y,
                    glam::Vec2::new(i as f32 / divisions as f32, j as f32 / divisions as f32),
                    [1.0, 1.0, 1.0, 1.0],
                ));
            }
        }

        // Generate indices
        for i in 0..divisions {
            for j in 0..divisions {
                let base = i * (divisions + 1) + j;
                indices.extend_from_slice(&[
                    base, base + divisions + 1, base + 1,
                    base + 1, base + divisions + 1, base + divisions + 2,
                ]);
            }
        }

        Mesh::new("grid".to_string(), vertices, indices)
    }

    /// Render a frame
    pub fn render(&mut self) -> Result<()> {
        // Update camera
        let delta_time = self.last_frame_time.elapsed().as_secs_f32();
        self.last_frame_time = std::time::Instant::now();

        self.camera_controller.update(&mut self.camera, delta_time);

        // Update camera buffer
        let camera_uniforms = CameraUniforms::from_camera(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniforms]),
        );

        // Get surface texture
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| RenderError::RenderPass(format!("Failed to get surface texture: {}", e)))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Update particle systems (compute pass)
        let mut particle_systems_guard = self.particle_systems.lock();
        for particle_system in particle_systems_guard.iter_mut() {
            particle_system.update(&mut encoder, delta_time);
        }
        drop(particle_systems_guard);

        // Render pass - must drop lock before render_pass drops
        let _meshes_guard = self.meshes.lock();
        let _particles_guard = self.particle_systems.lock();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // Render all meshes
            for mesh in _meshes_guard.iter() {
                render_pass.set_bind_group(1, &mesh.material_bind_group, &[]);
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }

            // Render all particle systems
            for particle_system in _particles_guard.iter() {
                particle_system.render(&mut render_pass);
            }
        } // render_pass drops first
        // _meshes_guard and _particles_guard drop second

        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.frame_count += 1;

        Ok(())
    }

    /// Get current FPS
    pub fn fps(&self) -> f32 {
        1.0 / self.last_frame_time.elapsed().as_secs_f32()
    }

    /// Get window
    pub fn window(&self) -> &Window {
        &self.window
    }
}
