use crate::{
    toml::data::{TomlValue, TomlValueKind},
    utils::config::{get_config, ConfigError},
};
use std::{fs, io};

#[derive(Debug)]
pub enum InitError {
    Error(io::Error),
    ConfigError(ConfigError),
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InitError::Error(e) => write!(f, "Error: {}", e),
            InitError::ConfigError(e) => write!(f, "Config error: {}", e),
        }
    }
}
impl std::error::Error for InitError {}

#[derive(Debug)]
pub struct InitOpts {
    // Overwrite path
    pub overwrite: bool,

    // Workspace name
    pub workspace: Option<String>,
}

pub fn run(opts: InitOpts) -> Result<(), InitError> {
    let mut config = get_config().map_err(InitError::ConfigError)?;
    let mut toml = config.toml;

    // unless user is making a custom workspace
    if config.initial_run && opts.workspace.is_none() {
        println!(
            "\x1b[1;32m»\x1b[0m Created new workspace for storing cloups: default ({}).",
            config.current_dir.to_string_lossy()
        );
        return Ok(());
    }

    // todo: don't panic, send error back
    let active_workspace = config
        .data
        .find(|w| w.active)
        .expect("Active workspace does not exist");

    if opts.workspace.is_none() {
        if !opts.overwrite {
            println!(
                "\x1b[1;33m»\x1b[0m Overwrite existing location? (\x1b[1m{}: {}\x1b[0m)\n\nPass the '-o' flag to overwrite",
                active_workspace.name,
                active_workspace.location.to_string_lossy(),
            );
            return Ok(());
        }

        if let Some(TomlValueKind::Table(key_values)) = toml.get_mut("workspaces") {
            key_values
                .iter_mut()
                .find(|w| w.key == active_workspace.name)
                .map(|w| {
                    if let TomlValueKind::String(value) = &mut w.kind {
                        *value = config.current_dir.to_string_lossy().to_string();
                    }
                    w
                });
        }

        println!(
            "\x1b[1;32m»\x1b[0m New location for workspace {}: ({})",
            active_workspace.name,
            config.current_dir.to_string_lossy()
        );

        return fs::write(config.config_path, toml.to_toml()).map_err(InitError::Error);
    }

    // workspace MUST have been passed
    let name = opts.workspace.unwrap();
    if let Some(TomlValueKind::Table(key_values)) = toml.get_mut("workspaces") {
        let workspace = key_values.iter_mut().find(|w| w.key == name);

        // check if workspace exists
        // if does NOT exist, prompt to CREATE new workspace
        // if does exist, "to overwrite htis..."

        // create workspace (it doesn't exist)
        if workspace.is_none() {
            println!(
                "\x1b[1;32m»\x1b[0m Created new workspace for storing cloups: {name}\n\nTo change to this workspace, use 'cloup workspace {name}'",
            );

            key_values.push(TomlValue {
                key: name.clone(),
                kind: TomlValueKind::String(config.current_dir.to_string_lossy().to_string()),
            });

            return fs::write(&config.config_path, toml.to_toml()).map_err(InitError::Error);
        };

        // overwrite existing workspace
        let workspace = workspace.unwrap();
        if !opts.overwrite {
            if let TomlValueKind::String(value) = &workspace.kind {
                println!(
                    "\x1b[1;33m»\x1b[0m Overwrite current location? (\x1b[1m{}: {}\x1b[0m)\n\nPass the '-o' flag to overwrite",
                    name,
                    value,
                );
            }

            return Ok(());
        } else if let TomlValueKind::String(value) = &mut workspace.kind {
            *value = config.current_dir.to_string_lossy().to_string();
        }
    } else {
        return Err(InitError::ConfigError(ConfigError::KeyMissing));
    }

    fs::write(&config.config_path, toml.to_toml()).map_err(InitError::Error)
}
