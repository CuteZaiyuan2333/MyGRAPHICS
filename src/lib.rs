use winit::{
    event::*,
    event_loop::ControlFlow,
    platform::pump_events::EventLoopExtPumpEvents,
};
use std::time::{Duration, Instant};

mod context;
mod renderer;
mod texture;

pub use context::WgpuContext;
pub use renderer::Renderer;

pub fn new(title: &str, width: u32, height: u32) -> Graphics {
    Graphics::new(title, width, height)
}

pub struct Graphics {
    context: WgpuContext,
    renderer: Renderer,
    last_frame_time: Instant,
    should_close: bool,
}

impl Graphics {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        // Initialize logging
        env_logger::try_init().ok();

        // Block on async context creation
        let context = pollster::block_on(WgpuContext::new(title, width, height));
        let mut renderer = Renderer::new(&context);
        
        // Initial resize
        renderer.resize(&context, width, height);

        Self {
            context,
            renderer,
            last_frame_time: Instant::now(),
            should_close: false,
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let size = self.context.window.inner_size();
        (size.width, size.height)
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.renderer.set_color(color);
    }

    pub fn set_picture(&mut self, path: &str) {
        self.renderer.set_picture(&self.context, path);
    }

    pub fn draw_triangle(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) {
        self.renderer.draw_triangle(p1, p2, p3);
    }

    pub fn draw_char(&mut self, character: char, pos: [f32; 2], size: f32) {
        self.renderer.draw_char(character, pos, size);
    }

    pub fn set_font(&mut self, family: &str) {
        self.renderer.set_font(family);
    }

    pub fn set_font_path(&mut self, path: &str) {
        self.renderer.set_font_path(path);
    }

    pub fn update(&mut self, target_ms: f32) {
        if self.should_close {
            std::process::exit(0);
        }

        // 1. Render
        self.renderer.render(&self.context);

        // 2. Pump Events
        let mut resize_event = None;
        let mut close_requested = false;
        
        // We use a 0 timeout to poll events without blocking
        let _ = self.context.event_loop.pump_events(Some(Duration::ZERO), |event, target| {
            target.set_control_flow(ControlFlow::Poll);
            
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                         close_requested = true;
                         target.exit();
                    },
                    WindowEvent::Resized(new_size) => {
                        resize_event = Some(new_size);
                    },
                    WindowEvent::RedrawRequested => {
                        // Handled by our explicit render
                    },
                    _ => {}
                },
                _ => {}
            }
        });

        if close_requested {
            self.should_close = true;
             std::process::exit(0);
        }

        if let Some(size) = resize_event {
             if size.width > 0 && size.height > 0 {
                 self.context.config.width = size.width;
                 self.context.config.height = size.height;
                 self.context.surface.configure(&self.context.device, &self.context.config);
                 self.renderer.resize(&self.context, size.width, size.height);
             }
        }

        // 3. Frame Pacing
        let elapsed = self.last_frame_time.elapsed();
        let target_duration = Duration::from_millis(target_ms as u64);
        
        if elapsed < target_duration {
            std::thread::sleep(target_duration - elapsed);
        }
        
        self.last_frame_time = Instant::now();
    }
}
