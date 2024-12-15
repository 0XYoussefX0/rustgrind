use anyhow::{bail, Context, Result};
use colored::{Color, Colorize};
use std::env::set_current_dir;
use std::fs::{self, create_dir};
use std::io::Write;
use std::{
    io,
    path::Path,
    process::{Command, Stdio},
};

use crate::embedded::EMBEDDED_FILES;
use crate::info_file::{InfoFile, ProblemInfo};
use crate::term::press_enter_prompt;

pub fn init() -> Result<()> {
    let rustgrind_dir = Path::new("rustgrind");

    if rustgrind_dir.exists() {
        bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
    }

    if Path::new("exercises").exists() && Path::new("solutions").exists() {
        bail!(IN_INITIALIZED_DIR_ERR);
    }

    let mut stdout = io::stdout().lock();

    stdout.write_all(b"This command will create the directory `rustgrind/` which will contain the exercises.\nPress ENTER to continue ")?;
    press_enter_prompt(&mut stdout)?;

    create_dir(rustgrind_dir).context("Failed to create the `rustgrind/` directory")?;
    set_current_dir(rustgrind_dir)
        .context("Failed to change the current directory to `rustgrind/`")?;

    let info_file = InfoFile::parse()?;
    EMBEDDED_FILES
        .init_exercises_and_solutions_dirs(&info_file.problems)
        .context("Failed to initialize the rustgrind problems and solutions directories")?;

    let current_cargo_toml = include_str!("../dev-Cargo.toml");
    let updated_cargo_toml = update_cargo_toml(&info_file.problems, current_cargo_toml);

    fs::write("Cargo.toml", updated_cargo_toml)
        .context("Failed to create the file `rustlings/Cargo.toml`")?;

    fs::write("rust-analyzer.toml", RUST_ANALYZER_TOML)
        .context("Failed to create the file `rustlings/rust-analyzer.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Failed to create the file `rustlings/.gitignore`")?;

    create_dir(".vscode").context("Failed to create the directory `rustgrind/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Failed to create the file `rustgrind/.vscode/extensions.json`")?;

    // Ignore any Git error because Git initialization is not required.
    let _ = Command::new("git")
        .arg("init")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let my_green = Color::TrueColor {
        r: 34,
        g: 203,
        b: 135,
    };

    stdout.write_all("Initialization done ✓".color(my_green).as_bytes())?;
    stdout.write_all(b"\n\n")?;

    stdout.write_all(POST_INIT_MSG.bold().as_bytes())?;

    Ok(())
}

const CARGO_TOML_SIZE_ESTIMATE: usize = 1 << 13;

fn update_cargo_toml(problems: &[ProblemInfo], current_toml_file: &str) -> String {
    let mut updated_cargo_toml = String::with_capacity(CARGO_TOML_SIZE_ESTIMATE);
    updated_cargo_toml.push_str("bin = [");
    for problem in problems {
        let problem_entry = format!(
            "\n{{ name = \"{}\", path = \"../problems/{}/{}.rs\" }},",
            problem.name, problem.dir, problem.name
        );
        let solution_entry = format!(
            "\n{{ name = \"{}_sol\", path = \"../solutions/{}/{}.rs\" }},",
            problem.name, problem.dir, problem.name
        );
        updated_cargo_toml.push_str(&problem_entry);
        updated_cargo_toml.push_str(&solution_entry);
    }
    updated_cargo_toml.push_str("\n]\n");
    updated_cargo_toml.push_str(current_toml_file);

    updated_cargo_toml
}

const RUSTLINGS_DIR_ALREADY_EXISTS_ERR: &str =
    "A directory with the name `rustgrind` already exists in the current directory.
You probably already initialized Rustgrind.
Run `cd rustgrind`
Then run `rustgrind` again";

const IN_INITIALIZED_DIR_ERR: &str =
    "It looks like Rustlings is already initialized in this directory.";

const POST_INIT_MSG: &str = "Run `cd rustlings` to go into the generated directory.
Then run `rustlings` to get started.
";

const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const GITIGNORE: &[u8] = b"Cargo.lock
target/
.vscode/
";

const RUST_ANALYZER_TOML: &[u8] = br#"check.command = "clippy"
check.extraArgs = ["--profile", "test"]
"#;
