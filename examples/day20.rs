use ndarray::Array2;

// fn parse_input(input: &str) -> ([bool; 512], Array2<char>) {
//     // The first line is the image enhancement algorithm
//     let algorithm = input
//         .lines()
//         .next()
//         .expect("Could not get the first line")
//         // Convert each #->true, .->false
//         .chars()
//         .map(|c| match c {
//             '#' => true,
//             '.' => false,
//             _ => panic!("Unexpected character while parsing algorithm: {}", c),
//         })
//         .collect();

//     // Then there's a double newline

//     // Then there's the image data
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

    // let (algo, input_image) = parse_input(input_str);
}
