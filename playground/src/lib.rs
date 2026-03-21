/// Grid will manage the current state of the greed
/// It includes:
///     - Position of apples
///     - Position of snake (head + rest of his body)
///     - Position of walls
///     - Snake vision
///     - Snake state
///     - Count steps

use rand::prelude::*;
use rand::{make_rng, rngs::StdRng};
use std::collections::{VecDeque, HashSet};
use itertools::Itertools;
use std::fmt;

#[derive(Clone)]
pub enum Tile {
    Empty,
    Green,
    Red,
    Head,
    Body,
    Wall
}

#[derive(Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 
            match self {
                Tile::Empty => '.',
                Tile::Green => 'G',
                Tile::Red => 'R',
                Tile::Body => 'S',
                Tile::Head => 'H',
                Tile::Wall => '#',
            }
        )
    }
}

pub enum State {
    Alive,
    Dead
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    pub fn get_neigh(&self, limit_x: usize, limit_y: usize) -> Vec<Coord> {
        let mut neigh = Vec::<Coord>::new();
        if self.x != 1 { neigh.push(Coord{ x: self.x - 1, y: self.y }); }
        if self.x != limit_x - 1 { neigh.push(Coord{ x: self.x + 1, y: self.y }); }
        if self.y != 1 { neigh.push(Coord{ x: self.x, y: self.y - 1 }); }
        if self.y != limit_y - 1 { neigh.push(Coord{ x: self.x, y: self.y + 1 }); }
        neigh
    }
}

pub struct PlayGround {
    height: usize,
    width: usize,
    grid: Vec<Vec<Tile>>,
    // Empties is used as a pool of coordinates for all random picks.
    empties: HashSet<Coord>,
    snake: VecDeque<Coord>,
    counter: u64,
    state: State,
    seed: StdRng
}

impl PlayGround {

    fn _change_tile(&mut self, c: Coord, tile: Tile) {
        self.grid[c.y][c.x] = tile;
    }

    fn _init_empties(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.empties.insert(Coord{x: x + 1, y: y + 1});
            }
        }
    }

    fn _init_walls(&mut self) {
        [0, self.width + 1]
            .iter()
            .cartesian_product(0..self.height + 2)
            .for_each(|(x, y)| self.grid[y][*x] = Tile::Wall);
        [0, self.height + 1]
            .iter()
            .cartesian_product(1..self.width + 1)
            .for_each(|(y, x)| self.grid[*y][x] = Tile::Wall)
    }

    fn _init_snake(&mut self) {
        for _ in 0..3 {
            match self.snake.back() {
                None => {
                    self.snake.push_back(
                        Coord {
                            x: self.seed.random_range(1..self.width + 1),
                            y: self.seed.random_range(1..self.height + 1)
                        }
                    )
                },
                Some(c) => {
                    let mut neigh = c.get_neigh(self.width + 1, self.height + 1);
                    neigh.retain(|&n| n != *self.snake.front().unwrap());
                    self.snake.push_back(*neigh.choose(&mut self.seed).unwrap());
                }
            }
        }
        self.snake.clone() // no choice to clone here, otherwise rust complains for unanderstandable ownership reason.
            .iter()
            .enumerate()
            .for_each(|(idx, coord)| {
                match idx {
                    0 => self._change_tile(*coord, Tile::Head),
                    _ => self._change_tile(*coord, Tile::Body)
                }
                self.empties.remove(coord);
            });
    }

    fn _init_apples(&mut self) {
        for _ in 0..2 {
            self._place_apple(Tile::Green);
        }
        self._place_apple(Tile::Red);
    }

    fn _place_apple(&mut self, tile: Tile) -> bool {
        // For Rust non sense reason, I need to clone the empties structure...
        let rust_bug = self.empties.clone();
        let try_choose = rust_bug.iter().choose(&mut self.seed);
        match try_choose {
            None => false,
            Some(choosen) => {
                let empties = &mut self.empties;
                empties.remove(choosen);
                self._change_tile(*choosen, tile);
                true
            }
        }
    }

    fn _is_dead(&self, c: Coord) -> bool {
        self.coord_to_char(c) == '#'
        || (self.coord_to_char(c) == 'S' && c != *self.snake.back().unwrap())
        || (self.coord_to_char(c) == 'R' && self.snake.len() == 1)
    }
    
    fn tile_to_char(tile: &Tile) -> char {
        match tile {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Green => 'G',
            Tile::Red => 'R',
            Tile::Head => 'H',
            Tile::Body => 'S'
        }
    }

    fn coord_to_char(&self, c: Coord) -> char {
        PlayGround::tile_to_char(&self.grid[c.y][c.x])
    }

    pub fn is_alive(&self) -> bool {
        match self.state {
            State::Alive => true,
            _ => false
        }
    }

    /// Haut, bas, gauche, droite
    pub fn snake_view(&self) -> Vec<Vec<Tile>> {
        let mut view = Vec::<Vec<Tile>>::new();

        let head = self.snake.front().unwrap();
        let up_view = (0..head.y).rev()
            .fold(Vec::<Tile>::new(), |mut acc, y| {
                acc.push(self.grid[y][head.x].clone());
                acc
            });
        let down_view = (head.y + 1..self.height + 2)
            .fold(Vec::<Tile>::new(), |mut acc, y| {
                acc.push(self.grid[y][head.x].clone());
                acc
            });
        let left_view = (0..head.x).rev()
            .fold(Vec::<Tile>::new(), |mut acc, x| {
                acc.push(self.grid[head.y][x].clone());
                acc
            });
        let right_view = (head.x + 1..self.width + 2)
            .fold(Vec::<Tile>::new(), |mut acc, x| {
                acc.push(self.grid[head.y][x].clone());
                acc
            });

        view.push(up_view);
        view.push(down_view);
        view.push(left_view);
        view.push(right_view);

        view
    }

    pub fn print_snake_view(&self) {
        let view = self.snake_view();
        let indent_left = view[2].len();
        let indent_right = view[3].len();

        view[0]
            .iter()
            .rev()
            .for_each(|tile| {
                (0..indent_left)
                    .for_each(|_| print!(" "));
                print!("{tile}");
                (0..indent_right)
                    .for_each(|_| print!(" "));
                println!();
            });
        view[2]
            .iter()
            .rev()
            .for_each(|tile| {
                print!("{tile}")
            });
        print!("{}", Tile::Head);
        view[3]
            .iter()
            .for_each(|tile| {
                print!("{tile}")
            });
        println!();
        view[1]
            .iter()
            .for_each(|tile| {
                (0..indent_left)
                    .for_each(|_| print!(" "));
                print!("{tile}");
                (0..indent_right)
                    .for_each(|_| print!(" "));
                println!();
            });
    }

    pub fn next(&mut self, dir: Dir) -> State {
        let mut coord = self.snake[0].clone();
        match dir {
            Dir::Down => { coord.y += 1; }
            Dir::Up => { coord.y -= 1; }
            Dir::Left => { coord.x -= 1; }
            Dir::Right => { coord.x += 1; }
        }

        if self._is_dead(coord) {
            self.state = State::Dead;
            return State::Dead
        }

        let tile = self.grid[coord.y][coord.x].clone();
        self._change_tile(*self.snake.front().unwrap(), Tile::Body);
        self.snake.push_front(coord);
        self._change_tile(coord, Tile::Head);
        match tile {
            Tile::Empty => {
                self.empties.remove(&coord);
                let tail = self.snake.pop_back().unwrap();
                self.empties.insert(tail);
                self._change_tile(tail, Tile::Empty);
            },
            Tile::Green => {
                if !self._place_apple(Tile::Green) {
                    self.state = State::Dead;
                    return State::Dead;
                }
            },
            Tile::Red => {
                let tail_one = self.snake.pop_back().unwrap();
                self.empties.insert(tail_one);
                let tail_two = self.snake.pop_back().unwrap();
                self.empties.insert(tail_two);
                self._change_tile(tail_one, Tile::Empty);
                self._change_tile(tail_two, Tile::Empty);
                self._place_apple(Tile::Red);
            },
            Tile::Body => {
                self.empties.remove(&coord);
                let _ = self.snake.pop_back().unwrap();
            },
            _ => {
                unreachable!();
            }
        }

        State::Alive
    }

    pub fn new(height: usize, width: usize) -> PlayGround {
        
        let mut p = PlayGround {
            height,
            width,
            grid: vec![vec![Tile::Empty; width + 2]; height + 2],
            empties: HashSet::<Coord>::new(),
            seed: make_rng(),
            snake: VecDeque::<Coord>::new(),
            state: State::Alive,
            counter: 0
        };

        p._init_walls();
        p._init_empties();
        p._init_snake();
        p._init_apples();

        p
    }

}

impl fmt::Display for PlayGround {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut construct = String::new();
        construct.push('\n');
        self.grid
            .iter()
            .for_each(|row| {
                row
                    .iter()
                    .for_each(|tile| {
                        construct.push(PlayGround::tile_to_char(tile))
                    });
                construct.push('\n');
            });
        write!(f, "{}", construct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_playground() {
        let playgroung = PlayGround::new(10, 10);
        assert_eq!(playgroung.snake.len(), 3);
    }
}
