use puzzle::{count_guard_path, count_guard_blocking_possibilities};

fn main() {

    let guard_path_count = count_guard_path("src/resources/puzzle.txt");
    //5444
    println!("Distinct guard path count: {guard_path_count}");

    let blocking_possibilities = count_guard_blocking_possibilities("src/resources/puzzle.txt");
    //1840 too low
    println!("Blocking possibilities: {blocking_possibilities}");
}
