use graphics::*;
use piston_components::components::{
    PistonComponent,
    buttons::{MyButton, Style},
    sliders::Slider,
};
use piston_window::{
    graphics::Context,
    wgpu::{Color, Device, hal::Rect},
    wgpu_graphics::WgpuGraphics,
};
use piston_window::{graphics::Text, *};
use playground::{Dir, PlayGround, State, Tile};
use rand::make_rng;
use wgpu_graphics::{Texture, TextureSettings};

use piston_ctx::{Ctx, CtxValues};

use crate::contexts::board_helpers::Board;

fn build_playground(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "      CANCEL".into(),
        |ctx| {
            ctx.ctx = Ctx::Lobby;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        },
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

    let mut slider_width =
        Slider::new(10, 50, ctx.selected_width, [200., 200., 500., 60.], |ctx| {
            &mut ctx.selected_width
        });

    let mut slider_height = Slider::new(
        10,
        50,
        ctx.selected_height,
        [200., 450., 500., 60.],
        |ctx| &mut ctx.selected_height,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let transform = c.transform.trans(210., 150.);
        Text::new(40)
            .draw(
                "Select the width (10 to 50)",
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_width.draw(&c, g, e, ctx);
        let transform = c.transform.trans(725., 240.);
        Text::new(40)
            .draw(
                &ctx.selected_width.to_string(),
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        let transform = c.transform.trans(210., 400.);
        Text::new(40)
            .draw(
                "Select the height (10 to 50)",
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_height.draw(&c, g, e, ctx);
        let transform = c.transform.trans(725., 490.);
        Text::new(40)
            .draw(
                &ctx.selected_height.to_string(),
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

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
            ctx.playground = None;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        },
    );

    
    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);
    
        let playground = ctx.playground.as_ref().unwrap();
        let alive = playground.state == State::Alive;
        let score = playground.get_score();
    
        let board = Board::new(playground, alive);

        board.draw(&c, g);

        Rectangle::new(color::CYAN).draw([750., 3., 270., 763.], &c.draw_state, c.transform, g);

        Text::new(32)
            .draw_pos(
                "PLAYING BOARD",
                [780., 50.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(40)
            .draw_pos(
                &format!("Score: {}", score.to_string()),
                [780., 550.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "HEAD",
                [825., 150.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "BODY",
                [825., 200.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "WALL",
                [825., 250.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "UNSEEN",
                [825., 300.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "RED APPLE",
                [825., 350.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "GREEN APPLE",
                [825., 400.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Rectangle::new(color::OLIVE).draw([780., 120., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::PURPLE).draw([780., 170., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::hex("320000")).draw(
            [780., 220., 40., 40.],
            &c.draw_state,
            c.transform,
            g,
        );
        Rectangle::new(color::GRAY).draw([780., 270., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::RED).draw([780., 320., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::GREEN).draw([780., 370., 40., 40.], &c.draw_state, c.transform, g);

        back_button.draw(&c, g, e, ctx);
    });

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Right) || button == Button::Keyboard(Key::D) {
            ctx.playground.as_mut().unwrap().next(Dir::Right);
        } else if button == Button::Keyboard(Key::Down) || button == Button::Keyboard(Key::S) {
            ctx.playground.as_mut().unwrap().next(Dir::Down);
        } else if button == Button::Keyboard(Key::Up) || button == Button::Keyboard(Key::W) {
            ctx.playground.as_mut().unwrap().next(Dir::Up);
        } else if button == Button::Keyboard(Key::Left) || button == Button::Keyboard(Key::A) {
            ctx.playground.as_mut().unwrap().next(Dir::Left);
        }
    }

    back_button.handle_event(e, ctx);

}

pub fn playing_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match ctx.playground {
        None => build_playground(window, e, ctx),
        _ => play(window, e, ctx),
    }
}
