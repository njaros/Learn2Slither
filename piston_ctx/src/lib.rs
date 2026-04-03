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
    pub from: Option<(String, usize)>,
    pub from_bool: bool,
    pub from_model_name: String,
    pub from_model_idx: usize,
    pub from_model_cursor: usize,
    pub pause: bool,
    pub speed_time: usize,
    pub step_by_step: bool,
    pub name: String,
}

impl TrainingParams {
    pub fn new() -> Self {
        Self {
            interactive: false,
            rounds: 2500,
            ets: "jaja_v1".into(),
            from: None,
            from_bool: false,
            from_model_name: "".into(),
            from_model_idx: 0,
            from_model_cursor: 0,
            pause: false,
            speed_time: 100,
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
    pub lshift_pressed: bool,
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
