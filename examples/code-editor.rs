use my_graphics::Graphics;

fn main(){
	let mut g = my_graphics::new("editor-function-example", 800, 600);
	let mut example_position = position{
		lt: [0.0, 0.0],
		rb: [1000.0, 1000.0],
		rt: [1.0, 0.0]
	};
	loop{
		g.set_color([0.1, 0.1, 0.2, 1.0]);
		draw_rect(&mut g, example_position.lt, example_position.rb);
		g.update(41.66);
	}
}

struct position{
	lt: [f32; 2],
	rb: [f32; 2],
	rt: [f32; 2]
}

fn draw_rect(g: &mut my_graphics::Graphics, [x, y]:[f32; 2], [w, h]:[f32; 2]) {
    g.draw_triangle([x, y], [x + w, y], [x, y + h]);
    g.draw_triangle([x + w, y], [x + w, y + h], [x, y + h]);
}