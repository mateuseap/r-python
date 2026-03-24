use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1060_positive_numbers() {
    for case in discover_cases(1060) {
        run_beecrowd_case(1060, case);
    }
}
