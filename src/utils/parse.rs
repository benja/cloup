use std::path::PathBuf;

use crate::commands::{
    apply::ApplyOpts, create::CreateOpts, init::InitOpts, workspace::WorkspaceOpts,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    let version = get_flag_params(["-V", "--version"], &argv);

    if version.is_some() {
        println!("cloup {}", VERSION);
        std::process::exit(0);
    }

    match command {
        "init" => {
            let usage: UsageError = UsageError {
                message: "Set cloup storage location".to_string(),
                usage: "$ cloup init [flags]".to_string(),
                examples: vec![
                    "cloup init".to_string(),
                    "cloup init -w my-workspace".to_string(),
                ],
                flags: vec![
                    (
                        "-w, --workspace <name>".to_string(),
                        "New workspace location for cloups".to_string(),
                    ),
                    (
                        "-o, --overwrite".to_string(),
                        "Overwrite cloup storage location".to_string(),
                    ),
                ],
            };

            if help.is_some() {
                return Err(CommandError::BadUsage(usage));
            }

            let overwrite = get_flag_params(["-o", "--overwrite"], &argv).is_some();
            let workspace = if let Some(params) = get_flag_params(["-w", "--workspace"], &argv) {
                if params.len() > 1 || params.is_empty() {
                    return Err(CommandError::BadUsage(usage));
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
        "create" => {
            let usage = UsageError {
                message: "Create a new cloup".to_string(),
                usage: "$ cloup create <name> [flags]".to_string(),
                examples: vec![
                    "cloup create my-cloup".to_string(),
                    "cloup create my-cloup -w my-workspace".to_string(),
                    "cloup create my-cloup -f file1 file2 -e file3 file4".to_string(),
                ],
                flags: vec![
                    (
                        "-w, --workspace <name>".to_string(),
                        "Store cloup in a specific workspace".to_string(),
                    ),
                    (
                        "-f, --files <file1> <file2>".to_string(),
                        "Files to include in cloup".to_string(),
                    ),
                    (
                        "-e, --exclude <file1> <file2>".to_string(),
                        "Files to exclude from cloup".to_string(),
                    ),
                ],
            };

            if help.is_some() {
                return Err(CommandError::BadUsage(usage));
            }

            if argv.len() < 2 {
                return Err(CommandError::BadUsage(usage));
            }

            let workspace = if let Some(params) = get_flag_params(["-w", "--workspace"], &argv) {
                if params.len() > 1 || params.is_empty() {
                    return Err(CommandError::BadUsage(usage));
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

            Ok(Command::Create(CreateOpts {
                name,
                files,
                exclude,
                workspace,
            }))
        }
        "apply" => {
            let usage = UsageError {
                message: "Apply cloup to current directory".to_string(),
                usage: "$ cloup apply <name> [flags]".to_string(),
                examples: vec![
                    "cloup apply my-cloup".to_string(),
                    "cloup apply my-cloup -w my-workspace".to_string(),
                ],
                flags: vec![(
                    "-w, --workspace <name>".to_string(),
                    "Apply cloup from a specific workspace".to_string(),
                )],
            };

            if help.is_some() {
                return Err(CommandError::BadUsage(usage));
            }

            if argv.len() < 2 {
                return Err(CommandError::BadUsage(usage));
            }

            let workspace = if let Some(params) = get_flag_params(["-w", "--workspace"], &argv) {
                if params.len() > 1 || params.is_empty() {
                    return Err(CommandError::BadUsage(usage));
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
            let usage = UsageError {
                message: "List all cloups in the current workspace".to_string(),
                usage: "$ cloup list [flags]".to_string(),
                examples: vec!["cloup list".to_string()],
                flags: vec![(
                    "-h, --help".to_string(),
                    "Print help information".to_string(),
                )],
            };

            if help.is_some() {
                return Err(CommandError::BadUsage(usage));
            }

            Ok(Command::List())
        }
        "workspace" | "w" => {
            let usage = UsageError {
                message: "Set the current workspace or list all workspaces".to_string(),
                usage: "cloup workspace [flags]".to_string(),
                examples: vec![
                    "cloup workspace".to_string(),
                    "cloup workspace -l".to_string(),
                    "cloup workspace -c my-workspace".to_string(),
                ],
                flags: vec![
                    (
                        "-l, --list".to_string(),
                        "List all workspaces in the current directory".to_string(),
                    ),
                    (
                        "-c, --create <name>".to_string(),
                        "Create a new workspace in current directory".to_string(),
                    ),
                ],
            };

            if help.is_some() {
                return Err(CommandError::BadUsage(usage));
            }

            if argv.len() < 2 {
                return Err(CommandError::BadUsage(usage));
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
    flags: Vec<(String, String)>,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CommandError::NotRecognized | CommandError::NoArgs => {
                write!(f, "{}", no_args())
            }
            CommandError::BadUsage(data) => {
                let max_len = data.flags.iter().map(|(a, _)| a.len()).max().unwrap_or(0) + 2;

                write!(
                    f,
                    "{}\n\n\x1b[1mUSAGE\x1b[0m\n    {}\n\n\x1b[1mEXAMPLES\x1b[0m\n    {}\n\n\x1b[1mFLAGS\x1b[0m\n    {}\n",
                    data.message,
                    data.usage,
                    data.examples.join("\n    "),
                    data.flags
                        .iter()
                        .map(|(a, b)| format!("{:<width$} {}", a, b, width = max_len))
                        .collect::<Vec<_>>()
                        .join("\n    ")
                )
            }
        }
    }
}
impl std::error::Error for CommandError {}

fn no_args() -> String {
    format!(
        "Local template manager

\x1b[1mVERSION\x1b[0m
    {VERSION}

\x1b[1mUSAGE\x1b[0m
    $ cloup [command] [flags]

\x1b[1mOPTIONS\x1b[0m
    -h, --help      Print help information
    -V, --version   Print version information
    
\x1b[1mCOMMANDS\x1b[0m
    init            Sets the current directory as a location for cloups
    create          Create a new cloup
    apply           Apply a cloup to the current directory
    list            List all cloups in the current workspace
    workspace       Sets the current workspace or list all workspaces
"
    )
}
