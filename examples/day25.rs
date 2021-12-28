use ndarray::{Array2, ArrayView2};

fn parse_input(input: &str) -> Array2<u8> {
    let nrows = input.lines().count();
    let ncols = input.chars().take_while(|&c| c != '\n').count();
    let mut result = Array2::zeros((nrows, ncols));
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, c) in row.trim().chars().enumerate() {
            match c {
                '.' => result[[row_idx, col_idx]] = 0,
                '>' => result[[row_idx, col_idx]] = 1,
                'v' => result[[row_idx, col_idx]] = 2,
                _ => panic!("Input, {}, was unexpected", c),
            }
        }
    }

    result
}

fn step_in_direction(inp_arr: ArrayView2<u8>, herd_to_step: u8) -> (Array2<u8>, usize) {
    assert!(
        [1, 2].contains(&herd_to_step),
        "herd to step was not 1 or 2"
    );
    let dir = if herd_to_step == 1 { (0, 1) } else { (1, 0) };

    let mut result = inp_arr.to_owned();

    let mut num_changed: usize = 0;

    // For each slug in the herd (denoted by `herd_to_step`)
    inp_arr
        .indexed_iter()
        // Is this item one of the herd we're interested in?
        .filter(|(_, item)| **item == herd_to_step)
        // Which ones have open neighbors?
        .filter(|(idx, _)| {
            // Make sure that anything over get's properly wrapped back around
            let new_idx = (
                (idx.0 + dir.0) % inp_arr.nrows(),
                (idx.1 + dir.1) % inp_arr.ncols(),
            );
            if inp_arr[new_idx] == 0 {
                true
            } else {
                false
            }
        })
        // Move the ones that are open
        .for_each(|(idx, _)| {
            // Update the count of number changed
            num_changed += 1;

            // Get the new index
            let new_idx = (
                (idx.0 + dir.0) % inp_arr.nrows(),
                (idx.1 + dir.1) % inp_arr.ncols(),
            );
            // Make the item at current idx 0
            result[idx] = 0;

            // Make the item at new idx the new slug
            result[new_idx] = herd_to_step;
        });

    (result, num_changed)
}

fn part1(arr: ArrayView2<u8>) -> usize {
    let mut cuces = arr.to_owned();

    let mut num_changed = 100;
    let mut iter_num = 0;

    // Step through until no sea cucumbers move
    while num_changed > 0 {
        let r = step_in_direction(cuces.view(), 1);
        let arr2 = r.0;
        let east_changed = r.1;
        let r = step_in_direction(arr2.view(), 2);
        cuces = r.0;
        iter_num += 1;
        num_changed = east_changed + r.1;
    }

    iter_num
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day25.txt").expect("Failed to read day 25 input");
    let arr = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(arr.view());
    println!("Part 1 took {:.6} ms", part1_time.elapsed().as_millis());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = solve(&input_image, &algo, 50);
    // println!("Part 2 took {:.6} ms", part2_time.elapsed().as_millis());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input_1() {
    let input_str = "...>>>>>...\n";
    let expected = ndarray::arr2(&[[0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0]]);
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_parse_input_2() {
    let input_str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    let expected = ndarray::arr2(&[
        [2, 0, 0, 0, 1, 1, 0, 2, 2, 1],
        [0, 2, 2, 1, 1, 0, 2, 2, 0, 0],
        [1, 1, 0, 1, 2, 1, 0, 0, 0, 2],
        [1, 1, 2, 1, 1, 0, 1, 0, 2, 0],
        [2, 1, 2, 0, 2, 2, 0, 2, 0, 0],
        [1, 0, 1, 1, 0, 0, 2, 0, 0, 0],
        [0, 2, 2, 0, 0, 1, 0, 1, 2, 0],
        [2, 0, 2, 0, 0, 1, 1, 2, 0, 2],
        [0, 0, 0, 0, 2, 0, 0, 2, 0, 1],
    ]);
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_step_in_direction_1() {
    let arr = ndarray::arr2(&[[0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0]]);
    let (got, _) = step_in_direction(arr.view(), 1);
    let expected = ndarray::arr2(&[[0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0]]);
    assert_eq!(expected, got);
}

#[test]
fn test_step_in_direction_2() {
    let arr = ndarray::arr2(&[[0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0]]);
    let (got, _) = step_in_direction(arr.view(), 2);
    let expected = ndarray::arr2(&[[0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0]]);
    assert_eq!(expected, got);
}

#[test]
fn test_step_in_direction_3() {
    let arr = ndarray::arr2(&[[0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0]]);
    let (got, _) = step_in_direction(arr.view(), 1);
    let expected = ndarray::arr2(&[[0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0]]);
    assert_eq!(expected, got);
}

#[test]
fn test_can_do_two_steps_correctly() {
    let input_str = "..........
.>v....v..
.......>..
..........";
    let arr = parse_input(input_str);
    let expected = ndarray::arr2(&[
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 2, 0, 0, 0, 0, 2, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    let (got1, _) = step_in_direction(arr.view(), 1);
    let (got2, _) = step_in_direction(got1.view(), 2);

    assert_eq!(expected, got2);
}

#[test]
fn test_over_side_of_map() {
    let input_str = "...>...
.......
......>
v.....>
......>
.......
..vvv..";
    let arr = parse_input(input_str);
    let (got1, _) = step_in_direction(arr.view(), 1);
    let (got2, _) = step_in_direction(got1.view(), 2);

    let expected = ndarray::arr2(&[
        [0, 0, 2, 2, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0],
        [2, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 2, 0, 0],
    ]);

    assert_eq!(expected, got2);
}

#[test]
fn test_part1() {
    let arr = ndarray::arr2(&[
        [2, 0, 0, 0, 1, 1, 0, 2, 2, 1],
        [0, 2, 2, 1, 1, 0, 2, 2, 0, 0],
        [1, 1, 0, 1, 2, 1, 0, 0, 0, 2],
        [1, 1, 2, 1, 1, 0, 1, 0, 2, 0],
        [2, 1, 2, 0, 2, 2, 0, 2, 0, 0],
        [1, 0, 1, 1, 0, 0, 2, 0, 0, 0],
        [0, 2, 2, 0, 0, 1, 0, 1, 2, 0],
        [2, 0, 2, 0, 0, 1, 1, 2, 0, 2],
        [0, 0, 0, 0, 2, 0, 0, 2, 0, 1],
    ]);
    let expected = 58;
    let got = part1(arr.view());
    assert_eq!(expected, got);
}
