// use std::{env, fs, io::ErrorKind};

// use crate::constants::CONFIG_FILENAME;

// use colored::Colorize;

// use crate::{cli::InitCommands, utils::get_config};

// pub fn run(options: InitCommands) {
//     // folder path (string) for where the config file will be stored
//     let config_dirname = dirs::config_dir()
//         .expect("Config directory not found")
//         .join("cloup");

//     // config file path
//     let config_path = config_dirname.join(CONFIG_FILENAME);

//     // create config folder from string
//     if let Err(e) = fs::create_dir(&config_dirname) {
//         match e.kind() {
//             ErrorKind::PermissionDenied => {
//                 eprintln!("Permission denied when creating config directory")
//             }
//             _ => (),
//         }
//     }

//     // Attempt to read existing config, if this errors there might be no config yet
//     // and we have to handle the creation of a new config, or if there is a config
//     // already, we want to push this location into the list, provided the namespace
//     // does not collide with an existing namespace, if it collides we can either
//     // overwrite location and namespace or provide a warning message
//     let config = get_config();

//     let namespace = options.namespace.unwrap_or_default();
//     let path = env::current_dir()
//         .expect("Current dir should exist")
//         .to_string_lossy()
//         .to_string();

//     // if config didn't already exist, create config with namespace and path
//     if let Err(_) = &config {
//         create_default_location(&config_path, &namespace, &path);
//     }

//     // if config exists, there must be at least one location added prior
//     if let Ok(mut config) = config {
//         let one_location_exists = config.raw.locations.len() > 0;

//         // TODO: We currently overwrite namespaces by default, but it would be nice to let the user
//         // specify a flag that they want to overwrite
//         if one_location_exists {
//             // if namespace matches existing namespace (whether specified or default "")
//             // we want to overwrite the existing namespace
//             if config
//                 .raw
//                 .locations
//                 .iter()
//                 .find(|l| l.namespace == namespace)
//                 .is_some()
//             {
//                 config
//                     .raw
//                     .locations
//                     .iter_mut()
//                     .find(|l| l.namespace == namespace)
//                     .map(|l| l.path = path.clone());
//             } else {
//                 // add namespace to list
//                 config.raw.locations.push(Location {
//                     default: false,
//                     namespace: namespace.to_string(),
//                     path: path.clone(),
//                 });

//                 println!(
//                     "📚 Successfully added template directory with namespace {}",
//                     namespace.to_string().bright_purple()
//                 )
//             }

//             // write to file
//             fs::write(
//                 config_dirname.join(".config"),
//                 toml::to_string(&config.raw).unwrap(),
//             )
//             .expect("An error occurred when writing config file")
//         }
//     } else {
//         create_default_location(&config_path, &namespace, &path);
//     }
// }
