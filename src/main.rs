mod utils;

use clap::Parser;
use std::env;

use utils::{Cli, Cloup, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();

    let cloup = Cloup::new();

    match command {
        Commands::Init => Cloup::init(env::current_dir()?),
        Commands::Create { name } => cloup.create(&name),
    };

    Ok(())
}
