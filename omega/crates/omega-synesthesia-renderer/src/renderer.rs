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
                    memory_hints: Default::default(),
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
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
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
            cache: None,
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

        // Render pass
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
            let meshes = self.meshes.lock();
            for mesh in meshes.iter() {
                render_pass.set_bind_group(1, &mesh.material_bind_group, &[]);
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }
        }

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
