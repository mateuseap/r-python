use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1028_to_real() {
    for case in discover_cases(1028) {
        run_beecrowd_case(1028, case);
    }
}
