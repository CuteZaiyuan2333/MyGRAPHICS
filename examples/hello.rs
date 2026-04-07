use my_graphics::KeyCode;

#[my_graphics::main]
fn main(mut win: my_graphics::window::Window) {
    let mut x = 100.0;
    loop {
        if win.is_key_down(KeyCode::ArrowRight) {
            x += 5.0;
        }
        if win.is_key_down(KeyCode::ArrowLeft) {
            x -= 5.0;
        }

        // 绘制三角形
        win.push_color_stack([1.0, 0.0, 0.0, 1.0]);
        win.draw_triangle([x, 100.0], [x + 50.0, 100.0], [x + 25.0, 150.0]);
        win.pull_color_stack();

        // 绘制文字
        win.push_color_stack([1.0, 1.0, 1.0, 1.0]);
        win.draw_text(&format!("Current X: {:.1}", x), [x, 80.0]);
        win.pull_color_stack();

        let size = win.get_size();
        win.draw_text(&format!("Window size: {:.0}x{:.0}", size[0], size[1]), [10.0, 30.0]);

        win.update(16);
    }
}
