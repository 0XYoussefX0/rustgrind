use crate::info_file::ProblemInfo;

const CARGO_TOML_SIZE_ESTIMATE: usize = 1 << 13;

pub fn update_cargo_toml(problems: &[ProblemInfo], current_toml_file: &str) -> String {
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
