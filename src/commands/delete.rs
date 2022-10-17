use std::{fs, process};

use colored::Colorize;

use crate::utils::get_config;

pub fn run(name: &str) {
    let config = get_config().unwrap();

    let folder = config.default_template_dir.join(&name);

    if !folder.is_dir() {
        eprintln!("Template {} does not exist", &name.bright_purple());
        process::exit(1);
    }

    if !fs::remove_dir_all(folder).is_ok() {
        eprintln!("Was not able to delete template for some reason");
        process::exit(1);
    }

    println!("🚀 Successfully deleted cloup {}", &name.bright_purple());
}
