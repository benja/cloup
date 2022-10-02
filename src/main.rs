mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{ApplyCommands, Cli, Command, CreateCommands, InitCommands};
use commands::{apply, create, delete, init, list};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();

    // If user has old config type, create new and delete old

    match command {
        Command::Init { namespace } => init::run(InitCommands { namespace }),
        Command::Create { name, files } => create::run(&name, CreateCommands { files }),
        Command::Apply { name, overwrite } => apply::run(&name, ApplyCommands { overwrite }),
        Command::Delete { name } => delete::run(&name),
        Command::List => list::run(),
    };

    Ok(())
}
