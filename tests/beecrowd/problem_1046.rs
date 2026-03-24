use super::{discover_cases, run_beecrowd_case};

#[test]
fn problem_1046_game_time() {
    for case in discover_cases(1046) {
        run_beecrowd_case(1046, case);
    }
}
