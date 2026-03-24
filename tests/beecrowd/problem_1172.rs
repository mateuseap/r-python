use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1172_array_replacement_i() {
    for case in discover_cases(1172) {
        run_beecrowd_case(1172, case);
    }
}
