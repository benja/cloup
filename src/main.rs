mod commands;
mod dirs;
mod toml;
mod utils;

use commands::{apply, create, init, list, workspace};
use std::env;
use utils::parse::{command_parser, Command};

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = env::args().skip(1).collect();

    let command = command_parser(argv)?;

    match command {
        Command::Init(opts) => init::run(opts)?,
        Command::Create(opts) => create::run(opts)?,
        Command::Apply(opts) => apply::run(opts)?,
        Command::List() => list::run()?,
        Command::Workspace(opts) => workspace::run(opts)?,
    }

    Ok(())
}

fn main() {
    if let Err(err) = run_app() {
        eprintln!("{err}");
    }
}
