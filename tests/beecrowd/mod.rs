use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::path::{Path, PathBuf};

mod problem_1000;
mod problem_1001;
mod problem_1002;
mod problem_1005;
mod problem_1010;
mod problem_1018;
mod problem_1019;
mod problem_1028;
mod problem_1035;
mod problem_1036;
mod problem_1037;
mod problem_1038;
mod problem_1046;
mod problem_1049;
mod problem_1050;
mod problem_1051;
mod problem_1052;
mod problem_1059;
mod problem_1060;
mod problem_1071;
mod problem_1078;
mod problem_1101;
mod problem_1114;
mod problem_1153;
mod problem_1154;

fn fixture_dir(problem_id: u32) -> PathBuf {
    PathBuf::from("tests/fixtures/beecrowd").join(format!("{problem_id:04}"))
}

fn read_case_file(dir: &Path, prefix: &str, case: u32) -> String {
    let file = dir.join(format!("{prefix}_{case:02}.txt"));
    fs::read_to_string(&file).unwrap_or_else(|e| panic!("Failed to read {}: {}", file.display(), e))
}

pub fn discover_cases(problem_id: u32) -> Vec<u32> {
    let dir = fixture_dir(problem_id);
    let mut cases = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if let Some(num_str) = name.strip_prefix("input_") {
                    if let Some(num_str) = num_str.strip_suffix(".txt") {
                        if let Ok(num) = num_str.parse::<u32>() {
                            cases.push(num);
                        }
                    }
                }
            }
        }
    }

    cases.sort_unstable();
    cases
}

pub fn run_beecrowd_case(problem_id: u32, case: u32) {
    let dir = fixture_dir(problem_id);
    let solution = dir.join("solution.rpy");
    assert!(
        solution.exists(),
        "Missing solution file at {}",
        solution.display()
    );

    let input = read_case_file(&dir, "input", case);
    let expected = read_case_file(&dir, "expected", case);

    let mut cmd = cargo_bin_cmd!("r-python");
    let output = cmd
        .arg(&solution)
        .write_stdin(input)
        .output()
        .expect("Failed to execute r-python binary");

    assert!(
        output.status.success(),
        "Problem {problem_id} case {case} exited with status {:?}. Stderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim_end(),
        expected.trim_end(),
        "Problem {problem_id} case {case} produced unexpected output. Stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
pub mod problem_1172;
