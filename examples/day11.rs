use ndarray::{arr2, Array, Array2, ArrayBase, Dim, OwnedRepr};

fn parse_input(input: &str) -> Array2<u8> {
    let mut result = Array2::zeros((10, 10));
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, value) in row.chars().enumerate() {
            result[(row_idx, col_idx)] =
                value.to_digit(10).expect("Could not convert to digit") as u8;
        }
    }
    result
}

fn get_neighbors(nrows: i32, ncols: i32, row_idx: i32, col_idx: i32) -> Vec<(usize, usize)> {
    const NEIGHBOR_DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    NEIGHBOR_DIRS
        .iter()
        .map(|(row_change, col_change)| ((row_idx) + row_change, (col_idx) + col_change))
        .filter(|(new_row_idx, new_col_idx)| {
            (*new_row_idx >= 0)
                & (*new_col_idx >= 0)
                & (*new_row_idx < nrows)
                & (*new_col_idx < ncols)
        })
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

fn main() {
    let neighbors_arr: Array2<Vec<(usize, usize)>> = Array2::from_shape_vec(
        (10, 10),
        (0..10)
            .into_iter()
            .flat_map(|row_idx| {
                (0..10)
                    .into_iter()
                    .map(move |col_idx| get_neighbors(10, 10, row_idx, col_idx))
            })
            .collect(),
    )
    .expect("Could not create neighbors array");

    dbg!(&neighbors_arr);
}

#[test]
fn test_parse_input() {
    let input_str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let got = parse_input(input_str);

    let expected = arr2(&[
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ]);

    assert_eq!(expected, got);
}
