use std::path::PathBuf;

use crate::commands::{
    apply::ApplyOpts, create::CreateOpts, init::InitOpts, workspace::WorkspaceOpts,
};

const NO_ARGS_MESSAGE: &str = "» Usage: cloup [command] [flags]\n
Commands:
    \x1b[1;32minit [flags]\x1b[0m
        Sets the current directory as a location for cloups

        \x1b[3m-w, --workspace <name>\x1b[0m
            New workspace location for cloups

l        \x1b[3m-o, --overwrite\x1b[0m
            Overwrite location for workspace

        \x1b[3m-h, --help\x1b[0m
            For more information about the command\n

    \x1b[1;32mcreate <name> [flags]\x1b[0m
        Create a new cloup

        \x1b[3m-f, --files [...]\x1b[0m
            Only include the specified files

        \x1b[3m-e, --exclude [...]\x1b[0m
            Exclude specified files

        \x1b[3m-w, --workspace <name>\x1b[0m
            Specify a workspace to create the cloup, example: -w my-workspace

        \x1b[3m-h, --help\x1b[0m
            For more information about the command\n

    \x1b[1;32mapply <name> [flags]\x1b[0m
        Apply a cloup to the current directory

        \x1b[3m-w, --workspace <name>\x1b[0m
            Specify a workspace to apply the cloup, example: -w my-workspace

        \x1b[3m-h, --help\x1b[0m
            For more information about the command\n
            
    \x1b[1;32mworkspace <name?> [flags]\x1b[0m
        Sets the current workspace or list all workspaces

        \x1b[3m-l, --list\x1b[0m
            List all workspaces in the workspace

        \x1b[3m-c, --create\x1b[0m
            Create a new workspace in the current directory
        
        \x1b[3m-h, --help\x1b[0m
            For more information about the command\n
";

fn get_flag_params(flags: [&str; 2], args: &[String]) -> Option<Vec<PathBuf>> {
    let index = flags
        .iter()
        .find_map(|&flag| args.iter().position(|e| e == flag));

    if let Some(pos) = index {
        let mut params = args[pos + 1..].to_vec();
        let other_flag = params.iter().position(|e| e.starts_with('-'));

        if let Some(idx) = other_flag {
            params.truncate(idx);
        }

        Some(params.iter().map(PathBuf::from).collect())
    } else {
        None
    }
}

pub fn command_parser(argv: Vec<String>) -> Result<Command, CommandError> {
    if argv.is_empty() {
        return Err(CommandError::NoArgs);
    }

    let command = argv[0].as_str();
    let help = get_flag_params(["-h", "--help"], &argv);

    match command {
        "init" => {
            if help.is_some() {
                return Err(CommandError::BadUsage(UsageError {
                    message: "Init command help".to_string(),
                    usage: "cloup init [flags]".to_string(),
                    examples: vec!["cloup init -w my-workspace".to_string()],
                    flags: vec![
                        "-w, --workspace <name>".to_string(),
                        "-o, --overwrite".to_string(),
                    ],
                }));
            }

            let overwrite = get_flag_params(["-o", "--overwrite"], &argv).is_some();
            let workspace = if let Some(params) = get_flag_params(["-w", "--workspace"], &argv) {
                if params.len() > 1 || params.is_empty() {
                    return Err(CommandError::BadUsage(UsageError {
                        message: "Invalid workspace name".to_string(),
                        usage: "cloup init [flags]".to_string(),
                        examples: vec!["cloup init -w my-workspace".to_string()],
                        flags: vec![
                            "-w, --workspace <name>".to_string(),
                            "-o, --overwrite".to_string(),
                        ],
                    }));
                }
                Some(params[0].to_string_lossy().to_string())
            } else {
                None
            };

            Ok(Command::Init(InitOpts {
                overwrite,
                workspace,
            }))
        }
        "create" | "c" => {
            if help.is_some() {
                return Err(CommandError::BadUsage(UsageError {
                    message: "Create command help".to_string(),
                    usage: "cloup create <name> [flags]".to_string(),
                    examples: vec![
                        "cloup create my-cloup -w my-workspace".to_string(),
                        "cloup create my-cloup".to_string(),
                    ],
                    flags: vec![
                        "-f, --files [...files]".to_string(),
                        "-e, --exclude [...files]".to_string(),
                        "-w, --workspace <name>".to_string(),
                    ],
                }));
            }

            if argv.len() < 2 {
                return Err(CommandError::BadUsage(UsageError {
                    message: "Missing cloup name".to_string(),
                    usage: "cloup create <name> [flags]".to_string(),
                    examples: vec!["cloup create my-cloup".to_string()],
                    flags: vec!["-f, --files (optional)".to_string()],
                }));
            }

            let workspace = if let Some(params) = get_flag_params(["-w", "--workspace"], &argv) {
                if params.len() > 1 || params.is_empty() {
                    return Err(CommandError::BadUsage(UsageError {
                        message: "Invalid workspace name".to_string(),
                        usage: "cloup create <name> [flags]".to_string(),
                        examples: vec!["cloup create my-cloup -w my-workspace".to_string()],
                        flags: vec![
                            "-w, --workspace <name>".to_string(),
                            "-f, --files (optional)".to_string(),
                        ],
                    }));
                }
                Some(params[0].to_string_lossy().to_string())
            } else {
                None
            };

            let name = argv[1].to_string();
            let mut files = vec![];

            let files_params = get_flag_params(["-f", "--files"], &argv);
            if let Some(f) = files_params {
                files = f;
            }

            let mut exclude = vec![];
            let exclude_params = get_flag_params(["-e", "--exclude"], &argv);
            if let Some(e) = exclude_params {
                exclude = e;
            }

            // print files
            println!("» Files: {:?}", files);
            println!("» Exclude: {:?}", exclude);

            Ok(Command::Create(CreateOpts {
                name,
                files,
                exclude,
                workspace,
            }))
        }
        "apply" => {
            if help.is_some() {
                return Err(CommandError::BadUsage(UsageError {
                    message: "Apply command help".to_string(),
                    usage: "cloup apply <name> [flags]".to_string(),
                    examples: vec![
                        "cloup apply my-cloup -w my-workspace".to_string(),
                        "cloup apply my-cloup".to_string(),
                    ],
                    flags: vec!["-w, --workspace <name>".to_string()],
                }));
            }

            if argv.len() < 2 {
                return Err(CommandError::BadUsage(UsageError {
                    message: "Missing cloup name".to_string(),
                    usage: "cloup apply <name> [flags]".to_string(),
                    examples: vec!["cloup apply my-cloup".to_string()],
                    flags: vec!["-w, --workspace <name>".to_string()],
                }));
            }

            let workspace = if let Some(params) = get_flag_params(["-w", "--workspace"], &argv) {
                if params.len() > 1 || params.is_empty() {
                    return Err(CommandError::BadUsage(UsageError {
                        message: "Invalid workspace name".to_string(),
                        usage: "cloup apply <name> [flags]".to_string(),
                        examples: vec!["cloup apply my-cloup -w my-workspace".to_string()],
                        flags: vec!["-w, --workspace <name>".to_string()],
                    }));
                }
                Some(params[0].to_string_lossy().to_string())
            } else {
                None
            };

            Ok(Command::Apply(ApplyOpts {
                name: argv[1].to_string(),
                workspace,
            }))
        }
        "list" => {
            if help.is_some() {
                return Err(CommandError::BadUsage(UsageError {
                    message: "List command help".to_string(),
                    usage: "cloup list".to_string(),
                    examples: vec!["cloup list".to_string()],
                    flags: vec![],
                }));
            }

            Ok(Command::List())
        }
        "workspace" | "w" => {
            if help.is_some() {
                return Err(CommandError::BadUsage(UsageError {
                    message: "Workspace command help".to_string(),
                    usage: "cloup workspace <name?> [flags]".to_string(),
                    examples: vec![
                        "cloup workspace work".to_string(),
                        "cloup workspace -l".to_string(),
                    ],
                    flags: vec!["-l, --list".to_string()],
                }));
            }

            if argv.len() < 2 {
                return Err(CommandError::BadUsage(UsageError {
                    message: "Missing workspace name".to_string(),
                    usage: "cloup workspace <name?> [flags]".to_string(),
                    examples: vec!["cloup workspace work".to_string()],
                    flags: vec!["-l, --list".to_string()],
                }));
            }

            let name = Some(argv[1].to_string());
            let list = get_flag_params(["-l", "--list"], &argv);
            let create = get_flag_params(["-c", "--create"], &argv);

            Ok(Command::Workspace(WorkspaceOpts {
                list: list.is_some(),
                create: create.is_some(),
                name,
            }))
        }
        _ => Err(CommandError::NotRecognized),
    }
}

#[derive(Debug)]
pub enum Command {
    Init(InitOpts),
    Create(CreateOpts),
    Workspace(WorkspaceOpts),
    Apply(ApplyOpts),
    List(),
}

#[derive(Debug)]
pub enum CommandError {
    NotRecognized,
    NoArgs,
    BadUsage(UsageError),
}

#[derive(Debug)]
pub struct UsageError {
    message: String,
    usage: String,
    examples: Vec<String>,
    flags: Vec<String>,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CommandError::NotRecognized | CommandError::NoArgs => write!(f, "{NO_ARGS_MESSAGE}"),
            CommandError::BadUsage(data) => {
                let examples_str = data.examples.join("\n\t");
                let flags_str = data.flags.join("\n\t");

                write!(
                    f,
                    "» \x1b[1;37mUsage: {}\x1b[0m\n\nExamples:\n\t{}\n\nFlags:\n\t{}",
                    data.usage, examples_str, flags_str
                )
            }
        }
    }
}
