use std::{fs, process};

use colored::Colorize;
use fs_extra::dir;

use crate::{
    cli::CreateCommands,
    utils::{copy_file_to_template, get_config},
};

pub fn run(name: &str, options: CreateCommands) {
    let config = get_config();

    let current_dir = config.current_dir;
    let template_dir = config.template_dir.join(&name);

    if name.starts_with('.') {
        eprintln!("Name of template should not start with a dot");
        process::exit(1);
    }

    if fs::create_dir(&template_dir).is_err() {
        eprintln!("Template {} already exists", &name.bright_purple());
        process::exit(1);
    }

    // If the files vec is empty, we know they want to use the entire folder as a template
    if options.files.is_empty() {
        fs_extra::dir::copy(
            &current_dir,
            &template_dir,
            &dir::CopyOptions {
                content_only: true,
                ..Default::default()
            },
        )
        .expect("Template could not be created based on folder");
    }

    for file in options.files {
        let file_path = current_dir.join(&file);

        // Check if referenced file does exist
        if !file_path.exists() {
            eprintln!(
                "{} {:?} {}",
                "The path,".bright_red(),
                file_path,
                ", does not exist".bright_red()
            );
            process::exit(1);
        }

        copy_file_to_template(
            &file_path,
            &template_dir,
            (!file_path.is_dir()).then_some(&file),
        );
    }

    println!(
        "🚀 Successfully created cloup {} \n\n{}",
        &name.to_string().bright_purple(),
        format!("Apply this cloup with `cloup apply {}`", &name)
    );
}
