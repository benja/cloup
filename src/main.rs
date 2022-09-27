mod utils;

use clap::Parser;
use std::env;

use utils::{ApplyCommands, Cli, Cloup, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();

    if let Command::Init { name } = &command {
        Cloup::init(env::current_dir()?, name);
    }

    let cloup = Cloup::new();

    match command {
        Command::Create { name, files } => cloup.create(&name, files),
        Command::Apply { name, overwrite } => cloup.apply(&name, ApplyCommands { overwrite }),
        Command::Delete { name } => cloup.delete(&name),
        Command::List => cloup.list(),
        _ => (),
    };

    Ok(())
}
