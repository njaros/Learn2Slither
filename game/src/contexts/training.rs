use crate::contexts::{
    board_helpers::Board,
    file_handler::{get_model, get_model_bests, get_model_names},
};
use convenient_lib::Res;
use graphics::*;
use piston_components::{
    app_params::app_params::{AppParams, Route, TrainingParams},
    components::{
        PistonComponent,
        buttons::{ButtonActionFromVal, ButtonStoreVal, MyButton, Style},
        sliders::{Slider, SliderVertical},
        switch::{NamedSwitch, NamedSwitchAction},
        text_area::TextArea,
    },
};
use piston_window::{
    graphics::{Rectangle, Text},
    *,
};
use playground::{Dir, PlayGround};
use qlearning::{
    agent::agent::{Agent, Model},
    state::list_all_ets,
    train_loop,
};
use rand::make_rng;
use std::{
    path::Path,
    time::{Duration, Instant},
};

const MODEL_PATH: &str = "models";

fn dir_to_usize(dir: Dir) -> usize {
    match dir {
        Dir::Up => 0,
        Dir::Right => 1,
        Dir::Down => 2,
        Dir::Left => 3,
    }
}

fn training_form<'a>(window: &mut PistonWindow, e: &Event, app: &'a mut AppParams) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "     CANCEL".into(),
        |app| app.route = Route::Lobby,
    );

    let mut train_button = MyButton::new(
        Style::GREEN,
        [532., 650., 200., 75.],
        "      TRAIN".into(),
        |app| {
            app.last_training_frame = Instant::now();
            app.agent = match app.training_params.from_bool {
                true => match &app.training_params.from_model {
                    None => None,
                    Some(m) => Some(Agent::from_model(Some(&app.training_params.name), m).unwrap()),
                },
                false => match &app.training_params.ets {
                    None => None,
                    Some(ets) => {
                        Some(Agent::new(ets.clone(), app.training_params.name.clone()).unwrap())
                    }
                },
            }
        },
    );

    let mut ets_buttons = list_all_ets()
        .iter()
        .enumerate()
        .map(|(idx, ets_name)| {
            ButtonStoreVal::new(
                Style::BLUE,
                [650., 200. + 75. * idx as f64, 250., 60.],
                String::from("     ") + &ets_name.clone(),
                Some(ets_name.clone()),
                |app| &mut app.training_params.ets,
            )
        })
        .collect::<Vec<_>>();

    let mut ets_overflow_y = SliderVertical::new(
        0,
        match app.training_params.ets_list.len() > 5 {
            true => app.training_params.ets_list.len() - 5,
            false => 0,
        },
        std::cmp::min(
            app.training_params.ets_cursor,
            match app.training_params.ets_list.len() > 3 {
                true => app.training_params.ets_list.len() - 5,
                false => 0,
            },
        ),
        [950., 200., 30., 360.],
        |app| &mut app.training_params.ets_cursor,
    );

    let mut models_buttons = match &app.training_params.from_model_names {
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
                    |app| &mut app.training_params.from_model_name,
                    |app| &mut app.training_params.from_model_idx_list,
                    |app| {
                        app.training_params.from_model = None;
                        app.training_params.from_model_idx = "none".into();
                        get_model_bests(
                            &Path::new(MODEL_PATH).join(&app.training_params.from_model_name),
                        )
                    },
                )
            })
            .collect::<Vec<_>>(),
    };

    let mut models_overflow_y = SliderVertical::new(
        0,
        match &app.training_params.from_model_names {
            Err(_) => 0,
            Ok(list) => match list.len() > 3 {
                true => list.len() - 3,
                false => 0,
            },
        },
        std::cmp::min(
            app.training_params.from_model_cursor,
            match &app.training_params.from_model_names {
                Err(_) => 0,
                Ok(list) => match list.len() > 3 {
                    true => list.len() - 3,
                    false => 0,
                },
            },
        ),
        [950., 200., 30., 210.],
        |app| &mut app.training_params.from_model_cursor,
    );

    let mut bests_buttons = match &app.training_params.from_model_idx_list {
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
                    |app| &mut app.training_params.from_model_idx,
                    |app| &mut app.training_params.from_model,
                    |app| {
                        Some(
                            get_model(
                                &mut Path::new(MODEL_PATH)
                                    .join(&app.training_params.from_model_name)
                                    .join(&app.training_params.from_model_idx),
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
        match &app.training_params.from_model_idx_list {
            Err(_) => 0,
            Ok(list) => match list.len() > 4 {
                true => list.len() - 4,
                false => 0,
            },
        },
        std::cmp::min(
            app.training_params.from_model_idx_cursor,
            match &app.training_params.from_model_idx_list {
                Err(_) => 0,
                Ok(list) => match list.len() > 4 {
                    true => list.len() - 4,
                    false => 0,
                },
            },
        ),
        [655., 520., 265., 30.],
        |app| &mut app.training_params.from_model_idx_cursor,
    );

    let mut from_switch = NamedSwitchAction::new(
        [600., 50.],
        300.,
        color::MAGENTA,
        "        from existing".into(),
        app.training_params.from_bool,
        |app| &mut app.training_params.from_bool,
        |app| &mut app.training_params.from_model_names,
        |_| get_model_names(MODEL_PATH),
        |app| &mut app.training_params.ets_list,
        |_| list_all_ets(),
    );

    let mut interactive_switch = NamedSwitch::new(
        [50., 50.],
        300.,
        color::MAGENTA,
        " interactive".into(),
        app.training_params.interactive,
        |app| &mut app.training_params.interactive,
    );

    let mut pause_switch = NamedSwitch::new(
        [50., 200.],
        300.,
        color::CYAN,
        " pause".into(),
        app.training_params.pause,
        |app| &mut app.training_params.pause,
    );

    let mut snake_view_switch = NamedSwitch::new(
        [50., 270.],
        300.,
        color::CYAN,
        " snake view".into(),
        app.training_params.snake_view,
        |app| &mut app.training_params.snake_view,
    );

    let mut speed_slider = Slider::new(
        0,
        1000,
        app.training_params.speed_time,
        [350., 340., 150., 50.],
        |app| &mut app.training_params.speed_time,
    );

    let mut rounds_slider = Slider::new(
        1,
        100000,
        app.training_params.rounds,
        [350., 500., 150., 50.],
        |app| &mut app.training_params.rounds,
    );

    let mut name_text_area = TextArea::new(
        [350., 575.],
        7,
        36,
        app.training_params.name.clone(),
        |app| &mut app.training_params.name,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        from_switch.draw(&c, g, e, app);
        if app.training_params.from_bool {
            Text::new(32)
                .draw(
                    "Model to train from",
                    &mut app.glyphs,
                    &c.draw_state,
                    c.transform.trans(650., 160.),
                    g,
                )
                .unwrap();

            models_buttons
                .iter()
                .skip(app.training_params.from_model_cursor)
                .take(3)
                .enumerate()
                .for_each(|(idx, m)| m.draw_pos([650., 200. + 75. * idx as f64], &c, g, e, app));

            match &app.training_params.from_model_names {
                Err(_) => {}
                Ok(list) => match list.len() > 3 {
                    true => models_overflow_y.draw(&c, g, e, app),
                    false => {}
                },
            };

            bests_buttons
                .iter()
                .skip(app.training_params.from_model_idx_cursor)
                .take(4)
                .enumerate()
                .for_each(|(idx, m)| m.draw_pos([650. + 75. * idx as f64, 450.], &c, g, e, app));

            match &app.training_params.from_model_idx_list {
                Err(_) => {}
                Ok(list) => match list.len() > 4 {
                    true => bests_buttons_overflow_x.draw(&c, g, e, app),
                    false => {}
                },
            };

            match &app.training_params.from_model {
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
        } else {
            Text::new(32)
                .draw(
                    "Train algorithm",
                    &mut app.glyphs,
                    &c.draw_state,
                    c.transform.trans(650., 160.),
                    g,
                )
                .unwrap();

            ets_buttons
                .iter()
                .skip(app.training_params.ets_cursor)
                .take(5)
                .enumerate()
                .for_each(|(idx, b)| b.draw_pos([650., 200. + 75. * idx as f64], &c, g, e, app));

            if app.training_params.ets_list.len() > 5 {
                ets_overflow_y.draw(&c, g, e, app);
            }
        }
        Rectangle::new(color::CYAN).draw([50., 500., 300., 50.], &c.draw_state, c.transform, g);
        Text::new(32)
            .draw(
                " number of rounds",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(60., 535.),
                g,
            )
            .unwrap();
        rounds_slider.draw(&c, g, e, app);
        Text::new(32)
            .draw(
                &app.training_params.rounds.to_string(),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(510., 535.),
                g,
            )
            .unwrap();

        Rectangle::new(color::CYAN).draw([50., 575., 300., 50.], &c.draw_state, c.transform, g);
        Text::new(32)
            .draw(
                " model name",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(60., 610.),
                g,
            )
            .unwrap();
        name_text_area.draw(&c, g, e, app);

        // Interactive buttons.
        interactive_switch.draw(&c, g, e, app);
        if app.training_params.interactive {
            snake_view_switch.draw(&c, g, e, app);
            Rectangle::new(color::CYAN).draw([50., 340., 300., 50.], &c.draw_state, c.transform, g);
            Text::new(32)
                .draw(
                    "frame period(ms)",
                    &mut app.glyphs,
                    &c.draw_state,
                    c.transform.trans(60., 375.),
                    g,
                )
                .unwrap();
            speed_slider.draw(&c, g, e, app);
            Text::new(32)
                .draw(
                    &(app.training_params.speed_time.to_string()),
                    &mut app.glyphs,
                    &c.draw_state,
                    c.transform.trans(510., 375.),
                    g,
                )
                .unwrap();
            pause_switch.draw(&c, g, e, app);
        }

        if app.training_params.name.len() > 0
            && ((app.training_params.from_bool && app.training_params.from_model.is_some())
            || (!app.training_params.from_bool && app.training_params.ets.is_some()))
        {
            train_button.draw(&c, g, e, app);
        }

        back_button.draw(&c, g, e, app);
    });

    from_switch.handle_event(e, app);
    if app.training_params.from_bool {
        match &app.training_params.from_model_names {
            Err(_) => {}
            Ok(list) => match list.len() > 3 {
                true => models_overflow_y.handle_event(e, app),
                false => {}
            },
        };

        models_buttons
            .iter_mut()
            .skip(app.training_params.from_model_cursor)
            .take(3)
            .enumerate()
            .for_each(|(idx, m)| m.handle_event_pos([650., 200. + 75. * idx as f64], e, app));

        match &app.training_params.from_model_idx_list {
            Err(_) => {}
            Ok(list) => match list.len() > 4 {
                true => bests_buttons_overflow_x.handle_event(e, app),
                false => {}
            },
        };

        bests_buttons
            .iter_mut()
            .skip(app.training_params.from_model_idx_cursor)
            .take(4)
            .enumerate()
            .for_each(|(idx, m)| m.handle_event_pos([650. + 75. * idx as f64, 450.], e, app));
    } else {
        ets_buttons
            .iter_mut()
            .skip(app.training_params.ets_cursor)
            .take(5)
            .enumerate()
            .for_each(|(idx, b)| b.handle_event_pos([650., 200. + 75. * idx as f64], e, app));
        if app.training_params.ets_list.len() > 5 {
            ets_overflow_y.handle_event(e, app);
        }
    }

    interactive_switch.handle_event(e, app);
    if app.training_params.interactive {
        pause_switch.handle_event(e, app);
        snake_view_switch.handle_event(e, app);
        speed_slider.handle_event(e, app);
    }

    rounds_slider.handle_event(e, app);
    name_text_area.handle_event(e, app);

    if app.training_params.name.len() > 0
        && ((app.training_params.from_bool && app.training_params.from_model.is_some())
        || (!app.training_params.from_bool && app.training_params.ets.is_some()))
    {
        train_button.handle_event(e, app);
    }
    back_button.handle_event(e, app);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Escape) {
            app.route = Route::Lobby;
        }
    }
}

fn training_board(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |app| {
            app.route = Route::Lobby;
            app.agent = None;
            app.playground = None;
            app.training_params = TrainingParams::new() // reset params
        },
    );

    let mut view_switch = NamedSwitch::new(
        [720., 75.],
        150.,
        color::CYAN,
        "    View".into(),
        app.training_params.snake_view,
        |app| &mut app.training_params.snake_view,
    );

    let mut pause_switch = NamedSwitch::new(
        [720., 150.],
        100.,
        color::CYAN,
        " Pause".into(),
        app.training_params.pause,
        |app| &mut app.training_params.pause,
    );

    let mut next_step_button = MyButton::new(
        Style::GREEN,
        [930., 150., 80., 50.],
        " Next".into(),
        |app| app.training_params.next_step = true,
    );

    let mut save_current_button = MyButton::new(
        Style::BLUE,
        [710., 215., 150., 50.],
        " Snapshot".into(),
        |app| {
            app.agent
                .as_ref()
                .unwrap()
                .snapshot(app.playground.as_ref().unwrap().get_score(), 's')
                .unwrap();
            app.agent
                .as_ref()
                .unwrap()
                .snapshot(app.playground.as_ref().unwrap().get_score(), 'l')
                .unwrap();
            app.training_params.just_snapshoted = true;
        },
    );

    let mut save_bests_button = MyButton::new(
        Style::BLUE,
        [865., 215., 150., 50.],
        "Save bests".into(),
        |app| {
            app.agent.as_ref().unwrap().save().unwrap();
            app.agent
                .as_ref()
                .unwrap()
                .snapshot(app.playground.as_ref().unwrap().get_score(), 'l')
                .unwrap();
            app.training_params.just_save_all = true;
        },
    );

    let mut speed_slider = Slider::new(
        0,
        1000,
        app.training_params.speed_time,
        [820., 275., 130., 40.],
        |app| &mut app.training_params.speed_time,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let board = Board::new(
            &app.playground.as_ref().unwrap(),
            app.training_params.snake_view,
        );
        let bests_scores = &app.agent.as_ref().unwrap().get_best_score();
        board.draw(&c, g);

        pause_switch.draw(&c, g, e, app);

        Rectangle::new(color::BLACK).draw([720., 10., 300., 60.], &c.draw_state, c.transform, g);
        Rectangle::new_border(color::CYAN, 1.).draw(
            [720., 10., 300., 60.],
            &c.draw_state,
            c.transform,
            g,
        );

        Text::new_color(color::WHITE, 28)
            .draw(
                &format!("On training: {}", app.training_params.name),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(740., 45.),
                g,
            )
            .unwrap();

        if app.training_params.pause && !app.training_params.train_finished {
            next_step_button.draw(&c, g, e, app);
        }
        if app.training_params.pause || app.training_params.train_finished {
            if !app.training_params.just_save_all {
                save_bests_button.draw(&c, g, e, app);
            } else {
                let shapes = save_bests_button.button_shapes;
                Rectangle::new(color::GREEN).draw(shapes, &c.draw_state, c.transform, g);
                Text::new(22)
                    .draw(
                        "    Bests saved",
                        &mut app.glyphs,
                        &c.draw_state,
                        c.transform.trans(shapes[0], shapes[1] + 35.),
                        g,
                    )
                    .unwrap();
            }
            if !app.training_params.just_snapshoted {
                save_current_button.draw(&c, g, e, app);
            } else {
                let shapes = save_current_button.button_shapes;
                Rectangle::new(color::GREEN).draw(shapes, &c.draw_state, c.transform, g);
                Text::new(22)
                    .draw(
                        " Snapshot done",
                        &mut app.glyphs,
                        &c.draw_state,
                        c.transform.trans(shapes[0], shapes[1] + 35.),
                        g,
                    )
                    .unwrap();
            }
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
                &app.training_params.speed_time.to_string(),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(960., 305.),
                g,
            )
            .unwrap();
        back_button.draw(&c, g, e, app);
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
                    "round {} of {}",
                    app.training_params.current_round, app.training_params.rounds
                ),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(722., 356.),
                g,
            )
            .unwrap();
        Text::new_color(color::WHITE, 16)
            .draw(
                &format!(
                    "current score: {}",
                    app.playground.as_ref().unwrap().get_score()
                ),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(860., 356.),
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
                &"Best scores reached: ",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(725., 395.),
                g,
            )
            .unwrap();

        bests_scores.iter().enumerate().for_each(|(idx, score)| {
            Text::new_color(color::WHITE, 16)
                .draw(
                    &format!("{}: score: {score}", idx + 1),
                    &mut app.glyphs,
                    &c.draw_state,
                    c.transform.trans(740., 425. + (20. * idx as f64)),
                    g,
                )
                .unwrap();
        });
    });

    let playground = app.playground.as_mut().unwrap();
    let agent = app.agent.as_mut().unwrap();

    if !app.training_params.train_finished {
        if match app.training_params.pause {
            false => {
                app.last_training_frame.elapsed()
                    > Duration::from_millis(app.training_params.speed_time as u64)
            }
            true => app.training_params.next_step,
        } {
            app.training_params.just_snapshoted = false;
            app.last_training_frame = Instant::now();
            app.training_params.next_step = false;
            if playground.is_alive() {
                let env = &playground.snake_view();
                let current_state = agent.ets.env_to_state(env);
                match app.training_params.previous_state {
                    None => {}
                    Some(p) => {
                        let reward = app.training_params.rewarder.get_reward(env);
                        agent.bellman(
                            p,
                            Some(current_state),
                            dir_to_usize(app.training_params.last_dir),
                            reward,
                        );
                    }
                }
                app.training_params.previous_state = Some(current_state);
                app.training_params.last_dir = agent.play(current_state, true);
                playground.next(app.training_params.last_dir);
                match playground.is_alive() {
                    true => playground.print_snake_view(),
                    false => {
                        println!("try: {}: score: {:03} | current explo_rate: {}, current discount_factor: {}\n",
                            app.training_params.current_round + 1,
                            playground.get_score(),
                            agent.exploration_rate,
                            agent.discount_factor
                        );
                        println!("{playground}")
                    }
                }
            } else {
                agent.bellman(
                    app.training_params.previous_state.unwrap(),
                    None,
                    dir_to_usize(app.training_params.last_dir),
                    app.training_params.rewarder.end_training_reward,
                );
                if agent.store_score(playground.get_score()) {
                    app.training_params.just_save_all = false;
                }
                app.training_params.current_round += 1;
                if app.training_params.current_round == app.training_params.rounds {
                    app.training_params.train_finished = true;
                } else {
                    agent.reduce_exploration_by(5. / (8. * app.training_params.rounds as f64));
                    agent.increase_discount_factor_by(1. / app.training_params.rounds as f64);
                    app.playground = Some(PlayGround::new(10, 10, make_rng()));
                    app.playground.as_ref().unwrap().print_snake_view();
                    app.training_params
                        .rewarder
                        .init(&app.playground.as_ref().unwrap().snake_view());
                    app.training_params.previous_state = None
                }
            }
        }
    }

    pause_switch.handle_event(e, app);
    if app.training_params.pause && !app.training_params.train_finished {
        next_step_button.handle_event(e, app);
    }
    if app.training_params.pause || app.training_params.train_finished {
        if !app.training_params.just_save_all {
            save_bests_button.handle_event(e, app);
        }
        if !app.training_params.just_snapshoted {
            save_current_button.handle_event(e, app);
        }
    }
    speed_slider.handle_event(e, app);
    back_button.handle_event(e, app);
    view_switch.handle_event(e, app);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Space) {
            app.training_params.pause = !app.training_params.pause;
        } else if button == Button::Keyboard(Key::Down) || button == Button::Keyboard(Key::Right) {
            app.training_params.speed_time = std::cmp::min(
                1000,
                match app.training_params.speed_time {
                    0 => 1,
                    _ => app.training_params.speed_time * 2,
                },
            )
        } else if button == Button::Keyboard(Key::Up) || button == Button::Keyboard(Key::Left) {
            app.training_params.speed_time = match app.training_params.speed_time {
                1 => 0,
                _ => app.training_params.speed_time / 2,
            }
        } else if button == Button::Keyboard(Key::V) {
            app.training_params.snake_view = !app.training_params.snake_view;
        } else if app.training_params.pause
            && (button == Button::Keyboard(Key::N)
                || button == Button::Keyboard(Key::Return)
                || button == Button::Keyboard(Key::Return2))
        {
            app.training_params.next_step = true;
        } else if button == Button::Keyboard(Key::Escape) {
            app.route = Route::Lobby;
            app.agent = None;
            app.playground = None;
            app.training_params = TrainingParams::new();
        }
    }
}

fn train_view(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |app| {
            app.route = Route::Lobby;
            app.agent = None;
            app.playground = None;
            app.training_params = TrainingParams::new() // reset params
        },
    );

    if !app.training_params.train_finished {
        train_loop(
            &mut app.agent.as_mut().unwrap(),
            app.training_params.rounds,
            true,
            None
        )
        .unwrap();
    }
    app.training_params.train_finished = true;

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        Text::new(30)
            .draw(
                &format!(
                    "Model {}'s training is finished, 10 bests models from it are saved",
                    app.training_params.name
                ),
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(50., 50.),
                g,
            )
            .unwrap();
        Text::new(30)
            .draw(
                "and ready to be tested",
                &mut app.glyphs,
                &c.draw_state,
                c.transform.trans(50., 90.),
                g,
            )
            .unwrap();

        back_button.draw(&c, g, e, app);
    });

    back_button.handle_event(e, app);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Escape) {
            app.route = Route::Lobby;
            app.agent = None;
            app.playground = None;
            app.training_params = TrainingParams::new();
        }
    }
}

pub fn training_route(window: &mut PistonWindow, e: &Event, app: &mut AppParams) {
    match &app.agent {
        None => training_form(window, e, app),
        Some(_agent) => {
            match &mut app.playground {
                None => {
                    app.playground = Some(PlayGround::new(10, 10, make_rng()));
                    app.playground.as_ref().unwrap().print_snake_view();
                    app.training_params
                        .rewarder
                        .init(&app.playground.as_ref().unwrap().snake_view());
                    app.training_params.previous_state = None
                }
                _ => {}
            }
            match app.training_params.interactive {
                true => training_board(window, e, app),
                false => train_view(window, e, app),
            }
        }
    }
}
