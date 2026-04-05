use convenient_lib::Res;
use graphics::*;
use piston_components::components::PistonComponent;
use piston_components::components::buttons::{ButtonActionFromVal, MyButton, Style};
use piston_components::components::sliders::{Slider, SliderVertical};
use piston_components::components::switch::NamedSwitch;
use piston_ctx::{Ctx, CtxValues, LeaderBoard, LeaderBoardItem, TestingParams};
use piston_window::{graphics::Text, *};
use playground::{Dir, PlayGround};
use qlearning::{Agent, Model};
use rand::make_rng;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::contexts::board_helpers::Board;
use crate::contexts::file_handler::{get_model, get_model_bests};

const MODEL_PATH: &str = "models";

fn dir_to_usize(dir: Dir) -> usize {
    match dir {
        Dir::Up => 0,
        Dir::Right => 1,
        Dir::Down => 2,
        Dir::Left => 3,
    }
}

fn test_form(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {

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

    let mut test_button = MyButton::new(
        Style::GREEN,
        [532., 650., 200., 75.],
        "        TEST".into(),
        |ctx| {
            ctx.agent = match &ctx.testing_params.model {
                None => None,
                Some(m) => Some(Agent::from_model(&ctx.testing_params.model_name, m).unwrap()),
            }
        },
    );

    let mut slider_width =
        Slider::new(
            10,
            50,
            ctx.selected_width,
            [75., 180., 300., 40.],
            |ctx| &mut ctx.selected_width
        );

    let mut slider_height = Slider::new(
        10,
        50,
        ctx.selected_height,
        [75., 330., 300., 40.],
        |ctx| &mut ctx.selected_height,
    );

    let mut models_buttons = match &ctx.testing_params.model_names {
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
                    |ctx| &mut ctx.testing_params.model_name,
                    |ctx| &mut ctx.testing_params.model_idx_list,
                    |ctx| {
                        ctx.testing_params.model = None;
                        ctx.testing_params.model_idx = "none".into();
                        get_model_bests(
                            &Path::new(MODEL_PATH).join(&ctx.testing_params.model_name),
                        )
                    },
                )
            })
            .collect::<Vec<_>>(),
    };

    let mut models_overflow_y = SliderVertical::new(
        0,
        match &ctx.testing_params.model_names {
            Err(_) => 0,
            Ok(list) => match list.len() > 3 {
                true => list.len() - 3,
                false => 0,
            },
        },
        std::cmp::min(
            ctx.testing_params.model_cursor,
            match &ctx.testing_params.model_names {
                Err(_) => 0,
                Ok(list) => match list.len() > 3 {
                    true => list.len() - 3,
                    false => 0,
                },
            },
        ),
        [950., 200., 30., 210.],
        |ctx| &mut ctx.testing_params.model_cursor,
    );

    let mut bests_buttons = match &ctx.testing_params.model_idx_list {
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
                    |ctx| &mut ctx.testing_params.model_idx,
                    |ctx| &mut ctx.testing_params.model,
                    |ctx| {
                        Some(
                            get_model(
                                &mut Path::new(MODEL_PATH)
                                    .join(&ctx.testing_params.model_name)
                                    .join(&ctx.testing_params.model_idx),
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
        match &ctx.testing_params.model_idx_list {
            Err(_) => 0,
            Ok(list) => match list.len() > 4 {
                true => list.len() - 4,
                false => 0,
            },
        },
        std::cmp::min(
            ctx.testing_params.model_idx_cursor,
            match &ctx.testing_params.model_idx_list {
                Err(_) => 0,
                Ok(list) => match list.len() > 4 {
                    true => list.len() - 4,
                    false => 0,
                },
            },
        ),
        [655., 520., 265., 30.],
        |ctx| &mut ctx.testing_params.model_idx_cursor,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let transform = c.transform.trans(80., 150.);
        Text::new(28)
            .draw(
                "Select the width (10 to 50)",
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_width.draw(&c, g, e, ctx);
        let transform = c.transform.trans(385., 208.);
        Text::new(32)
            .draw(
                &ctx.selected_width.to_string(),
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        let transform = c.transform.trans(80., 300.);
        Text::new(28)
            .draw(
                "Select the height (10 to 50)",
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_height.draw(&c, g, e, ctx);
        let transform = c.transform.trans(385., 360.);
        Text::new(32)
            .draw(
                &ctx.selected_height.to_string(),
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        Text::new(32)
                .draw(
                    "Model to train from",
                    &mut ctx.glyphs,
                    &c.draw_state,
                    c.transform.trans(650., 160.),
                    g,
                )
                .unwrap();

        models_buttons
            .iter()
            .skip(ctx.testing_params.model_cursor)
            .take(3)
            .enumerate()
            .for_each(|(idx, m)| m.draw_pos([650., 200. + 75. * idx as f64], &c, g, e, ctx));

        match &ctx.testing_params.model_names {
            Err(_) => {}
            Ok(list) => match list.len() > 3 {
                true => models_overflow_y.draw(&c, g, e, ctx),
                false => {}
            },
        };

        bests_buttons
            .iter()
            .skip(ctx.testing_params.model_idx_cursor)
            .take(4)
            .enumerate()
            .for_each(|(idx, m)| m.draw_pos([650. + 75. * idx as f64, 450.], &c, g, e, ctx));

        match &ctx.testing_params.model_idx_list {
            Err(_) => {}
            Ok(list) => match list.len() > 4 {
                true => bests_buttons_overflow_x.draw(&c, g, e, ctx),
                false => {}
            },
        };

        match &ctx.testing_params.model {
            None => {}
            Some(m) => Text::new(32)
                .draw(
                    &format!("score reached: {}", m.score),
                    &mut ctx.glyphs,
                    &c.draw_state,
                    c.transform.trans(670., 600.),
                    g,
                )
                .unwrap()
            }

        if ctx.testing_params.model.is_some() {
            test_button.draw(&c, g, e, ctx);
        }
        back_button.draw(&c, g, e, ctx);
    });

    slider_width.handle_event(e, ctx);
    slider_height.handle_event(e, ctx);

    match &ctx.testing_params.model_names {
        Err(_) => {}
        Ok(list) => match list.len() > 3 {
            true => models_overflow_y.handle_event(e, ctx),
            false => {}
        },
    };

    models_buttons
        .iter_mut()
        .skip(ctx.testing_params.model_cursor)
        .take(3)
        .enumerate()
        .for_each(|(idx, m)| m.handle_event_pos([650., 200. + 75. * idx as f64], e, ctx));

    match &ctx.testing_params.model_idx_list {
        Err(_) => {}
        Ok(list) => match list.len() > 4 {
            true => bests_buttons_overflow_x.handle_event(e, ctx),
            false => {}
        },
    };

    bests_buttons
        .iter_mut()
        .skip(ctx.testing_params.model_idx_cursor)
        .take(4)
        .enumerate()
        .for_each(|(idx, m)| m.handle_event_pos([650. + 75. * idx as f64, 450.], e, ctx));


    if ctx.testing_params.model.is_some() {
        test_button.handle_event(e, ctx);
    }
    back_button.handle_event(e, ctx);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Escape) {
            ctx.ctx = Ctx::Lobby;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        }
    }
}

fn testing_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |ctx| {
            ctx.ctx = Ctx::Lobby;
            ctx.agent = None;
            ctx.playground = None;
            ctx.testing_params = TestingParams::new() // reset params
        },
    );

    let mut view_switch = NamedSwitch::new(
        [720., 75.],
        150.,
        color::CYAN,
        "    View".into(),
        ctx.testing_params.snake_view,
        |ctx| &mut ctx.testing_params.snake_view,
    );

    let mut pause_switch = NamedSwitch::new(
        [720., 150.],
    100.,
        color::CYAN,
        " Pause".into(),
        ctx.testing_params.pause,
        |ctx| &mut ctx.testing_params.pause,
    );

    let mut next_step_button = MyButton::new(
        Style::GREEN,
        [930., 150., 80., 50.],
        " Next".into(),
        |ctx| ctx.testing_params.next_step = true
    );

    let mut speed_slider = Slider::new(
        0,
        1000,
        ctx.testing_params.speed_time,
        [820., 275., 130., 40.],
        |ctx| &mut ctx.testing_params.speed_time,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let board = Board::new(&ctx.playground.as_ref().unwrap(), ctx.testing_params.snake_view);

        board.draw(&c, g);

        pause_switch.draw(&c, g, e, ctx);

        if ctx.testing_params.pause {
            next_step_button.draw(&c, g, e, ctx);
        }

        view_switch.draw(&c, g, e, ctx);
        Text::new(28).draw(
            " Speed",
            &mut ctx.glyphs,
            &c.draw_state,
            c.transform.trans(720., 305.),
            g).unwrap();
        speed_slider.draw(&c, g, e, ctx);
        Text::new(28).draw(
            &ctx.testing_params.speed_time.to_string(),
            &mut ctx.glyphs,
            &c.draw_state,
            c.transform.trans(960., 305.),
            g).unwrap();

        Rectangle::new(color::BLACK).draw([710., 330., 300., 300.], &c.draw_state, c.transform, g);
        Rectangle::new_border(color::WHITE, 0.5).draw([714., 334., 293., 35.], &c.draw_state, c.transform, g);
        Text::new_color(color::WHITE, 16).draw(
            &format!("current score: {}", ctx.playground.as_ref().unwrap().get_score()),
            &mut ctx.glyphs,
            &c.draw_state,
            c.transform.trans(722., 356.),
            g).unwrap();
        Rectangle::new_border(color::WHITE, 0.5).draw([714., 373., 293., 253.], &c.draw_state, c.transform, g);
        Text::new_color(color::WHITE, 16).draw(
            &"LeaderBoard:",
            &mut ctx.glyphs,
            &c.draw_state,
            c.transform.trans(725., 395.),
            g).unwrap();
        
        ctx.leaderboard.leaderboard
            .iter()
            .enumerate()
            .for_each(|(idx, item)| {
                Text::new_color(color::WHITE, 16).draw(
                &format!("{}: score: {}  {} ets: {}", idx + 1, item.score, item.model_name, item.ets_name),
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(740., 425. + (20. * idx as f64)),
                g).unwrap();
            });
        
        back_button.draw(&c, g, e, ctx);
    });

    let playground = ctx.playground.as_mut().unwrap();
    let agent = ctx.agent.as_mut().unwrap();

    if match ctx.testing_params.pause {
        false => ctx.last_training_frame.elapsed() > Duration::from_millis(ctx.testing_params.speed_time as u64),
        true => ctx.testing_params.next_step
    } {
        ctx.last_training_frame = Instant::now();
        ctx.testing_params.next_step = false;
        if playground.is_alive() {
            let env = &playground.snake_view();
            let state = agent.ets.env_to_state(env);
            let dir = agent.play(state);
            playground.next(dir);
            if !playground.is_alive() && !ctx.testing_params.infinite_loop {
                ctx.testing_params.pause = true;
            }
        }
        else {
            if ctx.selected_height == 10 && ctx.selected_width == 10 {
                if ctx.leaderboard.leaderboard.len() < 10 {
                    ctx.leaderboard.leaderboard.push(
                        LeaderBoardItem {
                            score: playground.get_score(),
                            model_name: ctx.agent.as_ref().unwrap().name.clone(),
                            ets_name: ctx.agent.as_ref().unwrap().ets.get_name()
                        }
                    );
                    ctx.leaderboard.leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
                } else {
                    match ctx.leaderboard.leaderboard[9].score < playground.get_score() {
                        true => {
                            ctx.leaderboard.leaderboard.remove(9);
                            ctx.leaderboard.leaderboard.push(
                                LeaderBoardItem {
                                    score: playground.get_score(),
                                    model_name: ctx.agent.as_ref().unwrap().name.clone(),
                                    ets_name: ctx.agent.as_ref().unwrap().ets.get_name()
                                }
                            );
                            ctx.leaderboard.leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
                        },
                        false => {}
                    }
                }
            }
            ctx.playground = Some(PlayGround::new(ctx.selected_height, ctx.selected_width, make_rng()));
        }
    }

    pause_switch.handle_event(e, ctx);
    if ctx.testing_params.pause {
        next_step_button.handle_event(e, ctx);
    }
    speed_slider.handle_event(e, ctx);
    back_button.handle_event(e, ctx);
    view_switch.handle_event(e, ctx);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Space) {
            ctx.testing_params.pause = !ctx.testing_params.pause;
        }
        else if button == Button::Keyboard(Key::Down) || button == Button::Keyboard(Key::Right) {
            ctx.testing_params.speed_time = std::cmp::min(1000, 
                match ctx.testing_params.speed_time {
                    0 => 1,
                    _ => ctx.testing_params.speed_time * 2
                }
            )
        }
        else if button == Button::Keyboard(Key::Up) || button == Button::Keyboard(Key::Left) {
            ctx.testing_params.speed_time = match ctx.testing_params.speed_time {
                1 => 0,
                _ => ctx.testing_params.speed_time / 2
            }
        }
        else if button == Button::Keyboard(Key::V) {
            ctx.testing_params.snake_view = !ctx.testing_params.snake_view;
        }
        else if ctx.testing_params.pause
        && (button == Button::Keyboard(Key::N)
        || button == Button::Keyboard(Key::Return)
        || button == Button::Keyboard(Key::Return2)) {
            ctx.testing_params.next_step = true;
        }
        else if button == Button::Keyboard(Key::Escape) {
            ctx.ctx = Ctx::Lobby;
            ctx.agent = None;
            ctx.playground = None;
            ctx.testing_params = TestingParams::new();
        }
    }

}

pub fn testing_route(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match ctx.agent {
        Some(_) => {
            match ctx.playground {
                Some(_) => testing_board(window, e, ctx),
                None => ctx.playground = Some(PlayGround::new(ctx.selected_height, ctx.selected_width, make_rng()))
            }
        },
        None => test_form(window, e, ctx),
    }
}
