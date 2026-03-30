use playground::{Dir};
use convenient_lib::{Res, Void};
use rand::{RngExt, make_rng, rngs::StdRng, seq::IndexedRandom};
use serde::{Serialize, Deserialize};
use std::io::{BufReader, BufWriter, Read};
use serde_json::json;
use std::fs;
use interpretors::state::{ETSFactory, ets_lib::ETS};

pub type QTable = Vec<Vec<f64>>;

#[derive(Serialize, Deserialize)]
struct Model {
    score: u32,
    model: QTable,
    ets_name: String,
    name: String
}

pub struct Agent {
    name: String,
    pub ets: Box<dyn ETS>,
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

    pub fn new(ets_name: String, name: String) -> Res<Agent> {
        // duplicate all data to avoid the Rust ownership issues.
        let ets_for_cardinal = ETSFactory::create(ets_name.clone())?;

        Ok(Agent {
            name: name,
            ets: ETSFactory::create(ets_name)?,
            q_table: vec![vec![1f64; 4]; ets_for_cardinal.get_cardinal()],
            best_models: vec![],
            learning_rate: 0.1,
            discount_factor: 0.,
            max_discount_factor: 0.9,
            exploration_rate: 0.5,
            seed: make_rng(),
            actions: [Dir::Up, Dir::Right, Dir::Down, Dir::Left]
        })
    }

    pub fn from(name: String, idx: usize, new_name: Option<&String>) -> Res<Agent> {
        let path = String::from("models/") + &name + "/" + &idx.to_string() + ".json";
        let file = fs::File::open(path)?;
        let mut contents = String::new();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut contents)?;

        let parsed_json = serde_json::from_str::<Model>(&contents)?;
        let q_table = parsed_json.model;
        let ets_name = parsed_json.ets_name;
        Ok(
            Agent {
                name: new_name.unwrap_or(&name).clone(),
                ets: ETSFactory::create(ets_name)?,
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

    pub fn save(&self) -> Void {
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
                            "name": self.name,
                            "ets_name": self.ets.get_name()
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
