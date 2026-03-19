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
use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn get_neigh(&self) -> [Coord; 4] {
        [
            Coord {
                x: self.x - 1,
                y: self.y
            },
            Coord {
                x: self.x + 1,
                y: self.y
            },
            Coord {
                x: self.x,
                y: self.y - 1
            },
            Coord {
                x: self.x,
                y: self.y + 1
            }
        ]
    }
}

struct PlayGround {
    height: usize,
    width: usize,
    seed: StdRng,
    green_1: Coord,
    green_2: Coord,
    red: Coord,
    snake: Vec<Coord>,
    counter: u64
}

impl PlayGround {

    fn _init_snake(height: usize, width: usize, snake: &mut VecDeque::<Coord>, seed: &mut StdRng) {
        match snake.back() {
            None => {
                snake.push_back(
                    Coord {
                        x: seed.random_range(0..width),
                        y: seed.random_range(0..height)
                    }
                )
            }
            Some(&c) => {
                c.get_neigh()
            }
        }
    }

    pub fn new(height: usize, width: usize) -> PlayGround {
        
        let mut seed: StdRng = make_rng();
        
        let mut snake = VecDeque::<Coord>::new();

        [0..3]
            .iter()
            .for_each(|_| {
                match snake.back() {
                    
                }
            });


        let green_1: Coord = Coord {
            x: seed.random_range(0..width),
            y: seed.random_range(0..height)
        };
        let green_2: 

        PlayGround {
            height,
            width,
            seed: seed,

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
