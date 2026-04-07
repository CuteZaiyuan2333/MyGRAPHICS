use my_graphics;

#[my_graphics::main(title = "my program", width = 800.0, height = 600.0)]
fn main(mut win:my_graphics::window::Window){
    win.push_color_stack([1.0, 1.0, 1.0, 1.0]);
    loop{
        clean(&mut win, [1.0, 1.0, 1.0, 1.0]);
        win.update(16);
    }
}

fn draw_rect(win: &mut my_graphics::window::Window, [x, y, w, h]:[f32; 4]){
    win.draw_triangle(
        [x    , y    ],
        [x    , y + h],
        [x + w, y + h]
    );
    win.draw_triangle(
        [x + w, y + h],
        [x + w, y    ],
        [x    , y    ]
    );
}

fn clean(win: &mut my_graphics::window::Window, color:[f32; 4]){
    win.push_color_stack(color);
    draw_rect(win, [0.0, 0.0, win.get_size()[0], win.get_size()[1]]);
    win.pull_color_stack();
}