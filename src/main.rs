mod utils;

use clap::Parser;
use std::env;

use utils::{Cli, Cloup, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();

    let cloup = Cloup::new();

    // println!("{} {:#?}", "Command ran:".bright_purple(), &command);

    match command {
        Command::Init { name } => Cloup::init(env::current_dir()?),
        Command::Create { name, files } => cloup.create(&name, files),
        Command::Apply { name } => cloup.apply(&name),
        Command::List => cloup.list(),
    };

    Ok(())
}
