use graphics::draw_state::Blend;
use graphics::*;
use piston_components::components::PistonComponent;
use piston_components::components::buttons::{MyButton, Style};
use piston_ctx::{Ctx, CtxValues};
use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use piston_window::{graphics::Text, *};
use std::path::Path;
use wgpu_graphics::{Texture, TextureSettings};

pub fn testing_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |ctx| ctx.ctx = Ctx::Lobby,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);
        g.clear_stencil(0);

        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
            [0.0, 0.0, 100.0, 100.0],
            &c.draw_state,
            c.transform,
            g,
        );
        back_button.draw(&c, g, e, ctx);
        Rectangle::new([0.5, 1.0, 0.0, 0.3]).draw(
            [50.0, 50.0, 100.0, 100.0],
            &c.draw_state,
            c.transform,
            g,
        );
        let transform = c.transform.trans(200.0, 200.0);
        Ellipse::new([1.0, 0.0, 0.0, 1.0]).draw(
            [0.0, 0.0, 50.0, 50.0],
            &DrawState::new_clip(),
            transform,
            g,
        );
    });

    back_button.handle_event(e, ctx);
}
