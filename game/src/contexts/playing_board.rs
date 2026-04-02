use graphics::*;
use piston_components::components::{
    PistonComponent,
    buttons::{MyButton, Style},
    sliders::Slider,
};
use piston_window::{
    graphics::Context,
    wgpu::{Color, Device, hal::Rect},
    wgpu_graphics::WgpuGraphics,
};
use piston_window::{graphics::Text, *};
use playground::{Dir, PlayGround, State, Tile};
use rand::make_rng;
use wgpu_graphics::{Texture, TextureSettings};

use piston_ctx::{Ctx, CtxValues};

fn tile_to_color(tile: &Tile) -> [f32; 4] {
    match tile {
        Tile::Body => color::PURPLE,
        Tile::Boom => color::BLACK,
        Tile::Empty => color::WHITE,
        Tile::Green => color::GREEN,
        Tile::Head => color::OLIVE,
        Tile::Red => color::RED,
        Tile::Wall => color::hex("320000"),
    }
}

fn build_playground(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "      CANCEL".into(),
        |ctx| {
            ctx.ctx = Ctx::Lobby;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        },
    );

    let mut create_button = MyButton::new(
        Style::GREEN,
        [532., 650., 200., 75.],
        "     CREATE".into(),
        |ctx| {
            ctx.playground = Some(PlayGround::new(
                ctx.selected_height,
                ctx.selected_width,
                make_rng(),
            ));
        },
    );

    let mut slider_width =
        Slider::new(10, 50, ctx.selected_width, [200., 200., 500., 60.], |ctx| {
            ctx.selected_width =
                Slider::cursor_to_current(ctx.last_mouse_pos.unwrap()[0], 60., 200., 500., 10, 50)
        });

    let mut slider_height = Slider::new(
        10,
        50,
        ctx.selected_height,
        [200., 450., 500., 60.],
        |ctx| {
            ctx.selected_height =
                Slider::cursor_to_current(ctx.last_mouse_pos.unwrap()[0], 60., 200., 500., 10, 50)
        },
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let transform = c.transform.trans(210., 150.);
        Text::new(40)
            .draw(
                "Select the width (10 to 50)",
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_width.draw(&c, g, e, ctx);
        let transform = c.transform.trans(725., 240.);
        Text::new(40)
            .draw(
                &ctx.selected_width.to_string(),
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        let transform = c.transform.trans(210., 400.);
        Text::new(40)
            .draw(
                "Select the height (10 to 50)",
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();
        slider_height.draw(&c, g, e, ctx);
        let transform = c.transform.trans(725., 490.);
        Text::new(40)
            .draw(
                &ctx.selected_height.to_string(),
                &mut ctx.glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

        create_button.draw(&c, g, e, ctx);
        back_button.draw(&c, g, e, ctx);
    });

    slider_width.handle_event(e, ctx);
    slider_height.handle_event(e, ctx);
    create_button.handle_event(e, ctx);
    back_button.handle_event(e, ctx);
}

fn play(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    let mut back_button = MyButton::new(
        Style::RED,
        [772., 650., 200., 75.],
        "       HOME".into(),
        |ctx| {
            ctx.ctx = Ctx::Lobby;
            ctx.playground = None;
            ctx.selected_height = 10;
            ctx.selected_width = 10;
        },
    );

    window.draw_2d(e, |c, g, _| {
        clear([0.8, 0.8, 0.8, 1.0], g);

        let state = ctx.playground.as_ref().unwrap().state;
        let start_grid_x = 11.;
        let start_grid_y = 10.;
        let max_size = std::cmp::max(ctx.selected_width, ctx.selected_height);
        let spacing = 100. / max_size as f64;
        let rect_size = 480. / max_size as f64;
        let mut grid = Vec::<Vec<Rectangle>>::new();
        let score = ctx.playground.as_ref().unwrap().get_score();

        if state == State::Alive {
            let view = ctx.playground.as_ref().unwrap().snake_view();
            let unknown_left = view[2].len();
            let unknown_right = view[3].len();

            view[0].iter().rev().for_each(|tile| {
                let mut row = Vec::<Rectangle>::new();
                (0..unknown_left).for_each(|_| row.push(Rectangle::new(color::GRAY)));
                row.push(Rectangle::new(tile_to_color(tile)));
                (0..unknown_right).for_each(|_| row.push(Rectangle::new(color::GRAY)));
                grid.push(row);
            });
            let mut row = Vec::<Rectangle>::new();
            view[2]
                .iter()
                .rev()
                .for_each(|tile| row.push(Rectangle::new(tile_to_color(tile))));
            match state {
                State::Dead => match score {
                    0 => row.push(Rectangle::new(tile_to_color(&Tile::Empty))),
                    _ => row.push(Rectangle::new(tile_to_color(&Tile::Boom))),
                },
                _ => row.push(Rectangle::new(tile_to_color(&Tile::Head))),
            };
            view[3]
                .iter()
                .for_each(|tile| row.push(Rectangle::new(tile_to_color(tile))));
            grid.push(row);
            view[1].iter().for_each(|tile| {
                let mut row = Vec::<Rectangle>::new();
                (0..unknown_left).for_each(|_| row.push(Rectangle::new(color::GRAY)));
                row.push(Rectangle::new(tile_to_color(tile)));
                (0..unknown_right).for_each(|_| row.push(Rectangle::new(color::GRAY)));
                grid.push(row);
            });
        } else {
            let view = ctx.playground.as_ref().unwrap().get_grid();
            view.iter().for_each(|row| {
                let mut rect_row = Vec::<Rectangle>::new();
                row.iter()
                    .for_each(|tile| rect_row.push(Rectangle::new(tile_to_color(tile))));
                grid.push(rect_row);
            });
        }

        grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, rect)| {
                rect.draw(
                    [
                        start_grid_x + (rect_size + spacing) * x as f64,
                        start_grid_y + (rect_size + spacing) * y as f64,
                        rect_size,
                        rect_size,
                    ],
                    &c.draw_state,
                    c.transform,
                    g,
                );
            });
        });

        Text::new(40)
            .draw_pos(
                &format!("Score: {}", score.to_string()),
                [780., 550.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "HEAD",
                [825., 150.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "BODY",
                [825., 200.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "WALL",
                [825., 250.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "UNSEEN",
                [825., 300.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "RED APPLE",
                [825., 350.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Text::new(32)
            .draw_pos(
                "GREEN APPLE",
                [825., 400.],
                &mut ctx.glyphs,
                &c.draw_state,
                c.transform,
                g,
            )
            .unwrap();
        Rectangle::new(color::OLIVE).draw([780., 120., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::PURPLE).draw([780., 170., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::hex("320000")).draw(
            [780., 220., 40., 40.],
            &c.draw_state,
            c.transform,
            g,
        );
        Rectangle::new(color::GRAY).draw([780., 270., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::RED).draw([780., 320., 40., 40.], &c.draw_state, c.transform, g);
        Rectangle::new(color::GREEN).draw([780., 370., 40., 40.], &c.draw_state, c.transform, g);

        back_button.draw(&c, g, e, ctx);
    });

    back_button.handle_event(e, ctx);

    if let Some(button) = e.press_args() {
        if button == Button::Keyboard(Key::Right) || button == Button::Keyboard(Key::D) {
            ctx.playground.as_mut().unwrap().next(Dir::Right);
        } else if button == Button::Keyboard(Key::Down) || button == Button::Keyboard(Key::S) {
            ctx.playground.as_mut().unwrap().next(Dir::Down);
        } else if button == Button::Keyboard(Key::Up) || button == Button::Keyboard(Key::W) {
            ctx.playground.as_mut().unwrap().next(Dir::Up);
        } else if button == Button::Keyboard(Key::Left) || button == Button::Keyboard(Key::A) {
            ctx.playground.as_mut().unwrap().next(Dir::Left);
        }
    }
}

pub fn playing_board(window: &mut PistonWindow, e: &Event, ctx: &mut CtxValues) {
    match ctx.playground {
        None => build_playground(window, e, ctx),
        _ => play(window, e, ctx),
    }
}
