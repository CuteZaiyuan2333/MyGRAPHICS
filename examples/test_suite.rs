use my_graphics::KeyCode;

#[my_graphics::main]
fn main(mut win: my_graphics::window::Window) {
    let mut frame_count = 0u64;
    let mut triangle_x = 400.0;
    
    loop {
        frame_count += 1;
        let t = frame_count as f32 * 0.02;

        // --- 1. 测试文字渲染与颜色栈 ---
        win.color_stack = [1.0, 1.0, 1.0, 1.0]; // 白色
        win.draw_text("MyGRAPHICS Comprehensive Test Suite", [20.0, 20.0]);
        
        win.color_stack = [0.5, 0.5, 0.5, 1.0]; // 灰色
        win.draw_text("Controls: Arrow Keys to move triangle, Space to change color", [20.0, 50.0]);

        // --- 2. 测试按键输入与动态三角形 ---
        if win.is_key_down(KeyCode::ArrowLeft) { triangle_x -= 5.0; }
        if win.is_key_down(KeyCode::ArrowRight) { triangle_x += 5.0; }
        
        // 空格键按下时变为金色，否则为橙色
        if win.is_key_down(KeyCode::Space) {
            win.color_stack = [1.0, 0.84, 0.0, 1.0]; 
        } else {
            win.color_stack = [1.0, 0.5, 0.0, 1.0];
        }
        
        // 自动上下浮动的三角形
        let y_offset = t.sin() * 50.0;
        win.draw_triangle(
            [triangle_x, 150.0 + y_offset], 
            [triangle_x + 40.0, 230.0 + y_offset], 
            [triangle_x - 40.0, 230.0 + y_offset]
        );

        // --- 3. 测试线段渲染 (旋转射线) ---
        win.color_stack = [0.0, 1.0, 0.7, 1.0]; // 青色
        for i in 0..8 {
            let angle = t + (i as f32 * std::f32::consts::PI / 4.0);
            let x2 = 150.0 + angle.cos() * 80.0;
            let y2 = 400.0 + angle.sin() * 80.0;
            win.draw_line([150.0, 400.0], [x2, y2]);
        }
        win.draw_text("Rotation Test", [110.0, 490.0]);

        // --- 4. 测试鼠标输入与贝塞尔曲线 ---
        let mouse = win.get_mouse_pos();
        win.color_stack = [1.0, 0.3, 0.3, 1.0]; // 浅红色
        win.draw_text(&format!("Mouse: [{}, {}]", mouse[0] as i32, mouse[1] as i32), [600.0, 20.0]);

        // 绘制一条复杂的贝塞尔曲线，其中一个控制点跟随鼠标
        win.color_stack = [0.4, 0.6, 1.0, 1.0]; // 天蓝色
        win.draw_bezier(
            [400.0, 550.0], // P1: 起点
            [mouse[0], mouse[1]], // P2: 鼠标控制点
            [700.0, 100.0], // P3: 固定控制点
            [750.0, 550.0]  // P4: 终点
        );
        
        // 辅助线：显示鼠标到曲线起点的连接
        win.color_stack = [0.2, 0.2, 0.2, 1.0];
        win.draw_line([400.0, 550.0], [mouse[0], mouse[1]]);

        // --- 5. 退出检测 ---
        if win.is_key_down(KeyCode::Escape) {
            break;
        }

        // 刷新渲染，锁定约 60FPS
        win.update(16);
    }
}
