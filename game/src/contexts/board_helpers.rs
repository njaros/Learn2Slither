use piston::{Button, Key, PressEvent};
use piston_ctx::CtxValues;
use piston_window::graphics::{Rectangle, color};
use playground::{Dir, PlayGround, State, Tile};

pub fn tile_to_color(tile: &Tile) -> [f32; 4] {
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

pub struct Board {
    start_grid_x: f64,
    start_grid_y: f64,
    spacing: f64,
    rect_size: f64,
    grid: Vec<Vec<Rectangle>>
}

impl Board {
    pub fn new(engine: &PlayGround, snake_view: bool) -> Self {
        let start_grid_x = 11.;
        let start_grid_y = 10.;
        let max_size = std::cmp::max(engine.width, engine.height);
        let spacing = 100. / max_size as f64;
        let rect_size = 480. / max_size as f64;
        let state = engine.state;
        let mut grid = Vec::<Vec<Rectangle>>::new();
        
        if snake_view {
            let view = &engine.snake_view();
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
                State::Dead => match engine.get_score() {
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
        }
        else {
            let view = &engine.get_grid();
            view.iter().for_each(|row| {
                let mut rect_row = Vec::<Rectangle>::new();
                row.iter()
                    .for_each(|tile| rect_row.push(Rectangle::new(tile_to_color(tile))));
                grid.push(rect_row);
            });
        }

        Self {
            start_grid_x,
            start_grid_y,
            spacing,
            rect_size,
            grid
        }
    }

    pub fn draw(&self, c: &piston_window::graphics::Context, g: &mut piston_window::wgpu_graphics::WgpuGraphics<'_>) {
        self.grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, rect)| {
                rect.draw(
                    [
                        self.start_grid_x + (self.rect_size + self.spacing) * x as f64,
                        self.start_grid_y + (self.rect_size + self.spacing) * y as f64,
                        self.rect_size,
                        self.rect_size,
                    ],
                    &c.draw_state,
                    c.transform,
                    g,
                );
            });
        });
    }

}