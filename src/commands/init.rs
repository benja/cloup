use std::{env, fs, io::ErrorKind};

use serde::{Deserialize, Serialize};

use crate::cli::InitCommands;

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

pub fn run(options: InitCommands) {
    // 1. read config, does it exist already? if so, we want to push a new location into config (provided they)
    // specified a namespace. if they did not specify a namespace, we want to overwrite / update the existing
    // default location. not everyone is gonna use namespaces, so we want to operate like before, where you could
    // only set one template_dir location
    // if config exists, read config and push to it with namespace
    // let config = get_config();

    // init should be responsible for setting up the initial version of the config

    // on install, for anyone that used `dirs::data_dir()` we have to migrate over to new config file

    // where config file is stored
    let config_dirname = dirs::config_dir()
        .expect("Config directory not found")
        .join("cloup");

    // create config file
    if let Err(e) = fs::create_dir(&config_dirname) {
        match e.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Permission denied when creating config directory")
            }
            _ => (),
        }
    }

    // TODO: Improve config file, use config-managable crate for this
    let file = toml::to_string(&Config {
        locations: vec![Location {
            default: true,
            namespace: options.namespace.unwrap_or("".to_string()),
            path: env::current_dir()
                .expect("Current dir should exist")
                .to_string_lossy()
                .to_string(),
        }],
    })
    .unwrap();

    fs::write(config_dirname.join(".config"), file)
        .expect("An error occurred when writing config file");

    println!("📚 Successfully made this the template directory for cloups");
}
