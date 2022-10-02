use std::fs;

use colored::Colorize;

use crate::utils::get_config;

pub fn run() {
    let config = get_config();

    let folders: Vec<_> = fs::read_dir(config.default_template_dir)
    .expect("Template folder does not exist. Run `cloup init` in a folder to set it up as a template folder.")
    .filter(|f| f.as_ref().map(|f| !f.file_name().to_string_lossy().starts_with('.')).unwrap_or(false)).collect();

    if folders.is_empty() {
        eprintln!(
            "You have no cloups, create one with {}",
            "cloup create <name>".bright_purple()
        );
        return;
    } else {
        println!("Your available cloups:\n");
    }

    for folder in folders {
        println!(
            "— {}",
            folder
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string()
                .bright_purple()
        );
    }

    println!(
        "\nYou can apply a cloup with with {}",
        "cloup apply <name>".bright_purple()
    );
}
