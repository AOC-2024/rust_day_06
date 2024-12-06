use day_06::count_guard_path;

#[test]
fn should_count_guard_path() {
    assert_eq!(count_guard_path("tests/resources/puzzle.txt"), 41);
}