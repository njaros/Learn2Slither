use graphics::*;
use interpretors::state::list_all_ets;
use piston_components::components::{
    PistonComponent,
    buttons::{MyButton, Style},
};
use piston_ctx::{Ctx, CtxValues};
use piston_window::{graphics::Context, wgpu::Device, wgpu_graphics::WgpuGraphics};
use piston_window::{graphics::Text, *};
use wgpu_graphics::{Texture, TextureSettings};

pub fn training_form(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "     CANCEL".into(),
        |ctx| ctx.ctx = Ctx::Lobby,
    );

    // let etx_buttons = list_all_ets()
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, ets_name)| MyButton::new(
    //         Style::BLUE,
    //         [700., 100. + 150. * idx as f64, 300., 100.],
    //         ets_name.clone(),
    //         |ctx| ctx.training_params.ets = ets_name.clone(),
    //     ))
    //     .collect::<Vec<_>>();

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        back_button.draw(&c, g, e, ctx);
        Rectangle::new([10.0, 31.0, 0.0, 1.0]).draw(
            [30.0, 40.0, 100.0, 100.0],
            &c.draw_state,
            c.transform,
            g,
        );
    });

    back_button.handle_event(e, ctx);
}

pub fn training_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match ctx.agent {
        None => training_form(window, e, ctx),
        _ => {}
    }
}
