use crate::utils::config::{get_config, ConfigError};

#[derive(Debug)]
pub enum ListError {
    ConfigError(ConfigError),
}

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
            if entry.file_name() == ".DS_Store" {
                None
            } else {
                Some(entry.file_name())
            }
        })
        .collect::<Vec<_>>();

    if cloups.is_empty() {
        println!("\x1b[1;33m»\x1b[0m No cloups in workspace.")
    } else {
        println!(
            "\x1b[1;32m»\x1b[0m Cloups in workspace {:?}:",
            workspace.name
        );
        for cloup in cloups {
            println!("\x1b[1m{}\x1b[0m", cloup.to_string_lossy());
        }
    }

    Ok(())
}
