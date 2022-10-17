use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialise folder to store cloups in, only need to run once
    Init,
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
    // Interact with namespaces
    Namespace {
        /// Create namespace with name
        #[clap(short, long)]
        create: String,

        /// Delete namespace
        #[clap(short, long)]
        delete: String,

        /// Set namespace as default
        #[clap(short, long)]
        r#use: String,

        /// List all namespaces
        #[clap(short, long)]
        list: String,
    },
}

pub struct ApplyCommands {
    pub overwrite: bool,
}

pub struct CreateCommands {
    pub files: Vec<String>,
}

// pub struct NamespaceCommands {
//     pub namespace: Option<String>,
// }
