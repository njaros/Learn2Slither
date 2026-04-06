use graphics::*;
use piston_components::{app_params::app_params::{AppParams, Route}, components::{
    PistonComponent,
    buttons::{MyButton, Style},
    sliders::Slider,
}};
use piston_window::{graphics::Text, *};
use playground::{Dir, PlayGround, State};
use rand::make_rng;

use crate::contexts::board_helpers::Board;

fn build_playground(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "      CANCEL".into(),
        |app| {
            app.route = Route::Lobby;
            app.selected_height = 10;
            app.selected_width = 10;
        },
    );

    let mut create_button = MyButton::new(
        Style::GREEN,
        [532., 650., 200., 75.],
        "     CREATE".into(),
        |app| {
            app.playground = Some(PlayGround::new(
                app.selected_height,
                app.selected_width,
                make_rng(),
            ));
        },
    );

    let mut slider_width = Slider::new(5, 50, app.selected_width, [200., 200., 500., 60.], |app| {
        &mut app.selected_width
    });

    let mut slider_height =
        Slider::new(5, 50, app.selected_height, [200., 450., 500., 60.], |app| {
            &mut app.selected_height
        });

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let transform = c.transform.trans(210., 150.);
        Text::new(40)
            .draw(
                "Select the width (5 to 50)",
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_width.draw(&c, g, e, app);
        let transform = c.transform.trans(725., 240.);
        Text::new(40)
            .draw(
                &app.selected_width.to_string(),
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        let transform = c.transform.trans(210., 400.);
        Text::new(40)
            .draw(
                "Select the height (5 to 50)",
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_height.draw(&c, g, e, app);
        let transform = c.transform.trans(725., 490.);
        Text::new(40)
            .draw(
                &app.selected_height.to_string(),
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        create_button.draw(&c, g, e, app);
        back_button.draw(&c, g, e, app);
    });

    slider_width.handle_event(e, app);
    slider_height.handle_event(e, app);
    create_button.handle_event(e, app);
    back_button.handle_event(e, app);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Escape) {
            app.route = Route::Lobby;
            app.selected_width = 10;
            app.selected_height = 10;
        }
    }
}

fn playing_board(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |app| {
            app.route = Route::Lobby;
            app.playground = None;
            app.selected_height = 10;
            app.selected_width = 10;
        },
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let playground = app.playground.as_ref().unwrap();
        let alive = playground.state == State::Alive;
        let score = playground.get_score();

        let board = Board::new(playground, alive);

        board.draw(&c, g);

        Rectangle::new(color::CYAN).draw([750., 3., 270., 763.], &c.draw_state, c.transform, g);

        Text::new(32)
            .draw_pos(
                "PLAYING BOARD",
                [780., 50.],
                &mut app.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(40)
            .draw_pos(
                &format!("Score: {}", score.to_string()),
                [780., 550.],
                &mut app.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "HEAD",
                [825., 150.],
                &mut app.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "BODY",
                [825., 200.],
                &mut app.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "WALL",
                [825., 250.],
                &mut app.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "UNSEEN",
                [825., 300.],
                &mut app.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "RED APPLE",
                [825., 350.],
                &mut app.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "GREEN APPLE",
                [825., 400.],
                &mut app.glyphs,
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

        back_button.draw(&c, g, e, app);
    });

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Right) || button == Button::Keyboard(Key::D) {
            app.playground.as_mut().unwrap().next(Dir::Right);
        } else if button == Button::Keyboard(Key::Down) || button == Button::Keyboard(Key::S) {
            app.playground.as_mut().unwrap().next(Dir::Down);
        } else if button == Button::Keyboard(Key::Up) || button == Button::Keyboard(Key::W) {
            app.playground.as_mut().unwrap().next(Dir::Up);
        } else if button == Button::Keyboard(Key::Left) || button == Button::Keyboard(Key::A) {
            app.playground.as_mut().unwrap().next(Dir::Left);
        } else if button == Button::Keyboard(Key::Escape) {
            app.route = Route::Lobby;
            app.playground = None;
        }
    }

    back_button.handle_event(e, app);
}

pub fn playing_route(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    match app.playground {
        None => build_playground(window, e, app),
        _ => playing_board(window, e, app),
    }
}
