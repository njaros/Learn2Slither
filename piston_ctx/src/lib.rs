use std::time::Instant;

use convenient_lib::Res;
use interpretors::reward::reward_interpretor::RewardInterpretor;
use piston_window::graphics::glyph_cache::rusttype::GlyphCache;
use piston_window::wgpu_graphics::{Texture, TextureContext};
use playground::{Dir, PlayGround};
use qlearning::{Agent, Model};

pub enum Ctx {
    Lobby,
    Train,
    Test,
    Play,
}

pub struct TrainingParams {
    pub name: String,
    pub current_round: usize,
    pub rounds: usize,
    pub previous_state: Option<usize>,
    pub last_dir: Dir,
    pub train_finished: bool,
    pub rewarder: RewardInterpretor,
    pub ets_list: Vec<String>,
    pub ets: Option<String>,
    pub ets_cursor: usize,
    pub from_bool: bool,
    pub from_model_names: Res<Vec<String>>,
    pub from_model_name: String,
    pub from_model_cursor: usize,
    pub from_model_idx_list: Res<Vec<String>>,
    pub from_model_idx: String,
    pub from_model_idx_cursor: usize,
    pub from_model: Option<Model>,
    pub interactive: bool,
    pub pause: bool,
    pub snake_view: bool,
    pub speed_time: usize,
    pub step_by_step: bool,
}

impl TrainingParams {
    pub fn new() -> Self {
        Self {
            current_round: 0,
            rewarder: RewardInterpretor::new(),
            train_finished: false,
            interactive: false,
            rounds: 2500,
            previous_state: None,
            last_dir: Dir::Down,
            ets_list: vec![],
            ets: None,
            ets_cursor: 0,
            from_bool: false,
            from_model_names: Err("On init state".into()),
            from_model_name: "none".into(),
            from_model_cursor: 0,
            from_model_idx_list: Err("On init state".into()),
            from_model_idx: "none".into(),
            from_model_idx_cursor: 0,
            from_model: None,
            pause: false,
            snake_view: false,
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
    pub last_training_frame: Instant,

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
