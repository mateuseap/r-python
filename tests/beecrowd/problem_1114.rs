use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1114_fixed_password() {
    for case in discover_cases(1114) {
        run_beecrowd_case(1114, case);
    }
}
