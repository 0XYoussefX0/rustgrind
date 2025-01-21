use anyhow::{bail, Context, Result};
use app_state::StateFileStatus;
use clap::{Parser, Subcommand};
use std::{
    io::{self, IsTerminal, Write},
    path::Path,
    process::ExitCode,
};

use crate::{
    app_state::AppState,
    info_file::InfoFile,
    term::{clear_terminal, press_enter_prompt},
    watch,
};

#[derive(Parser)]
#[command(
    version,
    about = "Rustgrind is a collection of curated coding challenges based on the NeetCode 150, designed to help you practice problem-solving and algorithmic thinking while mastering Rust."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the official Rustgrind problems
    Init,

    /// Run a single problem. Runs the next pending problem if the problem name is not specified
    Run {
        #[arg(help = "The name of the problem")]
        name: Option<String>,
    },

    /// Reset a single problem
    Reset {
        #[arg(help = "The name of the problem")]
        name: String,
    },

    /// Show a hint. Shows the hint of the next pending problem if the problem name is not specified
    Hint {
        #[arg(help = "The name of the problem")]
        name: Option<String>,
    },
}

mod app_state;
mod cargo_toml;
mod cmd;
mod embedded;
mod info_file;
mod init;
mod list;
mod problem;
mod term;
mod watch;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    if let Some(Commands::Init) = cli.command {
        init::init().context("Initialization failed")?;
        return Ok(ExitCode::SUCCESS);
    }

    if !Path::new("problems").is_dir() {
        eprintln!("{PRE_INIT_MSG}");
        return Ok(ExitCode::FAILURE);
    }

    let info_file = InfoFile::parse()?;

    let (mut app_state, state_file_status) =
        AppState::new(info_file.problems, info_file.final_message)?;

    // Show the welcome message if the state file doesn't exist yet.
    match state_file_status {
        StateFileStatus::NotRead => {
            let mut stdout = io::stdout().lock();
            clear_terminal(&mut stdout)?;

            let welcome_message = info_file.welcome_message.trim_ascii();
            write!(stdout, "{welcome_message}\n\nPress ENTER to continue ")?;
            press_enter_prompt(&mut stdout)?;
            clear_terminal(&mut stdout)?;
            // Flush to be able to show errors occurring before printing a newline to stdout.
            stdout.flush()?;
        }
        StateFileStatus::Read => (),
    }

    match cli.command {
        None => {
            if !io::stdout().is_terminal() {
                bail!("Unsupported or missing terminal/TTY");
            }

            let notify_problem_names = &*app_state
                .problems()
                .iter()
                .map(|problem| problem.name.as_bytes())
                .collect::<Vec<_>>()
                .leak();

            watch::watch(&mut app_state, notify_problem_names)?;
        }
        Some(command) => match command {
            Commands::Run { name } => {
                if let Some(name) = name {
                    app_state.set_current_exercise_by_name(&name)?;
                }
                return run::run(&mut app_state);
            }
            Commands::Reset { name } => {
                app_state.set_current_exercise_by_name(&name)?;
                let exercise_path = app_state.reset_current_exercise()?;
                println!("The exercise {exercise_path} has been reset");
            }
            Commands::Hint { name } => {
                if let Some(name) = name {
                    app_state.set_current_exercise_by_name(&name)?;
                }
                println!("{}", app_state.current_exercise().hint);
            }
            Commands::Init => unreachable!(),
        },
    }

    Ok(ExitCode::SUCCESS)
}

const PRE_INIT_MSG: &str = r#"
       Welcome to...
                _             _           _ 
 _ __ _   _ ___| |_ __ _ _ __(_)_ __   __| |
| '__| | | / __| __/ _` | '__| | '_ \ / _` |
| |  | |_| \__ \ || (_| | |  | | | | | (_| |
|_|   \__,_|___/\__\__, |_|  |_|_| |_|\__,_|
                   |___/                    

The `problems/` directory couldn't be found in the current directory.
If you are just starting with Rustgrind, run the command `rustgrind init` to initialize it.
"#;
