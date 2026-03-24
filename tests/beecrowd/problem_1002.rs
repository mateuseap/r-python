use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1002_area_circle() {
    let cases = discover_cases(1002);
    assert!(
        !cases.is_empty(),
        "No fixtures found for problem 1002; add input/expected files under tests/fixtures/beecrowd/1002"
    );

    for case in cases {
        run_beecrowd_case(1002, case);
    }
}
