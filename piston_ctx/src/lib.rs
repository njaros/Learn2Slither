use piston_window::graphics::glyph_cache::rusttype::GlyphCache;
use piston_window::wgpu_graphics::{Texture, TextureContext};
use playground::PlayGround;

pub enum Ctx {
    Lobby,
    Train,
    Test,
    Play,
}

pub struct CtxValues<'a> {
    // Caches
    pub glyphs: GlyphCache<'a, TextureContext, Texture>,

    // App nav contexts
    pub ctx: Ctx,
    pub mouse_pressed: bool,
    pub last_mouse_pos: Option<[f64; 2]>,
    pub exit: bool,

    // Playground params
    pub playground: Option<PlayGround>,
    pub selected_width: usize,
    pub selected_height: usize,
}
