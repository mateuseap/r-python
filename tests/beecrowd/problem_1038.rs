use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1038_snack() {
    for case in discover_cases(1038) {
        run_beecrowd_case(1038, case);
    }
}
