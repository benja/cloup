use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialise folder to store cloups in, only need to run once
    Init {
        /// Namespace to store this under
        #[clap(short, long)]
        namespace: Option<String>,
    },
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
    // /// See namespaces
    //     Namespaces {
    //         /// Set a namespace as default
    //         name: Option<String>,
    //     },
}

pub struct ApplyCommands {
    pub overwrite: bool,
}

pub struct CreateCommands {
    pub files: Vec<String>,
}

pub struct InitCommands {
    pub namespace: Option<String>,
}
