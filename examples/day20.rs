use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (Vec<char>, HashMap<(i64, i64), char>) {
    // The first line is the image enhancement algorithm
    let algorithm: Vec<char> = input
        .lines()
        .next()
        .expect("Could not get the first line")
        .chars()
        .collect();

    // Then there's a double newline
    let (_, image_str) = input
        .split_once("\n\n")
        .expect("Could not get the image lines");

    // Get the image array into a HashMap
    let mut image: HashMap<(i64, i64), char> = HashMap::with_capacity(image_str.len());
    for (row_idx, row) in image_str.lines().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            // Insert the character into the hashmap
            image.insert((row_idx as i64, col_idx as i64), c);
        }
    }

    (algorithm, image)
}

// The directions have to go top to bottom, left to right
const DIRS: [(i64, i64); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_index(
    image: &HashMap<(i64, i64), char>,
    this_pixel: (i64, i64),
    value_at_inf: char,
) -> usize {
    // What are the neighbors for this_pixel?
    let neighbors: Vec<(i64, i64)> = DIRS
        .into_iter()
        .map(|(row, col)| (this_pixel.0 + row, this_pixel.1 + col))
        .collect();

    // Get their values
    let binary_representation: String = neighbors
        .iter()
        .map(|&key| match image.get(&key) {
            Some(c) => *c,
            None => value_at_inf,
        })
        .map(|c| match c {
            '#' => '1',
            '.' => '0',
            some_char => panic!("Character, {}, was not '0' or '1'", some_char),
        })
        .collect();

    // Convert it to decimal
    usize::from_str_radix(&binary_representation, 2).expect("Could not parse binary representation")
}

fn bubble_out(curr_points: &HashMap<(i64, i64), char>) -> HashMap<(i64, i64), char> {
    // Everything is either off, or it is on and stored in the hashmap
    let curr_keys: HashSet<(i64, i64)> = curr_points.keys().cloned().collect();

    // For each point in the curr_points, bubble it out, and collect the points in a hashset
    let neighbors: HashSet<(i64, i64)> = curr_points
        .iter()
        .flat_map(|((row, col), _)| DIRS.iter().map(move |(r, c)| (row + r, col + c)))
        .collect();

    // Remove any of the current keys from the neighbors
    let neighbors = neighbors.difference(&curr_keys);

    // Build the corresponding hashmap
    let mut result = curr_points.clone();
    // Add the new points
    for n in neighbors {
        result.insert(*n, '.');
    }

    result
}

fn enhance(
    curr_points: &HashMap<(i64, i64), char>,
    algo: &[char],
    value_at_inf: char,
) -> (HashMap<(i64, i64), char>, char) {
    // First bubble out to get all locations that might require enhancement
    let expanded_pts = bubble_out(curr_points);

    // Create a new hashmap to put results in
    let mut result: HashMap<(i64, i64), char> = HashMap::with_capacity(expanded_pts.len());

    // For each point in `expanded_pts`, get its replacement
    for &this_pixel in expanded_pts.keys() {
        let idx = get_index(curr_points, this_pixel, value_at_inf);
        result.insert(this_pixel, algo[idx]);
    }

    // Figure out if we need to change the value at infinity
    let val_at_idx_0 = algo[0];
    let val_at_end = algo[algo.len() - 1];
    let new_val_at_infinity: char = if val_at_idx_0 == '.' {
        '.'
    } else {
        // If current `value_at_inf` is '.' then '#'
        if value_at_inf == '.' {
            '#'
        } else {
            // if `val_at_end` is '#' then '#' else '.'
            if val_at_end == '#' {
                '#'
            } else {
                '.'
            }
        }
    };

    (result, new_val_at_infinity)
}

fn part1(curr_points: &HashMap<(i64, i64), char>, algo: &[char]) -> usize {
    // Run enhance twice, then count the pixels == '#'
    let (once, new_val_at_infinity) = enhance(curr_points, algo, '.');
    let (twice, _) = enhance(&once, algo, new_val_at_infinity);
    twice.values().filter(|&&c| c == '#').count()
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day20.txt").expect("Failed to read day 20 input");
    let (algo, input_image) = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input_image, &algo);
    println!("Part 1 took {:.6} ms", part1_time.elapsed().as_millis());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = part2(&board_numbers, &board_views);
    // println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    let (algo, input_image) = parse_input(input_str);

    let expected_algo = vec![
        '.', '.', '#', '.', '#', '.', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#',
        '.', '#', '#', '#', '.', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '#',
        '.', '#', '.', '.', '#', '#', '#', '.', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#',
        '#', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '#', '.', '.',
        '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '.', '.',
        '#', '#', '#', '#', '.', '.', '#', '.', '.', '#', '#', '#', '#', '#', '.', '.', '#', '#',
        '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '#', '.', '#', '.', '#',
        '.', '.', '#', '.', '#', '#', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '#',
        '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '#',
        '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '#', '#', '.', '.', '#', '.', '.', '#',
        '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.',
        '.', '#', '#', '#', '.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.',
        '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '.', '#', '#', '.', '.', '#',
        '.', '.', '.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.',
        '#', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.',
        '.', '#', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#', '#', '#', '#', '.', '#',
        '#', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '#',
        '.', '#', '.', '#', '#', '.', '.', '#', '.', '.', '.', '#', '#', '.', '#', '.', '#', '#',
        '.', '.', '#', '#', '#', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.',
        '.', '.', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '.', '#', '#',
        '#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '.',
        '#', '.', '.', '#', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '#', '#',
        '.', '.', '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '.', '.', '#',
        '#', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.',
        '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.',
        '#', '#', '#', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.',
        '#', '#', '.', '.', '#', '.', '#', '.', '.', '#', '#', '#', '.', '.', '#', '#', '#', '#',
        '#', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.',
        '.', '.', '.', '.', '#', '.', '.', '#',
    ];

    let expected_image = HashMap::from([
        ((0, 0), '#'),
        (((0, 1), '.')),
        (((0, 2), '.')),
        (((0, 3), '#')),
        (((0, 4), '.')),
        ((1, 0), '#'),
        (((1, 1), '.')),
        (((1, 2), '.')),
        (((1, 3), '.')),
        (((1, 4), '.')),
        ((2, 0), '#'),
        (((2, 1), '#')),
        (((2, 2), '.')),
        (((2, 3), '.')),
        (((2, 4), '#')),
        ((3, 0), '.'),
        (((3, 1), '.')),
        (((3, 2), '#')),
        (((3, 3), '.')),
        (((3, 4), '.')),
        ((4, 0), '.'),
        (((4, 1), '.')),
        (((4, 2), '#')),
        (((4, 3), '#')),
        (((4, 4), '#')),
    ]);

    assert_eq!(expected_algo, algo);
    assert_eq!(expected_image, input_image);
}

#[test]
fn test_enhance() {
    let image = HashMap::from([
        ((0, 0), '#'),
        ((0, 1), '.'),
        ((0, 2), '.'),
        ((0, 3), '#'),
        ((0, 4), '.'),
        ((1, 0), '#'),
        ((1, 1), '.'),
        ((1, 2), '.'),
        ((1, 3), '.'),
        ((1, 4), '.'),
        ((2, 0), '#'),
        ((2, 1), '#'),
        ((2, 2), '.'),
        ((2, 3), '.'),
        ((2, 4), '#'),
        ((3, 0), '.'),
        ((3, 1), '.'),
        ((3, 2), '#'),
        ((3, 3), '.'),
        ((3, 4), '.'),
        ((4, 0), '.'),
        ((4, 1), '.'),
        ((4, 2), '#'),
        ((4, 3), '#'),
        ((4, 4), '#'),
    ]);

    let algo = vec![
        '.', '.', '#', '.', '#', '.', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#',
        '.', '#', '#', '#', '.', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '#',
        '.', '#', '.', '.', '#', '#', '#', '.', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#',
        '#', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '#', '.', '.',
        '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '.', '.',
        '#', '#', '#', '#', '.', '.', '#', '.', '.', '#', '#', '#', '#', '#', '.', '.', '#', '#',
        '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '#', '.', '#', '.', '#',
        '.', '.', '#', '.', '#', '#', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '#',
        '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '#',
        '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '#', '#', '.', '.', '#', '.', '.', '#',
        '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.',
        '.', '#', '#', '#', '.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.',
        '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '.', '#', '#', '.', '.', '#',
        '.', '.', '.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.',
        '#', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.',
        '.', '#', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#', '#', '#', '#', '.', '#',
        '#', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '#',
        '.', '#', '.', '#', '#', '.', '.', '#', '.', '.', '.', '#', '#', '.', '#', '.', '#', '#',
        '.', '.', '#', '#', '#', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.',
        '.', '.', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '.', '#', '#',
        '#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '.',
        '#', '.', '.', '#', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '#', '#',
        '.', '.', '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '.', '.', '#',
        '#', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.',
        '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.',
        '#', '#', '#', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.',
        '#', '#', '.', '.', '#', '.', '#', '.', '.', '#', '#', '#', '.', '.', '#', '#', '#', '#',
        '#', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.',
        '.', '.', '.', '.', '#', '.', '.', '#',
    ];

    let (got, new_val_at_infinity) = enhance(&image, &algo, '.');
    assert_eq!('.', new_val_at_infinity);

    let expected_image = HashMap::from([
        ((-1, -1), '.'),
        ((-1, 0), '#'),
        ((-1, 1), '#'),
        ((-1, 2), '.'),
        ((-1, 3), '#'),
        ((-1, 4), '#'),
        ((-1, 5), '.'),
        ((0, -1), '#'),
        ((0, 0), '.'),
        ((0, 1), '.'),
        ((0, 2), '#'),
        ((0, 3), '.'),
        ((0, 4), '#'),
        ((0, 5), '.'),
        ((1, -1), '#'),
        ((1, 0), '#'),
        ((1, 1), '.'),
        ((1, 2), '#'),
        ((1, 3), '.'),
        ((1, 4), '.'),
        ((1, 5), '#'),
        ((2, -1), '#'),
        ((2, 0), '#'),
        ((2, 1), '#'),
        ((2, 2), '#'),
        ((2, 3), '.'),
        ((2, 4), '.'),
        ((2, 5), '#'),
        ((3, -1), '.'),
        ((3, 0), '#'),
        ((3, 1), '.'),
        ((3, 2), '.'),
        ((3, 3), '#'),
        ((3, 4), '#'),
        ((3, 5), '.'),
        ((4, -1), '.'),
        ((4, 0), '.'),
        ((4, 1), '#'),
        ((4, 2), '#'),
        ((4, 3), '.'),
        ((4, 4), '.'),
        ((4, 5), '#'),
        ((5, -1), '.'),
        ((5, 0), '.'),
        ((5, 1), '.'),
        ((5, 2), '#'),
        ((5, 3), '.'),
        ((5, 4), '#'),
        ((5, 5), '.'),
    ]);

    assert_eq!(expected_image, got);
}

#[test]
fn test_get_index_1() {
    let image = HashMap::from([
        ((0, 0), '#'),
        ((0, 1), '#'),
        ((0, 2), '.'),
        ((1, 0), '#'),
        ((1, 1), '.'),
        ((1, 2), '.'),
        ((2, 0), '#'),
        ((2, 1), '#'),
        ((2, 2), '#'),
    ]);

    let got = get_index(&image, (0, 0), '.');
    assert_eq!(26, got);
}

#[test]
fn test_get_replacement_2() {
    let image = HashMap::from([
        ((0, 0), '#'),
        ((0, 1), '#'),
        ((0, 2), '.'),
        ((1, 0), '#'),
        ((1, 1), '.'),
        ((1, 2), '.'),
        ((2, 0), '#'),
        ((2, 1), '#'),
        ((2, 2), '#'),
    ]);

    let got = get_index(&image, (1, 1), '.');
    assert_eq!(423, got);
}

#[test]
fn test_get_replacement_3() {
    let image = HashMap::from([
        ((0, 0), '.'),
        ((0, 1), '.'),
        ((0, 2), '.'),
        ((1, 0), '#'),
        ((1, 1), '.'),
        ((1, 2), '.'),
        ((2, 0), '.'),
        ((2, 1), '#'),
        ((2, 2), '.'),
    ]);

    let got = get_index(&image, (1, 1), '.');
    assert_eq!(34, got);
}

#[test]
fn test_part1() {
    let image = HashMap::from([
        ((0, 0), '#'),
        ((0, 1), '.'),
        ((0, 2), '.'),
        ((0, 3), '#'),
        ((0, 4), '.'),
        ((1, 0), '#'),
        ((1, 1), '.'),
        ((1, 2), '.'),
        ((1, 3), '.'),
        ((1, 4), '.'),
        ((2, 0), '#'),
        ((2, 1), '#'),
        ((2, 2), '.'),
        ((2, 3), '.'),
        ((2, 4), '#'),
        ((3, 0), '.'),
        ((3, 1), '.'),
        ((3, 2), '#'),
        ((3, 3), '.'),
        ((3, 4), '.'),
        ((4, 0), '.'),
        ((4, 1), '.'),
        ((4, 2), '#'),
        ((4, 3), '#'),
        ((4, 4), '#'),
    ]);

    let algo = vec![
        '.', '.', '#', '.', '#', '.', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#',
        '.', '#', '#', '#', '.', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '#',
        '.', '#', '.', '.', '#', '#', '#', '.', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#',
        '#', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '#', '.', '.',
        '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '.', '.',
        '#', '#', '#', '#', '.', '.', '#', '.', '.', '#', '#', '#', '#', '#', '.', '.', '#', '#',
        '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '#', '.', '#', '.', '#',
        '.', '.', '#', '.', '#', '#', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '#',
        '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '#',
        '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '#', '#', '.', '.', '#', '.', '.', '#',
        '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.',
        '.', '#', '#', '#', '.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.',
        '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '.', '#', '#', '.', '.', '#',
        '.', '.', '.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.',
        '#', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.',
        '.', '#', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#', '#', '#', '#', '.', '#',
        '#', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '#',
        '.', '#', '.', '#', '#', '.', '.', '#', '.', '.', '.', '#', '#', '.', '#', '.', '#', '#',
        '.', '.', '#', '#', '#', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.',
        '.', '.', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '.', '#', '#',
        '#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '.',
        '#', '.', '.', '#', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '#', '#',
        '.', '.', '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '.', '.', '#',
        '#', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.',
        '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.',
        '#', '#', '#', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.',
        '#', '#', '.', '.', '#', '.', '#', '.', '.', '#', '#', '#', '.', '.', '#', '#', '#', '#',
        '#', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.',
        '.', '.', '.', '.', '#', '.', '.', '#',
    ];

    let got = part1(&image, &algo);
    assert_eq!(35, got);
}
