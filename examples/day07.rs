fn parse_input(input: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().expect("Could not read number"))
        .collect();

    // Sort it for easier calculation later
    nums.sort_unstable();
    nums
}

fn unsigned_diff(u1: usize, u2: usize) -> usize {
    if u1 > u2 {
        u1 - u2
    } else {
        u2 - u1
    }
}

fn part1(input: &[usize]) -> usize {
    // `input` must be sorted

    // Get the position of the middle item
    let middle_idx = (input.len() / 2) + (input.len() % 2);

    // Get the median
    let median = input[middle_idx];

    // Get the differences with respect to the median
    input.iter().map(|&n| unsigned_diff(n, median)).sum()
}

fn sum_to_n(n: usize) -> usize {
    ((n + 1) * n) / 2
}

fn cost_at_point(crab_positions: &[usize], central_position: usize) -> usize {
    crab_positions
        .iter()
        // Get the distance to each crab position from the central_position
        .map(|&n| unsigned_diff(n, central_position))
        // Get the cost to travel that distance
        .map(sum_to_n)
        // Sum it up
        .sum()
}

fn part2(input: &[usize]) -> usize {
    // What are the points we need to test
    let minimum = input.iter().min().expect("Could not find the minimum");
    let maximum = input.iter().max().expect("Could not find the maximum");
    (*minimum..=*maximum)
        .into_iter()
        // Test each
        .map(|test_position| cost_at_point(input, test_position))
        // Select the minimum fuel cost
        .min()
        .expect("Could not find a minimum cost")
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day07.txt").expect("Failed to read day 7 input");
    let numbers = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&numbers);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&numbers);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "16,1,2,0,4,2,7,1,2,14";
    let expected: Vec<usize> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let nums: Vec<usize> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];
    let got = part1(&nums);
    assert_eq!(37, got);
}

#[test]
fn test_sum_to_n1() {
    let got = sum_to_n(4);
    assert_eq!(10, got);
}

#[test]
fn test_sum_to_n2() {
    let got = sum_to_n(5);
    assert_eq!(15, got);
}

#[test]
fn test_cost_at_point1() {
    let nums: Vec<usize> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];
    let got = cost_at_point(&nums, 5);
    assert_eq!(168, got);
}

#[test]
fn test_cost_at_point2() {
    let nums: Vec<usize> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];
    let got = cost_at_point(&nums, 2);
    assert_eq!(206, got);
}

#[test]
fn test_part2() {
    let nums: Vec<usize> = vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16];
    let got = part2(&nums);
    assert_eq!(168, got);
}
