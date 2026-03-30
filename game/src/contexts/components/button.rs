use piston_window::*;

pub fn button(
	draw_state: &DrawState,
	g: &mut WgpuGraphics<'_>,
	button_shapes: [f32; 4],
	button_color: [f32; 4],
	border_button_color: [f32; 4],
	text: String,
	text_font_size: u32,
	text_color: [f32; 4]
) {
	Rectangle::new(button_color).draw(button_shapes, draw_state, c.transform, g);

}