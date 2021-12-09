use ndarray::{arr2, Array2, ArrayView2, ArrayViewMut2};

fn parse_input(input: &str) -> Array2<u8> {
    let nrows = input.lines().count();
    let ncols = input.chars().take_while(|&c| c != '\n').count();
    let mut result = Array2::zeros((nrows, ncols));
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, value) in row.chars().enumerate() {
            if value.is_ascii_digit() {
                result[[row_idx, col_idx]] =
                    value.to_digit(10).expect("Could not convert to digit") as u8;
            }
        }
    }

    result
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

fn is_lowest_of_neighbors(arr: ArrayView2<u8>, row_idx: usize, col_idx: usize) -> bool {
    // Get the indices of neighbors
    let neighbor_inds = get_neighbors(arr, row_idx, col_idx);

    let val_to_check = arr[[row_idx, col_idx]];

    // For each of those indices
    neighbor_inds
        .iter()
        // Get the value
        .map(|(neighbor_row_idx, neighbor_col_idx)| arr[[*neighbor_row_idx, *neighbor_col_idx]])
        // Is each one larger than the value at this (row_idx, col_idx)
        .all(|val| val > val_to_check)
}

fn part1(arr: ArrayView2<u8>) -> usize {
    // For each location
    arr.indexed_iter()
        // Filter to items where adjacent neighbors lower than the item
        .filter(|((row_idx, col_idx), _)| is_lowest_of_neighbors(arr, *row_idx, *col_idx))
        // Add 1 to each
        .map(|(_, &item)| (usize::from(item)) + 1)
        // Sum them up
        .sum()
}

fn get_basin_size(arr: &mut ArrayViewMut2<u8>, row_idx: usize, col_idx: usize) -> usize {
    // Get the height at the current location
    let height = arr[[row_idx, col_idx]];
    // Set this position as visited
    arr[[row_idx, col_idx]] = u8::MAX;

    // Visit adjacent points
    get_neighbors(arr.view(), row_idx, col_idx)
        .iter()
        .filter_map(|(new_row_idx, new_col_idx)| {
            let adjacent_height = arr[[*new_row_idx, *new_col_idx]];
            if (adjacent_height > height) && adjacent_height < 9 {
                Some(get_basin_size(arr, *new_row_idx, *new_col_idx))
            } else {
                None
            }
        })
        .sum::<usize>()
        + 1
}

fn part2(arr: ArrayView2<u8>) -> usize {
    // Create a mutable copy of the array
    let mut basin_array = arr.to_owned();

    // For each location
    let mut basins: Vec<usize> = arr
        .indexed_iter()
        // Find the lowest points
        .filter(|((row_idx, col_idx), _)| is_lowest_of_neighbors(arr, *row_idx, *col_idx))
        // Get the basin size
        .map(|((row_idx, col_idx), _)| {
            get_basin_size(&mut basin_array.view_mut(), row_idx, col_idx)
        })
        .collect();

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day09.txt").expect("Failed to read day 9 input");
    let arr = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(arr.view());
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(arr.view());
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let expected: Array2<u8> = arr2(&[
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ]);

    let got = parse_input(&input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_get_neighbors() {
    let arr: Array2<u8> = arr2(&[
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ]);
    let got = get_neighbors(arr.view(), 0, 0);
    let expected = vec![(1, 0), (0, 1)];
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let arr: Array2<u8> = arr2(&[
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ]);

    let got = part1(arr.view());
    assert_eq!(15, got);
}

#[test]
fn test_part1_actual() {
    let input_str = std::fs::read_to_string("input/day09.txt").expect("Failed to read day 9 input");
    let arr = parse_input(&input_str);

    let got = part1(arr.view());
    assert_eq!(516, got);
}

#[test]
fn test_part2() {
    let arr: Array2<u8> = arr2(&[
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ]);

    let got = part2(arr.view());
    assert_eq!(1134, got);
}

#[test]
fn test_part2_actual() {
    let input_str = std::fs::read_to_string("input/day09.txt").expect("Failed to read day 9 input");
    let arr = parse_input(&input_str);

    let got = part2(arr.view());
    assert_eq!(1023660, got);
}
