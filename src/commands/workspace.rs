use std::fs;

use crate::{
    toml::data::{TomlValue, TomlValueKind},
    utils::config::{get_config, ConfigError},
};

#[derive(Debug)]
pub enum WorkspaceError {
    NotFound,
    Error(std::io::Error),
    ConfigError(ConfigError),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WorkspaceError::NotFound => {
                write!(f, "Workspace not found. Use the '-c' flag to create it")
            }
            WorkspaceError::Error(e) => write!(f, "Error: {}", e),
            WorkspaceError::ConfigError(e) => write!(f, "Config error: {}", e),
        }
    }
}
impl std::error::Error for WorkspaceError {}

#[derive(Debug)]
pub struct WorkspaceOpts {
    // List all workspaces (return early)
    pub list: bool,

    // Create a new workspace
    pub create: bool,

    // Name of workspace to set as active
    pub name: Option<String>,
}

pub fn run(opts: WorkspaceOpts) -> Result<(), WorkspaceError> {
    let config = get_config().map_err(WorkspaceError::ConfigError)?;
    let mut toml = config.toml;

    if opts.list {
        config.data.workspaces.iter().for_each(|w| {
            println!(
                "{}{}: {}",
                if w.active { "\x1b[1;33m»\x1b[0m " } else { "" },
                w.name,
                w.location.to_string_lossy(),
            );
        });
        return Ok(());
    }

    if let Some(name) = opts.name {
        if !config.data.workspaces.iter().any(|w| w.name == name) {
            // if not found, we _can_ create it
            if opts.create {
                if let Some(TomlValueKind::Table(key_values)) = toml.get_mut("workspaces") {
                    key_values.push(TomlValue {
                        key: name.clone(),
                        kind: TomlValueKind::String(
                            config.current_dir.to_string_lossy().to_string(),
                        ),
                    });
                }

                fs::write(config.config_path, toml.to_toml()).map_err(WorkspaceError::Error)?;
                println!(
                    "\x1b[1;33m»\x1b[0m Workspace \x1b[1;33m{}\x1b[0m created",
                    name
                );
                return Ok(());
            }

            return Err(WorkspaceError::NotFound);
        }

        if opts.create {
            println!(
                "\x1b[1;33m»\x1b[0m Workspace \x1b[1;33m{}\x1b[0m already exists",
                name
            );
            return Ok(());
        }

        if let Some(TomlValueKind::String(value)) = toml.get_mut("active_workspace") {
            *value = name.clone();
        }

        fs::write(config.config_path, toml.to_toml()).map_err(WorkspaceError::Error)?;
        println!(
            "\x1b[1;33m»\x1b[0m The active workspace has been set to \x1b[1;33m{}\x1b[0m",
            name
        );
        return Ok(());
    }

    Ok(())
}
