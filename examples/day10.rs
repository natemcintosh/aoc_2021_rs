#[derive(Debug, PartialEq)]
enum ParseResult<'a> {
    Corrupted(char),
    Incomplete(Vec<char>),
    Ok(&'a str),
}

fn parse_line(s: &str) -> ParseResult {
    // An array of opening chars
    const OPENERS: [char; 4] = ['(', '[', '{', '<'];
    const CLOSERS: [char; 4] = [')', ']', '}', '>'];

    let mut stack: Vec<char> = Vec::with_capacity(20);

    // Iterate over all the inputs
    for c in s.trim().chars() {
        // For each opening bracket, push it's opposite onto the stack
        if OPENERS.contains(&c) {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => unreachable!(),
            }
        } else if CLOSERS.contains(&c) {
            // `c` needs to match the top of the stack, otherwise it is Corrupt
            let required_closer = stack.last().expect("A closer with no matching opener");
            if c == *required_closer {
                stack.pop();
            } else {
                return ParseResult::Corrupted(c);
            }
        } else {
            panic!("Got {} which is not an opener or closer", c);
        }
    }
    if stack.is_empty() {
        return ParseResult::Ok(s);
    }

    stack.reverse();
    ParseResult::Incomplete(stack)
}

fn part1(input: &[ParseResult]) -> usize {
    input
        .iter()
        // Get only the corrupted input characters
        .filter_map(|r| match r {
            ParseResult::Corrupted(c) => Some(c),
            _ => None,
        })
        // Get the score for each character
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn part2(input: &[ParseResult]) -> usize {
    let mut completion_scores: Vec<usize> = input
        // Get just the incomplete lines
        .iter()
        .filter_map(|r| match r {
            ParseResult::Incomplete(v) => Some(v),
            _ => None,
        })
        // For each set of characters, calculate the score
        .map(|v| {
            v.iter().fold(0_usize, |acc, c| {
                (acc * 5)
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();

    // Sort the scores
    completion_scores.sort_unstable();

    // Return the middle value
    let mid_idx = (completion_scores.len() - 1) / 2;
    completion_scores[mid_idx]
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day10.txt").expect("Failed to read day 10 input");
    let input: Vec<ParseResult> = input_str.lines().map(parse_line).collect();
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 took {:.6} ns", part1_time.elapsed().as_nanos());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&input);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_valid_1() {
    let input = "()";
    let got = parse_line(input);
    assert_eq!(ParseResult::Ok("()"), got);
}

#[test]
fn test_valid_2() {
    let input = "([])";
    let got = parse_line(input);
    assert_eq!(ParseResult::Ok("([])"), got);
}

#[test]
fn test_valid_3() {
    let input = "{()()()}";
    let got = parse_line(input);
    assert_eq!(ParseResult::Ok("{()()()}"), got);
}

#[test]
fn test_valid_4() {
    let input = "[<>({}){}[([])<>]]";
    let got = parse_line(input);
    assert_eq!(ParseResult::Ok("[<>({}){}[([])<>]]"), got);
}

#[test]
fn test_corrupted_1() {
    let input = "(]";
    let got = parse_line(input);
    assert_eq!(ParseResult::Corrupted(']'), got);
}

#[test]
fn test_corrupted_2() {
    let input = "{()()()>";
    let got = parse_line(input);
    assert_eq!(ParseResult::Corrupted('>'), got);
}

#[test]
fn test_corrupted_3() {
    let input = "(((()))}";
    let got = parse_line(input);
    assert_eq!(ParseResult::Corrupted('}'), got);
}

#[test]
fn test_incomplete_1() {
    let input = "[({(<(())[]>[[{[]{<()<>>";
    let got = parse_line(input);
    assert!(std::matches!(got, ParseResult::Incomplete(_)));
}

#[test]
fn test_incomplete_2() {
    let input = "[(()[<>])]({[<{<<[]>>(";
    let got = parse_line(input);
    assert!(std::matches!(got, ParseResult::Incomplete(_)));
}

#[test]
fn test_incomplete_3() {
    let input = "(((({<>}<{<{<>}{[]{[]{}";
    let got = parse_line(input);
    assert!(std::matches!(got, ParseResult::Incomplete(_)));
}

#[test]
fn test_incomplete_4() {
    let input = "{<[[]]>}<{[{[{[]{()[[[]";
    let got = parse_line(input);
    assert!(std::matches!(got, ParseResult::Incomplete(_)));
}

#[test]
fn test_part1() {
    let input_str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    let input: Vec<ParseResult> = input_str.lines().map(parse_line).collect();
    let got = part1(&input);
    assert_eq!(26397, got);
}

#[test]
fn test_part1_actual() {
    let input_str =
        std::fs::read_to_string("input/day10.txt").expect("Failed to read day 10 input");
    let input: Vec<ParseResult> = input_str.lines().map(parse_line).collect();
    let got = part1(&input);
    assert_eq!(319329, got);
}

#[test]
fn test_incomplete_stack_1() {
    let input = "[({(<(())[]>[[{[]{<()<>>";
    let got = parse_line(input);
    let v: Vec<char> = vec!['}', '}', ']', ']', ')', '}', ')', ']'];
    assert_eq!(ParseResult::Incomplete(v), got);
}

#[test]
fn test_incomplete_stack_2() {
    let input = "[(()[<>])]({[<{<<[]>>(";
    let got = parse_line(input);
    let v: Vec<char> = vec![')', '}', '>', ']', '}', ')'];
    assert_eq!(ParseResult::Incomplete(v), got);
}

#[test]
fn test_incomplete_stack_3() {
    let input = "(((({<>}<{<{<>}{[]{[]{}";
    let got = parse_line(input);
    let v: Vec<char> = vec!['}', '}', '>', '}', '>', ')', ')', ')', ')'];
    assert_eq!(ParseResult::Incomplete(v), got);
}

#[test]
fn test_incomplete_stack_4() {
    let input = "{<[[]]>}<{[{[{[]{()[[[]";
    let got = parse_line(input);
    let v: Vec<char> = vec![']', ']', '}', '}', ']', '}', ']', '}', '>'];
    assert_eq!(ParseResult::Incomplete(v), got);
}

#[test]
fn test_incomplete_stack_5() {
    let input = "<{([{{}}[<[[[<>{}]]]>[]]";
    let got = parse_line(input);
    let v: Vec<char> = vec![']', ')', '}', '>'];
    assert_eq!(ParseResult::Incomplete(v), got);
}

#[test]
fn test_part2() {
    let input_str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";
    let input: Vec<ParseResult> = input_str.lines().map(parse_line).collect();
    let got = part2(&input);
    assert_eq!(288957, got);
}

#[test]
fn test_part2_actual() {
    let input_str =
        std::fs::read_to_string("input/day10.txt").expect("Failed to read day 10 input");
    let input: Vec<ParseResult> = input_str.lines().map(parse_line).collect();
    let got = part2(&input);
    assert_eq!(3515583998, got);
}
