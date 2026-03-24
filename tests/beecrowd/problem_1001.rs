use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1001_extremely_basic() {
    let cases = discover_cases(1001);
    assert!(
        !cases.is_empty(),
        "No fixtures found for problem 1001; add input/expected files under tests/fixtures/beecrowd/1001"
    );

    for case in cases {
        run_beecrowd_case(1001, case);
    }
}
