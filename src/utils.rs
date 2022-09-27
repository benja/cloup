use clap::{Parser, Subcommand};
use colored::Colorize;
use fs_extra::dir::CopyOptions as DirCopyOptions;
use fs_extra::file::CopyOptions as FileCopyOptions;
use std::{
    env,
    fs::{self},
    io::ErrorKind,
    path::PathBuf,
    process::{self},
};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialise folder to store cloups in, only need to run once
    Init { name: Option<String> },
    /// Create a new cloup
    Create {
        /// Name of cloup
        name: String,

        /// If left empty, will create template recursively from current directory, otherwise you can specify files with the -f flag
        #[clap(short, long, multiple_values = true)]
        files: Vec<String>,
    },
    /// Apply cloup to folder
    Apply {
        /// Name of cloup
        name: String,

        /// Pass this flag to overwrite files that already exist from template
        #[clap(short)]
        overwrite: bool,
    },
    /// Delete a cloup
    Delete {
        /// Name of cloup
        name: String,
    },
    /// List all available cloups
    List,
}
#[derive(Debug)]
pub struct Cloup {
    current_dir: PathBuf,
    template_dir: PathBuf,
    // TODO: ignored_paths: Vec<String>,
}

#[derive(Debug)]
pub struct ApplyCommands {
    pub overwrite: bool,
}

impl Cloup {
    pub fn new() -> Cloup {
        let template_dir = Self::template_dir();
        let current_dir = env::current_dir();

        if template_dir.is_err() {
            eprint!(
                "Please run {} to initialise a template folder before creating cloups",
                "cloup init".bright_purple()
            );
            process::exit(1);
        }

        if current_dir.is_err() {
            eprint!(
                "Current directory might not exist or have the required permissions to continue"
            );
            process::exit(1);
        }

        Cloup {
            current_dir: current_dir.unwrap(),
            template_dir: template_dir.unwrap(),
        }
    }

    pub fn init(current_dir: PathBuf, _namespace: &Option<String>) {
        let config_dirname = dirs::data_dir()
            .expect("Data directory not found")
            .join("cloup");

        if let Err(e) = fs::create_dir(&config_dirname) {
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied when creating config directory")
                }
                _ => (),
            }
        }

        // TODO: Improve config file, use config-managable crate for this

        fs::write(
            config_dirname.join(".config"),
            format!("template_dir={:?}", current_dir),
        )
        .expect("An error occurred when writing config file");

        println!("📚 Successfully made this the template directory for cloups");
    }

    pub fn create(&self, name: &str, files: Vec<String>) {
        let current_dir = &self.current_dir;
        let template_dir = self.template_dir.join(&name);

        if name.starts_with('.') {
            eprintln!("Name of template should not start with a dot");
            process::exit(1);
        }

        if fs::create_dir(&template_dir).is_err() {
            eprintln!("Template {} already exists", &name.bright_purple());
            process::exit(1);
        }

        // If the files vec is empty, we know they want to use the entire folder as a template
        if files.is_empty() {
            fs_extra::dir::copy(
                current_dir,
                &template_dir,
                &DirCopyOptions::from(DirCopyOptions {
                    content_only: true,
                    ..DirCopyOptions::new()
                }),
            )
            .expect("Template could not be created based on folder");
        }

        for file in files {
            let file_path = current_dir.join(&file);

            // Check if referenced file does exist
            if !file_path.exists() {
                eprintln!(
                    "{} {:?} {}",
                    "The path,".bright_red(),
                    file_path,
                    ", does not exist".bright_red()
                );
                process::exit(1);
            }

            Cloup::copy_to_template(
                &file_path,
                &template_dir,
                (!file_path.is_dir()).then(|| &file),
            );
        }

        println!(
            "🚀 Successfully created cloup {} \n\n{}",
            &name.to_string().bright_purple(),
            format!("Apply this cloup with `cloup apply {}`", &name)
        );
    }

    pub fn apply(&self, name: &str, options: ApplyCommands) {
        let current_dir = &self.current_dir;
        let template_dir = self.template_dir.join(&name);

        if !template_dir.is_dir() {
            eprint!(
                "A cloup with the name {} does not exist",
                &name.to_string().bright_purple()
            );
            process::exit(1);
        }

        // copy all files from the template dir to current dir
        let _ = fs_extra::dir::copy(
            &template_dir,
            &current_dir,
            &DirCopyOptions::from(DirCopyOptions {
                content_only: true,
                skip_exist: true,
                overwrite: options.overwrite,
                ..DirCopyOptions::new()
            }),
        )
        .map_err(|_| {
            eprintln!(
                "Some files could not be written, to overwrite add the {} flag to the same command",
                "-o".to_string().bright_purple()
            );
            process::exit(1);
        });

        eprintln!(
            "To overwrite existing files, add the {} flag to the same command",
            "-o".to_string().bright_purple()
        );

        println!(
            "🎨 Successfully applied cloup {}",
            &name.to_string().bright_purple(),
        );
    }

    pub fn delete(&self, name: &str) {
        let folder = &self.template_dir.join(&name);

        if !folder.is_dir() {
            eprintln!("Template {} does not exist", &name.bright_purple());
            process::exit(1);
        }

        if !fs::remove_dir_all(folder).is_ok() {
            eprintln!("Was not able to delete template for some reason");
            process::exit(1);
        }

        println!("🚀 Successfully deleted cloup {}", &name.bright_purple());
    }

    pub fn list(&self) {
        let folders: Vec<_> = fs::read_dir(&self.template_dir)
        .expect("Template folder does not exist. Run `cloup init` in a folder to set it up as a template folder.")
        .filter(|f| f.as_ref().map(|f| !f.file_name().to_string_lossy().starts_with('.')).unwrap_or(false)).collect();

        if folders.is_empty() {
            eprintln!(
                "You have no cloups, create one with {}",
                "cloup create <name>".bright_purple()
            );
            return;
        } else {
            println!("Your available cloups:\n");
        }

        for folder in folders {
            println!(
                "— {}",
                folder
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .to_string()
                    .bright_purple()
            );
        }

        println!(
            "\nYou can apply a cloup with with {}",
            "cloup apply <name>".bright_purple()
        );
    }

    fn copy_to_template(file_path: &PathBuf, template_dir: &PathBuf, file: Option<&String>) {
        // If no file is specified, we are working with a folder
        if let None = file {
            fs_extra::dir::copy(file_path, &template_dir, &DirCopyOptions::new())
                .map_err(|e| {
                    eprintln!("{}", e);
                    process::exit(1);
                })
                .ok();

            return;
        }

        let template_path = template_dir.join(&file.expect("File should exist"));

        // create sub folders for file to be allowed to move file into that folder
        template_path
            .parent()
            .filter(|p| !p.is_dir())
            .map(|p| fs::create_dir_all(p));

        fs_extra::file::copy(
            &file_path,
            template_path,
            &FileCopyOptions::from(FileCopyOptions {
                overwrite: true,
                ..Default::default()
            }),
        )
        .map_err(|e| {
            fs::remove_dir(&template_dir).expect("Should be allowed to remove dir");
            eprintln!("{}", e);
            process::exit(1);
        })
        .ok();
    }

    fn template_dir() -> Result<PathBuf, String> {
        let config_location_dirname = dirs::data_dir()
            .expect("Data directory could not be found")
            .join("cloup");

        if fs::read_dir(&config_location_dirname).is_err() {
            return Err("Please run `cloup init` first in a template directory".to_string());
        }

        let mut content = match fs::read_to_string(config_location_dirname.join(".config")) {
            Ok(c) => c,
            Err(_) => return Err("Config could not be parsed".to_string()),
        };

        if content.len() > 13 {
            content.replace_range(0..13, "");
            content = content.replace('"', "");
        }

        Ok(PathBuf::from(content))
    }
}
