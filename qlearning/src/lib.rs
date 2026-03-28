use mathlib::pow;
use playground::{Dir, Tile};
use rand::{RngExt, make_rng, rngs::StdRng, seq::IndexedRandom};
use serde::{Serialize, Deserialize};
use std::io::{BufReader, BufWriter, Read};
use serde_json::json;
use std::fs;
use std::error::Error;

pub struct StateInterpretor {}
pub struct RewardInterpretor {
    old_env: Vec<Vec<Tile>>,
    pub end_training_reward: f64
}
// HAUT: idx = 0, DROITE 1, BAS 2, GAUCHE 3
impl StateInterpretor {
    pub fn env_to_state(env: &Vec<Vec<Tile>>) -> usize {
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

impl RewardInterpretor {

    pub fn new() -> RewardInterpretor {
        RewardInterpretor { old_env: vec![vec![]], end_training_reward: -100. }
    }

    fn _find_lower_dir(&self, new_env: &Vec<Vec<Tile>>) -> usize {
        new_env
            .iter()
            .zip(self.old_env.iter())
            .enumerate()
            .fold(0usize, |acc, (idx, (n, o))| {
                match n.len() < o.len() {
                    true => idx,
                    false => acc
                }
            })
    }

    pub fn init(&mut self, init_env: &Vec<Vec<Tile>>) {
        self.old_env = init_env.clone();
    }

    pub fn get_reward(&mut self, new_env: &Vec<Vec<Tile>>) -> f64 {
        let row_idx = self._find_lower_dir(new_env);
        let reward = match self.old_env[row_idx][0] {
            Tile::Empty => -1.,
            Tile::Green => 20.,
            Tile::Red => -30.,
            Tile::Body => -30.,
            _ => unreachable!()
        };
        self.old_env = new_env.clone();
        reward
    }

}

pub type QTable = Vec<Vec<f64>>;

#[derive(Serialize, Deserialize)]
struct Model {
    score: u32,
    model: QTable,
    name: String
}

pub struct Agent {
    name: String,
    q_table: QTable,
    best_models: Vec<(QTable, u32)>,
    learning_rate: f64,
    max_discount_factor: f64,
    discount_factor: f64,
    exploration_rate: f64,
    seed: StdRng,
    actions: [Dir; 4]
}

impl Agent {

    pub fn new(state_cardinal: usize, action_cardinal: usize, name: String) -> Agent {
        Agent {
            name: name,
            q_table: vec![vec![1f64; action_cardinal]; state_cardinal],
            best_models: vec![],
            learning_rate: 0.1,
            discount_factor: 0.,
            max_discount_factor: 0.9,
            exploration_rate: 0.5,
            seed: make_rng(),
            actions: [Dir::Up, Dir::Right, Dir::Down, Dir::Left]
        }
    }

    pub fn from(name: String, idx: usize, new_name: Option<&String>) -> Result<Agent, Box<dyn Error>> {
        let path = String::from("models/") + &name + "/" + &idx.to_string() + ".json";
        let file = fs::File::open(path)?;
        let mut contents = String::new();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut contents)?;

        let parsed_json = serde_json::from_str::<Model>(&contents)?;
        let q_table = parsed_json.model;
        Ok(
            Agent {
                name: new_name.unwrap_or(&name).clone(),
                q_table,
                best_models: vec![],
                learning_rate: 0.1,
                max_discount_factor: 0.9,
                discount_factor: 0.9,
                exploration_rate: 0.,
                seed: make_rng(),
                actions: [Dir::Up, Dir::Right, Dir::Down, Dir::Left]
            }
        )
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let folder_path = String::from("models/") + &self.name;
        fs::create_dir_all(&folder_path)?;
        self.best_models
            .iter()
            .enumerate()
            .try_for_each(|(idx, (model, score))| {
                let path = folder_path.clone() + "/" + &idx.to_string() + ".json";
                let file = fs::File::create(&path);
                if file.is_ok() {
                    let mut writer = BufWriter::new(file.expect("cannot recover the original error because try_for_each is a shit feature."));
                    let v = json!(
                        {
                            "score": score,
                            "model": model,
                            "name": self.name
                        }
                    );
                    match serde_json::to_writer_pretty(&mut writer, &v) {
                        Ok(_) => {},
                        Err(_) => return Err("le try_for_each c'est de la merde")
                    }
                }
                else {
                    return Err("le try_for_each c'est de la merde");
                }
                Ok(())
            })?;
        Ok(())
    }

    pub fn store_score(&mut self, score: u32) {
        if self.best_models.len() < 10 {
            self.best_models.push((self.q_table.clone(), score))
        }
        else {
            let (idx, lesser_score) = self.best_models
                .iter()
                .enumerate()
                .fold((0usize, u32::MAX), |(idx_less, less), (idx, (_, curr))| {
                    match less < *curr {
                        true => (idx_less, less),
                        false => (idx, *curr)
                    }
                });
            match lesser_score < score {
                true => {
                    self.best_models.remove(idx);
                    self.best_models.push((self.q_table.clone(), score))
                },
                false => {}
            }
        }
    }

    pub fn reduce_exploration_by(&mut self, reducer: f64) {
        if self.exploration_rate > 0. {
            self.exploration_rate -= reducer;
        }
    }

    pub fn increase_discount_factor_by(&mut self, increaser: f64) {
        if self.discount_factor < self.max_discount_factor {
            self.discount_factor += increaser;
            if self.discount_factor > self.max_discount_factor {
                self.discount_factor = self.max_discount_factor;
            }
        }
    }

    pub fn bellman(&mut self, previous_state: usize, new_state: Option<usize>, action: usize, reward: f64) {
        let current_val = self.q_table[previous_state][action];
        self.q_table[previous_state][action] =
            (1f64 - self.learning_rate) * current_val +
            self.learning_rate * (reward + self.discount_factor * match new_state {
                None => 0f64,
                Some(state) => {
                    self.q_table[state]
                        .iter()
                        .fold(f64::MIN, |acc, &n| f64::max(acc, n))
                }
            })
    }

    pub fn play(&mut self, state: usize) -> Dir {
        if self.exploration_rate > 0. && self.exploration_rate > self.seed.random_range(0f64..1.)  {
            return *self.actions.choose(&mut self.seed).unwrap();
        }
        self.actions[self.q_table[state]
            .iter()
            .enumerate()
            .fold((0usize, f64::MIN), |(max_idx, acc_max), (idx, &n)| {
                if acc_max > n {
                    return (max_idx, acc_max)
                }
                else {
                    return (idx, n)
                }
            }).0]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
