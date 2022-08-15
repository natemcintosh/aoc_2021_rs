use std::collections::BinaryHeap;

use ndarray::{concatenate, Array2, ArrayView2, Axis};

fn parse_input(input: &str) -> Array2<u8> {
    let nrows = input.lines().count();
    let ncols = input
        .lines()
        .next()
        .expect("There was not even a single line of text to read")
        .chars()
        .count();

    let mut arr = Array2::<u8>::zeros((nrows, ncols));
    for (ridx, line) in input.lines().enumerate() {
        for (cidx, n) in line.chars().enumerate() {
            // arr[(ridx, cidx)] = n.try_into().expect("Could not parse number");
            arr[(ridx, cidx)] = n
                .to_digit(10)
                .expect("Could not parse number")
                .try_into()
                .expect("Could not convert u32 to u8");
        }
    }
    arr
}

fn get_neighbors(arr: ArrayView2<u8>, row_idx: usize, col_idx: usize) -> Vec<(usize, usize)> {
    let nrows = arr.nrows() as i32;
    let ncols = arr.ncols() as i32;
    const NEIGHBOR_DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    NEIGHBOR_DIRS
        .iter()
        .map(|(row_change, col_change)| {
            ((row_idx as i32) + row_change, (col_idx as i32) + col_change)
        })
        .filter(|(new_row_idx, new_col_idx)| {
            (*new_row_idx >= 0)
                & (*new_col_idx >= 0)
                & (*new_row_idx < nrows)
                & (*new_col_idx < ncols)
        })
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Not entirely sure, might actually be Dijkstra's.
/// Based on [this](https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples)
fn uniform_cost_search(
    arr: ArrayView2<u8>,
    start: (usize, usize),
    goal: (usize, usize),
) -> (Array2<usize>, Array2<(usize, usize)>) {
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        position: start,
    });

    let mut costs = Array2::<usize>::zeros((arr.nrows(), arr.ncols()));
    costs.fill(usize::MAX);

    let mut predecessors = Array2::<(usize, usize)>::from_elem((arr.nrows(), arr.ncols()), (0, 0));

    while let Some(State {
        cost,
        position: pos,
    }) = frontier.pop()
    {
        if pos == goal {
            return (costs, predecessors);
        }

        if cost > costs[pos] {
            continue;
        }

        for nbr in get_neighbors(arr, pos.0, pos.1) {
            let new = State {
                cost: cost + usize::from(arr[nbr]),
                position: nbr,
            };

            // If the new cost is less than before
            if new.cost < costs[new.position] {
                // add it to the frontier
                frontier.push(new);

                // update the costs array
                costs[new.position] = new.cost;

                // Update its predecessor
                predecessors[new.position] = pos;
            }
        }
    }
    (costs, predecessors)
}

fn array_inc(arr: ArrayView2<u8>) -> Array2<u8> {
    arr.mapv(|v| if v >= 9 { 1 } else { v + 1 })
}

fn quintuple_map(arr: ArrayView2<u8>) -> Array2<u8> {
    // Make four copies below this one, each with the numbers one larger than the last
    let r2 = array_inc(arr);
    let r3 = array_inc(r2.view());
    let r4 = array_inc(r3.view());
    let r5 = array_inc(r4.view());
    let c1 = concatenate![Axis(0), arr, r2, r3, r4, r5];

    // Make four copies of big_col, each one larger than the last
    let c2 = array_inc(c1.view());
    let c3 = array_inc(c2.view());
    let c4 = array_inc(c3.view());
    let c5 = array_inc(c4.view());
    // let big_arr = concatenate(
    //     Axis(1),
    //     &[c1.view(), c2.view(), c3.view(), c4.view(), c5.view()],
    // )
    // .expect("Could not concatenate columns side by side");
    concatenate![Axis(1), c1, c2, c3, c4, c5]
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day15.txt").expect("Failed to read day 15 input");
    let arr = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    let goal = (arr.nrows() - 1, arr.ncols() - 1);

    // Part 1
    let part1_time = std::time::Instant::now();
    let (costs, _) = uniform_cost_search(arr.view(), (0, 0), goal);
    let part1_result = costs[goal];
    // dbg!(reconstruct_path(predecessors.view(), (0, 0), goal));
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let arr_part2 = quintuple_map(arr.view());
    let goal = (arr_part2.nrows() - 1, arr_part2.ncols() - 1);
    let (costs, _) = uniform_cost_search(arr_part2.view(), (0, 0), goal);
    let part2_result = costs[goal];
    // let part2_result =
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    fn reconstruct_path(
        arr: ArrayView2<(usize, usize)>,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let mut result: Vec<_> = Vec::new();
        result.push(goal);
        let mut curr = goal;
        let mut itr = 0;
        while (curr != start) & (itr < 1_000) {
            itr += 1;
            curr = arr[curr];
            result.push(curr);
        }
        if itr < 1_000 {
            result.reverse();
            return Some(result);
        }
        None
    }
    #[test]
    fn test_parse_input() {
        let input_str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let expected = arr2(&[
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]);
        let got = parse_input(input_str);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_part1() {
        let arr: Array2<u8> = arr2(&[
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]);

        let expected_rows: [usize; 19] = [0, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 9];
        let expected_cols: [usize; 19] = [0, 0, 0, 1, 2, 3, 4, 5, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9];
        let expected_path: Vec<(usize, usize)> = expected_rows
            .iter()
            .zip(expected_cols.iter())
            .map(|(&r, &c)| (r, c))
            .collect();
        let (got_costs, got_path) = uniform_cost_search(arr.view(), (0, 0), (9, 9));

        let goal = (9, 9);
        assert_eq!(got_costs[goal], 40);

        let path = reconstruct_path(got_path.view(), (0, 0), goal);
        let got_path = path.expect("Could not find a proper path home");
        assert_eq!(expected_path, got_path);
    }

    #[test]
    fn test_array_inc() {
        let arr = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let expected = arr2(&[[2, 3, 4], [5, 6, 7], [8, 9, 1]]);
        let got = array_inc(arr.view());
        assert_eq!(expected, got);
    }

    #[test]
    fn test_quintuple_map() {
        let arr: Array2<u8> = arr2(&[
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]);
        let expected = parse_input(
            "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479",
        );

        let got = quintuple_map(arr.view());

        assert_eq!(expected, got);
    }

    #[test]
    fn test_part2() {
        let arr = parse_input(
            "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479",
        );
        let goal = (arr.nrows() - 1, arr.ncols() - 1);
        let (costs, _) = uniform_cost_search(arr.view(), (0, 0), goal);
        let got = costs[goal];
        assert_eq!(315, got);
    }
}
