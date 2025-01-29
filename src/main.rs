use std::env;


mod commands;
use commands::Commands;
mod data_files;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let command: &str = &args[0].clone();

    match command {
        "new"     => Commands::new(&args[1]),
        "connect" => Commands::connect(&args[1]),
        "remove"  => Commands::remove(&args[1]),
        "list"    => Commands::list(),

        _         => println!("Command not found"),
    }
}
