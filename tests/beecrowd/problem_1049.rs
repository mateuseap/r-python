use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1049_animal() {
    let cases = discover_cases(1049);
    assert!(
        !cases.is_empty(),
        "No fixtures found for problem 1049; add input/expected files under tests/fixtures/beecrowd/1049"
    );

    for case in cases {
        run_beecrowd_case(1049, case);
    }
}
