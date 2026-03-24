use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1000_hello_world() {
    let cases = discover_cases(1000);
    assert!(
        !cases.is_empty(),
        "No fixtures found for problem 1000; add input/expected files under tests/fixtures/beecrowd/1000"
    );

    for case in cases {
        run_beecrowd_case(1000, case);
    }
}
