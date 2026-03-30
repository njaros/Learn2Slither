use piston_components::components::buttons;
use std::path::Path;
use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use graphics::*;
use wgpu_graphics::{Texture, TextureSettings};
use piston_window::{graphics::Text, *};

use crate::{Ctx, CtxValues};

pub fn lobby(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {

	window.draw_2d(e, |c, g, _| {
		clear([0.8, 0.8, 0.8, 1.0], g);
		g.clear_stencil(0);

		let transform = c.transform.trans(10.0, 100.0);
	
		Rectangle::new([1.0, 0.0, 0.0, 1.0])
			.draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);
		text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
			"Hello world!",
			&mut ctx.glyphs,
			&c.draw_state,
			transform, g
		).unwrap();
	});
	
	if let Some(Button::Keyboard(Key::A)) = e.press_args() {
		println!("Welcome to Training board");
		ctx.ctx = Ctx::Train;
	}

	if let Some(Button::Keyboard(Key::S)) = e.press_args() {
		println!("Welcome to Testing board");
		ctx.ctx = Ctx::Test;
	}

	if let Some(Button::Keyboard(Key::P)) = e.press_args() {
		println!("Welcome to Playing board");
		ctx.ctx = Ctx::Play;
	}
}