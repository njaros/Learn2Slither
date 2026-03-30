use clap::{Command, arg, command, value_parser};
use convenient_lib::Void;
use interpretors::{reward::reward_interpretor::RewardInterpretor, state};
use playground::{PlayGround, Dir};
use qlearning::Agent;
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
            arg!(-n --name <Name> "Name of the model.").required(true),
            arg!(-e --ets <Name> "Name of the ETS to use")
        ]))
        .subcommand(Command::new("test").about("use a model to play").args([
            arg!(-n --name <Name> "Name of the model.").required(true),
            arg!(-i --index <Index> "Index of the model (0 to 9").required(true)
                .value_parser(value_parser!(usize)),
            arg!(-r --retrain <Name> "Start a new training from this model").required(false),
            arg!(-t --tall <Usize> "height of the board (3 to 100)").required(false)
                .conflicts_with("retrain")
                .value_parser(value_parser!(usize)),
            arg!(-w --width <Usize> "width of the board (3 to 100)").required(false)
                .conflicts_with("retrain")
                .value_parser(value_parser!(usize)),
            arg!(-s --sleep_time <U64> "time of a frame in millisec.").required(false)
                .conflicts_with("retrain")
                .value_parser(value_parser!(u64))
        ]))
        .subcommand_required(true)
        .get_matches();

    let subcommand = matches.subcommand_name().unwrap();
    let ctx = matches.subcommand_matches(subcommand).unwrap();
    let name = ctx.get_one::<String>("name").unwrap().clone();

    match subcommand {
        "train" => {
            let ets = ctx.get_one::<String>("ets").unwrap_or(&String::from("jaja_v1")).clone();
            train(name, ets)?
        },
        "test" => {
            let index = *ctx.get_one::<usize>("index").unwrap();
            let retrain = ctx.get_one::<String>("retrain");
            let height = *ctx.get_one::<usize>("tall").unwrap_or(&10usize);
            let width = *ctx.get_one::<usize>("width").unwrap_or(&10usize);
            let sleep_time = *ctx.get_one::<u64>("sleep_time").unwrap_or(&100);
            test(name, index, height, width, sleep_time, retrain)?
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
            let mut state = agent.ets.env_to_state(env);
            let mut dir = agent.play(state);
            let mut env = playground.next(dir);
            while playground.is_alive() {
                // print!("{playground}");
                let next_state = agent.ets.env_to_state(&env);
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

fn train(name: String, ets_name: String) -> Void {
    let mut agent = Agent::new(ets_name, name)?;
    _train(&mut agent)?;

    Ok(())
}

fn test(name: String, index: usize, height: usize, width: usize, sleep_time: u64, retrain: Option<&String>) -> Void {
    let mut agent = Agent::from(name, index, retrain)?;
    if retrain.is_some() {
        _train(&mut agent)?;
    }
    else {
        let sleep_time = Duration::from_millis(sleep_time);
        let mut playground = PlayGround::new(height, width, make_rng());
        while playground.is_alive() {
            print!("{playground}");
            sleep(sleep_time);
            let env = playground.snake_view();
            let state = agent.ets.env_to_state(&env);
            let dir = agent.play(state);
            playground.next(dir);
        }
        println!("score = {}", playground.get_score());
    }
    
    Ok(())
}