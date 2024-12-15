use std::fs::{self, create_dir};
use std::io;

use crate::info_file::ProblemInfo;
use anyhow::{Context, Error, Result};

pub static EMBEDDED_FILES: EmbeddedFiles = rustlings_macros::include_files!();

pub struct EmbeddedFiles {
    pub info_file: &'static str,
    problem_files: &'static [ProblemFile],
    pub problem_dirs: &'static [ProblemDir],
}

pub struct ProblemDir {
    pub name: &'static str,
    readme: &'static [u8],
}

fn create_dir_if_not_exists(path: &str) -> Result<()> {
    if let Err(e) = create_dir(path) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(Error::from(e).context(format!("Failed to create the directory {path}")));
        }
    }

    Ok(())
}

impl ProblemDir {
    fn init_on_disk(&self) -> Result<()> {
        // 20 = 10 + 10
        // problems/ + /README.md
        let mut problem_dir_path = String::with_capacity(19 + self.name.len());
        problem_dir_path.push_str("problems/");
        problem_dir_path.push_str(self.name);
        create_dir_if_not_exists(&problem_dir_path)?;

        let mut readme_path = problem_dir_path;
        readme_path.push_str("/README.md");

        let _ = fs::write(&readme_path, self.readme)
            .with_context(|| format!("Failed to write the file {readme_path}"));

        let mut solution_dir_path = readme_path;
        solution_dir_path.clear();
        solution_dir_path.push_str("solutions/");
        solution_dir_path.push_str(self.name);
        create_dir_if_not_exists(&solution_dir_path)?;

        return Ok(());
    }
}

struct ProblemFile {
    problem: &'static [u8],
    solution: &'static [u8],
    dir_ind: usize,
}

impl EmbeddedFiles {
    pub fn init_exercises_and_solutions_dirs(&self, problems: &[ProblemInfo]) -> Result<()> {
        create_dir("problems").context("Failed to create the directory `problems`")?;
        create_dir("solutions").context("Failed to create the directory `solutions`")?;

        fs::write(
            "problems/README.md",
            include_bytes!("../problems/README.md"),
        )
        .context("Failed to write the file problems/README.md")?;

        fs::write(
            "solutions/README.md",
            include_bytes!("../solutions/README.md"),
        )
        .context("Failed to write the file solutions/README.md")?;

        for dir in self.problem_dirs {
            dir.init_on_disk()?;
        }

        let mut problem_path = String::with_capacity(64);
        let problem_prefix = "problems/";
        problem_path.push_str(problem_prefix);

        let mut solution_path = String::with_capacity(64);
        let solution_prefix = "solutions/";
        solution_path.push_str(solution_prefix);

        for (problem, problem_file) in problems.iter().zip(self.problem_files) {
            let dir = &self.problem_dirs[problem_file.dir_ind];

            problem_path.truncate(problem_prefix.len());
            problem_path.push_str(dir.name);
            problem_path.push_str(&problem.name);
            problem_path.push_str(".rs");

            solution_path.truncate(solution_prefix.len());
            solution_path.push_str(dir.name);
            solution_path.push_str(&problem.name);
            solution_path.push_str(".rs");

            fs::write(&problem_path, problem_file.problem)
                .with_context(|| format!("Failed to write the problem file {problem_path}"))?;

            fs::write(&solution_path, problem_file.solution)
                .with_context(|| format!("Failed to write the problem file {problem_path}"))?;
        }

        Ok(())
    }
}
