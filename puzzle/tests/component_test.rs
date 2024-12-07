use puzzle::{count_guard_path, count_guard_blocking_possibilities};


#[test]
fn should_count_guard_path() {
    assert_eq!(count_guard_path("tests/resources/puzzle.txt"), 41);
}

#[test]
fn should_count_guard_blocking_possibilities() {
    assert_eq!(count_guard_blocking_possibilities("tests/resources/puzzle.txt"), 6);
}