use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SingleSfn {
    Number(usize),
    Another(Box<Sfn>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sfn(SingleSfn, SingleSfn);
// struct Sfn {
//     left: SingleSfn,
//     right: SingleSfn,
//     depth: usize,
// }

fn parse_stream<T: Iterator<Item = char>>(so_far: Option<Sfn>, char_stream: &mut T) -> Option<Sfn> {
    // assert!(
    //     !letters.next().expect("Nothing in this string").ne(&'['),
    //     "Snail fish number did not start with '['"
    // );

    // Get the next character and decide what to do
    let nc = char_stream.next().expect("No character to peek at");
    let mut possible_left_1: Option<SingleSfn> = None;
    let mut possible_left_2: Option<Sfn> = None;
    if nc.is_ascii_digit() {
        possible_left_1 = Some(SingleSfn::Number(
            nc.to_digit(10)
                .expect("Could not parse number")
                .try_into()
                .unwrap(),
        ));
    } else if nc.eq(&'[') {
        possible_left_2 = parse_stream(so_far.clone(), char_stream);
    } else {
        panic!("Next character was not '[' or a number");
    }

    let left = if let Some(n) = possible_left_1 {
        n
    } else if let Some(n) = possible_left_2 {
        SingleSfn::Another(Box::new(n))
    } else {
        panic!("Foiled again")
    };

    // Get the comma
    char_stream.next();

    let nc = char_stream.next().expect("No character to peek at");
    let mut possible_right_1: Option<SingleSfn> = None;
    let mut possible_right_2: Option<Sfn> = None;
    if nc.is_ascii_digit() {
        possible_right_1 = Some(SingleSfn::Number(
            nc.to_digit(10)
                .expect("Could not parse number")
                .try_into()
                .unwrap(),
        ));
    } else if nc.eq(&'[') {
        possible_right_2 = parse_stream(so_far, char_stream);
    } else {
        dbg!(nc);
        panic!("Next character was not '[' or a number");
    }

    let right = if let Some(n) = possible_right_1 {
        n
    } else if let Some(n) = possible_right_2 {
        SingleSfn::Another(Box::new(n))
    } else {
        panic!("Foiled again")
    };

    let sfn = Some(Sfn(left, right));

    // Consume the ']' character
    char_stream.next();

    sfn
}

impl From<&str> for Sfn {
    fn from(s: &str) -> Self {
        let mut char_stream = s.chars();
        // Advance it and consume the first '[' character
        char_stream.next();
        let sfn = parse_stream(None, &mut char_stream);
        sfn.expect("Did not successfully parse snail fish number")
    }
}

impl Display for Sfn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        match self.0 {
            SingleSfn::Number(n) => write!(f, "{}", n)?,
            SingleSfn::Another(ref sfn) => sfn.fmt(f)?,
        }
        write!(f, ",")?;
        match self.1 {
            SingleSfn::Number(n) => write!(f, "{}", n)?,
            SingleSfn::Another(ref sfn) => sfn.fmt(f)?,
        }
        write!(f, "]")?;
        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Sfn> {
    input.lines().map(Sfn::from).collect()
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day18.txt").expect("Failed to read day 18 input");
    let sfns = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    // let part1_result = part1(&area);
    println!("Part 1 took {:.6} ns", part1_time.elapsed().as_nanos());

    // Part 2
    let part2_time = std::time::Instant::now();
    // let part2_result = get_all_possible_vels(&area).count();
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    // println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_sfn_from_1() {
        let s = "[1,2]";
        let want = Sfn(SingleSfn::Number(1), SingleSfn::Number(2));
        let got = Sfn::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_construct_sfn_from_2() {
        let s = "[[1,2],3]";
        let want = Sfn(
            SingleSfn::Another(Box::new(Sfn(SingleSfn::Number(1), SingleSfn::Number(2)))),
            SingleSfn::Number(3),
        );
        let got = Sfn::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_construct_sfn_from_3() {
        let s = "[9,[8,7]]";
        let want = Sfn(
            SingleSfn::Number(9),
            SingleSfn::Another(Box::new(Sfn(SingleSfn::Number(8), SingleSfn::Number(7)))),
        );
        let got = Sfn::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_construct_sfn_from_4() {
        let s = "[[1,9],[8,5]]";
        let want = Sfn(
            SingleSfn::Another(Box::new(Sfn(SingleSfn::Number(1), SingleSfn::Number(9)))),
            SingleSfn::Another(Box::new(Sfn(SingleSfn::Number(8), SingleSfn::Number(5)))),
        );
        let got = Sfn::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_construct_sfn_from_5() {
        let s = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let p1 = Sfn(SingleSfn::Number(1), SingleSfn::Number(2));
        let p2 = Sfn(SingleSfn::Number(3), SingleSfn::Number(4));
        let p3 = Sfn(SingleSfn::Number(5), SingleSfn::Number(6));
        let p4 = Sfn(SingleSfn::Number(7), SingleSfn::Number(8));
        let double1 = Sfn(
            SingleSfn::Another(Box::new(p1)),
            SingleSfn::Another(Box::new(p2)),
        );
        let double2 = Sfn(
            SingleSfn::Another(Box::new(p3)),
            SingleSfn::Another(Box::new(p4)),
        );
        let double3 = Sfn(
            SingleSfn::Another(Box::new(double1)),
            SingleSfn::Another(Box::new(double2)),
        );
        let want = Sfn(SingleSfn::Another(Box::new(double3)), SingleSfn::Number(9));

        let got = Sfn::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_fmt_1() {
        let strs = vec![
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        ];

        // For each of the above, turn it into a number, and then back into a string,
        // and assert that it's identical to the original
        for s in strs {
            let sfn = Sfn::from(s);
            let got = sfn.to_string();
            dbg!(&got);
            assert_eq!(s.to_owned(), got);
        }
    }

    #[test]
    fn test_fmt_2() {
        let strs = vec![
            "[[[4,5],[[0,6],[4,5]]],[3,[[5,0],[0,8]]]]",
            "[[8,3],2]",
            "[[4,[7,[5,6]]],[[[7,8],5],[[7,0],1]]]",
            "[[[1,8],[7,6]],[[8,6],[3,2]]]",
            "[[[4,[2,0]],[1,[7,0]]],9]",
            "[2,[[[2,3],5],[6,5]]]",
            "[9,[1,[0,3]]]",
            "[5,[5,[8,[8,4]]]]",
            "[5,[1,[4,[0,8]]]]",
            "[1,[[[6,1],9],2]]",
            "[7,[[6,1],[[7,8],[4,2]]]]",
            "[[[[6,6],[3,3]],[6,[7,6]]],4]",
            "[[[3,[9,8]],[[6,6],[9,3]]],[[[9,2],3],[[7,6],0]]]",
            "[[[[5,2],6],[9,[1,7]]],[[9,9],[9,[4,3]]]]",
            "[[[7,6],[9,5]],[[[6,3],[8,4]],[[4,0],8]]]",
            "[[[0,[1,9]],[8,[4,4]]],1]",
            "[[1,[1,[9,4]]],[5,[[9,3],9]]]",
            "[[[1,3],[[2,3],9]],[7,9]]",
            "[[8,[[6,9],[5,9]]],[5,[5,[9,4]]]]",
            "[[[[3,7],[8,0]],[4,[8,9]]],[[[3,8],[3,5]],[9,0]]]",
            "[[[0,5],[5,1]],[3,[0,[0,5]]]]",
            "[7,[[4,[1,6]],0]]",
            "[[3,[4,4]],[[[0,5],9],[8,[9,5]]]]",
            "[[8,[5,2]],[[[7,4],[3,2]],4]]",
            "[[[[6,4],[7,9]],5],[3,[[4,3],[4,3]]]]",
            "[[[[7,0],6],[6,7]],[[[9,7],[3,7]],[[4,1],[0,6]]]]",
            "[[6,[[1,0],[1,7]]],[3,[3,0]]]",
            "[[[2,[6,0]],4],[[3,9],[4,1]]]",
            "[[[0,[8,4]],[[8,7],5]],[1,6]]",
            "[[[[4,0],7],9],[6,[8,[9,3]]]]",
            "[[[[0,8],7],[5,[4,0]]],[5,[6,[8,7]]]]",
            "[[[1,4],[[9,7],4]],[[4,[6,4]],1]]",
            "[[5,[[8,6],9]],1]",
            "[[[[5,7],[8,3]],[[3,2],[1,9]]],[2,[1,2]]]",
            "[[[9,6],[1,5]],[8,6]]",
            "[3,1]",
            "[[2,[[2,0],4]],[[[3,4],1],3]]",
            "[[[[8,6],[5,9]],7],2]",
            "[[[[1,0],[8,5]],[[6,5],[0,0]]],[[[3,4],[4,6]],[[5,0],8]]]",
            "[[[[6,4],[9,4]],[[2,1],[2,2]]],[[[7,9],1],[[6,1],5]]]",
            "[2,[[4,4],5]]",
            "[[[[0,8],9],[8,6]],[[[9,7],[0,8]],[[9,3],7]]]",
            "[[[[2,0],[7,8]],[[8,5],[6,8]]],[7,[[1,1],[2,3]]]]",
            "[[9,[5,[4,7]]],[0,[9,2]]]",
            "[5,[[[7,5],3],[6,[5,3]]]]",
            "[[1,[5,1]],[[[0,3],[3,9]],3]]",
            "[7,[[0,[0,1]],[1,2]]]",
            "[[4,[8,0]],[3,[[2,4],7]]]",
            "[8,[[1,[8,9]],[0,0]]]",
            "[0,[[2,9],[[9,7],[5,3]]]]",
            "[[[6,[3,4]],[[0,6],[4,3]]],9]",
            "[[[[0,6],6],6],[[7,8],[[7,3],[5,0]]]]",
            "[[[7,[4,5]],[9,2]],[6,[[5,5],[0,2]]]]",
            "[[[6,8],[5,[0,8]]],[[1,[6,6]],[0,6]]]",
            "[[[[4,7],7],[2,7]],[[8,0],[[6,5],[2,0]]]]",
            "[8,[[4,9],[[8,8],2]]]",
            "[2,[[4,[5,8]],[[8,7],[0,9]]]]",
            "[[[[2,8],0],6],[[[4,4],0],[1,3]]]",
            "[1,[[[8,5],1],8]]",
            "[[3,3],[[[5,6],[6,2]],5]]",
            "[[9,2],[3,[[3,2],4]]]",
            "[[[[2,4],[6,3]],[[4,6],4]],[[[1,9],[0,4]],[[2,6],[9,0]]]]",
            "[[[4,[6,7]],[[8,4],[6,2]]],[[5,2],[[4,8],0]]]",
            "[[[6,0],[[3,2],5]],[[[9,0],[7,0]],5]]",
            "[[2,[9,3]],[[4,[4,6]],[9,6]]]",
            "[[3,[3,6]],[[[2,4],1],[9,[7,7]]]]",
            "[4,[1,[[3,6],[4,1]]]]",
            "[[3,7],[[5,6],6]]",
            "[[[0,8],4],[[3,5],[[6,2],6]]]",
            "[[[6,[8,9]],[5,[2,4]]],[4,[3,4]]]",
            "[5,[[[6,8],[5,7]],[5,[9,9]]]]",
            "[[[[9,5],6],3],[[[8,2],4],[1,8]]]",
            "[[9,[9,3]],[[[5,7],0],[[5,4],[7,4]]]]",
            "[[[[7,7],7],6],9]",
            "[[9,8],[2,[7,7]]]",
            "[[[[5,9],6],[8,[9,2]]],[[[8,5],[9,5]],[3,[8,3]]]]",
            "[[[4,[3,8]],[8,[4,3]]],[[0,5],[5,[4,5]]]]",
            "[[[0,5],[[7,7],5]],[[[2,7],[6,0]],[[7,9],[2,2]]]]",
            "[6,[2,8]]",
            "[[[2,7],7],[[[8,4],[3,9]],1]]",
            "[[[2,0],[[0,5],[9,4]]],[[7,[6,2]],9]]",
            "[[1,[[8,3],[3,4]]],1]",
            "[[[[2,0],9],3],[1,[7,[2,1]]]]",
            "[4,[[6,[5,7]],[[1,1],[0,5]]]]",
            "[[[6,[0,7]],[4,[8,6]]],3]",
            "[[[8,5],6],[1,[[6,0],4]]]",
            "[[[[6,5],[5,6]],[[0,1],[2,7]]],[[7,[7,6]],[[3,2],[4,0]]]]",
            "[[[5,[0,0]],0],5]",
            "[[[[7,2],[5,9]],2],[3,7]]",
            "[7,[[[1,1],4],[[4,4],2]]]",
            "[9,[[[9,1],1],3]]",
            "[[[[6,9],[3,9]],[7,[1,5]]],[[[5,0],6],[[5,9],8]]]",
            "[[7,[1,[2,1]]],[7,[[6,3],[7,1]]]]",
            "[3,[0,[1,3]]]",
            "[9,[[[6,6],6],[6,4]]]",
            "[[[2,[0,4]],1],[[9,[5,1]],[[9,6],[5,2]]]]",
            "[[[9,8],6],[0,[6,[0,5]]]]",
            "[[[7,3],[[9,9],0]],7]",
            "[[[7,5],[6,8]],[6,[[0,8],9]]]",
            "[[[2,[0,5]],[[2,9],[5,7]]],7]",
        ];

        // For each of the above, turn it into a number, and then back into a string,
        // and assert that it's identical to the original
        for s in strs {
            let sfn = Sfn::from(s);
            let got = sfn.to_string();
            dbg!(&got);
            assert_eq!(s.to_owned(), got);
        }
    }
}
