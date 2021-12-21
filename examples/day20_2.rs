use std::collections::{HashMap, HashSet};

const DIRS: [(i64, i64); 5] = [(-1, 0), (-1, 1), (0, 1), (1, 0), (1, 1)];

fn bubble_out(curr_points: &HashMap<(i64, i64), char>) -> HashMap<(i64, i64), char> {
    // Everything is either off, or it is on and stored in the hashmap
    let curr_keys: HashSet<(i64, i64)> = curr_points.keys().cloned().collect();

    // For each point in the curr_points, bubble it out, and collect the points in a hashset
    let neighbors: HashSet<(i64, i64)> = curr_points
        .iter()
        .flat_map(|((row, col), _)| DIRS.iter().map(move |(r, c)| (row + r, col + c)))
        .collect();

    // Remove any of the current keys from the neighbors
    let neighbors = neighbors.difference(&curr_keys);

    // Build the corresponding hashmap
    let mut result = curr_points.clone();
    // Add the new points
    for n in neighbors {
        result.insert(*n, '.');
    }

    result
}

fn main() {
    println!("Hello, world!");
}
