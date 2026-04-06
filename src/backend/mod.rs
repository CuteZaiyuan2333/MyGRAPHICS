use std::sync::Arc;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use winit::window::Window;
use crate::commands::DrawCmd;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

pub struct Backend {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    line_pipeline: wgpu::RenderPipeline,
    triangle_verts: Vec<Vertex>,
    line_verts: Vec<Vertex>,
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    current_frame: Vec<DrawCmd>,
    text_cache: HashMap<String, (glyphon::Buffer, [f32; 4])>,
    scale_factor: f32,
}

impl Backend {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
        }];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffers,
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
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffers,
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
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let mut atlas = glyphon::TextAtlas::new(&device, &queue, config.format);
        let text_renderer = glyphon::TextRenderer::new(&mut atlas, &device, wgpu::MultisampleState::default(), None);

        Self {
            device,
            queue,
            surface,
            config,
            pipeline,
            line_pipeline,
            triangle_verts: Vec::with_capacity(1024),
            line_verts: Vec::with_capacity(1024),
            font_system,
            swash_cache,
            atlas,
            text_renderer,
            current_frame: Vec::new(),
            text_cache: HashMap::new(),
            scale_factor,
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>, scale_factor: f32) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.scale_factor = scale_factor;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // 将 to_ndc 改为纯函数以避免借用冲突
    fn calc_ndc(p: [f32; 2], width: u32, height: u32, scale: f32) -> [f32; 2] {
        let px = p[0] * scale;
        let py = p[1] * scale;
        [
            (px / width as f32) * 2.0 - 1.0,
            1.0 - (py / height as f32) * 2.0,
        ]
    }

    pub fn set_frame(&mut self, cmds: Vec<DrawCmd>) {
        self.current_frame = cmds;
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(t) => t,
            Err(_) => return,
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.triangle_verts.clear();
        self.line_verts.clear();

        let width = self.config.width;
        let height = self.config.height;
        let scale = self.scale_factor;

        // 1. 预处理文字 (Mutable borrow phase)
        for cmd in &self.current_frame {
            if let DrawCmd::Text { text, color, .. } = cmd {
                let cache_entry = self.text_cache.entry(text.clone()).or_insert_with(|| {
                    let buffer = glyphon::Buffer::new(&mut self.font_system, glyphon::Metrics::new(16.0 * scale, 20.0 * scale));
                    (buffer, [0.0; 4])
                });
                
                if cache_entry.1 != *color {
                    let color_u32 = glyphon::Color::rgba(
                        (color[0] * 255.0) as u8,
                        (color[1] * 255.0) as u8,
                        (color[2] * 255.0) as u8,
                        (color[3] * 255.0) as u8,
                    );
                    cache_entry.0.set_text(&mut self.font_system, text, glyphon::Attrs::new().color(color_u32), glyphon::Shaping::Advanced);
                    cache_entry.0.set_size(&mut self.font_system, width as f32, height as f32);
                    cache_entry.1 = *color;
                }
            }
        }

        // 2. 解析几何体与文字区域 (Immutable borrow phase)
        let mut text_areas = Vec::new();
        for cmd in &self.current_frame {
            match cmd {
                DrawCmd::Triangle { verts, color } => {
                    for v in verts {
                        self.triangle_verts.push(Vertex { position: Self::calc_ndc(*v, width, height, scale), color: *color });
                    }
                }
                DrawCmd::Line { p1, p2, color } => {
                    self.line_verts.push(Vertex { position: Self::calc_ndc(*p1, width, height, scale), color: *color });
                    self.line_verts.push(Vertex { position: Self::calc_ndc(*p2, width, height, scale), color: *color });
                }
                DrawCmd::Bezier { p1, p2, p3, p4, color } => {
                    let segments = 24;
                    let mut prev_p = *p1;
                    for i in 1..=segments {
                        let t = i as f32 / segments as f32;
                        let nt = 1.0 - t;
                        let x = nt.powi(3) * p1[0] + 3.0 * nt.powi(2) * t * p2[0] + 3.0 * nt * t.powi(2) * p3[0] + t.powi(3) * p4[0];
                        let y = nt.powi(3) * p1[1] + 3.0 * nt.powi(2) * t * p2[1] + 3.0 * nt * t.powi(2) * p3[1] + t.powi(3) * p4[1];
                        let curr_p = [x, y];
                        self.line_verts.push(Vertex { position: Self::calc_ndc(prev_p, width, height, scale), color: *color });
                        self.line_verts.push(Vertex { position: Self::calc_ndc(curr_p, width, height, scale), color: *color });
                        prev_p = curr_p;
                    }
                }
                DrawCmd::Text { text, pos, .. } => {
                    let (buffer, _) = self.text_cache.get(text).unwrap();
                    text_areas.push(glyphon::TextArea {
                        buffer,
                        left: pos[0] * scale,
                        top: pos[1] * scale,
                        scale: 1.0,
                        bounds: glyphon::TextBounds {
                            left: 0,
                            top: 0,
                            right: width as i32,
                            bottom: height as i32,
                        },
                        default_color: glyphon::Color::rgb(255, 255, 255),
                    });
                }
            }
        }

        // 3. 准备渲染 (Final prepare)
        self.text_renderer.prepare(
            &self.device,
            &self.queue,
            &mut self.font_system,
            &mut self.atlas,
            glyphon::Resolution { width, height },
            text_areas,
            &mut self.swash_cache,
        ).unwrap();

        let tri_vbuf = if !self.triangle_verts.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.triangle_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else { None };

        let line_vbuf = if !self.line_verts.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.line_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else { None };

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
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

            if let Some(ref vbuf) = tri_vbuf {
                rpass.set_pipeline(&self.pipeline);
                rpass.set_vertex_buffer(0, vbuf.slice(..));
                rpass.draw(0..self.triangle_verts.len() as u32, 0..1);
            }

            if let Some(ref vbuf) = line_vbuf {
                rpass.set_pipeline(&self.line_pipeline);
                rpass.set_vertex_buffer(0, vbuf.slice(..));
                rpass.draw(0..self.line_verts.len() as u32, 0..1);
            }

            self.text_renderer.render(&self.atlas, &mut rpass).unwrap();
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.atlas.trim();
    }
}
