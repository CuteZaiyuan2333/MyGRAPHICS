#[cfg(test)]
mod tests {
    use crate::commands::{RenderCmd, DrawCmd};
    use crate::window::Window;
    use parking_lot::RwLock;
    use std::sync::Arc;
    use crossbeam_channel::unbounded;

    #[test]
    fn test_command_queue() {
        let (tx, rx) = unbounded();
        let input = Arc::new(RwLock::new(Default::default()));
        let mut win = Window::new(tx, input, [800.0, 600.0]);

        win.draw_triangle([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]);
        win.update(0); // 必须调用 update 才会发送指令
        
        let cmd = rx.try_recv().unwrap();
        if let RenderCmd::Frame(cmds) = cmd {
            if let DrawCmd::Triangle { verts, .. } = &cmds[0] {
                assert_eq!(verts[0], [0.0, 0.0]);
            } else {
                panic!("Expected Triangle draw command");
            }
        } else {
            panic!("Expected Frame command");
        }
    }

    #[test]
    fn test_color_stack() {
        let (tx, rx) = unbounded();
        let input = Arc::new(RwLock::new(Default::default()));
        let mut win = Window::new(tx, input, [800.0, 600.0]);

        win.color_stack = [1.0, 0.5, 0.2, 1.0];
        win.draw_line([0.0, 0.0], [10.0, 10.0]);
        win.update(0);

        let cmd = rx.try_recv().unwrap();
        if let RenderCmd::Frame(cmds) = cmd {
            if let DrawCmd::Line { color, .. } = &cmds[0] {
                assert_eq!(*color, [1.0, 0.5, 0.2, 1.0]);
            } else {
                panic!("Expected Line draw command");
            }
        } else {
            panic!("Expected Frame command");
        }
    }
}
