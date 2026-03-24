use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1052_month() {
    for case in discover_cases(1052) {
        run_beecrowd_case(1052, case);
    }
}
