pub mod window;
pub mod commands;
mod backend;
#[cfg(test)]
mod tests;

use std::sync::Arc;
use crossbeam_channel::unbounded;
use parking_lot::RwLock;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::window::Window;
use crate::commands::{RenderCmd, InputState};
use crate::backend::Backend;

pub use my_graphics_macros::main;
pub use winit::keyboard::KeyCode;

pub fn run<F>(user_main: F)
where
    F: FnOnce(Window) + Send + 'static,
{
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window_raw = Arc::new(WindowBuilder::new()
        .with_title("MyGRAPHICS")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap());

    let (cmd_tx, cmd_rx) = unbounded::<RenderCmd>();
    let input_state = Arc::new(RwLock::new(InputState::default()));
    
    let logical_size = window_raw.inner_size();
    let win_proxy = Window::new(
        cmd_tx, 
        input_state.clone(), 
        [logical_size.width as f32, logical_size.height as f32]
    );

    std::thread::spawn(move || {
        user_main(win_proxy);
    });

    let mut backend = pollster::block_on(Backend::new(window_raw.clone()));

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(size) => {
                    backend.resize(size, window_raw.scale_factor() as f32);
                }
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    backend.resize(window_raw.inner_size(), scale_factor as f32);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let sf = window_raw.scale_factor();
                    input_state.write().mouse_pos = [
                        (position.x / sf) as f32, 
                        (position.y / sf) as f32
                    ];
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                        if event.state == ElementState::Pressed {
                            input_state.write().keys_down.insert(code);
                        } else {
                            input_state.write().keys_down.remove(&code);
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    backend.render();
                }
                _ => {}
            },
            Event::AboutToWait => {
                while let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        RenderCmd::Frame(cmds) => {
                            backend.set_frame(cmds);
                            window_raw.request_redraw();
                        }
                        RenderCmd::Close => elwt.exit(),
                    }
                }
            }
            _ => {}
        }
    }).unwrap();
}
