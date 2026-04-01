use graphics::*;
use piston_components::components::PistonComponent;
use piston_components::components::buttons::{MyButton, Style};
use piston_ctx::{Ctx, CtxValues};
use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use piston_window::{graphics::Text, *};
use std::path::Path;
use wgpu_graphics::{Texture, TextureSettings};

pub fn lobby(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut test_button = MyButton::new(
        Style::BLUE,
        [10., 650., 200., 75.],
        "     Testing".into(),
        |ctx| ctx.ctx = Ctx::Test,
    );

    let mut train_button = MyButton::new(
        Style::GREEN,
        [250., 650., 200., 75.],
        "     Training".into(),
        |ctx| ctx.ctx = Ctx::Train,
    );

    let mut play_button = MyButton::new(
        Style::BLUE,
        [490., 650., 200., 75.],
        "        Play".into(),
        |ctx| ctx.ctx = Ctx::Play,
    );

    let mut exit_button = MyButton::new(
        Style::RED,
        [730., 650., 200., 75.],
        "        EXIT".into(),
        |ctx| ctx.exit = true,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        test_button.draw(&c, g, &e, ctx);
        train_button.draw(&c, g, &e, ctx);
        play_button.draw(&c, g, &e, ctx);
        exit_button.draw(&c, g, &e, ctx);
    });

    test_button.handle_event(e, ctx);
    train_button.handle_event(e, ctx);
    play_button.handle_event(e, ctx);
    exit_button.handle_event(e, ctx);

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
