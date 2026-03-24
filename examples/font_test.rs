use my_graphics;

fn main() {
    let mut graphics = my_graphics::new("Font Test", 800, 600);
    
    // Set a font family (standard across most OS)
    graphics.set_font("serif");

    loop {
        // Clear screen with a dark blue
        graphics.set_color([0.1, 0.1, 0.2, 1.0]);
        let (w, h) = graphics.get_size();
        draw_rect(&mut graphics, 0.0, 0.0, w as f32, h as f32);

        // Draw some text
        graphics.set_color([1.0, 1.0, 1.0, 1.0]); // White
        graphics.draw_char('H', [100.0, 100.0], 64.0);
        graphics.draw_char('e', [150.0, 100.0], 64.0);
        graphics.draw_char('l', [190.0, 100.0], 64.0);
        graphics.draw_char('l', [210.0, 100.0], 64.0);
        graphics.draw_char('o', [230.0, 100.0], 64.0);

        // Draw with different color
        graphics.set_color([1.0, 0.5, 0.0, 1.0]); // Orange
        graphics.draw_char('!', [300.0, 100.0], 64.0);

        graphics.update(41.66);
    }
}

fn draw_rect(g: &mut my_graphics::Graphics, x: f32, y: f32, w: f32, h: f32) {
    g.draw_triangle([x, y], [x + w, y], [x, y + h]);
    g.draw_triangle([x + w, y], [x + w, y + h], [x, y + h]);
}
