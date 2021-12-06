fn parse_input(input: &str) -> [usize; 9] {
    let nums: Vec<usize> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().expect("Could not read number"))
        .collect();

    let mut v: [usize; 9] = [0; 9];
    for n in nums {
        v[n] += 1;
    }
    v
}

fn fish_life(population: &mut [usize; 9]) {
    population.rotate_left(1);
    population[6] += population[8];
}

fn solve(input: &[usize; 9], n_days: usize) -> usize {
    let mut population = *input;
    for _ in 0..n_days {
        fish_life(&mut population);
    }

    population.iter().sum()
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day06.txt").expect("Failed to read day 6 input");
    let numbers = parse_input(&input_str);
    println!(
        "Setup took {:.6} µs",
        setup_time.elapsed().as_micros()
    );

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = solve(&numbers, 80);
    println!(
        "Part 1 took {:.6} µs",
        part1_time.elapsed().as_micros()
    );

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = solve(&numbers, 256);
    println!(
        "Part 2 took {:.6} µs",
        part2_time.elapsed().as_micros()
    );

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "3,4,3,1,2";
    let expected: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let input: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];
    let expected_18: usize = 26;
    let expected_80: usize = 5934;

    let got_18 = solve(&input, 18);
    let got_80 = solve(&input, 80);

    assert_eq!(expected_18, got_18);
    assert_eq!(expected_80, got_80);
}

#[test]
fn test_part2() {
    let input: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];
    let got = solve(&input, 256);
    assert_eq!(26984457539, got);
}
