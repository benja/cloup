use std::{fs, ops::Not, path::Path};

use crate::utils::config::{get_config, ConfigError};

#[derive(Debug)]
pub enum ListError {
    ConfigError(ConfigError),
}

impl std::fmt::Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ListError::ConfigError(e) => write!(f, "Config error: {}", e),
        }
    }
}
impl std::error::Error for ListError {}

pub fn run() -> Result<(), ListError> {
    let config = get_config().map_err(ListError::ConfigError)?;

    let workspace = config
        .data
        .workspaces
        .iter()
        .find(|w| w.active)
        .expect("Must have an active workspace");

    let cloups = std::fs::read_dir(&workspace.location)
        .expect("Could not read directory")
        .filter_map(|entry| {
            let entry = entry.expect("Could not read entry");

            if entry.file_name() == ".DS_Store"
                || entry.file_name().to_string_lossy().starts_with("cl_").not()
            {
                None
            } else {
                Some(entry.file_name())
            }
        })
        .collect::<Vec<_>>();

    if cloups.is_empty() {
        println!(
            "\x1b[1;33m»\x1b[0m No cloups in workspace '{}'",
            workspace.name
        )
    } else {
        println!(
            "\x1b[1;32m»\x1b[0m Cloups in workspace '{}':",
            workspace.name
        );

        for (i, cloup) in cloups.iter().enumerate() {
            let cloup_path = workspace.location.join(cloup);
            let cloup_name = cloup.to_string_lossy().replace("cl_", "");
            let size_in_mb = calculate_size(&cloup_path) as f64 / 1_000_000.0;

            let size_str = if size_in_mb > 1000.0 {
                format!("{:.2} GB", size_in_mb / 1000.0)
            } else {
                format!("{:.2} MB", size_in_mb)
            };

            let prefix = if i == cloups.len() - 1 {
                "    └── "
            } else {
                "    ├── "
            };

            println!("{}\x1b[1m{}\x1b[0m ({})", prefix, cloup_name, size_str);
        }
    }

    Ok(())
}

fn calculate_size(path: &Path) -> u64 {
    let mut total_size = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                total_size += calculate_size(&entry_path);
            } else if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
            }
        }
    }

    total_size
}
