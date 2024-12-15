use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::ExitCode;

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

    /// Check all the problems, marking them as done or pending accordingly
    CheckAll,

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

mod embedded;
mod info_file;
mod init;
mod term;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::Init => {
                init::init().context("Initialization failed")?;
                return Ok(ExitCode::SUCCESS);
            }
            _ => todo!(),
        },
        None => todo!(),
    }
}
