use graphics::*;
use interpretors::state::list_all_ets;
use piston_components::components::{
    PistonComponent,
    buttons::{ButtonStoreVal, MyButton, Style},
    sliders::{Slider, SliderVertical},
    switch::NamedSwitch, text_area::TextArea,
};
use piston_ctx::{Ctx, CtxValues};
use piston_window::{
    graphics::{Rectangle, Text},
    *,
};
use models_handler::get_model_names;

pub fn training_form<'a>(window: &mut PistonWindow, e: &Event, ctx: &'a mut CtxValues) {

    let model_names = &get_model_names();

    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "     CANCEL".into(),
        |ctx| ctx.ctx = Ctx::Lobby,
    );

    let mut etx_buttons = list_all_ets()
        .iter()
        .enumerate()
        .map(|(idx, ets_name)| {
            ButtonStoreVal::new(
                Style::BLUE,
                [650., 200. + 75. * idx as f64, 300., 60.],
                String::from("     ") + &ets_name.clone(),
                ets_name.clone(),
                |ctx| &mut ctx.training_params.ets,
            )
        })
        .collect::<Vec<_>>();

    let mut models_buttons = match model_names {
        Err(_) => Vec::<ButtonStoreVal<String>>::new(),
        Ok(names) => {
            names
                .iter()
                .enumerate()
                .map(|(idx, model_name)| {
                    ButtonStoreVal::new(
                        Style::BLUE,
                        [650., 200. + 75. * idx as f64, 250., 60.],
                        String::from("     ") + &model_name.clone(),
                        model_name.clone(),
                        |ctx| &mut ctx.training_params.from_model_name,
                    )
                })
                .collect::<Vec<_>>()
        }
    };

    let mut models_slider = SliderVertical::new(
        0,
        match model_names {
                Err(_) => 0,
                Ok(list) => {
                    match list.len() > 3 {
                        true => list.len() - 3,
                        false => 0
                    }
                }
            },
            ctx.training_params.from_model_cursor,
            [950., 200., 30., 500.],
            |ctx| &mut ctx.training_params.from_model_cursor
    );

    let mut from_switch = NamedSwitch::new(
        [600., 50.],
        color::MAGENTA,
        "        from existing".into(),
        ctx.training_params.from_bool,
        |ctx| &mut ctx.training_params.from_bool
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

    let mut speed_slider = Slider::new(
        0,
        1000,
        ctx.training_params.speed_time,
        [350., 270., 150., 50.],
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
        |ctx| &mut ctx.training_params.name
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
                c.transform.trans(600., 150.),
                g,
            )
            .unwrap();

            models_buttons.iter().for_each(|m| m.draw(&c, g, e, ctx));
        }
        else {
            Text::new(32)
            .draw(
                "Train algorithm",
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(600., 150.),
                g,
            )
            .unwrap();
        
            etx_buttons.iter().for_each(|b| b.draw(&c, g, e, ctx));
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
            Rectangle::new(color::CYAN).draw([50., 270., 300., 50.], &c.draw_state, c.transform, g);
            Text::new(32)
                .draw(
                    "frame period(ms)",
                    &mut ctx.glyphs,
                    &c.draw_state,
                    c.transform.trans(60., 305.),
                    g,
                )
                .unwrap();
            speed_slider.draw(&c, g, e, ctx);
            Text::new(32)
                .draw(
                    &(ctx.training_params.speed_time.to_string() + &String::from(" ms")),
                    &mut ctx.glyphs,
                    &c.draw_state,
                    c.transform.trans(510., 305.),
                    g,
                )
                .unwrap();
            pause_switch.draw(&c, g, e, ctx);
        }

        back_button.draw(&c, g, e, ctx);
    });

    from_switch.handle_event(e, ctx);
    if ctx.training_params.from_bool {
        models_buttons.iter_mut().for_each(|m| m.handle_event(e, ctx));
    }
    else {
        etx_buttons.iter_mut().for_each(|b| b.handle_event(e, ctx));
    }

    interactive_switch.handle_event(e, ctx);
    if ctx.training_params.interactive {
        step_to_step_switch.handle_event(e, ctx);
        pause_switch.handle_event(e, ctx);
        speed_slider.handle_event(e, ctx);
    }

    rounds_slider.handle_event(e, ctx);
    name_text_area.handle_event(e, ctx);
    back_button.handle_event(e, ctx);
}

pub fn training_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match ctx.agent {
        None => training_form(window, e, ctx),
        _ => {}
    }
}
