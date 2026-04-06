use graphics::*;
use piston_components::app_params::app_params::{AppParams, Route};
use piston_components::components::PistonComponent;
use piston_components::components::buttons::{MyButton, Style};
use piston_window::*;

use crate::contexts::file_handler::get_model_names;

const MODEL_PATH: &str = "models";

pub fn lobby(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    let mut test_button = MyButton::new(
        Style::BLUE,
        [52., 650., 200., 75.],
        "     TESTING".into(),
        |app| {
            app.route = Route::Test;
            app.testing_params.model_names = get_model_names(MODEL_PATH);
        },
    );

    let mut train_button = MyButton::new(
        Style::GREEN,
        [292., 650., 200., 75.],
        "     TRAINING".into(),
        |app| app.route = Route::Train,
    );

    let mut play_button = MyButton::new(
        Style::BLUE,
        [532., 650., 200., 75.],
        "        PLAY".into(),
        |app| app.route = Route::Play,
    );

    let mut exit_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "        EXIT".into(),
        |app| app.exit = true,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        image(&app.logo, c.transform.zoom(2.), g);

        Text::new_color(color::OLIVE, 75)
            .draw(
                "LEARN",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(400., 100.),
                g,
            )
            .unwrap();
        Text::new_color(color::OLIVE, 75)
            .draw(
                "TO",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(450., 200.),
                g,
            )
            .unwrap();
        Text::new_color(color::OLIVE, 75)
            .draw(
                "SLITHER",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(390., 300.),
                g,
            )
            .unwrap();

        test_button.draw(&c, g, &e, app);
        train_button.draw(&c, g, &e, app);
        play_button.draw(&c, g, &e, app);
        exit_button.draw(&c, g, &e, app);
    });

    test_button.handle_event(e, app);
    train_button.handle_event(e, app);
    play_button.handle_event(e, app);
    exit_button.handle_event(e, app);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Escape) {
            app.exit = true;
        }
    }
}
