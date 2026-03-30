/// ETS for Environment_To_State.
/// Here can be listed all the different logic of the Q-Learning of the agent.

use playground::Tile;
use mathlib::pow;

type Env = Vec<Vec<Tile>>;

pub trait ETS {
	fn get_cardinal(&self) -> usize;
	fn env_to_state(&self, env: &Env) -> usize;
	fn description(&self) -> String;
	fn get_name(&self) -> String;
}

pub struct JajaV1 {}
impl ETS for JajaV1 {

	fn get_name(&self) -> String {
		String::from("jaja_v1")
	}

	fn get_cardinal(&self) -> usize {
		4096
	}

	fn description(&self) -> String {
		String::from("Take care about what is the nearest object and if it is juste besides for each size.")
	}

	fn env_to_state(&self, env: &Env) -> usize {	
		env
		.iter()
		.enumerate()
		.fold(0usize, |acc, (idx, line)| {
			let mut line_idx = 0usize;
			while line[line_idx] == Tile::Empty {
				line_idx += 1;
			}
			acc + (pow(8, idx) * match line[line_idx] {
				Tile::Wall => match line_idx { 0 => 0, _ => 1},
				Tile::Body => match line_idx { 0 => 2, _ => 3},
				Tile::Green => match line_idx { 0 => 4, _ => 5},
				Tile::Red => match line_idx { 0 => 6, _ => 7},
				_ => unreachable!()
			})
		})
	}
}

/// Rust compiler has a stupid rule that attempt to use dyn keyword on a trait having
/// just one structing using it.
/// I implemented Dummy to make Rust compiler happy...
/// It will be erased at Dadou_ETS implementation
pub struct Dummy{}
impl ETS for Dummy {

	fn get_name(&self) -> String {
		String::from("dummy")
	}

	fn description(&self) -> String {
		String::from("I'm a dummy")
	}
	fn env_to_state(&self, _env: &Env) -> usize {
		0
	}
	fn get_cardinal(&self) -> usize {
		1
	}
}