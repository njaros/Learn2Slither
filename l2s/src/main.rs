use clap::{Command, arg, command, value_parser};
use convenient_lib::Void;
use playground::{PlayGround, Dir};
use std::error::Error;
use qlearning::{Agent, StateInterpretor, RewardInterpretor};
use rand::{make_rng, rngs::StdRng};
use std::thread::sleep;
use std::time::Duration;

fn dir_to_usize(dir: Dir) -> usize {
    match dir {
        Dir::Up => 0,
        Dir::Right => 1,
        Dir::Down => 2,
        Dir::Left => 3
    }
}

fn main() -> Void {
    let matches = command!()
        .subcommand(Command::new("train").about("Train a new model.").args([
            arg!(-n --name <Name> "Name of the model.").required(true)
        ]))
        .subcommand(Command::new("test").about("use a model to play").args([
            arg!(-n --name <Name> "Name of the model.").required(true),
            arg!(-i --index <Index> "Index of the model (0 to 9").required(true).value_parser(value_parser!(usize)),
            arg!(-r --retrain <Name> "Start a new training from this model").required(false)
        ]))
        .subcommand_required(true)
        .get_matches();

    let subcommand = matches.subcommand_name().unwrap();
    let ctx = matches.subcommand_matches(subcommand).unwrap();
    let name = ctx.get_one::<String>("name").unwrap().clone();

    match subcommand {
        "train" => train(name)?,
        "test" => {
            let index = *ctx.get_one::<usize>("index").unwrap();
            let retrain = ctx.get_one::<String>("retrain");
            test(name, index, retrain)?
        },
        _ => unreachable!()
    };

    Ok(())
}

fn _train(agent: &mut Agent) -> Void {
    let mut reward_interpretor = RewardInterpretor::new();
    let sleep_time = Duration::from_millis(10);
    let mut best_score = 0u32;

    (1..2500)
        .for_each(|pouet| {
            let mut score = 0u32;
            let mut playground: PlayGround = PlayGround::new(10, 10, make_rng());
            let env = &playground.snake_view();
            reward_interpretor.init(env);
            let mut state = StateInterpretor::env_to_state(env);
            let mut dir = agent.play(state);
            let mut env = playground.next(dir);
            while playground.is_alive() {
                // print!("{playground}");
                let next_state = StateInterpretor::env_to_state(&env);
                let reward = reward_interpretor.get_reward(&env);
                agent.bellman(state, Some(next_state), dir_to_usize(dir), reward);
                state = next_state;
                // sleep(sleep_time);
                dir = agent.play(state);
                env = playground.next(dir);
                // println!("try: {pouet}: score: {score}, best: {best_score}");
            }
            agent.bellman(state, None, dir_to_usize(dir), reward_interpretor.end_training_reward);
            // print!("{playground}");
            score = playground.get_score();
            if score > best_score {
                best_score = score;
            }
            println!("try: {pouet}: score: {score}, best: {best_score}");
            agent.store_score(playground.get_score());
            agent.reduce_exploration_by(0.00025);
            agent.increase_discount_factor_by(0.0003);
        });

        agent.save()?;

    Ok(())
}

fn train(name: String) -> Void {
    let mut agent = Agent::new(4096, 4, name);
    _train(&mut agent)?;

    Ok(())
}

fn test(name: String, index: usize, retrain: Option<&String>) -> Void {
    let mut agent = Agent::from(name, index, retrain)?;
    if retrain.is_some() {
        _train(&mut agent)?;
    }
    else {
        let sleep_time = Duration::from_millis(100);
        let mut playground = PlayGround::new(10, 10, make_rng());
        while playground.is_alive() {
            println!("{playground}");
            sleep(sleep_time);
            let env = playground.snake_view();
            let state = StateInterpretor::env_to_state(&env);
            let dir = agent.play(state);
            playground.next(dir);
        }
        println!("score = {}", playground.get_score());
    }
    
    Ok(())
}