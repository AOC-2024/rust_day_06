use std::collections::HashSet;

use parser::read_puzzle;

pub fn count_guard_path(input_path: &str) -> usize {
    let (map, mut guard) = read_puzzle(input_path);
    let itinerary = guard.itinerary(&map);

    match itinerary {
        Some(points) => {
            let unique_points: HashSet<_> = points.into_iter().collect();
            unique_points.len()
        },
        None => 0
    }

}

pub fn count_guard_blocking_possibilities(input_path: &str) -> usize {
    let (map, guard) = read_puzzle(input_path);

    return guard.find_all_blocking_obstructions(&map).len();
}