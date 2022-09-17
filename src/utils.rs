use clap::{Parser, Subcommand};
use fs_extra::dir::CopyOptions;
use std::{env, fs, io::ErrorKind, path::PathBuf};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Create { name: String },
}

#[derive(Debug)]
pub struct Cloup {
    template_dir: PathBuf,
}

impl Cloup {
    pub fn new() -> Cloup {
        let template_dir = match Self::template_dir() {
            Ok(path) => path,
            Err(_) => {
                Self::init(env::current_dir().unwrap());
                match Self::template_dir() {
                    Ok(path) => path,
                    Err(_) => panic!("shit gone south"),
                }
            }
        };

        Cloup { template_dir }
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

    pub fn create(&self, name: &str) {
        fs::create_dir(self.template_dir.join(&name)).expect("Could not write dir");

        let options = CopyOptions::from(CopyOptions {
            content_only: true,
            ..CopyOptions::new()
        });

        fs_extra::dir::copy(
            env::current_dir().expect("Should be in a directory"),
            self.template_dir.join(&name),
            &options,
        )
        .expect("shit passed");

        println!("Congrats it worked");
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
