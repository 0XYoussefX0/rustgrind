use anyhow::{Context, Result};
use serde::Deserialize;

use crate::embedded::EMBEDDED_FILES;

// #[inline(always)]
// fn default_true() -> bool {
//     true
// }

#[derive(Deserialize)]
pub struct ProblemInfo {
    pub name: String,

    pub dir: String,
    // #[serde(default = "default_true")]
    // pub test: bool,

    // #[serde(default)]
    // pub strict_clippy: bool,

    // pub hint: Vec<String>,

    // #[serde(default)]
    // pub skip_check_unsolved: bool,
}

#[derive(Deserialize)]
pub struct InfoFile {
    // pub welcome_message: Option<String>,
    // pub final_message: Option<String>,
    pub problems: Vec<ProblemInfo>,
}

impl InfoFile {
    pub fn parse() -> Result<Self> {
        toml_edit::de::from_str(EMBEDDED_FILES.info_file)
            .context("Failed to parse the embedded `info.toml` file")
    }
}
