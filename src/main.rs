use clap::Parser;


mod hello_world;
mod guessing_game;


/// CLI interface.
#[derive(Parser)]
struct Cli {
    #[clap(default_value="hello_world")]
    example: String,
}


fn main() {
    let args: Cli = Cli::parse();

    match args.example.as_ref() {
        "hello_world" => hello_world::run(),
        "guessing_game" => guessing_game::run(),
        _ => panic!("Unknown command"),
    }
}