use std::env;
use std::fs;


mod commands;
use commands::Commands;
mod data_files;
mod stdin;
mod data;

fn main() {
    // create data file
    if !fs::exists(data::data_dir()).unwrap() {
        fs::create_dir(data::data_dir())
            .expect("Failed to create data directory");
    }

    // commands management
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let command: &str = &args[0].clone();

    match command {
        "new"     => Commands::new(&args[1]),
        "connect" => Commands::connect(&args[1]),
        "remove"  => Commands::remove(&args[1]),
        "list"    => Commands::list(),

        "help"    => Commands::help(),

        _         => println!("Command not found"),
    }
}

