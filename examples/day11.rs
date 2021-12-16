use ndarray::{arr2, Array2, ArrayView2, ArrayViewMut2};

fn parse_input(input: &str) -> Array2<u8> {
    let mut result = Array2::zeros((10, 10));
    for (row_idx, row) in input.trim().lines().enumerate() {
        for (col_idx, value) in row.chars().enumerate() {
            result[(row_idx, col_idx)] =
                value.to_digit(10).expect("Could not convert to digit") as u8;
        }
    }
    result
}

fn get_neighbors(nrows: i32, ncols: i32, row_idx: i32, col_idx: i32) -> Vec<(usize, usize)> {
    const NEIGHBOR_DIRS: [(i32, i32); 8] = [
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];
    NEIGHBOR_DIRS
        .iter()
        .map(|(row_change, col_change)| ((row_idx) + row_change, (col_idx) + col_change))
        .filter(|(new_row_idx, new_col_idx)| {
            (*new_row_idx >= 0)
                & (*new_col_idx >= 0)
                & (*new_row_idx < nrows)
                & (*new_col_idx < ncols)
        })
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Octopus {
    EnergyLevel(u8),
    AboutToFlash,
    AlreadyFlashed,
}

fn time_step(
    arr: &mut ArrayViewMut2<Octopus>,
    neighbors_array: ArrayView2<Vec<(usize, usize)>>,
) -> usize {
    // First, the energy level of each octopus increases by 1.
    arr.mapv_inplace(|octo| match octo {
        Octopus::EnergyLevel(e) if e < 9 => Octopus::EnergyLevel(e + 1),
        Octopus::EnergyLevel(_) => Octopus::AboutToFlash,
        Octopus::AboutToFlash => panic!("Should not have any about to flash yet"),
        Octopus::AlreadyFlashed => panic!("Should not have any that already flashed"),
    });

    // Then, any octopus with an energy level greater than 9 flashes. This increases the
    // energy level of all adjacent octopuses by 1, including octopuses that are
    // diagonally adjacent. If this causes an octopus to have an energy level greater
    // than 9, it also flashes. This process continues as long as new octopuses keep
    // having their energy level increased beyond 9. (An octopus can only flash at most once per step.)

    let mut count: usize = 0;

    loop {
        // Find the ones about to flash
        let about_to_flash: Vec<(usize, usize)> = arr
            .indexed_iter()
            .filter(|(_, &octo)| octo == Octopus::AboutToFlash)
            .map(|((r, c), _)| (r, c))
            .collect();

        if about_to_flash.is_empty() {
            // No one else will flash this step. Set everyone who has flashed back to 0
            arr.mapv_inplace(|octo| match octo {
                Octopus::EnergyLevel(e) => Octopus::EnergyLevel(e),
                Octopus::AlreadyFlashed => Octopus::EnergyLevel(0),
                Octopus::AboutToFlash => Octopus::EnergyLevel(0),
            });

            // Return the count
            return count;
        } else {
            // Increase the count
            count += about_to_flash.len();

            // Carry out the flashes
            about_to_flash.iter().for_each(|(r, c)| {
                // Flash the octopus in question
                arr[(*r, *c)] = Octopus::AlreadyFlashed;

                // Increment its neighbors
                for (neighbor_r, neighbor_c) in &neighbors_array[(*r, *c)] {
                    match arr[(*neighbor_r, *neighbor_c)] {
                        Octopus::EnergyLevel(e) if e < 9 => {
                            arr[(*neighbor_r, *neighbor_c)] = Octopus::EnergyLevel(e + 1);
                        }
                        Octopus::EnergyLevel(_) => {
                            arr[(*neighbor_r, *neighbor_c)] = Octopus::AboutToFlash;
                        }
                        // Do nothing if about to flash or already flashed
                        Octopus::AboutToFlash => (),
                        Octopus::AlreadyFlashed => (),
                    }
                }
            });
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
    }
}

fn part1(
    arr: ArrayView2<Octopus>,
    n_steps: usize,
    neighbors_array: ArrayView2<Vec<(usize, usize)>>,
) -> usize {
    let mut arr = arr.to_owned();
    let mut counter: usize = 0;
    for _ in 0..n_steps {
        counter += time_step(&mut arr.view_mut(), neighbors_array);
    }

    counter
}

fn part2(arr: ArrayView2<Octopus>, neighbors_array: ArrayView2<Vec<(usize, usize)>>) -> usize {
    let mut arr = arr.to_owned();
    const NOCTOPI: usize = 100;
    for iter in 1.. {
        let counter = time_step(&mut arr.view_mut(), neighbors_array);
        if counter == NOCTOPI {
            return iter;
        } else if iter > 10_000 {
            return 0;
        }
    }

    // Have to put this otherwise compiler freaks out
    0
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day11.txt").expect("Failed to read day 11 input");
    let input = parse_input(&input_str);
    let input = input.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(input.view(), 100, neighbors_arr.view());
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(input.view(), neighbors_arr.view());
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let got = parse_input(input_str);

    let expected = arr2(&[
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ]);

    assert_eq!(expected, got);
}

#[test]
fn test_part1_1() {
    let arr = arr2(&[
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ]);
    let arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let got = part1(arr.view(), 2, neighbors_arr.view());
    assert_eq!(35, got);
}

#[test]
fn test_part1_2() {
    let arr = arr2(&[
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ]);
    let arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let got = part1(arr.view(), 10, neighbors_arr.view());
    assert_eq!(204, got);
}

#[test]
fn test_part1_3() {
    let arr = arr2(&[
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ]);
    let arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let got = part1(arr.view(), 100, neighbors_arr.view());
    assert_eq!(1656, got);
}

#[test]
fn test_time_step_1() {
    let arr = arr2(&[
        [1, 1, 1, 1, 1],
        [1, 9, 9, 9, 1],
        [1, 9, 1, 9, 1],
        [1, 9, 9, 9, 1],
        [1, 1, 1, 1, 1],
    ]);
    let mut arr = arr.mapv(|n| Octopus::EnergyLevel(n));
    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let expected = arr2(&[
        [3, 4, 5, 4, 3],
        [4, 0, 0, 0, 4],
        [5, 0, 0, 0, 5],
        [4, 0, 0, 0, 4],
        [3, 4, 5, 4, 3],
    ]);
    let expected = expected.mapv(|n| Octopus::EnergyLevel(n));

    let got = time_step(&mut arr.view_mut(), neighbors_arr.view());

    assert_eq!(expected, arr);
    assert_eq!(9, got);
}

#[test]
fn test_time_step_2() {
    let arr = arr2(&[
        [3, 4, 5, 4, 3],
        [4, 0, 0, 0, 4],
        [5, 0, 0, 0, 5],
        [4, 0, 0, 0, 4],
        [3, 4, 5, 4, 3],
    ]);
    let mut arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let expected = arr2(&[
        [4, 5, 6, 5, 4],
        [5, 1, 1, 1, 5],
        [6, 1, 1, 1, 6],
        [5, 1, 1, 1, 5],
        [4, 5, 6, 5, 4],
    ]);
    let expected = expected.mapv(|n| Octopus::EnergyLevel(n));

    let got = time_step(&mut arr.view_mut(), neighbors_arr.view());

    assert_eq!(expected, arr);
    assert_eq!(0, got);
}

#[test]
fn test_time_step_3() {
    let arr = arr2(&[
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ]);
    let mut arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let expected = arr2(&[
        [6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
        [3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
        [6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
        [7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
        [7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
        [5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
        [3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
        [7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
        [5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
        [6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
    ]);
    let expected = expected.mapv(|n| Octopus::EnergyLevel(n));

    let got = time_step(&mut arr.view_mut(), neighbors_arr.view());

    assert_eq!(expected, arr);
    assert_eq!(0, got);
}

#[test]
fn test_time_step_4() {
    let arr = arr2(&[
        [6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
        [3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
        [6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
        [7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
        [7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
        [5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
        [3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
        [7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
        [5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
        [6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
    ]);
    let mut arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let expected = arr2(&[
        [8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
        [5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
        [8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
        [8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
        [8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
        [6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
        [6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
        [0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
        [9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
        [8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
    ]);
    let expected = expected.mapv(|n| Octopus::EnergyLevel(n));

    let got = time_step(&mut arr.view_mut(), neighbors_arr.view());

    assert_eq!(expected, arr);
    assert_eq!(35, got);
}

#[test]
fn test_time_step_5() {
    let arr = arr2(&[
        [8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
        [5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
        [8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
        [8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
        [8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
        [6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
        [6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
        [0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
        [9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
        [8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
    ]);
    let mut arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let expected = arr2(&[
        [0, 0, 5, 0, 9, 0, 0, 8, 6, 6],
        [8, 5, 0, 0, 8, 0, 0, 5, 7, 5],
        [9, 9, 0, 0, 0, 0, 0, 0, 3, 9],
        [9, 7, 0, 0, 0, 0, 0, 0, 4, 1],
        [9, 9, 3, 5, 0, 8, 0, 0, 6, 3],
        [7, 7, 1, 2, 3, 0, 0, 0, 0, 0],
        [7, 9, 1, 1, 2, 5, 0, 0, 0, 9],
        [2, 2, 1, 1, 1, 3, 0, 0, 0, 0],
        [0, 4, 2, 1, 1, 2, 5, 0, 0, 0],
        [0, 0, 2, 1, 1, 1, 9, 0, 0, 0],
    ]);
    let expected = expected.mapv(|n| Octopus::EnergyLevel(n));

    let got = time_step(&mut arr.view_mut(), neighbors_arr.view());

    assert_eq!(expected, arr);
    assert_eq!(45, got);
}

#[test]
fn test_part2() {
    let arr = arr2(&[
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ]);
    let arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let got = part2(arr.view(), neighbors_arr.view());
    assert_eq!(195, got);
}

#[test]
fn test_part1_actual() {
    let input_str =
        std::fs::read_to_string("input/day11.txt").expect("Failed to read day 11 input");
    let arr = parse_input(&input_str);
    let arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let got = part1(arr.view(), 100, neighbors_arr.view());
    assert_eq!(1757, got);
}

#[test]
fn test_part2_actual() {
    let input_str =
        std::fs::read_to_string("input/day11.txt").expect("Failed to read day 11 input");
    let arr = parse_input(&input_str);
    let arr = arr.mapv(|n| Octopus::EnergyLevel(n));

    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    let got = part2(arr.view(), neighbors_arr.view());
    assert_eq!(422, got);
}
