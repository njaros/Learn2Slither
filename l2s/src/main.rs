use clap::{Command, arg, command, value_parser};
use convenient_lib::Void;
use playground::PlayGround;
use qlearning::agent::agent::Agent;
use qlearning::train_loop;
use rand::make_rng;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Void {
    let matches = command!()
        .subcommand(Command::new("train").about("Train a new model.").args([
            arg!(-n --name <Name> "Name of the model.").required(true),
            arg!(-e --ets <Name> "Name of the ETS to use"),
            arg!(-f --from <Name> "Name of an existing model to train from"),
            arg!(-i --idx <String> "Index of the model taken from (usually 0 to 9 or s or l)"),
            arg!(-r --rounds <Usize> "Number of rounds for the training")
                .required(true)
                .value_parser(value_parser!(usize)),
            arg!(-d --display <U64> "Display mode with the time between each frame.")
                .value_parser(value_parser!(u64))
        ]))
        .subcommand(
            Command::new("test").about("use a model to play").args([
                arg!(-n --name <Name> "Name of the model.").required(true),
                arg!(-i --index <String> "Index of the model (usually 0 to 9 or s or l)"),
                arg!(-t --tall <Usize> "height of the board (3 to 100)")
                    .required(false)
                    .value_parser(value_parser!(usize)),
                arg!(-w --width <Usize> "width of the board (3 to 100)")
                    .required(false)
                    .value_parser(value_parser!(usize)),
                arg!(-s --sleep_time <U64> "time of a frame in millisec.")
                    .required(false)
                    .value_parser(value_parser!(u64)),
            ]),
        )
        .subcommand_required(true)
        .get_matches();

    let subcommand = matches.subcommand_name().unwrap();
    let ctx = matches.subcommand_matches(subcommand).unwrap();
    let name = ctx.get_one::<String>("name").unwrap().clone();

    match subcommand {
        "train" => {
            let ets = ctx
                .get_one::<String>("ets")
                .unwrap_or(&String::from("jaja_v1"))
                .clone();
            let rounds = *ctx.get_one::<usize>("rounds").unwrap();
            let display = ctx.get_one::<u64>("display");
            let from_model = ctx.get_one::<String>("from");
            let idx = ctx.get_one::<String>("idx").unwrap_or(&String::from("l")).clone();
            train(name, ets, rounds, from_model, idx, display)?
        }
        "test" => {
            let index = ctx.get_one::<String>("index").unwrap_or(&String::from("l")).clone();
            let height = *ctx.get_one::<usize>("tall").unwrap_or(&10usize);
            let width = *ctx.get_one::<usize>("width").unwrap_or(&10usize);
            let sleep_time = *ctx.get_one::<u64>("sleep_time").unwrap_or(&100);
            test(name, index, height, width, sleep_time)?
        }
        _ => unreachable!(),
    };

    Ok(())
}

fn train(name: String, ets_name: String, rounds: usize, from_model: Option<&String>, model_idx: String, display: Option<&u64>) -> Void {
    let mut agent = match from_model {
        None =>  Agent::new(ets_name, name)?,
        Some(m) => Agent::from(m.clone(), model_idx.clone(), Some(&name))?
    };
    train_loop(&mut agent, rounds, true, display)?;

    Ok(())
}

fn test(
    name: String,
    index: String,
    height: usize,
    width: usize,
    sleep_time: u64,
) -> Void {
    let mut agent = Agent::from(name, index, None)?;
    let sleep_time = Duration::from_millis(sleep_time);
    let mut playground = PlayGround::new(height, width, make_rng());
    while playground.is_alive() {
        playground.print_snake_view();
        sleep(sleep_time);
        let env = playground.snake_view();
        let state = agent.ets.env_to_state(&env);
        let dir = agent.play(state, true);
        playground.next(dir);
    }
    println!("{playground}");
    println!("DEAD ! score = {}", playground.get_score());
    Ok(())
}
