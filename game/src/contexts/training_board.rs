use graphics::*;
use interpretors::state::list_all_ets;
use piston_components::components::{
    PistonComponent,
    buttons::{ButtonStoreVal, MyButton, Style},
    sliders::Slider,
    switch::NamedSwitch, text_area::TextArea,
};
use piston_ctx::{Ctx, CtxValues};
use piston_window::{
    graphics::{Rectangle, Text},
    *,
};

pub fn training_form<'a>(window: &mut PistonWindow, e: &Event, ctx: &'a mut CtxValues) {
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
                [650., 150. + 75. * idx as f64, 300., 60.],
                String::from("     ") + &ets_name.clone(),
                ets_name.clone(),
                |ctx| &mut ctx.training_params.ets,
            )
        })
        .collect::<Vec<_>>();

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
        [100., 650.],
        8,
        32,
        ctx.training_params.name.clone(),
        |ctx| &mut ctx.training_params.name
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        Text::new(32)
            .draw(
                "Select your train algorithm.",
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform.trans(600., 100.),
                g,
            )
            .unwrap();

        // Training params buttons
        etx_buttons.iter().for_each(|b| b.draw(&c, g, e, ctx));
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

    interactive_switch.handle_event(e, ctx);
    if ctx.training_params.interactive {
        step_to_step_switch.handle_event(e, ctx);
        pause_switch.handle_event(e, ctx);
        speed_slider.handle_event(e, ctx);
    }

    rounds_slider.handle_event(e, ctx);
    name_text_area.handle_event(e, ctx);
    etx_buttons.iter_mut().for_each(|b| b.handle_event(e, ctx));
    back_button.handle_event(e, ctx);
}

pub fn training_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match ctx.agent {
        None => training_form(window, e, ctx),
        _ => {}
    }
}
