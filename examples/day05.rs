use ndarray::{arr2, Array2};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct LineEnds {
    p1: Point,
    p2: Point,
}

impl LineEnds {
    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn rise_and_run(&self) -> (i64, i64) {
        let rise = self.p2.y - self.p1.y;
        let run = self.p2.x - self.p1.x;

        // Make sure we're looking at the smallest possible step sizes
        let d = gcd(rise, run);
        (rise / d, run / d)
    }

    fn points_on_segment(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();
        let mut xpoint = self.p1.x;
        let mut ypoint = self.p1.y;
        let (rise, run) = self.rise_and_run();

        // The number of integer points between the two endpoints is gcd(rise, run) + 1
        let n_points = gcd((self.p1.x - self.p2.x).abs(), (self.p1.y - self.p2.y).abs()) + 1;

        for _ in 0..n_points {
            result.push(Point {
                x: xpoint,
                y: ypoint,
            });

            ypoint += rise;
            xpoint += run;
        }

        result
    }
}

fn parse_input(input: &str) -> Vec<LineEnds> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let p1 = parts.next().unwrap();
            let p2 = parts.next().unwrap();
            let p1 = p1.split(',').collect::<Vec<_>>();
            let p2 = p2.split(',').collect::<Vec<_>>();
            let p1 = Point {
                x: p1[0].parse::<i64>().unwrap(),
                y: p1[1].parse::<i64>().unwrap(),
            };
            let p2 = Point {
                x: p2[0].parse::<i64>().unwrap(),
                y: p2[1].parse::<i64>().unwrap(),
            };
            LineEnds { p1, p2 }
        })
        .collect()
}

fn gcd(a_in: i64, b_in: i64) -> i64 {
    let mut a = a_in.abs();
    let mut b = b_in.abs();
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn count_line_points(input: &[LineEnds]) -> Array2<usize> {
    let points_visited: Vec<_> = input
        .iter()
        .flat_map(|&line| line.points_on_segment())
        .collect();

    let max_x = &points_visited.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = &points_visited.iter().map(|p| p.y).max().unwrap() + 1;
    let mut array = Array2::<usize>::zeros((max_y as usize, max_x as usize));

    // Fill up the array, adding 1 to each point that is visited
    // Note that the x-dimension goes along the columns, and the y-dimension goes along the rows
    for point in points_visited {
        array[[point.y as usize, point.x as usize]] += 1;
    }

    array
}

fn part1(lines: &[LineEnds]) -> usize {
    // Filter out any that are not horizontal or vertical
    let new_lines: Vec<LineEnds> = lines
        .iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .copied()
        .collect::<Vec<_>>();

    count_line_points(&new_lines)
        .iter()
        .filter(|&n| *n >= 2)
        .count()
}

fn part2(lines: &[LineEnds]) -> usize {
    count_line_points(lines).iter().filter(|&n| *n >= 2).count()
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day05.txt").expect("Failed to read day 5 input");
    let lines = parse_input(&input_str);
    println!(
        "Setup took {:.6} µs",
        setup_time.elapsed().as_micros()
    );

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&lines);
    println!(
        "Part 1 took {:.6} µs",
        part1_time.elapsed().as_micros()
    );

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&lines);
    println!(
        "Part 2 took {:.6} µs",
        part2_time.elapsed().as_micros()
    );

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    let expected_input: Vec<LineEnds> = vec![
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 5, y: 9 },
        },
        LineEnds {
            p1: Point { x: 8, y: 0 },
            p2: Point { x: 0, y: 8 },
        },
        LineEnds {
            p1: Point { x: 9, y: 4 },
            p2: Point { x: 3, y: 4 },
        },
        LineEnds {
            p1: Point { x: 2, y: 2 },
            p2: Point { x: 2, y: 1 },
        },
        LineEnds {
            p1: Point { x: 7, y: 0 },
            p2: Point { x: 7, y: 4 },
        },
        LineEnds {
            p1: Point { x: 6, y: 4 },
            p2: Point { x: 2, y: 0 },
        },
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 2, y: 9 },
        },
        LineEnds {
            p1: Point { x: 3, y: 4 },
            p2: Point { x: 1, y: 4 },
        },
        LineEnds {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 8, y: 8 },
        },
        LineEnds {
            p1: Point { x: 5, y: 5 },
            p2: Point { x: 8, y: 2 },
        },
    ];
    let got = parse_input(&input_str);
    assert_eq!(expected_input, got);
}

#[test]
fn test_count_line_points_horizontal_vertical() {
    let input: Vec<LineEnds> = vec![
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 5, y: 9 },
        },
        LineEnds {
            p1: Point { x: 8, y: 0 },
            p2: Point { x: 0, y: 8 },
        },
        LineEnds {
            p1: Point { x: 9, y: 4 },
            p2: Point { x: 3, y: 4 },
        },
        LineEnds {
            p1: Point { x: 2, y: 2 },
            p2: Point { x: 2, y: 1 },
        },
        LineEnds {
            p1: Point { x: 7, y: 0 },
            p2: Point { x: 7, y: 4 },
        },
        LineEnds {
            p1: Point { x: 6, y: 4 },
            p2: Point { x: 2, y: 0 },
        },
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 2, y: 9 },
        },
        LineEnds {
            p1: Point { x: 3, y: 4 },
            p2: Point { x: 1, y: 4 },
        },
        LineEnds {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 8, y: 8 },
        },
        LineEnds {
            p1: Point { x: 5, y: 5 },
            p2: Point { x: 8, y: 2 },
        },
    ];
    let input: Vec<LineEnds> = input
        .iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .map(|&l| l)
        .collect();

    let got = count_line_points(&input);

    let expected_counts: Array2<usize> = arr2(&[
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
    ]);

    assert_eq!(expected_counts, got);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(12, 4), 4);
    assert_eq!(gcd(12, 3), 3);
    assert_eq!(gcd(12, 2), 2);
    assert_eq!(gcd(11, 1), 1);
}

#[test]
fn test_points_on_segment_1() {
    let le = LineEnds {
        p1: Point { x: 0, y: 0 },
        p2: Point { x: 1, y: 1 },
    };
    let expected = vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }];
    let got = le.points_on_segment();
    assert_eq!(expected, got);
}

#[test]
fn test_points_on_segment_2() {
    let le = LineEnds {
        p1: Point { x: 0, y: 0 },
        p2: Point { x: 2, y: 4 },
    };
    let expected = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 2 },
        Point { x: 2, y: 4 },
    ];
    let got = le.points_on_segment();
    assert_eq!(expected, got);
}

#[test]
fn test_points_on_segment_3() {
    let le = LineEnds {
        p1: Point { x: 0, y: -3 },
        p2: Point { x: 2, y: 23 },
    };
    let expected = vec![
        Point { x: 0, y: -3 },
        Point { x: 1, y: 10 },
        Point { x: 2, y: 23 },
    ];
    let got = le.points_on_segment();
    assert_eq!(expected, got);
}

#[test]
fn test_points_on_segment_4() {
    let le = LineEnds {
        p1: Point { x: -2, y: 0 },
        p2: Point { x: 12, y: 7 },
    };
    let expected = vec![
        Point { x: -2, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 2, y: 2 },
        Point { x: 4, y: 3 },
        Point { x: 6, y: 4 },
        Point { x: 8, y: 5 },
        Point { x: 10, y: 6 },
        Point { x: 12, y: 7 },
    ];
    let got = le.points_on_segment();
    assert_eq!(expected, got);
}

#[test]
fn test_points_on_segment_5() {
    let le = LineEnds {
        p1: Point { x: -2, y: 2 },
        p2: Point { x: 0, y: 0 },
    };
    let expected = vec![
        Point { x: -2, y: 2 },
        Point { x: -1, y: 1 },
        Point { x: 0, y: 0 },
    ];
    let got = le.points_on_segment();
    assert_eq!(expected, got);
}

#[test]
fn test_points_on_segment_6() {
    let le = LineEnds {
        p1: Point { x: 2, y: 2 },
        p2: Point { x: 2, y: 1 },
    };
    let expected = vec![Point { x: 2, y: 2 }, Point { x: 2, y: 1 }];
    let got = le.points_on_segment();
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let input: Vec<LineEnds> = vec![
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 5, y: 9 },
        },
        LineEnds {
            p1: Point { x: 8, y: 0 },
            p2: Point { x: 0, y: 8 },
        },
        LineEnds {
            p1: Point { x: 9, y: 4 },
            p2: Point { x: 3, y: 4 },
        },
        LineEnds {
            p1: Point { x: 2, y: 2 },
            p2: Point { x: 2, y: 1 },
        },
        LineEnds {
            p1: Point { x: 7, y: 0 },
            p2: Point { x: 7, y: 4 },
        },
        LineEnds {
            p1: Point { x: 6, y: 4 },
            p2: Point { x: 2, y: 0 },
        },
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 2, y: 9 },
        },
        LineEnds {
            p1: Point { x: 3, y: 4 },
            p2: Point { x: 1, y: 4 },
        },
        LineEnds {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 8, y: 8 },
        },
        LineEnds {
            p1: Point { x: 5, y: 5 },
            p2: Point { x: 8, y: 2 },
        },
    ];
    let expected = 5;
    let got = part1(&input);
    assert_eq!(expected, got);
}

#[test]
fn test_part2() {
    let input: Vec<LineEnds> = vec![
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 5, y: 9 },
        },
        LineEnds {
            p1: Point { x: 8, y: 0 },
            p2: Point { x: 0, y: 8 },
        },
        LineEnds {
            p1: Point { x: 9, y: 4 },
            p2: Point { x: 3, y: 4 },
        },
        LineEnds {
            p1: Point { x: 2, y: 2 },
            p2: Point { x: 2, y: 1 },
        },
        LineEnds {
            p1: Point { x: 7, y: 0 },
            p2: Point { x: 7, y: 4 },
        },
        LineEnds {
            p1: Point { x: 6, y: 4 },
            p2: Point { x: 2, y: 0 },
        },
        LineEnds {
            p1: Point { x: 0, y: 9 },
            p2: Point { x: 2, y: 9 },
        },
        LineEnds {
            p1: Point { x: 3, y: 4 },
            p2: Point { x: 1, y: 4 },
        },
        LineEnds {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 8, y: 8 },
        },
        LineEnds {
            p1: Point { x: 5, y: 5 },
            p2: Point { x: 8, y: 2 },
        },
    ];
    let expected = 12;
    let got = part2(&input);
    assert_eq!(expected, got);
}
