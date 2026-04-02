use graphics::*;
use piston_components::components::{
    PistonComponent,
    buttons::{MyButton, Style},
};
use piston_ctx::{Ctx, CtxValues};
use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use piston_window::{graphics::Text, *};
use wgpu_graphics::{Texture, TextureSettings};

pub fn training_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |ctx| ctx.ctx = Ctx::Lobby,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        back_button.draw(&c, g, e, ctx);
        Rectangle::new([10.0, 31.0, 0.0, 1.0]).draw(
            [30.0, 40.0, 100.0, 100.0],
            &c.draw_state,
            c.transform,
            g,
        );
    });

    back_button.handle_event(e, ctx);

    if let Some(Button::Keyboard(Key::L)) = e.press_args() {
        println!("Welcome to Training lobby");
        ctx.ctx = Ctx::Lobby;
    }
}
