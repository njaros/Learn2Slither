use piston_window::graphics::glyph_cache::rusttype::GlyphCache;
use piston_window::wgpu_graphics::{TextureContext, Texture};
use playground::PlayGround;

pub enum Ctx {
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