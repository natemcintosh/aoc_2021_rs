use itertools::Itertools;
use std::{collections::HashMap, str};

fn parse_input(input: &str) -> (Vec<(char, char)>, HashMap<&str, (String, String)>) {
    let (start_str, rules) = input
        .split_once("\n\n")
        .expect("Could not split around double newline");

    // Now break up the start_str into pairs of letters
    let pairs: Vec<(char, char)> = start_str.chars().tuple_windows().collect();

    let m = rules
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
            // let new_val: String = [cs[0], char_val, cs[1]].into_iter().collect();
            let left_side: String = [cs[0], char_val].into_iter().collect();
            let right_side: String = [char_val, cs[1]].into_iter().collect();
            (key, (left_side, right_side))
        })
        .collect::<HashMap<&str, (String, String)>>();

    (pairs, m)
}

// fn step(input_str: &mut HashMap<&str, usize>, rules: &HashMap<&str, String>) {
//     // Iterate over the rules. For each one that is found in the input_str, get its
//     // values, and increase them by 1 in the input_str map
// }

// fn part1(input_str: &str, rules: &HashMap<&str, String>, nsteps: usize) -> usize {
//     let mut s = input_str.to_string();
//     for _ in 0..nsteps {
//         step(&s, rules);
//     }

//     let freqs = s.chars().counts();
//     let min_max = freqs.iter().minmax_by_key(|x| x.1);
//     let (min_char, max_char) = match min_max {
//         itertools::MinMaxResult::NoElements => panic!("Could not find min max"),
//         itertools::MinMaxResult::OneElement(_) => panic!("Could only find one element"),
//         itertools::MinMaxResult::MinMax((_, min_char), (_, max_char)) => (min_char, max_char),
//     };
//     max_char - min_char
// }

fn main() {
    let s = "NNCB";
    let s = s.replace("AB", "NCN");
    println!("{}", s);
}

// #[test]
// fn test_parse_input() {
//     let input_str = "NNCB

// CH -> B
// HH -> N
// CB -> H
// NH -> C
// HB -> C
// HC -> B
// HN -> C
// NN -> C
// BH -> H
// NC -> B
// NB -> B
// BN -> B
// BB -> N
// BC -> B
// CC -> N
// CN -> C";

//     let (start_str, rules) = parse_input(input_str);
//     let expected_str = "NNCB";
//     let expected_rules = HashMap::from([
//         ("CH", "CBH".to_string()),
//         ("HH", "HNH".to_string()),
//         ("CB", "CHB".to_string()),
//         ("NH", "NCH".to_string()),
//         ("HB", "HCB".to_string()),
//         ("HC", "HBC".to_string()),
//         ("HN", "HCN".to_string()),
//         ("NN", "NCN".to_string()),
//         ("BH", "BHH".to_string()),
//         ("NC", "NBC".to_string()),
//         ("NB", "NBB".to_string()),
//         ("BN", "BBN".to_string()),
//         ("BB", "BNB".to_string()),
//         ("BC", "BBC".to_string()),
//         ("CC", "CNC".to_string()),
//         ("CN", "CCN".to_string()),
//     ]);

//     assert_eq!(expected_str, start_str);
//     assert_eq!(expected_rules, rules);
// }

// #[test]
// fn test_step_1() {
//     let input_str = "NNCB";
//     let rules = HashMap::from([
//         ("CH", "CBH".to_string()),
//         ("HH", "HNH".to_string()),
//         ("CB", "CHB".to_string()),
//         ("NH", "NCH".to_string()),
//         ("HB", "HCB".to_string()),
//         ("HC", "HBC".to_string()),
//         ("HN", "HCN".to_string()),
//         ("NN", "NCN".to_string()),
//         ("BH", "BHH".to_string()),
//         ("NC", "NBC".to_string()),
//         ("NB", "NBB".to_string()),
//         ("BN", "BBN".to_string()),
//         ("BB", "BNB".to_string()),
//         ("BC", "BBC".to_string()),
//         ("CC", "CNC".to_string()),
//         ("CN", "CCN".to_string()),
//     ]);

//     let got = step(input_str, &rules);
//     assert_eq!("NCNBCHB", got);
// }

// #[test]
// fn test_part1() {
//     let input_str = "NNCB";
//     let rules = HashMap::from([
//         ("CH", "CBH".to_string()),
//         ("HH", "HNH".to_string()),
//         ("CB", "CHB".to_string()),
//         ("NH", "NCH".to_string()),
//         ("HB", "HCB".to_string()),
//         ("HC", "HBC".to_string()),
//         ("HN", "HCN".to_string()),
//         ("NN", "NCN".to_string()),
//         ("BH", "BHH".to_string()),
//         ("NC", "NBC".to_string()),
//         ("NB", "NBB".to_string()),
//         ("BN", "BBN".to_string()),
//         ("BB", "BNB".to_string()),
//         ("BC", "BBC".to_string()),
//         ("CC", "CNC".to_string()),
//         ("CN", "CCN".to_string()),
//     ]);

//     let got = part1(input_str, &rules, 10);
//     assert_eq!(1588, got);
// }
