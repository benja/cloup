use colored::Colorize;
use fs_extra::{dir, file};
use std::{env, path::PathBuf};
use std::{
    fs::{self},
    process::{self},
};

pub struct Cloup {
    pub current_dir: PathBuf,
    pub template_dir: PathBuf,
    // TODO: ignored_paths: Vec<String>,
}

pub fn get_config() -> Cloup {
    let template_dir = template_dir();
    let current_dir = env::current_dir();

    if template_dir.is_err() {
        eprint!(
            "Please run {} to initialise a template folder before creating cloups",
            "cloup init".bright_purple()
        );
        process::exit(1);
    }

    if current_dir.is_err() {
        eprint!("Current directory might not exist or have the required permissions to continue");
        process::exit(1);
    }

    Cloup {
        current_dir: current_dir.unwrap(),
        template_dir: template_dir.unwrap(),
    }
}

pub fn template_dir() -> Result<PathBuf, String> {
    let cloup_config_dir = dirs::data_dir()
        .expect("Data directory could not be found")
        .join("cloup");

    if fs::read_dir(&cloup_config_dir).is_err() {
        return Err("Please run `cloup init` first in a template directory".to_string());
    }

    let mut content = match fs::read_to_string(cloup_config_dir.join(".config")) {
        Ok(c) => c,
        Err(_) => return Err("Config could not be parsed".to_string()),
    };

    if content.len() > 13 {
        content.replace_range(0..13, "");
        content = content.replace('"', "");
    }

    Ok(PathBuf::from(content))
}

pub fn copy_file_to_template(file_path: &PathBuf, template_dir: &PathBuf, file: Option<&String>) {
    // If no file is specified, we are working with a folder
    if let None = file {
        fs_extra::dir::copy(file_path, &template_dir, &dir::CopyOptions::new())
            .map_err(|e| {
                eprintln!("{}", e);
                process::exit(1);
            })
            .ok();

        return;
    }

    let template_path = template_dir.join(&file.expect("File should exist"));

    // create sub folders for file to be allowed to move file into that folder
    template_path
        .parent()
        .filter(|p| !p.is_dir())
        .map(|p| fs::create_dir_all(p));

    fs_extra::file::copy(
        &file_path,
        template_path,
        &file::CopyOptions::from(file::CopyOptions {
            overwrite: true,
            ..Default::default()
        }),
    )
    .map_err(|e| {
        fs::remove_dir(&template_dir).expect("Should be allowed to remove dir");
        eprintln!("{}", e);
        process::exit(1);
    })
    .ok();
}
