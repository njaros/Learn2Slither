use piston::{Button, ButtonEvent, CursorEvent, Event, MouseCursorEvent, MouseRelativeEvent, mouse};
use piston_window::graphics::{Context, Rectangle};
use piston_window::wgpu_graphics::WgpuGraphics;
use piston_ctx::{CtxValues};

pub fn button(
	c: &Context,
	g: &mut WgpuGraphics<'_>,
	e: &Event,
	ctx: &mut CtxValues,
	button_shapes: [f64; 4],
	button_color: [f32; 4],
	border_button_color: [f32; 4],
	text: String,
	text_font_size: u32,
	text_color: [f32; 4],
	on_click: fn(ctx: &mut CtxValues)
) {
	Rectangle::new(button_color).draw(button_shapes, &c.draw_state, c.transform, g);
	let l_border = button_shapes[0];
	let r_border = l_border + button_shapes[2];
	let u_border = button_shapes[1];
	let d_border = u_border + button_shapes[3];
	if let Some([mouse_x, mouse_y]) = e.mouse_cursor_args() {
		if mouse_x > l_border && mouse_x < r_border && mouse_y > u_border && mouse_y < d_border {
			if e.button_args().unwrap().button == Button::Mouse(mouse::MouseButton::Left) {}
		}
	}
}