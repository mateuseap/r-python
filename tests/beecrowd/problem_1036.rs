use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1036_fuel_spent() {
    for case in discover_cases(1036) {
        run_beecrowd_case(1036, case);
    }
}
