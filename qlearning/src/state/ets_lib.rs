use mathlib::pow;
/// ETS for Environment_To_State.
/// Here can be listed all the different logic of the Q-Learning of the agent.
use playground::Tile;

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
        "jaja_v1".into()
    }

    fn get_cardinal(&self) -> usize {
        4096
    }

    fn description(&self) -> String {
        "Take care about what is the nearest object and if it is just besides for each size.".into()
    }

    fn env_to_state(&self, env: &Env) -> usize {
        env.iter().enumerate().fold(0usize, |acc, (idx, line)| {
            let mut line_idx = 0usize;
            while line[line_idx] == Tile::Empty {
                line_idx += 1;
            }
            acc + (pow(8, idx)
                * match line[line_idx] {
                    Tile::Wall => match line_idx {
                        0 => 0,
                        _ => 1,
                    },
                    Tile::Body => match line_idx {
                        0 => 2,
                        _ => 3,
                    },
                    Tile::Green => match line_idx {
                        0 => 4,
                        _ => 5,
                    },
                    Tile::Red => match line_idx {
                        0 => 6,
                        _ => 7,
                    },
                    _ => unreachable!(),
                })
        })
    }
}

pub struct Dummy {}
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
