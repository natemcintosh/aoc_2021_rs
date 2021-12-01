fn parse_input(input_str: &str) -> Vec<i32> {
    // Split by line, and parse each line into a number
    input_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(depth_measurements: &[i32]) -> usize {
    // Take the difference of each number and the one before it
    depth_measurements
        .windows(2)
        .map(|nums| nums[1] - nums[0])
        .filter(|&delta| delta > 0)
        .count()
}

fn part2(depth_measurements: &[i32]) -> usize {
    // Sum up each window of length 3
    let length_3_window_sums: Vec<i32> =  depth_measurements.windows(3).map(|nums| nums.iter().sum()).collect();

    // Take the differences of the sums of the windows
    length_3_window_sums
        .windows(2)
        .map(|nums| nums[1] - nums[0])
        .filter(|&delta| delta > 0)
        .count()
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day01.txt").expect("Failed to read day 1 input file");

    // Parse the input into a vector of numbers
    let input = parse_input(&input_str);

    println!(
        "Setup took {:.6} microseconds",
        setup_time.elapsed().as_micros()
    );

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&input);
    println!(
        "Part 2 took {:.6} microseconds",
        part2_time.elapsed().as_micros()
    );

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

// Test functions
#[test]
fn test_parse_input() {
    // The input
    let input_str: &str = "199
200
208
210
200
207
240
269
260
263";

    let expected: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, part1(&input));
}

#[test]
fn test_part2() {
    let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(5, part2(&input));
}
