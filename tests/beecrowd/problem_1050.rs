use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1050_ddd() {
    for case in discover_cases(1050) {
        run_beecrowd_case(1050, case);
    }
}
