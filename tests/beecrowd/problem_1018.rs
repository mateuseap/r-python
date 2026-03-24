use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1018_banknotes() {
    let cases = discover_cases(1018);
    assert!(
        !cases.is_empty(),
        "No fixtures found for problem 1018; add input/expected files under tests/fixtures/beecrowd/1018"
    );

    for case in cases {
        run_beecrowd_case(1018, case);
    }
}
