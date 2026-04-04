use crate::contexts::{board_helpers::Board, file_handler::{get_model, get_model_bests, get_model_names}};
use convenient_lib::Res;
use graphics::*;
use interpretors::state::list_all_ets;
use piston_components::components::{
    PistonComponent,
    buttons::{ButtonActionFromVal, ButtonStoreVal, MyButton, Style},
    sliders::{Slider, SliderVertical},
    switch::{NamedSwitch, NamedSwitchAction},
    text_area::TextArea,
};
use piston_ctx::{Ctx, CtxValues};
use piston_window::{
    graphics::{Rectangle, Text},
    *,
};
use playground::{Dir, PlayGround};
use qlearning::{Agent, Model};
use rand::make_rng;
use std::{path::Path, time::{Duration, Instant}};

const MODEL_PATH: &str = "models";

fn dir_to_usize(dir: Dir) -> usize {
    match dir {
        Dir::Up => 0,
        Dir::Right => 1,
        Dir::Down => 2,
        Dir::Left => 3,
    }
}

pub fn training_form<'a>(window: &mut PistonWindow, e: &Event, ctx: &'a mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "     CANCEL".into(),
        |ctx| ctx.ctx = Ctx::Lobby,
    );

    let mut train_button = MyButton::new(
        Style::GREEN,
        [532., 650., 200., 75.],
        "      TRAIN".into(),
        |ctx| {
            ctx.last_training_frame = Instant::now();
            ctx.agent = match ctx.training_params.from_bool {
                true => match &ctx.training_params.from_model {
                    None => None,
                    Some(m) => Some(Agent::from_model(&ctx.training_params.name, m).unwrap()),
                },
                false => match &ctx.training_params.ets {
                    None => None,
                    Some(ets) => {
                        Some(
                            Agent::new(
                                ets.clone(),
                                ctx.training_params.name.clone(),
                            )
                            .unwrap(),
                        )
                    }
                }
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
                |ctx| &mut ctx.training_params.ets,
            )
        })
        .collect::<Vec<_>>();

    let mut ets_overflow_y = SliderVertical::new(
        0,
        match ctx.training_params.ets_list.len() > 5 {
            true => ctx.training_params.ets_list.len() - 5,
            false => 0,
        },
        std::cmp::min(
            ctx.training_params.ets_cursor,
            match ctx.training_params.ets_list.len() > 3 {
                true => ctx.training_params.ets_list.len() - 5,
                false => 0,
            },
        ),
        [950., 200., 30., 360.],
        |ctx| &mut ctx.training_params.ets_cursor,
    );

    let mut models_buttons = match &ctx.training_params.from_model_names {
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
                    |ctx| &mut ctx.training_params.from_model_name,
                    |ctx| &mut ctx.training_params.from_model_idx_list,
                    |ctx| {
                        ctx.training_params.from_model = None;
                        ctx.training_params.from_model_idx = "none".into();
                        get_model_bests(
                            &Path::new(MODEL_PATH).join(&ctx.training_params.from_model_name),
                        )
                    },
                )
            })
            .collect::<Vec<_>>(),
    };

    let mut models_overflow_y = SliderVertical::new(
        0,
        match &ctx.training_params.from_model_names {
            Err(_) => 0,
            Ok(list) => match list.len() > 3 {
                true => list.len() - 3,
                false => 0,
            },
        },
        std::cmp::min(
            ctx.training_params.from_model_cursor,
            match &ctx.training_params.from_model_names {
                Err(_) => 0,
                Ok(list) => match list.len() > 3 {
                    true => list.len() - 3,
                    false => 0,
                },
            },
        ),
        [950., 200., 30., 210.],
        |ctx| &mut ctx.training_params.from_model_cursor,
    );

    let mut bests_buttons = match &ctx.training_params.from_model_idx_list {
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
                    |ctx| &mut ctx.training_params.from_model_idx,
                    |ctx| &mut ctx.training_params.from_model,
                    |ctx| {
                        Some(
                            get_model(
                                &mut Path::new(MODEL_PATH)
                                    .join(&ctx.training_params.from_model_name)
                                    .join(&ctx.training_params.from_model_idx),
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
        match &ctx.training_params.from_model_idx_list {
            Err(_) => 0,
            Ok(list) => match list.len() > 4 {
                true => list.len() - 4,
                false => 0,
            },
        },
        std::cmp::min(
            ctx.training_params.from_model_idx_cursor,
            match &ctx.training_params.from_model_idx_list {
                Err(_) => 0,
                Ok(list) => match list.len() > 4 {
                    true => list.len() - 4,
                    false => 0,
                },
            },
        ),
        [655., 520., 265., 30.],
        |ctx| &mut ctx.training_params.from_model_idx_cursor,
    );

    let mut from_switch = NamedSwitchAction::new(
        [600., 50.],
        color::MAGENTA,
        "        from existing".into(),
        ctx.training_params.from_bool,
        |ctx| &mut ctx.training_params.from_bool,
        |ctx| &mut ctx.training_params.from_model_names,
        |_| get_model_names(MODEL_PATH),
        |ctx| &mut ctx.training_params.ets_list,
        |_| list_all_ets(),
    );

    let mut interactive_switch = NamedSwitch::new(
        [50., 50.],
        color::MAGENTA,
        " interactive".into(),
        ctx.training_params.interactive,
        |ctx| &mut ctx.training_params.interactive,
    );

    let mut step_to_step_switch = NamedSwitch::new(
        [50., 125.],
        color::CYAN,
        " step_to_step".into(),
        ctx.training_params.step_by_step,
        |ctx| &mut ctx.training_params.step_by_step,
    );

    let mut pause_switch = NamedSwitch::new(
        [50., 200.],
        color::CYAN,
        " pause".into(),
        ctx.training_params.pause,
        |ctx| &mut ctx.training_params.pause,
    );

    let mut snake_view_switch = NamedSwitch::new(
        [50., 270.],
        color::CYAN,
        " snake view".into(),
        ctx.training_params.snake_view,
        |ctx| &mut ctx.training_params.snake_view,
    );

    let mut speed_slider = Slider::new(
        0,
        1000,
        ctx.training_params.speed_time,
        [350., 340., 150., 50.],
        |ctx| &mut ctx.training_params.speed_time,
    );

    let mut rounds_slider = Slider::new(
        1,
        5000,
        ctx.training_params.rounds,
        [350., 500., 150., 50.],
        |ctx| &mut ctx.training_params.rounds,
    );

    let mut name_text_area = TextArea::new(
        [350., 575.],
        7,
        36,
        ctx.training_params.name.clone(),
        |ctx| &mut ctx.training_params.name,
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        from_switch.draw(&c, g, e, ctx);
        if ctx.training_params.from_bool {
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
                .skip(ctx.training_params.from_model_cursor)
                .take(3)
                .enumerate()
                .for_each(|(idx, m)| m.draw_pos([650., 200. + 75. * idx as f64], &c, g, e, ctx));

            match &ctx.training_params.from_model_names {
                Err(_) => {}
                Ok(list) => match list.len() > 3 {
                    true => models_overflow_y.draw(&c, g, e, ctx),
                    false => {}
                },
            };

            bests_buttons
                .iter()
                .skip(ctx.training_params.from_model_idx_cursor)
                .take(4)
                .enumerate()
                .for_each(|(idx, m)| m.draw_pos([650. + 75. * idx as f64, 450.], &c, g, e, ctx));

            match &ctx.training_params.from_model_idx_list {
                Err(_) => {}
                Ok(list) => match list.len() > 4 {
                    true => bests_buttons_overflow_x.draw(&c, g, e, ctx),
                    false => {}
                },
            };

            match &ctx.training_params.from_model {
                None => {}
                Some(m) => Text::new(32)
                    .draw(
                        &format!("score reached: {}", m.score),
                        &mut ctx.glyphs,
                        &c.draw_state,
                        c.transform.trans(670., 600.),
                        g,
                    )
                    .unwrap(),
            }
        } else {
            Text::new(32)
                .draw(
                    "Train algorithm",
                    &mut ctx.glyphs,
                    &c.draw_state,
                    c.transform.trans(650., 160.),
                    g,
                )
                .unwrap();

            ets_buttons
                .iter()
                .skip(ctx.training_params.ets_cursor)
                .take(5)
                .enumerate()
                .for_each(|(idx, b)| b.draw_pos([650., 200. + 75. * idx as f64], &c, g, e, ctx));

            if ctx.training_params.ets_list.len() > 5 {
                ets_overflow_y.draw(&c, g, e, ctx);
            }
        }
        Rectangle::new(color::CYAN).draw([50., 500., 300., 50.], &c.draw_state, c.transform, g);
        Text::new(32)
            .draw(
                " number of rounds",
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(60., 535.),
                g,
            )
            .unwrap();
        rounds_slider.draw(&c, g, e, ctx);
        Text::new(32)
            .draw(
                &ctx.training_params.rounds.to_string(),
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(510., 535.),
                g,
            )
            .unwrap();

        Rectangle::new(color::CYAN).draw([50., 575., 300., 50.], &c.draw_state, c.transform, g);
        Text::new(32)
            .draw(
                " model name",
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(60., 610.),
                g,
            )
            .unwrap();
        name_text_area.draw(&c, g, e, ctx);

        // Interactive buttons.
        interactive_switch.draw(&c, g, e, ctx);
        if ctx.training_params.interactive {
            step_to_step_switch.draw(&c, g, e, ctx);
            snake_view_switch.draw(&c, g, e, ctx);
            Rectangle::new(color::CYAN).draw([50., 340., 300., 50.], &c.draw_state, c.transform, g);
            Text::new(32)
                .draw(
                    "frame period(ms)",
                    &mut ctx.glyphs,
                    &c.draw_state,
                    c.transform.trans(60., 375.),
                    g,
                )
                .unwrap();
            speed_slider.draw(&c, g, e, ctx);
            Text::new(32)
                .draw(
                    &(ctx.training_params.speed_time.to_string()),
                    &mut ctx.glyphs,
                    &c.draw_state,
                    c.transform.trans(510., 375.),
                    g,
                )
                .unwrap();
            pause_switch.draw(&c, g, e, ctx);
        }

        if (ctx.training_params.from_bool && ctx.training_params.from_model.is_some())
        || (!ctx.training_params.from_bool && ctx.training_params.ets.is_some()) {
            train_button.draw(&c, g, e, ctx);
        }

        back_button.draw(&c, g, e, ctx);
    });

    from_switch.handle_event(e, ctx);
    if ctx.training_params.from_bool {
        match &ctx.training_params.from_model_names {
            Err(_) => {}
            Ok(list) => match list.len() > 3 {
                true => models_overflow_y.handle_event(e, ctx),
                false => {}
            },
        };

        models_buttons
            .iter_mut()
            .skip(ctx.training_params.from_model_cursor)
            .take(3)
            .enumerate()
            .for_each(|(idx, m)| m.handle_event_pos([650., 200. + 75. * idx as f64], e, ctx));

        match &ctx.training_params.from_model_idx_list {
            Err(_) => {}
            Ok(list) => match list.len() > 4 {
                true => bests_buttons_overflow_x.handle_event(e, ctx),
                false => {}
            },
        };

        bests_buttons
            .iter_mut()
            .skip(ctx.training_params.from_model_idx_cursor)
            .take(4)
            .enumerate()
            .for_each(|(idx, m)| m.handle_event_pos([650. + 75. * idx as f64, 450.], e, ctx));
    } else {
        ets_buttons
            .iter_mut()
            .skip(ctx.training_params.ets_cursor)
            .take(5)
            .enumerate()
            .for_each(|(idx, b)| b.handle_event_pos([650., 200. + 75. * idx as f64], e, ctx));
        if ctx.training_params.ets_list.len() > 5 {
            ets_overflow_y.handle_event(e, ctx);
        }
    }

    interactive_switch.handle_event(e, ctx);
    if ctx.training_params.interactive {
        step_to_step_switch.handle_event(e, ctx);
        pause_switch.handle_event(e, ctx);
        snake_view_switch.handle_event(e, ctx);
        speed_slider.handle_event(e, ctx);
    }

    rounds_slider.handle_event(e, ctx);
    name_text_area.handle_event(e, ctx);

    if (ctx.training_params.from_bool && ctx.training_params.from_model.is_some())
        || (!ctx.training_params.from_bool && ctx.training_params.ets.is_some()) {
            train_button.handle_event(e, ctx);
        }
    back_button.handle_event(e, ctx);
}

pub fn interactive_train_view(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |ctx| {
            ctx.ctx = Ctx::Lobby;
            ctx.agent = None;
            ctx.playground = None;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        },
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let board = Board::new(ctx.playground.as_ref().unwrap(), ctx.training_params.snake_view);
        board.draw(&c, g);

        back_button.draw(&c, g, e, ctx);
    });

    let playground = ctx.playground.as_mut().unwrap();
    let agent = ctx.agent.as_mut().unwrap();

    if !ctx.training_params.train_finished {
        if ctx.last_training_frame.elapsed() > Duration::from_millis(ctx.training_params.speed_time as u64) {
            ctx.last_training_frame = Instant::now();
            if playground.is_alive() {
                let env = &playground.snake_view();
                let current_state = agent.ets.env_to_state(env);
                match ctx.training_params.previous_state {
                    None => {},
                    Some(p) => {
                        let reward = ctx.training_params.rewarder.get_reward(env);
                        agent.bellman(
                            p,
                            Some(current_state),
                            dir_to_usize(ctx.training_params.last_dir),
                            reward
                        );
                    }
                }
                ctx.training_params.previous_state = Some(current_state);
                ctx.training_params.last_dir = agent.play(current_state);
                playground.next(ctx.training_params.last_dir);
            }
            else {
                agent.bellman(
                    ctx.training_params.previous_state.unwrap(),
                    None,
                    dir_to_usize(ctx.training_params.last_dir),
                    ctx.training_params.rewarder.end_training_reward);
                agent.store_score(playground.get_score());
                ctx.training_params.current_round += 1;
                if ctx.training_params.current_round == ctx.training_params.rounds {
                    ctx.training_params.train_finished = true;
                }
                else {
                    agent.reduce_exploration_by(1. / (0.8 * ctx.training_params.rounds as f64));
                    agent.increase_discount_factor_by(0.75 / ctx.training_params.rounds as f64);
                    ctx.playground = Some(PlayGround::new(10, 10, make_rng()));
                    ctx.training_params.rewarder.init(&ctx.playground.as_ref().unwrap().snake_view());
                    ctx.training_params.previous_state = None
                }
            }
        }
    }

    back_button.handle_event(e, ctx);

}

pub fn train_view(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {

}

pub fn training_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match &ctx.agent {
        None => training_form(window, e, ctx),
        Some(_agent) => {
            match &mut ctx.playground {
                None => {
                    ctx.playground = Some(PlayGround::new(10, 10, make_rng()));
                    ctx.training_params.rewarder.init(&ctx.playground.as_ref().unwrap().snake_view());
                    ctx.training_params.previous_state = None
                },
                _ => {}
            }
            match ctx.training_params.interactive {
                true => interactive_train_view(window, e, ctx),
                false => train_view(window, e, ctx)
            }
        }
    }
}
