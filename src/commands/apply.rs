use std::process;

use colored::Colorize;
use fs_extra::dir;

use crate::{cli::ApplyCommands, utils::get_config};

pub fn run(name: &str, options: ApplyCommands) {
    let config = get_config().unwrap();

    let template_dir = config.default_template_dir.join(&name);

    if !template_dir.is_dir() {
        eprint!(
            "A cloup with the name {} does not exist",
            &name.to_string().bright_purple()
        );
        process::exit(1);
    }

    // copy all files from the template dir to current dir
    let _ = fs_extra::dir::copy(
        &template_dir,
        &config.current_dir,
        &dir::CopyOptions::from(dir::CopyOptions {
            content_only: true,
            skip_exist: true,
            overwrite: options.overwrite,
            ..Default::default()
        }),
    )
    .map_err(|_| {
        eprintln!(
            "Some files could not be written, to overwrite add the {} flag to the same command",
            "-o".to_string().bright_purple()
        );
        process::exit(1);
    });

    eprintln!(
        "To overwrite existing files, add the {} flag to the same command",
        "-o".to_string().bright_purple()
    );

    println!(
        "🎨 Successfully applied cloup {}",
        &name.to_string().bright_purple(),
    );
}
