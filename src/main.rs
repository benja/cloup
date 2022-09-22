#![feature(result_option_inspect)]

mod utils;

use clap::Parser;
use std::env;

use utils::{ApplyCommands, Cli, Cloup, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();

    let cloup = Cloup::new();

    // println!("{} {:#?}", "Command ran:".bright_purple(), &command);

    match command {
        Command::Init { name } => Cloup::init(env::current_dir()?, name),
        Command::Create { name, files } => cloup.create(&name, files),
        Command::Apply { name, overwrite } => cloup.apply(&name, ApplyCommands { overwrite }),
        Command::Delete { name } => cloup.delete(&name),
        Command::List => cloup.list(),
    };

    Ok(())
}
