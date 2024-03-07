#![allow(dead_code)]

use std::env;

use commands::{apply, create, init, list, workspace};
use utils::parse::{command_parser, Command};
mod commands;
mod dirs;
mod toml;
mod utils;

pub const CONFIG_FILENAME: &str = "config.toml";

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    let command = match command_parser(argv) {
        Ok(command) => command,
        Err(e) => {
            return eprintln!("{}", e);
        }
    };

    match command {
        Command::Init(opts) => {
            if let Err(e) = init::run(opts) {
                eprintln!("{:?}", e);
            }
        }

        Command::Create(opts) => {
            if let Err(e) = create::run(opts) {
                eprintln!("{:?}", e);
            }
        }
        Command::Apply(opts) => {
            if let Err(e) = apply::run(opts) {
                eprintln!("{:?}", e);
            }
        }
        Command::List() => {
            if let Err(e) = list::run() {
                eprintln!("{:?}", e);
            }
        }
        Command::Workspace(opts) => {
            if let Err(e) = workspace::run(opts) {
                eprintln!("{:?}", e);
            }
        }
    }
}
