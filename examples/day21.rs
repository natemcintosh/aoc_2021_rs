use itertools::Itertools;

fn parse_input(input: &str) -> (usize, usize) {
    let (p1_line, p2_line) = input
        .split_once('\n')
        .expect("Could not split around newline");

    let p1_start: usize = p1_line
        .split_ascii_whitespace()
        .last()
        .expect("Could not get last item from first line")
        .parse()
        .expect("Could not convert first to number");

    let p2_start: usize = p2_line
        .split_ascii_whitespace()
        .last()
        .expect("Could not get last item from second line")
        .parse()
        .expect("Could not convert second to number");

    (p1_start, p2_start)
}

fn part1(p1_start: usize, p2_start: usize) -> usize {
    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut p1_loc = p1_start;
    let mut p2_loc = p2_start;

    let locs = [10, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut n_die_rolls = 0;

    for (a, b, c) in (1..=100).cycle().tuples() {
        // Increase die rolls
        n_die_rolls += 3;

        // If odd, then it's the first player
        if a % 2 == 1 {
            p1_loc = locs[(p1_loc + a + b + c) % 10];
            p1_score += p1_loc;
            if p1_score >= 1_000 {
                return p2_score * n_die_rolls;
            }
        } else {
            p2_loc = locs[(p2_loc + a + b + c) % 10];
            p2_score += p2_loc;
            if p2_score >= 1_000 {
                return p1_score * n_die_rolls;
            }
        }
    }

    unreachable!()
}

fn part2(p1_start: usize, p2_start: usize) -> usize {
    // It looks like at each step, the number of possibilities increases by 2
    // The pattern looks like
    // 1
    // 2,3,4
    // 3:1, 4:2, 5:3, 6:2, 7:1
    // 4:1, 5:3, 6:6, 7:7, 8:6, 9:3, 10:1
    // 5:1, 6:4, 7:9, 8:16, 9:19, 10:16, 11:9, 12:4, 13:1
    // The number of items produced at each stage, n, (initial stage is 0) is 3^n,

    
    0
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day21.txt").expect("Failed to read day 21 input");
    let (p1_start, p2_start) = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(p1_start, p2_start);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = solve(&input_image, &algo, 50);
    // println!("Part 2 took {:.6} ms", part2_time.elapsed().as_millis());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "Player 1 starting position: 10
Player 2 starting position: 6";
    let expected = (10, 6);
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let got = part1(4, 8);
    assert_eq!(739785, got);
}

#[test]
fn test_part1_actual() {
    let input_str =
        std::fs::read_to_string("input/day21.txt").expect("Failed to read day 21 input");
    let (p1_start, p2_start) = parse_input(&input_str);
    let got = part1(p1_start, p2_start);
    assert_eq!(900099, got);
}

#[test]
fn test_part2() {
    let got = part2(4, 8);
    assert_eq!(444356092776315, got);
}
