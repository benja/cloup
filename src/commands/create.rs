use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::utils::{
    config::{get_config, ConfigError, Workspace},
    file::{copy_recursive, FileError},
};

#[derive(Debug)]
pub enum CreateError {
    DestinationNotFound,
    NameExists,
    Error(std::io::Error),
    ConfigError(ConfigError),
    FileError(FileError),
}

impl std::fmt::Display for CreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CreateError::DestinationNotFound => write!(f, "Destination not found"),
            CreateError::NameExists => write!(f, "Name already exists"),
            CreateError::Error(e) => write!(f, "Error: {}", e),
            CreateError::ConfigError(e) => write!(f, "Config error: {}", e),
            CreateError::FileError(e) => write!(f, "File error: {}", e),
        }
    }
}
impl std::error::Error for CreateError {}

#[derive(Debug)]
pub struct CreateOpts {
    // Name of cloup
    pub name: String,

    // Files to insert, if empty, recursively clone from current directory
    pub files: Vec<PathBuf>,

    // Exclude files (or directories) from being copied to cloup
    pub exclude: Vec<PathBuf>,

    // Workspace to create cloup in
    pub workspace: Option<String>,
}

pub fn run(opts: CreateOpts) -> Result<(), CreateError> {
    let config = get_config().map_err(CreateError::ConfigError)?;

    if let Some(workspace) = find_workspace(&opts, &config.data.workspaces) {
        let files: Vec<PathBuf> = opts
            .files
            .iter()
            .map(|f| config.current_dir.join(f))
            .collect();
        let exclude: Vec<PathBuf> = opts
            .exclude
            .iter()
            .map(|f| config.current_dir.join(f))
            .collect();

        let cloup_path = workspace.location.join(format!("cl_{}", &opts.name));
        if cloup_path.exists() {
            return Err(CreateError::NameExists);
        } else {
            fs::create_dir_all(&cloup_path).map_err(CreateError::Error)?;
        }

        if copy_files(&files, &exclude, &cloup_path).is_err() {
            fs::remove_dir_all(&cloup_path).map_err(CreateError::Error)?;
        }

        // if the created cloup's folder size is 0, remove it
        if cloup_path.read_dir().map_err(CreateError::Error)?.count() == 0 {
            fs::remove_dir_all(&cloup_path).map_err(CreateError::Error)?;
            println!("No files to copy, removing cloup");
            return Ok(());
        }

        println!(
            "\x1b[1;32m»\x1b[0m Created cloup '{}' in workspace '{}'",
            opts.name, workspace.name,
        );

        Ok(())
    } else {
        Err(CreateError::DestinationNotFound)
    }
}

fn copy_files(
    files: &[PathBuf],
    exclude: &[PathBuf],
    destination: &Path,
) -> Result<(), CreateError> {
    if files.is_empty() {
        copy_recursive(
            &std::env::current_dir().map_err(CreateError::Error)?,
            destination,
            exclude,
        )
        .map_err(CreateError::FileError)?;
        return Ok(());
    }

    for file in files {
        // if file is in exclude, skip
        if exclude.contains(file) {
            continue;
        }

        let destination = destination.join(
            file.file_name()
                .ok_or(CreateError::Error(std::io::ErrorKind::InvalidInput.into()))?,
        );

        println!("\x1b[1;32mCopying {:?} to {:?}\x1b[0m", file, &destination);

        if file.is_dir() {
            fs::create_dir_all(&destination).map_err(CreateError::Error)?;
            copy_recursive(file, &destination, exclude).map_err(CreateError::FileError)?;
        } else {
            fs::copy(file, &destination).map_err(CreateError::Error)?;
        }
    }

    Ok(())
}

fn find_workspace<'a>(opts: &'a CreateOpts, workspaces: &'a [Workspace]) -> Option<&'a Workspace> {
    if opts.workspace.is_none() {
        workspaces.iter().find(|w| w.active)
    } else {
        workspaces
            .iter()
            .find(|w| w.name == opts.workspace.clone().unwrap())
    }
}
