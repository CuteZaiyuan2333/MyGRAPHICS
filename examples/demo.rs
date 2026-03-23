use my_graphics;

fn main() {
    let mut graphics = my_graphics::new("MyGRAPHICS Demo", 800, 600);
    let mut angle = 0.0f32;

    loop {
        let (w, h) = graphics.get_size();
        let w = w as f32;
        let h = h as f32;
        
        angle += 0.01;
        
        // Triangle 1: Red
        graphics.set_color([1.0, 0.0, 0.0, 1.0]);
        let offset = angle.sin() * 50.0;
        
        graphics.draw_triangle(
            [w / 2.0 + offset, 50.0],
            [50.0, h - 50.0],
            [w - 50.0, h - 50.0]
        );
        
        // Triangle 2: Blue, moving
        graphics.set_color([0.0, 0.0, 1.0, 0.8]);
        graphics.draw_triangle(
            [w / 2.0, h / 2.0],
            [w / 2.0 + 100.0, h / 2.0 + 100.0],
            [w / 2.0 - 100.0, h / 2.0 + 100.0]
        );
        
        graphics.update(16.0);
    }
}
