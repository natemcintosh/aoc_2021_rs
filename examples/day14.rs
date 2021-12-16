use itertools::Itertools;
use std::{collections::HashMap, str};

fn parse_input(
    input: &str,
) -> (
    HashMap<(char, char), usize>,
    HashMap<(char, char), ((char, char), (char, char))>,
) {
    let (start_str, rules) = input
        .split_once("\n\n")
        .expect("Could not split around double newline");

    // Now break up the start_str into pairs of letters
    let pairs: HashMap<(char, char), usize> =
        start_str
            .trim()
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut acc, (c1, c2)| {
                {
                    *acc.entry((c1, c2)).or_insert(0) += 1;
                }
                acc
            });

    let rules = rules
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .expect("Could not split around arrow")
        })
        .map(|(key, val)| {
            let cs: Vec<char> = key.chars().collect();
            let char_val = val
                .chars()
                .next()
                .expect("Could not convert str val to char");
            let left_side = (cs[0], char_val);
            let right_side = (char_val, cs[1]);
            ((cs[0], cs[1]), (left_side, right_side))
        })
        .collect::<HashMap<(char, char), ((char, char), (char, char))>>();

    (pairs, rules)
}

fn step(
    input: &HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), ((char, char), (char, char))>,
) -> HashMap<(char, char), usize> {
    // Create a result vec
    let mut result: HashMap<(char, char), usize> = HashMap::new();

    // Iterate over the input
    input
        .iter()
        // Check if a rule exists for this pair
        .filter(|(pair, _)| rules.contains_key(pair))
        // If so, add the new values to the result
        .for_each(|(pair, val)| {
            // Get the replacement pairs
            let (p1, p2) = rules
                .get(pair)
                .expect("Could not find pair we should have filtered out");

            // Add them to the result
            *result.entry(*p1).or_insert(0) += val;
            *result.entry(*p2).or_insert(0) += val;
        });

    result
}

fn solve(
    input_str: &HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), ((char, char), (char, char))>,
    nsteps: usize,
    last_char: char,
) -> usize {
    let mut input = input_str.clone();
    for _ in 0..nsteps {
        input = step(&input, rules);
    }

    // Calculate how many times each letter appears
    let mut letter_count: HashMap<char, usize> = HashMap::new();
    // Get only the first from each. For the last item, also get its last character
    for ((c1, _), val) in &input {
        *letter_count.entry(*c1).or_insert(0) += val;
    }

    // Get the last character from
    *letter_count.entry(last_char).or_insert(0) += 1;

    // Get the min and the max
    let min_max = letter_count.iter().map(|(_, &n)| n).minmax();
    let (min_char, max_char) = match min_max {
        itertools::MinMaxResult::NoElements => panic!("Could not find min max"),
        itertools::MinMaxResult::OneElement(_) => panic!("Could only find one element"),
        itertools::MinMaxResult::MinMax(min_char, max_char) => (min_char, max_char),
    };
    max_char - min_char

    // SHOULD USE A HASHMAP, AND THEN ADD 1 TO THE COUNT OF THE LAST LETTER OF THE INPUT.
    // SUM ONLY USING THE FIRST ITEM IN EACH HASHMAP
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day14.txt").expect("Failed to read day 14 input");
    let (input, rules) = parse_input(&input_str);
    let last_letter = input_str
        .lines()
        .next()
        .expect("Could not get first line from input string")
        .chars()
        .last()
        .expect("Could not get last char of first line");
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = solve(&input, &rules, 10, last_letter);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = solve(&input, &rules, 40, last_letter);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let (input_str, rules) = parse_input(input_str);
    let expected_str: HashMap<(char, char), usize> =
        HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
    let expected_rules = HashMap::from([
        (('C', 'H'), (('C', 'B'), ('B', 'H'))),
        (('H', 'H'), (('H', 'N'), ('N', 'H'))),
        (('C', 'B'), (('C', 'H'), ('H', 'B'))),
        (('N', 'H'), (('N', 'C'), ('C', 'H'))),
        (('H', 'B'), (('H', 'C'), ('C', 'B'))),
        (('H', 'C'), (('H', 'B'), ('B', 'C'))),
        (('H', 'N'), (('H', 'C'), ('C', 'N'))),
        (('N', 'N'), (('N', 'C'), ('C', 'N'))),
        (('B', 'H'), (('B', 'H'), ('H', 'H'))),
        (('N', 'C'), (('N', 'B'), ('B', 'C'))),
        (('N', 'B'), (('N', 'B'), ('B', 'B'))),
        (('B', 'N'), (('B', 'B'), ('B', 'N'))),
        (('B', 'B'), (('B', 'N'), ('N', 'B'))),
        (('B', 'C'), (('B', 'B'), ('B', 'C'))),
        (('C', 'C'), (('C', 'N'), ('N', 'C'))),
        (('C', 'N'), (('C', 'C'), ('C', 'N'))),
    ]);

    assert_eq!(expected_str, input_str);
    assert_eq!(expected_rules, rules);
}

#[test]
fn test_step_1() {
    let input: HashMap<(char, char), usize> =
        HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
    let rules = HashMap::from([
        (('C', 'H'), (('C', 'B'), ('B', 'H'))),
        (('H', 'H'), (('H', 'N'), ('N', 'H'))),
        (('C', 'B'), (('C', 'H'), ('H', 'B'))),
        (('N', 'H'), (('N', 'C'), ('C', 'H'))),
        (('H', 'B'), (('H', 'C'), ('C', 'B'))),
        (('H', 'C'), (('H', 'B'), ('B', 'C'))),
        (('H', 'N'), (('H', 'C'), ('C', 'N'))),
        (('N', 'N'), (('N', 'C'), ('C', 'N'))),
        (('B', 'H'), (('B', 'H'), ('H', 'H'))),
        (('N', 'C'), (('N', 'B'), ('B', 'C'))),
        (('N', 'B'), (('N', 'B'), ('B', 'B'))),
        (('B', 'N'), (('B', 'B'), ('B', 'N'))),
        (('B', 'B'), (('B', 'N'), ('N', 'B'))),
        (('B', 'C'), (('B', 'B'), ('B', 'C'))),
        (('C', 'C'), (('C', 'N'), ('N', 'C'))),
        (('C', 'N'), (('C', 'C'), ('C', 'N'))),
    ]);

    let out = step(&input, &rules);
    let expected: HashMap<(char, char), usize> = HashMap::from([
        (('N', 'C'), 1),
        (('C', 'N'), 1),
        (('N', 'B'), 1),
        (('B', 'C'), 1),
        (('C', 'H'), 1),
        (('H', 'B'), 1),
    ]);

    assert_eq!(expected, out);
}

#[test]
fn test_part1_1() {
    let input: HashMap<(char, char), usize> =
        HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
    let rules = HashMap::from([
        (('C', 'H'), (('C', 'B'), ('B', 'H'))),
        (('H', 'H'), (('H', 'N'), ('N', 'H'))),
        (('C', 'B'), (('C', 'H'), ('H', 'B'))),
        (('N', 'H'), (('N', 'C'), ('C', 'H'))),
        (('H', 'B'), (('H', 'C'), ('C', 'B'))),
        (('H', 'C'), (('H', 'B'), ('B', 'C'))),
        (('H', 'N'), (('H', 'C'), ('C', 'N'))),
        (('N', 'N'), (('N', 'C'), ('C', 'N'))),
        (('B', 'H'), (('B', 'H'), ('H', 'H'))),
        (('N', 'C'), (('N', 'B'), ('B', 'C'))),
        (('N', 'B'), (('N', 'B'), ('B', 'B'))),
        (('B', 'N'), (('B', 'B'), ('B', 'N'))),
        (('B', 'B'), (('B', 'N'), ('N', 'B'))),
        (('B', 'C'), (('B', 'B'), ('B', 'C'))),
        (('C', 'C'), (('C', 'N'), ('N', 'C'))),
        (('C', 'N'), (('C', 'C'), ('C', 'N'))),
    ]);

    let got = solve(&input, &rules, 10, 'B');
    assert_eq!(1588, got);
}

#[test]
fn test_part1_2() {
    let input: HashMap<(char, char), usize> =
        HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
    let rules = HashMap::from([
        (('C', 'H'), (('C', 'B'), ('B', 'H'))),
        (('H', 'H'), (('H', 'N'), ('N', 'H'))),
        (('C', 'B'), (('C', 'H'), ('H', 'B'))),
        (('N', 'H'), (('N', 'C'), ('C', 'H'))),
        (('H', 'B'), (('H', 'C'), ('C', 'B'))),
        (('H', 'C'), (('H', 'B'), ('B', 'C'))),
        (('H', 'N'), (('H', 'C'), ('C', 'N'))),
        (('N', 'N'), (('N', 'C'), ('C', 'N'))),
        (('B', 'H'), (('B', 'H'), ('H', 'H'))),
        (('N', 'C'), (('N', 'B'), ('B', 'C'))),
        (('N', 'B'), (('N', 'B'), ('B', 'B'))),
        (('B', 'N'), (('B', 'B'), ('B', 'N'))),
        (('B', 'B'), (('B', 'N'), ('N', 'B'))),
        (('B', 'C'), (('B', 'B'), ('B', 'C'))),
        (('C', 'C'), (('C', 'N'), ('N', 'C'))),
        (('C', 'N'), (('C', 'C'), ('C', 'N'))),
    ]);

    let got = solve(&input, &rules, 2, 'B');
    assert_eq!(5, got);
}

#[test]
fn test_part2() {
    let input: HashMap<(char, char), usize> =
        HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
    let rules = HashMap::from([
        (('C', 'H'), (('C', 'B'), ('B', 'H'))),
        (('H', 'H'), (('H', 'N'), ('N', 'H'))),
        (('C', 'B'), (('C', 'H'), ('H', 'B'))),
        (('N', 'H'), (('N', 'C'), ('C', 'H'))),
        (('H', 'B'), (('H', 'C'), ('C', 'B'))),
        (('H', 'C'), (('H', 'B'), ('B', 'C'))),
        (('H', 'N'), (('H', 'C'), ('C', 'N'))),
        (('N', 'N'), (('N', 'C'), ('C', 'N'))),
        (('B', 'H'), (('B', 'H'), ('H', 'H'))),
        (('N', 'C'), (('N', 'B'), ('B', 'C'))),
        (('N', 'B'), (('N', 'B'), ('B', 'B'))),
        (('B', 'N'), (('B', 'B'), ('B', 'N'))),
        (('B', 'B'), (('B', 'N'), ('N', 'B'))),
        (('B', 'C'), (('B', 'B'), ('B', 'C'))),
        (('C', 'C'), (('C', 'N'), ('N', 'C'))),
        (('C', 'N'), (('C', 'C'), ('C', 'N'))),
    ]);

    let got = solve(&input, &rules, 40, 'B');
    assert_eq!(2_188_189_693_529, got);
}

#[test]
fn test_part1_actual() {
    let input_str =
        std::fs::read_to_string("input/day14.txt").expect("Failed to read day 14 input");
    let (input, rules) = parse_input(&input_str);

    let got = solve(&input, &rules, 10, 'K');
    assert_eq!(2345, got);
}

#[test]
fn test_part2_actual() {
    let input_str =
        std::fs::read_to_string("input/day14.txt").expect("Failed to read day 14 input");
    let (input, rules) = parse_input(&input_str);

    let got = solve(&input, &rules, 40, 'K');
    assert_eq!(2432786807053, got);
}
