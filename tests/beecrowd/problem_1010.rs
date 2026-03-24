use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1010_string_length() {
    for case in discover_cases(1010) {
        run_beecrowd_case(1010, case);
    }
}
