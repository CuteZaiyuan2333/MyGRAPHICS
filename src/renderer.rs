use wgpu::util::DeviceExt;
use crate::context::WgpuContext;
use crate::texture::Texture;
use glam::Mat4;
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Range;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

struct DrawBatch {
    texture: Arc<Texture>,
    indices: Range<u32>,
}

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    _camera_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    
    vertex_buffer: wgpu::Buffer, // Dynamic buffer
    vertex_capacity: usize,
    
    white_texture: Arc<Texture>,
    texture_cache: HashMap<String, Arc<Texture>>,
    
    // Current State
    current_color: [f32; 4],
    current_texture: Arc<Texture>,
    _current_shader: String, // Placeholder for future
    
    // Batching
    vertices: Vec<Vertex>,
    batches: Vec<DrawBatch>,
}

impl Renderer {
    pub fn new(ctx: &WgpuContext) -> Self {
        let device = &ctx.device;
        let config = &ctx.config;

        // 1. Bind Group Layouts
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        // 2. Pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: 8,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        wgpu::VertexAttribute {
                            offset: 24,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // 3. Camera Buffer (Initial)
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[0.0f32; 16]), // Identity/Zero
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // 4. Default Texture
        let white_texture = Arc::new(Texture::white_pixel(device, &ctx.queue, &texture_bind_group_layout));
        
        let vertex_capacity = 1024;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (vertex_capacity * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            _camera_bind_group_layout: camera_bind_group_layout,
            texture_bind_group_layout,
            camera_buffer,
            camera_bind_group,
            vertex_buffer,
            vertex_capacity,
            white_texture: white_texture.clone(),
            texture_cache: HashMap::new(),
            current_color: [1.0, 1.0, 1.0, 1.0],
            current_texture: white_texture,
            _current_shader: String::new(),
            vertices: Vec::with_capacity(vertex_capacity),
            batches: Vec::new(),
        }
    }

    pub fn resize(&mut self, ctx: &WgpuContext, width: u32, height: u32) {
        // Update projection matrix
        // Orthographic: 0..W, H..0 (Y-down? No. 0,0 is top-left.)
        // To map 0..W to -1..1 (x)
        // To map 0..H to 1..-1 (y) (Wait, y=0 is top -> 1. y=H is bottom -> -1)
        let proj = Mat4::orthographic_rh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        
        // wgpu uses column-major matrices. glam is column-major.
        let raw_matrix = proj.to_cols_array();
        ctx.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&raw_matrix));
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.current_color = color;
        // Revert to white texture
        self.current_texture = self.white_texture.clone();
    }

    pub fn set_picture(&mut self, ctx: &WgpuContext, path: &str) {
        // Load if not in cache
        if !self.texture_cache.contains_key(path) {
            // Synchronous load (users wanted simple code)
            if let Ok(img) = image::open(path) {
                 let texture = Texture::from_image(&ctx.device, &ctx.queue, &img, Some(path), &self.texture_bind_group_layout);
                 self.texture_cache.insert(path.to_string(), Arc::new(texture));
            } else {
                eprintln!("Failed to load image: {}", path);
                return;
            }
        }
        
        if let Some(tex) = self.texture_cache.get(path) {
            self.current_texture = tex.clone();
            self.current_color = [1.0, 1.0, 1.0, 1.0]; // Reset color to white so texture shows
        }
    }
    
    // Internal helper to start a new batch if needed
    fn ensure_batch(&mut self) {
        let texture_changed = if let Some(last) = self.batches.last() {
            !Arc::ptr_eq(&last.texture, &self.current_texture)
        } else {
            true
        };

        if texture_changed {
            let start = self.vertices.len() as u32;
            self.batches.push(DrawBatch {
                texture: self.current_texture.clone(),
                indices: start..start, // Empty initially
            });
        }
    }

    pub fn draw_triangle(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) {
        self.ensure_batch();
        
        // Push vertices
        // UVs? If we are in color mode, UVs don't matter (sample white pixel).
        // If in texture mode, user didn't provide UVs.
        // Auto-UV? Or require UVs in draw_triangle?
        // User said: "draw_triangle([x,y], [x,y], [x,y])". NO UVs mentioned.
        // Problem: How to map texture?
        // Maybe defaults: (0,0), (0,1), (1,1)?
        // Or user expects texture to map to the triangle's bounding box?
        // Or maybe set_picture isn't for UV mapping but for sprites?
        // Given simplicity, let's assume UVs are (0,0), (0,1), (1,0) for the 3 points or something standard.
        // Or maybe we map 0..1 based on screen? No.
        // Let's use 0,0 - 0,1 - 1,1 for now.
        
        let uvs = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0]]; // Arbitrary
        
        let v1 = Vertex { position: p1, color: self.current_color, tex_coords: uvs[0] };
        let v2 = Vertex { position: p2, color: self.current_color, tex_coords: uvs[1] };
        let v3 = Vertex { position: p3, color: self.current_color, tex_coords: uvs[2] };
        
        self.vertices.push(v1);
        self.vertices.push(v2);
        self.vertices.push(v3);
        
        // Update batch count
        if let Some(last) = self.batches.last_mut() {
            last.indices.end += 3;
        }
    }

    pub fn render(&mut self, ctx: &WgpuContext) {
        if self.vertices.is_empty() { return; }
        
        // Resize buffer if needed
        let needed_size = self.vertices.len() * std::mem::size_of::<Vertex>();
        if needed_size > (self.vertex_capacity * std::mem::size_of::<Vertex>()) {
             self.vertex_capacity = self.vertices.len().max(self.vertex_capacity * 2);
             self.vertex_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vertex Buffer"),
                size: (self.vertex_capacity * std::mem::size_of::<Vertex>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        
        // Write buffer
        ctx.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        
        let output = match ctx.surface.get_current_texture() {
            Ok(o) => o,
            Err(_) => return, // Handle error
        };
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });
        
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.camera_bind_group, &[]);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            
            for batch in &self.batches {
                rpass.set_bind_group(1, &batch.texture.bind_group, &[]);
                rpass.draw(batch.indices.clone(), 0..1);
            }
        }
        
        ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        // Clear for next frame
        self.vertices.clear();
        self.batches.clear();
        // Reset state? User persists state across frames in loop.
    }
}
