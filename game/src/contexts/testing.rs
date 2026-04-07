use convenient_lib::Res;
use graphics::*;
use piston_components::app_params::app_params::{AppParams, LeaderBoardItem, Route, TestingParams};
use piston_components::components::PistonComponent;
use piston_components::components::buttons::{ButtonActionFromVal, MyButton, Style};
use piston_components::components::sliders::{LerpMode, Slider, SliderVertical};
use piston_components::components::switch::NamedSwitch;
use piston_window::{graphics::Text, *};
use playground::PlayGround;
use qlearning::agent::agent::{Agent, Model};
use rand::make_rng;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::contexts::board_helpers::Board;
use crate::contexts::file_handler::{get_model, get_model_bests};

const MODEL_PATH: &str = "models";

fn test_form(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
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

    let mut test_button = MyButton::new(
        Style::GREEN,
        [532., 650., 200., 75.],
        "        TEST".into(),
        |app| {
            app.agent = match &app.testing_params.model {
                None => None,
                Some(m) => Some(Agent::from_model(None, m).unwrap()),
            }
        },
    );

    let mut slider_width = Slider::new(
        5,
        50,
        app.selected_width,
        [75., 180., 300., 40.],
        LerpMode::Linear,
        |app| &mut app.selected_width,
    );

    let mut slider_height = Slider::new(
        5,
        50,
        app.selected_height,
        [75., 330., 300., 40.],
        LerpMode::Linear,
        |app| &mut app.selected_height,
    );

    let mut models_buttons = match &app.testing_params.model_names {
        Err(_) => Vec::<ButtonActionFromVal<String, Res<Vec<String>>>>::new(),
        Ok(names) => names
            .iter()
            .enumerate()
            .map(|(idx, model_name)| {
                ButtonActionFromVal::new(
                    Style::BLUE,
                    [650., 200. + 75. * idx as f64, 250., 60.],
                    String::from("     ") + &model_name.clone(),
                    model_name.clone(),
                    |app| &mut app.testing_params.model_name,
                    |app| &mut app.testing_params.model_idx_list,
                    |app| {
                        app.testing_params.model = None;
                        app.testing_params.model_idx = "none".into();
                        get_model_bests(&Path::new(MODEL_PATH).join(&app.testing_params.model_name))
                    },
                )
            })
            .collect::<Vec<_>>(),
    };

    let mut models_overflow_y = SliderVertical::new(
        0,
        match &app.testing_params.model_names {
            Err(_) => 0,
            Ok(list) => match list.len() > 3 {
                true => list.len() - 3,
                false => 0,
            },
        },
        std::cmp::min(
            app.testing_params.model_cursor,
            match &app.testing_params.model_names {
                Err(_) => 0,
                Ok(list) => match list.len() > 3 {
                    true => list.len() - 3,
                    false => 0,
                },
            },
        ),
        [950., 200., 30., 210.],
        LerpMode::Linear,
        |app| &mut app.testing_params.model_cursor,
    );

    let mut bests_buttons = match &app.testing_params.model_idx_list {
        Err(_) => Vec::<ButtonActionFromVal<String, Option<Model>>>::new(),
        Ok(names) => names
            .iter()
            .enumerate()
            .map(|(idx, best_idx)| {
                ButtonActionFromVal::new(
                    Style::BLUE,
                    [650. + 75. * idx as f64, 450., 50., 50.],
                    String::from("  ") + &best_idx.clone(),
                    best_idx.clone(),
                    |app| &mut app.testing_params.model_idx,
                    |app| &mut app.testing_params.model,
                    |app| {
                        Some(
                            get_model(
                                &mut Path::new(MODEL_PATH)
                                    .join(&app.testing_params.model_name)
                                    .join(&app.testing_params.model_idx),
                            )
                            .unwrap(),
                        )
                    },
                )
            })
            .collect::<Vec<_>>(),
    };

    let mut bests_buttons_overflow_x = Slider::new(
        0,
        match &app.testing_params.model_idx_list {
            Err(_) => 0,
            Ok(list) => match list.len() > 4 {
                true => list.len() - 4,
                false => 0,
            },
        },
        std::cmp::min(
            app.testing_params.model_idx_cursor,
            match &app.testing_params.model_idx_list {
                Err(_) => 0,
                Ok(list) => match list.len() > 4 {
                    true => list.len() - 4,
                    false => 0,
                },
            },
        ),
        [655., 520., 265., 30.],
        LerpMode::Linear,
        |app| &mut app.testing_params.model_idx_cursor,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let transform = c.transform.trans(80., 150.);
        Text::new(28)
            .draw(
                "Select the width (5 to 50)",
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_width.draw(&c, g, e, app);
        let transform = c.transform.trans(385., 208.);
        Text::new(32)
            .draw(
                &app.selected_width.to_string(),
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        let transform = c.transform.trans(80., 300.);
        Text::new(28)
            .draw(
                "Select the height (5 to 50)",
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_height.draw(&c, g, e, app);
        let transform = c.transform.trans(385., 360.);
        Text::new(32)
            .draw(
                &app.selected_height.to_string(),
                &mut app.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        Text::new(32)
            .draw(
                "Select a model to test",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(650., 160.),
                g,
            )
            .unwrap();

        models_buttons
            .iter()
            .skip(app.testing_params.model_cursor)
            .take(3)
            .enumerate()
            .for_each(|(idx, m)| m.draw_pos([650., 200. + 75. * idx as f64], &c, g, e, app));

        match &app.testing_params.model_names {
            Err(_) => {}
            Ok(list) => match list.len() > 3 {
                true => models_overflow_y.draw(&c, g, e, app),
                false => {}
            },
        };

        bests_buttons
            .iter()
            .skip(app.testing_params.model_idx_cursor)
            .take(4)
            .enumerate()
            .for_each(|(idx, m)| m.draw_pos([650. + 75. * idx as f64, 450.], &c, g, e, app));

        match &app.testing_params.model_idx_list {
            Err(_) => {}
            Ok(list) => match list.len() > 4 {
                true => bests_buttons_overflow_x.draw(&c, g, e, app),
                false => {}
            },
        };

        match &app.testing_params.model {
            None => {}
            Some(m) => {
                Text::new(32)
                    .draw(
                        &format!("score reached: {}", m.score),
                        &mut app.glyphs,
                        &c.draw_state,
                        c.transform.trans(670., 585.),
                        g,
                    )
                    .unwrap();
                Text::new(32)
                    .draw(
                        &format!("from ets: {}", m.ets_name),
                        &mut app.glyphs,
                        &c.draw_state,
                        c.transform.trans(670., 625.),
                        g,
                    )
                    .unwrap();
            }
        }

        if app.testing_params.model.is_some() {
            test_button.draw(&c, g, e, app);
        }
        back_button.draw(&c, g, e, app);
    });

    slider_width.handle_event(e, app);
    slider_height.handle_event(e, app);

    match &app.testing_params.model_names {
        Err(_) => {}
        Ok(list) => match list.len() > 3 {
            true => models_overflow_y.handle_event(e, app),
            false => {}
        },
    };

    models_buttons
        .iter_mut()
        .skip(app.testing_params.model_cursor)
        .take(3)
        .enumerate()
        .for_each(|(idx, m)| m.handle_event_pos([650., 200. + 75. * idx as f64], e, app));

    match &app.testing_params.model_idx_list {
        Err(_) => {}
        Ok(list) => match list.len() > 4 {
            true => bests_buttons_overflow_x.handle_event(e, app),
            false => {}
        },
    };

    bests_buttons
        .iter_mut()
        .skip(app.testing_params.model_idx_cursor)
        .take(4)
        .enumerate()
        .for_each(|(idx, m)| m.handle_event_pos([650. + 75. * idx as f64, 450.], e, app));

    if app.testing_params.model.is_some() {
        test_button.handle_event(e, app);
    }
    back_button.handle_event(e, app);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Escape) {
            app.route = Route::Lobby;
            app.selected_height = 10;
            app.selected_width = 10;
        }
    }
}

fn testing_board(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |app| {
            app.route = Route::Lobby;
            app.agent = None;
            app.playground = None;
            app.selected_height = 10;
            app.selected_width = 10;
            app.testing_params = TestingParams::new() // reset params
        },
    );

    let mut view_switch = NamedSwitch::new(
        [720., 80.],
        150.,
        color::CYAN,
        "    View".into(),
        app.testing_params.snake_view,
        |app| &mut app.testing_params.snake_view,
    );

    let mut pause_switch = NamedSwitch::new(
        [720., 145.],
        100.,
        color::CYAN,
        " Pause".into(),
        app.testing_params.pause,
        |app| &mut app.testing_params.pause,
    );

    let mut infinite_loop_switch = NamedSwitch::new(
        [720., 210.],
        150.,
        color::CYAN,
        "  Infinite".into(),
        app.testing_params.infinite_loop,
        |app| &mut app.testing_params.infinite_loop,
    );

    let mut next_step_button = MyButton::new(
        Style::GREEN,
        [930., 145., 80., 50.],
        " Next".into(),
        |app| app.testing_params.next_step = true,
    );

    let mut speed_slider = Slider::new(
        0,
        1000,
        app.testing_params.speed_time,
        [820., 275., 130., 40.],
        LerpMode::Linear,
        |app| &mut app.testing_params.speed_time,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let board = Board::new(
            &app.playground.as_ref().unwrap(),
            app.testing_params.snake_view,
        );

        board.draw(&c, g);

        Rectangle::new(color::BLACK).draw([720., 10., 300., 60.], &c.draw_state, c.transform, g);
        Rectangle::new_border(color::CYAN, 1.).draw(
            [720., 10., 300., 60.],
            &c.draw_state,
            c.transform,
            g,
        );

        Text::new_color(color::WHITE, 28)
            .draw(
                &format!("On testing: {}", app.testing_params.model_name),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(740., 45.),
                g,
            )
            .unwrap();

        pause_switch.draw(&c, g, e, app);
        infinite_loop_switch.draw(&c, g, e, app);

        if app.testing_params.pause {
            next_step_button.draw(&c, g, e, app);
        }

        view_switch.draw(&c, g, e, app);
        Text::new(28)
            .draw(
                " Speed",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(720., 305.),
                g,
            )
            .unwrap();
        speed_slider.draw(&c, g, e, app);
        Text::new(28)
            .draw(
                &app.testing_params.speed_time.to_string(),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(960., 305.),
                g,
            )
            .unwrap();

        Rectangle::new(color::BLACK).draw([710., 330., 300., 300.], &c.draw_state, c.transform, g);
        Rectangle::new_border(color::WHITE, 0.5).draw(
            [714., 334., 293., 35.],
            &c.draw_state,
            c.transform,
            g,
        );
        Text::new_color(color::WHITE, 16)
            .draw(
                &format!(
                    "current score: {}",
                    app.playground.as_ref().unwrap().get_score()
                ),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(722., 356.),
                g,
            )
            .unwrap();
        Rectangle::new_border(color::WHITE, 0.5).draw(
            [714., 373., 293., 253.],
            &c.draw_state,
            c.transform,
            g,
        );
        Text::new_color(color::WHITE, 16)
            .draw(
                &"LeaderBoard (for 10*10 maps only):",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(725., 395.),
                g,
            )
            .unwrap();

        app.leaderboard
            .leaderboard
            .iter()
            .enumerate()
            .for_each(|(idx, item)| {
                Text::new_color(color::WHITE, 16)
                    .draw(
                        &format!(
                            "{}: score: {}  {} ets: {}",
                            idx + 1,
                            item.score,
                            item.model_name,
                            item.ets_name
                        ),
                        &mut app.glyphs,
                        &c.draw_state,
                        c.transform.trans(740., 425. + (20. * idx as f64)),
                        g,
                    )
                    .unwrap();
            });

        back_button.draw(&c, g, e, app);
    });

    let playground = app.playground.as_mut().unwrap();
    let agent = app.agent.as_mut().unwrap();

    if match app.testing_params.pause {
        false => {
            app.last_training_frame.elapsed()
                > Duration::from_millis(app.testing_params.speed_time as u64)
        }
        true => app.testing_params.next_step,
    } {
        app.last_training_frame = Instant::now();
        app.testing_params.next_step = false;
        if playground.is_alive() {
            let env = &playground.snake_view();
            let state = agent.ets.env_to_state(env);
            let dir = agent.play(state, true);
            playground.next(dir);
            if !playground.is_alive() {
                println!("{playground}");
                println!("\nDEAD ! score: {}\n", playground.get_score());
                if app.selected_height == 10 && app.selected_width == 10 {
                    if app.leaderboard.leaderboard.len() < 10 {
                        app.leaderboard.leaderboard.push(LeaderBoardItem {
                            score: playground.get_score(),
                            model_name: app.agent.as_ref().unwrap().name.clone(),
                            ets_name: app.agent.as_ref().unwrap().ets.get_name(),
                        });
                        app.leaderboard
                            .leaderboard
                            .sort_by(|a, b| b.score.cmp(&a.score));
                    } else {
                        match app.leaderboard.leaderboard[9].score < playground.get_score() {
                            true => {
                                app.leaderboard.leaderboard.remove(9);
                                app.leaderboard.leaderboard.push(LeaderBoardItem {
                                    score: playground.get_score(),
                                    model_name: app.agent.as_ref().unwrap().name.clone(),
                                    ets_name: app.agent.as_ref().unwrap().ets.get_name(),
                                });
                                app.leaderboard
                                    .leaderboard
                                    .sort_by(|a, b| b.score.cmp(&a.score));
                            }
                            false => {}
                        }
                    }
                }
                if !app.testing_params.infinite_loop {
                    app.testing_params.pause = true;
                }
            } else {
                playground.print_snake_view();
            }
        } else {
            app.playground = Some(PlayGround::new(
                app.selected_height,
                app.selected_width,
                make_rng(),
            ));
            app.playground.as_ref().unwrap().print_snake_view();
        }
    }

    pause_switch.handle_event(e, app);
    if app.testing_params.pause {
        next_step_button.handle_event(e, app);
    }
    speed_slider.handle_event(e, app);
    back_button.handle_event(e, app);
    view_switch.handle_event(e, app);
    infinite_loop_switch.handle_event(e, app);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Space) {
            app.testing_params.pause = !app.testing_params.pause;
        } else if button == Button::Keyboard(Key::Down) || button == Button::Keyboard(Key::Right) {
            app.testing_params.speed_time = std::cmp::min(
                1000,
                match app.testing_params.speed_time {
                    0 => 1,
                    _ => app.testing_params.speed_time * 2,
                },
            )
        } else if button == Button::Keyboard(Key::Up) || button == Button::Keyboard(Key::Left) {
            app.testing_params.speed_time = match app.testing_params.speed_time {
                1 => 0,
                _ => app.testing_params.speed_time / 2,
            }
        } else if button == Button::Keyboard(Key::V) {
            app.testing_params.snake_view = !app.testing_params.snake_view;
        } else if app.testing_params.pause
            && (button == Button::Keyboard(Key::N)
                || button == Button::Keyboard(Key::Return)
                || button == Button::Keyboard(Key::Return2))
        {
            app.testing_params.next_step = true;
        } else if button == Button::Keyboard(Key::Escape) {
            app.route = Route::Lobby;
            app.agent = None;
            app.playground = None;
            app.selected_height = 10;
            app.selected_width = 10;
            app.testing_params = TestingParams::new();
        }
    }
}

pub fn testing_route(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    match app.agent {
        Some(_) => match app.playground {
            Some(_) => testing_board(window, e, app),
            None => {
                app.playground = Some(PlayGround::new(
                    app.selected_height,
                    app.selected_width,
                    make_rng(),
                ));
                app.playground.as_ref().unwrap().print_snake_view()
            }
        },
        None => test_form(window, e, app),
    }
}
