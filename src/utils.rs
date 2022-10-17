use colored::Colorize;
use fs_extra::{dir, file};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::{env, path::PathBuf};
use std::{
    fs::{self},
    process::{self},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub default: bool,
    pub namespace: String,
    pub path: String,
}

/// The config returned by [get_config]. Includes the raw config, your current directory and the default template directory (for storing cloups)
/// so that we don't have to traverse the raw config every time we wish to access these values.
#[derive(Debug)]
pub struct ParsedConfig {
    pub raw: Config,
    pub current_dir: PathBuf,
    pub default_template_dir: PathBuf,
}

#[derive(Debug)]
pub enum ConfigParseError {
    OutdatedConfig,
    CouldNotParse,
    NotFound,
}

pub fn get_config() -> Result<ParsedConfig, ConfigParseError> {
    // read from config (from v.0.2.0 we changed the type of config, so parse error might happen)
    let dir = dirs::config_dir()
        .expect("Config dir could not be found")
        .join("cloup");

    // cloup init has to be ran before we can parse a config
    if fs::read_dir(&dir).is_err() {
        // eprintln!("Please run `cloup init` first in a template directory");
        return Err(ConfigParseError::OutdatedConfig);
    }

    // cloup config file location
    let file = dir.join(".config");

    // parse and serialize content
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Err(ConfigParseError::NotFound),
            _ => return Err(ConfigParseError::CouldNotParse),
        },
    };

    let content: Config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(_) => {
            // eprintln!(
            //     "You're running an outdated config, please use {} again in your template directory",
            //     "cloup init".bright_purple()
            // );
            return Err(ConfigParseError::OutdatedConfig);
        }
    };

    let current_dir = env::current_dir()
        .expect("Current directory might not exist or have the required permissions to continue");

    Ok(ParsedConfig {
        default_template_dir: PathBuf::from(
            &content
                .locations
                .iter()
                .find(|l| l.default)
                .expect("Default dir could not be found")
                .path,
        ),
        raw: content,
        current_dir,
    })
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

pub fn create_default_location(config_path: &PathBuf, namespace: &str, path: &str) {
    fs::write(
        config_path,
        toml::to_string(&Config {
            locations: vec![Location {
                default: true,
                namespace: namespace.to_string(),
                path: path.to_string(),
            }],
        })
        .unwrap(),
    )
    .expect("An error occurred when writing config file");

    println!("📚 Successfully created default template directory for cloups");
    process::exit(1);
}
