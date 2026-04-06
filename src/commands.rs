use parking_lot::RwLock;
use std::collections::HashSet;
use std::sync::Arc;
pub use winit::keyboard::KeyCode;

#[derive(Clone)]
pub enum DrawCmd {
    Triangle {
        verts: [[f32; 2]; 3],
        color: [f32; 4],
    },
    Line {
        p1: [f32; 2],
        p2: [f32; 2],
        color: [f32; 4],
    },
    Bezier {
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        p4: [f32; 2],
        color: [f32; 4],
    },
    Text {
        text: String,
        pos: [f32; 2],
        color: [f32; 4],
    },
}

pub enum RenderCmd {
    Frame(Vec<DrawCmd>),
    Close,
}

#[derive(Default)]
pub struct InputState {
    pub mouse_pos: [f32; 2],
    pub keys_down: HashSet<KeyCode>,
    pub mouse_down: [bool; 5],
}

pub type SharedInput = Arc<RwLock<InputState>>;
