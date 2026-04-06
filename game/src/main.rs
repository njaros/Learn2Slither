mod contexts;
use convenient_lib::Void;
use piston_components::app_params::app_params::{
    AppParams, LeaderBoard, Route, TestingParams, TrainingParams,
};
use piston_window::{wgpu_graphics::Texture, *};
use std::{path::Path, time::Instant};
use wgpu_graphics::TextureSettings;

use crate::contexts::file_handler::{get_leaderboard, save_leaderboard};

const LEADERBOARD_PATH: &str = "leaderboard.json";

fn main() -> Void {
    let mut window: PistonWindow = WindowSettings::new("Learn2Slither", (1024, 768))
        .build()
        .unwrap();

    let assets = Path::new("assets");

    let mut app = AppParams {
        glyphs: window
            .load_font(assets.join("FiraSans-Regular.ttf"), TextureSettings::new())
            .unwrap(),
        route: Route::Lobby,
        logo: Texture::from_path(
            &mut window.create_texture_context(),
            assets.join("snake.jpg"),
            &TextureSettings::new(),
        )
        .unwrap(),
        mouse_pressed: false,
        lshift_pressed: false,
        last_mouse_pos: None,
        exit: false,
        last_training_frame: Instant::now(),
        playground: None,
        selected_height: 10,
        selected_width: 10,
        training_params: TrainingParams::new(),
        testing_params: TestingParams::new(),
        leaderboard: match get_leaderboard(LEADERBOARD_PATH) {
            Err(err) => {
                println!("{err}");
                LeaderBoard {
                    leaderboard: vec![],
                }
            }
            Ok(l) => l.clone(),
        },
        agent: None,
    };

    window.set_lazy(true);

    while let Some(e) = window.next()
        && !app.exit
    {
        match app.route {
            Route::Lobby => contexts::lobby::lobby(&mut window, &e, &mut app),
            Route::Test => contexts::testing::testing_route(&mut window, &e, &mut app),
            Route::Train => contexts::training::training_route(&mut window, &e, &mut app),
            Route::Play => contexts::playing::playing_route(&mut window, &e, &mut app),
        }

        // Saving mouse's params.
        if let Some(pos) = e.mouse_cursor_args() {
            app.last_mouse_pos = Some(pos);
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                app.mouse_pressed = true;
            }
            if button == Button::Keyboard(Key::LShift) {
                app.lshift_pressed = true;
            }
        }
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                app.mouse_pressed = false;
            }
            if button == Button::Keyboard(Key::LShift) {
                app.lshift_pressed = false;
            }
        }
    }

    save_leaderboard(LEADERBOARD_PATH, &app.leaderboard)?;

    Ok(())
}
