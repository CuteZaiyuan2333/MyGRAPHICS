use my_graphics::{self, KeyCode, MouseButton};

fn main() {
    let mut graphics = my_graphics::new("Input Test", 800, 600);
    
    loop {
        // Clear screen
        graphics.set_color([0.1, 0.1, 0.1, 1.0]);
        let (w, h) = graphics.get_size();
        draw_rect(&mut graphics, 0.0, 0.0, w as f32, h as f32);

        // Mouse Position
        let (mx, my) = graphics.get_mouse_pos();
        graphics.set_color([1.0, 1.0, 1.0, 1.0]);
        draw_text(&mut graphics, &format!("Mouse: ({:.1}, {:.1})", mx, my), 20.0, 20.0, 24.0);

        // Mouse Buttons
        if graphics.is_mouse_down(MouseButton::Left) {
            graphics.set_color([1.0, 0.0, 0.0, 1.0]);
            draw_text(&mut graphics, "Left Button Pressed", 20.0, 50.0, 24.0);
        }
        if graphics.is_mouse_down(MouseButton::Right) {
            graphics.set_color([0.0, 0.0, 1.0, 1.0]);
            draw_text(&mut graphics, "Right Button Pressed", 20.0, 80.0, 24.0);
        }

        // Last Key
        if let Some(key) = graphics.get_last_key() {
             graphics.set_color([0.0, 1.0, 0.0, 1.0]);
             draw_text(&mut graphics, &format!("Last Key: {:?}", key), 20.0, 110.0, 24.0);
        }

        // All Pressed Keys
        let keys = graphics.get_pressed_keys();
        if !keys.is_empty() {
            graphics.set_color([1.0, 1.0, 0.0, 1.0]);
            draw_text(&mut graphics, &format!("Keys Down: {:?}", keys), 20.0, 140.0, 24.0);
        }

        // Mouse Wheel
        let wheel = graphics.get_mouse_wheel();
        if wheel != 0.0 {
            println!("Wheel delta: {}", wheel);
        }

        // Draw a circle at mouse position (using triangles)
        graphics.set_color([0.0, 1.0, 1.0, 0.5]);
        draw_rect(&mut graphics, mx - 10.0, my - 10.0, 20.0, 20.0);

        graphics.update(16.0);
    }
}

fn draw_rect(g: &mut my_graphics::Graphics, x: f32, y: f32, w: f32, h: f32) {
    g.draw_triangle([x, y], [x + w, y], [x, y + h]);
    g.draw_triangle([x + w, y], [x + w, y + h], [x, y + h]);
}

fn draw_text(g: &mut my_graphics::Graphics, text: &str, x: f32, y: f32, size: f32) {
    let mut current_x = x;
    for c in text.chars() {
        g.draw_char(c, [current_x, y], size);
        current_x += size * 0.6; // Simple char spacing
    }
}
