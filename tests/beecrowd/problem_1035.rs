use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1035_selection_test() {
    let cases = discover_cases(1035);
    assert!(
        !cases.is_empty(),
        "No fixtures found for problem 1035; add input/expected files under tests/fixtures/beecrowd/1035"
    );

    for case in cases {
        run_beecrowd_case(1035, case);
    }
}
