use itertools::Itertools;

fn parse_input(input: &str) -> Area {
    // The input is
    // target area: x=175..227, y=-134..-79
    let mut chars = input.chars().skip_while(|c| *c != '=').skip(1);

    // For x_low, split once on the first '=', then take while not '.'
    let x_low: i64 = chars
        .by_ref()
        .take_while(|c| *c != '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse x_low");

    // For x_high, drop while '.', then take while not ','
    let x_high: i64 = chars
        .by_ref()
        .skip_while(|c| *c != '.')
        .skip(1)
        .take_while(|c| *c != ',')
        .collect::<String>()
        .parse()
        .expect("Could not parse x_high");

    // For y_low, drop while not '=', then take while not '.'
    let y_low: i64 = chars
        .by_ref()
        .skip_while(|c| *c != '=')
        .skip(1)
        .take_while(|c| *c != '.')
        .collect::<String>()
        .parse()
        .expect("Could not parse y_low");

    // For y_high, drop while '.', then take while not end of line
    let y_high: i64 = chars
        .by_ref()
        .skip_while(|c| *c != '.')
        .skip(1)
        .take_while(|c| *c != '\n')
        .collect::<String>()
        .parse()
        .expect("Could not parse y_high");

    Area {
        x_low,
        x_high,
        y_low,
        y_high,
    }
}

#[derive(Clone, Copy, Debug)]
struct YState {
    pos: i64,
    vel: i64,
    target_bottom: i64,
}

impl Iterator for YState {
    type Item = i64;

    // Will always return the next Y position
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.target_bottom {
            self.pos += self.vel;
            self.vel -= 1;
            Some(self.pos)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct XState {
    pos: i64,
    vel: i64,
    target_max_range: i64,
}

impl Iterator for XState {
    type Item = i64;

    // Will always return the next X position
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos <= self.target_max_range {
            self.pos += self.vel;
            self.vel -= self.vel.signum();
            Some(self.pos)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ProbeState {
    x: XState,
    y: YState,
}

impl Iterator for ProbeState {
    type Item = (i64, i64);

    // Will always return the next X, Y position
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.x.next() {
            self.y.next().map(|y| (x, y))
        } else {
            None
        }
    }
}

fn sum_to_n(n: i64) -> i64 {
    ((n + 1) * n) / 2
}

#[derive(Debug, PartialEq, Eq)]
struct Area {
    x_low: i64,
    x_high: i64,
    y_low: i64,
    y_high: i64,
}

impl Area {
    fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.x_low && x <= self.x_high && y >= self.y_low && y <= self.y_high
    }
}

impl ProbeState {
    fn will_hit_area(&self, area: &Area) -> bool {
        // If we are going in the wrong x direction, we will never hit the area
        if self.x.vel.signum() != area.x_high.signum() {
            return false;
        }

        // Quickly check that it has enough x velocity to reach the target
        if sum_to_n(self.x.vel) < area.x_low {
            return false;
        }

        // Iterate over the ProbePos and check if the position is in the area.
        // Iterator will stop once we have gone past the area.
        for (x, y) in self.into_iter() {
            if area.contains(x, y) {
                return true;
            }
        }
        false
    }

    fn max_height(&self) -> Option<i64> {
        // While it's going up, y2 >= y1, and while it's going down, y2 <= y1
        // As soon as we see that it's coming down, return y1
        self.y
            .tuple_windows()
            .find(|&(y1, y2)| y2 <= y1)
            .map(|(y1, _)| y1)
    }
}

fn part1(area: &Area) -> i64 {
    // Get the minimum x veloctiy that will hit the area
    let min_goal = area.x_low as f64;
    // The sum from 1 to n is n(n+1)/2. So the to reach the goal we need to
    // find n such that n(n+1)/2 >= goal. That's the minimum x velocity
    let min_x_vel = (0.5 * ((8.0 * min_goal + 1.0).sqrt() - 1.0)).ceil() as i64;

    // Get the maximum x velocity that will hit the area
    let max_goal = area.x_high as f64;
    let max_x_vel = (0.5 * ((8.0 * max_goal + 1.0).sqrt() - 1.0)).ceil() as i64;

    // Since we are going for height, don't go lower than 0
    let min_y_vel = 0;

    // The highest y velocity is abs(area.y_low) - 1
    let max_y_vel = area.y_low.abs() - 1;

    // Work downwards to find the min y velocity that will hit the area
    let result = (min_y_vel..=max_y_vel).into_iter().rev().find_map(|y_vel| {
        // Try each x velocity
        (min_x_vel..=max_x_vel).into_iter().find_map(|x_vel| {
            // Create a ProbeState with the given x and y velocities
            let probe = ProbeState {
                x: XState {
                    pos: 0,
                    vel: x_vel,
                    target_max_range: area.x_high,
                },
                y: YState {
                    pos: 0,
                    vel: y_vel,
                    target_bottom: area.y_low,
                },
            };

            // Check if the probe will hit the area
            if probe.will_hit_area(area) {
                // If so, return the max height
                probe.max_height()
            } else {
                None
            }
        })
    });

    result.expect("Could not find a solution for part 1")
}

fn get_all_possible_vels(area: &Area) -> impl Iterator<Item = (i64, i64)> + '_ {
    // The minimum x velocity that will hit the area
    let min_goal = area.x_low as f64;
    // The sum from 1 to n is n(n+1)/2. So the to reach the goal we need to
    // find n such that n(n+1)/2 >= goal. That's the minimum x velocity
    let min_x_vel = (0.5 * ((8.0 * min_goal + 1.0).sqrt() - 1.0)).ceil() as i64;

    // The maximum x velocity that will hit the area is a speed where the first step
    // will hit the x_high limit
    let max_x_vel = area.x_high;

    // The lowest y velocity that will hit the area is one where the first step will
    // just hit the y_low limit
    let min_y_vel = area.y_low;

    // The highest y velocity is abs(min_y_vel) - 1
    let max_y_vel = min_y_vel.abs() - 1;

    // Iterate over each possible x velocity with each possible y velocity
    (min_x_vel..=max_x_vel)
        .into_iter()
        .flat_map(move |x_vel| {
            (min_y_vel..=max_y_vel)
                .into_iter()
                .map(move |y_vel| (x_vel, y_vel))
        })
        // Filter out the velocity pairs that will never hit the area
        .filter(|(xv, yv)| {
            let probe = ProbeState {
                x: XState {
                    pos: 0,
                    vel: *xv,
                    target_max_range: area.x_high,
                },
                y: YState {
                    pos: 0,
                    vel: *yv,
                    target_bottom: area.y_low,
                },
            };
            probe.will_hit_area(area)
        })
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day17.txt").expect("Failed to read day 17 input");
    let area = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&area);
    println!("Part 1 took {:.6} ns", part1_time.elapsed().as_nanos());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = get_all_possible_vels(&area).count();
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input_1() {
    let input_str = "target area: x=20..30, y=-10..-5\n";
    let got = parse_input(input_str);
    let want = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };
    assert_eq!(got, want);
}

#[test]
fn test_parse_input_2() {
    let input_str = "target area: x=175..227, y=-134..-79\n";
    let got = parse_input(input_str);
    let want = Area {
        x_low: 175,
        x_high: 227,
        y_low: -134,
        y_high: -79,
    };
    assert_eq!(got, want);
}

#[test]
fn test_y_iter_1() {
    let y_iter = YState {
        pos: 0,
        vel: 2,
        target_bottom: -10,
    };
    let expected_pos = vec![2, 3, 3, 2, 0, -3, -7];
    for (got, expected) in y_iter.into_iter().zip(expected_pos) {
        assert_eq!(got, expected);
    }
}

#[test]
fn test_y_iter_2() {
    let y_iter = YState {
        pos: 0,
        vel: 3,
        target_bottom: -10,
    };
    let expected_pos = vec![3, 5, 6, 6, 5, 3, 0, -4, -9];
    for (got, expected) in y_iter.into_iter().zip(expected_pos) {
        assert_eq!(got, expected);
    }
}

#[test]
fn test_y_iter_3() {
    let y_iter = YState {
        pos: 0,
        vel: 0,
        target_bottom: -10,
    };
    let expected_pos = vec![0, -1, -3, -6];
    for (got, expected) in y_iter.into_iter().zip(expected_pos) {
        assert_eq!(got, expected);
    }
}

#[test]
fn test_x_iter_1() {
    let x_iter = XState {
        pos: 0,
        vel: 7,
        target_max_range: 30,
    };
    let expected_pos = vec![7, 13, 18, 22, 25, 27, 28, 28, 28];
    for (got, expected) in x_iter.into_iter().zip(expected_pos) {
        assert_eq!(got, expected);
    }
}

#[test]
fn test_x_iter_2() {
    let x_iter = XState {
        pos: 0,
        vel: 6,
        target_max_range: 30,
    };
    let expected_pos = vec![6, 11, 15, 18, 20, 21, 21, 21, 21];
    for (got, expected) in x_iter.into_iter().zip(expected_pos) {
        assert_eq!(got, expected);
    }
}

#[test]
fn test_x_iter_3() {
    let x_iter = XState {
        pos: 0,
        vel: 9,
        target_max_range: 30,
    };
    let expected_pos = vec![9, 17, 24, 30];
    for (got, expected) in x_iter.into_iter().zip(expected_pos) {
        assert_eq!(got, expected);
    }
}

#[test]
fn test_probe_iter_1() {
    let probe = ProbeState {
        x: XState {
            pos: 0,
            vel: 7,
            target_max_range: 30,
        },
        y: YState {
            pos: 0,
            vel: 2,
            target_bottom: -10,
        },
    };
    let expected_pos = vec![
        (7, 2),
        (13, 3),
        (18, 3),
        (22, 2),
        (25, 0),
        (27, -3),
        (28, -7),
    ];
    for (got, expected) in probe.into_iter().zip(expected_pos) {
        assert_eq!(got, expected);
    }
}

#[test]
fn test_will_hit_area_1() {
    let area = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };

    let probe = ProbeState {
        x: XState {
            pos: 0,
            vel: 7,
            target_max_range: area.x_high,
        },
        y: YState {
            pos: 0,
            vel: 2,
            target_bottom: area.y_low,
        },
    };

    assert!(probe.will_hit_area(&area));
}

#[test]
fn test_will_hit_area_2() {
    let area = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };

    let probe = ProbeState {
        x: XState {
            pos: 0,
            vel: 6,
            target_max_range: area.x_high,
        },
        y: YState {
            pos: 0,
            vel: 3,
            target_bottom: area.y_low,
        },
    };

    assert!(probe.will_hit_area(&area));
}

#[test]
fn test_will_hit_area_3() {
    let area = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };

    let probe = ProbeState {
        x: XState {
            pos: 0,
            vel: 9,
            target_max_range: area.x_high,
        },
        y: YState {
            pos: 0,
            vel: 0,
            target_bottom: area.y_low,
        },
    };

    assert!(probe.will_hit_area(&area));
}

#[test]
fn test_will_not_hit_area_1() {
    let area = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };

    let probe = ProbeState {
        x: XState {
            pos: 0,
            vel: 17,
            target_max_range: area.x_high,
        },
        y: YState {
            pos: 0,
            vel: -4,
            target_bottom: area.y_low,
        },
    };

    assert!(!probe.will_hit_area(&area));
}

#[test]
fn test_part1() {
    let area = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };

    let got = part1(&area);
    assert_eq!(45, got);
}

#[test]
fn test_part1_actual() {
    let input_str =
        std::fs::read_to_string("input/day17.txt").expect("Failed to read day 17 input");
    let area = parse_input(&input_str);

    let got = part1(&area);
    assert_eq!(8911, got);
}

#[test]
fn test_get_all_possible_vels() {
    let area = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };

    let mut expected_vels: Vec<(i64, i64)> = [
        (23, -10),
        (25, -9),
        (27, -5),
        (29, -6),
        (22, -6),
        (21, -7),
        (9, 0),
        (27, -7),
        (24, -5),
        (25, -7),
        (26, -6),
        (25, -5),
        (6, 8),
        (11, -2),
        (20, -5),
        (29, -10),
        (6, 3),
        (28, -7),
        (8, 0),
        (30, -6),
        (29, -8),
        (20, -10),
        (6, 7),
        (6, 4),
        (6, 1),
        (14, -4),
        (21, -6),
        (26, -10),
        (7, -1),
        (7, 7),
        (8, -1),
        (21, -9),
        (6, 2),
        (20, -7),
        (30, -10),
        (14, -3),
        (20, -8),
        (13, -2),
        (7, 3),
        (28, -8),
        (29, -9),
        (15, -3),
        (22, -5),
        (26, -8),
        (25, -8),
        (25, -6),
        (15, -4),
        (9, -2),
        (15, -2),
        (12, -2),
        (28, -9),
        (12, -3),
        (24, -6),
        (23, -7),
        (25, -10),
        (7, 8),
        (11, -3),
        (26, -7),
        (7, 1),
        (23, -9),
        (6, 0),
        (22, -10),
        (27, -6),
        (8, 1),
        (22, -8),
        (13, -4),
        (7, 6),
        (28, -6),
        (11, -4),
        (12, -4),
        (26, -9),
        (7, 4),
        (24, -10),
        (23, -8),
        (30, -8),
        (7, 0),
        (9, -1),
        (10, -1),
        (26, -5),
        (22, -9),
        (6, 5),
        (7, 5),
        (23, -6),
        (28, -10),
        (10, -2),
        (11, -1),
        (20, -9),
        (14, -2),
        (29, -7),
        (13, -3),
        (23, -5),
        (24, -8),
        (27, -9),
        (30, -7),
        (28, -5),
        (21, -10),
        (7, 9),
        (6, 6),
        (21, -5),
        (27, -10),
        (7, 2),
        (30, -9),
        (21, -8),
        (22, -7),
        (24, -9),
        (20, -6),
        (6, 9),
        (29, -5),
        (8, -2),
        (27, -8),
        (30, -5),
        (24, -7),
    ]
    .into_iter()
    .collect();
    expected_vels.dedup();
    expected_vels.sort();

    let mut got: Vec<(i64, i64)> = get_all_possible_vels(&area).collect();
    got.sort();

    assert_eq!(expected_vels, got);
}

#[test]
fn test_part2() {
    let area = Area {
        x_low: 20,
        x_high: 30,
        y_low: -10,
        y_high: -5,
    };

    let got = get_all_possible_vels(&area).count();
    assert_eq!(112, got);
}

#[test]
fn test_part2_actual() {
    let input_str =
        std::fs::read_to_string("input/day17.txt").expect("Failed to read day 17 input");
    let area = parse_input(&input_str);

    let got = get_all_possible_vels(&area).count();
    assert_eq!(4748, got);
}
