use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProblemInfo {
    name: String,
    dir: String,
}

#[derive(Deserialize)]
struct InfoFile {
    problems: Vec<ProblemInfo>,
}

#[proc_macro]
pub fn include_files(_: TokenStream) -> TokenStream {
    let info_file: &str = include_str!("../info.toml");
    let problems = toml_edit::de::from_str::<InfoFile>(info_file)
        .expect("Failed to parse `info.toml")
        .problems;

    let problem_files = problems
        .iter()
        .map(|problem| format!("../problems/{}/{}.rs", problem.dir, problem.name));
    let solution_files = problems
        .iter()
        .map(|problem| format!("../solutions/{}/{}.rs", problem.dir, problem.name));

    let mut dirs = Vec::with_capacity(32);
    let mut dir_inds = vec![0; problems.len()];

    for (problem, dir_ind) in problems.iter().zip(&mut dir_inds) {
        // The directory is often the last one inserted.
        if let Some(ind) = dirs.iter().rev().position(|dir| *dir == problem.dir) {
            *dir_ind = dirs.len() - 1 - ind;
            continue;
        }

        dirs.push(problem.dir.as_str());
        *dir_ind = dirs.len() - 1;
    }

    let readmes = dirs
        .iter()
        .map(|dir| format!("../problems/{dir}/README.md"));

    quote! {
        EmbeddedFiles {
            info_file: #info_file,
            problem_files: &[#(ProblemFile { problem: include_bytes!(#problem_files), solution: include_bytes!(#solution_files), dir_ind: #dir_inds }),*],
            problem_dirs: &[#(ProblemDir { name: #dirs, readme: include_bytes!(#readmes) }),*]
        }
    }
    .into()
}
