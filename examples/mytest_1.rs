use my_graphics::Graphics;

fn main(){
	let mut graphic = my_graphics::new("mytest_1", 400, 300);
	let (mut n, mut m) = (200.0, 200.0);
	let (mut p, mut q) = (1, 1);
	let (mut width, mut height);
	
	loop{
		(width, height) = graphic.get_size();
		clean(&mut graphic, [0.5, 0.5, 0.5, 1.0]);
		graphic.set_color([1.0, 1.0, 1.0, 1.0]);
		draw_rectangle(&mut graphic, [n, m, 200.0, 200.0]);
		graphic.set_color([0.0, 0.0, 0.0, 1.0]);
		graphic.draw_char('H', [n + 100.0, m + 100.0], 64.0);
		if n + 200.0 >= width as f32{
			p = -p;
		}else if n <= 0.0{
			p = -p;
		}
		
		if m + 200.0 >= height as f32{
			q = -q;
		}else if m <= 0.0{
			q = -q;
		}
		
		n = n + 5.0 * p as f32;
		m = m + 5.0 * q as f32;
		
		graphic.update(16.0);
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