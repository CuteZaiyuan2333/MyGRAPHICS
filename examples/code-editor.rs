use my_graphics;

fn main(){
	let mut g = my_graphics::new("editor-function-example", 800, 600);
	let example_position = Position{
		lt: [0.0, 0.0],
		rb: [500.0, 500.0]
	};
	loop{
		g.set_color([0.1, 0.1, 0.2, 1.0]);
		draw_rect(&mut g, example_position.lt, example_position.rb);
        draw_editor(&mut g, 10, 25, 16.0, 0, [0.0, 0.0]);
		g.update(41.66);
	}
}

struct Position{
	lt: [f32; 2],
	rb: [f32; 2]
}

fn draw_rect(g: &mut my_graphics::Graphics, [x, y]:[f32; 2], [w, h]:[f32; 2]) {
    g.draw_triangle([x, y], [x + w, y], [x, y + h]);
    g.draw_triangle([x + w, y], [x + w, y + h], [x, y + h]);
}

fn draw_line(g: &mut my_graphics::Graphics, index:u32, length:f32, size:f32, [x, y]:[f32; 2]){
    if index % 2 == 0{
        g.set_color([0.5, 0.5, 0.5, 1.0]);
    }else{
        g.set_color([0.25, 0.25, 0.25, 1.0]);
    }
    draw_rect(g, [x, y], [length, size]);
    if index % 2 == 0{
        g.set_color([0.0, 0.0, 0.0, 1.0]);
    }else{
        g.set_color([1.0, 1.0, 1.0, 1.0]);
    }
    let len = draw_u32_as_many_char_and_return_length(g, index, [x, y], size);
    if index % 2 == 0{
        g.set_color([0.5, 0.5, 0.5, 1.0]);
    }else{
        g.set_color([0.25, 0.25, 0.25, 1.0]);
    }
    draw_rect(g, [x + len as f32, y], [length - len as f32, size]);
}

fn draw_u32_as_many_char_and_return_length(g: &mut my_graphics::Graphics, input:u32, [x, y]:[f32; 2], size:f32) -> u32{
    let s = input.to_string();
    let len = s.len() as u32;
    for (i, c) in s.chars().enumerate(){
        let current_x = x + (i as f32 * size);
        g.draw_char(c, [current_x, y], size);
    }
    len
}

fn draw_editor(g: &mut my_graphics::Graphics, ver:u32, hor:u32, size:f32, num:u32, [x, y]:[f32; 2]){
    for i in 0..ver{
        draw_line(g, num + i, hor as f32 * size, size, [x, y + i as f32 * size]);
    }
}