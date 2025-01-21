use crate::cmd::CmdRunner;
use crate::info_file::ProblemInfo;
use crate::problem::Problem;
use crate::term;
use anyhow::{Context, Result};
use std::env;
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::Read,
    path::MAIN_SEPARATOR_STR,
};

pub enum StateFileStatus {
    Read,
    NotRead,
}

#[must_use]
pub enum ExercisesProgress {
    // All exercises are done.
    AllDone,
    // A new exercise is now pending.
    NewPending,
    // The current exercise is still pending.
    CurrentPending,
}

pub struct AppState {
    current_problem_ind: usize,
    problems: Vec<Problem>,
    // Caches the number of done exercises to avoid iterating over all exercises every time.
    n_done: u16,
    final_message: String,
    state_file: File,
    // Preallocated buffer for reading and writing the state file.
    file_buf: Vec<u8>,
    cmd_runner: CmdRunner,
    // Running in VS Code.
    vs_code: bool,
}

const STATE_FILE_NAME: &str = ".rustlings-state.txt";

const STATE_FILE_HEADER: &[u8] = b"DON'T EDIT THIS FILE!\n\n";

impl AppState {
    pub fn new(
        problems: Vec<ProblemInfo>,
        final_message: String,
    ) -> Result<(Self, StateFileStatus)> {
        let cmd_runner = CmdRunner::build()?;
        let mut state_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(STATE_FILE_NAME)
            .with_context(|| {
                format!("Failed to open or create the state file {STATE_FILE_NAME}")
            })?;

        let dir_canonical_path = term::canonicalize("problems");
        let mut problems_info = problems
            .into_iter()
            .map(|problem| {
                // Leaking to be able to borrow in the watch mode `Table`.
                // Leaking is not a problem because the `AppState` instance lives until
                // the end of the program.

                let path = problem.path().leak();
                let name = problem.name.leak();
                let dir = problem.dir.leak();
                let hints = problem.hints.leak();

                let canonical_path = dir_canonical_path.as_deref().map(|dir_canonical_path| {
                    let mut canonical_path;
                    canonical_path = String::with_capacity(
                        2 + dir_canonical_path.len() + dir.len() + name.len(),
                    );
                    canonical_path.push_str(dir_canonical_path);
                    canonical_path.push_str(MAIN_SEPARATOR_STR);
                    canonical_path.push_str(dir);
                    canonical_path.push_str(MAIN_SEPARATOR_STR);
                    canonical_path.push_str(name);
                    canonical_path.push_str(".rs");
                    canonical_path
                });

                Problem {
                    dir,
                    name,
                    path,
                    canonical_path,
                    test: problem.test,
                    strict_clippy: problem.strict_clippy,
                    hints,
                    // Updated below.
                    done: false,
                }
            })
            .collect::<Vec<_>>();

        let mut current_problem_ind = 0;
        let mut n_done = 0;
        let mut file_buf = Vec::with_capacity(2048);
        let state_file_status = 'block: {
            if state_file.read_to_end(&mut file_buf).is_err() {
                break 'block StateFileStatus::NotRead;
            }

            let mut lines = file_buf.split(|c| *c == b'\n').skip(2);

            let Some(current_problem_name) = lines.next() else {
                break 'block StateFileStatus::NotRead;
            };

            if current_problem_name.is_empty() || lines.next().is_none() {
                break 'block StateFileStatus::NotRead;
            }

            let mut done_problems = HashSet::with_capacity(problems_info.len());

            for done_problem_name in lines {
                if done_problem_name.is_empty() {
                    break;
                }
                done_problems.insert(done_problem_name);
            }

            for (ind, problem) in problems_info.iter_mut().enumerate() {
                if done_problems.contains(problem.name.as_bytes()) {
                    problem.done = true;
                    n_done += 1;
                }

                if problem.name.as_bytes() == current_problem_name {
                    current_problem_ind = ind;
                }
            }

            StateFileStatus::Read
        };

        file_buf.clear();
        file_buf.extend_from_slice(STATE_FILE_HEADER);

        let slf = Self {
            current_problem_ind,
            problems: problems_info,
            n_done,
            final_message,
            state_file,
            file_buf,
            cmd_runner,
            vs_code: env::var_os("TERM_PROGRAM").is_some_and(|v| v == "vscode"),
        };

        Ok((slf, state_file_status))
    }

    pub fn problems(&self) -> &[Problem] {
        &self.problems
    }
}
