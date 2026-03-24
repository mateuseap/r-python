use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1019_time_conversion() {
    for case in discover_cases(1019) {
        run_beecrowd_case(1019, case);
    }
}
