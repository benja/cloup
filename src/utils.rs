use clap::{Parser, Subcommand};
use colored::Colorize;
use fs_extra::dir::CopyOptions as DirCopyOptions;
use fs_extra::file::CopyOptions as FileCopyOptions;
use std::{
    env, fs,
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
    // _ignored_paths: Vec<String>,
}

#[derive(Debug)]
pub struct ApplyCommands {
    pub overwrite: bool,
}

impl Cloup {
    pub fn new() -> Cloup {
        let template_dir = match Self::template_dir() {
            Ok(path) => path,
            Err(_) => {
                eprint!(
                    "Please run {} before creating templates.",
                    "cloup init".bright_purple()
                );
                process::exit(1);
            }
        };

        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(_) => {
                eprint!("Current dir might not exist or have the required permissions to continue");
                process::exit(1);
            }
        };

        Cloup {
            current_dir,
            template_dir,
            // _ignored_paths: vec!["node_modules".to_string()],
        }
    }

    pub fn init(current_dir: PathBuf, _namespace: Option<String>) {
        let config_dirname = dirs::data_dir()
            .expect("Data directory not found, that's on you")
            .join("cloup");

        match fs::create_dir(&config_dirname) {
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {println!("Permissions were denied to create Cloup config directory")}
            },
            _ => (),
        }

        fs::write(
            config_dirname.join(".config"),
            format!("template_dir={:?}", current_dir),
        )
        .expect("Config could not be written");
    }
    fn handle_file_copy(current_dir: &PathBuf, template_dir: &PathBuf, file: &String) {
        let file_path = current_dir.join(&file);

        if !file_path.exists() {
            eprintln!(
                "{} {:?} {}",
                "The path,".bright_red(),
                file_path,
                ", does not exist".bright_red()
            );
            process::exit(1);
        }

        if file_path.is_dir() {
            fs_extra::dir::copy(
                file_path,
                &template_dir,
                &DirCopyOptions::new(),
            )
            .expect("Folder contents should have permission to move to new directory");
            return
        }

        println!("{:?}", &file_path);
        println!("{:?}", &template_dir.join(&file));

        // create sub folders for file to be allowed to move file into that folder
        template_dir
            .iter()
            .map(|p| fs::create_dir_all(p));

        let _ = fs_extra::file::copy(
            &file_path,
            template_dir.join(&file),
            &FileCopyOptions::from(FileCopyOptions {
                overwrite: true,
                ..Default::default()
            }),
        )
        .inspect_err(|e| {
            fs::remove_dir(&template_dir)
                .expect("Should be allowed to remove dir");
                
            eprintln!("{}", e);
            eprintln!("File should have permission to move to new directory");
            process::exit(1);
        });
        
    }
    pub fn create(&self, name: &str, files: Vec<String>) {
        let current_dir = &self.current_dir;
        let template_dir = self.template_dir.join(&name);

        if fs::create_dir(&template_dir).is_err() {
            eprintln!("Template {} already exists", &name.bright_purple());
            process::exit(1);
        }

        if !files.is_empty() {
            println!("{:#?}", files);

            for file in &files {
                Cloup::handle_file_copy(current_dir, &template_dir, file);
            }
        } else {
            // take entire current dir and slap in folder
            fs_extra::dir::copy(
                current_dir,
                template_dir,
                &DirCopyOptions::from(DirCopyOptions {
                    content_only: true,
                    ..DirCopyOptions::new()
                }),
            )
            .expect("Could not create template by whole folder");
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
        if fs_extra::dir::copy(
            &template_dir,
            &current_dir,
            &DirCopyOptions::from(DirCopyOptions {
                content_only: true,
                skip_exist: true,
                overwrite: options.overwrite,
                ..DirCopyOptions::new()
            }),
        ).is_err() {
            eprintln!("Some files could not be written, to overwrite add the {} flag to the same command", "-o".to_string().bright_purple())
        }

        println!(
            "🎨 Successfully applied cloup {}",
            &name.to_string().bright_purple(),
        );

        
    }

    pub fn delete(&self, name: &str) {
        let folder = &self.template_dir.join(&name);

        if folder.is_dir() {
            match fs::remove_dir_all(folder) {
                Ok(_) => println!("🎉 Successfully deleted template"),
                Err(e) => println!("Was not able to delete template for some reason {e}"),
            }
        } else {
            println!("Template {} does not exist", &name.bright_purple())
        }
    }

    pub fn list(&self) {
        let mut folders = fs::read_dir(&self.template_dir)
            .expect("Template folder should exist")
            .peekable();

        println!("Your available cloups:\n");

        if folders.peek().is_none() {
            eprintln!("There are no cloups");
            process::exit(1);
        }

        let mut cloups = vec![];

        for folder in folders {
            let name = folder.unwrap().file_name().to_string_lossy().to_string();

            let ignored_names = [".git", ".DS"];

            if ignored_names.iter().any(|n| name.contains(n)) {
                continue;
            }

            cloups.push(name);
        }

        if cloups.is_empty() {
            eprintln!(
                "You have no cloups, create one with {}",
                "cloup create <name>".bright_purple()
            );
            process::exit(1);
        }

        for cloup in cloups {
            println!("— {}", cloup.bright_purple());
        }

        println!(
            "\nYou can apply a cloup with with {}",
            "cloup apply <name>".bright_purple()
        );
    }

    fn template_dir() -> Result<PathBuf, String> {
        let config_dirname = dirs::data_dir().expect("Data dir not found").join("cloup");

        if fs::read_dir(&config_dirname).is_err() {
            return Err("Please run `cloup init` first in a template directory".to_string());
        }

        let mut content = fs::read_to_string(config_dirname.join(".config")).expect("Config could not be read.");
        

        if content.len() > 13 {
            content.replace_range(0..13, "");
            content = content.replace('"', "");
        }

        Ok(PathBuf::from(content))
    }
}
