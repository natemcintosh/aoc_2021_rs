use std::collections::{HashMap, HashSet};

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

    // Convert from Vec<&str> to Vec<HashSet<char>>
    let char_sets: Vec<HashSet<char>> = in_signal.iter().map(|&s| s.chars().collect()).collect();
    let out_char_set: Vec<HashSet<char>> =
        out_signal.iter().map(|&s| s.chars().collect()).collect();

    // Create a HashMap to put everything in. Use the index of the HashSet as the key
    let mut map: HashMap<usize, char> = HashMap::new();

    // Populate the HashMap
    // Get 1
    let one_idx = char_sets
        .iter()
        .enumerate()
        .find(|(_, s)| s.len() == 2)
        .expect("Could not find the set representing 1")
        .0;
    map.insert(one_idx, '1');

    // Get the 4
    let four_idx = char_sets
        .iter()
        .enumerate()
        .find(|(_, s)| s.len() == 4)
        .expect("Could not find the set representing 4")
        .0;
    map.insert(four_idx, '4');

    // Get the 7
    let seven_idx = char_sets
        .iter()
        .enumerate()
        .find(|(_, s)| s.len() == 3)
        .expect("Could not find the set representing 7")
        .0;
    map.insert(seven_idx, '7');

    // Get the 8
    let eight_idx = char_sets
        .iter()
        .enumerate()
        .find(|(_, s)| s.len() == 7)
        .expect("Could not find the set representing 8")
        .0;
    map.insert(eight_idx, '8');

    // What are the HashSets for 1 and 4?
    let one_char_set = &char_sets[one_idx];
    let four_char_set = &char_sets[four_idx];

    // Now match on the remaining items to add them to the HashMap
    char_sets.iter().enumerate().for_each(|(idx, s)| {
        let one_intersection_size = s.intersection(one_char_set).count();
        let four_intersection_size = s.intersection(four_char_set).count();
        if s.len() == 5 {
            if one_intersection_size == 2 {
                map.insert(idx, '3');
            } else if four_intersection_size == 3 {
                map.insert(idx, '5');
            } else {
                map.insert(idx, '2');
            }
        } else if s.len() == 6 {
            if one_intersection_size == 1 {
                map.insert(idx, '6');
            } else if four_intersection_size == 4 {
                map.insert(idx, '9');
            } else {
                map.insert(idx, '0');
            }
        }
    });

    // Use it to get the output number
    let number_as_chars: String = out_char_set
        .iter()
        // Find the index of this char set in `char_sets`
        .map(|searching_for| {
            char_sets
                .iter()
                .enumerate()
                .find(|(_, item)| searching_for.clone() == (*item).clone())
                .expect("Could not find a matching char set")
        })
        // Get the char for that index
        .map(|(idx, _)| map.get(&idx).expect("Could not get number from map"))
        .map(|c| *c)
        .collect();

    number_as_chars
        .parse()
        .expect("Could not convert final string to a number")
}

fn part2(input: &[(Vec<&str>, Vec<&str>)]) -> usize {
    // For each line of the input
    input
        .iter()
        // Get the output numbers
        .map(get_output_numers)
        // Sum them up
        .sum()
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day08.txt").expect("Failed to read day 8 input");
    let numbers: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();
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
    let input_str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";

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

#[test]
fn test_part2_1() {
    let input_str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(5353, got);
}

#[test]
fn test_part2_full_sample() {
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

    let got = part2(&input);
    assert_eq!(61229, got);
}

#[test]
fn test_part2_2() {
    let input_str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(8394, got);
}

#[test]
fn test_part2_3() {
    let input_str =
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(9781, got);
}

#[test]
fn test_part2_4() {
    let input_str = "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(1197, got);
}

#[test]
fn test_part2_5() {
    let input_str =
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(9361, got);
}

#[test]
fn test_part2_6() {
    let input_str =
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(4873, got);
}

#[test]
fn test_part2_7() {
    let input_str =
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(8418, got);
}

#[test]
fn test_part2_8() {
    let input_str =
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(4548, got);
}

#[test]
fn test_part2_9() {
    let input_str =
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(1625, got);
}

#[test]
fn test_part2_10() {
    let input_str =
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(8717, got);
}

#[test]
fn test_part2_11() {
    let input_str =
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();

    let got = part2(&input);
    assert_eq!(4315, got);
}

#[test]
fn test_part2_actual() {
    let input_str = std::fs::read_to_string("input/day08.txt").expect("Failed to read day 8 input");
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_str.lines().map(parse_input_line).collect();
    let got = part2(&input);
    assert_eq!(983030, got);
}
