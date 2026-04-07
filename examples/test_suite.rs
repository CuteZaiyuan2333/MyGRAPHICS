use my_graphics::KeyCode;

#[my_graphics::main(title = "MyGRAPHICS Test Suite - 400x400", width = 400.0, height = 400.0)]
fn main(mut win: my_graphics::window::Window) {
    let mut frame_count = 0u64;
    let mut triangle_x = 200.0;
    
    loop {
        frame_count += 1;
        let t = frame_count as f32 * 0.02;

        //win.color_stack = [1.0, 1.0, 1.0, 1.0];
        win.push_color_stack([1.0, 1.0, 1.0, 1.0]);
        win.draw_text("MyGRAPHICS: Custom Size!", [20.0, 20.0]);
        win.pull_color_stack();
        
        if win.is_key_down(KeyCode::ArrowLeft) { triangle_x -= 5.0; }
        if win.is_key_down(KeyCode::ArrowRight) { triangle_x += 5.0; }
        
        //win.color_stack = [1.0, 0.5, 0.0, 1.0];
        win.push_color_stack([1.0, 0.5, 0.0, 1.0]);
        let y_offset = t.sin() * 50.0;
        win.draw_triangle(
            [triangle_x, 150.0 + y_offset], 
            [triangle_x + 40.0, 230.0 + y_offset], 
            [triangle_x - 40.0, 230.0 + y_offset]
        );
        win.pull_color_stack();

        if win.is_key_down(KeyCode::Escape) {
            break;
        }

        win.update(16);
    }
}
