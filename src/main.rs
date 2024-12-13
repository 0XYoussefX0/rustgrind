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

                let repo_url = "https://github.com/0XYoussefX0/rustgrind";
                let target_dir = "./rustgrind";
                let result = process::Command::new("git")
                    .arg("clone")
                    .arg(repo_url)
                    .arg(target_dir)
                    .output();

                match result {
                    Ok(output) => {
                        if !output.status.success() {
                            let err_message = String::from_utf8_lossy(&output.stderr);
                            eprintln!(
                                "Error: Initialization failed\n\n Error Message: {}",
                                err_message
                            );
                            process::exit(1)
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: Initialization failed\n\n Error Message: {}", err);
                        process::exit(1)
                    }
                }

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
Is this your first time? No worries, Rustgrind is designed to help you improve at solving LeetCode-style problems while practicing Rust! Here's how it works:

    1. Rustgrind is centered around the NeetCode 150 problems, carefully curated to strengthen your problem-solving and algorithmic thinking skills, all in Rust.

    2. Keep your editor open in the rustgrind/ directory. Rustgrind will display the path of the current problem. Open the corresponding file, implement your solution, and save the file. Rustgrind will automatically detect changes, run the tests, and provide feedback. If all tests pass, you'll move on to the next problem.

    3. If you're struggling, type h to see a hint for the current problem.

    4. If a problem feels unclear or too challenging, open an issue on our GitHub page (https://github.com/0XYoussefX0/rustgrind). Fellow learners and maintainers are there to help!

Rustgrind is your companion to mastering Rust and cracking coding interviews. Ready to grind?
"#
            );

            let mut user_input = String::new();

            loop {
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read input");

                if user_input.trim().is_empty() {
                    break;
                }
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
