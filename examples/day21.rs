use itertools::Itertools;

fn parse_input(input: &str) -> (usize, usize) {
    let (p1_line, p2_line) = input
        .split_once('\n')
        .expect("Could not split around newline");

    let p1_start: usize = p1_line
        .split_ascii_whitespace()
        .last()
        .expect("Could not get last item from first line")
        .parse()
        .expect("Could not convert first to number");

    let p2_start: usize = p2_line
        .split_ascii_whitespace()
        .last()
        .expect("Could not get last item from second line")
        .parse()
        .expect("Could not convert second to number");

    (p1_start, p2_start)
}

fn part1(p1_start: usize, p2_start: usize) -> usize {
    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut p1_loc = p1_start;
    let mut p2_loc = p2_start;

    let locs = [10, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut n_die_rolls = 0;

    for (a, b, c) in (1..=100).cycle().tuples() {
        // Increase die rolls
        n_die_rolls += 3;

        // If odd, then it's the first player
        if a % 2 == 1 {
            p1_loc = locs[(p1_loc + a + b + c) % 10];
            p1_score += p1_loc;
            if p1_score >= 1_000 {
                return p2_score * n_die_rolls;
            }
        } else {
            p2_loc = locs[(p2_loc + a + b + c) % 10];
            p2_score += p2_loc;
            if p2_score >= 1_000 {
                return p1_score * n_die_rolls;
            }
        }
    }

    unreachable!()
}

fn add_three(count: &[usize]) -> Vec<usize> {
    // count is a slice where the "key" is the index, and the "value" is the number
    // of times that key appears
    // This means there will be some numbers near the start with a count of 0, and that's
    // fine

    let mut result = vec![0; count.len() + 3];
    for (item, count) in count.iter().enumerate() {
        result[item + 1] += count;
        result[item + 2] += count;
        result[item + 3] += count;
    }

    result
}

/// Returns an array representing the board, where the index repesents a spot on the
/// board (1-10). Note that this array has a spot at 0, which should hold 0. The value
/// at each index represents the number pawns at that location
fn roll_dirac_dice_thre_times(start: usize) -> [usize; 11] {
    const LOCS: [usize; 10] = [10, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // After rolling dirac dice three times, we would add the following to the start
    const TO_ADD: [usize; 27] = [
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ];

    // Count up the results
    let mut result = [0_usize; 11];

    TO_ADD
        .iter()
        // Add start to everything in `TO_ADD`
        .map(|f| f + start)
        // Get the correct board location using locs
        .map(|f| LOCS[f % 10])
        // Add 1 to that location on the board
        .for_each(|f| result[f] += 1);

    result
}

fn part2(p1_start: usize, p2_start: usize) -> usize {
    // The idea here is to have an array representing the board, where index 0 is board
    // position 1, index 1 is board position 2, etc.
    // Each item in the array is an array representing the score from 0 to 21
    // (inclusive), and the value at each index tells us how many "players" are at that
    // spot on the board, and have
    // that score
    let mut p1_old: [[usize; 22]; 10] = [[0; 22]; 10];
    let mut p2_old: [[usize; 22]; 10] = [[0; 22]; 10];
    let mut p1_new: [[usize; 22]; 10] = [[0; 22]; 10];
    let mut p2_new: [[usize; 22]; 10] = [[0; 22]; 10];

    // Add the players to the correct positions. They have starting scores of 0
    p1_old[p1_start][0] = 1;
    p2_old[p2_start][0] = 1;

    // To keep track of games won
    let mut p1_games_won: usize = 0;
    let mut p2_games_won: usize = 0;

    // Compute the ending positions for each of the ten board positions
    let positions_after_rolling: [[usize; 11]; 10] = [
        roll_dirac_dice_thre_times(1),
        roll_dirac_dice_thre_times(2),
        roll_dirac_dice_thre_times(3),
        roll_dirac_dice_thre_times(4),
        roll_dirac_dice_thre_times(5),
        roll_dirac_dice_thre_times(6),
        roll_dirac_dice_thre_times(7),
        roll_dirac_dice_thre_times(8),
        roll_dirac_dice_thre_times(9),
        roll_dirac_dice_thre_times(10),
    ];

    let mut idx: usize = 0;

    // Begin playing the game
    loop {
        // Check if all the pawns have finished

        // Player 1
        for (board_pos, scores_at_pos) in p1_old.iter().enumerate() {
            // For each player 1 position, get the positions after rolling
            let new_positions = positions_after_rolling[board_pos];
            
            // For each new position
            for new_pos in new_positions {
                // Need to move pawns from `board_pos` to `new_pos`, and add `new_pos` 
                // to the score at `new_pos` for each old score as `board_pos`
                for (old_score, n_items) in scores_at_pos.iter().enumerate() {
                    let new_score = old_score + new_pos;
                    if new_score >= 21 {
                        p1_games_won += n_items
                    } else {
                        p1_new[new_pos][new_score] += n_items;
                    }
                }
            }
        }

        // Player 2
        for (board_pos, scores_at_pos) in p2_old.iter().enumerate() {
            // For each player 1 position, get the positions after rolling
            let new_positions = positions_after_rolling[board_pos];
            
            // For each new position
            for new_pos in new_positions {
                // Need to move pawns from `board_pos` to `new_pos`, and add `new_pos` 
                // to the score at `new_pos` for each old score as `board_pos`
                for (old_score, n_items) in scores_at_pos.iter().enumerate() {
                    let new_score = old_score + new_pos;
                    if new_score >= 21 {
                        p2_games_won += n_items
                    } else {
                        p2_new[new_pos][new_score] += n_items;
                    }
                }
            }
        }

        // Finally copy new into old. Don't have to clone bc array impls Copy trait,
        // so they are copied instead of moved.
        p1_old = p1_new;
        p2_old = p2_new;
        // Clean out the new arrays
        p1_new = [[0; 22]; 10];
        p2_new = [[0; 22]; 10];

        // Failsafe
        idx += 1;
        if idx >= 10_000_000 {
            return 0;
        }
    }

}

// fn part2(p1_start: usize, p2_start: usize) -> usize {
//     // It looks like at each step, the number of possibilities increases by 2
//     // The pattern looks like
//     // 1
//     // 2,3,4
//     // 3:1, 4:2, 5:3, 6:2, 7:1
//     // 4:1, 5:3, 6:6, 7:7, 8:6, 9:3, 10:1
//     // 5:1, 6:4, 7:9, 8:16, 9:19, 10:16, 11:9, 12:4, 13:1
//     // The number of items produced at each stage, n, (initial stage is 0) is 3^n,

//     let mut p1_games_won = 0;
//     let mut p2_games_won = 0;

//     // Put the starting positions in vectors
//     let mut p1_positions = vec![0; p1_start + 1];
//     p1_positions[p1_start] = 1;
//     let mut p2_positions = vec![0; p2_start + 1];
//     p2_positions[p2_start] = 1;

//     loop {
//         // Player 1 rolls the dice three times
//         p1_positions = add_three(&p1_positions);
//         p1_positions = add_three(&p1_positions);
//         p1_positions = add_three(&p1_positions);

//         // Get all of the positions into the 1..=10 range
//         // Anything at index 11 should be at index 1
//         if p1_positions.len() >= 12 {
//             let cnt = p1_positions[11];
//             p1_positions[1] += cnt;
//             if p1_positions.len() >= 13 {
//                 let cnt = p1_positions[12];
//                 p1_positions[2] += cnt;
//                 if p1_positions.len() >= 14 {
//                     let cnt = p1_positions[13];
//                     p1_positions[3] += cnt;
//                     p1_positions.pop();
//                 }
//                 p1_positions.pop();
//             }
//             p1_positions.pop();
//         }

//         // If player 1 has any counts at index 21, that means they won that many games
//         // Add that to their count, and pop that index from the vec
//         while p1_positions.len() >= 22 {
//             p1_games_won += p1_positions
//                 .pop()
//                 .expect("Could not pop player 1 items at index 21");
//         }

//         // Player 2 rolls the dice three times
//         p2_positions = add_three(&p2_positions);
//         p2_positions = add_three(&p2_positions);
//         p2_positions = add_three(&p2_positions);

//         // If player 2 has any counts at index 21, that means they won that many games
//         // Add that to their count, and pop that index from the vec
//         while p2_positions.len() >= 22 {
//             p2_games_won += p2_positions
//                 .pop()
//                 .expect("Could not pop player 1 items at index 21");
//         }

//         // If both have all zeros, all games are over
//         if p1_positions.iter().all(|x| *x == 0) && p2_positions.iter().all(|x| *x == 0) {
//             break;
//         }
//     }

//     p1_games_won.max(p2_games_won)
// }

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day21.txt").expect("Failed to read day 21 input");
    let (p1_start, p2_start) = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(p1_start, p2_start);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = part2(p1_start, p2_start);
    // println!("Part 2 took {:.6} ms", part2_time.elapsed().as_millis());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "Player 1 starting position: 10
Player 2 starting position: 6";
    let expected = (10, 6);
    let got = parse_input(input_str);
    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let got = part1(4, 8);
    assert_eq!(739785, got);
}

#[test]
fn test_part1_actual() {
    let input_str =
        std::fs::read_to_string("input/day21.txt").expect("Failed to read day 21 input");
    let (p1_start, p2_start) = parse_input(&input_str);
    let got = part1(p1_start, p2_start);
    assert_eq!(900099, got);
}

#[test]
fn test_add_three_1() {
    // This represents 0:1, 1:2, 2:3
    let input = [1, 2, 3];
    let got = add_three(&input);
    // Expect to get 1,2,3,2,2,3,3,4,4,3,3,3,4,4,4,5,5,5
    // So a counter would hold {1: 1, 2: 3, 3: 6, 4: 5, 5: 3}
    let expected = vec![0, 1, 3, 6, 5, 3];
    assert_eq!(expected, got);
}

#[test]
fn test_add_three_2() {
    // Input is
    // 3:1, 4:2, 5:3, 6:2, 7:1
    let input = [0, 0, 0, 1, 2, 3, 2, 1];
    let got = add_three(&input);
    // Output should be
    // 4:1, 5:3, 6:6, 7:7, 8:6, 9:3, 10:1
    let expected = vec![0, 0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
    assert_eq!(expected, got);
}

#[test]
fn test_roll_dirac_dice_1() {
    let start = 1;
    let got = roll_dirac_dice_thre_times(start);
    // index                    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    let expected: [usize; 11] = [0, 0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
    assert_eq!(expected, got);
}

#[test]
fn test_roll_dirac_dice_2() {
    let start = 5;
    let got = roll_dirac_dice_thre_times(start);

    // index                    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    let expected: [usize; 11] = [0, 7, 6, 3, 1, 0, 0, 0, 1, 3, 6];
    assert_eq!(expected, got);
}

// #[test]
// fn test_part2() {
// let got = part2(4, 8);
//     assert_eq!(444356092776315, got);
// }
