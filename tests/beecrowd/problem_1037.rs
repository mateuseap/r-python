use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1037_interval() {
    for case in discover_cases(1037) {
        run_beecrowd_case(1037, case);
    }
}
