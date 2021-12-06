#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn new(s: &str) -> Self {
        match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Input string was not recognized direction"),
        }
    }
}

fn parse_input(input_str: &str) -> Vec<(Direction, i64)> {
    // For each line in the input
    input_str
        .lines()
        // Split around the space
        .map(str::split_whitespace)
        // Convert the first item of each line to a Direction, and the second to a i64
        .map(|mut line_parts| {
            (
                Direction::new(
                    line_parts
                        .next()
                        .expect("Could not get anything from line after splitting whitespace"),
                ),
                line_parts
                    .next()
                    .expect("Could not get a second part of the line")
                    .parse::<i64>()
                    .expect("Could not parse &str to int"),
            )
        })
        .collect::<Vec<_>>()
}

fn part1(instructions: &[(Direction, i64)]) -> i64 {
    let mut depth: i64 = 0;
    let mut distance: i64 = 0;

    for (dir, value) in instructions {
        match dir {
            Direction::Forward => distance += value,
            Direction::Up => depth -= value,
            Direction::Down => depth += value,
        }
    }

    depth * distance
}

fn part2(instructions: &[(Direction, i64)]) -> i64 {
    let mut depth: i64 = 0;
    let mut distance: i64 = 0;
    let mut aim: i64 = 0;

    for (dir, value) in instructions {
        match dir {
            Direction::Forward => {
                depth += aim * value;
                distance += value;
            }
            Direction::Up => aim -= value,
            Direction::Down => aim += value,
        }
    }

    depth * distance
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 2
    let input_str =
        std::fs::read_to_string("input/day02.txt").expect("Failed to read day 2 input file");

    // Parse the input into a vector
    let instructions = parse_input(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&instructions);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&instructions);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let test_str: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let got = parse_input(&test_str);
    let expected: Vec<(Direction, i64)> = vec![
        (Direction::Forward, 5),
        (Direction::Down, 5),
        (Direction::Forward, 8),
        (Direction::Up, 3),
        (Direction::Down, 8),
        (Direction::Forward, 2),
    ];
    assert_eq!(expected, got)
}

#[test]
fn test_part1() {
    let instructions = vec![
        (Direction::Forward, 5),
        (Direction::Down, 5),
        (Direction::Forward, 8),
        (Direction::Up, 3),
        (Direction::Down, 8),
        (Direction::Forward, 2),
    ];
    let got = part1(&instructions);
    let expected = 150;
    assert_eq!(expected, got)
}

#[test]
fn test_part2() {
    let instructions = vec![
        (Direction::Forward, 5),
        (Direction::Down, 5),
        (Direction::Forward, 8),
        (Direction::Up, 3),
        (Direction::Down, 8),
        (Direction::Forward, 2),
    ];
    let got = part2(&instructions);
    let expected = 900;
    assert_eq!(expected, got)
}
