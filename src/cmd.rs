use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct CmdRunner {
    target_dir: PathBuf,
}

#[derive(Deserialize)]
struct CargoMetadata {
    target_directory: PathBuf,
}

impl CmdRunner {
    pub fn build() -> Result<Self> {
        // Get the target directory from Cargo.
        let metadata_output = Command::new("cargo")
            .arg("metadata")
            .arg("-q")
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps")
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .output()
            .context(CARGO_METADATA_ERR)?;

        if !metadata_output.status.success() {
            bail!("The command `cargo metadata …` failed. Are you in the `rustlings/` directory?");
        }

        let metadata: CargoMetadata = serde_json::de::from_slice(&metadata_output.stdout)
        .context(
            "Failed to read the field `target_directory` from the output of the command `cargo metadata …`",
        )?;

        Ok(Self {
            target_dir: metadata.target_directory,
        })
    }
}

const CARGO_METADATA_ERR: &str = "Failed to run the command `cargo metadata …`
Did you already install Rust?
Try running `cargo --version` to diagnose the problem.";
