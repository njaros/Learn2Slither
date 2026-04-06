use playground::Tile;

pub struct RewardInterpretor {
    old_env: Vec<Vec<Tile>>,
    pub end_training_reward: f64,
    starving: f64,
}

impl RewardInterpretor {
    pub fn new() -> RewardInterpretor {
        RewardInterpretor {
            old_env: vec![vec![]],
            end_training_reward: -100.,
            starving: 0.,
        }
    }

    fn _find_lower_dir(&self, new_env: &Vec<Vec<Tile>>) -> usize {
        new_env
            .iter()
            .zip(self.old_env.iter())
            .enumerate()
            .fold(0usize, |acc, (idx, (n, o))| match n.len() < o.len() {
                true => idx,
                false => acc,
            })
    }

    pub fn init(&mut self, init_env: &Vec<Vec<Tile>>) {
        self.old_env = init_env.clone();
        self.starving = 0.
    }

    pub fn get_reward(&mut self, new_env: &Vec<Vec<Tile>>) -> f64 {
        let row_idx = self._find_lower_dir(new_env);
        let reward = match self.old_env[row_idx][0] {
            Tile::Empty => {
                if self.starving < 10. {
                    self.starving += 1.;
                }
                -self.starving
            }
            Tile::Green => {
                self.starving = 0.;
                20.
            }
            Tile::Red => {
                if self.starving < 10. {
                    self.starving += 1.;
                }
                -30. - self.starving
            }
            Tile::Body => {
                if self.starving < 10. {
                    self.starving += 1.;
                }
                -60. - self.starving
            }
            _ => unreachable!(),
        };
        self.old_env = new_env.clone();
        reward
    }
}
