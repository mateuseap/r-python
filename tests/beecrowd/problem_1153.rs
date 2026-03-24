use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1153_simple_factorial() {
    for case in discover_cases(1153) {
        run_beecrowd_case(1153, case);
    }
}
