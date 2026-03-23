use my_graphics::Graphics;

fn main(){
	let mut graphic = my_graphics::new("mytest_1", 400, 300);
	loop{
		clean(&mut graphic, [0.5, 0.5, 0.5, 1.0]);
		graphic.set_color([1.0, 1.0, 1.0, 1.0]);
		graphic.draw_triangle(
		[0.0, 0.0],
		[0.0, 200.0],
		[200.0, 0.0]
		);
		graphic.set_color([1.0, 0.0, 0.0, 1.0]);
		graphic.draw_triangle(
		[200.0, 200.0],
		[0.0, 200.0],
		[200.0, 0.0]
		);
		draw_rectangle(&mut graphic, [200.0, 200.0, 200.0, 200.0]);
		graphic.update(32.0);
	}
}

fn clean(graphic: &mut Graphics, color:[f32; 4]){
	graphic.set_color(color);
	let (width, height) = graphic.get_size();
	draw_rectangle(graphic, [0.0, 0.0, width as f32, height as f32]);
}

fn draw_rectangle(graphic: &mut Graphics, [x, y, w, h]:[f32; 4]){
	graphic.draw_triangle(
	[x, y + h],
	[x, y],
	[x + w, y]
	);
	graphic.draw_triangle(
	[x + w, y],
	[x + w, y + h],
	[x, y + h]
	);
}