use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1101_sequence_sum() {
    for case in discover_cases(1101) {
        run_beecrowd_case(1101, case);
    }
}
