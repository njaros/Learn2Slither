use std::{thread::sleep, time::Duration};

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

pub fn train_loop(agent: &mut Agent, rounds: usize, display_rounds: bool, display_sleep: Option<&u64>) -> Void {
    let mut reward_interpretor = RewardInterpretor::new();
    let mut best_score = 0u32;
    let mut current_score = 0u32;

    (1..rounds).for_each(|round| {
        let mut playground: PlayGround = PlayGround::new(10, 10, make_rng());
        let env = &playground.snake_view();
        if let Some(sleep_time) = display_sleep {
            playground.print_snake_view();
            sleep(Duration::from_millis(*sleep_time));
        }
        reward_interpretor.init(env);
        let mut state = agent.ets.env_to_state(env);
        let mut dir = agent.play(state, display_sleep.is_some());
        let mut env = playground.next(dir);
        while playground.is_alive() {
            if let Some(sleep_time) = display_sleep {
                playground.print_snake_view();
                sleep(Duration::from_millis(*sleep_time));
            }
            let next_state = agent.ets.env_to_state(&env);
            let reward = reward_interpretor.get_reward(&env);
            agent.bellman(state, Some(next_state), dir_to_usize(dir), reward);
            state = next_state;
            dir = agent.play(state, display_sleep.is_some());
            env = playground.next(dir);
        }
        if let Some(sleep_time) = display_sleep {
            println!("{playground}");
            sleep(Duration::from_millis(*sleep_time));
        }
        agent.bellman(
            state,
            None,
            dir_to_usize(dir),
            reward_interpretor.end_training_reward,
        );
        current_score = playground.get_score();
        if display_rounds {
            if current_score > best_score {
                best_score = current_score;
            }
            println!("try: {round}: score: {:03}, best: {:03} | current explo_rate: {}, current discount_factor: {}", current_score, best_score, agent.exploration_rate, agent.discount_factor);
        }
        agent.store_score(current_score);
        agent.reduce_exploration_by(5. / (8. * rounds as f64));
        agent.increase_discount_factor_by(1. / rounds as f64);
    });

    agent.save()?;
    agent.snapshot(current_score, 'l')?;

    Ok(())
}
