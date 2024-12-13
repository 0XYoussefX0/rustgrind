use clap::{Parser, Subcommand};
use colored::{Color, Colorize};
use std::{io, path::Path, process};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::Init => {
                let dir_exists: bool = Path::new("./rustgrind").is_dir();

                if dir_exists {
                    eprintln!(
                        "{}",
                        r#"Error: Initialization failed

Caused by:
    A directory with the name `rustgrind` already exists in the current directory.
    You probably already initialized Rustgrind.
    Run `cd rustgrind`
    Then run `rustgrind` again"#
                    );
                    process::exit(1);
                }

                println!("This command will create the directory `rustgrind/` which will contain the exercises.");
                println!("Press ENTER to continue ");

                let mut user_input = String::new();

                loop {
                    io::stdin()
                        .read_line(&mut user_input)
                        .expect("Failed to read input");

                    if user_input.trim().is_empty() {
                        break;
                    }
                }

                clone_repo();

                let my_green = Color::TrueColor {
                    r: 34,
                    g: 203,
                    b: 135,
                };

                println!("{}", "Initialization done ✓\n".color(my_green));
                println!(
                    "{}",
                    "Run `cd rustgrind` to go into the generated directory.".bold()
                );
                println!("{}", "Then run `rustgrind` to get started".bold());
            }
            _ => {}
        },
        None => {
            let dir_exists: bool = Path::new("./problems").is_dir();

            if !dir_exists {
                eprintln!(
                    "{}",
                    r#"
The `problems/` directory couldn't be found in the current directory.
If you are just starting with Rustgrind, run the command `rustgrind init` to initialize it.
                "#
                );
                process::exit(1);
            }

            println!(
                "{}",
                r#"
       Welcome to...  
                _             _           _ 
 _ __ _   _ ___| |_ __ _ _ __(_)_ __   __| |
| '__| | | / __| __/ _` | '__| | '_ \ / _` |
| |  | |_| \__ \ || (_| | |  | | | | | (_| |
|_|   \__,_|___/\__\__, |_|  |_|_| |_|\__,_|
                   |___/                    
                   "#
            );
        }
    }
}
