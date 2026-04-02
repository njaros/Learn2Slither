use piston_window::graphics::glyph_cache::rusttype::GlyphCache;
use piston_window::wgpu_graphics::{Texture, TextureContext};
use playground::PlayGround;
use qlearning::Agent;

pub enum Ctx {
    Lobby,
    Train,
    Test,
    Play,
}

pub struct TrainingParams {
    pub interactive: bool,
    pub rounds: usize,
    pub ets: String,
    pub from: Option<String>,
    pub pause: bool,
    pub speed: f64,
    pub step_by_step: bool,
    pub name: String,
}

impl TrainingParams {
    pub fn new() -> Self {
        Self {
            interactive: true,
            rounds: 2500,
            ets: "jajav1".into(),
            from: None,
            pause: false,
            speed: 0.5,
            step_by_step: false,
            name: "no_name".into(),
        }
    }
}

pub struct SelectModelParams {
    pub name: String,
    pub index: usize,
}

pub struct CtxValues<'a> {
    // Caches
    pub glyphs: GlyphCache<'a, TextureContext, Texture>,

    // App navigation contexts
    pub ctx: Ctx,
    pub mouse_pressed: bool,
    pub last_mouse_pos: Option<[f64; 2]>,
    pub exit: bool,

    // Playground params
    pub playground: Option<PlayGround>,
    pub selected_width: usize,
    pub selected_height: usize,

    // Training params
    pub training_params: TrainingParams,
    pub agent: Option<Agent>,

    // Select model params
    pub model: Option<SelectModelParams>,
}
