mod hello_world;
mod guessing_game;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return hello_world::run();
    }

    match args[1].as_str() {
        "hello_world" => hello_world::run(),
        "guessing_game" => guessing_game::run(),
        _ => panic!("Unknown command"),
    }
}