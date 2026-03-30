use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use graphics::*;
use graphics::draw_state::Blend;
use wgpu_graphics::{Texture, TextureSettings};
use piston_window::{graphics::Text, *};
use std::path::Path;

use crate::{Ctx, CtxValues};

pub fn testing_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
	let assets_path = Path::new("assets");

    let blends = [Blend::Alpha, Blend::Add, Blend::Invert, Blend::Multiply];
    let mut blend = 0;
    let mut clip_inside = true;
    // Cannot use the ? operator because the error management of Texture 
    // is badly implemented.
    let rust_logo = Texture::from_path(
        &mut window.create_texture_context(),
        assets_path.join("rust.png"),
        &TextureSettings::new()
    ).unwrap();
	
	window.draw_2d(e, |c, g, _| {
		clear([0.8, 0.8, 0.8, 1.0], g);
		g.clear_stencil(0);
	
		Rectangle::new([1.0, 0.0, 0.0, 1.0])
			.draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);

		let draw_state = c.draw_state.blend(blends[blend]);
		Rectangle::new([0.5, 1.0, 0.0, 0.3])
			.draw([50.0, 50.0, 100.0, 100.0], &draw_state, c.transform, g);

		let transform = c.transform.trans(100.0, 100.0);
		// Clip rectangle from upper left corner.
		let clipped = c.draw_state.scissor([100, 100, 100, 100]);
		Image::new().draw(&rust_logo, &clipped, transform, g);

		let transform = c.transform.trans(200.0, 200.0);
		Ellipse::new([1.0, 0.0, 0.0, 1.0])
			.draw([0.0, 0.0, 50.0, 50.0], &DrawState::new_clip(), transform, g);
		Image::new().draw(&rust_logo,
			&if clip_inside { DrawState::new_inside() }
			else { DrawState::new_outside() },
			transform, g);
		});

	if let Some(Button::Keyboard(Key::A)) = e.press_args() {
		blend = (blend + 1) % blends.len();
		println!("Changed blending to {:?}", blend);
	}

	if let Some(Button::Keyboard(Key::S)) = e.press_args() {
		clip_inside = !clip_inside;
		if clip_inside {
			println!("Changed to clip inside");
		} else {
			println!("Changed to clip outside");
		}
	}

	if let Some(Button::Keyboard(Key::L)) = e.press_args() {
		println!("Welcome to Lobby");
		ctx.ctx = Ctx::Lobby;
	}
}
