use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1005_average_1() {
    for case in discover_cases(1005) {
        run_beecrowd_case(1005, case);
    }
}
