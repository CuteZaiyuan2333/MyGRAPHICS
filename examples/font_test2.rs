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
        
        draw_grid(&mut graphics, 32);
        
        // Draw some text
        // graphics.set_color([1.0, 1.0, 1.0, 1.0]); // White
        // graphics.draw_char('H', [100.0, 100.0], 64.0);
        // graphics.draw_char('e', [150.0, 100.0], 64.0);
        // graphics.draw_char('l', [190.0, 100.0], 64.0);
        // graphics.draw_char('l', [210.0, 100.0], 64.0);
        // graphics.draw_char('o', [230.0, 100.0], 64.0);

        // Draw with different color
        // graphics.set_color([1.0, 0.5, 0.0, 1.0]); // Orange
        // graphics.draw_char('!', [300.0, 100.0], 64.0);

        graphics.update(41.66);
    }
}

fn draw_rect(g: &mut my_graphics::Graphics, x: f32, y: f32, w: f32, h: f32) {
    g.draw_triangle([x, y], [x + w, y], [x, y + h]);
    g.draw_triangle([x + w, y], [x + w, y + h], [x, y + h]);
}

fn draw_grid(g: &mut my_graphics::Graphics, size: u32){
    let (w, h) = g.get_size();
    let (hn, vn) = (w/size, h/size);
    let (mut color, mut color2) = (0.0, 0.0);
    for n in 0..vn{
        g.set_color([color, color, color, 1.0]);
        if color == 0.0{
            color = 1.0;
        }else if color == 1.0{
            color = 0.0;
        }
        for m in 0..hn{
            draw_rect(g, 
            (m as f32) * (size as f32),
            (n as f32) * (size as f32),
            size as f32,
            size as f32
            );
            g.set_color([1.0, 0.0, 0.0, 1.0]);
            g.draw_char('H',[
                (m as f32) * (size as f32),
                (n as f32) * (size as f32)
            ], size as f32);
            if color2 == 0.0{
                color2 = 1.0;
            }else if color2 == 1.0{
                color2 = 0.0;
            }
            g.set_color([color2, color2, color2, 1.0]);
        }
        color2 = color;
    }
}