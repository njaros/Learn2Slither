use graphics::*;
use piston_components::components::{
    PistonComponent,
    buttons::{MyButton, Style},
    sliders::Slider,
};
use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use piston_window::{graphics::Text, *};
use playground::PlayGround;
use rand::make_rng;
use wgpu_graphics::{Texture, TextureSettings};

use piston_ctx::{Ctx, CtxValues};

fn build_playground(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {

    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "      CANCEL".into(),
        |ctx| {
            ctx.ctx = Ctx::Lobby;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        }
    );

    let mut create_button = MyButton::new(
        Style::GREEN,
        [532., 650., 200., 75.],
        "     CREATE".into(),
        |ctx| {
            ctx.playground = Some(PlayGround::new(
                ctx.selected_height,
                ctx.selected_width,
                make_rng(),
            ));
        },
    );

    let mut slider_width = Slider::new(10, 100, ctx.selected_width, [200., 200., 500., 60.], |ctx| {
        ctx.selected_width =
            Slider::cursor_to_current(ctx.last_mouse_pos.unwrap()[0], 60., 200., 500., 10, 100)
    });

    let mut slider_height =
        Slider::new(10, 100, ctx.selected_height, [200., 450., 500., 60.], |ctx| {
            ctx.selected_height =
                Slider::cursor_to_current(ctx.last_mouse_pos.unwrap()[0], 60., 200., 500., 10, 100)
        });

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let transform = c.transform.trans(210., 150.);
        Text::new(40).draw("Select the width (10 to 100)", &mut ctx.glyphs, &c.draw_state, transform, g).unwrap();
        slider_width.draw(&c, g, e, ctx);
        let transform = c.transform.trans(725., 240.);
        Text::new(40).draw(&ctx.selected_width.to_string(), &mut ctx.glyphs, &c.draw_state, transform, g).unwrap();
    
        let transform = c.transform.trans(210., 400.);
        Text::new(40).draw("Select the height (10 to 100)", &mut ctx.glyphs, &c.draw_state, transform, g).unwrap();
        slider_height.draw(&c, g, e, ctx);
        let transform = c.transform.trans(725., 490.);
        Text::new(40).draw(&ctx.selected_height.to_string(), &mut ctx.glyphs, &c.draw_state, transform, g).unwrap();

        create_button.draw(&c, g, e, ctx);
        back_button.draw(&c, g, e, ctx);
    });

    slider_width.handle_event(e, ctx);
    slider_height.handle_event(e, ctx);
    create_button.handle_event(e, ctx);
    back_button.handle_event(e, ctx);

}

fn play(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |ctx| {
            ctx.ctx = Ctx::Lobby;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        }
    );

    window.draw_2d(e, |c, g, _| {
        back_button.draw(&c, g, e, ctx);
        CircleArc::new([1.0, 0.0, 0.0, 1.0], 3.4, 20., 30.).draw(
            [30.0, 50.0, 100.0, 100.0],
            &c.draw_state,
            c.transform,
            g,
        );
    });

    back_button.handle_event(e, ctx);
}

pub fn playing_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match ctx.playground {
        None => build_playground(window, e, ctx),
        _ => play(window, e, ctx)
    }
}
