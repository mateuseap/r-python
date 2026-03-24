use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1051_taxes() {
    for case in discover_cases(1051) {
        run_beecrowd_case(1051, case);
    }
}
