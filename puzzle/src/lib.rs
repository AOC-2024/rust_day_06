use std::collections::HashSet;

use parser::read_puzzle;

pub fn count_guard_path(input_path: &str) -> usize {
    let (map, mut guard) = read_puzzle(input_path);
    let itinerary = guard.itinerary(&map);

    let unique_points: HashSet<_> = itinerary.into_iter().collect();
    unique_points.len()
}
