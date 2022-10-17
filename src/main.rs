mod cli;
mod commands;
mod tests;
mod utils;

use clap::Parser;
use cli::{ApplyCommands, Cli, Command, CreateCommands};
use commands::{apply, create, delete, init, list};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();

    match command {
        Command::Init { name } => init::run(name),
        Command::Create { name, files } => create::run(&name, CreateCommands { files }),
        Command::Apply { name, overwrite } => apply::run(&name, ApplyCommands { overwrite }),
        Command::Delete { name } => delete::run(&name),
        Command::List => list::run(),
    };

    Ok(())
}
