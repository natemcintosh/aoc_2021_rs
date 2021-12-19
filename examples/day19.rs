use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Div, Mul, Sub},
};

use itertools::Itertools;

fn parse_input(input: &str) -> HashMap<usize, Vec<Point>> {
    // Split on double newlines
    input
        .split("\n\n")
        .map(|scanner| {
            let mut scanner_lines = scanner.lines();
            // The first line is the scanner number
            let scanner_number = scanner_lines
                .next()
                .expect("Could not get first line containing scanner number")
                // Split on spaces, and grab the item at index 2
                .split_whitespace()
                .nth(2)
                .expect("Could not get scanner number")
                // Parse the scanner number
                .parse::<usize>()
                .expect("Could not parse scanner number");

            // The rest of the lines are Points
            let mut ps = scanner_lines
                // Map each line to a Point
                .map(|line| {
                    let mut nums = line.trim().split(',');
                    // Grab the first item
                    let x: i64 = nums
                        .next()
                        .expect("Could not get x coordinate")
                        .parse()
                        .expect("Could not parse x coordinate");
                    // Grab the second item
                    let y: i64 = nums
                        .next()
                        .expect("Could not get y coordinate")
                        .parse()
                        .expect("Could not parse y coordinate");
                    // Grab the third item
                    let z: i64 = nums
                        .next()
                        .expect("Could not get z coordinate")
                        .parse()
                        .expect("Could not parse z coordinate");
                    // Return the Point
                    Point { 0: x, 1: y, 2: z }
                })
                .collect::<Vec<Point>>();
            ps.sort_unstable();

            (scanner_number, ps)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i64, i64, i64);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
            2: self.2 + other.2,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        }
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            0: self.0 / rhs.0,
            1: self.1 / rhs.1,
            2: self.2 / rhs.2,
        }
    }
}

fn rotate_z_90(p: &Point) -> Point {
    Point {
        0: -p.1,
        1: p.0,
        2: p.2,
    }
}

fn rotate_z_180(p: &Point) -> Point {
    Point {
        0: -p.0,
        1: -p.1,
        2: p.2,
    }
}

fn rotate_z_270(p: &Point) -> Point {
    Point {
        0: p.1,
        1: -p.0,
        2: p.2,
    }
}

fn rotate_y_90(p: &Point) -> Point {
    Point {
        0: p.2,
        1: p.1,
        2: -p.0,
    }
}

fn rotate_y_180(p: &Point) -> Point {
    Point {
        0: -p.0,
        1: p.1,
        2: -p.2,
    }
}

fn rotate_y_270(p: &Point) -> Point {
    Point {
        0: -p.2,
        1: p.1,
        2: p.0,
    }
}

fn rotate_0(p: &Point) -> Point {
    p.clone()
}

fn rotate_x_90(p: &Point) -> Point {
    Point {
        0: p.0,
        1: -p.2,
        2: p.1,
    }
}

fn rotate_x_180(p: &Point) -> Point {
    Point {
        0: p.0,
        1: -p.1,
        2: -p.2,
    }
}

fn rotate_x_270(p: &Point) -> Point {
    Point {
        0: p.0,
        1: p.2,
        2: -p.1,
    }
}

fn offset_if_match(v1: &[Point], v2: &[Point], match_num: usize) -> Option<Point> {
    // Calculate the distance from each pair of points
    v1.iter()
        .zip(v2.iter())
        // Calculate the distance
        .map(|(&p1, &p2)| p1 - p2)
        // How many times do we see each distance?
        .counts()
        // Look for a distance that appears >= match_num times
        .iter()
        .find(|(_, &count)| count >= match_num)
        .map(|(&p, _)| p)
}

fn rotate_and_compare(v1: &[Point], v2: &[Point], match_num: usize) -> Option<Point> {
    // Try each possible set of rotations, and if we find a match, return the
    // offset
    let x_rots = [rotate_0, rotate_x_90, rotate_x_180, rotate_x_270];
    let y_rots = [rotate_0, rotate_y_90, rotate_y_180, rotate_y_270];
    let z_rots = [rotate_0, rotate_z_90, rotate_z_180, rotate_z_270];
    let rotations: Vec<_> = x_rots
        .iter()
        .flat_map(|x_rot| y_rots.iter().map(move |y_rot| (x_rot, y_rot)))
        .flat_map(|(x_rot, y_rot)| z_rots.iter().map(move |z_rot| (x_rot, y_rot, z_rot)))
        .collect();

    for (x_rot, y_rot, z_rot) in rotations {
        let v2_rot: Vec<Point> = v2
            .iter()
            .map(|p| x_rot(p))
            .map(|p| y_rot(&p))
            .map(|p| z_rot(&p))
            .collect();

        if let Some(offset) = offset_if_match(&v1, &v2_rot, match_num) {
            return Some(offset);
        }
    }
    None
}

fn main() {
    println!("Hello, world!");
    let p = Point(1, 2, 3);
    let x_rots = [rotate_x_90, rotate_x_180, rotate_x_270];
    let y_rots = [rotate_y_90, rotate_y_180, rotate_y_270];
    let z_rots = [rotate_z_90, rotate_z_180, rotate_z_270];
    // Create a vector of all possible rotations
    let rotations: Vec<_> = x_rots
        .iter()
        .flat_map(|x_rot| y_rots.iter().map(move |y_rot| (x_rot, y_rot)))
        .flat_map(|(x_rot, y_rot)| z_rots.iter().map(move |z_rot| (x_rot, y_rot, z_rot)))
        .collect();
}

#[test]
fn test_parse_input() {
    let input_str = "--- scanner 0 ---
718,-319,-758
-765,759,419    

--- scanner 1 ---
716,649,-468
523,-512,-705";
    let expected: HashMap<usize, Vec<Point>> = HashMap::from([
        (0, vec![Point(-765, 759, 419), Point(718, -319, -758)]),
        (1, vec![Point(523, -512, -705), Point(716, 649, -468)]),
    ]);
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_offset_if_match_easy() {
    let mut v1 = [Point(0, 2, 0), Point(4, 1, 0), Point(3, 3, 0)];
    v1.sort();
    let mut v2 = [Point(-1, -1, 0), Point(-5, 0, 0), Point(-2, 1, 0)];
    v2.sort();
    let got = offset_if_match(&v1, &v2, 3);
    assert_eq!(Some(Point(5, 2, 0)), got);
}

#[test]
fn test_rotate_and_compare() {
    let input_str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390";

    let input = parse_input(input_str);
    let v1 = input.get(&0).unwrap();
    let v2 = input.get(&1).unwrap();

    let got = rotate_and_compare(&v1, &v2, 12);
    // assert!(matches!(got, Some(_)));
}

#[test]
fn test_rotate_z_90() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_z_90(&p1);
    assert_eq!(p2, Point(-20, 10, 30));
}

#[test]
fn test_rotate_z_180() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_z_180(&p1);
    assert_eq!(p2, Point(-10, -20, 30));
}

#[test]
fn test_rotate_z_270() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_z_270(&p1);
    assert_eq!(p2, Point(20, -10, 30));
}

#[test]
fn test_rotate_y_90() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_y_90(&p1);
    assert_eq!(p2, Point(30, 20, -10));
}

#[test]
fn test_rotate_y_180() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_y_180(&p1);
    assert_eq!(p2, Point(-10, 20, -30));
}

#[test]
fn test_rotate_y_270() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_y_270(&p1);
    assert_eq!(p2, Point(-30, 20, 10));
}

#[test]
fn test_rotate_x_90() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_x_90(&p1);
    assert_eq!(p2, Point(10, -30, 20));
}

#[test]
fn test_rotate_x_180() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_x_180(&p1);
    assert_eq!(p2, Point(10, -20, -30));
}

#[test]
fn test_rotate_x_270() {
    let p1 = Point(10, 20, 30);
    let p2 = rotate_x_270(&p1);
    assert_eq!(p2, Point(10, 30, -20));
}
