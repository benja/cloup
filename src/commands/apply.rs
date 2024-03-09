use crate::utils::{
    config::{get_config, ConfigError, Workspace},
    file::{self, FileError},
};

#[derive(Debug)]
pub enum ApplyError {
    NotFound,
    ConfigError(ConfigError),
    FileError(FileError),
}

impl std::fmt::Display for ApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApplyError::NotFound => write!(f, "Workspace not found"),
            ApplyError::ConfigError(e) => write!(f, "Config error: {}", e),
            ApplyError::FileError(e) => write!(f, "File error: {}", e),
        }
    }
}
impl std::error::Error for ApplyError {}

#[derive(Debug)]
pub struct ApplyOpts {
    // Name of cloup
    pub name: String,

    // Workspace to create cloup in
    pub workspace: Option<String>,
}

pub fn run(opts: ApplyOpts) -> Result<(), ApplyError> {
    let config = get_config().map_err(ApplyError::ConfigError)?;

    if let Some(workspace) = find_workspace(&opts, &config.data.workspaces) {
        let cloup_path = workspace.location.join(format!("cl_{}", &opts.name));

        // we want to take all files in the cloup path and copy them to the current directory
        if cloup_path.exists() {
            println!(
                "\x1b[1;32m»\x1b[0m Applied cloup \x1b[1m{}\x1b[0m to \x1b[1m{}\x1b[0m",
                &opts.name,
                config.current_dir.to_string_lossy()
            );

            // there has to be some mechanism in case some files already exist and it overwrites, essentially you have to be asked yes or no whether you want to overwrite each file. So we prompt the user for each file that already exists and ask if they want to overwrite it.

            return file::copy_recursive(&cloup_path, &config.current_dir, &[])
                .map_err(ApplyError::FileError);
        }

        println!(
            "\x1b[1;33m»\x1b[0m Cloup \x1b[1m{}\x1b[0m does not exist in workspace '{}'",
            &opts.name, workspace.name
        );

        Ok(())
    } else {
        Err(ApplyError::NotFound)
    }
}

fn find_workspace<'a>(opts: &'a ApplyOpts, workspaces: &'a [Workspace]) -> Option<&'a Workspace> {
    if opts.workspace.is_none() {
        workspaces.iter().find(|w| w.active)
    } else {
        workspaces
            .iter()
            .find(|w| w.name == opts.workspace.clone().unwrap())
    }
}
