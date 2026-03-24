use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1071_sum_odd_numbers() {
    for case in discover_cases(1071) {
        run_beecrowd_case(1071, case);
    }
}
