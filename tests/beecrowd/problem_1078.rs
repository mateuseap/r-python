use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1078_multiplication_table() {
    let cases = discover_cases(1078);
    assert!(
        !cases.is_empty(),
        "No fixtures found for problem 1078; add input/expected files under tests/fixtures/beecrowd/1078"
    );

    for case in cases {
        run_beecrowd_case(1078, case);
    }
}
