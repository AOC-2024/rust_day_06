use day_06::count_guard_path;

fn main() {

    let guard_path_count = count_guard_path("src/resources/puzzle.txt");
    println!("Distinct guard path count: {guard_path_count}");
}
