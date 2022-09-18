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
    Init {
        name: Option<String>,
    },
    Create {
        name: String,

        #[clap(short, long, multiple_values = true)]
        files: Vec<String>,
    },
    Apply {
        name: String,
    },
    List,
}
#[derive(Debug)]
pub struct Cloup {
    current_dir: PathBuf,
    template_dir: PathBuf,
    // _ignored_paths: Vec<String>,
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

    pub fn init(current_dir: PathBuf) {
        let config_dirname = dirs::data_dir()
            .expect("Data directory not found, that's on you")
            .join("cloup");

        match fs::create_dir(&config_dirname) {
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => {
                    println!("Permissions were denied to create Cloup config directory")
                }
                _ => (),
            },
            _ => (),
        }

        fs::write(
            config_dirname.join(".config"),
            format!("template_dir={:?}", current_dir),
        )
        .expect("Config could not be written");
    }

    pub fn create(&self, name: &str, files: Vec<String>) {
        let current_dir = &self.current_dir;
        let template_dir = self.template_dir.join(&name);

        match fs::create_dir(&template_dir) {
            Ok(_) => {
                if !files.is_empty() {
                    for file in files {
                        let file_path = current_dir.join(&file);

                        match file_path.exists() {
                            true => {
                                if file_path.is_dir() {
                                    fs_extra::dir::copy(
                                        file_path,
                                        &template_dir,
                                        &DirCopyOptions::new(),
                                    )
                                    .expect("Folder contents should have permission to move to new directory");
                                } else {
                                    fs_extra::file::copy(
                                        &file_path,
                                        template_dir.join(&file),
                                        &FileCopyOptions::new(),
                                    )
                                    .expect("File should have permission to move to new directory");
                                }
                            }
                            false => {
                                eprintln!(
                                    "{} {:?} {}, does not exist",
                                    "The path,".bright_red(),
                                    file_path,
                                    ", does not exist".bright_red()
                                );
                                process::exit(1);
                            }
                        }
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
                    .expect("Files should have permission to move to new directory");
                }

                println!(
                    "🚀 Successfully created cloup {} \n\n{}",
                    &name.to_string().bright_purple(),
                    format!("Apply this cloup with `cloup apply {}`", &name)
                );
            }
            Err(_) => {
                eprintln!("Folder could not be created, likely because it already exists.");
                process::exit(1);
            }
        }
    }

    pub fn apply(&self, name: &str) {
        let current_dir = &self.current_dir;
        let template_dir = self.template_dir.join(&name);

        match template_dir.is_dir() {
            true => {
                // copy all files from the template dir to current dir
                fs_extra::dir::copy(
                    &template_dir,
                    &current_dir,
                    &DirCopyOptions::from(DirCopyOptions {
                        content_only: true,
                        ..DirCopyOptions::new()
                    }),
                )
                .expect("Files should be able to be copied to current directory");

                println!(
                    "🎨 Successfully applied cloup {}",
                    &name.to_string().bright_purple(),
                );
            }
            false => {
                eprint!(
                    "A cloup with the name {} does not exist",
                    &name.to_string().bright_purple()
                );
                process::exit(1);
            }
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

        if let Err(_) = fs::read_dir(&config_dirname) {
            return Err("Please run `cloup init` first in a template directory".to_string());
        }

        let mut content = match fs::read_to_string(config_dirname.join(".config")) {
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
