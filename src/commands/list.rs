use std::fs;

use colored::Colorize;

use crate::utils::get_config;

// TODO: List command is not properly displaying the available cloups in the workspaces. Default should also be a "namespace"

pub fn run() {
    let config = get_config().unwrap();

    println!("\nYour available cloups:\n");

    for location in config.raw.locations {
        let folders: Vec<_> = fs::read_dir(location.path)
        .expect("Template folder does not exist. Run `cloup init` in a folder to set it up as a template folder.")
        .filter(|f| f.as_ref().map(|f| !f.file_name().to_string_lossy().starts_with('.'))
        .unwrap_or(false)).collect();

        if folders.is_empty() {
            if location.namespace.len() > 0 {
                eprintln!("\n {}", location.namespace.bright_blue());
            } else {
                eprint!("\n {}", "Default".bright_blue())
            }

            eprintln!(
                "\nYou have no cloups, create one with {} {}",
                "cloup create <name> -n".bright_purple(),
                location.namespace.bright_purple()
            );
        } else {
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
        }
    }

    println!(
        "\nYou can apply a cloup with with {}",
        "cloup apply <name>".bright_purple()
    );
}
