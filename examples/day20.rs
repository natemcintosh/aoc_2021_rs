use std::collections::HashMap;

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

const DIRS: [(i64, i64); 5] = [(-1, 0), (-1, 1), (0, 1), (1, 0), (1, 1)];

fn get_replacement(
    image: &HashMap<(i64, i64), char>,
    this_pixel: (i64, i64),
    algo: &[char],
) -> char {
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
            None => '.',
        })
        .collect();

    // Convert it to decimal
    let idx = usize::from_str_radix(&binary_representation, 2)
        .expect("Could not parse binary representation");

    // What should the replacement pixel be?
    algo[idx]
}

// fn enhance(arr: ArrayView2<bool>, algo: &[bool]) -> Array2<bool> {
//     // Create a result
//     let mut result = pad_arr(arr, false);

//     // Assuming that the input arr has already been padded.
//     let input_ncols = arr.ncols() - 2;
//     let input_nrows = arr.nrows() - 2;
//     // For each window of size 3, get the decimal index for the algorithm
//     // the windows() method seems to go across the rows first, then down the columns
//     for (window, (row_idx, col_idx)) in itertools::zip_eq(
//         arr.windows((3, 3)),
//         (0..input_nrows)
//             .into_iter()
//             .flat_map(|r| (0..input_ncols).into_iter().map(move |c| (r, c))),
//     ) {
//         dbg!(window, row_idx, col_idx);
//         let idx = arr_to_binary_to_decimal(window);
//         let result_pixel = algo[idx];
//         result[(row_idx + 1, col_idx + 1)] = result_pixel;
//     }

//     result
// }

fn main() {
    println!("Hello, world!");
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

// #[test]
// fn test_enhance() {
//     let arr = arr2(&[
//         [false, false, false, false, false, false, false],
//         [false, true, false, false, true, false, false],
//         [false, true, false, false, false, false, false],
//         [false, true, true, false, false, true, false],
//         [false, false, false, true, false, false, false],
//         [false, false, false, true, true, true, false],
//         [false, false, false, false, false, false, false],
//     ]);

//     let algo = vec![
//         false, false, true, false, true, false, false, true, true, true, true, true, false, true,
//         false, true, false, true, false, true, true, true, false, true, true, false, false, false,
//         false, false, true, true, true, false, true, true, false, true, false, false, true, true,
//         true, false, true, true, true, true, false, false, true, true, true, true, true, false,
//         false, true, false, false, false, false, true, false, false, true, false, false, true,
//         true, false, false, true, true, true, false, false, true, true, true, true, true, true,
//         false, true, true, true, false, false, false, true, true, true, true, false, false, true,
//         false, false, true, true, true, true, true, false, false, true, true, false, false, true,
//         false, true, true, true, true, true, false, false, false, true, true, false, true, false,
//         true, false, false, true, false, true, true, false, false, true, false, true, false, false,
//         false, false, false, false, true, false, true, true, true, false, true, true, true, true,
//         true, true, false, true, true, true, false, true, true, true, true, false, false, false,
//         true, false, true, true, false, true, true, false, false, true, false, false, true, false,
//         false, true, true, true, true, true, false, false, false, false, false, true, false, true,
//         false, false, false, false, true, true, true, false, false, true, false, true, true, false,
//         false, false, false, false, false, true, false, false, false, false, false, true, false,
//         false, true, false, false, true, false, false, true, true, false, false, true, false,
//         false, false, true, true, false, true, true, true, true, true, true, false, true, true,
//         true, true, false, true, true, true, true, false, true, false, true, false, false, false,
//         true, false, false, false, false, false, false, false, true, false, false, true, false,
//         true, false, true, false, false, false, true, true, true, true, false, true, true, false,
//         true, false, false, false, false, false, false, true, false, false, true, false, false,
//         false, true, true, false, true, false, true, true, false, false, true, false, false, false,
//         true, true, false, true, false, true, true, false, false, true, true, true, false, true,
//         false, false, false, false, false, false, true, false, true, false, false, false, false,
//         false, false, false, true, false, true, false, true, false, true, true, true, true, false,
//         true, true, true, false, true, true, false, false, false, true, false, false, false, false,
//         false, true, true, true, true, false, true, false, false, true, false, false, true, false,
//         true, true, false, true, false, false, false, false, true, true, false, false, true, false,
//         true, true, true, true, false, false, false, false, true, true, false, false, false, true,
//         true, false, false, true, false, false, false, true, false, false, false, false, false,
//         false, true, false, true, false, false, false, false, false, false, false, true, false,
//         false, false, false, false, false, false, true, true, false, false, true, true, true, true,
//         false, false, true, false, false, false, true, false, true, false, true, false, false,
//         false, true, true, false, false, true, false, true, false, false, true, true, true, false,
//         false, true, true, true, true, true, false, false, false, false, false, false, false,
//         false, true, false, false, true, true, true, true, false, false, false, false, false,
//         false, true, false, false, true,
//     ];

//     let got = enhance(arr.view(), &algo);

//     let expected_arr = arr2(&[
//         [
//             false, false, false, false, false, false, false, false, false,
//         ],
//         [false, false, true, true, false, true, true, false, false],
//         [false, true, false, false, true, false, true, false, false],
//         [false, true, true, false, true, false, false, true, false],
//         [false, true, true, true, true, false, false, true, false],
//         [false, false, true, false, false, true, true, false, false],
//         [false, false, false, true, true, false, false, true, false],
//         [false, false, false, false, true, false, true, false, false],
//         [
//             false, false, false, false, false, false, false, false, false,
//         ],
//     ]);
//     assert_eq!(expected_arr, got);
// }
