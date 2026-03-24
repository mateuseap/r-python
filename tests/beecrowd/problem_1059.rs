use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1059_even_numbers() {
    for case in discover_cases(1059) {
        run_beecrowd_case(1059, case);
    }
}
