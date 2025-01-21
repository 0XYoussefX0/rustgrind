use anyhow::{Context, Result};
use serde::Deserialize;

use crate::embedded::EMBEDDED_FILES;

#[inline(always)]
fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
pub struct ProblemInfo {
    pub name: String,

    pub dir: String,
    #[serde(default = "default_true")]
    pub test: bool,

    #[serde(default)]
    pub strict_clippy: bool,

    pub hints: Vec<String>,

    #[serde(default)]
    pub skip_check_unsolved: bool,
}

impl ProblemInfo {
    pub fn path(&self) -> String {
        // 14 = 10 + 1 + 3
        // problems/ + / + .rs
        let mut path = String::with_capacity(14 + self.dir.len() + self.name.len());
        path.push_str("problems/");
        path.push_str(&self.dir);
        path.push('/');
        path.push_str(&self.name);
        path.push_str(".rs");
        path
    }
}

#[derive(Deserialize)]
pub struct InfoFile {
    pub welcome_message: String,
    pub final_message: String,
    pub problems: Vec<ProblemInfo>,
}

impl InfoFile {
    pub fn parse() -> Result<Self> {
        toml_edit::de::from_str(EMBEDDED_FILES.info_file)
            .context("Failed to parse the embedded `info.toml` file")
    }
}
