use graphics::*;
use piston_components::components::PistonComponent;
use piston_components::components::buttons::{MyButton, Style};
use piston_ctx::{Ctx, CtxValues};
use piston_window::*;

use crate::contexts::file_handler::get_model_names;

const MODEL_PATH: &str = "models";

pub fn lobby(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut test_button = MyButton::new(
        Style::BLUE,
        [52., 650., 200., 75.],
        "     TESTING".into(),
        |ctx| {
            ctx.ctx = Ctx::Test;
            ctx.testing_params.model_names = get_model_names(MODEL_PATH);
        }
    );

    let mut train_button = MyButton::new(
        Style::GREEN,
        [292., 650., 200., 75.],
        "     TRAINING".into(),
        |ctx| ctx.ctx = Ctx::Train,
    );

    let mut play_button = MyButton::new(
        Style::BLUE,
        [532., 650., 200., 75.],
        "        PLAY".into(),
        |ctx| ctx.ctx = Ctx::Play,
    );

    let mut exit_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
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

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Escape) {
            ctx.exit = true;
        }
    }
}
