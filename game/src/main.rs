mod contexts;
use convenient_lib::Void;
use glyph_cache::rusttype::GlyphCache;
use graphics::*;
use piston_ctx::{Ctx, CtxValues, TrainingParams};
use piston_window::*;
use playground::PlayGround;
use std::{path::Path, time::Instant};
use wgpu_graphics::{Texture, TextureContext, TextureSettings};

fn main() -> Void {
    let mut window: PistonWindow = WindowSettings::new("Learn2Slither", (1024, 768))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = Path::new("assets");
    let mut ctx_values = CtxValues {
        glyphs: window
            .load_font(assets.join("FiraSans-Regular.ttf"), TextureSettings::new())
            .unwrap(),
        ctx: Ctx::Lobby,
        mouse_pressed: false,
        lshift_pressed: false,
        last_mouse_pos: None,
        exit: false,
        last_training_frame: Instant::now(),
        playground: None,
        selected_height: 10,
        selected_width: 10,
        training_params: TrainingParams::new(),
        agent: None,
        model: None,
    };

    window.set_lazy(true);

    while let Some(e) = window.next()
        && !ctx_values.exit
    {
        match ctx_values.ctx {
            Ctx::Lobby => contexts::lobby::lobby(&mut window, &e, &mut ctx_values),
            Ctx::Test => contexts::testing_board::testing_board(&mut window, &e, &mut ctx_values),
            Ctx::Train => {
                contexts::training_board::training_board(&mut window, &e, &mut ctx_values)
            }
            Ctx::Play => contexts::playing_board::playing_board(&mut window, &e, &mut ctx_values),
        }

        // Saving mouse's params.
        if let Some(pos) = e.mouse_cursor_args() {
            ctx_values.last_mouse_pos = Some(pos);
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                ctx_values.mouse_pressed = true;
            }
            if button == Button::Keyboard(Key::LShift) {
                ctx_values.lshift_pressed = true;
            }
        }
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                ctx_values.mouse_pressed = false;
            }
            if button == Button::Keyboard(Key::LShift) {
                ctx_values.lshift_pressed = false;
            }
        }
    }
    Ok(())
}
