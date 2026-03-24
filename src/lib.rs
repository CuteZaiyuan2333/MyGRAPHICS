use winit::{
    event::*,
    event_loop::ControlFlow,
    platform::pump_events::EventLoopExtPumpEvents,
    keyboard::PhysicalKey,
};
use std::time::{Duration, Instant};
use std::collections::HashSet;

mod context;
mod renderer;
mod texture;

pub use context::WgpuContext;
pub use renderer::Renderer;
pub use winit::keyboard::KeyCode;
pub use winit::event::MouseButton;

pub fn new(title: &str, width: u32, height: u32) -> Graphics {
    Graphics::new(title, width, height)
}

struct InputState {
    keys_down: HashSet<KeyCode>,
    last_key: Option<KeyCode>,
    mouse_pos: (f32, f32),
    mouse_buttons_down: HashSet<MouseButton>,
    mouse_wheel_delta: f32,
}

impl InputState {
    fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            last_key: None,
            mouse_pos: (0.0, 0.0),
            mouse_buttons_down: HashSet::new(),
            mouse_wheel_delta: 0.0,
        }
    }
}

pub struct Graphics {
    context: WgpuContext,
    renderer: Renderer,
    last_frame_time: Instant,
    should_close: bool,
    input: InputState,
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
            input: InputState::new(),
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

    // Input Getters
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.input.keys_down.contains(&key)
    }

    pub fn get_last_key(&self) -> Option<KeyCode> {
        self.input.last_key
    }

    pub fn get_pressed_keys(&self) -> Vec<KeyCode> {
        self.input.keys_down.iter().cloned().collect()
    }

    pub fn get_mouse_pos(&self) -> (f32, f32) {
        self.input.mouse_pos
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.input.mouse_buttons_down.contains(&button)
    }

    pub fn get_mouse_wheel(&self) -> f32 {
        self.input.mouse_wheel_delta
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
        
        // Reset per-frame input data
        self.input.mouse_wheel_delta = 0.0;

        let input_state = &mut self.input;
        
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
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let PhysicalKey::Code(code) = event.physical_key {
                            if event.state == ElementState::Pressed {
                                input_state.keys_down.insert(code);
                                input_state.last_key = Some(code);
                            } else {
                                input_state.keys_down.remove(&code);
                            }
                        }
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        input_state.mouse_pos = (position.x as f32, position.y as f32);
                    },
                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed {
                            input_state.mouse_buttons_down.insert(button);
                        } else {
                            input_state.mouse_buttons_down.remove(&button);
                        }
                    },
                    WindowEvent::MouseWheel { delta, .. } => {
                        match delta {
                            MouseScrollDelta::LineDelta(_, y) => {
                                input_state.mouse_wheel_delta += y;
                            },
                            MouseScrollDelta::PixelDelta(pos) => {
                                input_state.mouse_wheel_delta += pos.y as f32 / 32.0; // Arbitrary normalization
                            }
                        }
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
