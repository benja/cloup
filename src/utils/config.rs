use std::{fs, path::PathBuf};

use crate::{
    dirs::{self},
    toml::data::{Toml, TomlValue, TomlValueKind},
};

const CONFIG_FILENAME: &str = "Cloup.toml";

#[derive(Debug)]
pub struct Config {
    /// Initial run?
    pub initial_run: bool,

    /// Version of config
    pub version: Option<String>,

    /// Current directory
    pub current_dir: PathBuf,

    /// Config location
    pub config_path: PathBuf,

    /// Data from Toml
    pub data: ConfigData,

    /// Toml in case it's needed
    pub toml: Toml,
}

#[derive(Debug)]
pub struct ConfigData {
    /// List of available workspaces
    pub workspaces: Vec<Workspace>,
}

impl Iterator for ConfigData {
    type Item = Workspace;

    fn next(&mut self) -> Option<Self::Item> {
        self.workspaces.pop()
    }
}

#[derive(Debug)]
pub struct Workspace {
    pub name: String,
    pub location: PathBuf,
    pub active: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    /// File and directory errors
    DirNotFound,
    FileNotFound,
    FileParseError,

    /// Config contents errors
    KeyMissing,
    KeyExists,
    UnexpectedValue,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::DirNotFound => write!(f, "Config directory not found"),
            ConfigError::FileNotFound => write!(
                f,
                "Config file not found. Run `cloup init` to create a config file."
            ),
            ConfigError::FileParseError => write!(f, "Config file parse error"),
            ConfigError::KeyMissing => write!(f, "Key missing in config file"),
            ConfigError::KeyExists => write!(f, "Key already exists in config file"),
            ConfigError::UnexpectedValue => write!(f, "Key present, but value was unexpected"),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        if error.kind() == std::io::ErrorKind::NotFound {
            ConfigError::FileNotFound
        } else {
            ConfigError::FileParseError
        }
    }
}

pub fn get_config() -> Result<Config, ConfigError> {
    let config_dir = dirs::config_dir()?.join("cloup");
    let config_path = config_dir.join(CONFIG_FILENAME);
    let current_dir = std::env::current_dir()?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
        return get_config();
    }

    if !config_path.exists() {
        // create default config
        let mut toml = Toml::new();
        toml.set(
            "version".to_string(),
            TomlValueKind::String("1.0".to_string()),
        );
        toml.set(
            "active_workspace".to_string(),
            TomlValueKind::String("default".to_string()),
        );
        toml.set(
            "workspaces".to_string(),
            TomlValueKind::Table(vec![TomlValue {
                key: "default".to_string(),
                kind: TomlValueKind::String(current_dir.to_string_lossy().to_string()),
            }]),
        );

        fs::write(&config_path, toml.to_toml())?;

        return Ok(Config {
            initial_run: true,
            version: Some("1".to_string()),
            current_dir: current_dir.clone(),
            config_path,
            data: ConfigData {
                workspaces: vec![Workspace {
                    name: "default".to_string(),
                    location: PathBuf::from(current_dir.to_string_lossy().to_string()),
                    active: true,
                }],
            },
            toml,
        });
    }

    let config_content = fs::read_to_string(&config_path)?;

    // Parse config file
    let toml = Toml::from(config_content);
    if toml.data.is_empty() {
        fs::remove_file(config_path)?;
        return get_config();
    }

    // Get contents of config file
    let version = toml.get("version");
    let active_workspace = toml
        .get("active_workspace")
        .ok_or(ConfigError::KeyMissing)?;
    let workspaces = toml.get("workspaces").ok_or(ConfigError::KeyMissing)?;

    let version = match version {
        Some(TomlValueKind::String(value)) => Some(value.clone()),
        _ => None,
    };

    let active_workspace = match active_workspace {
        TomlValueKind::String(value) => Ok(value.clone()),
        _ => Err(ConfigError::KeyMissing),
    }?;

    let workspaces = match workspaces {
        TomlValueKind::Table(values) => values
            .iter()
            .map(|v| {
                let result = match &v.kind {
                    TomlValueKind::String(location) => Ok(Workspace {
                        name: v.key.to_string(),
                        location: PathBuf::from(location),
                        active: active_workspace == v.key,
                    }),
                    _ => Err(ConfigError::UnexpectedValue),
                };
                result.map_err(ConfigError::from)
            })
            .collect(),
        _ => Err(ConfigError::UnexpectedValue),
    }?;

    Ok(Config {
        initial_run: false,
        version,
        current_dir,
        config_path,
        data: ConfigData { workspaces },
        toml,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let config = get_config().unwrap();
        println!("{:#?}", config);
    }
}
