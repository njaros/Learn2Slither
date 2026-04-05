use std::time::Instant;

use convenient_lib::Res;
use interpretors::reward::reward_interpretor::RewardInterpretor;
use piston_window::graphics::glyph_cache::rusttype::GlyphCache;
use piston_window::wgpu_graphics::{Texture, TextureContext};
use playground::{Dir, PlayGround};
use qlearning::{Agent, Model};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaderBoardItem {
    pub score: u32,
    pub model_name: String,
    pub ets_name: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaderBoard {
    pub leaderboard: Vec<LeaderBoardItem>
}

pub enum Ctx {
    Lobby,
    Train,
    Test,
    Play,
}

pub struct TestingParams {
    pub model_names: Res<Vec<String>>,
    pub model_name: String,
    pub model_cursor: usize,
    pub model_idx_list: Res<Vec<String>>,
    pub model_idx: String,
    pub model_idx_cursor: usize,
    pub model: Option<Model>,
    pub speed_time: usize,
    pub pause: bool,
    pub next_step: bool,
    pub snake_view: bool,
    pub infinite_loop: bool,
}

impl TestingParams {
    pub fn new() -> Self {
        Self {
            model_names: Err("On init state".into()),
            model_name: "none".into(),
            model_cursor: 0,
            model_idx_list: Err("On init state".into()),
            model_idx: "none".into(),
            model_idx_cursor: 0,
            model: None,
            speed_time: 100,
            pause: false,
            next_step: false,
            snake_view: false,
            infinite_loop: true
        }
    }
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
    pub next_step: bool,
    pub just_snapshoted: bool,
    pub just_save_all: bool
}

impl TrainingParams {
    pub fn new() -> Self {
        Self {
            name: "no_name".into(),
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
            next_step: false,
            just_snapshoted: false,
            just_save_all: false
        }
    }
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
    
    // Testing params
    pub testing_params: TestingParams,
    pub leaderboard: LeaderBoard,
    
    // Agent
    pub agent: Option<Agent>,
}
