use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1154_ages() {
    for case in discover_cases(1154) {
        run_beecrowd_case(1154, case);
    }
}
