use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    // For each row and column, we store the character at that position.
    input.lines().map(|line| line.chars().collect()).collect()
}

fn most_common_char(input: &[char]) -> char {
    let mut counts: [u32; 2] = [0, 0];
    for c in input {
        if *c == '0' {
            counts[0] += 1;
        } else {
            counts[1] += 1;
        }
    }
    if counts[0] > counts[1] {
        '0'
    } else {
        '1'
    }
}

fn flip_bits(input: &[char]) -> Vec<char> {
    input
        .iter()
        .map(|c| if *c == '0' { '1' } else { '0' })
        .collect()
}

fn part1(input: &[Vec<char>]) -> usize {
    // Collect the items in each column
    let mut cols: Vec<Vec<char>> = vec![Vec::new(); input[0].len()];
    for row in input {
        for (i, c) in row.iter().enumerate() {
            cols[i].push(*c);
        }
    }

    // For each column, get the most common character.
    let most_common_char_in_each_col: Vec<char> =
        cols.iter().map(|col| most_common_char(col)).collect();

    // gamma is the number created from the bits
    let gamma = usize::from_str_radix(
        &most_common_char_in_each_col
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>(),
        2,
    )
    .expect("Could not parse gamma");

    // Flip all the bits in most_common_char_in_each_col
    let least_common_char_in_each_col = flip_bits(&most_common_char_in_each_col);

    // epsilon is the number created from the least common bits
    let epsilon = usize::from_str_radix(
        &least_common_char_in_each_col
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>(),
        2,
    )
    .expect("Could not parse epsilon");

    gamma * epsilon
}

fn bit_criteria(input: &[char], is_oxygen: bool) -> char {
    let mut counts: [u32; 2] = [0, 0];
    for c in input {
        if *c == '0' {
            counts[0] += 1;
        } else {
            counts[1] += 1;
        }
    }

    match (counts[0], counts[1], is_oxygen) {
        (n_zeros, n_ones, oxy) if (n_zeros > n_ones) && oxy => '0',
        (n_zeros, n_ones, oxy) if (n_zeros > n_ones) && !oxy => '1',
        (n_zeros, n_ones, oxy) if (n_zeros < n_ones) && oxy => '1',
        (n_zeros, n_ones, oxy) if (n_zeros < n_ones) && !oxy => '0',
        (_, _, oxy) if oxy => '1',
        (_, _, oxy) if !oxy => '0',
        _ => unreachable!(),
    }
}

fn get_rows<T: Clone>(v: &[Vec<T>], rows: &HashSet<usize>) -> Vec<Vec<T>> {
    // Return the rows in the set
    v.iter()
        .enumerate()
        .filter(|(idx, _)| rows.contains(idx))
        .map(|(i, _)| v[i].clone())
        .collect()
}

fn get_col<T: Copy>(v: &[Vec<T>], wanted_col_idx: usize) -> Vec<T> {
    // Return only the one column of `v` that is wanted_col_idx
    let mut result = Vec::new();
    for row in v.iter() {
        for (col_idx, &item) in row.iter().enumerate() {
            if col_idx == wanted_col_idx {
                result.push(item);
            }
        }
    }
    result
}

fn get_number_for_sensor(input: &Vec<Vec<char>>, is_oxygen: bool) -> usize {
    let mut to_alter = input.to_owned();
    for col_idx in 0..input[0].len() {
        // Get this column
        let this_col = get_col(&to_alter, col_idx);

        // What's the one to keep according to bit_criteria?
        let desired_char = bit_criteria(&this_col, is_oxygen);

        // Gather a HashSet of the row indices desired to keep
        let rows_to_keep: HashSet<usize> = this_col
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == desired_char)
            .map(|(idx, _)| idx)
            .collect();

        // Get the new to_alter
        to_alter = get_rows(&to_alter, &rows_to_keep);

        // If only one row left, break out
        if to_alter.len() == 1 {
            break;
        }
    }

    usize::from_str_radix(
        &to_alter[0]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>(),
        2,
    )
    .expect("Could not parse number")
}

fn part2(input: &Vec<Vec<char>>) -> usize {
    let oxygen_number = get_number_for_sensor(input, true);

    let co2_number = get_number_for_sensor(input, false);

    oxygen_number * co2_number
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day03.txt").expect("Could not read day 3 input");
    let input = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());
    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&input);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let expected: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '0', '1', '1', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
        vec!['0', '1', '1', '1', '1'],
        vec!['0', '0', '1', '1', '1'],
        vec!['1', '1', '1', '0', '0'],
        vec!['1', '0', '0', '0', '0'],
        vec!['1', '1', '0', '0', '1'],
        vec!['0', '0', '0', '1', '0'],
        vec!['0', '1', '0', '1', '0'],
    ];
    assert_eq!(parse_input(&input), expected);
}

#[test]
fn test_part1() {
    let input: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '0', '1', '1', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
        vec!['0', '1', '1', '1', '1'],
        vec!['0', '0', '1', '1', '1'],
        vec!['1', '1', '1', '0', '0'],
        vec!['1', '0', '0', '0', '0'],
        vec!['1', '1', '0', '0', '1'],
        vec!['0', '0', '0', '1', '0'],
        vec!['0', '1', '0', '1', '0'],
    ];

    assert_eq!(part1(&input), 198);
}

#[test]
fn test_part1_actual() {
    let input_str = std::fs::read_to_string("input/day03.txt").expect("Could not read day 3 input");
    let input = parse_input(&input_str);
    assert_eq!(3320834, part1(&input));
}

#[test]
fn test_get_rows() {
    let input: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '0', '1', '1', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
        vec!['0', '1', '1', '1', '1'],
        vec!['0', '0', '1', '1', '1'],
        vec!['1', '1', '1', '0', '0'],
        vec!['1', '0', '0', '0', '0'],
        vec!['1', '1', '0', '0', '1'],
        vec!['0', '0', '0', '1', '0'],
        vec!['0', '1', '0', '1', '0'],
    ];

    let rows: HashSet<usize> = vec![0, 3, 8].into_iter().collect();
    let expected: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '0', '0', '0'],
    ];
    let got = get_rows(&input, &rows);
    assert_eq!(got, expected);
}

#[test]
fn test_get_col() {
    let input: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '0', '1', '1', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
        vec!['0', '1', '1', '1', '1'],
        vec!['0', '0', '1', '1', '1'],
        vec!['1', '1', '1', '0', '0'],
        vec!['1', '0', '0', '0', '0'],
        vec!['1', '1', '0', '0', '1'],
        vec!['0', '0', '0', '1', '0'],
        vec!['0', '1', '0', '1', '0'],
    ];
    let wanted_col_idx: usize = 2;
    let expected: Vec<char> = vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '0', '0', '0'];
    let got = get_col(&input, wanted_col_idx);
    assert_eq!(expected, got)
}

#[test]
fn test_part2() {
    let input: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '0', '1', '1', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
        vec!['0', '1', '1', '1', '1'],
        vec!['0', '0', '1', '1', '1'],
        vec!['1', '1', '1', '0', '0'],
        vec!['1', '0', '0', '0', '0'],
        vec!['1', '1', '0', '0', '1'],
        vec!['0', '0', '0', '1', '0'],
        vec!['0', '1', '0', '1', '0'],
    ];

    assert_eq!(part2(&input), 230);
}

#[test]
fn test_part2_actual() {
    let input_str = std::fs::read_to_string("input/day03.txt").expect("Could not read day 3 input");
    let input = parse_input(&input_str);
    assert_eq!(4481199, part2(&input));
}
