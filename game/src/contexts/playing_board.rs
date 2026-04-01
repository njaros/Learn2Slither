use graphics::*;
use piston_components::components::{
    PistonComponent,
    buttons::{MyButton, Style}, sliders::Slider,
};
use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use piston_window::{graphics::Text, *};
use wgpu_graphics::{Texture, TextureSettings};

use piston_ctx::{Ctx, CtxValues};

pub fn playing_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [730., 650., 200., 75.],
        "       HOME".into(),
        |ctx| ctx.ctx = Ctx::Lobby,
    );

    // println!("{}", ctx.selected_width);

    match ctx.playground {
        None => {
            let mut slider = Slider::new(
                10,
                100,
                ctx.selected_width,
                [10., 10., 200., 30.],
                |ctx| ctx.selected_width = Slider::cursor_to_current(
                    ctx.last_mouse_pos.unwrap()[0],
                    30.,
                    10.,
                    10.,
                    200.,
                    10,
                    100
                )
            );
            window.draw_2d(e, |c, g, _| {
                clear([0.8, 0.8, 0.8, 1.0], g);
                slider.draw(&c, g, e, ctx);
                back_button.draw(&c, g, e, ctx);
            });
            slider.handle_event(e, ctx);
        },
        _ => {
            window.draw_2d(e, |c, g, _| {
                back_button.draw(&c, g, e, ctx);
                CircleArc::new([1.0, 0.0, 0.0, 1.0], 3.4, 20., 30.).draw(
                    [30.0, 50.0, 100.0, 100.0],
                    &c.draw_state,
                    c.transform,
                    g,
                );
            });
        }
    }

    

    back_button.handle_event(e, ctx);
}
