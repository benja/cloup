use std::{env, fs, io::ErrorKind};

use crate::{constants::CONFIG_FILENAME, utils::create_default_location};

use crate::utils::{get_config, Location};

pub fn run() {
    // folder path (string) for where the config file will be stored
    let config_dirname = dirs::config_dir()
        .expect("Config directory not found")
        .join("cloup");

    // config file path
    let config_path = config_dirname.join(CONFIG_FILENAME);

    // create config folder from string
    if let Err(e) = fs::create_dir(&config_dirname) {
        match e.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Permission denied when creating config directory")
            }
            _ => (),
        }
    }

    // Attempt to read existing config, if this errors there might be no config yet
    // and we have to handle the creation of a new config, or if there is a config
    // already, we want to push this location into the list, provided the namespace
    // does not collide with an existing namespace, if it collides we can either
    // overwrite location and namespace or provide a warning message
    let config = get_config();

    let path = env::current_dir()
        .expect("Current dir should exist")
        .to_string_lossy()
        .to_string();

    match config {
        Ok(mut config) => {
            let default_namespace_exists = config
                .raw
                .locations
                .iter()
                .find(|l| l.namespace == "default")
                .is_some();

            if default_namespace_exists {
                config
                    .raw
                    .locations
                    .iter_mut()
                    .find(|l| l.namespace == "default")
                    .map(|l| l.path = path.clone());
            } else {
                // there may be other namespaces, just not the default one
                config.raw.locations.push(Location {
                    default: false,
                    namespace: "default".to_string(),
                    path: path.clone(),
                });
            }

            fs::write(
                config_dirname.join(".config"),
                toml::to_string(&config.raw).unwrap(),
            )
            .expect("An error occurred when writing config file");

            println!("📚 Successfully created default template directory");
        }
        // if config didn't already exist, create config with default namespace and path
        Err(_) => create_default_location(&config_path, "default", &path),
    }
}
