use puzzle::count_guard_path;

fn main() {

    let guard_path_count = count_guard_path("src/resources/puzzle.txt");

    //5444
    println!("Distinct guard path count: {guard_path_count}");
}
