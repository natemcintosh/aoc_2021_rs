use std::collections::BinaryHeap;

use ndarray::{Array2, ArrayView2};

fn parse_input(input: &str) -> Array2<u8> {
    let nrows = input.lines().count();
    let ncols = input
        .lines()
        .next()
        .expect("There was not even a single line of text to read")
        .chars()
        .count();

    let mut arr = Array2::<u8>::zeros((nrows, ncols));
    for (ridx, line) in input.lines().enumerate() {
        for (cidx, n) in line.chars().enumerate() {
            // arr[(ridx, cidx)] = n.try_into().expect("Could not parse number");
            arr[(ridx, cidx)] = n
                .to_digit(10)
                .expect("Could not parse number")
                .try_into()
                .expect("Could not convert u32 to u8");
        }
    }
    arr
}

fn get_neighbors(arr: ArrayView2<u8>, row_idx: usize, col_idx: usize) -> Vec<(usize, usize)> {
    let nrows = arr.nrows() as i32;
    let ncols = arr.ncols() as i32;
    const NEIGHBOR_DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    NEIGHBOR_DIRS
        .iter()
        .map(|(row_change, col_change)| {
            ((row_idx as i32) + row_change, (col_idx as i32) + col_change)
        })
        .filter(|(new_row_idx, new_col_idx)| {
            (*new_row_idx >= 0)
                & (*new_col_idx >= 0)
                & (*new_row_idx < nrows)
                & (*new_col_idx < ncols)
        })
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Not entirely sure, might actually be Dijkstra's.
/// Based on [this](https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples)
fn uniform_cost_search(
    arr: ArrayView2<u8>,
    start: (usize, usize),
    goal: (usize, usize),
) -> (Array2<usize>, Array2<(usize, usize)>) {
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        position: start,
    });

    let mut costs = Array2::<usize>::zeros((arr.nrows(), arr.ncols()));
    costs.fill(usize::MAX);

    let mut predecessors = Array2::<(usize, usize)>::from_elem((arr.nrows(), arr.ncols()), (0, 0));

    while let Some(State {
        cost,
        position: pos,
    }) = frontier.pop()
    {
        if pos == goal {
            return (costs, predecessors);
        }

        if cost > costs[pos] {
            continue;
        }

        for nbr in get_neighbors(arr, pos.0, pos.1) {
            let new = State {
                cost: cost + usize::from(arr[nbr]),
                position: nbr,
            };

            // If the new cost is less than before
            if new.cost < costs[new.position] {
                // add it to the frontier
                frontier.push(new);

                // update the costs array
                costs[new.position] = new.cost;

                // Update its predecessor
                predecessors[new.position] = pos;
            }
        }
    }
    (costs, predecessors)
}

fn reconstruct_path(
    arr: ArrayView2<(usize, usize)>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut result: Vec<_> = Vec::new();
    result.push(goal);
    let mut curr = goal;
    let mut itr = 0;
    while (curr != start) & (itr < 1_000) {
        itr += 1;
        curr = arr[curr];
        result.push(curr);
    }
    if itr < 1_000 {
        result.reverse();
        return Some(result);
    }
    None
}

// Can I use something like accumulate along each row and each column and use the cumulative
// scores to pick the best path? Possibly, but I don't know how it would work

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day15.txt").expect("Failed to read day 15 input");
    let arr = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let goal = (arr.nrows() - 1, arr.ncols() - 1);
    let (costs, _) = uniform_cost_search(arr.view(), (0, 0), goal);
    let part1_result = costs[goal];
    // dbg!(reconstruct_path(predecessors.view(), (0, 0), goal));
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    // let part2_result =
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    #[test]
    fn test_parse_input() {
        let input_str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let expected = arr2(&[
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]);
        let got = parse_input(input_str);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_part1() {
        let arr: Array2<u8> = arr2(&[
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]);

        let expected_rows: [usize; 19] = [0, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 9];
        let expected_cols: [usize; 19] = [0, 0, 0, 1, 2, 3, 4, 5, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9];
        let expected_path: Vec<(usize, usize)> = expected_rows
            .iter()
            .zip(expected_cols.iter())
            .map(|(&r, &c)| (r, c))
            .collect();
        let (got_costs, got_path) = uniform_cost_search(arr.view(), (0, 0), (9, 9));

        let goal = (9, 9);
        assert_eq!(got_costs[goal], 40);

        let path = reconstruct_path(got_path.view(), (0, 0), goal);
        let got_path = path.expect("Could not find a proper path home");
        assert_eq!(expected_path, got_path);
    }
}
