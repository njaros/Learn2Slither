use piston_window::graphics::{Context, Rectangle};
use piston_window::wgpu_graphics::WgpuGraphics;
use piston_ctx::{CtxValues};

pub fn button(
	c: &Context,
	g: &mut WgpuGraphics<'_>,
	ctx: &mut CtxValues,
	button_shapes: [f64; 4],
	button_color: [f32; 4],
	border_button_color: [f32; 4],
	text: String,
	text_font_size: u32,
	text_color: [f32; 4]
) {
	Rectangle::new(button_color).draw(button_shapes, &c.draw_state, c.transform, g);

}