use ndarray::{arr2, s, Array2, ArrayView2, Axis};

#[derive(Debug, PartialEq)]
struct Fold {
    axis: Axis,
    index: usize,
}

fn parse_input(input: &str) -> (Array2<bool>, Vec<Fold>) {
    let (dot_inds_str, fold_strs) = input
        .split_once("\n\n")
        .expect("Could not split on a double new line");

    // Convert all of the indices to number pairs, and flip them so they have the proper
    // row->col indexing order
    let dot_inds: Vec<(usize, usize)> = dot_inds_str
        .lines()
        .map(|line| {
            let (col, row) = line
                .split_once(',')
                .expect("Could not split coordinates around comma");
            (
                row.parse().expect("Could not parse row number"),
                col.parse().expect("Could not parse col number"),
            )
        })
        .collect();

    // Get the extents of the array
    let max_row = dot_inds
        .iter()
        .map(|(row, _)| row)
        .max()
        .expect("Could not find a maximum row")
        + 1;
    let max_col = dot_inds
        .iter()
        .map(|(_, col)| col)
        .max()
        .expect("Could not find a minimum col")
        + 1;

    // Create an array of all false
    let mut arr: Array2<bool> = Array2::from_elem((max_row, max_col), false);

    // At the indices, set to true
    dot_inds
        .iter()
        .for_each(|(row, col)| arr[[*row, *col]] = true);

    // Convert "fold along x/y=\d+" to a vec of Folds
    let folds: Vec<Fold> = fold_strs
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .last()
                .expect("Could not find fold str")
        })
        .map(|s| {
            let (x_or_y, fold_idx) = s.split_once('=').expect("Could not split around '='");
            let axis: Axis = match x_or_y {
                "y" => Axis(0),
                "x" => Axis(1),
                _ => panic!("Fold axis was not x or y"),
            };
            let index: usize = fold_idx.parse().expect("Could not parse fold index");
            Fold { axis, index }
        })
        .collect();

    (arr, folds)
}

fn apply_fold(arr: ArrayView2<bool>, fold: &Fold) -> Array2<bool> {
    // Split the array at the correct spot
    let (top_left, bottom_right) = arr.split_at(fold.axis, fold.index);
    // Remove the top row or left column from `bottom_right`
    let bottom_right = match fold.axis {
        Axis(0) => bottom_right.slice(s![1.., ..]),
        Axis(1) => bottom_right.slice(s![.., 1..]),
        _ => panic!("Fold should not be along an axis other than 0 or 1"),
    };

    // Flip the bottom/right side as required
    let mut new_bottom_right: Array2<bool> = bottom_right.into_owned();
    new_bottom_right.invert_axis(fold.axis);

    // The fold will not always fold the original array in half. One dimension will
    // always be the same between the two arrays, but one may be smaller than the other.
    // Assume the bottom_right array is always the smaller

    let new_dim: (usize, usize) = if bottom_right.nrows() < top_left.nrows() {
        (top_left.nrows() - bottom_right.nrows(), top_left.ncols())
    } else {
        (top_left.nrows(), top_left.ncols() - bottom_right.ncols())
    };

    if (new_dim.0 > 0) & (new_dim.1 > 0) {
        // Create an array of false to concatenate with `new_bottom_right`
        let fs = Array2::from_elem(new_dim, false);

        // Concatenate the falses with `new_bottom_right`
        new_bottom_right = ndarray::concatenate(fold.axis, &[fs.view(), new_bottom_right.view()])
            .expect("Could not concatenate axes");
    }

    // // Zip them together into a new array
    new_bottom_right.zip_mut_with(&top_left, |a, b| {
        *a = *a | b;
    });

    new_bottom_right
}

fn part1(arr: ArrayView2<bool>, fold: &Fold) -> usize {
    // Apply one fold, and count how many trues exist in the output
    apply_fold(arr, fold).iter().filter(|&&b| b).count()
}

fn part2(arr: ArrayView2<bool>, folds: &[Fold]) -> Array2<char> {
    let mut bool_result: Array2<bool> = arr.to_owned();
    for f in folds {
        bool_result = apply_fold(bool_result.view(), f);
    }

    bool_result.mapv(|b| if b { '#' } else { ' ' })
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day13.txt").expect("Failed to read day 13 input");
    let (arr, folds) = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(arr.view(), &folds[0]);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(arr.view(), &folds);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result:\n {}", part2_result);
}

#[test]
fn test_parse_input() {
    let input_str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let (arr, folds) = parse_input(input_str);

    let expected_arr: Array2<bool> = arr2(&[
        [
            false, false, false, true, false, false, true, false, false, true, false,
        ],
        [
            false, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            true, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, true, false, false, false, false, true, false, true,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, true, false, false, false, false, true, false, true, true, false,
        ],
        [
            false, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, true, false, false, false, true,
        ],
        [
            true, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            true, false, true, false, false, false, false, false, false, false, false,
        ],
    ]);

    let expected_folds = vec![
        Fold {
            axis: Axis(0),
            index: 7,
        },
        Fold {
            axis: Axis(1),
            index: 5,
        },
    ];

    assert_eq!(expected_arr, arr);
    assert_eq!(expected_folds, folds);
}

#[test]
fn test_fold_1() {
    let arr: Array2<bool> = arr2(&[
        [
            false, false, false, true, false, false, true, false, false, true, false,
        ],
        [
            false, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            true, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, true, false, false, false, false, true, false, true,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, true, false, false, false, false, true, false, true, true, false,
        ],
        [
            false, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, true, false, false, false, true,
        ],
        [
            true, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            true, false, true, false, false, false, false, false, false, false, false,
        ],
    ]);

    let fold = Fold {
        axis: Axis(0),
        index: 7,
    };
    let got = apply_fold(arr.view(), &fold);
    let expected = arr2(&[
        [
            true, false, true, true, false, false, true, false, false, true, false,
        ],
        [
            true, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, true, false, false, false, true,
        ],
        [
            true, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, true, false, true, false, false, true, false, true, true, true,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
    ]);

    assert_eq!(expected, got);
}

#[test]
fn test_fold_2() {
    let arr = arr2(&[
        [
            true, false, true, true, false, false, true, false, false, true, false,
        ],
        [
            true, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, true, false, false, false, true,
        ],
        [
            true, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, true, false, true, false, false, true, false, true, true, true,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
    ]);

    let fold = Fold {
        axis: Axis(1),
        index: 5,
    };

    let got = apply_fold(arr.view(), &fold);
    let expected: Array2<bool> = arr2(&[
        [true, true, true, true, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, false, false, false, true],
        [true, true, true, true, true],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]);

    assert_eq!(expected, got);
}

#[test]
fn test_part1() {
    let arr: Array2<bool> = arr2(&[
        [
            false, false, false, true, false, false, true, false, false, true, false,
        ],
        [
            false, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            true, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, true, false, false, false, false, true, false, true,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, true, false, false, false, false, true, false, true, true, false,
        ],
        [
            false, false, false, false, true, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, true, false, false, false, true,
        ],
        [
            true, false, false, false, false, false, false, false, false, false, false,
        ],
        [
            true, false, true, false, false, false, false, false, false, false, false,
        ],
    ]);

    let fold = Fold {
        axis: Axis(0),
        index: 7,
    };

    let got = part1(arr.view(), &fold);
    assert_eq!(17, got);
}

#[test]
fn test_part1_actual() {
    let input_str =
        std::fs::read_to_string("input/day13.txt").expect("Failed to read day 13 input");
    let (arr, folds) = parse_input(&input_str);
    let got = part1(arr.view(), &folds[0]);
    assert_eq!(661, got);
}
