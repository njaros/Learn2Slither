use convenient_lib::Void;
use playground::{Dir, PlayGround};
use rand::make_rng;

use crate::{agent::agent::Agent, reward::reward_interpretor::RewardInterpretor};

pub mod agent;
pub mod reward;
pub mod state;

fn dir_to_usize(dir: Dir) -> usize {
    match dir {
        Dir::Up => 0,
        Dir::Right => 1,
        Dir::Down => 2,
        Dir::Left => 3,
    }
}

pub fn train_loop(agent: &mut Agent, rounds: usize, from_cli: bool) -> Void {
    let mut reward_interpretor = RewardInterpretor::new();
    let mut best_score = 0u32;

    (1..rounds).for_each(|round| {
        let mut playground: PlayGround = PlayGround::new(10, 10, make_rng());
        let env = &playground.snake_view();
        reward_interpretor.init(env);
        let mut state = agent.ets.env_to_state(env);
        let mut dir = agent.play(state);
        let mut env = playground.next(dir);
        while playground.is_alive() {
            let next_state = agent.ets.env_to_state(&env);
            let reward = reward_interpretor.get_reward(&env);
            agent.bellman(state, Some(next_state), dir_to_usize(dir), reward);
            state = next_state;
            dir = agent.play(state);
            env = playground.next(dir);
        }
        agent.bellman(
            state,
            None,
            dir_to_usize(dir),
            reward_interpretor.end_training_reward,
        );
        let score = playground.get_score();
        if from_cli {
            if score > best_score {
                best_score = score;
            }
            println!("try: {round}: score: {score}, best: {best_score}");
        }
        agent.store_score(score);
        agent.reduce_exploration_by(1. / (0.8 * rounds as f64));
        agent.increase_discount_factor_by(0.75 / rounds as f64);
    });

    agent.save()?;

    Ok(())
}
