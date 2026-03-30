use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use graphics::*;
use wgpu_graphics::{Texture, TextureSettings};
use piston_window::{graphics::Text, *};

use crate::{Ctx, CtxValues};

pub fn training_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
	window.draw_2d(e, |c, g, _| {
		clear([0.8, 0.8, 0.8, 1.0], g);
		g.clear_stencil(0);
		
		Rectangle::new([10.0, 31.0, 0.0, 1.0])
		.draw([30.0, 40.0, 100.0, 100.0], &c.draw_state, c.transform, g);
});
if let Some(Button::Keyboard(Key::L)) = e.press_args() {
		println!("Welcome to Training lobby");
		ctx.ctx = Ctx::Lobby;
	}
}