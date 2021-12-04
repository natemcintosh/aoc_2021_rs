use ndarray::{arr2, Array2, ArrayView2};

fn parse_input(input: &str) -> (Vec<u16>, Vec<Array2<u16>>) {
    // Split on double newlines
    let mut lines = input.split("\n\n");

    // The first line is the bingo numbers called
    let bingo_numbers: Vec<u16> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().expect("Could not parse bingo number"))
        .collect();

    // Each following split item is a bingo board
    let bingo_boards: Vec<Array2<u16>> = lines
        .map(|board| {
            // Split on newlines
            let rows = board.split('\n');

            // The first line is the bingo board
            let mut board = Array2::<u16>::zeros((5, 5));

            // The rest of the lines are the bingo board
            for (row_idx, row) in rows.enumerate() {
                for (col_index, num) in row.split_whitespace().enumerate() {
                    board[(row_idx, col_index)] =
                        num.parse().expect("Could not parse board number");
                }
            }

            board
        })
        .collect();

    (bingo_numbers, bingo_boards)
}

fn mark_number(number: u16, board: ArrayView2<u16>, board_markers: &mut Array2<bool>) {
    // Find the index of an occurrence of the number in the board
    let indices = board
        .indexed_iter()
        .filter(|((_, _), &item)| item == number)
        .map(|((row_idx, col_idx), _)| (row_idx, col_idx))
        .next();

    if let Some((row_idx, col_idx)) = indices {
        // Mark the number in the board
        board_markers[(row_idx, col_idx)] = true;
    }
}

fn board_won(board_markers: ArrayView2<bool>) -> bool {
    if board_markers
        .columns()
        .into_iter()
        .any(|col| col.iter().all(|&item| item))
    {
        return true;
    } else if board_markers
        .rows()
        .into_iter()
        .any(|row| row.iter().all(|&item| item))
    {
        return true;
    }

    false
}

fn part1(bingo_numbers: &[u16], bingo_boards: &[ArrayView2<u16>]) -> usize {
    // Create the board markers, one for each board in bingo_boards, boolean arrays of whether or not a number has been marked
    let board_marker_example = arr2(&[[false; 5]; 5]);

    let mut board_markers: Vec<Array2<bool>> = Vec::new();
    for _ in bingo_boards {
        board_markers.push(board_marker_example.clone());
    }

    // For each bingo number
    for &number in bingo_numbers {
        // Mark it on all boards
        for (&board, marker) in bingo_boards.iter().zip(board_markers.iter_mut()) {
            mark_number(number, board, marker);
        }

        // Check if any board has won
        let possible_winner = board_markers
            .iter()
            .enumerate()
            .find(|(_, marker)| board_won(marker.view()));

        if let Some((winners_idx, winning_board)) = possible_winner {
            // Get the sum of all unmarked numbers on the winning board
            let sum_of_unmared_numbers: usize = bingo_boards[winners_idx]
                .iter()
                // Run through it with the marker board
                .zip(winning_board.iter())
                // Remove any items where the marker is marked
                .filter(|(_, &item)| !item)
                .map(|(&board_num, _)| board_num as usize)
                .sum();

            return sum_of_unmared_numbers * number as usize;
        }
    }

    panic!("Could not find any winning boards");
}

fn part2(bingo_numbers: &[u16], bingo_boards: &[ArrayView2<u16>]) -> usize {
    // Create the board markers, one for each board in bingo_boards, boolean arrays of whether or not a number has been marked
    let board_marker_example = arr2(&[[false; 5]; 5]);

    let mut board_markers: Vec<Array2<bool>> = Vec::new();
    for _ in bingo_boards {
        board_markers.push(board_marker_example.clone());
    }

    let mut winning_number: i32 = -1;
    let mut winning_boards_in_order: Vec<usize> = Vec::new();

    // For each bingo number
    for &number in bingo_numbers {
        // Mark it on all boards
        for (&board, marker) in bingo_boards.iter().zip(board_markers.iter_mut()) {
            mark_number(number, board, marker);
        }

        // Find any winners, and update `winning_boards_in_order`
        let winners_indices: Vec<usize> = board_markers
            .iter()
            .enumerate()
            .filter(|(_, marker)| board_won(marker.view()))
            .map(|(idx, _)| idx)
            .collect();

        // For each winners_indices check if it already exists in winning_boards_in_order, if not add it
        for &winner_idx in &winners_indices {
            if !winning_boards_in_order.contains(&winner_idx) {
                winning_boards_in_order.push(winner_idx);
            }
        }

        winning_number = i32::from(number);
        // If all have finished, then exit the loop
        if winners_indices.len() == bingo_boards.len() {
            break;
        }
    }
    // What is the winner's index
    let winner_idx = winning_boards_in_order.last().unwrap();

    // Take the `winner_idx`, and get the sum of all unmarked numbers on the winning board
    let sum_of_unmared_numbers: usize = bingo_boards[*winner_idx]
        .iter()
        // Run through it with the marker board
        .zip(board_markers[*winner_idx].iter())
        // Remove any items where the marker is marked
        .filter(|(_, &item)| !item)
        .map(|(&board_num, _)| board_num as usize)
        .sum();

    sum_of_unmared_numbers * (winning_number as usize)
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str = std::fs::read_to_string("input/day04.txt").expect("Failed to read day 4 input");
    let (board_numbers, boards) = parse_input(&input_str);
    let board_views: Vec<ArrayView2<u16>> = boards.iter().map(ndarray::ArrayBase::view).collect();
    println!(
        "Setup took {:.6} microseconds",
        setup_time.elapsed().as_micros()
    );

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&board_numbers, &board_views);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&board_numbers, &board_views);
    println!(
        "Part 2 took {:.6} microseconds",
        part2_time.elapsed().as_micros()
    );

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    let (numbers_drawn, boards) = parse_input(&input_str);

    let expected_numbers_drawn: Vec<u16> = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    let expected_boards: Vec<Array2<u16>> = vec![
        arr2(&[
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]),
        arr2(&[
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ]),
        arr2(&[
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]),
    ];

    assert_eq!(numbers_drawn, expected_numbers_drawn);
    assert_eq!(boards, expected_boards);
}

#[test]
fn test_mark_board() {
    let board = arr2(&[
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 10],
        [11, 12, 13, 14, 15],
        [16, 17, 18, 19, 20],
        [21, 22, 23, 24, 25],
    ]);
    let mut marker_board = arr2(&[
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]);
    mark_number(2, board.view(), &mut marker_board);

    let expected = arr2(&[
        [false, true, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]);
    assert_eq!(expected, marker_board);
}

#[test]
fn test_board_won_1() {
    let test_board1 = arr2(&[
        [true, true, true, true, true],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]);
    let got = board_won(test_board1.view());
    assert_eq!(true, got);
}

#[test]
fn test_board_won_2() {
    let test_board1 = arr2(&[
        [false, true, false, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false],
    ]);
    let got = board_won(test_board1.view());
    assert_eq!(true, got);
}

#[test]
fn test_board_won_3() {
    let test_board1 = arr2(&[
        [false, true, false, false, false],
        [false, false, false, false, true],
        [false, true, true, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false],
    ]);
    let got = board_won(test_board1.view());
    assert_eq!(false, got);
}

#[test]
fn test_part1() {
    let numbers_drawn: Vec<u16> = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    let boards: Vec<Array2<u16>> = vec![
        arr2(&[
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]),
        arr2(&[
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ]),
        arr2(&[
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]),
    ];
    let board_views: Vec<ArrayView2<u16>> = boards.iter().map(|b| b.view()).collect();

    let got = part1(&numbers_drawn, &board_views);
    assert_eq!(got, 4512);
}

#[test]
fn test_part2() {
    let numbers_drawn: Vec<u16> = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    let boards: Vec<Array2<u16>> = vec![
        arr2(&[
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]),
        arr2(&[
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ]),
        arr2(&[
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]),
    ];
    let board_views: Vec<ArrayView2<u16>> = boards.iter().map(|b| b.view()).collect();

    let got = part2(&numbers_drawn, &board_views);
    assert_eq!(got, 1924);
}
