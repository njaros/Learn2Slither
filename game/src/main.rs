mod contexts;
use convenient_lib::Void;
use piston_window::*;
use graphics::*;
use glyph_cache::rusttype::GlyphCache;
use playground::PlayGround;
use wgpu_graphics::{Texture, TextureContext, TextureSettings};
use std::path::Path;

enum Ctx {
    Lobby,
    Train,
    Test,
    Play
}

pub struct CtxValues<'a> {
    pub glyphs: GlyphCache<'a, TextureContext, Texture>,
    pub playground: Option<PlayGround>,
    pub ctx: Ctx
}

fn main() -> Void {
    let mut window: PistonWindow = WindowSettings::new("Learn2Slither", (1024, 768))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = Path::new("assets");
    let mut ctx_values = CtxValues {
        glyphs: window.load_font(
                    assets.join("FiraSans-Regular.ttf"),
                    TextureSettings::new()
                ).unwrap(),
        playground: None,
        ctx: Ctx::Lobby
    };

    window.set_lazy(true);

    while let Some(e) = window.next() {
        match ctx_values.ctx {
            Ctx::Lobby => contexts::lobby::lobby(&mut window, &e, &mut ctx_values),
            Ctx::Test => contexts::testing_board::testing_board(&mut window, &e, &mut ctx_values),
            Ctx::Train => contexts::training_board::training_board(&mut window, &e, &mut ctx_values),
            Ctx::Play => contexts::playing_board::playing_board(&mut window, &e, &mut ctx_values),
        }
    }
    Ok(())
}