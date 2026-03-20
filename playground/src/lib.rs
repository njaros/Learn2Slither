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
use std::fmt;

#[derive(Clone)]
pub enum Tile {
    Empty,
    Green,
    Red,
    Head,
    Body
}

#[derive(Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
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
        if self.x != 0 { neigh.push(Coord{ x: self.x - 1, y: self.y }); }
        if self.x != limit_x - 1 { neigh.push(Coord{ x: self.x + 1, y: self.y }); }
        if self.y != 0 { neigh.push(Coord{ x: self.x, y: self.y - 1 }); }
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
                self.empties.insert(Coord{x: x, y: y});
            }
        }
    }

    fn _init_snake(&mut self) {
        for _ in 0..3 {
            match self.snake.back() {
                None => {
                    self.snake.push_back(
                        Coord {
                            x: self.seed.random_range(0..self.width),
                            y: self.seed.random_range(0..self.height)
                        }
                    )
                },
                Some(c) => {
                    let mut neigh = c.get_neigh(self.width, self.height);
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

    fn _place_apple(&mut self, tile: Tile) {
        // For Rust bug reason, I need to clone the empties structure...
        let rust_bug = self.empties.clone();
        let choosen = rust_bug.iter().choose(&mut self.seed).unwrap();
        let empties = &mut self.empties;
        empties.remove(choosen);
        self._change_tile(*choosen, tile);
    }

    fn _is_dead(&self, c: Coord) -> bool {
        c.x == usize::MAX || c.y == usize::MAX
        || c.x == self.width || c.y == self.height
        || (self.coord_to_char(c) == 'S' && c != *self.snake.back().unwrap())
        || (self.coord_to_char(c) == 'R' && self.snake.len() == 1)
    }
    
    fn tile_to_char(tile: &Tile) -> char {
        match tile {
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
                self._place_apple(Tile::Green);
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
            grid: vec![vec![Tile::Empty; width]; height],
            empties: HashSet::<Coord>::new(),
            seed: make_rng(),
            snake: VecDeque::<Coord>::new(),
            state: State::Alive,
            counter: 0
        };

        p._init_empties();
        p._init_snake();
        p._init_apples();

        p
    }

}

impl fmt::Display for PlayGround {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut construct = String::new();
        for _ in 0..self.width + 2 {
            construct.push('#');
        }
        construct.push('\n');
        self.grid
            .iter()
            .for_each(|row| {
                construct.push('#');
                row
                    .iter()
                    .for_each(|tile| {
                        construct.push(PlayGround::tile_to_char(tile))
                    });
                construct.push('#');
                construct.push('\n');
            });
        for _ in 0..self.width + 2 {
            construct.push('#');
        }
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
