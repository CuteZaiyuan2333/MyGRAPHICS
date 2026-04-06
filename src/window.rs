use crossbeam_channel::Sender;
use crate::commands::{DrawCmd, RenderCmd, SharedInput};
pub use winit::keyboard::KeyCode;

pub struct Window {
    pub color_stack: [f32; 4],
    cmd_tx: Sender<RenderCmd>,
    input: SharedInput,
    pub width: f32,
    pub height: f32,
    // 本地缓冲，实现你建议的合并提交
    frame_buffer: Vec<DrawCmd>,
}

impl Window {
    pub(crate) fn new(cmd_tx: Sender<RenderCmd>, input: SharedInput, size: [f32; 2]) -> Self {
        Self {
            color_stack: [1.0, 1.0, 1.0, 1.0],
            cmd_tx,
            input,
            width: size[0],
            height: size[1],
            frame_buffer: Vec::with_capacity(1024),
        }
    }

    pub fn draw_triangle(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) {
        self.frame_buffer.push(DrawCmd::Triangle {
            verts: [p1, p2, p3],
            color: self.color_stack,
        });
    }

    pub fn draw_text(&mut self, text: &str, pos: [f32; 2]) {
        self.frame_buffer.push(DrawCmd::Text {
            text: text.to_string(),
            pos,
            color: self.color_stack,
        });
    }

    pub fn draw_line(&mut self, p1: [f32; 2], p2: [f32; 2]) {
        self.frame_buffer.push(DrawCmd::Line {
            p1,
            p2,
            color: self.color_stack,
        });
    }

    pub fn draw_bezier(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2]) {
        self.frame_buffer.push(DrawCmd::Bezier {
            p1, p2, p3, p4,
            color: self.color_stack,
        });
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.input.read().keys_down.contains(&key)
    }

    pub fn get_mouse_pos(&self) -> [f32; 2] {
        self.input.read().mouse_pos
    }

    pub fn update(&mut self, ms: u64) {
        // 核心：在 update 时一次性合并提交
        if !self.frame_buffer.is_empty() {
            let frame = std::mem::replace(&mut self.frame_buffer, Vec::with_capacity(1024));
            self.cmd_tx.send(RenderCmd::Frame(frame)).ok();
        }
        
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
