#[derive(Debug, PartialEq)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Signal {
    fn from(c: char) -> Self {
        match c {
            'a' => Signal::A,
            'b' => Signal::B,
            'c' => Signal::C,
            'd' => Signal::D,
            'e' => Signal::E,
            'f' => Signal::F,
            'g' => Signal::G,
            _ => panic!("Input signal was not in a-g"),
        }
    }
}

fn parse_input_line(input_line: &str) -> (Vec<&str>, Vec<&str>) {
    let (before, after) = input_line
        .split_once('|')
        .expect("Could not split around |");

    (gather_patterns(before), gather_patterns(after))
}

fn gather_patterns(input: &str) -> Vec<&str> {
    input.trim().split_whitespace().collect()
}

fn part1(input: &[(Vec<&str>, Vec<&str>)]) -> usize {
    // For each line of input
    input
        .iter()
        // Get just the output part
        .map(|line| line.1.clone())
        // Count the number of items with either 2, 3, 4, or 7 items
        // Those numbers line up with 1, 7, 4,  and 8 respectively
        .flat_map(|output_items| output_items.iter().map(|&s| s.len()).collect::<Vec<_>>())
        .filter(|pattern_len| [2, 3, 4, 7].contains(pattern_len))
        .count()
}

fn get_output_numers(input: &(Vec<&str>, Vec<&str>)) -> usize {
    // First get just the input signals
    let in_signal = &input.0;
    let out_signal = &input.1;

    // Convert everything to Vec<HashSet<char>> from Vec<&str>
    
    0
}

fn part2(input: &[(Vec<&str>, Vec<&str>)]) -> usize {
    // For each line of the input
    input
        .iter()
        // Get the output numbers
        .map(|line| get_output_numers(line))
        // Sum them up
        .sum()
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day08.txt").expect("Failed to read day 8 input");
    let numbers: Vec<_> = input_str.lines().map(parse_input_line).collect();
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&numbers);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = part2(&numbers);
    // println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
    // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let got: (Vec<&str>, Vec<&str>) = parse_input_line(input_str);

    let expected: (Vec<&str>, Vec<&str>) = (
        vec![
            "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",
        ],
        vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"],
    );
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let input_str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part1(&input);
    assert_eq!(26, got);
}

// #[test]
// fn test_part2_1() {
//     let input_str =
//         "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

//     let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

//     let got = part2(&input);
//     assert_eq!(5353, got);
// }

// #[test]
// fn test_part2_2() {
//     let input_str =
//         "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
//     let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

//     let got = part2(&input);
//     assert_eq!(61229, got);
// }
