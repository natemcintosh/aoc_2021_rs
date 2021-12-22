use std::{collections::HashSet, ops::RangeInclusive};

fn parse_input(input: &str) -> Vec<(bool, PointRanges)> {
    input.lines().map(parse_line).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PointRanges {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

impl PointRanges {
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    fn intersection(&self, other: &PointRanges) -> PointRanges {
        let x = range_intersection(&self.x, &other.x);
        let y = range_intersection(&self.y, &other.y);
        let z = range_intersection(&self.z, &other.z);
        PointRanges { x, y, z }
    }

    /// union could/will end up with multiple PointRanges Based on the various cases
    ///  _____
    /// |     |
    /// |     |
    /// |    _|
    /// |___|
    fn union(&self, other: &PointRanges) -> Vec<PointRanges> {
        // Get the union of each direction
        let mut xs = range_union(&self.x, &other.x);
        let mut ys = range_union(&self.y, &other.y);
        let mut zs = range_union(&self.z, &other.z);

        // Remove any empty ranges
        xs.retain(|r| !r.is_empty());
        ys.retain(|r| !r.is_empty());
        zs.retain(|r| !r.is_empty());

        // Have to pair up each Range in x with every range in y and every range in z
        let mut result: Vec<PointRanges> = Vec::new();
        for x in &xs {
            for y in &ys {
                for z in &zs {
                    result.push(PointRanges {
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    })
                }
            }
        }

        result
    }
}

fn parse_line(line: &str) -> (bool, PointRanges) {
    let (on_off, coords) = line
        .trim()
        .split_once(' ')
        .expect("Could not split instruction around space");

    let b = match on_off {
        "on" => true,
        "off" => false,
        s => panic!("Instruction, {}, was not on or off", s),
    };

    let mut pts = coords.split(',');
    let mut x_pts = pts
        .next()
        .expect("Could not get the first set of coordinates")
        .chars();
    let mut y_pts = pts
        .next()
        .expect("Could not get the second set of coordinates")
        .chars();
    let mut z_pts = pts
        .next()
        .expect("Could not get the third set of coordinates")
        .chars();

    // skip while not '=', take while not '.'
    let x_lo: i64 = x_pts
        .by_ref()
        .skip_while(|c| *c != '=')
        .skip(1)
        .take_while(|c| *c != '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse lower x into i64");

    // skip while '.' take the rest
    let x_hi: i64 = x_pts
        .by_ref()
        .skip_while(|c| *c == '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse upper x into i64");

    // skip while not '=', take while not '.'
    let y_lo: i64 = y_pts
        .by_ref()
        .skip_while(|c| *c != '=')
        .skip(1)
        .take_while(|c| *c != '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse lower y into i64");

    // skip while '.' take the rest
    let y_hi: i64 = y_pts
        .by_ref()
        .skip_while(|c| *c == '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse upper y into i64");

    // skip while not '=', take while not '.'
    let z_lo: i64 = z_pts
        .by_ref()
        .skip_while(|c| *c != '=')
        .skip(1)
        .take_while(|c| *c != '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse lower z into i64");

    // skip while '.' take the rest
    let z_hi: i64 = z_pts
        .by_ref()
        .skip_while(|c| *c == '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse upper z into i64");

    let pr = PointRanges {
        x: x_lo..=x_hi,
        y: y_lo..=y_hi,
        z: z_lo..=z_hi,
    };
    (b, pr)
}

fn is_outside_range(p: &PointRanges, low: i64, hi: i64) -> bool {
    (p.x.start() < &low)
        | (p.x.end() > &hi)
        | (p.y.start() < &low)
        | (p.y.end() > &hi)
        | (p.z.start() < &low)
        | (p.z.end() > &hi)
}

fn pointranges_to_hashset(pr: &PointRanges) -> HashSet<(i64, i64, i64)> {
    let cap: usize = (pr.x.end() - pr.x.start()) as usize
        * (pr.y.end() - pr.y.start()) as usize
        * (pr.z.end() - pr.z.start()) as usize;
    let mut result = HashSet::with_capacity(cap);
    for x in pr.x.clone() {
        for y in pr.y.clone() {
            for z in pr.z.clone() {
                result.insert((x, y, z));
            }
        }
    }
    result
}

fn part1(cubes: &[(bool, PointRanges)]) -> usize {
    cubes
        .iter()
        // Filter out any that exist outside of -50..50 (inclusive)
        .filter(|(_, pr)| !is_outside_range(pr, -50, 50))
        // Create hashsets of each
        .map(|(b, pr)| (b, pointranges_to_hashset(pr)))
        // Fold into a single hashset
        .reduce(|(_, acc), (b, hs)| {
            if *b {
                // Get the union
                let new_hs = acc.union(&hs).copied().collect::<HashSet<_>>();
                (b, new_hs)
            } else {
                // Get the difference
                let new_hs = acc.difference(&hs).copied().collect::<HashSet<_>>();
                (b, new_hs)
            }
        })
        .expect("Apprently the iterator was empty")
        .1
        .len()
}

/// range_union is commutative
fn range_union(x: &RangeInclusive<i64>, y: &RangeInclusive<i64>) -> Vec<RangeInclusive<i64>> {
    // Following cases
    // 1. One is empty -> return the non-empty
    // 2. Both are empty -> return an empty range
    // 3. No overlap -> just return the two inputs as a vec
    // 4. Any overlap -> return a RangeInclusive from the min of the two to the max of the two
    // 5. The ends are contiguous, e.g. 0..=10, 11..=15 -> return 0..=15

    // 1 and 2
    if x.is_empty() && !y.is_empty() {
        return vec![y.clone()];
    } else if y.is_empty() && !x.is_empty() {
        return vec![x.clone()];
    } else if x.is_empty() && y.is_empty() {
        return vec![y.clone()];
    }

    // 3 and 5
    if x.end() < y.start() {
        if y.start() - x.end() == 1 {
            return vec![*x.start()..=*y.end()];
        } else {
            return vec![x.clone(), y.clone()];
        }
    } else if y.end() < x.start() {
        if x.start() - y.end() == 1 {
            return vec![*y.start()..=*x.end()];
        } else {
            return vec![x.clone(), y.clone()];
        }
    }

    // 4. Any overlap -> return a RangeInclusive from the min of the two to the max of the two
    let start = x.start().min(y.start());
    let end = x.end().max(y.end());
    return vec![*start..=*end];
}

/// range_intersection calculates the overlap of two Ranges. If there is no overlap, it returns an empty range
/// range_intersection is commutative
fn range_intersection(x: &RangeInclusive<i64>, y: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    // If either is empty, then the inersection is empty
    if x.is_empty() {
        return x.clone();
    } else if y.is_empty() {
        return y.clone();
    }

    // If there is no overlap, returns an empty range (make sure to check for it)
    if (x.end() < y.start()) || (y.end() < x.start()) {
        return *y.end()..=*y.start();
    }

    // Otherwise, get the start and end of the new range
    let start = x.start().max(y.start());
    let end = x.end().min(y.end());
    *start..=*end
}

/// range_difference is not commutative
fn range_difference(x: &RangeInclusive<i64>, y: &RangeInclusive<i64>) -> Vec<RangeInclusive<i64>> {
    // Following cases:
    // 1. If x is empty, return an empty range
    // 2. If y is empty, return x
    // 3. No intersection of x and y -> x
    // 4. y is part in/part out of x, e.g. x=0..=10, y=5..=15 -> vec[0..=4]
    // 5. y is entirely within x, e.g. x=0..=10, y=3..=7 -> vec![0..=2, 8..=10]
    // 6. y overlaps with all of x -> vec![10..=5] (i.e. an empty range)

    // 1.
    if x.is_empty() {
        return vec![x.clone()];
    }

    // 2.
    if y.is_empty() {
        return vec![y.clone()];
    }

    // 3.
    if range_intersection(&x, &y).is_empty() {
        return vec![x.clone()];
    }

    // 6.
    if y.contains(x.start()) && y.contains(x.end()) {
        return vec![10..=5];
    }

    // 4.
    if (x.contains(y.start()) && !x.contains(y.end()))
        || (!x.contains(y.start()) && x.contains(y.end()))
    {
        if x.start() < y.start() {
            return vec![*x.start()..=(y.start() - 1)];
        } else if y.start() < x.start() {
            return vec![(y.end() + 1)..=*x.end()];
        } else {
            panic!("Should not have arrived here");
        }
    }

    // 5.
    if x.contains(y.start()) && x.contains(y.end()) {
        return vec![*x.start()..=(y.start() - 1), (y.end() + 1)..=*x.end()];
    }

    panic!("Should have covered all cases")
}

fn reduce_ranges(ranges: &[(bool, RangeInclusive<i64>)]) -> Vec<RangeInclusive<i64>> {
    let mut new_ranges: Vec<RangeInclusive<i64>> = vec![ranges[0].1.clone()];
    for (b, this_r) in &ranges[1..] {
        if *b {
            // Get the union of this range with existing ranges
            new_ranges = new_ranges
                .iter()
                .flat_map(|existing_r| range_union(this_r, existing_r))
                .collect();
        } else {
            // Get the difference of existing ranges with this range
            new_ranges = new_ranges
                .iter()
                .flat_map(|existing_r| range_difference(existing_r, this_r))
                .collect();
        }
    }

    // Perform one final union of the ranges
    new_ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()));
    let mut final_ranges = vec![new_ranges[0].clone()];
    for r in &new_ranges[1..] {
        // If end of the current last is before the start of the next
        if final_ranges[final_ranges.len() - 1].end() < r.start() {
            final_ranges.push(r.clone());
        } else if final_ranges[final_ranges.len() - 1].end() >= r.start() {
            // If the end of the current last is >= the start of the next
            let new_end = *final_ranges[final_ranges.len() - 1].start()..=*r.end();
            final_ranges.pop();
            final_ranges.push(new_end);
        }
    }
    final_ranges
}

fn solve(
    xs: &[(bool, RangeInclusive<i64>)],
    ys: &[(bool, RangeInclusive<i64>)],
    zs: &[(bool, RangeInclusive<i64>)],
) {
    // Reduce all of the xs, all of the ys, and all of the zs.
    let final_xs = reduce_ranges(xs);
    let final_ys = reduce_ranges(ys);
    let final_zs = reduce_ranges(zs);
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day22.txt").expect("Failed to read day 22 input");
    let cubes = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&cubes);
    println!("Part 1 took {:.6} ms", part1_time.elapsed().as_millis());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = solve(&input_image, &algo, 50);
    // println!("Part 2 took {:.6} ms", part2_time.elapsed().as_millis());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    let expected: Vec<(bool, PointRanges)> = vec![
        (
            true,
            PointRanges {
                x: 10..=12,
                y: 10..=12,
                z: 10..=12,
            },
        ),
        (
            true,
            PointRanges {
                x: 11..=13,
                y: 11..=13,
                z: 11..=13,
            },
        ),
        (
            false,
            PointRanges {
                x: 9..=11,
                y: 9..=11,
                z: 9..=11,
            },
        ),
        (
            true,
            PointRanges {
                x: 10..=10,
                y: 10..=10,
                z: 10..=10,
            },
        ),
    ];

    let got = parse_input(input_str);

    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let input_str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    let v = parse_input(input_str);
    let got = part1(&v);
    assert_eq!(39, got);
}

#[test]
fn test_range_union_some_overlap() {
    let x = 0..=10;
    let y = 5..=15;
    let got = range_union(&x, &y);
    assert_eq!(vec![0..=15], got);
}

#[test]
fn test_range_union_ends_contiguous() {
    let x = 0..=10;
    let y = 11..=15;
    let got = range_union(&x, &y);
    assert_eq!(vec![0..=15], got);
}

#[test]
fn test_range_union_no_overlap() {
    let x = 0..=10;
    let y = 12..=15;
    let got = range_union(&x, &y);
    assert_eq!(vec![0..=10, 12..=15], got);
}

#[test]
fn test_range_union_one_inside_the_other() {
    let x = 0..=10;
    let y = 3..=8;
    let got = range_union(&x, &y);
    assert_eq!(vec![0..=10], got);
}

#[test]
fn test_range_intersection_one_in_the_other() {
    let x = -10..=10;
    let y = -3..=8;
    let got = range_intersection(&x, &y);
    assert_eq!(-3..=8, got);
}

#[test]
fn test_range_intersection_no_intersection() {
    let x = 0..=10;
    let y = 12..=15;
    let got = range_intersection(&x, &y);
    assert!(got.is_empty());
}

#[test]
fn test_range_intersection_no_intersection_2() {
    let x = 0..=10;
    let y = 11..=15;
    let got = range_intersection(&x, &y);
    assert!(got.is_empty());
}

#[test]
fn test_range_intersection_some_intersection() {
    let x = 0..=10;
    let y = 5..=15;
    let got = range_intersection(&x, &y);
    assert_eq!(5..=10, got);
}

#[test]
fn test_range_intersection_one_intersection() {
    let x = 0..=5;
    let y = 5..=15;
    let got = range_intersection(&x, &y);
    assert_eq!(5..=5, got);
}

#[test]
fn test_range_difference_no_intersection() {
    let x = 0..=10;
    let y = 11..=15;
    let got = range_difference(&x, &y);
    assert_eq!(vec![x.clone()], got);
}

#[test]
fn test_range_difference_x_entirely_in_y() {
    let x = 0..=10;
    let y = -5..=15;
    let got = range_difference(&x, &y);
    assert!(got[0].is_empty());
}

#[test]
fn test_range_difference_some_overlap_1() {
    let x = -3..=10;
    let y = 5..=15;
    let got = range_difference(&x, &y);
    assert_eq!(vec![-3..=4], got);
}

#[test]
fn test_range_difference_some_overlap_2() {
    let x = -3..=10;
    let y = -5..=-2;
    let got = range_difference(&x, &y);
    assert_eq!(vec![-1..=10], got);
}

#[test]
fn test_range_difference_y_entirely_in_x() {
    let x = -30..=10;
    let y = -5..=-2;
    let got = range_difference(&x, &y);
    assert_eq!(vec![-30..=-6, -1..=10], got);
}

#[test]
fn test_reduce_ranges_1() {
    let ranges: Vec<(bool, RangeInclusive<i64>)> = vec![
        (true, 10..=12),
        (true, 11..=13),
        (false, 9..=11),
        (true, 10..=10),
    ];
    let expected: Vec<RangeInclusive<i64>> = vec![10..=10, 12..=13];
    let got = reduce_ranges(&ranges);
    assert_eq!(expected, got);
}

#[test]
fn test_reduce_ranges_2() {
    let ranges: Vec<(bool, RangeInclusive<i64>)> =
        vec![(true, -20..=12), (false, -1..=5), (true, 10..=20)];
    let expected: Vec<RangeInclusive<i64>> = vec![-20..=-2, 6..=20];
    let got = reduce_ranges(&ranges);
    assert_eq!(expected, got);
}
